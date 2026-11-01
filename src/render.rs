//! The renderer (vendored from gclass `a5775bf`, adapted for TALON HD).
//! Generic over [`Hw`], so the exact code that runs on the C64 is what the
//! host tests execute against the REU model.
//!
//! Per frame: DMA the 25 visible cell rows of the vertical panorama into the
//! back buffer (one contiguous block), sort sprites by z, then paint each
//! sprite back to front ([`draw_sprite`]): its on-screen columns are fetched
//! a band at a time with as few DMAs as possible and scattered into the
//! bitmap by the CPU ([`Hw::draw_cols`]); small sprites come from a RAM cache.
//!
//! All four multicolor codes are global ([`Palette`]), so nothing ever
//! writes cell attributes per frame and sprites can't clash.
//!
//! [`present`] then shows the buffer; call it during vertical blank.

use crate::format::{self, frame, header, le16, le24, le32, phase};
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
    /// REU address of the panorama shown now (`background_at` picks the animation frame).
    pub background: u32,
    /// Animation frames − 1 and log2 ticks per frame (`format::header::BG_MASK`, `BG_SHIFT`).
    pub bg_mask: u8,
    pub bg_shift: u8,
    /// Background panorama height in cell rows.
    pub bg_rows: u16,
    pub palette: Palette,
    pub border: u8,
    pub frames: u16,
    pub directory: u32,
    /// REU address of the HUD's static sprite blocks.
    pub hud: u32,
    /// REU address of the stage music (DLT5), 0 = none.
    pub music: u32,
    /// REU address of the paged code phases, their lengths and stamp (`format::header::PHASES`).
    pub phases: u32,
    pub phase_lens: [u8; crate::paging::PHASES as usize],
    pub phase_stamp: u16,
    /// Image size and the additive checksum of bytes `HEADER_LEN..size`.
    pub size: u32,
    pub checksum: u32,
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
#[inline(never)]
pub fn read_image<H: Hw + ?Sized>(hw: &mut H) -> Option<Image> {
    hw::init(hw);
    let h: [u8; 64] = record(hw, 0);
    if h[..8] != format::MAGIC {
        return None;
    }
    Some(Image {
        background: le24(&h, header::BACKGROUND),
        bg_mask: h[header::BG_MASK],
        bg_shift: h[header::BG_SHIFT],
        bg_rows: le16(&h, header::BG_ROWS),
        palette: Palette { d021: h[header::D021], screen: h[header::SCREEN], color: h[header::COLOR] },
        border: h[header::BORDER],
        frames: le16(&h, header::FRAMES),
        directory: le24(&h, header::DIRECTORY),
        hud: le24(&h, header::HUD),
        music: le24(&h, header::MUSIC),
        phases: le24(&h, header::PHASES),
        phase_lens: {
            let mut l = [0u8; crate::paging::PHASES as usize];
            for (i, v) in l.iter_mut().enumerate() {
                *v = h[header::PHASE_LENS + i];
            }
            l
        },
        phase_stamp: le16(&h, header::PHASE_STAMP),
        size: le32(&h, header::SIZE),
        checksum: le32(&h, header::CHECKSUM),
    })
}

/// Erase the image magic in the REU. The PRG does this at boot, so an image
/// left over from an earlier upload can't be mistaken for the one about to
/// arrive (the device uploads after starting the PRG).
#[inline(never)]
pub fn invalidate_image<H: Hw + ?Sized>(hw: &mut H) {
    hw::init(hw);
    hw.fill(layout::RECORD, 8, 0);
    hw::stash(hw, layout::RECORD, 0, 8);
}

