//! The GANTRY QUEEN fight (S1 boss), in TALON HD's flying staging (`docs/story/05-bosses/s1-gantry-queen.md`
//! with `docs/story/hd/models/s1.md`): she flies in over the airfield, the scroll stops, and she sways and
//! banks across the top of the screen while the TALON breaks her outer parts, then her core.
//!
//! - Phase 1 (180 -> 120): the targets are the parts the TALON can reach from below: both arm cannons and
//!   the two front legs, 15 HP each (the rear legs hang above the screen). TRIPLE FAN (two aimed 3-way fans
//!   from the arm muzzles) and MAST SWEEP (16 shots swept left to right).
//! - Phase 2 (120 -> 0): the rear legs snap off, the stage's `hot` turns yellow (bible `phase2_hot = 7`),
//!   the cab's crystal rim is the target. RING BURST (12-way), OFFSET RINGS (two rings half a gap apart),
//!   TRIPLE FAN from the cab.
//! - Defeat: a chain of explosions, 30,000 points, she sinks out past the TALON and the stage scrolls on.
//!
//! State lives here, beside TALON's vendored `World` (not in it), and is stepped once per tick after it,
//! so the fight replays exactly like the rest of the game. Integer math only; positions come from the
//! same `demo::queen` placement the renderer draws.

use crate::demo::{self, BOSS_FRAME, BOSS_SIZE, PANO_PERIOD, QUEEN_SPRITES, QUEEN_WRECK_FRAME, QUEEN_WRECK_X};
use crate::render::{Palette, Sprite};
use crate::sim::actors::{B_ROUND, SCROLL_Q8};
use crate::sim::world::*;

/// `Boss::state`.
pub const ABSENT: u8 = 0;
pub const ARRIVING: u8 = 1;
pub const PHASE1: u8 = 2;
pub const PHASE2: u8 = 3;
pub const DYING: u8 = 4;
pub const GONE: u8 = 5;
/// After the explosions: her wreck (damaged frames, `demo::QUEEN_WRECK_FRAME`) drifts down and out
/// smoothly while the stage scrolls on; GONE once it has left the bottom of the screen.
pub const WRECK: u8 = 6;

/// Stage lines: she comes into view at `ARRIVE` and the scroll stops at `HOVER` for the fight.
pub const ARRIVE: u16 = 4400;
pub const HOVER: u16 = 4560;
pub const PART_HP: u8 = 15;
pub const CORE_HP: u8 = 120;
pub const PART_SCORE: u32 = 1_000;
pub const KILL_SCORE: u32 = 30_000;
pub const DYING_TICKS: u8 = 120;
/// Phase 2 recolors `hot` (code 11) to yellow.
pub const PHASE2_HOT: u8 = 7;

/// Targets of phase 1, as `demo::queen` sprite indices: arm left, arm right, leg sw, leg se.
const TARGETS: [usize; 4] = [0, 1, 4, 5];
/// `Boss::broken` bits for the rear legs (sprites 2 and 3), dropped at phase 2.
const REAR_LEGS: u8 = 0x30;

/// Pattern ids and the phase rotations.
const TRIPLE: u8 = 0;
const SWEEP: u8 = 1;
const RING: u8 = 2;
const RINGS2: u8 = 3;
const P1_PATTERNS: [u8; 2] = [TRIPLE, SWEEP];
const P2_PATTERNS: [u8; 3] = [RING, RINGS2, TRIPLE];
const GAP_TICKS: u8 = 50;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Boss {
    pub state: u8,
    /// Phase 1 targets' HP (`TARGETS` order), and the core's.
    pub hp: [u8; 4],
    pub core: u8,
    /// Bit `TARGETS[k]` set once that part is broken; bits 4-5 the rear legs (phase 2).
    pub broken: u8,
    /// Pattern rotation index, the running pattern, its step and the ticks to its next step.
    pub turn: u8,
    pub pat: u8,
    pub step: u8,
    pub timer: u8,
    /// Lines below her hover line while she sinks away (dying).
    pub sink: i16,
    /// The playfield point of the weak part to shoot now (the attract pilot reads it, `crate::ace`); y
    /// `NO_AIM` when there is none.
    pub aim: [i16; 2],
    /// The wreck: its frame and where it froze (screen coordinates; it then drifts down by `sink`).
    pub wreck_frame: u16,
    pub wreck_x: i16,
    pub wreck_y: i16,
}

