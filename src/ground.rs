//! S1's ground and sea units in TALON HD's sprite staging (`docs/story/03-enemies/`: SKIFF, PILLBOX,
//! HATCH, CRAWLER). In TALON they were bitmap patches; HD has no bitmap layer, so each is a hand-built
//! 3D sprite drawn over the panorama, below everything that flies. They live in stage coordinates (the
//! level's world: x hires px, `u` stage line up from the start), so they scroll with the 3D level they
//! sit on, and their placements are checked against its geometry (`placements_sit_on_the_level`).
//!
//! - **SKIFF** (sea, 3 HP, 200): runs its lane at 0.75 px/tick relative to the water; every 100 ticks,
//!   while its lane is above the TALON, swings its bow one heading toward it for 14 ticks, fires one
//!   aimed shot (1.25 px/tick) and swings back.
//! - **PILLBOX** (ground, 4 HP, 200): within 160 lines above the TALON its turret steps one of 16
//!   headings toward it every 4 ticks; aligned, its barrels glow for 12 ticks (a steady `hot` glow, never
//!   a blink), then one aimed shot at ~1.5 px/tick; cooldown 90. Leaves a crater.
//! - **HATCH** (ground, 8 HP, 1,000 + a SPARK): shut until it is 40 lines into the playfield; then two
//!   cycles of doors opening over 20 ticks, three WASPs launched 20 ticks apart on `drop_u`, doors
//!   closing, a 90-tick rest. Shut doors shrug off TWIN bolts (HD has only TWIN, so open doors take full
//!   damage: the lesson stays "shoot it while it opens").
//! - **CRAWLER** (ground, 6 HP, 400): drives its path at 0.375 px/tick; every 64 px it stops, pivots
//!   toward the TALON, its muzzle glows for 16 ticks, fires 2 shots 8 ticks apart at ~1.8 px/tick along
//!   its hull, pivots back and drives on. Leaves a hulk.
//!
//! Every unit holds fire once the TALON is above it. They are surface units: the TALON flies over them
//! (no contact damage).
//!
//! The step is a paged phase (`crate::paging::GROUND`): every helper it needs is defined here. The
//! render-time `Ground::sprites`, the paging test `Ground::busy` and the helpers `hits`, `shoot`,
//! `launch` and `burst` are resident (their own section names keep them out of the phase's catch-all pattern; the phase
//! may call them).

use crate::demo::*;
use crate::render::Sprite;
use crate::sim::actors::{B_ROUND, BURST_TICKS};
use crate::sim::track::{DROP, MIRROR};
use crate::sim::world::*;

pub const SKIFF: u8 = 1;
pub const PILLBOX: u8 = 2;
pub const HATCH: u8 = 3;
pub const CRAWLER: u8 = 4;

/// Units alive (or wrecked) at once.
pub const UNITS: usize = 10;

/// `Unit::state`.
pub const FREE: u8 = 0;
pub const LIVE: u8 = 1;
pub const WRECK: u8 = 2;

/// Draw order: under the KEELBREAKER's hull (-2) and everything that flies.
pub const Z_GROUND: i16 = -3;

/// One authored unit: activated when the stage has travelled `at` lines; at world (`x`, `u`) moving
/// (`vx`, `vu`) in 1/8 px per tick (`vu` up the stage).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub at: u16,
    pub kind: u8,
    /// (x + 32) / 2: x is even, -32..=478.
    pub x2: u8,
    pub u: u16,
    pub vx: i8,
    pub vu: i8,
}

const fn p(at: u16, kind: u8, x: i16, u: u16, vx: i8, vu: i8) -> Placement {
    assert!(x % 2 == 0 && x >= -32 && x <= 478);
    Placement { at, kind, x2: ((x + 32) / 2) as u8, u, vx, vu }
}

impl Placement {
    /// World x, hires px.
    pub const fn x(&self) -> i16 {
        self.x2 as i16 * 2 - 32
    }
}
/// A ground unit standing at (`x`, `u`), activated just before it scrolls in at the top.
const fn st(kind: u8, x: i16, u: u16) -> Placement {
    p(u - 232, kind, x, u, 0, 0)
}