/// True when the whole image has arrived: the additive checksum over
/// `HEADER_LEN..size` matches the header. Reads through `SEG_BUF` in
/// `MAX_DMA` chunks.
#[inline(never)]
pub fn verify_image<H: Hw + ?Sized>(hw: &mut H, img: &Image) -> bool {
    if img.size <= format::HEADER_LEN || img.size > format::REU_SIZE {
        return false;
    }
    let mut sum = 0u32;
    let mut addr = format::HEADER_LEN;
    while addr < img.size {
        let n = (img.size - addr).min(format::MAX_DMA as u32) as u16;
        hw::fetch(hw, addr, layout::SEG_BUF, n);
        for i in 0..n {
            sum = sum.wrapping_add(hw.peek(layout::SEG_BUF + i) as u32);
        }
        addr += n as u32;
    }
    sum == img.checksum
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
/// sprites (hires, unexpanded, in front, green). Call once at boot.
/// On the C64 these boot-only functions are the `BOOT` paged phase (`crate::paging`).
#[inline(never)]
pub fn hud_init<H: Hw + ?Sized>(hw: &mut H, img: &Image) {
    for base in layout::HUD_BLOCK_BASE {
        hw::fetch(hw, img.hud, base, format::HUD_BLOCKS * hud::BLOCK as u16);
    }
    // Colors are fixed (the damage flash is on the border); X positions follow the shake in hud_update.
    for (i, &c) in hud::COLORS.iter().enumerate() {
        hw.poke(0xD027 + i as u16, c);
    }
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
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".hitext"))]
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
    let mut msb = 0u8;
    for (i, &x) in hud::xs(s).iter().enumerate() {
        hw.poke(0xD000 + 2 * i as u16, x as u8);
        if x > 255 {
            msb |= 1 << i;
        }
    }
    hw.poke(0xD010, msb);
    hw.poke(0xD020, hud::border(s));
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
    /// `$D016` XSCROLL (0-7 hires px): `layout::XSCROLL_REST` at rest; the camera shake moves it.
    pub xs: u8,
    /// Whole 8-px columns the background is shifted right (camera shake beyond XSCROLL's reach); the
    /// columns that wrap in at the edge are blanked to code 00.
    pub col: i8,
}

impl Scroll {
    pub fn new(img: &Image, scroll: u16) -> Scroll {
        let max = (img.bg_rows.max(layout::ROWS) - layout::ROWS) * 8;
        let s = scroll.min(max);
        Scroll { coarse: s >> 3, fine: (s & 7) as u8, xs: layout::XSCROLL_REST, col: 0 }
    }
    /// `$D016`: multicolor, 38 columns (the border hides the XSCROLL gap), XSCROLL.
    pub fn d016(self) -> u8 {
        layout::D016 | (self.xs & 7)
    }
    /// `$D011`: bitmap mode, display on, 24 rows, YSCROLL.
    pub fn d011(self) -> u8 {
        0x30 | (7 - self.fine)
    }
}

/// The REU address of the panorama frame for tick `t` (its low byte): an animated level is a loop of
/// whole panorama copies, frame `(t >> bg_shift) & bg_mask`, listed in `format::BG_TABLE`.
pub fn background_at<H: Hw + ?Sized>(hw: &mut H, img: &Image, t: u8) -> u32 {
    let k = (t >> img.bg_shift) & img.bg_mask;
    let e: [u8; 4] = record(hw, format::BG_TABLE + 4 * k as u32);
    le24(&e, 0)
}

/// DMA the visible window of the panorama: 25 contiguous cell rows.
/// With `scroll.col` the copy starts `8 * col` bytes earlier (one cell column per 8 bytes), so every
/// cell row moves right by `col` columns; the columns that come from the neighbouring cell row are
/// blanked (left edge for `col > 0`, right edge for `col < 0`).
pub fn restore_background<H: Hw + ?Sized>(hw: &mut H, img: &Image, buf: Buffer, scroll: Scroll) {
    let src = img.background + scroll.coarse as u32 * layout::ROW_STRIDE as u32;
    let shift = 8 * scroll.col as i32;
    hw::fetch(hw, src.wrapping_sub(shift as u32), buf.bitmap, layout::BITMAP_BYTES);
    if shift != 0 {
        let n = shift.unsigned_abs() as u16;
        let edge = if shift > 0 { 0 } else { layout::ROW_STRIDE - n };
        for r in 0..layout::ROWS {
            hw.fill(buf.bitmap + r * layout::ROW_STRIDE + edge, n, 0);
        }
    }
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
    hw.poke(0xD016, scroll.d016());
}