/// `Boss::aim` / `Midboss::aim` y when nothing can be hit.
pub const NO_AIM: i16 = -999;

/// 48 directions (7.5° apart), index 0 straight down the screen, rising toward +x (screen right):
/// (vx, vy) in Q6 px per tick at 1.25 px/tick (TALON's enemy bullets are Q6).
pub const DIRS: [(i8, i8); 48] = {
    let mut t = [(0i8, 0i8); 48];
    let mut k = 0;
    while k < 48 {
        let a = k as f64 * core::f64::consts::PI / 24.0;
        t[k] = (round(80.0 * demo::const_sin(a)), round(80.0 * demo::const_sin(a + core::f64::consts::FRAC_PI_2)));
        k += 1;
    }
    t
};

const fn round(v: f64) -> i8 {
    (if v < 0.0 { v - 0.5 } else { v + 0.5 }) as i8
}

/// tan of the half-step angles (3.75 + 7.5 k degrees, k = 0..5) in Q8: the octant's direction boundaries.
const TAN_STEP: [u8; 6] = [17, 51, 87, 126, 171, 225];

/// Steps (0..=6, 7.5 degrees each) of the angle from the `major` axis toward the `minor` one
/// (`minor <= major`, both < 128).
#[inline(never)]
fn octant_steps(minor: u16, major: u16) -> u8 {
    let mut k = 0;
    while k < 6 && (minor << 8) > major * TAN_STEP[k as usize] as u16 {
        k += 1;
    }
    k
}

/// The direction index closest to (`dx`, `dy`) (`DIRS`: 0 straight down, rising toward screen right),
/// in 16-bit math: scale into 7 bits, find the angle within the octant by its tangent thresholds, then
/// place the octant (a dot product against all 48 took 96 32-bit multiplies on the 6502).
#[inline(never)]
fn aim(dx: i16, dy: i16) -> u8 {
    let (mut ax, mut ay) = (dx.unsigned_abs(), dy.unsigned_abs());
    while ax > 127 || ay > 127 {
        ax >>= 1;
        ay >>= 1;
    }
    // Steps (0..=12) from the vertical toward the horizontal.
    let q = if ax <= ay { octant_steps(ax, ay) } else { 12 - octant_steps(ay, ax) };
    match (dx >= 0, dy >= 0) {
        (true, true) => q,
        (true, false) => 24 - q,
        (false, false) => 24 + q,
        (false, true) => if q == 0 { 0 } else { 48 - q },
    }
}

/// One round bullet from playfield (`x`, `y`) along direction `k`, `fast` = 1.5 instead of 1.25 px/tick.
#[inline(never)]
fn shoot(w: &mut World, x: i16, y: i16, k: u8, fast: bool) {
    let (vx, vy) = DIRS[(k % 48) as usize];
    let s = |v: i8| if fast { (v as i16 * 6 / 5) as i8 } else { v };
    w.spawn_ebullet(B_ROUND, x, y, s(vx), s(vy));
}

/// A burst at playfield (`x`, `y`): the ground units' resident helper (the same code; one copy).
use crate::ground::burst;

/// Where she is this tick: her sprites as the renderer places them (parts 0-5, body 6).
pub fn placement(b: &Boss, w: &World, out: &mut [Sprite]) {
    let lines = (w.scroll_lines() % PANO_PERIOD as u32) as u16;
    let lift = if b.state == ARRIVING { ((HOVER - lines.min(HOVER)) * 2) as i16 } else { -b.sink };
    demo::queen(w.tick as u16, lift, out);
}

