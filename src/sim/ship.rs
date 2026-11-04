//! TALON HD's addition to the vendored sim: each airframe on the promotion ladder flies differently
//! (user: "Let's make each ship unique!"; `docs/story/hd/ships/ship-*.md`: `feel`, `speed_levels_q8`,
//! `hitbox`). The game calls `fly` with the stage's airframe (in the ACE phase, only when it changes);
//! it sets a few `World` fields that are all zero for the TALON, so a reset `World` is the TALON with
//! TALON's own constants and TALON's replay digest still holds. Plain fields: no table lookups in the
//! per-tick code (the 6502 pays for them).
use super::World;

/// The TALON, the baseline: precise, quick to bank, nothing special and nothing missing.
pub const TALON: u8 = 0;
/// The PEREGRINE, a lance: faster in a straight line (64 Q8 px/tick more at every SPEED level), slower to
/// change sides (its sideways speed eases a quarter of the way to the stick's a tick) and to bank; its
/// bolts fly faster (10 px a tick: at most `BOLT_H`, so a bolt still sweeps every line).
pub const PEREGRINE: u8 = 1;
/// The CARACARA, slow and wide: 64 Q8 px/tick slower, a 6 x 6 hitbox, and two TWIN pairs a shot, 12 px
/// apart (four bolts across 28 px).
pub const CARACARA: u8 = 2;
pub const SHIPS: usize = 3;

/// Added to the TALON's SPEED level, straight and diagonal (`* 181 >> 8`).
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".rodata._ZN7talonhd3ace.ships"))]
static SPEED_BIAS: [i16; SHIPS] = [0, 64, -64];
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".rodata._ZN7talonhd3ace.ships"))]
static DIAGONAL_BIAS: [i16; SHIPS] = diagonal(SPEED_BIAS);

/// `SPEED_BIAS` scaled for diagonals as the sim's own table (`* DIAGONAL >> 8`), evaluated by rustc.
const fn diagonal(s: [i16; SHIPS]) -> [i16; SHIPS] {
    let mut d = [0i16; SHIPS];
    let mut i = 0;
    while i < SHIPS {
        d[i] = (s[i] as i32 * super::player::DIAGONAL >> 8) as i16;
        i += 1;
    }
    d
}
/// Non-zero: the sideways velocity moves a quarter of the way to the stick's a tick (0: at once).
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".rodata._ZN7talonhd3ace.ships"))]
static SLIDE: [u8; SHIPS] = [0, 1, 0];
/// Added to the hitbox's half size (the TALON's 4 x 4 is half 2).
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".rodata._ZN7talonhd3ace.ships"))]
static HITBOX: [i16; SHIPS] = [0, 0, 1];
/// Added to the TWIN bolts' speed (8 px/tick).
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".rodata._ZN7talonhd3ace.ships"))]
static BOLT_SPEED: [i16; SHIPS] = [0, 2, 0];
/// Non-zero: each shot is two pairs, this far either side of the nose.
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".rodata._ZN7talonhd3ace.ships"))]
static SPREAD: [i16; SHIPS] = [0, 0, 6];

/// Airframes with models in the level images: a later stage flies the last of them.
pub const BUILT: u8 = 2;

/// The airframe of stage `stage` (0 = S1).
#[inline(always)]
pub fn of_stage(stage: u8) -> u8 {
    if stage < BUILT { stage } else { BUILT - 1 }
}

