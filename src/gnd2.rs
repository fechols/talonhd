//! S2 TERRACES' ground units (paged phase `paging::S2GND`, while `Ground::gnd` counts any): ROOF HATCHes,
//! BELL TURRETs, DOME MORTARs and S2's PILLBOXes, placed by `stage2::waves` (their behaviour: the
//! `crate::stage2` module doc). A mortar's shell is handed to `crate::air2`.

use crate::demo::*;
use crate::ground::{burst, hits, launch, shoot, Ground, Unit, NO_UNIT};
use crate::ground::{BELL, FREE, KESTREL, LIVE, MORTAR, PIKE, ROOF, SHELL, UNITS, WRECK};
use crate::sim::world::*;

crate::stage2::s2_helpers!();

/// ROOF HATCH modes.
const SHUT: u8 = 0;
const OPENING: u8 = 1;
const OPEN: u8 = 2;
const CLOSING: u8 = 3;
const REST: u8 = 4;
const DONE: u8 = 5;
/// DOME MORTAR modes (`SHUT`, `OPENING` as above).
const AIM: u8 = 2;
const FIRE: u8 = 3;
const HOLD: u8 = 4;
const BACK: u8 = 5;
const CLOSE: u8 = 6;
/// Farthest a shell is lobbed on either axis (px): its speed must fit the bomb's Q6 `i8` (`air2`).
const SHELL_REACH: i16 = 178;


/// One tick at stage line `at` (paged: `paging::S2GND`): every ground unit; `Ground::gnd` recounted.
#[inline(never)]
pub fn step(g: &mut Ground, w: &mut World, at: u16) {
    let mut n = 0;
    for i in 0..UNITS {
        let u = &mut g.units[i];
        if u.state == FREE || u.kind == PIKE || u.kind == KESTREL || u.kind == SHELL {
            continue;
        }
        step_unit(u, w, at);
        if u.mode == FIRE && u.kind == MORTAR {
            u.mode = HOLD;
            u.t = 20;
            lob(g, i, w);
        }
        if g.units[i].state != FREE {
            n += 1;
        }
    }
    g.gnd = n;
}

/// A ground unit's tick: its place, its behaviour, TWIN hits.
#[inline(never)]
fn step_unit(u: &mut Unit, w: &mut World, at: u16) {
    place(u, at);
    if u.py > 176 + 48 || u.py < -72 || u.px < -48 || u.px > 352 {
        u.state = FREE;
        return;
    }
    if u.state == WRECK {
        return;
    }
    let s = seen(u, w);
    let (half, score, wreck) = match u.kind {
        ROOF => {
            roof(u, w, s);
            (12, 800, S2_ROOF_FRAME + ROOF_POSES)
        }
        BELL => {
            // Steps every 6 ticks within 150 lines, glows 18, a 3-way spread; cooldown 110.
            turret(u, w, s, 6, 150, 18, 110, S2_BELL_FRAME);
            (14, 500, S2_WRECK_FRAME)
        }
        MORTAR => {
            mortar(u, s);
            (13, 600, S2_WRECK_FRAME + 1)
        }
        _ => {
            // S1's PILLBOX: every 4 ticks within 160 lines, glows 12, one shot; cooldown 90.
            turret(u, w, s, 4, 160, 12, 90, S2_PILLBOX_FRAME);
            (11, 200, S2_WRECK_FRAME)
        }
    };
    if u.py < -8 || u.py > 184 {
        return;
    }
    let mut n = hits(w, (u.px, u.py), half);
    // A shut hatch shrugs the bolts off; a shut dome takes half.
    if n == 0 || (u.kind == ROOF && u.mode == SHUT) {
        return;
    }
    if u.kind == MORTAR && u.mode == SHUT {
        n = (n + 1) >> 1;
    }
    u.hp = u.hp.saturating_sub(n);
    if u.hp == 0 {
        fall(u, w, score);
        u.state = WRECK;
        u.frame = wreck as u8;
    }
}

/// A DOME MORTAR's shell toward the ship's position (from unit `i`, which just fired); `air2` flies it.
#[inline(never)]
fn lob(g: &mut Ground, i: usize, w: &mut World) {
    let (mx, my) = (g.units[i].px, g.units[i].py);
    let (dx, dy) = (((w.player.x >> 8) as i16 - mx).clamp(-SHELL_REACH, SHELL_REACH), ((w.player.y >> 8) as i16 - my).clamp(-SHELL_REACH, SHELL_REACH));
    for j in 0..UNITS {
        let u = &mut g.units[j];
        if u.state == FREE {
            // Position in 1/32 px; the velocity reaches the mark in 90 ticks (91/256 ≈ 32/90).
            *u = NO_UNIT;
            u.kind = SHELL;
            u.state = LIVE;
            u.hp = 1;
            u.cool = 255;
            u.x = mx * 32;
            u.u = (my * 32) as u16;
            u.base = ((dx * 91) >> 8) as i8 as u8;
            u.n = ((dy * 91) >> 8) as i8 as u8;
            u.px = mx;
            u.py = my;
            u.frame = S2_SHELL_FRAME as u8;
            g.air += 1;
            return;
        }
    }
}

