//! The ASTROLABE (S2 boss, `docs/story/05-bosses/s2-astrolabe.md`, HD model in `docs/story/hd/models/
//! s2.md`): the great dome at the top of TERRACES (`s2::GREAT_DOME`) is level geometry with its shutters
//! shut; the fight draws the body (shutters rolling apart, 8 poses), the three rings turning over it
//! and the lens. It never moves; it aims. Every attack is a combination lock.
//!
//! - It comes into view at `ARRIVE`; at `HOLD` the scroll stops with the dome's centre at playfield line
//!   64 (the bible's), and the shutters roll apart (8 ticks a pose).
//! - Phase 1: the three rings are the targets (the bible's 35 HP each, tripled for TWIN as the
//!   CAMPANILE's). A bolt hits the first ring band it flies into (squared distance from the centre,
//!   16-bit); a destroyed ring is gone, its volley with it. The lens shrugs bolts off, except for 60
//!   ticks after a LENS FLARE (down to two thirds: the rest waits for phase 2).
//!   - SIGHT LANE: the rings turn their notches into line and a dotted guide drops from the lens
//!     (45 ticks), then a lane of rounds falls from the lens, sweeping 96 px across at 1 px/tick from
//!     the side of the ship it starts on.
//!   - RING VOLLEY: one live ring spins up (20 ticks), then fires 6 rounds from its notches, turned the
//!     way it spins (the rounds spiral).
//!   - LENS FLARE: the rings stop and the lens pulses (30 ticks), then an 8-round cone at the ship
//!     (70 degrees, 1.75 px/tick), and the lens stays open for 60 ticks.
//!   - DOME MORTARS: the arena's own (`stage2::PLACEMENTS`' last two, on the side domes) lob throughout.
//! - Phase 2 (all rings down): `hot` goes yellow, the lens is bare (the bible's 75, tripled); between
//!   attacks the shutters half-close for 40 ticks, hiding it. PINCER LANES (two lanes close from the edges
//!   to 48 px apart and open again), STARBURST (the lens pulses, then three 16-round rings 14 ticks
//!   apart, each turned 11 degrees on), LENS FLARE.
//! - Defeat: 40,000 points, bursts across the dome, the shutters close for the last time (the stage
//!   clear greys the crystal out); the stage clear (`crate::clear`) follows its state as the QUEEN's.
//!
//! The step and the render-time `sprites` are the paged phase `ASTRO` (`paging::ASTRO`). 16-bit math only.

use crate::clear::Clear;
use crate::demo::*;
use crate::ground::burst;
use crate::render::Sprite;
use crate::sim::actors::B_ROUND;
use crate::sim::world::*;

/// `Astrolabe::state` (the GANTRY QUEEN's numbers: the stage clear and the attract pilot read either).
pub const ABSENT: u8 = 0;
pub const ENTERING: u8 = 1;
pub const PHASE1: u8 = 2;
pub const PHASE2: u8 = 3;
pub const DYING: u8 = 4;
pub const GONE: u8 = 5;
/// After the bursts: the shutters close over the dead dome; GONE once shut.
pub const WRECK: u8 = 6;
const _: () = assert!(DYING == crate::boss::DYING && GONE == crate::boss::GONE && WRECK == crate::boss::WRECK);

/// Stage lines: the dome comes into view, then the scroll holds with its centre at playfield line 64.
pub const ARRIVE: u16 = 4700;
pub const HOLD: u16 = 4872;
/// The dome (`s2::GREAT_DOME`): world x and line.
const DOME_X: i16 = 160;
const DOME_U: i16 = 5000;
/// Its centre in playfield px.
const CX: i16 = DOME_X - crate::game::OX;
/// The bible's 35 per ring and 75 for the lens, tripled for HD's TWIN (as the CAMPANILE's).
pub const RING_HP: u8 = 105;
pub const LENS_HP: u8 = 225;
pub const RING_SCORE: u32 = 1_000;
pub const SCORE: u32 = 40_000;
/// The lens: open this long after a LENS FLARE (phase 1), when it takes hits down to `LENS_FLOOR` (the
/// rest is phase 2's: the rings must fall first).
const OPEN_TICKS: u8 = 60;
const LENS_FLOOR: u8 = LENS_HP / 3 * 2;

