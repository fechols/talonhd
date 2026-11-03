//! The CAMPANILE (S2 midboss, `docs/story/05-bosses/s2-campanile.md`, HD model in `docs/story/hd/models/
//! s2.md`): the hill village's bell tower is level geometry (`s2::TOWER`); its spinning crystal crown and
//! bell hub are sprites. It never moves: it turns in steps like a clock hand and rings.
//!
//! - It comes into view at `ARRIVE`; the scroll stops at `HOLD` with the tower in the upper half of the
//!   screen and resumes when it falls.
//! - Phase 1 (240 -> 120, the bible's 80 -> 40 tripled for TWIN; the crown takes the hits): CHIME RING (the crown spins faster for 30 ticks, then an
//!   8-shard ring, one from each point, 0.875 px/tick, and the chime: `hot` flashes to `light` for 2
//!   frames, a palette step, never a blink) alternating with SPOKE (one point glows for 20 ticks, then 5
//!   fast shots along it, 5 ticks apart; the spoke moves on a point).
//! - Phase 2 (120 -> 0): the bell hub opens (it takes hits too: 72 HP), the crown turns the other way, and
//!   CHIME RING becomes DOUBLE PEAL (two 16-shard rings 53 ticks apart, the second turned 7.5 degrees,
//!   0.75 px/tick).
//! - Silenced: 12,000 points, 3 SPARKs, a chain of bursts as the crown shatters.
//!
//! The step and the render-time `sprites` are in the S2 phase (`paging::S2`: the step paged by
//! `stage2::run`, the sprites by `game::scene` while the tower is in view). The chime flips the stage
//! palette's `hot` itself (`Clear::pal`), so nothing of it is resident. 16-bit math only.

use crate::demo::*;
use crate::ground::{burst, hits};
use crate::render::Sprite;
use crate::sim::actors::B_ROUND;
use crate::sim::world::*;

/// `Campanile::state` (the KEELBREAKER's numbers, so the attract pilot reads either midboss alike).
pub const ABSENT: u8 = 0;
pub const ENTERING: u8 = 1;
pub const PHASE1: u8 = 2;
pub const PHASE2: u8 = 3;
pub const DYING: u8 = 4;
pub const GONE: u8 = 6;

/// Stage lines: the crown comes into view, then the scroll holds with the tower in the upper half.
pub const ARRIVE: u16 = 2250;
pub const HOLD: u16 = 2368;
/// The tower (`s2::TOWER`): world x and line.
const TOWER_X: i16 = 160;
const TOWER_U: i16 = 2500;
/// The bible's 56 + 24 (phase 2 at 40 of 80) in TALON's damage, tripled for HD's TWIN, the only weapon
/// (it fell in 4 s to the attract pilot; the bible's par is 35 s).
pub const CROWN_HP: u8 = 168;
pub const HUB_HP: u8 = 72;
/// Phase 2 when the total (crown + hub) is down to this.
const PHASE2_HP: u8 = 120;
pub const SCORE: u32 = 12_000;

const RING: u8 = 0;
const SPOKE: u8 = 1;
const PEAL: u8 = 2;
const NONE: u8 = 0xFF;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Campanile {
    pub state: u8,
    /// The tower's playfield line.
    pub y: i16,
    pub crown: u8,
    pub hub: u8,
    /// Crown spin sample (8 over 45 degrees, wrapping) and ticks toward its next step.
    pub spin: u8,
    pub tick: u8,
    pub pat: u8,
    pub turn: u8,
    pub step: u8,
    pub timer: u8,
    /// SPOKE's point (0..8) and, while it glows, the tip's offset from the centre (`glow` != 0).
    pub point: u8,
    pub glow: u8,
    pub gx: i8,
    pub gy: i8,
    /// The hub's pose (0 shut .. 3 open), the chime's frames left and the `hot` it lit over.
    pub pose: u8,
    pub flash: u8,
    pub hot: u8,
    /// The playfield point to shoot now (`crate::ace`); y `boss::NO_AIM` when there is none.
    pub aim: [i16; 2],
}

