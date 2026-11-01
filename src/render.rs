//! The renderer (vendored from gclass `a5775bf`, adapted for TALON HD).
//! Generic over [`Hw`], so the exact code that runs on the C64 is what the
//! host tests execute against the REU model.
//!
//! Per frame: DMA the 25 visible cell rows of the vertical panorama into the
//! back buffer (one contiguous block), sort sprites by z, then paint each
//! sprite back to front:
//! 1. Fetch the column table entries for the on-screen byte columns.
//! 2. Per column, fetch its segment list and run [`Hw::draw_column`]:
//!    opaque runs become chained ≤8-byte DMAs (split at cell rows), mixed
//!    bytes are blended by the CPU.
//!
//! All four multicolor codes are global ([`Palette`]), so nothing ever
//! writes cell attributes per frame and sprites can't clash.
//!
//! [`present`] then shows the buffer; call it during vertical blank.

use crate::format::{self, column, frame, header, le16, le24, phase};
use crate::hw::{self, Hw};
use crate::hud;
use crate::layout::{self, Buffer, job};

/// One sprite instance in a scene. `y` is a screen line: 0 is the first
/// visible line.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Sprite {
    pub frame: u16,
    pub x: i16,
    pub y: i16,
    /// Larger z is drawn later (in front). Ties keep list order.
    pub z: i16,
}

/// The four global colors behind multicolor codes 00, 01, 10 and 11.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Palette {
    /// Code 00: `$D021`.
    pub d021: u8,
    /// Codes 01 (high nibble) and 10 (low nibble): every screen RAM byte.
    pub screen: u8,
    /// Code 11: every color RAM nibble.
    pub color: u8,
}

impl Palette {
    pub const fn new(c00: u8, c01: u8, c10: u8, c11: u8) -> Palette {
        Palette { d021: c00 & 15, screen: (c01 & 15) << 4 | (c10 & 15), color: c11 & 15 }
    }
    /// The C64 color shown by multicolor `code`.
    pub fn color_of(&self, code: u8) -> u8 {
        match code & 3 {
            0 => self.d021,
            1 => self.screen >> 4,
            2 => self.screen & 15,
            _ => self.color,
        }
    }
}

/// Parsed REU image header.
#[derive(Clone, Copy, Debug, Default)]
pub struct Image {
    pub background: u32,
    /// Background panorama height in cell rows.
    pub bg_rows: u16,
    pub palette: Palette,
    pub border: u8,
    pub frames: u16,
    pub directory: u32,
    /// REU address of the HUD's static sprite blocks.
    pub hud: u32,
}

fn record<H: Hw + ?Sized, const N: usize>(hw: &mut H, addr: u32) -> [u8; N] {
    let mut out = [0u8; N];
    hw::read_into(hw, addr, layout::RECORD, &mut out);
    out
}

#[inline]
fn peek16<H: Hw + ?Sized>(hw: &mut H, addr: u16) -> u16 {
    hw.peek(addr) as u16 | (hw.peek(addr + 1) as u16) << 8
}

#[inline]
fn poke16<H: Hw + ?Sized>(hw: &mut H, addr: u16, value: u16) {
    hw.poke(addr, value as u8);
    hw.poke(addr + 1, (value >> 8) as u8);
}

/// Read and validate the image header at REU 0.
pub fn read_image<H: Hw + ?Sized>(hw: &mut H) -> Option<Image> {
    hw::init(hw);
    let h: [u8; 48] = record(hw, 0);
    if h[..8] != format::MAGIC {
        return None;
    }
    Some(Image {
        background: le24(&h, header::BACKGROUND),
        bg_rows: le16(&h, header::BG_ROWS),
        palette: Palette { d021: h[header::D021], screen: h[header::SCREEN], color: h[header::COLOR] },
        border: h[header::BORDER],
        frames: le16(&h, header::FRAMES),
        directory: le24(&h, header::DIRECTORY),
        hud: le24(&h, header::HUD),
    })
}

/// Show `pal`: one `$D021` write, a fill of both buffers' screen RAM and of
/// color RAM. Neither is double-buffered, so call it in vertical blank
/// (palette effects: flashes, boss phases, stage transitions).
pub fn set_palette<H: Hw + ?Sized>(hw: &mut H, pal: Palette) {
    hw.poke(0xD021, pal.d021);
    for b in layout::BUFFERS {
        hw.fill(b.screen, layout::CELLS, pal.screen);
    }
    hw.fill(layout::COLOR_RAM, layout::CELLS, pal.color);
}