/// S1 BREAKWATER (`04-levels/s1-breakwater.md` waves, placed on `crate::s1`'s geometry), sorted by `at`.
pub static PLACEMENTS: [Placement; PLACED as usize] = [
    // Screen 2: two SKIFFs cross the open sea left to right, 56 px apart.
    p(440, SKIFF, -20, 600, 6, 0),
    p(477, SKIFF, -20, 600, 6, 0),
    // Screen 4: four SKIFFs line abreast, coming up from below, staggered.
    p(800, SKIFF, 64, 785, 0, 6),
    p(800, SKIFF, 128, 777, 0, 6),
    p(800, SKIFF, 192, 769, 0, 6),
    p(800, SKIFF, 256, 761, 0, 6),
    // Screen 8: the SEA WALL's top.
    st(PILLBOX, 100, 1814),
    st(PILLBOX, 220, 1814),
    // Screen 10: the docks' quays, and three SKIFFs running down the channel.
    st(PILLBOX, 216, 2060),
    st(PILLBOX, 104, 2100),
    st(PILLBOX, 250, 2130),
    st(PILLBOX, 70, 2150),
    p(2000, SKIFF, 136, 2232, 0, -6),
    p(2000, SKIFF, 160, 2242, 0, -6),
    p(2000, SKIFF, 184, 2252, 0, -6),
    // Screen 14 (after the KEELBREAKER): six PILLBOXes on the quays and the last basin's strips.
    st(PILLBOX, 104, 3010),
    st(PILLBOX, 40, 3040),
    st(PILLBOX, 216, 3060),
    p(2860, SKIFF, 140, 3040, 0, 6),
    p(2860, SKIFF, 180, 3040, 0, 6),
    st(PILLBOX, 104, 3100),
    st(PILLBOX, 280, 3130),
    p(2900, SKIFF, 140, 3085, 0, 6),
    p(2900, SKIFF, 180, 3085, 0, 6),
    st(PILLBOX, 250, 3180),
    // The airfield: AA pits, hangar HATCHes, a CRAWLER on the near taxiway, a convoy down the runway.
    st(PILLBOX, 20, 3640),
    st(HATCH, 160, 3700),
    p(3620, CRAWLER, 340, 3826, -3, 0),
    st(PILLBOX, 140, 3920),
    st(PILLBOX, 296, 4000),
    p(3998, CRAWLER, 76, 4230, 0, -3),
    p(4046, CRAWLER, 76, 4278, 0, -3),
    st(HATCH, 296, 4300),
    p(4094, CRAWLER, 76, 4326, 0, -3),
    st(PILLBOX, 184, 4380),
];
pub const PLACED: u8 = 35;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Unit {
    pub kind: u8,
    pub state: u8,
    /// Behaviour sub-state (CRAWLER drive/pivot/glow/fire/return, HATCH doors, a telegraph count).
    pub mode: u8,
    pub hp: u8,
    pub t: u8,
    pub cool: u8,
    /// Heading frame (16, 0 = nose up the screen, counter-clockwise), or the HATCH's door pose.
    pub yaw: u8,
    /// The heading of its path.
    pub base: u8,
    /// Shots or cycles left.
    pub n: u8,
    /// Its placement (velocity).
    pub id: u8,
    /// World position in 1/8: x hires px, u stage lines.
    pub x: i16,
    pub u: u16,
    /// Playfield centre this tick, and the frame to draw (from `SKIFF_FRAME`), set by the step.
    pub px: i16,
    pub py: i16,
    pub frame: u8,
}

const NO_UNIT: Unit = Unit { kind: 0, state: 0, mode: 0, hp: 0, t: 0, cool: 0, yaw: 0, base: 0, n: 0, id: 0, x: 0, u: 0, px: 0, py: 0, frame: 0 };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ground {
    pub units: [Unit; UNITS],
    /// Next placement, the line it is due, the line of the last step (a smaller one: the stage looped
    /// or a new game began) and how many units are out.
    pub next: u8,
    pub next_at: u16,
    pub last: u16,
    pub live: u8,
}

impl Default for Ground {
    fn default() -> Self {
        Ground::new()
    }
}

