//! S2 TERRACES' air units (paged phase `paging::S2AIR`, after `stage2::step`, while `Ground::air` counts
//! any): PIKEs on their rails, KESTRELs, the DOME MORTAR's shells. `crate::stage2` places them; each
//! carries what it needs in its `Unit` (a phase reads only its own tables):
//! - PIKE: `x` (1/8 px, world), `cool` its rail, `mode` `stage2::LEFTWARD` | `FIRES` | 1 once fired;
//!   `u` (1/8 lines, world) follows from the rail;
//! - KESTREL: `x`, `u` its screen position in 1/16 px, `t` ticks still to wait, `mode` its manoeuvre |
//!   `stage2::FROM_LEFT`, `cool` the x/2 where it turns, `id` its pair;
//! - shell: `x`, `u` its screen position in 1/32 px, `base`, `n` its velocity, `t` its age, `cool` the
//!   bomb's slot once it is low.
//!
//! PIKEs and KESTRELs publish their velocity on screen (Q4 px/tick) in `base` and `n` for the attract
//! pilot (`ace::units`).

use crate::demo::*;
use crate::ground::{burst, hits, launch, shoot, Ground, Unit};
use crate::ground::{FREE, KESTREL, LIVE, PIKE, SHELL, UNITS};
use crate::sim::actors::{B_BOMB, B_ROUND};
use crate::sim::collide::HITBOX_DY;
use crate::sim::world::*;
use crate::stage2::{FIRES, FROM_LEFT, LEFTWARD};

crate::stage2::s2_helpers!();

/// A PIKE rail: the stage line at x = `RAIL_X0` and the change over each `RAIL_STEP` px after it.
pub struct Rail {
    pub u0: u16,
    pub du: [i8; RAIL_SEGS],
}
pub const RAIL_SEGS: usize = 14;
pub const RAIL_X0: i16 = -64;
pub const RAIL_STEP: i16 = 32;

/// The rails (generated from `crate::s2`'s geometry: `stage2`'s `the_rails_follow_the_level` prints
/// them): 0-2 vine rows on terrace 7, 3-7 rows on terraces 16-17, 8 the AQUEDUCT's channel, 9-11 village
/// lanes, 12 the gravel path from dome 3 to dome 4.
pub static RAILS: [Rail; 13] = [
    Rail { u0: 456, du: [3, 7, 7, 2, -2, -3, 0, 2, 1, -4, -7, -6, 0, 5] },
    Rail { u0: 477, du: [5, 8, 4, 0, -5, -3, 0, 1, 0, -5, -5, -3, 3, 7] },
    Rail { u0: 498, du: [7, 8, 3, -3, -7, -4, -1, 1, -1, -5, -4, 0, 6, 8] },
    Rail { u0: 1050, du: [4, -3, -6, -8, -3, 2, 4, 1, -2, -1, 3, 6, 6, 2] },
    Rail { u0: 1077, du: [-1, -5, -7, -5, 1, 5, 5, 2, -1, 0, 3, 4, 3, -4] },
    Rail { u0: 1102, du: [-2, -7, -6, -2, 5, 7, 5, 1, -2, 0, 3, 3, -1, -6] },
    Rail { u0: 1125, du: [-3, -4, -3, 3, 7, 6, 3, -3, -3, -1, 1, 1, -3, -6] },
    Rail { u0: 1140, du: [-3, -3, 0, 5, 9, 6, 0, -4, -5, -2, 1, 0, -4, -6] },
    Rail { u0: 1694, du: [0; RAIL_SEGS] },
    Rail { u0: 3230, du: [0; RAIL_SEGS] },
    Rail { u0: 3282, du: [0; RAIL_SEGS] },
    Rail { u0: 3341, du: [0; RAIL_SEGS] },
    Rail { u0: 3980, du: [0, 0, 0, 3, 25, 20, 12, 5, 0, -3, -2, 0, 0, 0] },
];

/// KESTREL manoeuvres.
const WAIT: u8 = 0;
const ENTER: u8 = 1;
const TURN: u8 = 2;
const EXIT: u8 = 3;
/// The shell: harmless until `SHELL_LOW`, lands at `SHELL_FLIGHT`.
pub const SHELL_LOW: u8 = 70;
pub const SHELL_FLIGHT: u8 = 90;

