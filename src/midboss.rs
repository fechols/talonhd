//! The KEELBREAKER (S1 midboss, `docs/story/05-bosses/s1-keelbreaker.md`), in TALON HD's sprite staging:
//! a destroyer 241 lines long slides up the docks' centre channel from below the screen; once its bow is
//! in the top third the scroll eases from 0.5 to 0.125 px/tick, so it reads as steaming ahead under the
//! TALON, and it creeps on up the screen (a line every 8 ticks) so its later turrets come into reach.
//! Its three turrets are separate sprites that track the TALON.
//!
//! - Phase 1 (72 -> 36): turrets fore, mid, aft, 12 HP each. BROADSIDE (each live turret in turn, fore to
//!   aft, a 3-round aimed burst) with WAKE MINES every fourth cycle (two slow mines from the stern that
//!   split into a 4-way cross where they land).
//! - Phase 2 (36 -> 0): the bridge core opens (36 HP). BRIDGE RING (12 shots with a gap aimed at the
//!   TALON) alternating with BROADSIDE from any turret still alive.
//! - Destroyed: 10,000 points, 3 SPARKs, a chain of bursts down the deck; the wreck falls behind.
//!   Survived for 40 s: the scroll returns to normal and the screen overtakes it (no score).
//!
//! The fight logic is a paged phase (`crate::paging::MIDBOSS`): every helper it needs is defined here, so
//! the phase calls only resident code. `Midboss::sprites` (render time) stays resident.

use crate::demo::{KEEL_FRAME, KEEL_H, KEEL_ROLLS, KEEL_W, TURRET_FRAME, TURRET_H, TURRET_W, BOMB_FRAME, BOMB_H, BOMB_W, isin};
use crate::render::Sprite;
use crate::sim::actors::{B_ROUND, BURST_TICKS, SCROLL_Q8};
use crate::sim::world::*;

/// `Midboss::state`.
pub const ABSENT: u8 = 0;
pub const ENTERING: u8 = 1;
pub const PHASE1: u8 = 2;
pub const PHASE2: u8 = 3;
pub const SINKING: u8 = 4;
pub const LEAVING: u8 = 5;
pub const GONE: u8 = 6;

/// Stage line where it leaves its berth.
pub const ARRIVE: u16 = 2400;
pub const TURRET_HP: u8 = 12;
pub const CORE_HP: u8 = 36;
pub const SCORE: u32 = 10_000;
pub const TURRET_SCORE: u32 = 500;
/// 40 s on screen (60 ticks a second), then the screen overtakes it.
pub const FIGHT_TICKS: u16 = 2400;
/// Scroll while it is fighting: 0.125 px/tick (Q8).
pub const SLOW_SCROLL_Q8: u32 = 32;

/// Hull centre line on screen: enters from below, holds with the bow in the top third.
const ENTER_Y: i16 = 192 + 128;
const HOLD_Y: i16 = 186;
/// It creeps up the screen during the fight, no further than this (hull centre line).
const CREEP_MIN_Y: i16 = 40;
/// Turret centres along the keel (lines from the hull centre, bow up) and the bridge core's.
const TURRET_DY: [i16; 3] = [-81, 38, 86];
const CORE_DY: i16 = -19;
/// The hull's centre column (multicolor px): the channel.
const CX: i16 = 80;
/// Turret deck height shifts them sideways as the hull rolls (multicolor px per roll sample).
const ROLL_DX: [i16; 5] = [-1, -1, 0, 1, 1];