/// 48 directions, index 0 straight down the screen, rising toward +x, at 1.25 px/tick (Q6).
const DIRS: [(i8, i8); 48] = crate::boss::DIRS;

/// The heading frame (16, 0 = up the screen, counter-clockwise) closest to (`dx`, `dy`): the sector in the
/// quadrant from slope comparisons at 11.25° and 33.75° either side of each axis (no multiplies beyond
/// small constants, no table).
#[inline(never)]
fn yaw_to(dx: i16, dy: i16) -> u8 {
    let (ax, ay) = (dx.abs(), dy.abs());
    // 0 = along the vertical axis .. 4 = along the horizontal.
    let s = if ax * 5 < ay {
        0
    } else if ax * 3 < ay * 2 {
        1
    } else if ay * 3 >= ax * 2 {
        2
    } else if ay * 5 >= ax {
        3
    } else {
        4
    };
    match (dx < 0, dy < 0) {
        (true, true) => s,
        (true, false) => 8 - s,
        (false, false) => 8 + s,
        (false, true) => (16 - s) & 15,
    }
}

/// The 48-direction index (`DIRS`) of heading frame `yaw`.
fn dir_of(yaw: u8) -> u8 {
    (3 * yaw + 24) % 48
}

/// One heading step from `yaw` toward `to`.
fn turn(yaw: u8, to: u8) -> u8 {
    let d = to.wrapping_sub(yaw) & 15;
    if d == 0 {
        yaw
    } else if d < 8 {
        (yaw + 1) & 15
    } else {
        yaw.wrapping_sub(1) & 15
    }
}

/// An aimed round from (`x`, `y`) toward direction `k`, leaving from the muzzle 20 px out along it, at
/// 1.25 px/tick plus `boost` quarters (0: 1.25, 1: ~1.56, 2: ~1.88).
#[inline(never)]
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".text.resident.ground_shoot"))]
fn shoot(w: &mut World, x: i16, y: i16, k: u8, boost: u8) {
    let (vx, vy) = DIRS[k as usize];
    let (bx, by) = ((vx >> 2) * boost as i8, (vy >> 2) * boost as i8);
    w.spawn_ebullet(B_ROUND, x + (vx >> 2) as i16, y + (vy >> 2) as i16, vx + bx, vy + by);
}

/// A burst at playfield (`x`, `y`), reusing the slot with least time left (as TALON's own bursts).
/// Resident (`.hitext`): the GANTRY QUEEN's phase uses it too.
#[inline(never)]
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".hitext.ground_burst"))]
pub(crate) fn burst(w: &mut World, x: i16, y: i16) {
    let mut best = 0;
    for i in 1..BURSTS {
        if w.bursts[i].t < w.bursts[best].t {
            best = i;
        }
    }
    w.bursts[best] = Burst { t: BURST_TICKS, x, y };
}

/// Spend TWIN bolts that hit box (`c` ± `half`) (playfield px); returns the hits.
#[inline(never)]
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".text.resident.ground_hits"))]
fn hits(w: &mut World, c: (i16, i16), half: i16) -> u8 {
    let mut n = 0;
    for p in w.pairs.iter_mut() {
        if p.alive == 0 || (p.y + 5 - c.1).abs() >= half + 5 {
            continue;
        }
        let dx = p.x - c.0;
        if p.alive & 1 != 0 && (dx - 4).abs() < half + 2 {
            p.alive &= !1;
            n += 1;
        }
        if p.alive & 2 != 0 && (dx + 4).abs() < half + 2 {
            p.alive &= !2;
            n += 1;
        }
    }
    n
}

/// Canvas (multicolor px × lines) of a unit kind; wrecks keep their unit's.
#[inline(always)]
fn size(kind: u8) -> (u16, u16) {
    match kind {
        SKIFF => (SKIFF_W, SKIFF_H),
        CRAWLER => (CRAWLER_W, CRAWLER_H),
        _ => (PILLBOX_W, PILLBOX_H),
    }
}

/// Hit points by kind.
const HP: [u8; 5] = [0, 3, 4, 8, 6];

/// CRAWLER modes.
const DRIVE: u8 = 0;
const PIVOT: u8 = 1;
const GLOW: u8 = 2;
const FIRE: u8 = 3;
const BACK: u8 = 4;
/// Ticks between CRAWLER stops: 64 px at 0.375 px/tick.
const LEG_TICKS: u8 = 171;