/// Patterns.
const LANE: u8 = 0;
const VOLLEY: u8 = 1;
const FLARE: u8 = 2;
const PINCER: u8 = 3;
const STARBURST: u8 = 4;
const NONE: u8 = 0xFF;

/// Squared radii (px) of the hit bands, outer, middle and inner ring (`hand_s2::RINGS`, a couple of px
/// wider each way: a TWIN bolt moves 8 px a tick), and the lens.
const BANDS: [(i16, i16); 3] = [(94 * 94, 112 * 112), (64 * 64, 80 * 80), (36 * 36, 50 * 50)];
const LENS_R2: i16 = 22 * 22;
/// Notch radius of each ring (multiples of a `DIRS` vector, 80 = 1.25 px/tick: shifts in `notch`).
const LANE_LEN: u8 = 96;
const PINCER_LEN: u8 = 120;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Astrolabe {
    pub state: u8,
    /// The dome's centre, playfield line.
    pub y: i16,
    /// Hit points: outer, middle, inner ring, lens.
    pub hp: [u8; 4],
    /// Ring spin samples (16 over 60 degrees, wrapping).
    pub spin: [u8; 3],
    pub tick: u8,
    pub pat: u8,
    pub turn: u8,
    /// Ticks into the attack (0 while telegraphing) and the telegraph's ticks left (or the rest's).
    pub step: u8,
    pub timer: u8,
    /// The ring a volley comes from.
    pub ring: u8,
    /// SIGHT LANE's x (playfield px) and its sweep (+1 or -1).
    pub lane: i16,
    pub dir: i8,
    /// Shutter pose (0 shut .. 7 open), the lens's open ticks left (phase 1) and the phase-2 shutter
    /// cycle's ticks left.
    pub pose: u8,
    pub open: u8,
    pub shut: u8,
    /// The `hot` phase 2 painted over (restored if a game ends mid-fight).
    pub hot: u8,
    /// The playfield point to shoot now (`crate::ace`); y `boss::NO_AIM` when there is none.
    pub aim: [i16; 2],
}

impl Default for Astrolabe {
    fn default() -> Self {
        Astrolabe::new()
    }
}

const DIRS: [(i8, i8); 48] = crate::boss::DIRS;

impl Astrolabe {
    #[inline(always)]
    pub const fn new() -> Astrolabe {
        Astrolabe { state: ABSENT, y: 0, hp: [0; 4], spin: [0; 3], tick: 0, pat: NONE, turn: 0, step: 0, timer: 0, ring: 0, lane: 0, dir: 1, pose: 0, open: 0, shut: 0, hot: 0, aim: [0, crate::boss::NO_AIM] }
    }

    /// Whether it is fighting (the attract pilot's boss).
    pub fn fighting(&self) -> bool {
        self.state == PHASE1 || self.state == PHASE2
    }

    /// Whether it is drawn (resident: `game::scene` pages the phase for `sprites` only then): the shut
    /// dome is the level's own.
    #[inline(always)]
    pub fn shown(&self) -> bool {
        self.pose != 0
    }