/// Playfield centre of sprite `s` of `w` × `h` (multicolor px × lines), plus (`dx` hires, `dy` lines).
fn centre(s: &Sprite, w: u16, h: u16, dx: i16, dy: i16) -> (i16, i16) {
    (s.x * 2 + w as i16 + dx - crate::game::OX, s.y + (h / 2) as i16 + dy - crate::game::OY)
}

/// The playfield point of target `k` (hit box centre) and its half extents (hires px, lines).
fn target(k: usize, sp: &[Sprite]) -> ((i16, i16), (i16, i16)) {
    let i = TARGETS[k];
    if i < 2 {
        // An arm: the elbow and barrels, out toward its side and down.
        let (aw, ah) = demo::QUEEN_ARM_CANVAS;
        let sx = if i == 0 { -1 } else { 1 };
        (centre(&sp[i], aw, ah, sx * 56, 40), (26, 44))
    } else {
        // A front leg: the foot pad (82 hires out, 33 lines down from the part's origin).
        let (lw, lh) = demo::QUEEN_LEG_CANVAS;
        let sx = if i == 4 { -1 } else { 1 };
        (centre(&sp[i], lw, lh, sx * 82, 33), (26, 24))
    }
}

/// The cab's crystal rim (phase 2 target and muzzle), below the body's centre.
fn rim(sp: &[Sprite]) -> (i16, i16) {
    centre(&sp[6], BOSS_SIZE, BOSS_SIZE, 0, 104)
}

/// Spend bolts of the TALON's TWIN pairs that hit box (`c` ± `half`); returns the hits.
#[inline(never)]
fn hits(w: &mut World, c: (i16, i16), half: (i16, i16)) -> u8 {
    let mut n = 0;
    for i in 0..PAIRS {
        let p = w.pairs[i];
        let by = p.y + 5;
        if p.alive == 0 || (by - c.1).abs() >= half.1 + 5 {
            continue;
        }
        for bit in [1u8, 2] {
            let bx = if bit == 1 { p.x - 4 } else { p.x + 4 };
            if w.pairs[i].alive & bit != 0 && (bx - c.0).abs() < half.0 + 2 {
                w.pairs[i].alive &= !bit;
                n += 1;
            }
        }
    }
    n
}

impl Boss {
    /// Sprites she draws this tick (broken parts left out) into `out`; returns how many.
    pub fn sprites(&self, w: &World, out: &mut [Sprite]) -> usize {
        if self.state == WRECK {
            out[0] = Sprite { frame: self.wreck_frame, x: self.wreck_x, y: self.wreck_y + self.sink, z: 1 };
            return 1;
        }
        if !matches!(self.state, ARRIVING | PHASE1 | PHASE2 | DYING) {
            return 0;
        }
        let mut sp = [Sprite::default(); QUEEN_SPRITES];
        placement(self, w, &mut sp);
        let mut n = 0;
        for (i, s) in sp.iter().enumerate() {
            if self.broken & (1 << i) == 0 {
                out[n] = *s;
                n += 1;
            }
        }
        n
    }

    /// The stage palette with the boss's recolor applied.
    pub fn palette(&self, base: Palette) -> Palette {
        if matches!(self.state, PHASE2 | DYING) { Palette { color: PHASE2_HOT, ..base } } else { base }
    }

