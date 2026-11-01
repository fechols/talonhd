//! REU image format, shared by the host baker and the C64 runtime.
//! Vendored from gclass (`a5775bf`) and adapted: vertical panorama, four
//! global colors, per-frame phase count.
//!
//! All integers are little-endian; REU addresses are stored as u32 and use
//! the low 24 bits. The image is loaded at REU address 0.
//!
//! ```text
//! Header (64 bytes at 0)
//! Background panorama: bitmap rows, screen rows
//! Frame directory: FRAME_RECORD bytes per sprite frame
//! Per frame, per phase: column table, segments, pixels
//! ```
//!
//! ## Colors
//! All four multicolor codes are global: 00 is `$D021`, 01/10 are the
//! screen RAM nibbles (the same byte in every cell), 11 is color RAM (one
//! value everywhere). Nothing ever writes per-cell attributes, so sprites
//! and background can never clash, and one register write or one fill
//! recolors the whole screen (TALON HD's palette tricks).
//!
//! ## Background
//! A vertical panorama, 40 cells wide and `BG_ROWS` cell rows tall, stored
//! as bitmap cell rows (320 bytes each) top to bottom, so the 25 rows a
//! frame shows are one contiguous block.
//!
//! ## Sprites
//! A sprite frame is stored four times, once per horizontal *phase*
//! (`x & 3`), because the REU copies bytes and a multicolor byte holds four
//! pixels. A frame may instead store only phase 0 (`PHASE_COUNT` 1); it is
//! then always drawn at `x & !3`. Huge boss frames use that to cost a quarter. Within a phase the pixels are **column-major**: each byte column
//! holds all `h` lines contiguously, matching the bitmap's layout where the
//! 8 lines of a cell are adjacent. A visible run of opaque bytes is thus a
//! chain of ≤8-byte DMAs whose REU source address simply auto-increments.
//!
//! Each phase is one contiguous **block**, laid out for the C64 to fetch with as few DMAs as possible
//! and scatter with the CPU (an I/O register write costs 16 RAM stores on the C64U, `docs/validation.md`):
//!
//! ```text
//! block:  band table: BANDS + 1 u24 band starts (relative to the block), then the bands
//! band:   WB + 1 u16 column offsets (relative to the band; entry WB is the band's end), then WB records
//! record: u8 item count, then items in line order:
//!           Run  line lo, line hi (bit 0), length, then `length` pixel bytes (fully opaque bytes)
//!           Mix  line lo, line hi | ITEM_MIX, sprite bits, keep-mask (dst = dst & keep | bits)
//! ```
//!
//! Bands are `1 << BAND_SHIFT` lines (32 for tall sprites, one band otherwise); runs never cross a band.
//! A sprite's on-screen columns of a band are one contiguous byte range, so drawing costs a record fetch,
//! then per visible band an offsets fetch and one data fetch per `CHUNK` (`render::draw_sprite`), and the
//! CPU kernel (`render::draw_cols`, `asm/blit.S`) copies the bytes into the bitmap's cell layout.
//! Holes cost nothing (data is inline per run).

pub const MAGIC: [u8; 8] = *b"TALONHD1";
pub const HEADER_LEN: u32 = 64;
/// The panorama frame table, right after the header: `BG_MASK + 1` u32 REU addresses, one per animation
/// frame of the level (each a whole panorama, `BG_ROWS` × 320 bytes); frame 0 is `BACKGROUND`.
pub const BG_TABLE: u32 = HEADER_LEN;
pub const MAX_SPRITE_SIZE: u16 = 512;
/// Byte columns of a 512-wide sprite at phase 3.
pub const MAX_COLUMNS: u16 = (MAX_SPRITE_SIZE + 3 + 3) / 4;
pub const REU_SIZE: u32 = 16 << 20;
/// A single REU command must not cross a 512 KiB bank.
pub const REU_BANK: u32 = 0x8_0000;
/// Largest single DMA the runtime issues. The CPU (and so the raster IRQ that
/// multiplexes the HUD sprites) is stalled for the whole transfer: 256 bytes
/// is ~70 µs (about a raster line), inside the IRQ's ~2-line slack (`hud`).
pub const MAX_DMA: u16 = 256;
/// HUD sprite blocks (64 bytes each) copied into both VIC banks at boot.
pub const HUD_BLOCKS: u16 = 48;

