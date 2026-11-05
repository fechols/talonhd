//! TALON HD for the C64 Ultimate: TALON's S1 game (the vendored simulation, one tick per frame) drawn as
//! 3D-rendered software sprites over the scrolling S1 panorama, all pixels DMA'd by the REU into a
//! double-buffered 160×200 multicolor bitmap. (Main loop vendored from gclass `a5775bf`.)
//!
//! Attract mode: ACE (`talonhd::ace`, a GOAP pilot) flies until the joystick (port 2) or SPACE is touched; from then on
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
    /// `asm/reu.S`: a whole `hw::transfer` from `DMA_JOB`, 256-byte pieces.
    fn reu_transfer(command: u8);
    fn hud_irq();
    fn hud_irq_enable();
    #[cfg(not(feature = "rust-columns"))]
    fn draw_cols_asm();
    /// `asm/blit.S`: one sprite (the `SP_*` block at `SPRITE_JOB`); `render::draw_sprite` is its reference.
    fn draw_sprite_asm();
    /// `asm/bands.S`: one band row laid out line-major (`SH_A`, `SH_DST`); `render::transpose_row`.
    fn transpose_row_asm();
    /// `asm/border.S`: the painter's parameters and state, its staging cleared, CIA1 timer A phased to the
    /// raster (interrupts off; once, at boot). It stays off until the main loop turns it on.
    fn border_boot();
    /// `asm/border.S`: the painter's decode tables from `painter::COL`.
    fn border_tables();
}

struct C64;

