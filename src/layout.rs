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
/// `$D016` without XSCROLL: multicolor, 38 columns. The narrower window hides the bitmap's edges, so the
/// camera shake can move XSCROLL either way around its rest position.
pub const D016: u8 = 0x10;
/// XSCROLL at rest: the shake moves it -4..=+3 hires px.
pub const XSCROLL_REST: u8 = 4;
/// Lines of the bitmap the VIC shows in 24-row mode: bitmap lines
/// `fine..fine + VISIBLE` (see [`crate::render::Scroll`]).
pub const VISIBLE: i16 = 192;

/// Per-column segment list for the column being drawn (≤ 512 × 4 bytes).
pub const SEG_BUF: u16 = 0xC000;
pub const SEG_BUF_LEN: u16 = 0x0800;
/// Column job block read by the `draw_column` kernel (see [`job`]).
pub const JOB: u16 = 0xCA00;
/// 64-byte bounce buffer for header/frame/phase records fetched from the REU.
pub const RECORD: u16 = 0xCA20;
pub const RECORD_LEN: u16 = 0x0040;
/// Column table for the visible columns of the current sprite (≤ 129 × 8).
/// (gclass had it at `$8000`; TALON HD keeps `$8000-$8BFF` for HUD sprites.)
pub const COL_TABLE: u16 = 0xE000;
pub const COL_TABLE_LEN: u16 = 0x0410;
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

/// Status block the host verifier reads from a paused machine.
pub const STATUS: u16 = 0xCA80;
pub const COLOR_RAM: u16 = 0xD800;

/// Offsets inside the [`JOB`] block. All values little-endian.
pub mod job {
    /// u16: back-buffer address of this column's top cell (bitmap + col*8).
    pub const DST: u16 = 0;
    /// i16: screen y of sprite line 0.
    pub const Y: u16 = 2;
    /// u16: first visible sprite line.
    pub const TOP: u16 = 4;
    /// u16: end (exclusive) of visible sprite lines.
    pub const BOT: u16 = 6;
    /// u24: REU address of this column's pixel bytes (line 0).
    pub const DATA: u16 = 8;
    /// u16: number of 4-byte segments in [`super::SEG_BUF`].
    pub const COUNT: u16 = 11;
}

/// Offsets inside the [`STATUS`] block.
pub mod status {
    /// u8: magic `$C1` once the main loop runs.
    pub const MAGIC: u16 = 0;
    /// u8: non-zero while a flip is in progress (front state inconsistent).
    pub const FLIPPING: u16 = 1;
    /// u8: index of the buffer currently displayed.
    pub const FRONT: u16 = 2;
    /// u16: demo time of the displayed frame.
    pub const SHOWN_T: u16 = 4;
    /// u16: microseconds (CIA ticks) spent rendering the last frame.
    pub const RENDER_US: u16 = 6;
    /// u8: set by the host to freeze demo time (0 = run).
    pub const HOLD: u16 = 8;
    /// u16: microseconds of RENDER_US spent restoring the background.
    pub const BACKGROUND_US: u16 = 10;
    /// 3 × u16 (feature `profile`): sprite time in setup / column overhead /
    /// column kernel, in µs.
    pub const PROFILE_US: u16 = 16;
    /// Nonzero once a player has touched the stick (the frame is no longer the attract-mode replay).
    pub const HUMAN: u16 = 28;
}

pub const STATUS_MAGIC: u8 = 0xC1;

/// Byte offset of screen line `y` (0..199) within a bitmap column.
#[inline(always)]
pub const fn line_offset(y: u16) -> u16 {
    (y >> 3) * ROW_STRIDE + (y & 7)
}