    /// One tick, after the world's.
    #[inline(never)]
    pub fn step(&mut self, w: &mut World) {
        let lines = (w.scroll_lines() % PANO_PERIOD as u32) as u16;
        if lines < ARRIVE {
            // A new pass of the stage (or a new game): she is waiting again.
            *self = Boss::default();
            w.boss_ceiling = false;
            return;
        }
        if self.state == ABSENT {
            self.state = ARRIVING;
        }
        if self.state == ARRIVING {
            if lines >= HOVER {
                self.state = PHASE1;
                self.hp = [PART_HP; 4];
                self.core = CORE_HP;
                self.timer = GAP_TICKS;
                self.pat = 0xFF;
            }
            return;
        }
        if self.state == GONE {
            w.boss_ceiling = false;
            return;
        }
        if self.state == WRECK {
            self.drift(w);
            return;
        }
        // The fight holds the scroll (undo the world's step) and keeps the TALON below her.
        w.scroll = w.scroll.wrapping_sub(SCROLL_Q8);
        w.boss_ceiling = true;
        let mut sp = [Sprite::default(); QUEEN_SPRITES];
        placement(self, w, &mut sp);
        if self.state == DYING {
            self.dying(w, &sp);
            return;
        }
        self.take_hits(w, &sp);
        if self.state == DYING {
            return;
        }
        if self.timer != 0 {
            self.timer -= 1;
        }
        if self.timer == 0 {
            self.attack(w, &sp);
        }
    }

    #[inline(never)]
    fn take_hits(&mut self, w: &mut World, sp: &[Sprite]) {
        self.aim[1] = NO_AIM;
        if self.state == PHASE1 {
            for k in 0..4 {
                if self.broken & (1 << TARGETS[k]) != 0 {
                    continue;
                }
                let (c, half) = target(k, sp);
                if self.aim[1] == NO_AIM {
                    self.aim = [c.0, c.1];
                }
                let n = hits(w, c, half);
                if n != 0 {
                    self.hp[k] = self.hp[k].saturating_sub(n);
                    if self.hp[k] == 0 {
                        self.broken |= 1 << TARGETS[k];
                        burst(w, c.0, c.1);
                        burst(w, c.0 - 12, c.1 + 10);
                        w.add_score(PART_SCORE);
                    }
                }
            }
            if self.broken & 0x33 == 0x33 {
                // Phase 2: the rear legs snap off too; the core is open.
                self.state = PHASE2;
                self.broken |= REAR_LEGS;
                self.turn = 0;
                self.pat = 0xFF;
                self.timer = GAP_TICKS;
            }
        } else {
            let r = rim(sp);
            self.aim = [r.0, r.1];
            let n = hits(w, r, (90, 22));
            if n != 0 {
                self.core = self.core.saturating_sub(n);
                if self.core == 0 {
                    self.state = DYING;
                    self.timer = DYING_TICKS;
                    w.add_score(KILL_SCORE);
                    w.kills = w.kills.wrapping_add(1);
                }
            }
        }
    }

