//! Fixed C64 memory layout shared by the renderer, the asm kernel and the
//! host model. Vendored from gclass (`a5775bf`).
//!
//! | Region            | Buffer A (VIC bank 1) | Buffer B (VIC bank 2) |
//! |-------------------|-----------------------|-----------------------|
//! | screen RAM        | `$4C00`               | `$8C00`               |
//! | bitmap            | `$6000`               | `$A000`               |
//!
//! Both buffers use the same `$D018` value; only the `$DD00` bank changes.
//! Code, data and BSS live in `$0801-$3FFF`; the soft stack is `$CC00-$CFFF`.

/// Screen width in multicolor pixels / bytes per bitmap row of cells.
pub const WIDTH: i16 = 160;
pub const HEIGHT: i16 = 200;
pub const COLS: u16 = 40;
pub const ROWS: u16 = 25;
pub const BITMAP_BYTES: u16 = 8000;
pub const CELLS: u16 = 1000;
/// Bytes from one cell row to the next in the bitmap.
pub const ROW_STRIDE: u16 = 320;

/// One displayable frame buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Buffer {
    pub screen: u16,
    pub bitmap: u16,
    /// Low two bits written to `$DD00` to show this buffer.
    pub dd00: u8,
}

pub const BUFFERS: [Buffer; 2] = [
    Buffer { screen: 0x4C00, bitmap: 0x6000, dd00: 0b10 },
    Buffer { screen: 0x8C00, bitmap: 0xA000, dd00: 0b01 },
];

/// `$D018`: screen at bank+$0C00, bitmap at bank+$2000.
pub const D018: u8 = 0x38;
/// `$D016`: multicolor, 40 columns, XSCROLL 0: the window is the bitmap's full 320 dots, as wide as the
/// painted bands (user: no squeezed sides; the camera shake is vertical only).
pub const D016: u8 = 0x18;
/// The border painter's canvas (`asm/border.S`, from typo): the 24-row window (192 lines, raster 55..246,
/// hardware YSCROLL `7 - fine`) and 24 lines painted through `$D020` above it (raster 31..54, block T) and
/// below it (raster 247..7, block B), 240 in all. Screen line `y` (the game's coordinates, raster 55) is
/// canvas line `y + BAND_LINES` and bitmap line `y + fine`.
pub const BAND_LINES: i16 = 24;
pub const VISIBLE: i16 = 192;
pub const CANVAS: i16 = VISIBLE + 2 * BAND_LINES;
/// Cell rows composed per band: -3..=-1 above the bitmap and 25..=27 below it, so the painted lines
/// (bitmap lines `fine - 24 ..` and `fine + 192 ..`) always exist, with the bitmap's own hidden lines (row
/// 0's top, row 24's bottom).
pub const BAND_ROWS: u16 = 3;
pub const BAND_BYTES: u16 = BAND_ROWS * ROW_STRIDE;
/// Where the bands are composed while a frame is drawn: the top band's rows, then the bottom band's
/// (2 x `BAND_BYTES`), in the paging window's tail (the BANDS phase's data; `$9F80` is the end).
pub const BANDS: u16 = 0x9800;
/// REU: each buffer's painted lines (`render::stash_bands`), which the painter streams back while that
/// buffer is shown: above the phase slots (`paging::REU_PHASES`, 15 x 4 KiB). Line-major, the painter's
/// order: block T's 24 lines, then block B's, 40 bytes a line.
pub const BAND_REU: u32 = 0xFF_F000;
pub const BAND_SET: u32 = 0x800;
/// Bytes of one painted line, of a block, and of a band set.
pub const BAND_LINE: u16 = 40;
pub const BLOCK_BYTES: u16 = BAND_LINES as u16 * BAND_LINE;
pub const BAND_SET_BYTES: u16 = 2 * BLOCK_BYTES;
/// Where `render::stash_bands` lays a block's 4 source rows out line-major (`SEG_BUF`, free once the
/// sprites are drawn): 32 lines of 40 bytes, the block's 24 from line `fine`.
pub const BAND_LINES_BUF: u16 = SEG_BUF;
const _: () = assert!(BAND_SET_BYTES as u32 <= BAND_SET && BANDS + 2 * BAND_BYTES <= 0xA000 && 32 * BAND_LINE <= SEG_BUF_LEN);
/// Buffer `buf`'s band set in the REU.
pub const fn band_set(buf: Buffer) -> u32 {
    BAND_REU + if buf.bitmap == BUFFERS[0].bitmap { 0 } else { BAND_SET }
}
/// The `$D011` block B's flip writes (this frame's YSCROLL; `asm/border.S` `P_D011`).
pub const PAINTER_D011: u16 = 0xCB5F;
/// `asm/reu.S` `reu_transfer`: non-zero = the next piece loads every REU register (the painter's refill
/// sets it: it loads them itself); and the piece's length. Free bytes of the transpose job (`$CAD8`).
pub const REU_DIRTY: u16 = 0xCADE;
/// `asm/hud.S` `irq_clock_in`: CIA 2 timer A at the IRQ's entry (2 bytes).
pub const IRQ_T0: u16 = 0xCADA;
/// The border painter's block (`asm/border.S` `P_*`, `asm/hud.S`): parameters, state, diagnostics.
pub const PAINTER: u16 = 0xCAE0;
pub mod painter {
    /// u8: non-zero once the painter runs (`asm/hud.S` chains its blocks).
    pub const ON: u16 = 0;
    /// u8: bit 7 while no REU DMA may start (`asm/reu.S`, `asm/blit.S`).
    pub const GATE: u16 = 1;
    pub const NEXT: u16 = 2;
    /// u8: `$80 | buffer`: block B shows that buffer (bank, `$D016`, band set); 0 once done.
    pub const FLIP: u16 = 3;
    pub const D016: u16 = 4;
    /// u8: the shown buffer (whose band set block T and B are refilled from).
    pub const FRONT: u16 = 5;
    /// Calibration, poke-able live (typo's names): spin target, delay loop, 1-cycle trim, the turbo's
    /// `$D031`, timer A's phase offset and sync line; then the measured line offset.
    pub const TA: u16 = 6;
    pub const DL: u16 = 7;
    pub const DF: u16 = 8;
    pub const TURBO: u16 = 9;
    pub const AOFS: u16 = 10;
    pub const SYNC: u16 = 11;
    pub const DELTA: u16 = 12;
    /// The badline row's delay loop and trim (`DL + 6`, `DF + 6`); `LATEBLK` u16: blocks skipped (late).
    pub const DLB: u16 = 13;
    pub const DFB: u16 = 14;
    pub const LATEBLK: u16 = 15;
    /// u8: the border colour the painter puts in the corners (`hud_update` keeps it with `$D020`).
    pub const BORDER: u16 = 17;
    /// 7 × u8: the foreground's REU job (`asm/reu.S` `reu_transfer`): C64 address, REU address, length.
    pub const DMA_JOB: u16 = 21;
    /// 4 × u8: the colours of codes 00..11 for the decode tables (`border_tables`).
    pub const COL: u16 = 28;
    /// The staging (`asm/border.S` `STAGE`): one block's 24 lines, 6 a page (40 bytes each), 4 pages.
    pub const STAGE: u16 = 0x5800;
    /// Defaults for TA, DL, DF, TURBO, AOFS, SYNC (`asm/border.S` `defaults`): typo's TA/AOFS/SYNC; DL/DF
    /// calibrated on the C64U (firmware 1.1.0, FPGA 122, core 1.49; 2026-11-04): the bands on the window's
    /// x 32..351, every pixel 2 dots and in phase with the window's. The badline row's: DLB 71, DFB 4.
    pub const DEFAULTS: [u8; 6] = [37, 73, 2, 0x8F, 26, 100];
}

