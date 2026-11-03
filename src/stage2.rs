//! S2 TERRACES' units and waves (`docs/story/04-levels/s2-terraces.md`, the enemies in
//! `docs/story/03-enemies/`, their HD models in `crate::hand_s2`). The units share `Ground`'s pool and
//! its resident render-time `Ground::sprites`, which draws S2's kinds from the stage block (`demo::S2_*`).
//!
//! - **PIKE** (air, ground-locked, 1 HP, 100): races along a rail at 4 px/tick: a vine row, the
//!   AQUEDUCT's channel, a village lane or a gravel path (`air2::RAILS`, the level's own lines sampled every 32
//!   px, checked against `crate::s2` by `the_rails_follow_the_level`), nose along it. Some flick one
//!   shot straight down as they pass over the ship (1.5 px/tick).
//! - **KESTREL** (air, 3 HP, 300; both of a pair 500 more): enters from a side edge at line 56 or so,
//!   crosses at 3 px/tick to where the ship was when it entered, banks into a 16-tick turn (the bank is
//!   its telegraph), snaps a 3-way spread (30°) at the ship as it comes round, and climbs away.
//! - **ROOF HATCH** (farmhouse and village roofs, 6 HP, 800): shut until 40 lines in, then twice: its
//!   ridge hatch slides open over 24 ticks, two WASPs rise 20 ticks apart, it shuts, rests 100 ticks.
//!   Shut, it shrugs off TWIN bolts. Its wreck is the blown-open roof.
//! - **BELL TURRET** (rooftops, 10 HP, 500): within 150 lines above the ship the bell steps one of 16
//!   headings toward it every 6 ticks; aligned, its barrels glow 18 ticks, then a 3-way spread; cooldown 110.
//! - **DOME MORTAR** (the small domes, 8 HP, 600): its slit opens over 24 ticks (the telegraph), the tube
//!   swings to the ship and lobs a shell at where the ship is; it holds, swings back, shuts and rests
//!   150 ticks. Shut, it takes half damage.
//! - **Shell** (1 HP, 50): flies 90 ticks to its mark, climbing then falling (it grows, then shrinks);
//!   harmless while high, it is a falling bomb (the sim's own `B_BOMB`, shootable) for its last 20
//!   ticks, then bursts where it lands.
//! - **PILLBOX** (as S1's, in S2's light), and **WASP** strings released from `Ground::strings` along
//!   the sim's own curves.
//!
//! Air units that touch the ship destroy it, as WASPs do: a WASP is put on the ship and the sim's own
//! collision takes it next tick (the vendored sim stays unchanged).
//!
//! The step is two paged phases, run in place of S1's `ground` step (6502 code is large: one would not
//! fit the window): `paging::S2` (this module: the waves, the WASP strings, the ground units, the
//! mortar's lob) every tick something is out or due, then `paging::S2AIR` (`crate::air2`: PIKEs,
//! KESTRELs, shells) while any of those is out. A phase reads only its own tables, so an air unit
//! carries what it needs in its `Unit` from the moment it is placed.

use crate::ground::{Ground, Str, Unit, NO_UNIT};
use crate::ground::{BELL, FREE, KESTREL, LIVE, MORTAR, PIKE, PILLBOX2, ROOF, STRINGS, UNITS, WASPS2};
use crate::sim::track::{CURVES, DROP, HOOK, MIRROR, S_CURVE};
use crate::sim::world::*;

/// One authored S2 event, 6 bytes: due when the stage has travelled `at` lines.
/// - a ground unit: `a` = (x + 32) / 2, standing on line `at + 232` (it scrolls in at the top);
/// - PIKE: `a` rail (`air2::RAILS`), `b` bit 0 from the left, bit 1 fires, `c` ticks behind its string's
///   head (at most 12);
/// - KESTREL: `a` bit 0 from the left, bits 1.. its pair; `b` its line; `c` ticks of delay;
/// - WASP string: `kind` = `WASPS2` | count << 4, `a` x / 2, `b` y + 128, `c` curve (bit 7 mirrored)
///   | 0x40 when the last is the leader.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct P2 {
    pub at: u16,
    pub kind: u8,
    pub a: u8,
    pub b: u8,
    pub c: u8,
}