/// Fly airframe `ship` from now on (its fields; the velocity carries over). ACE phase on the C64 (room).
#[inline(never)]
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".text._ZN7talonhd3ace.fly"))]
pub fn fly(w: &mut World, ship: u8) {
    let s = ship as usize;
    w.ship = ship;
    w.speed_bias = SPEED_BIAS[s];
    w.diagonal_bias = DIAGONAL_BIAS[s];
    w.slide = SLIDE[s];
    w.hitbox_extra = HITBOX[s];
    w.bolt_speed_extra = BOLT_SPEED[s];
    w.spread = SPREAD[s];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::player::{SPEED_DIAGONAL_Q8, SPEED_Q8, TWIN_SPEED};
    use crate::sim::*;

    fn flown(ship: u8) -> World {
        let mut w = World::new();
        fly(&mut w, ship);
        w
    }

    #[test]
    fn the_talon_is_a_reset_world() {
        let mut w = World::new();
        let before = w.digest();
        fly(&mut w, TALON);
        assert_eq!(w.digest(), before);
        assert_eq!((w.speed_bias, w.diagonal_bias, w.slide, w.hitbox_extra, w.bolt_speed_extra, w.spread), (0, 0, 0, 0, 0, 0));
        assert_eq!(DIAGONAL_BIAS, [0, 45, -46]);
    }

    /// Full-speed slide, then the stick released (or reversed): ticks until vx is exactly `to`.
    fn settle(w: &mut World, from: u8, then: u8, to: i16) -> u32 {
        for _ in 0..40 {
            w.step_sim(from);
        }
        let mut n = 0;
        while w.player.vx != to {
            w.step_sim(then);
            n += 1;
            assert!(n < 40, "never settles: vx {}", w.player.vx);
        }
        n
    }

    #[test]
    fn the_peregrine_slide_settles() {
        let full = (SPEED_Q8[0] + 64) as i16;
        let mut l = flown(PEREGRINE);
        l.player.x = 150 << 8;
        let nl = settle(&mut l, LEFT, 0, 0);
        let x = l.player.x;
        for _ in 0..20 {
            l.step_sim(0);
        }
        assert_eq!(l.player.x, x, "no creep once released");
        let mut r = flown(PEREGRINE);
        r.player.x = 150 << 8;
        let nr = settle(&mut r, RIGHT, 0, 0);
        assert_eq!(nl, nr, "as quick to stop either way");
        // Full speed both ways, exactly.
        let mut r = flown(PEREGRINE);
        r.player.x = 40 << 8;
        settle(&mut r, 0, RIGHT, full);
        let mut l = flown(PEREGRINE);
        l.player.x = 260 << 8;
        settle(&mut l, 0, LEFT, -full);
    }

    #[test]
    fn a_wall_stops_the_slide() {
        let mut w = flown(PEREGRINE);
        for _ in 0..200 {
            w.step_sim(RIGHT);
        }
        assert_eq!(w.player.x, player::RIGHT_X << 8);
        let x = w.player.x;
        w.step_sim(LEFT);
        assert!(w.player.x < x, "off the wall at once");
    }

    #[test]
    fn a_spread_shot_is_whole() {
        let mut w = flown(CARACARA);
        for p in w.pairs.iter_mut() {
            p.alive = 3;
            p.y = 100;
        }
        w.pairs[3].alive = 0;
        let before = w.pairs;
        w.step_sim(FIRE);
        let fresh = w.pairs.iter().zip(before.iter()).filter(|(a, b)| a.alive == 3 && b.alive == 0).count();
        assert_eq!(fresh, 0, "one slot: no lopsided shot");
        w.pairs[7].alive = 0;
        w.step_sim(FIRE);
        let x = (w.player.x >> 8) as i16;
        let mut xs = [w.pairs[3].x, w.pairs[7].x];
        xs.sort();
        assert_eq!((w.pairs[3].alive, w.pairs[7].alive), (3, 3));
        assert_eq!(xs, [x - 6, x + 6], "two slots: both pairs");
    }

    #[test]
    fn the_peregrine_slides_into_a_turn() {
        let mut w = flown(PEREGRINE);
        let full = SPEED_Q8[0] + 64;
        let x0 = w.player.x;
        w.step_sim(RIGHT);
        assert_eq!(w.player.x - x0, full >> 2, "the first tick is a quarter of the way");
        for _ in 0..30 {
            w.step_sim(RIGHT);
        }
        assert!(w.player.vx as i32 > full - 4, "then full speed, faster than the TALON");
        let y0 = w.player.y;
        w.step_sim(UP);
        assert_eq!(y0 - w.player.y, full, "straight ahead at once");
        assert!(w.player.vx > 0, "still sliding right");
        let y0 = w.player.y;
        w.step_sim(UP | LEFT);
        assert_eq!(y0 - w.player.y, SPEED_DIAGONAL_Q8[0] + 45, "diagonals take the bias too");
    }

    #[test]
    fn the_peregrines_bolts_are_faster() {
        let mut w = flown(PEREGRINE);
        w.step_sim(FIRE);
        let y0 = w.pairs[0].y;
        w.step_sim(0);
        assert_eq!(y0 - w.pairs[0].y, TWIN_SPEED + 2);
    }

    #[test]
    fn the_caracara_is_slow_and_wide() {
        let mut w = flown(CARACARA);
        assert_eq!(w.hitbox_half(), 3, "6 x 6");
        let x0 = w.player.x;
        w.step_sim(RIGHT | FIRE);
        assert_eq!(w.player.x - x0, SPEED_Q8[0] - 64, "slower, and at once");
        let x = (w.player.x >> 8) as i16;
        let pairs: Vec<i16> = w.pairs.iter().filter(|p| p.alive == 3).map(|p| p.x).collect();
        assert_eq!(pairs, [x - 6, x + 6], "two pairs a shot");
        assert_eq!(of_stage(7), BUILT - 1);
    }
}
