//! TALON HD for the C64 Ultimate: TALON's S1 game (the vendored simulation, one tick per frame) drawn as
//! 3D-rendered software sprites over the scrolling S1 panorama, all pixels DMA'd by the REU into a
//! double-buffered 160×200 multicolor bitmap. (Main loop vendored from gclass `a5775bf`.)
//!
//! Attract mode: the sim's autopilot flies until the joystick (port 2) or SPACE is touched; from then on
//! the player does, until GAME OVER hands the stick back. While the autopilot flies, every frame is a pure
//! function of the tick count, so `talonhd compare` replays it on the host and checks digest and pixels.
//!
//! Needs the matching `talonhd.reu` at REU address 0 (16 MB REU); until it
//! appears the border flashes red, so it may be uploaded after starting.

#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};

use talonhd::game::{self, Game};
use talonhd::paging;
use talonhd::hw::Hw;
use talonhd::sim::{self, world::GAME_OVER};
use talonhd::layout::{self, BUFFERS, status};
use talonhd::render::{self, Image, Sprite};

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

/// The game and this frame's sprites live in BSS (the soft stack under $D000 is small); the game is
/// zero here and set up by `Game::reset` at boot (no 700-byte initial image in the program).
static mut GAME: Game = unsafe { core::mem::zeroed() };
/// In the free tail of `$0200-$07FF` after `.lowtext` (`.lowbss`, not loaded: `game::scene` fills it
/// before every use).
#[unsafe(link_section = ".lowbss")]
static mut SPRITES: [Sprite; game::INSTANCES] = [Sprite { frame: 0, x: 0, y: 0, z: 0 }; game::INSTANCES];

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

unsafe extern "C" {
    /// The BOOT phase's load image in the PRG (the other phases come in the level image).
    static __phase_boot_load_start: u8;
    static __phase_boot_size: u8;
    /// Gate both sampler voices with IRQs masked (`asm/hud.S`).
    fn audio_gate();
}

/// The phase in the window (0xFF: none).
static mut CURRENT_PHASE: u8 = 0xFF;
/// Each phase's length (BOOT's from the PRG, the rest from the level image's header).
static mut PHASE_LEN: [u16; paging::PHASES as usize] = [0; paging::PHASES as usize];
/// The stamp of the phases this PRG was built with: `scripts/build-target.sh` patches it in after the
/// link (`tools/phases.py`); the level image's header must carry the same (`install_phases`).
#[unsafe(no_mangle)]
#[used]
static PHASE_STAMP: u16 = 0xFFFF;

/// Copy the level image's phases to the phase bank, through the bitmap at `$A000` (not shown yet).
/// Runs from the BOOT phase, once the image is verified; a stamp from another build is fatal (border 8).
#[inline(never)]
fn install_phases(c: &mut C64, img: &Image) {
    // SAFETY: a plain read of a static the build patches; volatile so it is not folded to 0xFFFF.
    let stamp = unsafe { core::ptr::read_volatile(&raw const PHASE_STAMP) };
    if img.phases == 0 || img.phase_stamp != stamp {
        fatal(c, 8);
    }
    for p in 0..paging::PHASES {
        if p == paging::BOOT {
            continue;
        }
        let n = img.phase_lens[p as usize];
        let slot = p as u32 * paging::PHASE_SLOT;
        talonhd::hw::fetch(c, img.phases + slot, 0xA000, n);
        talonhd::hw::stash(c, 0xA000, paging::REU_PHASES + slot, n);
        unsafe { PHASE_LEN[p as usize] = n };
    }
}

/// `paging::page` on the C64: fetch phase `p` into the window unless it is there already. Called from
/// the game step, which runs with I/O banked out: bank it in for the REU registers, then restore.
#[unsafe(no_mangle)]
pub extern "C" fn talonhd_page(p: u8) {
    let c = &mut C64;
    unsafe {
        if CURRENT_PHASE == p {
            return;
        }
        CURRENT_PHASE = p;
    }
    let port = c.peek(0x0001);
    c.poke(0x0001, 0x35);
    let len = unsafe { PHASE_LEN[p as usize] };
    talonhd::hw::fetch(c, paging::REU_PHASES + p as u32 * paging::PHASE_SLOT, paging::WINDOW, len);
    c.poke(0x0001, port);
}