impl P2 {
    /// A ground unit's world x and line.
    pub const fn x(&self) -> i16 {
        self.a as i16 * 2 - 32
    }
    pub const fn u(&self) -> u16 {
        self.at + LEAD
    }
}

/// Ground units are placed this many lines before they stand at the top of the screen.
const LEAD: u16 = 232;

/// A ground unit standing at (`x`, `u`).
const fn st(kind: u8, x: i16, u: u16) -> P2 {
    assert!(x % 2 == 0 && x >= -32 && x <= 478);
    P2 { at: u - LEAD, kind, a: ((x + 32) / 2) as u8, b: 0, c: 0 }
}
/// A PIKE on `rail` from the left (or right) edge, `delay` ticks (at most 12) behind its string's head;
/// `fires`: one shot down as it passes over the ship.
const fn pike(at: u16, rail: u8, right: bool, delay: u8, fires: bool) -> P2 {
    assert!(delay <= 12);
    P2 { at, kind: PIKE, a: rail, b: right as u8 | (fires as u8) << 1, c: delay }
}
/// A KESTREL entering from the left (or right) edge at line `y`, `delay` ticks after `at`; `pair` tags
/// its pair (the bonus).
const fn kestrel(at: u16, from_left: bool, y: u8, delay: u8, pair: u8) -> P2 {
    P2 { at, kind: KESTREL, a: from_left as u8 | pair << 1, b: y, c: delay }
}
/// A string of `count` WASPs on `curve` from playfield (`x`, `y`), one every 10 ticks; the last is the
/// leader (drops a SPARK) when `leader`.
const fn wasps(at: u16, curve: u8, x: i16, y: i16, count: u8, leader: bool) -> P2 {
    assert!(count >= 1 && count < 16);
    P2 { at, kind: WASPS2 | count << 4, a: (x / 2) as u8, b: (y + 128) as u8, c: curve | (leader as u8) << 6 }
}