/// HATCH modes.
const SHUT: u8 = 0;
const OPENING: u8 = 1;
const OPEN: u8 = 2;
const CLOSING: u8 = 3;
const REST: u8 = 4;
const DONE: u8 = 5;

impl Ground {
    pub const fn new() -> Ground {
        Ground { units: [NO_UNIT; UNITS], next: 0, next_at: 0, last: 0, live: 0 }
    }

    /// Whether the step has work at stage line `at` (resident: decides the paging). Pure and called only
    /// from the step (I/O banked out), so it lives in the RAM under the I/O chips.
    #[inline(never)]
    #[cfg_attr(target_arch = "mos", unsafe(link_section = ".iotext.ground_busy"))]
    pub fn busy(&self, at: u16) -> bool {
        self.live != 0 || at >= self.next_at || at < self.last
    }

    /// Render time (resident): every unit and wreck, ground level, appended to the frame's sprites.
    #[inline(never)]
    #[cfg_attr(target_arch = "mos", unsafe(link_section = ".text.resident.ground_sprites"))]
    pub fn sprites(&self, out: &mut [Sprite; crate::game::INSTANCES], n: &mut usize) {
        for u in &self.units {
            if u.state != FREE {
                let (w, h) = size(u.kind);
                crate::game::put(out, n, SKIFF_FRAME + u.frame as u16, u.px, u.py, w, h, Z_GROUND);
            }
        }
    }

    /// One tick at stage line `at`, after the world's (paged).
    #[inline(never)]
    pub fn step(&mut self, w: &mut World, at: u16) {
        if at < self.last {
            // The stage looped or a new game began: start the placements over.
            *self = Ground::new();
        }
        self.last = at;
        while self.next < PLACED {
            let pl = &PLACEMENTS[self.next as usize];
            if at < pl.at {
                break;
            }
            let mut slot = 0;
            while slot < UNITS && self.units[slot].state != FREE {
                slot += 1;
            }
            if slot == UNITS {
                break;
            }
            // The heading of its path (down the screen when it stands still).
            let base = if pl.vx == 0 && pl.vu == 0 { 8 } else { yaw_to(pl.vx as i16, -pl.vu as i16) };
            let u = &mut self.units[slot];
            u.kind = pl.kind;
            u.state = LIVE;
            u.mode = 0;
            u.n = 0;
            u.hp = HP[pl.kind as usize];
            u.yaw = if pl.kind == HATCH { 0 } else { base };
            u.base = base;
            u.t = LEG_TICKS;
            u.cool = 30 + 23 * (self.next & 3);
            u.id = self.next;
            u.x = pl.x2 as i16 * 16 - 256;
            u.u = pl.u * 8;
            self.next += 1;
        }
        self.next_at = if self.next < PLACED { PLACEMENTS[self.next as usize].at } else { u16::MAX };
        let mut live = 0;
        for i in 0..UNITS {
            if self.units[i].state != FREE {
                step_unit(&mut self.units[i], w, at);
                if self.units[i].state != FREE {
                    live += 1;
                }
            }
        }
        self.live = live;
    }
}

/// What every behaviour needs about the TALON this tick, seen from the unit.
#[derive(Clone, Copy)]
struct Seen {
    /// Heading frame toward the TALON.
    target: u8,
    /// On screen and above the TALON: it may fire.
    above: bool,
    /// Lines from the unit down to the TALON.
    below: i16,
    /// The TALON is right of the unit.
    right: bool,
    tick: u8,
}

