//! TALON HD tech scene for the C64 Ultimate: a 512×512 3D-rendered boss,
//! drones and the player's ship over a vertically scrolling panorama, all
//! pixels DMA'd by the REU into a double-buffered 160×200 multicolor bitmap.
//! (Main loop vendored from gclass `a5775bf`.)
//!
//! Needs the matching `talonhd.reu` at REU address 0 (16 MB REU); until it
//! appears the border flashes red, so it may be uploaded after starting.

#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

use talonhd::demo;
use talonhd::hw::Hw;
use talonhd::layout::{self, BUFFERS, status};
use talonhd::render::{self, Image, Scroll, Sprite};

unsafe extern "C" {
    fn reu_execute(command: u8);
    fn hud_irq();
    fn hud_irq_enable();
    #[cfg(not(feature = "rust-columns"))]
    fn draw_column_asm();
}

struct C64;

/// Profiling (feature `profile`): time charged per [`Hw::mark`] slot, and
/// the last mark time.
static mut PROFILE: [u16; 3] = [0; 3];
static mut LAST_MARK: u16 = 0;

impl Hw for C64 {
    #[inline(always)]
    fn peek(&mut self, addr: u16) -> u8 {
        unsafe { read_volatile(addr as *const u8) }
    }
    #[inline(always)]
    fn poke(&mut self, addr: u16, value: u8) {
        unsafe { write_volatile(addr as *mut u8, value) }
    }
    #[inline(always)]
    fn reu_exec(&mut self, command: u8) {
        unsafe { reu_execute(command) }
    }
    #[cfg(not(feature = "rust-columns"))]
    #[inline(always)]
    fn draw_column(&mut self) {
        unsafe { draw_column_asm() }
    }
    #[inline(always)]
    fn fill(&mut self, addr: u16, len: u16, value: u8) {
        unsafe { core::ptr::write_bytes(addr as *mut u8, value, len as usize) }
    }
    #[cfg(feature = "profile")]
    fn mark(&mut self, slot: u8) {
        let now = micros(self);
        unsafe {
            let i = (slot as usize).min(2);
            PROFILE[i] = PROFILE[i].wrapping_add(now.wrapping_sub(LAST_MARK));
            LAST_MARK = now;
        }
    }
}

fn raster(c: &mut C64) -> u16 {
    loop {
        let hi = c.peek(0xD011) & 0x80;
        let lo = c.peek(0xD012);
        if c.peek(0xD011) & 0x80 == hi {
            return lo as u16 | (hi as u16) << 1;
        }
    }
}

/// CIA 2 timer A, free-running at the 1 MHz system clock (counts down).
fn micros(c: &mut C64) -> u16 {
    loop {
        let hi = c.peek(0xDD05);
        let lo = c.peek(0xDD04);
        if c.peek(0xDD05) == hi {
            return !(lo as u16 | (hi as u16) << 8);
        }
    }
}

/// Vertical blank: below the last bitmap line or above the first one.
fn in_vblank(c: &mut C64) -> bool {
    let r = raster(c);
    !(40..251).contains(&r)
}

fn fatal(c: &mut C64, color: u8) -> ! {
    loop {
        c.poke(0xD020, color);
        c.poke(0xD020, 0);
    }
}