/// S2's waves (`04-levels/s2-terraces.md`), placed on `crate::s2`'s geometry, sorted by `at`.
pub static PLACEMENTS: [P2; PLACED] = [
    // Screen 1: six WASPs in a slow S from the right; the leader drops a SPARK.
    wasps(200, S_CURVE | MIRROR, 240, -12, 6, true),
    // Screen 2: three PIKEs race along the vine rows, one a row, holding fire: the rail line.
    pike(330, 0, true, 0, false),
    pike(336, 1, false, 0, false),
    pike(342, 2, true, 0, false),
    // Screen 3: a KESTREL pair crosses left to right; a farmhouse's ROOF HATCH. Screen 4: two
    // PILLBOXes on a terrace wall, six WASPs swooping low across them from the left; another farmhouse.
    st(ROOF, 250, 675),
    st(PILLBOX2, 100, 826),
    kestrel(600, true, 56, 0, 0),
    kestrel(600, true, 56, 9, 0),
    st(PILLBOX2, 220, 834),
    st(ROOF, 70, 926),
    wasps(820, HOOK, 0, 40, 6, false),
    st(ROOF, 250, 1182),
    // Screen 5: five PIKEs down the rows from the top, staggered, each firing once over the ship.
    pike(990, 7, true, 0, true),
    pike(994, 6, false, 0, true),
    pike(998, 5, true, 0, true),
    pike(1002, 4, false, 0, true),
    pike(1006, 3, true, 0, true),
    // Screen 6 (checkpoint 1): two KESTREL pairs crossing in opposite directions, eight WASPs behind.
    kestrel(1200, true, 48, 0, 1),
    kestrel(1200, true, 48, 9, 1),
    kestrel(1200, false, 76, 30, 2),
    kestrel(1200, false, 76, 39, 2),
    wasps(1260, S_CURVE, 64, -12, 8, true),
    // Screen 8: the AQUEDUCT: PILLBOXes on two piers, four PIKEs along its channel.
    st(PILLBOX2, 90, 1694),
    st(PILLBOX2, 250, 1694),
    pike(1552, 8, true, 0, true),
    pike(1552, 8, true, 6, true),
    pike(1552, 8, true, 12, true),
    pike(1558, 8, true, 0, true),
    // Screen 10: the village: BELL TURRETs on the rooftops, ROOF HATCHes.
    st(ROOF, 172, 2034),
    st(ROOF, 56, 2038),
    st(ROOF, 236, 2084),
    st(BELL, 162, 2086),
    st(BELL, 254, 2142),
    st(BELL, 44, 2199),
    st(ROOF, 150, 2250),
    // (Screen 12: the CAMPANILE.)
    // Screen 14 (checkpoint 2): KESTREL pairs from both sides at once, six WASPs in a V.
    kestrel(2800, true, 56, 0, 3),
    kestrel(2800, true, 56, 9, 3),
    kestrel(2800, false, 84, 0, 4),
    kestrel(2800, false, 84, 9, 4),
    wasps(2860, DROP, 152, -12, 3, false),
    wasps(2860, DROP | MIRROR, 152, -12, 3, false),
    // Screen 16: four BELL TURRETs on the village edge firing in turn; three PIKEs down the lanes.
    st(BELL, 12, 3255),
    st(BELL, 290, 3256),
    st(BELL, 20, 3313),
    st(BELL, 284, 3315),
    pike(3180, 11, false, 0, true),
    pike(3186, 10, true, 0, true),
    pike(3192, 9, false, 0, true),
    // Screen 18: the observatory: three DOME MORTARs open in the small domes.
    st(MORTAR, 70, 3560),
    st(MORTAR, 250, 3640),
    st(MORTAR, 150, 3800),
    // Screen 20 (checkpoint 3): six PIKEs along a gravel path, both ways; a KESTREL pair overhead.
    pike(3880, 12, true, 0, true),
    pike(3880, 12, true, 6, true),
    pike(3880, 12, true, 12, true),
    pike(3920, 12, false, 0, true),
    pike(3920, 12, false, 6, true),
    pike(3920, 12, false, 12, true),
    kestrel(4000, true, 56, 0, 5),
    kestrel(4000, true, 56, 9, 5),
    // Screen 22: four DOME MORTARs and two PILLBOXes by the sundial court; ten WASPs rolling in from the
    // left, the leader drops a SPARK.
    st(PILLBOX2, 120, 4330),
    st(PILLBOX2, 186, 4330),
    st(MORTAR, 60, 4420),
    st(MORTAR, 254, 4440),
    wasps(4400, HOOK, 0, 24, 10, true),
    st(MORTAR, 110, 4640),
    st(MORTAR, 230, 4700),
    // (Screen 24: the ASTROLABE.) Until it exists, two strings close the stage.
    wasps(4520, S_CURVE, 48, -12, 4, false),
    wasps(4520, S_CURVE | MIRROR, 256, -12, 4, false),
];
pub const PLACED: usize = 68;

/// Hit points of S2's kinds, from `PIKE`.
const HP: [u8; 7] = [1, 3, 6, 10, 8, 4, 1];