    /// Render time (paged: `paging::ASTRO`): the body, the live rings over it, the lens, the guide dots.
    #[inline(never)]
    pub fn sprites(&self, out: &mut [Sprite; crate::game::INSTANCES], n: &mut usize) {
        let y = self.y;
        crate::game::put(out, n, S2_ASTRO_FRAME + self.pose as u16, CX, y, ASTRO_W, ASTRO_H, -4);
        if self.pose >= 4 {
            let mut i = 0;
            while i < 3 {
                if self.hp[i] != 0 {
                    let (w, h) = RING_SIZE[i];
                    crate::game::put(out, n, S2_RING_FRAME + RING_SPINS * i as u16 + self.spin[i] as u16, CX, y, w, h, -3);
                }
                i += 1;
            }
        }
        if self.pose == 7 && self.state != WRECK {
            crate::game::put(out, n, S2_LENS_FRAME + self.lens_frame() as u16, CX, y, LENS_W, LENS_H, -2);
        }
        if self.step == 0 && self.timer != 0 && (self.pat == LANE || self.pat == PINCER) {
            // The guide: dots down the lane(s) to come, falling a line a tick.
            let (a, b) = if self.pat == LANE { (self.lane, self.lane) } else { (PINCER_X0, 304 - PINCER_X0) };
            let drop = (self.timer & 15) as i16;
            let mut k = 0;
            while k < 5 {
                let dy = y + 40 + 24 * k - drop;
                crate::game::put(out, n, S2_SHELL_FRAME + 3, a, dy, SHELL_W, SHELL_H, -1);
                if a != b {
                    crate::game::put(out, n, S2_SHELL_FRAME + 3, b, dy, SHELL_W, SHELL_H, -1);
                }
                k += 1;
            }
        }
    }

    /// The lens's frame: pulsing before a flare or a starburst, wide while open, else a slow breath.
    fn lens_frame(&self) -> u8 {
        if self.step == 0 && self.timer != 0 && (self.pat == FLARE || self.pat == STARBURST) {
            (self.timer >> 1) & 3
        } else if self.open != 0 {
            3
        } else {
            (self.tick >> 4) & 1
        }
    }

    /// One tick at stage line `at`, after the world's (paged: `paging::ASTRO`). The fight's ticks go to
    /// `clear.fight` (BOSS TIME); phase 2 paints `clear.pal`'s `hot` yellow.
    #[inline(never)]
    pub fn step(&mut self, w: &mut World, clear: &mut Clear, at: u16) {
        if at < ARRIVE {
            if self.state == PHASE2 {
                // A game ended mid-fight: the stage's own `hot` again.
                clear.pal[3] = self.hot;
            }
            *self = Astrolabe::new();
            return;
        }
        self.tick = self.tick.wrapping_add(1);
        match self.state {
            ABSENT => {
                *self = Astrolabe::new();
                if at > HOLD {
                    // Joined past the dome (a test, a start line): it is behind us.
                    self.state = GONE;
                    return;
                }
                self.state = ENTERING;
                self.hp = [RING_HP, RING_HP, RING_HP, LENS_HP];
            }
            ENTERING => {
                if at >= HOLD {
                    w.scroll = (HOLD as u32) << 8;
                    if self.tick & 7 == 0 {
                        self.pose += 1;
                        if self.pose == 7 {
                            self.state = PHASE1;
                            self.timer = 40;
                        }
                    }
                }
            }
            PHASE1 | PHASE2 => {
                // The scroll stops for the fight (a constant store: no 32-bit math).
                w.scroll = (HOLD as u32) << 8;
                clear.fight += 1;
                if self.open != 0 {
                    self.open -= 1;
                }
                self.take_hits(w, clear);
                if self.state == PHASE1 || self.state == PHASE2 {
                    self.shutters();
                    self.attack(w);
                    self.turn_rings();
                }
            }
            DYING => {
                w.scroll = (HOLD as u32) << 8;
                if self.timer & 7 == 0 {
                    let t = self.timer as i16;
                    burst(w, CX + (t * 37 & 127) - 64, self.y + (t * 23 & 63) - 16);
                }
                self.timer -= 1;
                if self.timer == 0 {
                    self.state = WRECK;
                }
            }
            WRECK => {
                // The shutters close for the last time; the shut dome is the level's.
                if self.tick & 7 == 0 {
                    self.pose -= 1;
                    if self.pose == 0 {
                        self.state = GONE;
                    }
                }
            }
            _ => {}
        }
        self.y = at as i16 + 200 - DOME_U - crate::game::OY;
    }