/// Copy the HUD's static sprite blocks into both VIC banks and set up the 8
/// sprites (hires, unexpanded, in front, X positions). Call once at boot.
#[inline(never)]
pub fn hud_init<H: Hw + ?Sized>(hw: &mut H, img: &Image) {
    for base in layout::HUD_BLOCK_BASE {
        hw::fetch(hw, img.hud, base, format::HUD_BLOCKS * hud::BLOCK as u16);
    }
    let mut msb = 0u8;
    for (i, &x) in hud::X.iter().enumerate() {
        hw.poke(0xD000 + 2 * i as u16, x as u8);
        if x > 255 {
            msb |= 1 << i;
        }
    }
    hw.poke(0xD010, msb);
    hw.poke(0xD017, 0);
    hw.poke(0xD01B, 0);
    hw.poke(0xD01C, 0);
    hw.poke(0xD01D, 0);
    hw.poke(0xD015, 0xFF);
}

/// Per-frame HUD update (call in vertical blank): the multiplexer table
/// (not its row counter, which the IRQ owns), the 8 colors and the dynamic
/// blocks in both banks.
#[inline(never)]
pub fn hud_update<H: Hw + ?Sized>(hw: &mut H, s: &hud::State) {
    let t = hud::build(s);
    let base = layout::HUD_TABLE;
    for r in 0..hud::ROWS {
        hw.poke(base + hud::table::LINES + r as u16, t.lines[r]);
        hw.poke(base + hud::table::YS + r as u16, t.ys[r]);
        for c in 0..8 {
            hw.poke(base + hud::table::PTRS + (r * 8 + c) as u16, t.ptrs[r][c]);
        }
    }
    for (i, &c) in hud::colors(s).iter().enumerate() {
        hw.poke(0xD027 + i as u16, c);
    }
    let mut blocks = [[0u8; hud::BLOCK]; hud::block::DYNAMIC_COUNT];
    hud::dynamic_blocks(s, &mut blocks);
    for bank in layout::HUD_BLOCK_BASE {
        let mut a = bank + hud::block::DYNAMIC as u16 * hud::BLOCK as u16;
        for b in &blocks {
            for (k, &v) in b.iter().enumerate().take(63) {
                hw.poke(a + k as u16, v);
            }
            a += hud::BLOCK as u16;
        }
    }
}

/// Stable insertion sort by z (few sprites; no allocation).
pub fn sort_by_z(sprites: &mut [Sprite]) {
    for i in 1..sprites.len() {
        let s = sprites[i];
        let mut j = i;
        while j > 0 && sprites[j - 1].z > s.z {
            sprites[j] = sprites[j - 1];
            j -= 1;
        }
        sprites[j] = s;
    }
}

/// Vertical scroll: panorama line `scroll` is the first visible line.
///
/// The frame copies cell rows `coarse..coarse+25` into the bitmap. In 24-row
/// mode with YSCROLL `7 - fine` the VIC shows bitmap lines
/// `fine..fine+192`, so screen line `y` is bitmap line `y + fine` and
/// panorama line `scroll + y`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Scroll {
    pub coarse: u16,
    pub fine: u8,
}

impl Scroll {
    pub fn new(img: &Image, scroll: u16) -> Scroll {
        let max = (img.bg_rows.max(layout::ROWS) - layout::ROWS) * 8;
        let s = scroll.min(max);
        Scroll { coarse: s >> 3, fine: (s & 7) as u8 }
    }
    /// `$D011`: bitmap mode, display on, 24 rows, YSCROLL.
    pub fn d011(self) -> u8 {
        0x30 | (7 - self.fine)
    }
}

/// DMA the visible window of the panorama: 25 contiguous cell rows.
pub fn restore_background<H: Hw + ?Sized>(hw: &mut H, img: &Image, buf: Buffer, scroll: Scroll) {
    let src = img.background + scroll.coarse as u32 * layout::ROW_STRIDE as u32;
    hw::fetch(hw, src, buf.bitmap, layout::BITMAP_BYTES);
}

/// Render a whole frame into `buf`. Sorts `sprites` in place. Sprites are in
/// screen coordinates; they are drawn `scroll.fine` lines lower in the bitmap
/// so the display's YSCROLL leaves them where the scene put them.
pub fn render_frame<H: Hw + ?Sized>(hw: &mut H, img: &Image, buf: Buffer, scroll: Scroll, sprites: &mut [Sprite]) {
    restore_background(hw, img, buf, scroll);
    sort_by_z(sprites);
    for s in sprites.iter() {
        let mut s = *s;
        s.y += scroll.fine as i16;
        draw_sprite(hw, img, buf, &s);
    }
}

/// Show `buf` with its fine scroll. Call in vblank.
pub fn present<H: Hw + ?Sized>(hw: &mut H, buf: Buffer, scroll: Scroll) {
    let dd00 = hw.peek(0xDD00) & 0xFC | buf.dd00;
    hw.poke(0xDD00, dd00);
    hw.poke(0xD011, scroll.d011());
}