/// A KESTREL's velocity at each heading (0 up, counter-clockwise), 3 px/tick in Q4.
const KV: [(i8, i8); 16] = {
    let mut t = [(0i8, 0i8); 16];
    let mut h = 0;
    while h < 16 {
        let a = h as f64 * core::f64::consts::PI / 8.0;
        let (s, c) = (const_sin(a), const_sin(a + core::f64::consts::FRAC_PI_2));
        t[h] = (round(-48.0 * s), round(-48.0 * c));
        h += 1;
    }
    t
};

const fn round(v: f64) -> i8 {
    (if v < 0.0 { v - 0.5 } else { v + 0.5 }) as i8
}

/// One tick (paged: `paging::S2AIR`) at stage line `at`, after `stage2::step`.
#[inline(never)]
pub fn step(g: &mut Ground, w: &mut World, at: u16) {
    for i in 0..UNITS {
        let u = &mut g.units[i];
        if u.state == LIVE {
            match u.kind {
                PIKE => pike(u, w, at),
                KESTREL => kestrel(u, w, &mut g.pair),
                SHELL => shell(u, w),
                _ => {}
            }
        }
    }
}

/// An air unit within (`hw` + 1, 8) of the ship (the TALON's reach; `hw` is one less: a byte of S2AIR) destroys it as a WASP would: one is put on the ship for
/// the sim's own collision next tick (the unit bursts). Returns whether it hit.
#[inline(never)]
fn ram(u: &mut Unit, w: &mut World, hw: i16) -> bool {
    if w.state != PLAY || w.player.invuln != 0 {
        return false;
    }
    let (px, py) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16);
    // The airframe's hitbox grows the reach (`sim::ship`: the CARACARA's 6 x 6). |d| < r as one unsigned
    // compare: d + r - 1 in 0..2r - 1 (no abs: S2AIR is full).
    let (rw, rh) = (hw + w.hitbox_extra, 7 + w.hitbox_extra);
    if (u.px - px + rw) as u16 >= (rw + rw + 1) as u16 || (u.py - py - HITBOX_DY + rh) as u16 >= (rh + rh + 1) as u16 {
        return false;
    }
    launch(w, px, py, false);
    burst(w, u.px, u.py);
    u.state = FREE;
    true
}

/// PIKE: along its rail at 4 px/tick; one shot down over the ship.
#[inline(never)]
fn pike(u: &mut Unit, w: &mut World, at: u16) {
    let left = u.mode & LEFTWARD != 0;
    u.x += if left { -32 } else { 32 };
    let rail = &RAILS[u.cool as usize];
    let rel = u.x - RAIL_X0 * 8;
    if rel < 0 || rel >= RAIL_SEGS as i16 * RAIL_STEP * 8 {
        u.state = FREE;
        return;
    }
    let seg = (rel >> 8) as usize;
    let mut v = rail.u0 * 8;
    for s in 0..seg {
        v = v.wrapping_add((rail.du[s] as i16 * 8) as u16);
    }
    let d = rail.du[seg] as i16;
    u.u = v.wrapping_add(((d * (rel & 255)) >> 5) as u16);
    place(u, at);
    if u.py > 176 + 48 || u.py < -72 {
        u.state = FREE;
        return;
    }
    // Nose along the rail; its velocity on screen (Q4: x 4 px, y the rail's slope against the scroll).
    let (dx, dy) = if left { (-RAIL_STEP, d) } else { (RAIL_STEP, -d) };
    u.yaw = yaw_to(dx, dy);
    u.base = if left { -64i8 } else { 64 } as u8;
    u.n = (8 + 2 * dy) as i8 as u8;
    u.frame = S2_PIKE_FRAME as u8 + u.yaw;
    if ram(u, w, 11) {
        return;
    }
    let (px, below) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16 - u.py);
    if u.mode & (FIRES | 1) == FIRES && u.py >= 0 && below > 16 && (px - u.px).abs() < 6 {
        w.spawn_ebullet(B_ROUND, u.px, u.py + 8, 0, 96);
        u.mode |= 1;
    }
    if u.px > -8 && u.px < 312 && hits(w, (u.px, u.py), 8) != 0 {
        fall(u, w, 100);
    }
}