    /// TWIN bolts against the rings' bands and the lens: each bolt is spent on the first live part it
    /// flies into; the lens is armoured but while open. Also sets the attract pilot's aim.
    #[inline(never)]
    fn take_hits(&mut self, w: &mut World, clear: &mut Clear) {
        let lens = self.pose == 7 && (self.state == PHASE2 || (self.open != 0 && self.hp[3] > LENS_FLOOR));
        if self.pose >= 4 {
            // Every bolt: the left of each pair, then the right (4 px either side of its centre).
            let mut j = 0;
            while j < 2 * PAIRS {
                let (q, b) = (j >> 1, 1 + (j as u8 & 1));
                j += 1;
                let p = w.pairs[q];
                let dx = p.x + if b == 1 { 4 - CX } else { -4 - CX };
                let dy = p.y - self.y;
                if p.alive & b == 0 || dx.abs() >= 112 || dy.abs() >= 112 {
                    continue;
                }
                let part = self.part_at(dx * dx + dy * dy);
                if part < 4 {
                    w.pairs[q].alive &= !b;
                    if (part < 3 || lens) && self.hp[part] != 0 {
                        self.hp[part] -= 1;
                        if self.hp[part] == 0 {
                            self.fall(w, clear, part);
                        }
                    }
                }
            }
        }
        if self.state == PHASE1 && self.hp[3] < LENS_FLOOR {
            // Phase 1 leaves the lens its floor, however many bolts landed this tick.
            self.hp[3] = LENS_FLOOR;
        }
        // The pilot's aim (read while it fights): the inner ring from below (the pilot closes in under it,
        // inside the outer rings' circle, so its bolts start past their bands), then the middle and the
        // outer at their right flanks (a bolt there misses the rings inside), the lens while it is open.
        let (ax, ay) = if self.hp[2] != 0 {
            (0, 43)
        } else if self.hp[1] != 0 {
            (70, 30)
        } else if self.hp[0] != 0 {
            (86, 56)
        } else {
            (0, if lens { 0 } else { -1 })
        };
        self.aim = [CX + ax, if ay < 0 { crate::boss::NO_AIM } else { self.y + ay }];
    }

    /// The live part a bolt at squared distance `d2` from the centre is in: a ring's band (0 outer .. 2
    /// inner), the lens (3) or none (4).
    #[inline(never)]
    fn part_at(&self, d2: i16) -> usize {
        let mut i = 0;
        while i < 3 {
            if self.hp[i] != 0 && d2 >= BANDS[i].0 && d2 < BANDS[i].1 {
                return i;
            }
            i += 1;
        }
        if d2 < LENS_R2 && self.pose == 7 { 3 } else { 4 }
    }

    /// Part `part` (a ring, or the lens: 3) destroyed.
    fn fall(&mut self, w: &mut World, clear: &mut Clear, part: usize) {
        if part == 3 {
            self.state = DYING;
            self.timer = 96;
            self.pat = NONE;
            w.add_score(SCORE);
            w.kills = w.kills.wrapping_add(1);
            return;
        }
        w.add_score(RING_SCORE);
        burst(w, CX, self.y + [103, 72, 43][part]);
        if self.hp[0] == 0 && self.hp[1] == 0 && self.hp[2] == 0 {
            // Phase 2: the lens is bare and `hot` goes yellow.
            self.state = PHASE2;
            self.hot = clear.pal[3];
            clear.pal[3] = crate::boss::PHASE2_HOT;
            self.pat = NONE;
            self.timer = 40;
            self.turn = 0;
        }
    }

    /// Phase 2: the shutters half-close between attacks (the lens hidden), then reopen.
    fn shutters(&mut self) {
        if self.shut != 0 {
            self.shut -= 1;
        }
        let want = if self.shut > 12 { 3 } else { 7 };
        if self.tick & 1 == 0 {
            if self.pose < want {
                self.pose += 1;
            } else if self.pose > want {
                self.pose -= 1;
            }
        }
    }