/// Small-sprite cache slot layout: tag (`frame + 1`, u16; 0 = empty), x phase, byte columns, height
/// (u16), then the block. One-band blocks up to `SLOT_BLOCK` bytes are kept (bullets, shots, SPARKs), so
/// drawing them again costs no REU access at all.
const SLOT_META: u16 = 8;
const SLOT_BLOCK: u16 = layout::SPRITE_CACHE_SLOT - SLOT_META;

/// Forget every cached small sprite (call once after boot: the cache's RAM held anything before).
pub fn reset_sprite_cache<H: Hw + ?Sized>(hw: &mut H) {
    for i in 0..layout::SPRITE_CACHE_SLOTS {
        let slot = layout::SPRITE_CACHE + i * layout::SPRITE_CACHE_SLOT;
        hw.poke(slot, 0);
        hw.poke(slot + 1, 0);
    }
}

/// Draw one sprite at bitmap coordinates (`y` is a bitmap line).
///
/// Small one-band sprites are drawn from the RAM cache (fetched once). Others: the frame record, then
/// for each band with visible lines its column offsets and, one `CHUNK` at a time, the on-screen
/// columns' records; the CPU kernel ([`Hw::draw_cols`]) scatters them into the bitmap.
#[inline(never)]
pub fn draw_sprite<H: Hw + ?Sized>(hw: &mut H, img: &Image, buf: Buffer, s: &Sprite) {
    if s.frame >= img.frames {
        return;
    }
    let xp = (s.x & 3) as u16;
    let slot = layout::SPRITE_CACHE
        + ((s.frame.wrapping_mul(4) + xp) & (layout::SPRITE_CACHE_SLOTS - 1)) * layout::SPRITE_CACHE_SLOT;
    let tag = s.frame + 1;
    let (wb, h, block_ram, block, shift) = if peek16(hw, slot) == tag && hw.peek(slot + 2) == xp as u8 {
        (hw.peek(slot + 3) as i16, peek16(hw, slot + 4) as i16, slot + SLOT_META, 0u32, format::BAND_SHIFT_ONE)
    } else {
        let f: [u8; 32] = record(hw, img.directory + s.frame as u32 * format::FRAME_RECORD);
        // A 1-phase frame only has phase 0: it lands on x & !3.
        let p = if f[frame::PHASE_COUNT] == 1 { 0 } else { xp as usize };
        let ph = frame::PHASES + p * frame::PHASE_LEN;
        let wb = f[ph + phase::WB] as i16;
        let h = le16(&f, frame::H) as i16;
        let block = le24(&f, ph + phase::BLOCK);
        let len = le16(&f, ph + phase::LEN);
        if f[frame::BANDS] == 1 && len <= SLOT_BLOCK {
            hw::fetch(hw, block, slot + SLOT_META, len);
            poke16(hw, slot, tag);
            hw.poke(slot + 2, xp as u8);
            hw.poke(slot + 3, wb as u8);
            poke16(hw, slot + 4, h as u16);
            (wb, h, slot + SLOT_META, 0, format::BAND_SHIFT_ONE)
        } else {
            (wb, h, 0, block, f[frame::BAND_SHIFT])
        }
    };
    let base = s.x >> 2;

    // Horizontal clip: byte columns [j0, j1) are on screen.
    let j0 = if base < 0 { -base } else { 0 };
    let j1 = wb.min(layout::COLS as i16 - base);
    if j0 >= j1 {
        return;
    }
    // Vertical clip: sprite lines [top, bot) are on screen.
    let top = if s.y < 0 { -s.y } else { 0 };
    let bot = h.min(layout::HEIGHT - s.y);
    if top >= bot {
        return;
    }
    hw.mark(0);
    poke16(hw, layout::JOB + job::Y, s.y as u16);
    poke16(hw, layout::JOB + job::TOP, top as u16);
    poke16(hw, layout::JOB + job::BOT, bot as u16);
    let dst = buf.bitmap.wrapping_add(((base + j0) as u16) << 3);
    let ncols = (j1 - j0) as u16;

    if block_ram != 0 {
        // Cached: one band, already in RAM.
        let band = block_ram + peek16(hw, block_ram);
        let at = band + peek16(hw, band + 2 * j0 as u16);
        poke16(hw, layout::JOB + job::DST, dst);
        poke16(hw, layout::JOB + job::SRC, at);
        hw.poke(layout::JOB + job::NCOLS, ncols as u8);
        hw.mark(1);
        hw.draw_cols();
        hw.mark(2);
        return;
    }

    // The visible bands' starts, then per band its on-screen column offsets and records.
    let (b0, b1) = ((top as u16) >> shift, ((bot - 1) as u16) >> shift);
    let nb = b1 - b0 + 1;
    hw::fetch(hw, block + 3 * b0 as u32, layout::RECORD, 3 * nb);
    for b in 0..nb {
        let r = layout::RECORD + 3 * b;
        let band = block + (hw.peek(r) as u32 | (hw.peek(r + 1) as u32) << 8 | (hw.peek(r + 2) as u32) << 16);
        hw::fetch(hw, band + 2 * j0 as u32, layout::OFFS, 2 * (ncols + 1));
        let mut j = 0;
        while j < ncols {
            // As many columns as fit the chunk (always at least one: a band column is small).
            let start = peek16(hw, layout::OFFS + 2 * j);
            let mut k = j + 1;
            while k < ncols && peek16(hw, layout::OFFS + 2 * (k + 1)) - start <= layout::CHUNK_LEN {
                k += 1;
            }
            let end = peek16(hw, layout::OFFS + 2 * k);
            hw::fetch(hw, band + start as u32, layout::CHUNK, end - start);
            poke16(hw, layout::JOB + job::DST, dst.wrapping_add(j << 3));
            poke16(hw, layout::JOB + job::SRC, layout::CHUNK);
            hw.poke(layout::JOB + job::NCOLS, (k - j) as u8);
            hw.mark(1);
            hw.draw_cols();
            hw.mark(2);
            j = k;
        }
    }
}