/// Per-column segment list for the column being drawn (≤ 512 × 4 bytes).
pub const SEG_BUF: u16 = 0xC000;
pub const SEG_BUF_LEN: u16 = 0x0800;
/// Job block read by the sprite kernel `draw_cols` (see [`job`]).
pub const JOB: u16 = 0xCA00;
/// 64-byte bounce buffer for header/frame/phase records fetched from the REU.
pub const RECORD: u16 = 0xCA20;
pub const RECORD_LEN: u16 = 0x0040;
/// The two VIC banks.
pub const VIC_BANK: [u16; 2] = [0x4000, 0x8000];
/// HUD sprite blocks: 48 × 64 bytes in each bank, same contents.
/// - Bank 2 at `$8000` (pointers 0..47). Not `$9000`: in VIC banks 0 and 2
///   the VIC sees the character ROM at bank + `$1000-$1FFF`.
/// - Bank 1 at `$5000` (pointers 64..111 = the bank-2 pointer | `$40`), so
///   bank 1's `$4000-$4BFF` stays program space.
pub const HUD_BLOCK_BASE: [u16; 2] = [0x5000, 0x8000];
/// What bank 1's pointer adds (ORs) to the table's bank-2 pointer.
pub const HUD_BANK1_POINTER: u8 = 0x40;
/// Sprite pointers of each buffer (screen RAM + `$3F8`).
pub const SPRITE_POINTERS: [u16; 2] = [0x4FF8, 0x8FF8];
/// The multiplexer's table, read by the raster IRQ (`asm/hud.S`); see
/// [`crate::hud::Table`] for the layout.
pub const HUD_TABLE: u16 = 0xCB00;
/// Per bank, the REU variant each live HUD slot holds (`hud::LIVE` bytes each; `render::hud_update`).
pub const HUD_SHOWN: u16 = HUD_TABLE + crate::hud::table::LEN + 1;