    /// The rings turn at their own speeds and ways (outer every 4 ticks, middle back every 4 between them,
    /// inner every 2); a lane's telegraph brings their notches into line, a flare's stops them, a volley's spins its
    /// ring up.
    fn turn_rings(&mut self) {
        let tele = self.step == 0 && self.timer != 0;
        let mut i = 0;
        while i < 3 {
            let s = self.spin[i];
            self.spin[i] = if self.pat == LANE || self.pat == PINCER {
                // Into line (sample 0) by the short way, and held there.
                if s == 0 { 0 } else if s < 8 { s - 1 } else { (s + 1) & 15 }
            } else if self.pat == FLARE && tele {
                s
            } else if (self.pat == VOLLEY && tele && self.ring == i as u8) || self.tick & [3, 3, 1][i] == [0, 2, 1][i] {
                if i == 1 { s.wrapping_sub(1) & 15 } else { (s + 1) & 15 }
            } else {
                s
            };
            i += 1;
        }
    }

    #[inline(never)]
    fn attack(&mut self, w: &mut World) {
        if self.timer != 0 {
            self.timer -= 1;
            return;
        }
        if self.shut > 12 {
            return;
        }
        if self.pat == NONE {
            self.start(w);
            return;
        }
        self.step = self.step.wrapping_add(1);
        let s = self.step;
        let done = match self.pat {
            LANE => {
                self.lane += self.dir as i16;
                if s % 5 == 1 {
                    fire(w, self.lane, self.y + 12, 0, FAST);
                }
                s == LANE_LEN
            }
            PINCER => {
                // In from the edges to 48 px apart, a pause, out again.
                let mut d = if s < 60 { s } else { PINCER_LEN - s };
                if d > 56 {
                    d = 56;
                }
                let x = PINCER_X0 + 2 * d as i16;
                if s % 5 == 1 {
                    fire(w, x, self.y + 12, 0, FAST);
                    fire(w, 304 - x, self.y + 12, 0, FAST);
                }
                s == PINCER_LEN
            }
            VOLLEY => {
                // Six rounds from the ring's notches, turned 22.5 degrees the way it spins.
                let r = self.ring as usize;
                let base = 12 + (self.spin[r] + 1) / 2;
                let twist = if r == 1 { 45 } else { 3 };
                let mut k = 0;
                while k < 6 {
                    let d = wrap(base + 8 * k);
                    let (vx, vy) = DIRS[d as usize];
                    fire(w, CX + notch(vx, r), self.y + notch(vy, r), d + twist, BASE);
                    k += 1;
                }
                true
            }
            FLARE => {
                // A 70-degree cone of 8 at the ship, 1.75 px/tick; then the lens stays open.
                let a = self.cone_at_ship(w);
                let mut k = 0;
                while k < 9 {
                    if k != 4 {
                        fire(w, CX, self.y + 16, a + k, QUICK);
                    }
                    k += 1;
                }
                if self.state == PHASE1 {
                    self.open = OPEN_TICKS;
                }
                true
            }
            _ => {
                // STARBURST: three rings of 16, 14 ticks apart, each turned on 11 degrees.
                if s % 14 == 1 {
                    let ring = s / 14;
                    let base = ring + (ring >> 1);
                    let mut k = 0;
                    while k < 16 {
                        fire(w, CX, self.y + 12, base + 3 * k, SLOW);
                        k += 1;
                    }
                }
                s == 29
            }
        };
        if done {
            self.pat = NONE;
            self.timer = 40;
            if self.state == PHASE2 {
                self.shut = 52;
            }
        }
    }