impl Default for Campanile {
    fn default() -> Self {
        Campanile::new()
    }
}

/// 48 directions at 1.25 px/tick (Q6), 0 straight down, rising toward screen right.
const DIRS: [(i8, i8); 48] = crate::boss::DIRS;

impl Campanile {
    #[inline(always)]
    pub const fn new() -> Campanile {
        Campanile { state: ABSENT, y: 0, crown: 0, hub: 0, spin: 0, tick: 0, pat: NONE, turn: 0, step: 0, timer: 0, point: 0, glow: 0, gx: 0, gy: 0, pose: 0, flash: 0, hot: 0, aim: [0, crate::boss::NO_AIM] }
    }

    /// Whether it is fighting (the attract pilot's midboss).
    pub fn fighting(&self) -> bool {
        self.state == PHASE1 || self.state == PHASE2
    }

    /// Whether it is drawn (resident: `game::scene` pages the S2 phase for `sprites` only then).
    #[inline(always)]
    pub fn shown(&self) -> bool {
        self.state != ABSENT && self.state != GONE
    }

    /// Render time (paged: `paging::S2`): the crown, the hub over it, SPOKE's glowing point; ground level.
    #[inline(never)]
    pub fn sprites(&self, out: &mut [Sprite; crate::game::INSTANCES], n: &mut usize) {
        if self.state == DYING && self.timer < 40 {
            return;
        }
        let x = TOWER_X - crate::game::OX;
        crate::game::put(out, n, S2_CROWN_FRAME + (self.spin & 7) as u16, x, self.y, CROWN_W, CROWN_H, -2);
        crate::game::put(out, n, S2_HUB_FRAME + self.pose as u16, x, self.y, HUB_W, HUB_H, -1);
        if self.glow != 0 {
            crate::game::put(out, n, S2_SHELL_FRAME + 3, x + self.gx as i16, self.y + self.gy as i16, SHELL_W, SHELL_H, -1);
        }
    }

    /// One tick at stage line `at`, after the world's (paged: `paging::S2`).
    #[inline(never)]
    pub fn step(&mut self, w: &mut World, pal: &mut [u8; 4], at: u16) {
        if self.flash == 3 {
            self.hot = pal[3];
        }
        if self.flash != 0 {
            // The chime: `hot` lit to `light` (the stage palette's own), then back.
            self.flash -= 1;
            pal[3] = if self.flash != 0 { pal[2] } else { self.hot };
        }
        if at < ARRIVE {
            if self.flash != 0 {
                // Reset mid-chime: `hot` back first.
                pal[3] = self.hot;
            }
            *self = Campanile::new();
            return;
        }
        match self.state {
            ABSENT => {
                *self = Campanile::new();
                if at > HOLD {
                    // Joined past the tower (a test, a start line): it is behind us.
                    self.state = GONE;
                    return;
                }
                self.state = ENTERING;
                self.crown = CROWN_HP;
                self.hub = HUB_HP;
            }
            ENTERING => {
                if at >= HOLD {
                    self.state = PHASE1;
                    self.timer = 40;
                }
            }
            PHASE1 | PHASE2 => {
                // The scroll stops for the fight (a constant store: no 32-bit math).
                w.scroll = (HOLD as u32) << 8;
                if self.state == PHASE2 && self.pose < 3 && w.tick as u8 & 7 == 0 {
                    self.pose += 1;
                }
                self.take_hits(w);
                if self.state == PHASE1 || self.state == PHASE2 {
                    self.attack(w);
                }
            }
            DYING => {
                if self.timer & 7 == 0 {
                    let t = self.timer as i16;
                    burst(w, TOWER_X - crate::game::OX + (t * 37 & 63) - 32, self.y + (t * 23 & 63) - 32);
                }
                self.timer -= 1;
                if self.timer == 0 {
                    self.state = GONE;
                }
            }
            _ => {}
        }
        self.y = at as i16 + 200 - TOWER_U - crate::game::OY;
        // The crown steps like a clock hand: every 6 ticks, every 2 while it winds up for a ring; the
        // other way round in phase 2.
        self.tick += 1;
        let every = if self.pat == RING && self.timer != 0 { 2 } else { 6 };
        if self.tick >= every && self.state != DYING {
            self.tick = 0;
            self.spin = if self.state == PHASE2 { self.spin.wrapping_sub(1) } else { self.spin.wrapping_add(1) } & 7;
        }
    }

