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
//! Each column has a sorted segment list (4 bytes per segment):
//!
//! | byte | Run (b1 bits 7,6 clear) | Mix (bit 7 set)              | Skip (bit 6 set)       |
//! |------|-------------------------|------------------------------|------------------------|
//! | b0   | line bits 0-7           | line bits 0-7                | line bits 0-7          |
//! | b1   | line bit 8 (bit 0)      | line bit 8 + `$80`           | line bit 8 + `$40`     |
//! | b2   | run length lo           | sprite bits (already masked) | gap lo                 |
//! | b3   | run length hi           | keep-mask for the destination| gap hi                 |
//!
//! Runs cover fully opaque bytes (pure DMA); Mix entries cover bytes with
//! both opaque and transparent pixels (CPU: `dst = dst & keep | bits`).
//! A run's REU source is `DATA + line`. The column's data stores only run
//! bytes: a Skip, placed just before the run that follows a gap, subtracts
//! the gap's length from `DATA`, so gaps between runs (a sprite's holes, the
//! space between a boss's legs) cost 4 bytes instead of their length.

pub const MAGIC: [u8; 8] = *b"TALONHD1";
pub const HEADER_LEN: u32 = 64;
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
    /// u32: simple additive checksum of bytes [HEADER_LEN, SIZE).
    pub const CHECKSUM: usize = 28;
    /// u16: background panorama height in cell rows (≥ 25).
    pub const BG_ROWS: usize = 32;
    /// u32: REU address of the HUD's static sprite blocks (`HUD_BLOCKS` × 64 bytes).
    pub const HUD: usize = 36;
}

/// Frame directory record.
pub const FRAME_RECORD: u32 = 80;
pub mod frame {
    /// u16 width, u16 height.
    pub const W: usize = 0;
    pub const H: usize = 2;
    /// u8: stored phases, 4 (any x) or 1 (phase 0 only, drawn at `x & !3`).
    pub const PHASE_COUNT: usize = 4;
    /// Four phase records follow.
    pub const PHASES: usize = 8;
    pub const PHASE_LEN: usize = 16;
}
pub mod phase {
    /// u8: number of byte columns.
    pub const WB: usize = 0;
    /// u32: REU address of the column table (COLUMN_ENTRY bytes per column).
    pub const COLUMNS: usize = 8;
}
pub const COLUMN_ENTRY: u16 = 8;
pub mod column {
    /// u24: REU address of the column's pixel bytes (line 0).
    pub const DATA: usize = 0;
    /// u24: REU address of the column's segment list.
    pub const SEGS: usize = 3;
    /// u16: segment count.
    pub const COUNT: usize = 6;
}
pub const SEGMENT_LEN: u16 = 4;
pub const SEG_MIX: u8 = 0x80;
pub const SEG_SKIP: u8 = 0x40;
/// Gaps shorter than this are stored as data (a Skip costs 4 bytes).
pub const MIN_SKIP: usize = 8;

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