    #[inline(never)]
    fn attack(&mut self, w: &mut World, sp: &[Sprite]) {
        if self.pat == 0xFF {
            // Start the next pattern of this phase's rotation with its telegraph.
            let list: &[u8] = if self.state == PHASE1 { &P1_PATTERNS } else { &P2_PATTERNS };
            self.pat = list[self.turn as usize % list.len()];
            self.turn = self.turn.wrapping_add(1);
            self.step = 0;
            self.timer = match self.pat {
                TRIPLE => 24,
                SWEEP => 36,
                RING => 20,
                _ => 24,
            };
            return;
        }
        let (px, py) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16);
        let r = rim(sp);
        let done = match self.pat {
            TRIPLE => {
                // From each live arm's muzzles in phase 1, from the cab rim in phase 2.
                for i in 0..2usize {
                    let from = if self.state == PHASE1 {
                        if self.broken & (1 << i) != 0 {
                            continue;
                        }
                        target(i, sp).0
                    } else if i == 0 {
                        r
                    } else {
                        continue;
                    };
                    let k = aim(px - from.0, py - from.1);
                    for d in [46u8, 0, 2] {
                        shoot(w, from.0, from.1 + 40, k + d, true);
                    }
                }
                self.timer = 20;
                self.step += 1;
                self.step == 2
            }
            SWEEP => {
                shoot(w, r.0, r.1, 40 + self.step, false);
                self.timer = 4;
                self.step += 1;
                self.step == 17
            }
            RING | RINGS2 => {
                let offset = self.step * 2;
                for k in 0..12u8 {
                    shoot(w, r.0, r.1, k * 4 + offset, false);
                }
                self.timer = 16;
                self.step += 1;
                self.pat == RING || self.step == 2
            }
            _ => true,
        };
        if done {
            self.pat = 0xFF;
            self.timer = GAP_TICKS;
        }
    }

    #[inline(never)]
    fn dying(&mut self, w: &mut World, sp: &[Sprite]) {
        self.timer -= 1;
        let t = self.timer as i16;
        if self.timer & 3 == 0 {
            // Scattered over her rim (shifts and xors, no multiplies or divides on the 6502).
            let r = rim(sp);
            burst(w, r.0 + ((t << 5 ^ t) & 127) - 64, r.1 - ((t << 4 ^ t << 1) & 63));
        }
        if self.timer == 0 {
            // The last blasts cover the swap to her wreck, frozen where she was (the bank sample nearest
            // a wreck frame); it then drifts out (`drift`).
            let body = sp[6];
            // The nearest of the wreck's banks (8, 24, 40 of 49).
            let b = body.frame - BOSS_FRAME;
            self.wreck_frame = QUEEN_WRECK_FRAME + if b < 16 { 0 } else if b < 33 { 1 } else { 2 };
            self.wreck_x = body.x;
            self.wreck_y = body.y;
            self.sink = 0;
            self.pat = 0;
            self.step = 0;
            let r = rim(sp);
            burst(w, r.0, r.1);
            burst(w, r.0 - 40, r.1 - 20);
            burst(w, r.0 + 40, r.1 - 20);
            self.state = WRECK;
            w.boss_ceiling = false;
        }
    }

    /// The wreck drifts down, easing in over 64 ticks to a line a tick (the stage scrolls on under it),
    /// gliding a pixel a tick to the screen's centre (she may have died far to one side), until it has
    /// left the bottom of the screen.
    #[inline(never)]
    fn drift(&mut self, w: &mut World) {
        w.boss_ceiling = false;
        if self.wreck_x < QUEEN_WRECK_X {
            self.wreck_x += 1;
        } else if self.wreck_x > QUEEN_WRECK_X {
            self.wreck_x -= 1;
        }
        if self.pat < 64 {
            self.pat += 1;
        }
        self.step += self.pat;
        while self.step >= 64 {
            self.step -= 64;
            self.sink += 1;
        }
        if self.wreck_y + self.sink >= crate::layout::HEIGHT {
            self.state = GONE;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::sim::FIRE;

    #[test]
    fn aim_finds_the_nearest_direction() {
        // Against the exact nearest of the 48 angles (the old dot product over the rounded DIRS tied
        // neighbours and took the lower index: it was right only ~71% of the time).
        let (mut same, mut total) = (0u32, 0u32);
        for dy in -260i16..=260 {
            for dx in -320i16..=320 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let a = aim(dx, dy);
                let exact = ((dx as f64).atan2(dy as f64) / (core::f64::consts::PI / 24.0)).round().rem_euclid(48.0) as i16;
                let off = (a as i16 - exact).rem_euclid(48);
                assert!(off == 0 || off == 1 || off == 47, "aim({dx}, {dy}) = {a}, the nearest is {exact}");
                same += (off == 0) as u32;
                total += 1;
            }
        }
        assert!(same as f64 / total as f64 > 0.97, "{same} of {total} exact");
    }

    #[test]
    fn directions_are_unit_and_ordered() {
        assert_eq!(DIRS[0], (0, 80));
        assert_eq!(DIRS[12], (80, 0));
        assert_eq!(DIRS[24], (0, -80));
        assert_eq!(DIRS[36], (-80, 0));
        assert_eq!(aim(0, 100), 0);
        assert_eq!(aim(100, 100), 6);
        assert_eq!(aim(-100, 0), 36);
    }

    /// A pilot who parks under each live target in turn and holds FIRE (bullets are ignored by making the
    /// TALON invulnerable each tick) breaks her parts, opens phase 2, kills her, and the stage scrolls on.
    #[test]
    fn her_wreck_drifts_off_smoothly() {
        // Beat her (as the_fight_runs_to_the_end), then follow the wreck.
        let mut g = Game::new();
        g.world.track_base = 1_000_000;
        g.world.scroll = (ARRIVE as u32 - 10) << 8;
        let mut wreck_ys = vec![];
        let mut out = [Sprite::default(); QUEEN_SPRITES];
        for _ in 0..30_000 {
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            let mut sp = [Sprite::default(); QUEEN_SPRITES];
            placement(&g.boss, &g.world, &mut sp);
            let goal = if g.boss.state == PHASE1 {
                (0..4).find(|&k| g.boss.broken & (1 << TARGETS[k]) == 0).map(|k| target(k, &sp).0.0)
            } else {
                Some(rim(&sp).0)
            };
            let px = (g.world.player.x >> 8) as i16;
            let mut input = FIRE;
            if let Some(x) = goal {
                input |= if x < px - 2 { crate::sim::LEFT } else if x > px + 2 { crate::sim::RIGHT } else { 0 };
            }
            g.step(input);
            if g.boss.state == WRECK {
                let n = g.boss.sprites(&g.world, &mut out);
                assert_eq!(n, 1, "the wreck is one sprite");
                assert!((QUEEN_WRECK_FRAME..QUEEN_WRECK_FRAME + crate::demo::QUEEN_WRECKS).contains(&out[0].frame));
                wreck_ys.push(out[0].y);
                assert!(!g.world.boss_ceiling, "the wreck lets the TALON fly");
            }
            if g.boss.state == GONE {
                break;
            }
        }
        assert_eq!(g.boss.state, GONE);
        assert!(wreck_ys.len() > 100, "the wreck is seen for {} ticks", wreck_ys.len());
        // Smooth: never up, never more than a line a tick; gone only once fully below the screen.
        assert_eq!(out[0].x, QUEEN_WRECK_X, "the wreck glides to the centre");
        assert!(wreck_ys.windows(2).all(|w| (0..=1).contains(&(w[1] - w[0]))), "the wreck jumps");
        assert!(*wreck_ys.last().unwrap() >= crate::layout::HEIGHT - 1, "she left at y {}", wreck_ys.last().unwrap());
    }

    #[test]
    fn the_fight_runs_to_the_end() {
        let mut g = Game::new();
        g.world.track_base = 1_000_000; // no waves
        g.world.scroll = (ARRIVE as u32 - 10) << 8;
        let mut seen_phase2 = false;
        for _ in 0..30_000 {
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            // Steer under the first live target (or the rim).
            let mut sp = [Sprite::default(); QUEEN_SPRITES];
            placement(&g.boss, &g.world, &mut sp);
            let goal = if g.boss.state == PHASE1 {
                (0..4).find(|&k| g.boss.broken & (1 << TARGETS[k]) == 0).map(|k| target(k, &sp).0.0)
            } else {
                Some(rim(&sp).0)
            };
            let px = (g.world.player.x >> 8) as i16;
            let mut input = FIRE;
            if let Some(x) = goal {
                if x < px - 2 {
                    input |= crate::sim::LEFT;
                } else if x > px + 2 {
                    input |= crate::sim::RIGHT;
                }
            }
            g.step(input);
            seen_phase2 |= g.boss.state == PHASE2;
            if g.boss.state == GONE {
                break;
            }
        }
        assert!(seen_phase2, "phase 2 reached");
        assert_eq!(g.boss.state, GONE, "she was destroyed (state {}, hp {:?}, core {})", g.boss.state, g.boss.hp, g.boss.core);
        assert!(g.world.score >= KILL_SCORE + 4 * PART_SCORE);
        let before = g.world.scroll_lines();
        for _ in 0..20 {
            g.step(0);
        }
        assert!(g.world.scroll_lines() > before, "the stage scrolls on");
        assert!(!g.world.boss_ceiling);
    }
}