    #[inline(never)]
    fn take_hits(&mut self, w: &mut World) {
        let c = (TOWER_X - crate::game::OX, self.y);
        self.aim = [c.0, c.1];
        if self.state == PHASE2 && self.hub != 0 {
            self.hub = self.hub.saturating_sub(hits(w, c, 10));
        }
        if self.crown != 0 {
            self.crown = self.crown.saturating_sub(hits(w, c, 36));
        }
        if self.state == PHASE1 && self.crown + self.hub <= PHASE2_HP {
            self.state = PHASE2;
            self.pat = NONE;
            self.turn = 0;
            self.glow = 0;
            self.timer = 40;
        }
        if self.crown == 0 && self.hub == 0 {
            self.state = DYING;
            self.timer = 96;
            self.glow = 0;
            self.aim[1] = crate::boss::NO_AIM;
            w.add_score(SCORE);
            w.kills = w.kills.wrapping_add(1);
            for s in 0..3 {
                w.sparks[s] = Spark { alive: true, x0: c.0 - 16 + 16 * s as i16, y: (c.1 as i32) << 8, t: 16 };
            }
        }
    }

    /// Ring: the next step lights `hot` for 2 frames.
    fn chime(&mut self) {
        if self.flash == 0 {
            self.flash = 3;
        }
    }

    /// The direction of crown point `k` at this spin (`DIRS` index): 45 degrees apart, turned with the
    /// crown (a spin sample is 5.625 degrees, three quarters of a direction).
    fn point_dir(&self, k: u8) -> u8 {
        let d = 24 + ((self.spin & 7) * 3 + 2) / 4 + 6 * (k & 7);
        if d >= 48 { d - 48 } else { d }
    }

    #[inline(never)]
    fn attack(&mut self, w: &mut World) {
        if self.timer != 0 {
            self.timer -= 1;
            return;
        }
        if self.pat == NONE {
            // CHIME RING (phase 2: DOUBLE PEAL), then SPOKE, in turn; each starts with its telegraph.
            let ring = self.turn & 1 == 0;
            self.pat = if !ring { SPOKE } else if self.state == PHASE2 { PEAL } else { RING };
            self.turn = self.turn.wrapping_add(1);
            self.step = 0;
            self.timer = match self.pat {
                RING => 30,
                PEAL => 36,
                _ => 20,
            };
            if self.pat == SPOKE {
                let (vx, vy) = DIRS[self.point_dir(self.point) as usize];
                self.glow = 1;
                self.gx = tip(vx);
                self.gy = tip(vy);
            }
            return;
        }
        let c = (TOWER_X - crate::game::OX, self.y);
        let done = match self.pat {
            RING => {
                for k in 0..8 {
                    fire(w, c, self.point_dir(k), 0);
                }
                self.chime();
                true
            }
            SPOKE => {
                fire(w, c, self.point_dir(self.point), 1);
                self.step += 1;
                self.timer = 4;
                if self.step == 5 {
                    self.glow = 0;
                    self.point = (self.point + 1) & 7;
                }
                self.step == 5
            }
            _ => {
                // Two rings of 16, the second turned a direction (7.5 degrees) and held until the first
                // is 40 px out.
                let base = self.point_dir(0) + self.step;
                let mut k = 0;
                while k < 16 {
                    let d = base + 3 * k;
                    fire(w, c, if d >= 48 { d - 48 } else { d }, 2);
                    k += 1;
                }
                self.chime();
                self.step += 1;
                self.timer = 53;
                self.step == 2
            }
        };
        if done {
            self.pat = NONE;
            self.timer = 50;
        }
    }
}

/// The offset (px) of a crown point's tip, 44 px out, along a `DIRS` component (80 = the whole length).
fn tip(v: i8) -> i8 {
    (v >> 1) + (v >> 4)
}