#[inline(never)]
fn step_unit(u: &mut Unit, w: &mut World, at: u16) {
    let moving = u.state == LIVE && (u.kind == SKIFF || (u.kind == CRAWLER && u.mode == DRIVE));
    if moving {
        let pl = &PLACEMENTS[u.id as usize];
        u.x += pl.vx as i16;
        u.u = u.u.wrapping_add(pl.vu as i16 as u16);
    }
    let mut d = at as i16 + 200 - (u.u >> 3) as i16;
    if d < -2600 {
        d += PANO_PERIOD as i16;
    } else if d > 2600 {
        d -= PANO_PERIOD as i16;
    }
    u.px = (u.x >> 3) - crate::game::OX;
    u.py = d - crate::game::OY;
    if u.py > 176 + 48 || u.py < -72 || u.px < -48 || u.px > 352 {
        u.state = FREE;
        return;
    }
    if u.state == WRECK {
        u.frame = (WRECK_FRAME - SKIFF_FRAME) as u8 + (u.kind == CRAWLER) as u8;
        return;
    }
    let (plx, ply) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16);
    let below = ply - u.py;
    let seen = Seen { target: yaw_to(plx - u.px, below), above: u.py >= 0 && below > 0, below, right: plx > u.px, tick: w.tick as u8 };
    let (half, score) = match u.kind {
        SKIFF => {
            skiff(u, w, seen);
            (14, 200)
        }
        PILLBOX => {
            pillbox(u, w, seen);
            (11, 200)
        }
        HATCH => {
            hatch(u, w, seen);
            (12, 1000)
        }
        _ => {
            crawler(u, w, seen);
            (13, 400)
        }
    };
    if u.py < -8 || u.py > 184 {
        return;
    }
    let n = hits(w, (u.px, u.py), half);
    // Shut HATCH doors shrug the bolts off.
    if n == 0 || (u.kind == HATCH && u.yaw == 0) {
        return;
    }
    u.hp = u.hp.saturating_sub(n);
    if u.hp == 0 {
        burst(w, u.px, u.py);
        w.add_score(score);
        w.kills = w.kills.wrapping_add(1);
        if u.kind == HATCH {
            for s in 0..SPARKS {
                if !w.sparks[s].alive {
                    w.sparks[s] = Spark { alive: true, x0: u.px, y: (u.py as i32) << 8, t: 16 };
                    break;
                }
            }
        }
        u.state = if u.kind == SKIFF { FREE } else { WRECK };
        u.frame = (WRECK_FRAME - SKIFF_FRAME) as u8 + (u.kind == CRAWLER) as u8;
    }
}

/// SKIFF: every 100 ticks swing the bow one heading toward the TALON for 14 ticks, fire, swing back.
#[inline(never)]
fn skiff(u: &mut Unit, w: &mut World, s: Seen) {
    if u.mode != 0 {
        u.mode -= 1;
        if u.mode == 0 {
            if s.above {
                shoot(w, u.px, u.py, dir_of(s.target), 0);
            }
            u.yaw = u.base;
            u.cool = 100;
        }
    } else if u.cool != 0 {
        u.cool -= 1;
    } else if s.above {
        u.yaw = turn(u.base, s.target);
        u.mode = 14;
    }
    u.frame = u.yaw;
}

/// PILLBOX: track in range, glow 12 ticks when aligned, fire, cool down 90.
#[inline(never)]
fn pillbox(u: &mut Unit, w: &mut World, s: Seen) {
    let in_range = s.above && s.below <= 160;
    if u.mode != 0 {
        u.mode -= 1;
        if u.mode == 0 {
            if s.above {
                shoot(w, u.px, u.py, dir_of(u.yaw), 1);
            }
            u.cool = 90;
        }
    } else {
        if in_range && s.tick & 3 == 0 {
            u.yaw = turn(u.yaw, s.target);
        }
        if u.cool != 0 {
            u.cool -= 1;
        } else if in_range && u.yaw == s.target {
            u.mode = 12;
        }
    }
    u.frame = (PILLBOX_FRAME - SKIFF_FRAME) as u8 + u.yaw + if u.mode != 0 { UNIT_YAWS as u8 } else { 0 };
}