pub mod header {
    pub const MAGIC: usize = 0;
    /// u32: total bytes used by the image.
    pub const SIZE: usize = 8;
    /// u32: REU address of the background panorama: `BG_ROWS` bitmap cell
    /// rows of 320 bytes, top to bottom.
    pub const BACKGROUND: usize = 12;
    /// u8: `$D021` color (multicolor code 00).
    pub const D021: usize = 16;
    /// u8: border color.
    pub const BORDER: usize = 17;
    /// u16: number of sprite frames.
    pub const FRAMES: usize = 18;
    /// u32: REU address of the frame directory.
    pub const DIRECTORY: usize = 20;
    /// u8: global color RAM nibble (multicolor code 11).
    pub const COLOR: usize = 24;
    /// u8: global screen RAM byte (code 01 high nibble, code 10 low nibble).
    pub const SCREEN: usize = 25;
    /// u8: animated level: panorama frames − 1 (a power of two less one); 0 = one still panorama.
    pub const BG_MASK: usize = 26;
    /// u8: animated level: log2 of the ticks each panorama frame is shown.
    pub const BG_SHIFT: usize = 27;
    /// u32: simple additive checksum of bytes [HEADER_LEN, SIZE).
    pub const CHECKSUM: usize = 28;
    /// u16: background panorama height in cell rows (≥ 25).
    pub const BG_ROWS: usize = 32;
    /// u32: REU address of the HUD's static sprite blocks (`HUD_BLOCKS` × 64 bytes).
    pub const HUD: usize = 36;
    /// u32: REU address of the stage music, a DLT5 asset (`crate::audio`); 0 = none.
    pub const MUSIC: usize = 40;
    /// u32: REU address of the paged code phases (`crate::paging`): one `PHASE_SLOT` per id, in id order
    /// (the `BOOT` slot unused: the PRG carries that one); 0 = none.
    pub const PHASES: usize = 44;
    /// u8 × `paging::PHASES` (up to 16): each phase's length in 256-byte pages.
    pub const PHASE_LENS: usize = 48;
    /// u16: the phases' stamp (`paging::stamp`); the PRG carries the same one (patched in by the build).
    pub const PHASE_STAMP: usize = 34;
}

/// Frame directory record (one per frame, `FRAME_RECORD` bytes).
pub const FRAME_RECORD: u32 = 32;
pub mod frame {
    /// u16 width, u16 height.
    pub const W: usize = 0;
    pub const H: usize = 2;
    /// u8: stored phases, 4 (any x) or 1 (phase 0 only, drawn at `x & !3`).
    pub const PHASE_COUNT: usize = 4;
    /// u8: log2 of the band height in lines (`BAND_SHIFT_TALL` or `BAND_SHIFT_ONE`).
    pub const BAND_SHIFT: usize = 5;
    /// u8: number of bands.
    pub const BANDS: usize = 6;
    /// Four phase records (`PHASE_LEN` bytes each, see [`super::phase`]).
    pub const PHASES: usize = 8;
    pub const PHASE_LEN: usize = 6;
}
pub mod phase {
    /// u8: number of byte columns.
    pub const WB: usize = 0;
    /// u24: REU address of the phase's block.
    pub const BLOCK: usize = 1;
    /// u16: the block's length in bytes (`0xFFFF`: that or more).
    pub const LEN: usize = 4;
}
/// Bands of sprites taller than `BAND_LINES`: 32 lines, so a band of 40 on-screen columns fits `CHUNK`.
pub const BAND_SHIFT_TALL: u8 = 5;
/// Sprites up to this tall are one band.
pub const BAND_LINES: u16 = 32;
/// The band shift of a one-band sprite (any line >> 9 is band 0).
pub const BAND_SHIFT_ONE: u8 = 9;
/// Column record items: a Run is `line lo, line hi (bit 0), length (1..=255)` then its bytes; a Mix is
/// `line lo, line hi (bit 0) | ITEM_MIX, sprite bits (already masked), keep-mask for the destination`.
pub const ITEM_MIX: u8 = 0x80;
pub const RUN_HEADER: u16 = 3;
pub const MIX_LEN: u16 = 4;

#[inline(always)]
pub fn le16(b: &[u8], at: usize) -> u16 {
    b[at] as u16 | (b[at + 1] as u16) << 8
}
#[inline(always)]
pub fn le32(b: &[u8], at: usize) -> u32 {
    le24(b, at) | (b[at + 3] as u32) << 24
}
#[inline(always)]
pub fn le24(b: &[u8], at: usize) -> u32 {
    b[at] as u32 | (b[at + 1] as u32) << 8 | (b[at + 2] as u32) << 16
}