/// ROOF HATCH: shut until 40 lines in, then two cycles of open, launch two, shut, rest.
#[inline(never)]
fn roof(u: &mut Unit, w: &mut World, s: Seen) {
    match u.mode {
        SHUT => {
            if u.py >= 40 {
                u.mode = OPENING;
                u.t = 0;
            }
        }
        OPENING => {
            u.t += 1;
            if u.t == 24 {
                u.mode = OPEN;
                u.t = 0;
            }
        }
        OPEN => {
            u.t += 1;
            if u.t == 1 || u.t == 21 {
                launch(w, u.px, u.py, s.x > u.px);
            }
            if u.t == 40 {
                u.mode = CLOSING;
                u.t = 24;
            }
        }
        CLOSING => {
            u.t -= 1;
            if u.t == 0 {
                u.n += 1;
                u.mode = if u.n == 2 { DONE } else { REST };
                u.t = 100;
            }
        }
        REST => {
            u.t -= 1;
            if u.t == 0 {
                u.mode = OPENING;
            }
        }
        _ => {}
    }
    let pose = match u.mode {
        OPENING | CLOSING => {
            let p = u.t >> 2;
            if p > 5 { 5 } else { p }
        }
        OPEN => 5,
        _ => 0,
    };
    u.frame = S2_ROOF_FRAME as u8 + pose;
}

/// A turret (BELL TURRET, PILLBOX): within `range` lines it steps one heading toward the ship every
/// `every` ticks; aligned and cooled, it glows (its armed frames) `glow` ticks, then fires (the bell a
/// 3-way spread, the PILLBOX one round) and cools `cool` ticks.
#[inline(never)]
fn turret(u: &mut Unit, w: &mut World, s: Seen, every: u8, range: i16, glow: u8, cool: u8, frame: u16) {
    let in_range = s.above && s.below <= range;
    if u.mode != 0 {
        u.mode -= 1;
        if u.mode == 0 {
            if s.above {
                if u.kind == BELL {
                    spread(w, u.px, u.py, u.yaw, 0);
                } else {
                    shoot(w, u.px, u.py, dir_of(u.yaw), 1);
                }
            }
            u.cool = cool;
        }
    } else {
        u.t += 1;
        if u.t >= every {
            u.t = 0;
            if in_range {
                let d = s.target.wrapping_sub(u.yaw) & 15;
                if d != 0 {
                    u.yaw = if d < 8 { u.yaw + 1 } else { u.yaw.wrapping_sub(1) } & 15;
                }
            }
        }
        if u.cool != 0 {
            u.cool -= 1;
        } else if in_range && u.yaw == s.target {
            u.mode = glow;
        }
    }
    u.frame = (frame + u.yaw as u16 + if u.mode != 0 { UNIT_YAWS } else { 0 }) as u8;
}

/// DOME MORTAR: rest shut; open (the telegraph), swing to the ship, fire (`step` lobs the shell), hold,
/// swing back, shut.
#[inline(never)]
fn mortar(u: &mut Unit, s: Seen) {
    let in_range = s.above && s.below <= 170 && u.py >= 16;
    match u.mode {
        SHUT => {
            if u.cool != 0 {
                u.cool -= 1;
            } else if in_range {
                u.mode = OPENING;
                u.t = 0;
            }
        }
        OPENING => {
            u.t += 1;
            if u.t == 24 {
                u.mode = AIM;
            }
        }
        AIM | BACK => {
            u.t += 1;
            let to = if u.mode == AIM { s.target } else { 8 };
            if u.mode == AIM && !in_range {
                u.mode = BACK;
            } else if u.yaw == to {
                if u.mode == AIM {
                    u.mode = FIRE;
                } else {
                    u.mode = CLOSE;
                    u.t = 24;
                }
            } else if u.t & 1 == 0 {
                let d = to.wrapping_sub(u.yaw) & 15;
                u.yaw = if d < 8 { u.yaw + 1 } else { u.yaw.wrapping_sub(1) } & 15;
            }
        }
        HOLD => {
            u.t -= 1;
            if u.t == 0 {
                u.mode = BACK;
            }
        }
        CLOSE => {
            u.t -= 1;
            if u.t == 0 {
                u.mode = SHUT;
                u.cool = 150;
            }
        }
        _ => {}
    }
    u.frame = S2_MORTAR_FRAME as u8
        + match u.mode {
            SHUT => 0,
            OPENING | CLOSE => (u.t >= 6) as u8 + (u.t >= 12) as u8 + (u.t >= 18) as u8,
            _ => MORTAR_POSES as u8 + u.yaw,
        };
}