fn status_poke16(c: &mut C64, offset: u16, value: u16) {
    c.poke(layout::STATUS + offset, value as u8);
    c.poke(layout::STATUS + offset + 1, (value >> 8) as u8);
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i16 {
    let c = &mut C64;
    // 64 MHz with badlines off (C64U turbo registers); harmless elsewhere.
    c.poke(0xD031, 0x8F);
    c.poke(0xD011, 0x0B); // blank while setting up
    // Wait (flashing the border) until a TALON HD image is in the REU, so it
    // can be uploaded after starting the PRG, past any REU preload on reset.
    // Erase any earlier image's magic, then wait for a complete new one
    // (magic present and the whole-image checksum right).
    render::invalidate_image(c);
    let img: Image = loop {
        if let Some(img) = render::read_image(c) {
            if render::verify_image(c, &img) {
                break img;
            }
        }
        for _ in 0..200 {
            c.poke(0xD020, 2);
            c.poke(0xD020, 0);
        }
    };
    if img.bg_rows < layout::ROWS {
        fatal(c, 7);
    }

    // CIA 2 timer A: continuous, from $FFFF, for render timing.
    c.poke(0xDD04, 0xFF);
    c.poke(0xDD05, 0xFF);
    c.poke(0xDD0E, 0x11);

    let ddr = c.peek(0xDD02) | 3;
    c.poke(0xDD02, ddr);
    c.poke(0xD018, layout::D018);
    c.poke(0xD016, layout::D016);
    c.poke(0xD020, img.border);
    render::set_palette(c, img.palette);

    // HUD: sprite blocks into both banks, sprites set up, the multiplexer
    // table filled, then the raster IRQ chain started at the top border.
    render::hud_init(c, &img);
    render::hud_update(c, &demo::hud_state(0));
    c.poke(layout::HUD_TABLE, 0);
    let vector = hud_irq as usize as u16;
    c.poke(0xFFFE, vector as u8);
    c.poke(0xFFFF, (vector >> 8) as u8);
    c.poke(0xD012, talonhd::hud::TOP_IRQ_LINE);
    c.poke(0xD019, 0xFF);
    c.poke(0xD01A, 0x01);
    unsafe { hud_irq_enable() };
    for i in 0..16 {
        c.poke(layout::STATUS + i, 0);
    }

    let mut sprites = [Sprite::default(); demo::INSTANCES];
    let mut t: u16 = 0;
    let mut back = 1usize;
    let mut flipped_at = micros(c);
    loop {
        let buf = BUFFERS[back];
        let start = micros(c);
        #[cfg(feature = "raster-bars")]
        c.poke(0xD020, 2);
        let n = demo::scene(t, &mut sprites);
        let scroll = Scroll::new(&img, demo::scroll(t));
        // render::render_frame, split so the background can be timed.
        render::restore_background(c, &img, buf, scroll);
        let bg = micros(c).wrapping_sub(start);
        unsafe {
            PROFILE = [0; 3];
            LAST_MARK = micros(c);
        }
        render::sort_by_z(&mut sprites[..n]);
        for s in sprites[..n].iter() {
            let mut s = *s;
            s.y += scroll.fine as i16;
            render::draw_sprite(c, &img, buf, &s);
        }
        status_poke16(c, status::BACKGROUND_US, bg);
        for i in 0..3 {
            #[allow(static_mut_refs)]
            let v = unsafe { PROFILE[i] };
            status_poke16(c, status::PROFILE_US + 2 * i as u16, v);
        }

        #[cfg(feature = "raster-bars")]
        c.poke(0xD020, img.border);
        let took = micros(c).wrapping_sub(start);
        status_poke16(c, status::RENDER_US, took);

        // One flip per vertical blank: if we flipped less than a blank's
        // length ago, first wait for the beam to reach the picture.
        if micros(c).wrapping_sub(flipped_at) < 8000 {
            while in_vblank(c) {}
        }
        while !in_vblank(c) {}

        c.poke(layout::STATUS + status::FLIPPING, 1);
        // present() also turns the display on: $D011 = bitmap, 24 rows, YSCROLL.
        render::present(c, buf, scroll);
        flipped_at = micros(c);
        render::hud_update(c, &demo::hud_state(t));
        c.poke(layout::STATUS + status::FRONT, back as u8);
        status_poke16(c, status::SHOWN_T, t);
        c.poke(layout::STATUS + status::FLIPPING, 0);
        c.poke(layout::STATUS + status::MAGIC, layout::STATUS_MAGIC);

        back ^= 1;
        if c.peek(layout::STATUS + status::HOLD) == 0 {
            t = t.wrapping_add(1);
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    fatal(&mut C64, 10)
}