const BROADSIDE: u8 = 0;
const MINES: u8 = 1;
const RING: u8 = 2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mine {
    /// Ticks to landing (0 = free); position and velocity Q6 like TALON's bullets.
    pub t: u8,
    pub x: i16,
    pub y: i16,
    pub vx: i8,
    pub vy: i8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Midboss {
    pub state: u8,
    /// Hull centre line on screen.
    pub y: i16,
    pub hp: [u8; 3],
    pub core: u8,
    /// Ticks in the fight (for the 40 s limit) and the attack rotation.
    pub clock: u16,
    pub turn: u8,
    pub pat: u8,
    pub step: u8,
    pub timer: u8,
    pub mines: [Mine; 2],
    /// Each turret's heading frame (16 yaws), aimed during the step so drawing only reads it.
    pub yaw: [u8; 3],
}

/// 48 directions, index 0 straight down the screen, rising toward +x, at 1.25 px/tick (Q6).
const DIRS: [(i8, i8); 48] = crate::boss::DIRS;

/// The direction index closest to (`dx`, `dy`): the offset scaled to ±63 so the dot products stay in 16
/// bits (32-bit multiplies cost the 6502 dearly; 48 directions need no more precision).
#[inline(never)]
fn aim(dx: i16, dy: i16) -> u8 {
    let m = dx.abs().max(dy.abs()).max(1);
    let (sx, sy) = (dx * 63 / m, dy * 63 / m);
    let (mut best, mut at) = (i16::MIN, 0u8);
    let mut k = 0;
    while k < 48 {
        let (vx, vy) = DIRS[k];
        let d = vx as i16 * sx + vy as i16 * sy;
        if d > best {
            best = d;
            at = k as u8;
        }
        k += 1;
    }
    at
}

#[inline(never)]
fn shoot(w: &mut World, x: i16, y: i16, k: u8) {
    let (vx, vy) = DIRS[(k % 48) as usize];
    w.spawn_ebullet(B_ROUND, x, y, vx, vy);
}

#[inline(never)]
fn burst(w: &mut World, x: i16, y: i16) {
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

/// Playfield point of a deck position `dy` lines from the hull centre.
fn deck(m: &Midboss, dy: i16) -> (i16, i16) {
    (CX * 2 - crate::game::OX, m.y + dy - crate::game::OY)
}

/// Roll sample of the hull at tick `t`: a slow swell.
fn roll(t: u32) -> u16 {
    ((isin((t / 4) as u8) + 127) as u16 * (KEEL_ROLLS - 1) + 127) / 254
}

/// Turret heading frame (16 yaws, 0 = barrels up the screen, counter-clockwise) for direction `k`.
fn turret_yaw(k: u8) -> u16 {
    ((k as u16 + 24 + 1) / 3) % 16
}

impl Midboss {
    fn active(&self) -> bool {
        matches!(self.state, ENTERING | PHASE1 | PHASE2 | SINKING | LEAVING)
    }

    /// Render time (resident): hull, live turrets, mines. Ground level: below everything that flies.
    pub fn sprites(&self, w: &World, out: &mut [Sprite]) -> usize {
        if !self.active() {
            return 0;
        }
        let r = roll(w.tick);
        let mut n = 0;
        out[n] = Sprite { frame: KEEL_FRAME + r, x: CX - (KEEL_W / 2) as i16, y: self.y - (KEEL_H / 2) as i16, z: -2 };
        n += 1;
        for k in 0..3 {
            if self.hp[k] != 0 || self.state == ENTERING {
                out[n] = Sprite {
                    frame: TURRET_FRAME + self.yaw[k] as u16,
                    x: CX + ROLL_DX[r as usize] - (TURRET_W / 2) as i16,
                    y: self.y + TURRET_DY[k] - (TURRET_H / 2) as i16,
                    z: -1,
                };
                n += 1;
            }
        }
        for mine in &self.mines {
            if mine.t != 0 {
                let (x, y) = (mine.x >> 6, mine.y >> 6);
                out[n] = Sprite {
                    frame: BOMB_FRAME,
                    x: ((x + crate::game::OX) >> 1) - (BOMB_W / 2) as i16,
                    y: y + crate::game::OY - (BOMB_H / 2) as i16,
                    z: 8,
                };
                n += 1;
            }
        }
        n
    }

    /// One tick, after the world's (paged).
    #[inline(never)]
    pub fn step(&mut self, w: &mut World) {
        self.step_state(w);
        // The turrets track the TALON; drawing reads the headings.
        let (px, py) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16);
        for k in 0..3 {
            let (tx, ty) = deck(self, TURRET_DY[k]);
            self.yaw[k] = turret_yaw(aim(px - tx, py - ty)) as u8;
        }
    }

    #[inline(never)]
    fn step_state(&mut self, w: &mut World) {
        match self.state {
            ABSENT => {
                *self = Midboss { state: ENTERING, y: ENTER_Y, hp: [TURRET_HP; 3], core: CORE_HP, pat: 0xFF, ..Midboss::default() };
            }
            ENTERING => {
                // It steams in faster than the screen scrolls, then the scroll eases.
                self.y -= 1;
                if self.y <= HOLD_Y {
                    self.state = PHASE1;
                    self.timer = 40;
                }
            }
            PHASE1 | PHASE2 => {
                w.scroll = w.scroll.wrapping_sub(SCROLL_Q8 - SLOW_SCROLL_Q8);
                self.clock += 1;
                // Longer than the screen is tall: it creeps ahead, bringing the mid and aft turrets and
                // the bridge up into the TALON's reach.
                if self.clock % 8 == 0 && self.y > CREEP_MIN_Y {
                    self.y -= 1;
                }
                self.move_mines(w);
                self.take_hits(w);
                if self.state == SINKING {
                    return;
                }
                if self.clock >= FIGHT_TICKS {
                    self.state = LEAVING;
                    return;
                }
                if self.timer != 0 {
                    self.timer -= 1;
                }
                if self.timer == 0 {
                    self.attack(w);
                }
            }
            SINKING => {
                self.move_mines(w);
                self.timer -= 1;
                if self.timer % 6 == 0 {
                    let t = self.timer as i16;
                    let (x, y) = deck(self, (t * 37 & 127) - 90);
                    burst(w, x + (t * 13 & 31) - 16, y);
                }
                if self.timer < 60 {
                    self.y += 1;
                }
                if self.timer == 0 {
                    self.state = LEAVING;
                }
            }
            LEAVING => {
                // The screen overtakes it.
                self.move_mines(w);
                self.y += 2;
                if self.y > 192 + 140 {
                    self.state = GONE;
                }
            }
            _ => {}
        }
    }

    #[inline(never)]
    fn move_mines(&mut self, w: &mut World) {
        for i in 0..2 {
            let m = &mut self.mines[i];
            if m.t == 0 {
                continue;
            }
            m.x = m.x.wrapping_add(m.vx as i16);
            m.y = m.y.wrapping_add(m.vy as i16);
            m.t -= 1;
            if m.t == 0 {
                // Landed: a 4-way cross.
                let (x, y) = (m.x >> 6, m.y >> 6);
                for k in [0u8, 12, 24, 36] {
                    shoot(w, x, y, k);
                }
            }
        }
    }

    #[inline(never)]
    fn take_hits(&mut self, w: &mut World) {
        for k in 0..3 {
            if self.hp[k] == 0 {
                continue;
            }
            let c = deck(self, TURRET_DY[k]);
            let n = hits(w, c, (14, 14));
            if n != 0 {
                self.hp[k] = self.hp[k].saturating_sub(n);
                if self.hp[k] == 0 {
                    burst(w, c.0, c.1);
                    w.add_score(TURRET_SCORE);
                }
            }
        }
        if self.state == PHASE1 && self.hp == [0; 3] {
            self.state = PHASE2;
            self.pat = 0xFF;
            self.turn = 0;
            self.timer = 40;
        }
        if self.state == PHASE2 {
            let c = deck(self, CORE_DY);
            let n = hits(w, c, (24, 22));
            if n != 0 {
                self.core = self.core.saturating_sub(n);
                if self.core == 0 {
                    self.state = SINKING;
                    self.timer = 90;
                    w.add_score(SCORE);
                    w.kills = w.kills.wrapping_add(1);
                    for (s, dx) in [-16i16, 0, 16].into_iter().enumerate() {
                        w.sparks[s] = Spark { alive: true, x0: c.0 + dx, y: (c.1 as i32) << 8, t: 16 };
                    }
                }
            }
        }
    }

    #[inline(never)]
    fn attack(&mut self, w: &mut World) {
        if self.pat == 0xFF {
            self.pat = if self.state == PHASE1 {
                if self.turn % 4 == 3 { MINES } else { BROADSIDE }
            } else if self.turn % 2 == 0 {
                RING
            } else {
                BROADSIDE
            };
            self.turn = self.turn.wrapping_add(1);
            self.step = 0;
            self.timer = match self.pat {
                BROADSIDE => 20,
                MINES => 30,
                _ => 36,
            };
            return;
        }
        let (px, py) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16);
        let done = match self.pat {
            BROADSIDE => {
                // Turret `step / 3` fires round `step % 3`; dead turrets are skipped.
                let k = (self.step / 3) as usize;
                if k < 3 && self.hp[k] != 0 {
                    let (x, y) = deck(self, TURRET_DY[k]);
                    shoot(w, x, y, aim(px - x, py - y));
                    self.timer = 6;
                } else {
                    self.timer = 1;
                }
                self.step += 1;
                if self.step % 3 == 0 {
                    self.timer = 10;
                }
                self.step >= 9
            }
            MINES => {
                let (x, y) = deck(self, 116);
                for (i, d) in [(0usize, 44u8), (1, 4)] {
                    let (vx, vy) = DIRS[d as usize];
                    self.mines[i] = Mine { t: 70, x: x * 64, y: y * 64, vx: (vx as i16 * 3 / 5) as i8, vy: (vy as i16 * 3 / 5) as i8 };
                }
                true
            }
            _ => {
                // A 12-shot ring with a gap where the TALON is: skip the two shots either side of the aim.
                let (x, y) = deck(self, CORE_DY);
                let a = aim(px - x, py - y);
                for j in 1..11u8 {
                    shoot(w, x, y, a + 2 + 4 * j);
                }
                true
            }
        };
        if done {
            self.pat = 0xFF;
            self.timer = 50;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::sim::{FIRE, LEFT, RIGHT};

    #[test]
    fn turret_yaw_points_its_barrels() {
        assert_eq!(turret_yaw(24), 0); // aiming up the screen: barrels up
        assert_eq!(turret_yaw(0), 8); // down
        assert_eq!(turret_yaw(12), 12); // right: a quarter turn clockwise = 12 counter-clockwise steps
        assert_eq!(turret_yaw(36), 4); // left
        // The 16-bit aim is within one direction (7.5 degrees) of the exact one at every angle: the rounded
        // direction vectors differ in length by ~1.5 %, which biases near-ties (finer than a turret's 22.5).
        for k in 0..48u8 {
            let a = k as f64 * std::f64::consts::PI / 24.0;
            for r in [40.0, 120.0, 300.0] {
                let got = aim((r * a.sin()).round() as i16, (r * a.cos()).round() as i16);
                let off = (got as i16 - k as i16).rem_euclid(48);
                assert!(off <= 1 || off >= 47, "k {k} r {r}: got {got}");
            }
        }
        for t in 0..1000 {
            assert!(roll(t) < KEEL_ROLLS);
        }
    }

    /// A pilot who lines up under each live turret, then the bridge, and holds FIRE (kept invulnerable)
    /// sinks it; the stage scroll slows during the fight and recovers afterwards.
    #[test]
    fn the_destroyer_can_be_sunk() {
        let mut g = Game::new();
        g.world.track_base = 1_000_000;
        g.world.scroll = (ARRIVE as u32 - 4) << 8;
        let mut slow = false;
        for _ in 0..12_000 {
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            let m = g.midboss;
            let goal = (0..3).find(|&k| m.hp[k] != 0).map(|k| deck(&m, TURRET_DY[k]).0).unwrap_or(deck(&m, CORE_DY).0);
            let px = (g.world.player.x >> 8) as i16;
            let mut input = FIRE;
            if goal < px - 2 {
                input |= LEFT;
            } else if goal > px + 2 {
                input |= RIGHT;
            }
            let before = g.world.scroll;
            g.step(input);
            if matches!(g.midboss.state, PHASE1 | PHASE2) {
                slow |= g.world.scroll.wrapping_sub(before) == SLOW_SCROLL_Q8;
            }
            if g.midboss.state == GONE {
                break;
            }
        }
        assert!(slow, "the scroll eased during the fight");
        assert_eq!(g.midboss.state, GONE);
        assert_eq!(g.midboss.core, 0, "sunk, not survived (hp {:?})", g.midboss.hp);
        assert!(g.world.score >= SCORE + 3 * TURRET_SCORE);
    }
}