/// Status block the host verifier reads from a paused machine.
pub const STATUS: u16 = 0xCA80;
pub const COLOR_RAM: u16 = 0xD800;

/// Offsets inside the [`JOB`] block. All values little-endian.
pub mod job {
    /// u16: back-buffer address of the first column's top cell (bitmap + col*8).
    pub const DST: u16 = 0;
    /// i16: screen y of sprite line 0.
    pub const Y: u16 = 2;
    /// u16: first visible sprite line.
    pub const TOP: u16 = 4;
    /// u16: end (exclusive) of visible sprite lines.
    pub const BOT: u16 = 6;
    /// u16: RAM address of the first column record.
    pub const SRC: u16 = 8;
    /// u8: column records to draw (consecutive, each 8 bytes right of the last).
    pub const NCOLS: u16 = 10;
}

/// The sprite renderer's RAM: band data fetched per chunk into the segment buffer (sharing its bytes
/// with the music's staging and the attract pilot's Mind, which are not live while rendering), one band's
/// column offsets after it, and the small-sprite cache where the old column table was ($E000-$E40F, RAM
/// under the KERNAL ROM; persists across frames, nothing else writes it after boot).
pub const CHUNK: u16 = SEG_BUF;
pub const CHUNK_LEN: u16 = 0x0700;
pub const OFFS: u16 = SEG_BUF + 0x0700;
pub const SPRITE_CACHE: u16 = 0xE000;
/// A power of two (the slot is picked with a mask).
pub const SPRITE_CACHE_SLOTS: u16 = 8;
pub const SPRITE_CACHE_SLOT: u16 = 0x82;
const _: () = assert!(SPRITE_CACHE_SLOTS.is_power_of_two() && SPRITE_CACHE_SLOTS * SPRITE_CACHE_SLOT <= 0x0410);

/// Offsets inside the [`STATUS`] block.
pub mod status {
    /// u8: magic `$C1` once the main loop runs.
    pub const MAGIC: u16 = 0;
    /// u8: non-zero while a flip is in progress (front state inconsistent).
    pub const FLIPPING: u16 = 1;
    /// u8: index of the buffer currently displayed.
    pub const FRONT: u16 = 2;
    /// u8: the last level this C64 can reach (`Clear::top`: `clear::LAST_LEVEL` when it loads the levels'
    /// images, else its booted level), for `compare`'s replay.
    pub const TOP: u16 = 3;
    /// u16: demo time of the displayed frame.
    pub const SHOWN_T: u16 = 4;
    /// u16: microseconds (CIA ticks) spent rendering the last frame.
    pub const RENDER_US: u16 = 6;
    /// u8: set by the host to freeze demo time (0 = run).
    pub const HOLD: u16 = 8;
    /// u16: microseconds of RENDER_US spent restoring the background.
    pub const BACKGROUND_US: u16 = 10;
    /// u16 (feature `timing`): microseconds of the last frame's music service (decode and stash; the page
    /// is `AUDIO_PAGE_US`).
    pub const AUDIO_US: u16 = 12;
    /// u16 (feature `timing`): microseconds of the last game step (input, the attract pilot, the sim, the
    /// boss/midboss/ground steps, all their paging).
    pub const STEP_US: u16 = 14;
    /// 3 × u16 (feature `profile`): sprite time in setup / column overhead /
    /// column kernel, in µs.
    pub const PROFILE_US: u16 = 16;
    /// u16 (feature `timing`): microseconds the main loop waited for the painter's flip, IRQs left out (the
    /// frame's slack).
    pub const WAIT_US: u16 = 22;
    /// u16, free-running: microseconds spent in raster IRQs (the HUD rows and the painted blocks;
    /// `asm/hud.S` `irq_clock_*`). The timing laps leave it out.
    pub const IRQ_US: u16 = 24;
    /// u16 (feature `timing`): microseconds of the step's ACE slice (Mind DMAs, its phase page, the slice).
    pub const ACE_US: u16 = 26;
    /// u16 (feature `timing`): microseconds of `render::hud_update`.
    pub const HUD_US: u16 = 29;
    /// u8, bit 0: the border painter wanted (the 160×240 canvas; off = plain borders, 60 fps): 0 at boot,
    /// F1 flips it, tools poke it (`c64u.py canvas on|off`, emu64 `--painter`).
    pub const CANVAS: u16 = 31;
    /// Nonzero once a player has touched the stick (the frame is no longer the attract-mode replay).
    pub const HUMAN: u16 = 28;
}

pub const STATUS_MAGIC: u8 = 0xC1;

/// Byte offset of screen line `y` (0..199) within a bitmap column.
#[inline(always)]
pub const fn line_offset(y: u16) -> u16 {
    (y >> 3) * ROW_STRIDE + (y & 7)
}
