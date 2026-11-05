//! REU code paging (C64; on the host only a check): code that is only active some of the time lives in
//! the REU as a *phase* and is copied into one shared window just before it runs, as TALON's runtime does.
//!
//! - The C64 link script (`target-c64/link.ld`) links every phase at [`WINDOW`]; `tools/phases.py`
//!   extracts them (`dist/phases`) and `bake` puts them into each level image, one [`PHASE_SLOT`] per
//!   slot from the header's `PHASES` address.
//! - A phase id is its slot (low nibble) and its owner (high nibble): 0 = every level's (slots
//!   `0..COMMON`, the same code in every image), else level + 1. Each level's own phases share the slots
//!   from [`COMMON`] up: S1's fights and ground units and S2's waves, units and boss sit in the same slots
//!   of their own images, and the per-level load (`uci`) brings a level's code with its art. A level
//!   image carries the common phases and its own.
//! - [`page`] makes a phase current: one REU fetch from the loaded image when it is not already in the
//!   window (BOOT, which rides in the PRG, from [`REU_BOOT`]). It may be called from the game step, which
//!   runs with I/O banked out; the C64 side banks I/O in around the DMA. A level's phase may be paged
//!   only while that level's image is loaded: never on the tick the step moves to another level
//!   (`Game::step`), which the host checks ([`loaded`]) and the C64 refuses (`talonhd_page`: border 10,
//!   rather than run the loaded image's code in that slot).
//! - Code in a phase may call resident code only (never another phase), and nothing resident may call
//!   into a phase without paging it first. Render-time helpers of a paged module stay resident (the link
//!   script names them before the phase's catch-all pattern).

/// Where every phase runs: `$9000-$9FFF`, which VIC bank 2 sees as character ROM and the CPU as RAM.
pub const WINDOW: u16 = 0x9000;
pub const WINDOW_LEN: u16 = 0x1000;
/// REU address of the BOOT phase, which the PRG carries: stashed there first thing and paged from there
/// (level images leave its slot empty). The top 64 KiB bank's first 4 KiB.
pub const REU_BOOT: u32 = 0xFF_0000;
/// One slot per phase in a level image.
pub const PHASE_SLOT: u32 = 0x1000;
/// Slots in a level image (the header's `PHASE_LENS`; its 16th byte is the level).
pub const SLOTS: u8 = 15;
/// Slots `0..COMMON` hold every level's phases, `COMMON..SLOTS` the level's own.
pub const COMMON: u8 = 8;

/// Level `level`'s own phase in `slot`.
pub const fn of_level(level: u8, slot: u8) -> u8 {
    (level + 1) << 4 | slot
}

// Phase ids. Every level's:
/// The attract pilot's eyes (`ace::see`), paged only while nobody is playing.
pub const ATTRACT: u8 = 0;
/// Boot-only code (`render::hud_init`, the REU image checks): paged at startup and after a level load.
pub const BOOT: u8 = 1;
/// The music player's producer (`audio::service`, `start`) and the DLT5 decode kernel, paged every frame.
pub const AUDIO: u8 = 2;
/// One-time music setup (`audio::tune`): sampler calibration, track header, voice setup; the level load.
pub const TUNE: u8 = 3;
/// The attract pilot's thinking (`ace::think`: facts, goal, plan, the stick), paged after MOTOR.
pub const ACE: u8 = 4;
/// The attract pilot's lookahead (`ace::motor`), paged after ATTRACT's `ace::see`.
pub const MOTOR: u8 = 5;
/// The stage clear and the promotion (`clear::Clear::step`), paged from the boss's fall to the next
/// stage's first line.
pub const STAGE: u8 = 6;
/// The canvas's background (`render::restore_background`: the ring and the line shift) and the border
/// bands (`draw_canvas`, `render::stash_bands`), paged every frame before the background; the bands are
/// composed in the window's second half (`layout::BANDS`), so it is free of code while a frame is drawn.
pub const BANDS: u8 = 7;
// S1 BREAKWATER's (level 0):
/// The GANTRY QUEEN's fight logic (`boss::Boss::step` and its helpers).
pub const BOSS: u8 = of_level(0, 8);
/// The KEELBREAKER's fight logic (`midboss::Midboss::step` and its helpers).
pub const MIDBOSS: u8 = of_level(0, 9);
/// The ground and sea units' step (`ground::Ground::step`), paged while any is out or due.
pub const GROUND: u8 = of_level(0, 10);
// S2 TERRACES' (level 1):
/// The waves (`stage2::waves`) and the CAMPANILE, paged while a placement is due or a WASP string releases.
pub const S2: u8 = of_level(1, 8);
/// S2's ground units (`gnd2::step`), paged while any is out.
pub const S2GND: u8 = of_level(1, 9);
/// S2's PIKEs, KESTRELs and shells (`air2::step`), paged while any is out.
pub const S2AIR: u8 = of_level(1, 10);
/// S2's boss, the ASTROLABE (`astrolabe::Astrolabe::step`, and at render time its `sprites`).
pub const ASTRO: u8 = of_level(1, 11);