/// Draw one sprite at bitmap coordinates (`y` is a bitmap line).
pub fn draw_sprite<H: Hw + ?Sized>(hw: &mut H, img: &Image, buf: Buffer, s: &Sprite) {
    if s.frame >= img.frames {
        return;
    }
    let rec_addr = img.directory + s.frame as u32 * format::FRAME_RECORD;
    let f: [u8; 8] = record(hw, rec_addr);
    let h = le16(&f, frame::H) as i16;

    // A 1-phase frame only has phase 0: it lands on x & !3.
    let p = if f[frame::PHASE_COUNT] == 1 { 0 } else { (s.x & 3) as u32 };
    let base = s.x >> 2;
    let ph: [u8; 16] = record(hw, rec_addr + (frame::PHASES as u32) + p * frame::PHASE_LEN as u32);
    let wb = ph[phase::WB] as i16;

    // Horizontal clip: byte columns [j0, j1) are on screen.
    let j0 = if base < 0 { -base } else { 0 };
    let j1 = wb.min(layout::COLS as i16 - base);
    if j0 >= j1 {
        return;
    }
    // Vertical clip: sprite lines [top, bot) are on screen. The screen is a
    // whole number of cell rows, so clipped lines never split a cell row.
    let top = if s.y < 0 { -s.y } else { 0 };
    let bot = h.min(layout::HEIGHT - s.y);
    if top >= bot {
        return;
    }

    hw.mark(0);
    // Pixels, one column at a time.
    let ncols = (j1 - j0) as u16;
    let cols = le24(&ph, phase::COLUMNS) + j0 as u32 * format::COLUMN_ENTRY as u32;
    hw::fetch(hw, cols, layout::COL_TABLE, ncols * format::COLUMN_ENTRY);
    poke16(hw, layout::JOB + job::Y, s.y as u16);
    poke16(hw, layout::JOB + job::TOP, top as u16);
    poke16(hw, layout::JOB + job::BOT, bot as u16);
    let mut entry = layout::COL_TABLE;
    let mut dst = buf.bitmap.wrapping_add(((base + j0) as u16) << 3);
    for _ in 0..ncols {
        let count = peek16(hw, entry + column::COUNT as u16);
        if count != 0 {
            let segs = hw.peek(entry + 3) as u32
                | (hw.peek(entry + 4) as u32) << 8
                | (hw.peek(entry + 5) as u32) << 16;
            hw::fetch(hw, segs, layout::SEG_BUF, count * format::SEGMENT_LEN);
            poke16(hw, layout::JOB + job::DST, dst);
            for i in 0..3 {
                let b = hw.peek(entry + column::DATA as u16 + i);
                hw.poke(layout::JOB + job::DATA + i, b);
            }
            poke16(hw, layout::JOB + job::COUNT, count);
            hw.mark(1);
            hw.draw_column();
            hw.mark(2);
        }
        entry += format::COLUMN_ENTRY;
        dst = dst.wrapping_add(8);
    }
}

/// Reference column kernel (the C64 build normally uses `asm/blit.S`).
pub fn draw_column<H: Hw + ?Sized>(hw: &mut H) {
    let dst = peek16(hw, layout::JOB + job::DST);
    let y = peek16(hw, layout::JOB + job::Y);
    let top = peek16(hw, layout::JOB + job::TOP);
    let bot = peek16(hw, layout::JOB + job::BOT);
    let data = hw.peek(layout::JOB + job::DATA) as u32
        | (hw.peek(layout::JOB + job::DATA + 1) as u32) << 8
        | (hw.peek(layout::JOB + job::DATA + 2) as u32) << 16;
    let count = peek16(hw, layout::JOB + job::COUNT);
    let mut seg = layout::SEG_BUF;
    for _ in 0..count {
        let b0 = hw.peek(seg);
        let b1 = hw.peek(seg + 1);
        let b2 = hw.peek(seg + 2);
        let b3 = hw.peek(seg + 3);
        seg += format::SEGMENT_LEN;
        let line = b0 as u16 | ((b1 & 1) as u16) << 8;
        if line >= bot {
            break;
        }
        if b1 & format::SEG_MIX != 0 {
            if line >= top {
                let a = dst.wrapping_add(layout::line_offset(y.wrapping_add(line)));
                let v = hw.peek(a) & b3 | b2;
                hw.poke(a, v);
            }
        } else {
            let len = b2 as u16 | (b3 as u16) << 8;
            let s = line.max(top);
            let e = (line + len).min(bot);
            if s < e {
                hw::set_reu_addr(hw, data + s as u32);
                let mut sl = y.wrapping_add(s);
                let mut rem = e - s;
                while rem != 0 {
                    let n = (8 - (sl & 7)).min(rem);
                    hw::chain_fetch(hw, dst.wrapping_add(layout::line_offset(sl)), n);
                    sl += n;
                    rem -= n;
                }
            }
        }
    }
}