    /// The next pattern and its telegraph. Phase 1: LANE, VOLLEY, FLARE, VOLLEY; phase 2: PINCER,
    /// STARBURST, FLARE.
    fn start(&mut self, w: &World) {
        let t = self.turn;
        self.turn = self.turn.wrapping_add(1);
        self.step = 0;
        if self.state == PHASE2 {
            let k = t % 3;
            self.pat = [PINCER, STARBURST, FLARE][k as usize];
            self.timer = [45, 24, 30][k as usize];
            return;
        }
        self.pat = [LANE, VOLLEY, FLARE, VOLLEY][(t & 3) as usize];
        self.timer = [45, 20, 30, 20][(t & 3) as usize];
        if self.pat == VOLLEY {
            // The next live ring.
            let mut r = self.ring;
            let mut k = 0;
            while k < 3 {
                r = if r >= 2 { 0 } else { r + 1 };
                if self.hp[r as usize] != 0 {
                    break;
                }
                k += 1;
            }
            self.ring = r;
        } else if self.pat == LANE {
            // From the ship's outer side, across it toward the centre (the lane stays on the field).
            let px = (w.player.x >> 8) as i16;
            self.dir = if px < 152 { 1 } else { -1 };
            self.lane = px - 48 * self.dir as i16;
            if self.lane < 8 {
                self.lane = 8;
            } else if self.lane > 296 {
                self.lane = 296;
            }
        }
    }

    /// The first `DIRS` index (0 straight down) of a 70-degree cone at the ship from the lens (its 9
    /// directions on from here, below 96): a step (7.5 degrees) per 16 px across, at most 8 (the ship
    /// is always well below).
    fn cone_at_ship(&self, w: &World) -> u8 {
        let dx = (w.player.x >> 8) as i16 - CX;
        let k = (dx.abs() >> 4).min(8) as u8;
        wrap(if dx < 0 { 44 - k } else { 44 + k })
    }
}

/// Canvases of the rings (multicolor px × lines), outer to inner.
const RING_SIZE: [(u16, u16); 3] = [(RING_OUTER_W, RING_OUTER_H), (RING_MIDDLE_W, RING_MIDDLE_H), (RING_INNER_W, RING_INNER_H)];
/// PINCER LANES start this far in from each edge.
const PINCER_X0: i16 = 16;

/// Round speeds for `fire`: 1.25 px/tick (`DIRS`), 1.72 (LENS FLARE), 1.09 (STARBURST), 1.88 (lanes).
const BASE: u8 = 0;
const QUICK: u8 = 1;
const SLOW: u8 = 2;
const FAST: u8 = 3;

/// A round from (`x`, `y`) along `DIRS` direction `d` (below 96) at `speed`.
#[inline(never)]
fn fire(w: &mut World, x: i16, y: i16, d: u8, speed: u8) {
    let (vx, vy) = DIRS[wrap(d) as usize];
    let s = |v: i8| -> i8 {
        match speed {
            BASE => v,
            QUICK => v + (v >> 2) + (v >> 3),
            SLOW => v - (v >> 3),
            _ => v + (v >> 1),
        }
    };
    w.spawn_ebullet(B_ROUND, x, y, s(vx), s(vy));
}

/// A `DIRS` index mod 48.
fn wrap(d: u8) -> u8 {
    if d >= 48 { d - 48 } else { d }
}