/// HATCH: shut until 40 lines in, then two cycles of open, launch three, close, rest.
#[inline(never)]
fn hatch(u: &mut Unit, w: &mut World, s: Seen) {
    match u.mode {
        SHUT => {
            if u.py >= 40 {
                u.mode = OPENING;
                u.t = 0;
            }
        }
        OPENING => {
            u.t += 1;
            if u.t == 20 {
                u.mode = OPEN;
                u.t = 0;
            }
        }
        OPEN => {
            u.t += 1;
            if u.t == 1 || u.t == 21 || u.t == 41 {
                launch(w, u.px, u.py, s.right);
            }
            if u.t == 60 {
                u.mode = CLOSING;
                u.t = 20;
            }
        }
        CLOSING => {
            u.t -= 1;
            if u.t == 0 {
                u.n += 1;
                u.mode = if u.n == 2 { DONE } else { REST };
                u.t = 90;
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
    u.yaw = match u.mode {
        OPENING | CLOSING => u.t / 4,
        OPEN => 5,
        _ => 0,
    };
    u.frame = (HATCH_FRAME - SKIFF_FRAME) as u8 + u.yaw;
}

/// CRAWLER: drive, stop every 64 px, pivot to the TALON, glow 16 ticks, two shots, pivot back.
#[inline(never)]
fn crawler(u: &mut Unit, w: &mut World, s: Seen) {
    let in_range = s.above && s.below <= 150;
    match u.mode {
        DRIVE => {
            u.t -= 1;
            if u.t == 0 {
                u.t = LEG_TICKS;
                if in_range {
                    u.mode = PIVOT;
                }
            }
        }
        PIVOT => {
            if !in_range {
                u.mode = BACK;
            } else if u.yaw == s.target {
                u.mode = GLOW;
                u.cool = 16;
            } else if s.tick % 3 == 0 {
                u.yaw = turn(u.yaw, s.target);
            }
        }
        GLOW => {
            u.cool -= 1;
            if u.cool == 0 {
                u.mode = FIRE;
                u.n = 2;
            }
        }
        FIRE => {
            if u.cool == 0 {
                if s.above {
                    shoot(w, u.px, u.py, dir_of(u.yaw), 2);
                }
                u.n -= 1;
                u.cool = 8;
                if u.n == 0 {
                    u.mode = BACK;
                }
            } else {
                u.cool -= 1;
            }
        }
        _ => {
            if u.yaw == u.base {
                u.mode = DRIVE;
            } else if s.tick % 3 == 0 {
                u.yaw = turn(u.yaw, u.base);
            }
        }
    }
    let armed = u.mode == GLOW || u.mode == FIRE;
    u.frame = (CRAWLER_FRAME - SKIFF_FRAME) as u8 + u.yaw + if armed { UNIT_YAWS as u8 } else { 0 };
}

/// A WASP out of a HATCH's shaft on `drop_u`, dog-legging toward the TALON's side.
#[inline(never)]
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".text.resident.ground_launch"))]
fn launch(w: &mut World, x: i16, y: i16, right: bool) {
    for k in 0..WASPS {
        if !w.wasps[k].alive {
            w.wasps[k] = Wasp {
                alive: true,
                leader: false,
                curve: DROP | if right { MIRROR } else { 0 },
                seg: 0,
                seg_t: 0,
                angle: 128,
                x: x as i32 * crate::sim::PX,
                y: y as i32 * crate::sim::PX,
            };
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::sim::{FIRE, LEFT, RIGHT};

    #[test]
    fn placements_are_sorted_and_headings_map() {
        for pair in PLACEMENTS[..PLACED as usize].windows(2) {
            assert!(pair[0].at <= pair[1].at, "{pair:?}");
        }
        // The sector aim agrees with the exact angle to within half a step (11.25°) plus rounding.
        for deg in 0..360 {
            let a = (deg as f64).to_radians();
            let (dx, dy) = ((-100.0 * a.sin()).round() as i16, (-100.0 * a.cos()).round() as i16);
            let want = (deg as f64 / 22.5).round() as i16 % 16;
            let got = yaw_to(dx, dy) as i16;
            let off = (got - want).rem_euclid(16);
            assert!(off == 0 || ((deg as f64 / 22.5).fract() - 0.5).abs() < 0.1 && (off == 1 || off == 15), "{deg}°: got {got} want {want}");
        }
        assert_eq!(yaw_to(12, 0), 12, "moving right faces right");
        assert_eq!(yaw_to(0, -12), 0, "moving up the stage faces up");
        assert_eq!(yaw_to(-12, 0), 4, "left");
        assert_eq!(yaw_to(0, 12), 8, "down");
        assert_eq!(dir_of(8), 0, "down is DIRS 0");
        assert_eq!(dir_of(12), 12, "right is DIRS 12");
        assert_eq!(turn(15, 1), 0);
        assert_eq!(turn(1, 15), 0);
    }

    /// Every static unit stands on what the bible says (quays, wall-tops, pits, hangar roofs, taxiways
    /// and runways) and every SKIFF starts on water.
    #[test]
    fn placements_sit_on_the_level() {
        let m = crate::s1::level();
        for pl in &PLACEMENTS[..PLACED as usize] {
            let (x, u) = (pl.x().clamp(4, 316) as f64, pl.u as f64);
            let (z, mat) = crate::s1::surface(&m, x, u).expect("over geometry");
            match pl.kind {
                SKIFF => assert!(mat == crate::s1::SURFACE_WATER || z < 2.0, "{pl:?}: SKIFF on z {z} (mat {mat})"),
                HATCH => assert_eq!(mat, crate::s1::SURFACE_ROOF, "{pl:?}"),
                PILLBOX => assert!(z < 17.0 && mat != crate::s1::SURFACE_WATER, "{pl:?}: z {z} mat {mat}"),
                _ => assert!(z >= 4.0 && z < 7.0, "{pl:?}: CRAWLER on z {z} (mat {mat})"),
            }
            if pl.kind == PILLBOX {
                // Nothing tall right beside it (cranes, fuel tanks; tetrapods and containers are low enough).
                for (dx, du) in [(-10.0, 0.0), (10.0, 0.0), (0.0, -9.0), (0.0, 9.0)] {
                    let (zz, _) = crate::s1::surface(&m, x + dx, u + du).unwrap_or((0.0, 0));
                    assert!(zz < 17.0, "{pl:?}: something {zz} tall at {dx},{du}");
                }
            }
        }
    }

    /// The units come and go with the stage, fire at a TALON below them, and die to TWIN fire.
    #[test]
    fn the_ground_units_fight_and_fall() {
        let mut g = Game::new();
        g.world.track_base = 1_000_000;
        let mut seen = [false; 5];
        let mut wrecks = 0;
        let mut out = [Sprite::default(); crate::game::INSTANCES];
        let mut most = 0;
        let mut kills = 0u16;
        let mut shots = 0;
        for t in 0..20_000u32 {
            // Invulnerable, sweeping across the screen, firing.
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            let input = FIRE | if (t / 120) % 2 == 0 { LEFT } else { RIGHT };
            let bullets = g.world.ebullets.iter().filter(|b| b.kind != 0).count();
            let before = g.world.kills;
            g.step(input);
            kills += g.world.kills.wrapping_sub(before);
            shots += g.world.ebullets.iter().filter(|b| b.kind != 0).count().saturating_sub(bullets);
            for u in &g.ground.units {
                if u.state == LIVE {
                    seen[u.kind as usize] = true;
                }
                wrecks += (u.state == WRECK) as usize;
            }
            let n = crate::game::scene(&g, &mut out);
            most = most.max(g.ground.live);
            for s in &out[..n] {
                assert!(s.frame < FRAMES);
            }
            if g.world.scroll_lines() >= crate::boss::ARRIVE as u32 {
                break;
            }
        }
        assert!(seen[1..].iter().all(|&s| s), "every kind came on: {seen:?}");
        assert!(wrecks > 0, "craters stay behind");
        assert!(kills > 20, "kills {kills}");
        assert!(shots > 20, "shots {shots}");
        assert!(most as usize <= UNITS);
        println!("ground: most {most} units at once, {kills} kills, {shots} new bullets");
    }

    #[test]
    fn a_hatch_launches_wasps_unless_killed() {
        // Alone at the first HATCH, holding fire: it launches its six WASPs.
        let mut g = Game::new();
        g.world.track_base = 1_000_000;
        g.world.scroll = (3468u32 - 1) << 8;
        let mut launched = 0;
        for _ in 0..1200 {
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            let alive = g.world.wasps.iter().filter(|w| w.alive).count();
            g.step(0);
            launched += g.world.wasps.iter().filter(|w| w.alive).count().saturating_sub(alive);
        }
        assert_eq!(launched, 6);
    }
}