/// KESTREL: `Unit::mode` bit for "from the left" (`air2`).
pub const FROM_LEFT: u8 = 16;
/// PIKE: `Unit::mode` bits: moving left, fires (`air2`; bit 0 is "has fired").
pub const LEFTWARD: u8 = 2;
pub const FIRES: u8 = 4;
/// The helpers both S2 phases need, one copy in each (a phase may not call into another).
macro_rules! s2_helpers {
    () => {
        #[allow(dead_code)]
        /// What a unit sees of the ship this tick.
        #[derive(Clone, Copy)]
        struct Seen {
            x: i16,
            /// Heading frame toward the ship.
            target: u8,
            /// On screen and above the ship: it may fire.
            above: bool,
            /// Lines from the unit down to the ship.
            below: i16,
        }

        #[allow(dead_code)]
        #[inline(never)]
        fn seen(u: &Unit, w: &World) -> Seen {
            let (x, y) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16);
            let below = y - u.py;
            Seen { x, target: yaw_to(x - u.px, below), above: u.py >= 0 && below > 0, below }
        }

        /// Playfield position of a ground-locked unit at stage line `at`.
        #[allow(dead_code)]
        #[inline(never)]
        fn place(u: &mut Unit, at: u16) {
            let mut d = at as i16 + 200 - (u.u >> 3) as i16;
            if d < -2600 {
                d += PANO_PERIOD as i16;
            } else if d > 2600 {
                d -= PANO_PERIOD as i16;
            }
            u.px = (u.x >> 3) - crate::game::OX;
            u.py = d - crate::game::OY;
        }

        /// Destroyed: a burst, the points, the kill; the unit is free (a ground unit then wrecks).
        #[inline(never)]
        fn fall(u: &mut Unit, w: &mut World, score: u16) {
            burst(w, u.px, u.py);
            w.add_score(score as u32);
            w.kills = w.kills.wrapping_add(1);
            u.state = FREE;
        }

        /// The heading frame (16, 0 = up the screen, counter-clockwise) closest to (`dx`, `dy`) (as
        /// `ground`'s).
        #[inline(never)]
        fn yaw_to(dx: i16, dy: i16) -> u8 {
            let (ax, ay) = (dx.abs(), dy.abs());
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

        /// The 48-direction index (`boss::DIRS`, 0 straight down) of heading frame `yaw`.
        fn dir_of(yaw: u8) -> u8 {
            let d = 3 * yaw + 24;
            if d >= 48 { d - 48 } else { d }
        }

        /// Three rounds 15° apart, the middle one along heading frame `yaw`.
        #[inline(never)]
        fn spread(w: &mut World, x: i16, y: i16, yaw: u8, boost: u8) {
            let k = dir_of(yaw);
            shoot(w, x, y, if k < 2 { k + 46 } else { k - 2 }, boost);
            shoot(w, x, y, k, boost);
            shoot(w, x, y, if k >= 46 { k - 46 } else { k + 2 }, boost);
        }
    };
}
pub(crate) use s2_helpers;

/// The resident step for S2 (it only pages and calls): the waves phase while a placement is due (or a string is
/// releasing: `waves` keeps itself due), then the ground units' and the air units' phases while any of each is out.
#[inline(never)]
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".text.resident.s2_run"))]
pub fn run(g: &mut Ground, w: &mut World, at: u16) {
    if at >= g.next_at || at < g.last {
        crate::paging::page(crate::paging::S2);
        waves(g, w, at);
    }
    if g.gnd != 0 {
        crate::paging::page(crate::paging::S2GND);
        crate::gnd2::step(g, w, at);
    }
    if g.air != 0 {
        crate::paging::page(crate::paging::S2AIR);
        crate::air2::step(g, w, at);
    }
}