/// The offset of ring `r`'s notch along a `DIRS` component (80 = the vector's length): 103, 72 or 43 px.
#[inline(never)]
fn notch(v: i8, r: usize) -> i16 {
    (v as i16 * [82, 57, 34][r]) >> 6
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::sim::{FIRE, LEFT, RIGHT, UP, DOWN};

    /// A pilot holding FIRE under the live part's aim (kept invulnerable) closes the observatory: the
    /// scroll holds, the rings fall, phase 2 turns `hot` yellow, the lens falls and the shutters close.
    #[test]
    fn the_astrolabe_can_be_closed() {
        let mut g = Game::new();
        g.clear = crate::clear::Clear::at(1, 1);
        g.campanile.state = crate::campanile::GONE;
        g.world.scroll = ((ARRIVE - 10) as u32) << 8;
        let (mut held, mut phase2, mut yellow, mut shots) = (0, 0, false, 0);
        let mut pats = [false; 5];
        let mut t = 0;
        while g.astro.state != GONE {
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            let (px, py) = ((g.world.player.x >> 8) as i16, (g.world.player.y >> 8) as i16);
            let aim = g.astro.aim;
            let mut input = if t % 2 == 0 { FIRE } else { 0 };
            if aim[1] != crate::boss::NO_AIM {
                if aim[0] < px - 2 {
                    input |= LEFT;
                } else if aim[0] > px + 2 {
                    input |= RIGHT;
                }
                if py > aim[1] + 40 {
                    input |= UP;
                } else if py < aim[1] + 30 {
                    input |= DOWN;
                }
            }
            let bullets = g.world.ebullets.iter().filter(|b| b.kind != 0).count();
            g.step(input);
            // It draws in the KEELBREAKER's and QUEEN's slots of the sprite list.
            let (mut out, mut n) = ([crate::render::Sprite::default(); crate::game::INSTANCES], 0);
            g.astro.sprites(&mut out, &mut n);
            assert!(n <= crate::game::KEEL_SPRITES + crate::demo::QUEEN_SPRITES, "{n} sprites");
            if g.astro.fighting() {
                held += (g.world.scroll_lines() == HOLD as u32) as u32;
                if g.astro.pat != NONE {
                    pats[g.astro.pat as usize] = true;
                }
            }
            if g.astro.state == PHASE2 {
                phase2 += 1;
                yellow |= g.clear.pal[3] == crate::boss::PHASE2_HOT;
            }
            shots += g.world.ebullets.iter().filter(|b| b.kind != 0).count().saturating_sub(bullets);
            t += 1;
            assert!(t < 30_000, "still aiming: {:?}", g.astro.hp);
        }
        assert!(held > 600, "the scroll held for the fight ({held} ticks)");
        assert!(phase2 > 0 && yellow, "phase 2 and its yellow");
        assert!(pats.iter().all(|&p| p), "every pattern ran: {pats:?}");
        assert!(shots > 100, "{shots}");
        assert!(g.world.score >= SCORE + 3 * RING_SCORE);
        println!("ASTROLABE closed in {t} ticks ({} of phase 2), score {}", phase2, g.world.score);
        // The stage clear follows (it began as it fell) into S3's stage in S3's colors.
        let mut k = 0;
        while g.clear.holds() {
            g.step(0);
            k += 1;
            assert!(k < 5_000, "clear phase {}", g.clear.phase);
        }
        assert_eq!(g.clear.stage, 2);
        assert_eq!(g.clear.pal, crate::clear::PALETTES[2]);
        assert_eq!(g.world.scroll_lines(), 0);
        assert!(!g.astro.shown());
        for _ in 0..10 {
            g.step(0);
        }
        assert_eq!(g.astro.state, ABSENT, "ready for the next pass");
    }

    /// A bolt is spent on the first live band it meets; the lens shrugs bolts off while armoured.
    #[test]
    fn bolts_hit_the_band_they_fly_into() {
        let mut w = World::new();
        let mut clear = crate::clear::Clear::at(1, 1);
        let mut a = Astrolabe::new();
        a.state = PHASE1;
        a.pose = 7;
        a.y = 64;
        a.hp = [RING_HP, RING_HP, RING_HP, LENS_HP];
        let shoot = |w: &mut World, dx: i16, dy: i16| {
            w.pairs[0] = crate::sim::world::Pair { alive: 1, x: CX + dx - 4, y: 64 + dy };
        };
        for (dx, dy, part) in [(0, 72, 1), (0, 43, 2), (0, 103, 0), (86, 56, 0), (-60, 40, 1)] {
            let before = a.hp;
            shoot(&mut w, dx, dy);
            a.take_hits(&mut w, &mut clear);
            assert_eq!(w.pairs[0].alive, 0, "({dx},{dy}) spent");
            assert_eq!(a.hp[part], before[part] - 1, "({dx},{dy}) hits part {part}");
        }
        shoot(&mut w, 0, 0);
        a.take_hits(&mut w, &mut clear);
        assert_eq!((w.pairs[0].alive, a.hp[3]), (0, LENS_HP), "armoured lens");
        a.open = 10;
        shoot(&mut w, 0, 0);
        a.take_hits(&mut w, &mut clear);
        assert_eq!(a.hp[3], LENS_HP - 1, "open lens");
        a.hp[1] = 0;
        shoot(&mut w, 0, 72);
        a.take_hits(&mut w, &mut clear);
        assert_eq!(w.pairs[0].alive, 1, "a bolt flies on through a dead ring");
    }

    /// SIGHT LANE starts on the ship's outer side and sweeps across it, on the field wherever the ship is.
    #[test]
    fn the_lane_stays_on_the_field() {
        let mut w = World::new();
        for px in [8i32, 60, 151, 152, 250, 296] {
            let mut a = Astrolabe::new();
            a.state = PHASE1;
            w.player.x = px << 8;
            a.start(&w);
            assert_eq!(a.pat, LANE);
            let end = a.lane + LANE_LEN as i16 * a.dir as i16;
            assert!((8..=296).contains(&a.lane) && (8..=296).contains(&end), "ship {px}: {}..{end}", a.lane);
            assert!((a.lane.min(end)..=a.lane.max(end)).contains(&(px as i16).clamp(8, 296)), "ship {px} crossed");
        }
    }

    /// A game over in phase 2 on a later stage leaves the new game its own colors and a fresh fight.
    #[test]
    fn a_game_over_ends_the_fight() {
        let mut g = Game::new();
        g.clear = crate::clear::Clear::at(1, 1);
        g.clear.stage = 2;
        g.clear.pal = crate::clear::PALETTES[2];
        g.campanile.state = crate::campanile::GONE;
        g.world.scroll = (HOLD as u32) << 8;
        g.astro.state = PHASE2;
        g.astro.pose = 7;
        g.astro.hp = [0, 0, 0, LENS_HP];
        g.astro.hot = g.clear.pal[3];
        g.clear.pal[3] = crate::boss::PHASE2_HOT;
        g.world.state = crate::sim::world::DYING;
        g.world.lives = 0;
        g.world.timer = 1;
        let mut t = 0;
        while g.world.state != crate::sim::world::PLAY {
            g.step(0);
            assert_ne!(g.astro.state, PHASE2, "the fight stopped at the game over");
            t += 1;
            assert!(t < 1_000);
        }
        for _ in 0..10 {
            g.step(0);
        }
        assert_eq!(g.clear.pal, crate::clear::PALETTES[1], "S2's own colors");
        assert_eq!((g.astro.state, g.astro.pose), (ABSENT, 0));
    }

    /// The canvases it places its frames with are the models'.
    #[test]
    fn canvases_match_the_models() {
        assert_eq!(crate::hand_s2::astrolabe_body(0).canvas, (ASTRO_W, ASTRO_H));
        assert_eq!(crate::hand_s2::astrolabe_lens(0).canvas, (LENS_W, LENS_H));
        for i in 0..3 {
            assert_eq!(crate::hand_s2::astrolabe_ring(i).canvas, RING_SIZE[i]);
        }
    }

    #[test]
    fn notches_turn_with_the_rings() {
        // Sample 0: a notch at 3 o'clock (DIRS 12, +x), the others 60 degrees on.
        assert_eq!(DIRS[12].1, 0);
        assert!(DIRS[12].0 > 0);
        for r in 0..3 {
            let want = [103.0, 72.0, 43.0][r];
            let got = notch(80, r) as f64;
            assert!((got - want).abs() <= 2.0, "ring {r}: {got}");
        }
    }
}