/// Reference sprite kernel (the C64 build uses `asm/blit.S`): draw `NCOLS` consecutive column records
/// from RAM at `SRC` into the bitmap column at `DST` and the ones right of it, lines clipped to
/// [`TOP`, `BOT`), sprite line `l` at screen line `Y + l`.
pub fn draw_cols<H: Hw + ?Sized>(hw: &mut H) {
    let mut dst = peek16(hw, layout::JOB + job::DST);
    let y = peek16(hw, layout::JOB + job::Y);
    let top = peek16(hw, layout::JOB + job::TOP);
    let bot = peek16(hw, layout::JOB + job::BOT);
    let mut src = peek16(hw, layout::JOB + job::SRC);
    let ncols = hw.peek(layout::JOB + job::NCOLS);
    for _ in 0..ncols {
        let items = hw.peek(src);
        src += 1;
        for _ in 0..items {
            let b0 = hw.peek(src);
            let b1 = hw.peek(src + 1);
            let line = b0 as u16 | ((b1 & 1) as u16) << 8;
            if b1 & format::ITEM_MIX != 0 {
                if line >= top && line < bot {
                    let a = dst.wrapping_add(layout::line_offset(y.wrapping_add(line)));
                    let v = hw.peek(a) & hw.peek(src + 3) | hw.peek(src + 2);
                    hw.poke(a, v);
                }
                src += format::MIX_LEN;
            } else {
                let len = hw.peek(src + 2) as u16;
                let data = src + format::RUN_HEADER;
                let (s, e) = (line.max(top), (line + len).min(bot));
                let mut l = s;
                while l < e {
                    let v = hw.peek(data + (l - line));
                    hw.poke(dst.wrapping_add(layout::line_offset(y.wrapping_add(l))), v);
                    l += 1;
                }
                src = data + len;
            }
        }
        dst = dst.wrapping_add(8);
    }
}