/// The waves (paged: `paging::S2`): placements come due, units are placed, the WASP strings release.
#[inline(never)]
pub fn waves(g: &mut Ground, w: &mut World, at: u16) {
    // S1's track (the vendored sim's) never comes due: S2's waves are these placements. (This runs on
    // the stage's first tick: line 0 is before the last line seen, or a new game's first placement.)
    w.track_base = 0x7FFF_FFFF;
    if at < g.last {
        // The stage looped or a new game began: start the placements over.
        *g = Ground::new();
    }
    g.last = at;
    while (g.next as usize) < PLACED {
        let pl = PLACEMENTS[g.next as usize];
        if at < pl.at {
            break;
        }
        if pl.kind & 15 == WASPS2 {
            let mut s = 0;
            while s < STRINGS && g.strings[s].left != 0 {
                s += 1;
            }
            if s == STRINGS {
                break;
            }
            g.strings[s] = Str { left: pl.kind >> 4, timer: 0, rec: g.next };
        } else {
            let mut slot = 0;
            while slot < UNITS && g.units[slot].state != FREE {
                slot += 1;
            }
            if slot == UNITS {
                break;
            }
            spawn(&mut g.units[slot], &pl, g.next);
            if pl.kind == PIKE || pl.kind == KESTREL {
                g.air += 1;
            } else {
                g.gnd += 1;
            }
        }
        g.next += 1;
    }
    g.next_at = if (g.next as usize) < PLACED { PLACEMENTS[g.next as usize].at } else { u16::MAX };
    for s in 0..STRINGS {
        if g.strings[s].left != 0 {
            release(&mut g.strings[s], w);
            // Due again next tick while a string is releasing.
            g.next_at = at;
        }
    }
}

/// Placement `pl` (number `id`) as unit `u`.
#[inline(never)]
fn spawn(u: &mut Unit, pl: &P2, id: u8) {
    *u = NO_UNIT;
    let k = pl.kind;
    u.kind = k;
    u.state = LIVE;
    u.hp = HP[(k - PIKE) as usize];
    u.yaw = 8;
    u.base = 8;
    u.id = id;
    if k == PIKE {
        // Off its edge, its delay's worth of travel behind.
        let lead = 4 * pl.c as i16;
        let right = pl.b & 1 != 0;
        u.x = 8 * if right { -16 - lead } else { 336 + lead };
        u.mode = if right { 0 } else { LEFTWARD } | if pl.b & 2 != 0 { FIRES } else { 0 };
        u.cool = pl.a;
    } else if k == KESTREL {
        let left = pl.a & 1 != 0;
        u.x = if left { -40 * 16 } else { 344 * 16 };
        u.u = pl.b as u16 * 16;
        u.t = pl.c;
        u.yaw = if left { 12 } else { 4 };
        u.mode = if left { FROM_LEFT } else { 0 };
        u.id = pl.a >> 1;
    } else {
        u.x = pl.a as i16 * 16 - 256;
        u.u = pl.u() * 8;
        u.cool = 30 + 23 * (id & 3);
    }
}