/// The input byte (talon::sim UP DOWN LEFT RIGHT FIRE ARM): joystick port 2 from CIA1 port A, ARM from
/// SPACE (keyboard row 7, column 4), as TALON reads them (no POTX second button here).
#[inline(never)]
fn read_input(c: &mut C64) -> u8 {
    c.poke(0xDC02, 0xFF);
    c.poke(0xDC00, 0xFF);
    let mut input = !c.peek(0xDC00) & 31;
    let idle = c.peek(0xDC01);
    c.poke(0xDC00, 0x7F);
    if c.peek(0xDC01) & idle & 16 == 0 && idle & 16 != 0 {
        input |= sim::ARM;
    }
    c.poke(0xDC00, 0xFF);
    input
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
    // The BOOT phase rides in the PRG, in frame-buffer space boot overwrites soon after: stash it first.
    unsafe {
        let len = &raw const __phase_boot_size as u16;
        talonhd::hw::stash(c, &raw const __phase_boot_load_start as u16, paging::REU_PHASES + paging::BOOT as u32 * paging::PHASE_SLOT, len);
        PHASE_LEN[paging::BOOT as usize] = len;
    }
    // Boot-only code (image checks, HUD setup) runs from the paging window.
    paging::page(paging::BOOT);
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
    install_phases(c, &img);

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
    #[allow(static_mut_refs)]
    let g = unsafe { &mut GAME };
    g.reset();
    // The sampler's clock against CIA 2 (about 1.5 s of muted one-shots; no sampler, no music) and the
    // stage music's header.
    let player = talonhd::audio::player();
    paging::page(paging::TUNE);
    talonhd::audio::tune::calibrate(c, player);
    talonhd::audio::tune::load(c, player, img.music);
    #[cfg(feature = "start-keel")]
    {
        g.world.scroll = (talonhd::midboss::ARRIVE as u32 - 20) << 8;
        g.world.track_base = 1_000_000;
    }
    #[allow(static_mut_refs)]
    let sprites = unsafe { &mut SPRITES };
    render::hud_update(c, &game::hud_state(g));
    c.poke(layout::HUD_TABLE, 0);
    let vector = hud_irq as usize as u16;
    c.poke(0xFFFE, vector as u8);
    c.poke(0xFFFF, (vector >> 8) as u8);
    c.poke(0xD012, talonhd::hud::TOP_IRQ_LINE);
    c.poke(0xD019, 0xFF);
    c.poke(0xD01A, 0x01);
    unsafe { hud_irq_enable() };
    for i in 0..32 {
        c.poke(layout::STATUS + i, 0);
    }
    // The stage music: fill the ring, set both voices up, gate them together, start the clock.
    paging::page(paging::AUDIO);
    talonhd::audio::prime(c, player);
    if player.state == talonhd::audio::RUNNING {
        paging::page(paging::TUNE);
        talonhd::audio::tune::arm(c);
        unsafe { audio_gate() };
        talonhd::audio::tune::started(c, player);
    }

    let mut human = false;
    let mut shown_palette = img.palette;
    // Ever touched: from then on the frames are no longer the host's attract-mode replay.
    let mut touched = false;
    let mut back = 1usize;
    let mut flipped_at = micros(c);
    loop {
        let buf = BUFFERS[back];
        let start = micros(c);
        #[cfg(feature = "raster-bars")]
        c.poke(0xD020, 2);
        let n = game::scene(g, sprites);
        let scroll = game::view(g, &img);
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
        render::hud_update(c, &game::hud_state(g));
        // Keep the music ring ahead of the sampler (I/O is in; the AUDIO phase is paged every frame).
        paging::page(paging::AUDIO);
        talonhd::audio::service(c, player);
        // The boss's phase 2 recolors `hot` (a color RAM fill), in step with the frame just shown.
        let pal = game::palette(g, img.palette);
        if pal != shown_palette {
            render::set_palette(c, pal);
            shown_palette = pal;
        }
        c.poke(layout::STATUS + status::FRONT, back as u8);
        status_poke16(c, status::SHOWN_T, g.ticks as u16);
        c.poke(layout::STATUS + status::HUMAN, touched as u8);
        c.poke(layout::STATUS + status::FLIPPING, 0);
        c.poke(layout::STATUS + status::MAGIC, layout::STATUS_MAGIC);

        back ^= 1;
        if c.peek(layout::STATUS + status::HOLD) == 0 {
            // One sim tick per displayed frame. The stick takes over from the autopilot when touched
            // and hands back at GAME OVER.
            let stick = read_input(c);
            if stick != 0 {
                human = true;
                touched = true;
            }
            if g.world.state == GAME_OVER {
                human = false;
            }
            // The step runs with I/O banked out: the boss code lives in the RAM under $D000 (it and
            // everything it calls are pure computation; the HUD IRQ banks I/O in for itself).
            c.poke(0x0001, 0x34);
            let input = if human {
                stick
            } else {
                paging::page(paging::ATTRACT);
                sim::replay::autopilot(&g.world)
            };
            g.step(input);
            c.poke(0x0001, 0x35);
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    fatal(&mut C64, 10)
}