/// Profiling (feature `profile`): time charged per [`Hw::mark`] slot, and
/// the last mark time.
// Written before every read: no zeroed BSS needed; lives with the static frames (`.noinit`, $CB60).
#[unsafe(link_section = ".noinit")]
static mut PROFILE: [u16; 3] = [0; 3];
// Written before every read: no zeroed BSS needed; lives with the static frames (`.noinit`, $CB60).
#[unsafe(link_section = ".noinit")]
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
    /// Every DMA goes through `transfer` (the painter's gate, interrupts masked): one piece is a transfer.
    #[inline(always)]
    fn dma(&mut self, command: u8, c64: u16, addr: u32, len: u16) {
        self.transfer(command, c64, addr, len)
    }
    /// `asm/reu.S` `reu_transfer`: the pieces in one loop, the REU's addresses running on. Resident, one
    /// copy (`.uptext`, where the Rust loop was): phases call it.
    #[inline(never)]
    #[unsafe(link_section = ".uptext.transfer")]
    fn transfer(&mut self, command: u8, c64: u16, addr: u32, len: u16) {
        let j = DMA_JOB;
        self.poke(j, c64 as u8);
        self.poke(j + 1, (c64 >> 8) as u8);
        self.poke(j + 2, addr as u8);
        self.poke(j + 3, (addr >> 8) as u8);
        self.poke(j + 4, (addr >> 16) as u8);
        self.poke(j + 5, len as u8);
        self.poke(j + 6, (len >> 8) as u8);
        unsafe { reu_transfer(command) }
    }
    #[cfg(not(feature = "rust-columns"))]
    #[inline(always)]
    fn draw_cols(&mut self) {
        unsafe { draw_cols_asm() }
    }
    /// In the BANDS phase with `render::stash_bands` (`link.ld`).
    #[inline(never)]
    fn transpose_row(&mut self, src: u16, dst: u16) {
        self.poke(SHIFT_JOB, src as u8);
        self.poke(SHIFT_JOB + 1, (src >> 8) as u8);
        self.poke(SHIFT_JOB + 4, dst as u8);
        self.poke(SHIFT_JOB + 5, (dst >> 8) as u8);
        unsafe { transpose_row_asm() }
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

#[cfg(feature = "bench")]
unsafe extern "C" {
    /// `asm/bench.S`: time the renderer's primitives on this machine (u16 microseconds per test at `$CA60`,
    /// `$BE` at `$CA7E` when done). In the ACE phase (run once); the game then carries on.
    fn bench_all();
}

/// CIA 2 timer A, free-running at the 1 MHz system clock (counts down).
#[inline(never)]
#[unsafe(link_section = ".hitext.micros")]
fn micros(c: &mut C64) -> u16 {
    loop {
        let hi = c.peek(0xDD05);
        let lo = c.peek(0xDD04);
        if c.peek(0xDD05) == hi {
            return !(lo as u16 | (hi as u16) << 8);
        }
    }
}

/// The border painter's switch (`status::CANVAS` bit 0; bit 7 remembers F1 held): F1 pressed flips it.
/// When it is off and the painter runs, stop the painter and wait for the HUD chain to be back on plain
/// rows (a gate or block already armed completes and reopens the gate; at most a frame), so the frame
/// that follows is rendered without bands and shown the old way. Returns whether the canvas is wanted.
#[inline(never)]
fn canvas(c: &mut C64) -> bool {
    // F1: keyboard row 0, column 4, read against the idle port (a port 1 stick's fire shares the bit).
    c.poke(0xDC02, 0xFF);
    c.poke(0xDC00, 0xFF);
    let idle = c.peek(0xDC01);
    c.poke(0xDC00, 0xFE);
    let down = c.peek(0xDC01) & idle & 16 == 0 && idle & 16 != 0;
    c.poke(0xDC00, 0xFF);
    let at = layout::STATUS + status::CANVAS;
    let mut v = c.peek(at);
    if down && v & 0x80 == 0 {
        v ^= 1;
    }
    v = v & 1 | if down { 0x80 } else { 0 };
    c.poke(at, v);
    let pb = layout::PAINTER;
    if v & 1 == 0 && c.peek(pb + layout::painter::ON) != 0 {
        c.poke(pb + layout::painter::ON, 0);
        while c.peek(pb + layout::painter::NEXT) != 0 || c.peek(pb + layout::painter::GATE) & 0x80 != 0 {}
    }
    v & 1 != 0
}

/// Turn the border painter on for buffer `back`, just shown with its bands stashed. A block refills the
/// staging only after it paints (T loads B's lines, B loads T's), so first block T's lines go in by hand,
/// and ON is set after row 7's IRQ and before row 0's (raster 250..300, HUD lines shaken by up to 6), so
/// block T is the first to run.
#[inline(never)]
fn painter_on(c: &mut C64, back: usize) {
    let pb = layout::PAINTER;
    c.poke(pb + layout::painter::FRONT, back as u8);
    // Block T's 24 lines: four pages of 6 lines (240 bytes), as `asm/border.S` `refill`.
    let mut at = layout::band_set(BUFFERS[back]);
    let mut page = layout::painter::STAGE;
    while page < layout::painter::STAGE + 0x400 {
        talonhd::hw::fetch(c, at, page, 240);
        at += 240;
        page += 0x100;
    }
    while !(250..300).contains(&raster(c)) {}
    c.poke(pb + layout::painter::ON, 1);
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
/// Each slot's phase length in 256-byte pages: BOOT's from the PRG, the rest from the loaded level image's
/// header (`format::header::PHASE_LENS`, [`use_phases`]).
// Written before every read: no zeroed BSS needed; lives with the static frames (`.noinit`, $CB60).
#[unsafe(link_section = ".noinit")]
static mut PHASE_LEN: [u8; paging::SLOTS as usize] = [0; paging::SLOTS as usize];
/// Each slot's REU address >> 8 (the image's phases are page-aligned, `format::header::PHASES`; BOOT's
/// is `paging::REU_BOOT`): middle and high bytes, no 32-bit math.
#[unsafe(link_section = ".noinit")]
static mut PHASE_AT: [u16; paging::SLOTS as usize] = [0; paging::SLOTS as usize];
/// The owner nibble of the loaded image's own phases (its level + 1, [`use_phases`]): levels share
/// slots `paging::COMMON..`, so [`talonhd_page`] refuses any other level's phase (border 10) rather
/// than run whatever code this image keeps in that slot.
#[unsafe(link_section = ".noinit")]
static mut PHASE_OWNER: u8 = 0;
/// The stamp of the phases this PRG was built with: `scripts/build-target.sh` patches it in after the
/// link (`tools/phases.py`); every level image's header must carry the same (`boot_image`, `level_image`).
#[unsafe(no_mangle)]
#[used]
static PHASE_STAMP: u16 = 0xFFFF;

/// Page from this image's phases (the common ones and its level's, `paging`): where they are and their
/// lengths (BOOT's stays the PRG's). At boot once the image is verified, and after every level load. The
/// window's phase stays valid: only a common one can be in it, the same code in every image.
#[inline(never)]
#[unsafe(link_section = ".vtext.use_phases")]
fn use_phases(img: &Image) {
    // The middle and high bytes of the page-aligned address: no 32-bit shift.
    let b = img.phases.to_le_bytes();
    let mut at = u16::from_le_bytes([b[1], b[2]]);
    unsafe {
        PHASE_OWNER = img.level.wrapping_add(1);
        for s in 0..paging::SLOTS as usize {
            if s != paging::BOOT as usize {
                PHASE_LEN[s] = img.phase_lens[s];
                PHASE_AT[s] = at;
            }
            at = at.wrapping_add((paging::PHASE_SLOT >> 8) as u16);
        }
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
    // Another level's phase: its slot here holds this level's code (`paging::page` asserts it on the host).
    if p >> 4 != 0 && p >> 4 != unsafe { PHASE_OWNER } {
        fatal(c, 10);
    }
    let s = paging::slot(p) as usize;
    let (len, at) = unsafe { ((PHASE_LEN[s] as u16) << 8, PHASE_AT[s]) };
    let [lo, hi] = at.to_le_bytes();
    talonhd::hw::fetch(c, u32::from_le_bytes([0, lo, hi, 0]), paging::WINDOW, len);
    c.poke(0x0001, port);
}

/// The game has reached another level (`Clear::start_stage`, or a game over back to the first): load
/// that level's image file into the REU (`talonhd::uci`). The front buffer keeps the last frame (the next
/// stage's card, or GAME OVER) and the HUD runs on while it loads; the music is muted (`main` restarts it
/// on the new image's cue). The HUD's blocks (`hud::static_blocks`) and the phases are the same in every
/// image. From the TUNE phase (paged next for the music); [`level_image`] reads the result.
#[inline(never)]
fn load_level(c: &mut C64, g: &mut Game, from: u8) {
    talonhd::audio::mute(c);
    match talonhd::uci::load_image(c, g.clear.level) {
        talonhd::uci::LOADED => {}
        talonhd::uci::UNCHANGED => {
            // Nothing was written: this level again, as on a C64 that cannot load (`compare` replays
            // it from `status::TOP`).
            g.clear.level = from;
            g.clear.first = from;
            g.clear.top = from;
            c.poke(layout::STATUS + status::TOP, from);
        }
        // Part of the file may be in the REU (even under the old header): no level matches, so
        // `level_image` stops (border 9) before anything is drawn from it.
        _ => g.clear.level = 0xFF,
    }
}

/// The image [`load_level`] left in the REU: the level asked for, baked with these phases (the Ultimate's
/// `00` says the whole file arrived), else fatal (border 9). Resident (BOOT is full), called with BOOT
/// paged (for `render::read_image`). (By value: an `&mut` to main's image would put it on the soft stack.)
#[inline(never)]
fn level_image(c: &mut C64, g: &Game) -> Image {
    // SAFETY: a plain read of a static the build patches; volatile so it is not folded to 0xFFFF.
    let stamp = unsafe { core::ptr::read_volatile(&raw const PHASE_STAMP) };
    match render::read_image(c) {
        Some(img) if img.level == g.clear.level && img.phases != 0 && img.phase_stamp == stamp => {
            c.poke(0xD020, img.border);
            use_phases(&img);
            img
        }
        _ => fatal(c, 9),
    }
}

/// Start the image's stage music: the track's header, the ring filled, both voices set up and gated
/// together, the clock started (each level's image, `main`).
#[inline(always)]
fn start_music(c: &mut C64, player: &mut talonhd::audio::Player, music: u32) {
    paging::page(paging::TUNE);
    talonhd::audio::tune::load(c, player, music);
    paging::page(paging::AUDIO);
    talonhd::audio::prime(c, player);
    if player.state == talonhd::audio::RUNNING {
        paging::page(paging::TUNE);
        talonhd::audio::tune::arm(c);
        unsafe { audio_gate() };
        talonhd::audio::tune::started(c, player);
    }
}

/// Render time (us) above which a frame counts as heavy for the music (`audio::service`).
const AUDIO_HEAVY_US: u16 = 7000;

/// The sprite job block `draw_sprite_asm` reads (`asm/blit.S` `SP_*`): `SP_H` (u16) the frame's height
/// after a draw, `SP_HEIGHT` the target's lines.
const SPRITE_JOB: u16 = 0xCAA0;
const SP_H: u16 = 0xCAAF;
const SP_HEIGHT: u16 = 0xCAD7;
/// `reu_transfer`'s job (`asm/reu.S` `DJ`, in the painter's block: `layout::painter::DMA_JOB`): `$DF02..$DF08`.
const DMA_JOB: u16 = layout::PAINTER + layout::painter::DMA_JOB;
/// `transpose_row_asm`'s inputs (`asm/bands.S` `SH_A`, `SH_DST`).
const SHIFT_JOB: u16 = 0xCAD8;

/// The sprite job's per-sprite inputs (`SP_FRAME`, `SP_X`, then `SP_Y`, `SP_BITMAP`); `SP_DIR` (the frame
/// directory) is set once a frame by `main`. `draw_sprite_asm` never writes its inputs.
const SP_FRAME: u16 = SPRITE_JOB;
const SP_Y: u16 = SPRITE_JOB + 4;
const SP_DIR: u16 = SPRITE_JOB + 8;

/// `draw_sprite_asm` for the frame and x already in the job, at line `y` of the cell rows at `target`
/// (lines `0..height`); returns the frame's height. In the BANDS phase.
#[inline(never)]
fn blit(c: &mut C64, target: u16, height: u8, y: i16) -> i16 {
    c.poke(SP_Y, y as u8);
    c.poke(SP_Y + 1, (y >> 8) as u8);
    c.poke(SP_Y + 2, target as u8);
    c.poke(SP_Y + 3, (target >> 8) as u8);
    c.poke(SP_HEIGHT, height);
    unsafe { draw_sprite_asm() }
    c.peek(SP_H) as i16 | (c.peek(SP_H + 1) as i16) << 8
}

/// `render::draw_canvas` through the assembler path (`render::draw_sprite` is the reference the host
/// model runs): the bitmap (`fine` lines lower), then the bands the sprite reaches. In the BANDS phase (the
/// bands are composed in the window's second half).
#[inline(never)]
fn draw_canvas(c: &mut C64, frames: u16, buf: layout::Buffer, fine: u8, s: &render::Sprite, bands: bool) {
    const BAND: i16 = 8 * layout::BAND_ROWS as i16;
    if s.frame >= frames {
        return;
    }
    c.poke(SP_FRAME, s.frame as u8);
    c.poke(SP_FRAME + 1, (s.frame >> 8) as u8);
    c.poke(SP_FRAME + 2, s.x as u8);
    c.poke(SP_FRAME + 3, (s.x >> 8) as u8);
    let y = s.y + fine as i16;
    #[cfg(feature = "profile")]
    let t0 = cpu_micros(c);
    let h = blit(c, buf.bitmap, layout::HEIGHT as u8, y);
    #[cfg(feature = "profile")]
    let t1 = cpu_micros(c);
    if bands && y < 0 && y + h > -BAND {
        blit(c, layout::BANDS, BAND as u8, y + BAND);
    }
    if bands && y + h > layout::HEIGHT && y < layout::HEIGHT + BAND {
        blit(c, layout::BANDS + layout::BAND_BYTES, BAND as u8, y - layout::HEIGHT);
    }
    // Feature `profile`: the window's blits, the bands' blits (µs) and the sprites drawn, per frame.
    #[cfg(feature = "profile")]
    unsafe {
        PROFILE[0] = PROFILE[0].wrapping_add(t1.wrapping_sub(t0));
        PROFILE[1] = PROFILE[1].wrapping_add(cpu_micros(c).wrapping_sub(t1));
        PROFILE[2] += 1;
    }
}

/// Park (`stash`) or fetch the attract pilot's `Mind` (SEG_BUF <-> REU `ace::MIND_REU`). Called from the
/// step, which runs with I/O banked out: bank it in for the REU registers, then restore.
#[inline(never)]
#[unsafe(link_section = ".uptext.mind_dma")]
fn mind_dma(c: &mut C64, stash: bool) {
    let port = c.peek(0x0001);
    c.poke(0x0001, 0x35);
    if stash {
        talonhd::hw::stash(c, layout::SEG_BUF, talonhd::ace::MIND_REU, talonhd::ace::MIND_BYTES);
    } else {
        talonhd::hw::fetch(c, talonhd::ace::MIND_REU, layout::SEG_BUF, talonhd::ace::MIND_BYTES);
    }
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

/// `-Features timing`: the microseconds since `t0` (`cpu_micros`) into the status block at `offset` (in
/// `.hitext`: main RAM is full).
#[cfg(feature = "timing")]
#[inline(never)]
#[unsafe(link_section = ".hitext.lap")]
fn lap(c: &mut C64, offset: u16, t0: u16) {
    let d = cpu_micros(c).wrapping_sub(t0);
    status_poke16(c, offset, d);
}

/// `-Features timing`/`profile`: microseconds outside the raster IRQs (`micros` less the IRQ clock,
/// `asm/hud.S` `irq_clock_*`), so a lap does not charge the painted blocks to whatever they interrupted.
#[cfg(any(feature = "timing", feature = "profile"))]
#[inline(never)]
#[unsafe(link_section = ".hitext.cpu_micros")]
fn cpu_micros(c: &mut C64) -> u16 {
    loop {
        let irq = |c: &mut C64| c.peek(layout::STATUS + status::IRQ_US) as u16 | (c.peek(layout::STATUS + status::IRQ_US + 1) as u16) << 8;
        let a = irq(c);
        let now = micros(c);
        if irq(c) == a {
            return now.wrapping_sub(a);
        }
    }
}

fn status_poke16(c: &mut C64, offset: u16, value: u16) {
    c.poke(layout::STATUS + offset, value as u8);
    c.poke(layout::STATUS + offset + 1, (value >> 8) as u8);
}

/// Boot, from the BOOT phase: the turbo, the level image (verified, its phases installed), the display and
/// the HUD's blocks, and the game reset on the image's level.
#[inline(never)]
fn boot_image(c: &mut C64, g: &mut Game) -> Image {
    // 64 MHz with badlines off (C64U turbo registers); harmless elsewhere.
    c.poke(0xD031, 0x8F);
    c.poke(0xD011, 0x0B); // blank while setting up
    c.poke(layout::STATUS + status::CANVAS, 0); // the border painter off (F1 turns it on)
    // An image already in the REU (loaded before the PRG, e.g. from the C64U's menu) is used when it is
    // complete and was baked with this PRG's phases (same stamp). Otherwise erase whatever is there
    // and wait (flashing the border) for a complete new upload (magic present and the whole-image
    // checksum right): `tools/c64u.py run` clears the magic first, so `run` + `upload-reu` never starts
    // on the previous image.
    // SAFETY: a plain read of a static the build patches; volatile so it is not folded to 0xFFFF.
    let stamp = unsafe { core::ptr::read_volatile(&raw const PHASE_STAMP) };
    let preloaded = match render::read_image(c) {
        Some(img) if img.phase_stamp == stamp && render::verify_image(c, &img) => Some(img),
        _ => None,
    };
    if preloaded.is_none() {
        render::invalidate_image(c);
    }
    let img: Image = if let Some(img) = preloaded { img } else { loop {
        if let Some(img) = render::read_image(c) {
            if render::verify_image(c, &img) {
                break img;
            }
        }
        for _ in 0..200 {
            c.poke(0xD020, 2);
            c.poke(0xD020, 0);
        }
    } };
    if img.bg_rows < layout::ROWS {
        fatal(c, 7);
    }
    // A stamp from another build is fatal (border 8): the image's phases are not this PRG's.
    if img.phases == 0 || img.phase_stamp != stamp {
        fatal(c, 8);
    }
    use_phases(&img);

    // CIA 2 timer A: continuous, from $FFFF, for render timing.
    c.poke(0xDD04, 0xFF);
    c.poke(0xDD05, 0xFF);
    c.poke(0xDD0E, 0x11);

    let ddr = c.peek(0xDD02) | 3;
    c.poke(0xDD02, ddr);
    c.poke(0xD018, layout::D018);
    c.poke(0xD016, layout::D016);
    c.poke(0xD020, img.border);
    palette(c, img.palette);

    // HUD: sprite blocks into both banks, sprites set up, the multiplexer
    // table filled, then the raster IRQ chain started at the top border.
    render::hud_init(c, &img);
    // On the level of the image in the REU; stage clears go on to the next levels' images when the
    // Ultimate Command Interface can load them (`load_level`), else to this level again.
    let top = if talonhd::uci::present(c) { talonhd::clear::LAST_LEVEL } else { img.level };
    g.reset_at(img.level, top);
    // Debug: boot S1's image and load S2's at once (the per-level load in a few frames).
    #[cfg(feature = "start-s2")]
    {
        g.clear.level = top;
    }
    img
}

/// Set the palette (`render::set_palette`) and the border painter's decode tables to it.
#[inline(never)]
fn palette(c: &mut C64, pal: render::Palette) {
    // With the painter on, between block T's refill and block B: a block that came in mid-rebuild would
    // paint part of its lines through the old decode tables. The bitmap and both bands then change on the
    // same line.
    if c.peek(layout::PAINTER + layout::painter::ON) != 0 {
        while !(72..200).contains(&raster(c)) {}
    }
    render::set_palette(c, pal);
    let p = layout::PAINTER + layout::painter::COL;
    c.poke(p, pal.d021);
    c.poke(p + 1, pal.screen >> 4);
    c.poke(p + 2, pal.screen & 15);
    c.poke(p + 3, pal.color);
    unsafe { border_tables() }
}

/// Boot, from the TUNE phase (just paged for the sampler's calibration), after it: the HUD's first table, the raster IRQ chain
/// started at the top border (with the border painter), the status block cleared.
#[inline(never)]
fn boot_irq(c: &mut C64, g: &Game, hud: u32) {
    for bank in 0..2 {
        render::hud_update(c, &game::hud_state(g), hud, bank);
    }
    unsafe { border_boot() };
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
    c.poke(layout::STATUS + status::TOP, g.clear.top);
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> i16 {
    let c = &mut C64;
    // The BOOT phase rides in the PRG, in frame-buffer space boot overwrites soon after: stash it first.
    unsafe {
        let len = &raw const __phase_boot_size as u16;
        talonhd::hw::stash(c, &raw const __phase_boot_load_start as u16, paging::REU_BOOT, len);
        PHASE_LEN[paging::BOOT as usize] = len.div_ceil(256) as u8;
        PHASE_AT[paging::BOOT as usize] = (paging::REU_BOOT >> 8) as u16;
    }
    // Boot-only code (image checks, HUD setup) runs from the paging window.
    paging::page(paging::BOOT);
    #[allow(static_mut_refs)]
    let g = unsafe { &mut GAME };
    let mut img = boot_image(c, g);
    // The sampler's clock against CIA 2 (about 1.5 s of muted one-shots; no sampler, no music).
    let player = talonhd::audio::player();
    paging::page(paging::TUNE);
    talonhd::audio::tune::calibrate(c, player);
    #[cfg(feature = "start-keel")]
    {
        g.world.scroll = (talonhd::midboss::ARRIVE as u32 - 20) << 8;
        g.world.track_base = 1_000_000;
    }
    #[cfg(feature = "start-boss")]
    {
        g.world.scroll = (talonhd::astrolabe::HOLD as u32 - 10) << 8;
        g.campanile.state = talonhd::campanile::GONE;
    }
    boot_irq(c, g, img.hud);
    #[allow(static_mut_refs)]
    let sprites = unsafe { &mut SPRITES };
    let mut human = false;
    let mut shown_palette = img.palette;
    // Ever touched: from then on the frames are no longer the host's attract-mode replay.
    let mut touched = false;
    let mut back = 1usize;
    let mut flipped_at = micros(c);
    // One level's image at a time: its music and an empty sprite cache (the common frames are rendered
    // under each stage's light), then its frames until the step reaches another level (a stage clear, or
    // a game over back to the first), whose image is loaded before anything is drawn from it.
    #[cfg(feature = "bench")]
    let mut benched = false;
    loop {
        start_music(c, player, img.music);
        render::reset_sprite_cache(c);
        // Feature `bench`: the renderer's unit costs, measured after a full boot (HUD IRQ and music
        // running, as in play), once.
        #[cfg(feature = "bench")]
        if !benched {
            benched = true;
            // The bench lives in the ACE phase (BOOT is full).
            paging::page(paging::ACE);
            unsafe { bench_all() };
        }
        while g.clear.level == img.level {
            let buf = BUFFERS[back];
            let bands = canvas(c);
            let start = micros(c);
            #[cfg(feature = "raster-bars")]
            c.poke(0xD020, 2);
            let n = game::scene(g, sprites);
            let scroll = game::view(g, &img);
            // The animated level's frame for this tick.
            img.background = render::background_at(c, &img, g.ticks as u8);
            // render::render_frame, split so the background can be timed. The canvas's code is the BANDS
            // phase, its bands are composed in the window's second half: nothing pages until they are
            // stashed.
            paging::page(paging::BANDS);
            render::restore_background(c, &img, buf, scroll, bands);
            let bg = micros(c).wrapping_sub(start);
            unsafe {
                PROFILE = [0; 3];
                LAST_MARK = micros(c);
            }
            render::sort_by_z(&mut sprites[..n]);
            c.poke(SP_DIR, img.directory as u8);
            c.poke(SP_DIR + 1, (img.directory >> 8) as u8);
            c.poke(SP_DIR + 2, (img.directory >> 16) as u8);
            for s in sprites[..n].iter() {
                draw_canvas(c, img.frames, buf, scroll.fine, s, bands);
            }
            if bands {
                render::stash_bands(c, buf, scroll);
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

            c.poke(layout::STATUS + status::FLIPPING, 1);
            let pb = layout::PAINTER;
            if c.peek(pb + layout::painter::ON) != 0 {
                // The border painter flips at the end of block B (bitmap bank, $D016, $D011 with this
                // frame's YSCROLL, band set, together): ask, and wait for it.
                c.poke(pb + layout::painter::D016, scroll.d016());
                c.poke(layout::PAINTER_D011, scroll.d011());
                c.poke(pb + layout::painter::FLIP, 0x80 | back as u8);
                #[cfg(feature = "timing")]
                let t_wait = cpu_micros(c);
                while c.peek(pb + layout::painter::FLIP) != 0 {}
                // For the plain path's one-flip-a-blank guard once the canvas is turned off.
                flipped_at = micros(c);
                #[cfg(feature = "timing")]
                lap(c, status::WAIT_US, t_wait);
            } else {
                // One flip per vertical blank: if we flipped less than a blank's
                // length ago, first wait for the beam to reach the picture.
                if micros(c).wrapping_sub(flipped_at) < 8000 {
                    while in_vblank(c) {}
                }
                while !in_vblank(c) {}
                // present() also turns the display on: $D011 = bitmap, 24 rows, YSCROLL.
                render::present(c, buf, scroll);
                flipped_at = micros(c);
                // With the canvas on, this frame's bands are in its set: the painter takes over (from
                // block T on).
                if bands {
                    painter_on(c, back);
                }
            }
            #[cfg(feature = "timing")]
            let t_hud = cpu_micros(c);
            render::hud_update(c, &game::hud_state(g), img.hud, back);
            #[cfg(feature = "timing")]
            lap(c, status::HUD_US, t_hud);
            // Keep the music ring ahead of the sampler (I/O is in). Music is not game state, so its cost
            // follows the frame's: a frame whose render took over AUDIO_HEAVY_US decodes a little less than
            // it plays, light frames make it up (`audio::service`); the ring's ~1.9 s lead covers the gap.
            paging::page(paging::AUDIO);
            #[cfg(feature = "timing")]
            let t_audio = cpu_micros(c);
            talonhd::audio::service(c, player, took < AUDIO_HEAVY_US);
            #[cfg(feature = "timing")]
            lap(c, status::AUDIO_US, t_audio);
            // The boss's phase 2 recolors `hot` (a color RAM fill), in step with the frame just shown.
            let pal = game::palette(g, img.palette);
            if pal != shown_palette {
                palette(c, pal);
                shown_palette = pal;
            }
            c.poke(layout::STATUS + status::FRONT, back as u8);
            status_poke16(c, status::SHOWN_T, g.ticks as u16);
            c.poke(layout::STATUS + status::HUMAN, touched as u8);
            c.poke(layout::STATUS + status::FLIPPING, 0);
            c.poke(layout::STATUS + status::MAGIC, layout::STATUS_MAGIC);

            back ^= 1;
            #[cfg(feature = "timing")]
            let t_step = cpu_micros(c);
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
                #[cfg(feature = "timing")]
                let t_ace = cpu_micros(c);
                c.poke(0x0001, 0x34);
                let input = if human {
                    stick
                } else {
                    // ACE, one slice (one overlay) a tick: see (ATTRACT), the lookahead (MOTOR), think
                    // (ACE). The Mind is worked on in SEG_BUF (the renderer's scratch, idle during the
                    // step) and parked in the REU between ticks.
                    // SAFETY: SEG_BUF is ours during the step; Mind holds plain integers.
                    let mind = unsafe { &mut *(layout::SEG_BUF as *mut talonhd::ace::Mind) };
                    let slice = g.ace.slice();
                    if slice != 0 {
                        mind_dma(c, false);
                    }
                    let input = if slice == 0 {
                        paging::page(paging::ATTRACT);
                        talonhd::ace::see(g, mind)
                    } else if slice == 1 {
                        paging::page(paging::MOTOR);
                        talonhd::ace::motor(g, mind)
                    } else {
                        paging::page(paging::ACE);
                        talonhd::ace::think(g, mind)
                    };
                    if g.ace.slice() != 0 {
                        mind_dma(c, true);
                    }
                    input
                };
                #[cfg(feature = "timing")]
                {
                    c.poke(0x0001, 0x35);
                    lap(c, status::ACE_US, t_ace);
                    c.poke(0x0001, 0x34);
                }
                g.step(input);
                c.poke(0x0001, 0x35);
            }
            #[cfg(feature = "timing")]
            lap(c, status::STEP_US, t_step);
        }
        paging::page(paging::TUNE);
        load_level(c, g, img.level);
        paging::page(paging::BOOT);
        img = level_image(c, g);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    fatal(&mut C64, 10)
}