/// Release the next WASP of a string every 10 ticks, along the sim's own curves.
#[inline(never)]
fn release(s: &mut Str, w: &mut World) {
    if s.timer != 0 {
        s.timer -= 1;
        return;
    }
    let pl = &PLACEMENTS[s.rec as usize];
    s.left -= 1;
    s.timer = 9;
    let curve = pl.c & !0x40;
    let c = &CURVES[(curve & !MIRROR) as usize];
    let heading = if curve & MIRROR != 0 { c.heading.wrapping_neg() } else { c.heading };
    for k in 0..WASPS {
        let wp = &mut w.wasps[k];
        if !wp.alive {
            wp.alive = true;
            wp.leader = s.left == 0 && pl.c & 0x40 != 0;
            wp.curve = curve;
            wp.seg = 0;
            wp.seg_t = 0;
            wp.angle = heading;
            // Q8 from whole px: 16-bit math, then a byte move (no 32-bit multiply on the 6502).
            wp.x = ((pl.a as i16 * 2) as i32) << 8;
            wp.y = ((pl.b as i16 - 128) as i32) << 8;
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::demo::*;
    use crate::air2::{RAILS, RAIL_SEGS, RAIL_STEP, RAIL_X0, SHELL_FLIGHT};
    use crate::ground::{SHELL, WRECK};
    use crate::game::Game;
    use crate::sim::{FIRE as STICK_FIRE, LEFT, RIGHT};

    /// The rails sampled from the level (`crate::s2`): every `RAIL_STEP` px from `RAIL_X0`.
    fn rail_lines() -> Vec<Vec<f64>> {
        let xs: Vec<f64> = (0..=RAIL_SEGS).map(|i| (RAIL_X0 + i as i16 * RAIL_STEP) as f64).collect();
        let spots = crate::s2::spots();
        let lane = |near: f64| *spots.lanes.iter().min_by(|a, b| (*a - near).abs().total_cmp(&(*b - near).abs())).unwrap();
        let mut out = Vec::new();
        for r in [1, 3, 5] {
            out.push(xs.iter().map(|&x| crate::s2::row_u(7, r, x)).collect());
        }
        for (k, r) in [(16, 1), (16, 4), (17, 0), (17, 3), (17, 5)] {
            out.push(xs.iter().map(|&x| crate::s2::row_u(k, r, x)).collect());
        }
        out.push(xs.iter().map(|_| crate::s2::AQUEDUCT_U).collect());
        for near in [3230.0, 3282.0, 3341.0] {
            let u = lane(near);
            out.push(xs.iter().map(|_| u).collect());
        }
        // The gravel path as a line over x (flat beyond its ends): invert x(f) by bisection.
        let path: Vec<(f64, f64)> = (0..=400).map(|i| crate::s2::path_at(3, i as f64 / 400.0)).collect();
        out.push(
            xs.iter()
                .map(|&x| {
                    if x <= path[0].0 {
                        path[0].1
                    } else if x >= path[400].0 {
                        path[400].1
                    } else {
                        let i = path.windows(2).position(|w| w[0].0 <= x && x <= w[1].0).unwrap();
                        let (a, b) = (path[i], path[i + 1]);
                        a.1 + (b.1 - a.1) * (x - a.0) / (b.0 - a.0).max(1e-9)
                    }
                })
                .collect(),
        );
        out
    }

    fn rail_table() -> Vec<(u16, [i8; RAIL_SEGS])> {
        rail_lines()
            .iter()
            .map(|l| {
                let r: Vec<i32> = l.iter().map(|v| v.round() as i32).collect();
                let mut du = [0i8; RAIL_SEGS];
                for s in 0..RAIL_SEGS {
                    du[s] = (r[s + 1] - r[s]) as i8;
                }
                (r[0] as u16, du)
            })
            .collect()
    }

    #[test]
    fn the_rails_follow_the_level() {
        let want = rail_table();
        let have: Vec<(u16, [i8; RAIL_SEGS])> = RAILS.iter().map(|r| (r.u0, r.du)).collect();
        if want != have {
            for (u0, du) in &want {
                println!("    Rail {{ u0: {u0}, du: {du:?} }},");
            }
            panic!("RAILS differ from the level's lines (the table above)");
        }
    }

    fn s2_game() -> Game {
        let mut g = Game::new();
        g.clear = crate::clear::Clear::at(1, 1);
        g
    }

    /// The level's own line under rail `rail` at `x` (exact, from `crate::s2`).
    fn exact(rail: usize, x: f64, lines: &[Vec<f64>]) -> f64 {
        match rail {
            0..=2 => crate::s2::row_u(7, [1, 3, 5][rail], x),
            3..=7 => {
                let (k, r) = [(16, 1), (16, 4), (17, 0), (17, 3), (17, 5)][rail - 3];
                crate::s2::row_u(k, r, x)
            }
            _ => {
                let f = ((x - RAIL_X0 as f64) / RAIL_STEP as f64).clamp(0.0, RAIL_SEGS as f64 - 1e-9);
                let (i, fr) = (f as usize, f.fract());
                lines[rail][i] + (lines[rail][i + 1] - lines[rail][i]) * fr
            }
        }
    }

    #[test]
    fn s2_placements_are_sorted_and_valid() {
        for w in PLACEMENTS.windows(2) {
            assert!(w[0].at <= w[1].at, "{:?}", w);
        }
        for pl in &PLACEMENTS {
            match pl.kind & 15 {
                PIKE => assert!((pl.a as usize) < RAILS.len() && pl.c <= 12),
                KESTREL => assert!(pl.b >= 24 && pl.b <= 96, "KESTRELs cross above line 96"),
                WASPS2 => assert!((pl.c & !MIRROR & !0x40) < 3 && pl.kind >> 4 >= 1),
                ROOF | BELL | MORTAR | PILLBOX2 => {}
                k => panic!("kind {k}"),
            }
        }
    }

    /// Every rooftop unit sits on a roof, every mortar in a small dome, every PILLBOX on stone (terrace
    /// walls, the AQUEDUCT's deck, the sundial court), as the bible places them.
    #[test]
    fn s2_placements_sit_on_the_level() {
        let m = crate::s2::level_at(0.0);
        let spots = crate::s2::spots();
        for pl in PLACEMENTS.iter().filter(|p| matches!(p.kind, ROOF | BELL | MORTAR | PILLBOX2)) {
            let (x, u) = (pl.x() as f64, pl.u() as f64);
            let (z, mat) = crate::s2::surface(&m, x.clamp(1.0, 319.0), u).expect("over the level");
            match pl.kind {
                ROOF => {
                    assert_eq!(mat, crate::s2::SURFACE_ROOF, "{pl:?}");
                    let h = spots.houses.iter().min_by(|a, b| (a.x - x).hypot(a.u - u).total_cmp(&(b.x - x).hypot(b.u - u))).unwrap();
                    assert!((h.x - x).abs() < 3.0 && (h.u - u).abs() < 3.0 && h.w >= 30.0 && h.w >= h.l, "{pl:?} on {h:?}");
                }
                BELL => assert_eq!(mat, crate::s2::SURFACE_ROOF, "{pl:?}"),
                MORTAR => {
                    // (x is even in a placement: a dome at an odd x gets the pixel left of its centre.)
                    assert!(crate::s2::DOMES.iter().any(|d| (d.0 - x).abs() <= 1.0 && d.1 == u), "{pl:?}: not a dome's centre");
                    assert!(mat == crate::s2::SURFACE_DOME || mat == crate::s2::SURFACE_SHADE, "{pl:?}: mat {mat}");
                }
                _ => assert!(
                    z > 40.0 && matches!(mat, crate::s2::SURFACE_STONE | crate::s2::SURFACE_SHADE | crate::s2::SURFACE_GRAVEL | crate::s2::SURFACE_SOIL),
                    "{pl:?}: z {z} mat {mat}"
                ),
            }
        }
    }

    /// Every PIKE stays on its line (the level's own) for its whole run.
    #[test]
    fn pikes_ride_their_rails() {
        let lines = rail_lines();
        let mut g = s2_game();
        let mut checks = 0;
        let mut worst = 0.0f64;
        let mut seen = [false; 13];
        while g.world.scroll_lines() < 4200 {
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            g.step(0);
            for u in g.ground.units.iter().filter(|u| u.state == LIVE && u.kind == PIKE) {
                let rail = u.cool as usize;
                let x = u.x as f64 / 8.0;
                let off = (u.u as f64 / 8.0 - exact(rail, x, &lines)).abs();
                worst = worst.max(off);
                assert!(off < 2.5, "rail {rail} at x {x}: {off} lines off");
                seen[rail] = true;
                checks += 1;
            }
        }
        assert!(seen.iter().all(|&s| s), "every rail ran: {seen:?}");
        assert!(checks > 500, "{checks}");
        println!("PIKEs: {checks} checks, worst {worst:.2} lines off their rails");
    }

    /// S2's units come and go with the stage, fire at a ship below them, and fall to TWIN fire.
    #[test]
    fn the_s2_units_fight_and_fall() {
        let mut g = s2_game();
        let mut seen = [false; 12];
        let mut out = [crate::render::Sprite::default(); crate::game::INSTANCES];
        let (mut kills, mut shots, mut wrecks, mut most, mut wasps) = (0u16, 0, 0, 0, 0);
        let mut t = 0u32;
        while g.world.scroll_lines() < 4900 {
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            let input = STICK_FIRE | if (t / 120) % 2 == 0 { LEFT } else { RIGHT };
            let bullets = g.world.ebullets.iter().filter(|b| b.kind != 0).count();
            let alive = g.world.wasps.iter().filter(|w| w.alive).count();
            let before = g.world.kills;
            g.step(input);
            t += 1;
            kills += g.world.kills.wrapping_sub(before);
            shots += g.world.ebullets.iter().filter(|b| b.kind != 0).count().saturating_sub(bullets);
            wasps += g.world.wasps.iter().filter(|w| w.alive).count().saturating_sub(alive);
            for u in &g.ground.units {
                if u.state == LIVE {
                    seen[u.kind as usize] = true;
                }
                wrecks += (u.state == WRECK) as usize;
            }
            most = most.max(g.ground.units.iter().filter(|u| u.state != FREE).count());
            let n = crate::game::scene(&g, &mut out);
            for s in &out[..n] {
                // Only S2's block and the common frames: nothing of S1's stage.
                assert!(s.frame < S2_FRAMES || (DRONE_FRAME..KEEL_FRAME).contains(&s.frame) || s.frame >= PEREGRINE_FRAME, "frame {}", s.frame);
                assert!(s.frame < FRAMES);
            }
        }
        assert!(seen[PIKE as usize..=SHELL as usize].iter().all(|&s| s), "every kind came on: {seen:?}");
        assert!(wrecks > 0 && kills > 30 && shots > 30 && wasps > 30, "kills {kills}, shots {shots}, wrecks {wrecks}, wasps {wasps}");
        println!("S2: {kills} kills, {shots} new bullets, {wasps} WASPs, most {most} units out at once, score {}", g.world.score);
    }

    /// A shell climbs, falls and bursts where the ship was when the mortar fired.
    #[test]
    fn shells_land_where_the_ship_was() {
        let mut g = s2_game();
        g.world.scroll = (3328u32 + 40) << 8;
        let mut landed = 0;
        let mut prev: Vec<Unit> = Vec::new();
        for _ in 0..1500 {
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            g.step(0);
            let (px, py) = ((g.world.player.x >> 8) as i16, (g.world.player.y >> 8) as i16);
            for (i, u) in g.ground.units.iter().enumerate() {
                let was = prev.get(i).copied().unwrap_or_default();
                if was.kind == SHELL && was.state == LIVE && was.t == SHELL_FLIGHT - 1 && (u.state == FREE || u.kind != SHELL) {
                    let (dx, dy) = (was.px - px, was.py - py);
                    assert!(dx.abs() <= 4 && dy.abs() <= 4, "a shell landed {dx},{dy} from the ship");
                    landed += 1;
                }
            }
            prev = g.ground.units.to_vec();
        }
        assert!(landed >= 2, "{landed} shells landed");
    }

    /// ACE flies TERRACES: it clears units, keeps the ship and scores.
    #[test]
    fn the_ace_flies_s2() {
        let mut g = s2_game();
        let mut mind = crate::ace::Mind::new();
        let mut t = 0;
        while g.world.scroll_lines() < 4900 && t < 12_000 {
            let input = crate::ace::fly(&mut g, &mut mind);
            g.step(input);
            t += 1;
        }
        println!("ACE in S2: {} deaths, {} kills, score {} by line {}", g.world.deaths, g.world.kills, g.world.score, g.world.scroll_lines());
        assert!(g.world.deaths <= 2, "{} deaths", g.world.deaths);
        assert!(g.world.kills > 60);
    }
}