pub const PHASES: usize = 15;
/// Every phase, every level's (`dist/phases/<name>.bin`, `.phase.<name>` in `link.ld`): ids and names, in
/// the stamp's order.
pub const IDS: [u8; PHASES] = [ATTRACT, BOOT, AUDIO, TUNE, ACE, MOTOR, STAGE, BANDS, BOSS, MIDBOSS, GROUND, S2, S2GND, S2AIR, ASTRO];
pub const NAMES: [&str; PHASES] = ["attract", "boot", "audio", "tune", "ace", "motor", "stage", "bands", "boss", "midboss", "ground", "s2", "s2gnd", "s2air", "astro"];

/// The slot a phase occupies in its image.
#[inline(always)]
pub const fn slot(id: u8) -> u8 {
    id & 15
}

/// Whether phase `id` is in `level`'s image.
pub const fn in_level(id: u8, level: u8) -> bool {
    id >> 4 == 0 || id >> 4 == level + 1
}

/// `level`'s image's phase slots: each slot's image from `phases` (every phase, [`NAMES`] order), empty
/// for none (BOOT's: the PRG carries it); trailing empty slots left out.
#[cfg(feature = "host")]
pub fn level_slots(phases: &[Vec<u8>], level: u8) -> Vec<Vec<u8>> {
    let mut slots = vec![Vec::new(); SLOTS as usize];
    for (&id, img) in IDS.iter().zip(phases) {
        if id != BOOT && in_level(id, level) {
            slots[slot(id) as usize] = img.clone();
        }
    }
    while slots.last().is_some_and(|s| s.is_empty()) {
        slots.pop();
    }
    slots
}

/// The stamp tying the level images to the PRG built with them: an additive checksum of every phase
/// image ([`NAMES`] order, every level's) but `BOOT`'s (`tools/phases.py` patches it into the PRG, `bake`
/// into every header).
pub fn stamp(images: &[&[u8]]) -> u16 {
    let mut s = 0x5A5Au16;
    for (&id, img) in IDS.iter().zip(images.iter()) {
        if id == BOOT {
            continue;
        }
        for &b in img.iter() {
            s = s.rotate_left(1).wrapping_add(b as u16);
        }
        s = s.wrapping_add(img.len() as u16);
    }
    s
}

#[cfg(feature = "host")]
std::thread_local! {
    static LOADED: core::cell::Cell<u8> = const { core::cell::Cell::new(0xFF) };
}

/// Host check: the level whose image the C64 has loaded now (`Game::step`: the level at the tick's
/// start; `game::scene`: the current one); [`page`] asserts a level's phase is paged only then. A no-op
/// on the C64.
#[inline(always)]
pub fn loaded(level: u8) {
    #[cfg(feature = "host")]
    LOADED.with(|l| l.set(level));
    let _ = level;
}

#[cfg(target_arch = "mos")]
unsafe extern "C" {
    fn talonhd_page(phase: u8);
}

/// Make `phase` the one in the window.
#[inline(always)]
pub fn page(phase: u8) {
    #[cfg(feature = "host")]
    LOADED.with(|l| {
        let level = l.get();
        assert!(level == 0xFF || in_level(phase, level), "phase {phase:#04x} paged while level {level}'s image is loaded");
    });
    #[cfg(target_arch = "mos")]
    unsafe {
        talonhd_page(phase)
    };
    let _ = phase;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_ids_have_their_own_slots() {
        for (i, &a) in IDS.iter().enumerate() {
            assert!(slot(a) < SLOTS);
            assert_eq!(slot(a) < COMMON, a >> 4 == 0, "{}: a common phase in a level slot or back", NAMES[i]);
            for &b in &IDS[i + 1..] {
                for level in 0..2 {
                    assert!(!(in_level(a, level) && in_level(b, level) && slot(a) == slot(b)), "{} shares a slot", NAMES[i]);
                }
            }
        }
    }

    /// The host refuses a level's phase while another level's image is loaded (the C64: border 10).
    #[test]
    #[should_panic(expected = "paged while level 0")]
    fn another_levels_phase_is_refused() {
        loaded(0);
        page(BOSS);
        page(S2);
    }
}