/// KESTREL: wait, cross, turn and fire, climb away.
#[inline(never)]
fn kestrel(u: &mut Unit, w: &mut World, pair: &mut u8) {
    let from_left = u.mode & FROM_LEFT != 0;
    let mut roll = 1;
    match u.mode & 15 {
        WAIT => {
            if u.t != 0 {
                u.t -= 1;
            } else {
                // It turns where the ship is as it comes in.
                u.mode += ENTER;
                u.cool = (((w.player.x >> 8) as i16) >> 1) as u8;
            }
        }
        ENTER => {
            let turn_x = u.cool as i16 * 2;
            if (from_left && u.px >= turn_x) || (!from_left && u.px <= turn_x) {
                u.mode += TURN - ENTER;
                u.t = 0;
            }
        }
        TURN => {
            roll = if from_left { 0 } else { 2 };
            u.t += 1;
            if u.t & 3 == 0 {
                u.yaw = if from_left { u.yaw + 1 } else { u.yaw.wrapping_sub(1) } & 15;
            }
            if u.t == 16 {
                let to = yaw_to((w.player.x >> 8) as i16 - u.px, (w.player.y >> 8) as i16 - u.py);
                spread(w, u.px, u.py, to, 1);
                u.mode += EXIT - TURN;
            }
        }
        _ => {}
    }
    if u.mode & 15 != WAIT {
        let (vx, vy) = KV[u.yaw as usize];
        u.x += vx as i16;
        u.u = u.u.wrapping_add(vy as i16 as u16);
        u.base = vx as u8;
        u.n = vy as u8;
    }
    u.px = u.x >> 4;
    u.py = (u.u as i16) >> 4;
    u.frame = S2_KESTREL_FRAME as u8 + u.yaw * KESTREL_ROLLS as u8 + roll;
    if u.py < -40 {
        u.state = FREE;
        return;
    }
    if ram(u, w, 17) || u.px < -8 || u.px > 312 {
        return;
    }
    let n = hits(w, (u.px, u.py), 14);
    if n != 0 {
        u.hp = u.hp.saturating_sub(n);
        if u.hp == 0 {
            fall(u, w, 300);
            if *pair == u.id {
                w.add_score(500);
                *pair = 255;
            } else {
                *pair = u.id;
            }
        }
    }
}

/// The shell: up, over and down onto its mark; a falling bomb for its last ticks; a burst on landing.
#[inline(never)]
fn shell(u: &mut Unit, w: &mut World) {
    u.t += 1;
    let (vx, vy) = (u.base as i8, u.n as i8);
    u.x += vx as i16;
    u.u = u.u.wrapping_add(vy as i16 as u16);
    u.px = u.x >> 5;
    u.py = (u.u as i16) >> 5;
    let h = if u.t < SHELL_FLIGHT / 2 { u.t } else { SHELL_FLIGHT - u.t };
    u.frame = S2_SHELL_FRAME as u8 + (h >= 12) as u8 + (h >= 24) as u8 + (h >= 36) as u8;
    if u.t < SHELL_LOW {
        if hits(w, (u.px, u.py), 4) != 0 {
            fall(u, w, 50);
        }
        return;
    }
    let slot = u.cool as usize;
    if u.t == SHELL_LOW {
        for k in 0..EBULLETS {
            let b = &mut w.ebullets[k];
            if b.kind == 0 {
                b.kind = B_BOMB;
                b.x = u.x * 2;
                b.y = (u.u as i16) * 2;
                b.vx = vx * 2;
                b.vy = vy * 2;
                u.cool = k as u8;
                break;
            }
        }
    } else if slot < EBULLETS && w.ebullets[slot].kind != B_BOMB {
        // Shot down (the sim scored it).
        u.state = FREE;
        return;
    }
    if u.t == SHELL_FLIGHT {
        if slot < EBULLETS && w.ebullets[slot].kind == B_BOMB {
            w.ebullets[slot].kind = 0;
        }
        burst(w, u.px, u.py);
        u.state = FREE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A KESTREL just out of the TALON's reach rams the wider CARACARA (`sim::ship`).
    #[test]
    fn rams_use_the_airframes_hitbox() {
        for (ship, hit) in [(crate::sim::ship::TALON, false), (crate::sim::ship::CARACARA, true)] {
            let mut w = World::new();
            crate::sim::ship::fly(&mut w, ship);
            w.state = PLAY;
            w.player.invuln = 0;
            let (px, py) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16);
            let mut u = Unit { state: LIVE, px: px + 18, py: py + HITBOX_DY, ..Unit::default() };
            assert_eq!(ram(&mut u, &mut w, 17), hit, "ship {ship}");
        }
    }
}