/// A shard from crown point direction `k`'s tip: `speed` 0 the ring's 0.875 px/tick, 1 the spoke's 1.5,
/// 2 the peal's 0.75 (`DIRS` is 1.25; scaled by shifts).
#[inline(never)]
fn fire(w: &mut World, c: (i16, i16), k: u8, speed: u8) {
    let (vx, vy) = DIRS[k as usize];
    let s = |v: i8| -> i8 {
        match speed {
            0 => v - (v >> 2) - (v >> 4),
            1 => v + (v >> 3) + (v >> 4),
            _ => (v >> 1) + (v >> 3),
        }
    };
    w.spawn_ebullet(B_ROUND, c.0 + tip(vx) as i16, c.1 + tip(vy) as i16, s(vx), s(vy));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::sim::{FIRE, LEFT, RIGHT};

    /// A reset caught mid-chime (a game over sends the scroll back) leaves `hot` its own color.
    #[test]
    fn a_reset_mid_chime_restores_hot() {
        let mut w = World::new();
        let mut pal = crate::clear::PALETTES[1];
        let hot = pal[3];
        let mut c = Campanile::new();
        c.state = PHASE1;
        c.chime();
        c.step(&mut w, &mut pal, HOLD);
        assert_eq!(pal[3], pal[2], "the chime lit it");
        c.step(&mut w, &mut pal, 0);
        assert_eq!(pal[3], hot);
    }

    /// A pilot under the tower holding FIRE (kept invulnerable) silences it: the scroll holds through the
    /// fight, phase 2 opens the hub, and the stage scrolls on after.
    #[test]
    fn the_campanile_can_be_silenced() {
        let mut g = Game::new();
        g.clear = crate::clear::Clear::at(1, 1);
        g.world.scroll = ((ARRIVE - 10) as u32) << 8;
        let (mut held, mut phase2, mut rings, mut flashes) = (0, false, 0, 0);
        let mut t = 0;
        while g.campanile.state != GONE {
            g.world.player.invuln = 2;
            g.world.player.invuln_kind = INV_ARM;
            let px = (g.world.player.x >> 8) as i16;
            let goal = TOWER_X - crate::game::OX;
            let mut input = if t % 2 == 0 { FIRE } else { 0 };
            if goal < px - 2 {
                input |= LEFT;
            } else if goal > px + 2 {
                input |= RIGHT;
            }
            let bullets = g.world.ebullets.iter().filter(|b| b.kind != 0).count();
            g.step(input);
            if g.campanile.fighting() {
                held += (g.world.scroll_lines() == HOLD as u32) as u32;
            }
            phase2 |= g.campanile.state == PHASE2 && g.campanile.pose == 3;
            rings += (g.world.ebullets.iter().filter(|b| b.kind != 0).count() >= bullets + 8) as u32;
            flashes += (g.campanile.flash == 2 && g.clear.pal[3] == g.clear.pal[2]) as u32;
            t += 1;
            assert!(t < 20_000, "still ringing: crown {} hub {}", g.campanile.crown, g.campanile.hub);
        }
        assert!(held > 120, "the scroll held for the fight ({held} ticks)");
        assert!(phase2, "the hub opened");
        assert!(rings >= 2 && flashes >= 2, "rings {rings}, chimes {flashes}");
        assert!(g.world.score >= SCORE);
        let before = g.world.scroll_lines();
        for _ in 0..20 {
            g.step(0);
        }
        assert!(g.world.scroll_lines() > before, "the stage scrolls on");
        println!("CAMPANILE silenced in {t} ticks, score {}", g.world.score);
    }

    #[test]
    fn crown_points_turn_with_the_crown() {
        let mut c = Campanile::new();
        assert_eq!(c.point_dir(0), 24, "point 0 up the screen at rest");
        assert_eq!(c.point_dir(2), 36);
        c.spin = 4;
        assert_eq!(c.point_dir(0), 27, "half a step on: 22.5 degrees");
        for s in 0..8 {
            c.spin = s;
            for k in 0..8 {
                assert!(c.point_dir(k) < 48);
            }
        }
    }
}
