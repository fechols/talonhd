//! Collisions (reject-first, `docs/design.md` 13): every object gets a 16-line band mask first, and only
//! pairs whose masks intersect are tested on coordinates. AABB tests use centre distances against summed
//! half-extents.
use super::common::{band_mask, near, sway};
use super::actors::B_BOMB;
use super::player::{BOLT_DX, BOLT_H, BOLT_W};
use super::world::*;

/// WASP hurtbox (`03-enemies/common-wasp.md`): 16 x 14 centred on the anchor.
pub const WASP_HALF_W: i16 = 8;
pub const WASP_HALF_H: i16 = 7;
pub const WASP_SCORE: u32 = 100;
/// The TALON's hitbox at step 0 (`ship-talon.md`): 4 x 4 centred 2 px below the anchor.
pub const HITBOX_DY: i16 = 2;
pub static HITBOX_HALF: [i16; 5] = [2, 3, 4, 5, 6];
/// The SPARK pickup box is 16 x 16 around the hitbox centre; the SPARK itself is 8 x 8.
pub const PICKUP_HALF: i16 = 8;
pub const SPARK_HALF: i16 = 4;
pub const SPARK_SCORE: u32 = 100;
/// GANNET hurtbox 36 x 32 (`03-enemies/s1-gannet.md`), 500 points; its bombs are shootable for 50.
pub const GANNET_HALF_W: i16 = 18;
pub const GANNET_HALF_H: i16 = 16;
pub const GANNET_SCORE: u32 = 500;
pub const BOMB_SCORE: u32 = 50;
/// Bullet hitboxes (`docs/design.md` 14): round 2 x 2, the bomb 4 x 4.
pub const ROUND_HALF: i16 = 1;
pub const BOMB_HALF: i16 = 2;

impl World {
    /// The hitbox step (`docs/design.md` 5): main weapon not TWIN, FLANK, WINGS level; always 0 in M2.
    pub fn hitbox_step(&self) -> usize { if self.gauge.main != super::gauge::TWIN { 1 } else { 0 } }

    #[inline(never)]
    pub(crate) fn collide(&mut self) {
        let mut masks = [0u16; WASPS];
        let mut k = 0;
        while k < WASPS {
            if self.wasps[k].alive {
                masks[k] = band_mask((self.wasps[k].y >> 8) as i16 - WASP_HALF_H, 2 * WASP_HALF_H);
            }
            k += 1;
        }
        // The TALON first (bodies), then the shots, then the pickups.
        if self.state == PLAY {
            let px = (self.player.x >> 8) as i16;
            let py = (self.player.y >> 8) as i16 + HITBOX_DY;
            let half = HITBOX_HALF[self.hitbox_step()];
            let mask = band_mask(py - half, 2 * half);
            let mut k = 0;
            while k < WASPS {
                if masks[k] & mask != 0 {
                    let w = self.wasps[k];
                    if near((w.x >> 8) as i16, px, half + WASP_HALF_W) && near((w.y >> 8) as i16, py, half + WASP_HALF_H) {
                        if self.player.invuln == 0 {
                            self.wasps[k].alive = false;
                            masks[k] = 0;
                            self.spawn_burst((w.x >> 8) as i16, (w.y >> 8) as i16);
                            self.kill_player();
                            break;
                        }
                    }
                }
                k += 1;
            }
        }
        let mut i = 0;
        while i < PAIRS {
            if self.pairs[i].alive != 0 {
                let pair = self.pairs[i];
                let mask = band_mask(pair.y, BOLT_H);
                let by = pair.y + BOLT_H / 2;
                let mut k = 0;
                while k < WASPS {
                    if masks[k] & mask != 0 {
                        let wx = (self.wasps[k].x >> 8) as i16;
                        let wy = (self.wasps[k].y >> 8) as i16;
                        if near(by, wy, BOLT_H / 2 + WASP_HALF_H) {
                            let mut hit = 0u8;
                            if pair.alive & 1 != 0 && near(pair.x - BOLT_DX, wx, BOLT_W / 2 + WASP_HALF_W) { hit = 1; }
                            else if pair.alive & 2 != 0 && near(pair.x + BOLT_DX, wx, BOLT_W / 2 + WASP_HALF_W) { hit = 2; }
                            if hit != 0 {
                                self.pairs[i].alive &= !hit;
                                masks[k] = 0;
                                self.kill_wasp(k);
                                if self.pairs[i].alive == 0 { break; }
                            }
                        }
                    }
                    k += 1;
                }
            }
            i += 1;
        }
        self.collide_shooters();
        if self.state == PLAY {
            let px = (self.player.x >> 8) as i16;
            let py = (self.player.y >> 8) as i16 + HITBOX_DY;
            let mut s = 0;
            while s < SPARKS {
                let spark = self.sparks[s];
                if spark.alive && near(spark.x0 + sway(spark.t), px, PICKUP_HALF + SPARK_HALF)
                    && near((spark.y >> 8) as i16, py, PICKUP_HALF + SPARK_HALF)
                {
                    self.sparks[s].alive = false;
                    self.pickups = self.pickups.wrapping_add(1);
                    self.add_score(SPARK_SCORE);
                    self.advance_cursor();
                }
                s += 1;
            }
        }
    }

    /// GANNETs and enemy bullets: the TALON against both, then TWIN bolts against GANNETs and bombs (round
    /// bullets pass through player shots, `docs/design.md` 14).
    #[inline(never)]
    fn collide_shooters(&mut self) {
        if self.state == PLAY && self.player.invuln == 0 {
            let px = (self.player.x >> 8) as i16;
            let py = (self.player.y >> 8) as i16 + HITBOX_DY;
            let half = HITBOX_HALF[self.hitbox_step()];
            let mut k = 0;
            while k < GANNETS {
                let g = self.gannets[k];
                if g.state != 0 && near((g.x >> 8) as i16, px, half + GANNET_HALF_W) && near((g.y >> 8) as i16, py, half + GANNET_HALF_H) {
                    self.kill_gannet(k);
                    self.kill_player();
                    return;
                }
                k += 1;
            }
            let mut i = 0;
            while i < EBULLETS {
                let b = self.ebullets[i];
                if b.kind != 0 {
                    let bh = if b.kind == B_BOMB { BOMB_HALF } else { ROUND_HALF };
                    if near(b.x >> 6, px, half + bh) && near(b.y >> 6, py, half + bh) {
                        self.ebullets[i].kind = 0;
                        self.kill_player();
                        return;
                    }
                }
                i += 1;
            }
        }
        let mut i = 0;
        while i < PAIRS {
            let pair = self.pairs[i];
            if pair.alive != 0 {
                let by = pair.y + BOLT_H / 2;
                let mut bolt = 1u8;
                while bolt <= 2 {
                    if self.pairs[i].alive & bolt != 0 {
                        let bx = if bolt == 1 { pair.x - BOLT_DX } else { pair.x + BOLT_DX };
                        if self.bolt_hits_shooter(bx, by) { self.pairs[i].alive &= !bolt; }
                    }
                    bolt += 1;
                }
            }
            i += 1;
        }
    }

    /// One bolt at (`bx`, `by`) against the GANNETs, then the bombs; true when it hit (and is spent).
    fn bolt_hits_shooter(&mut self, bx: i16, by: i16) -> bool {
        let mut k = 0;
        while k < GANNETS {
            let g = self.gannets[k];
            if g.state != 0 && near((g.x >> 8) as i16, bx, BOLT_W / 2 + GANNET_HALF_W) && near((g.y >> 8) as i16, by, BOLT_H / 2 + GANNET_HALF_H) {
                self.gannets[k].hp -= 1;
                if self.gannets[k].hp == 0 {
                    self.kill_gannet(k);
                    self.kills = self.kills.wrapping_add(1);
                    self.add_score(GANNET_SCORE);
                }
                return true;
            }
            k += 1;
        }
        let mut i = 0;
        while i < EBULLETS {
            let b = self.ebullets[i];
            if b.kind == B_BOMB && near(b.x >> 6, bx, BOLT_W / 2 + BOMB_HALF) && near(b.y >> 6, by, BOLT_H / 2 + BOMB_HALF) {
                self.ebullets[i].kind = 0;
                self.add_score(BOMB_SCORE);
                return true;
            }
            i += 1;
        }
        false
    }

    #[inline(never)]
    fn kill_gannet(&mut self, k: usize) {
        let g = self.gannets[k];
        self.gannets[k].state = 0;
        self.spawn_burst((g.x >> 8) as i16, (g.y >> 8) as i16);
    }

    #[inline(never)]
    fn kill_wasp(&mut self, k: usize) {
        let w = self.wasps[k];
        self.wasps[k].alive = false;
        self.kills = self.kills.wrapping_add(1);
        self.add_score(WASP_SCORE);
        let (x, y) = ((w.x >> 8) as i16, (w.y >> 8) as i16);
        self.spawn_burst(x, y);
        if w.leader {
            let mut s = 0;
            while s < SPARKS {
                if !self.sparks[s].alive {
                    // Start the sway at its centre crossing so the SPARK appears where the leader died.
                    self.sparks[s] = Spark { alive: true, x0: x, y: w.y, t: 16 };
                    break;
                }
                s += 1;
            }
        }
    }

    /// The TALON is destroyed: one ship lost, everything stripped (`docs/design.md` 9), the resupply
    /// cursor set, and the death sequence started.
    #[inline(never)]
    fn kill_player(&mut self) {
        self.deaths = self.deaths.wrapping_add(1);
        self.spawn_burst((self.player.x >> 8) as i16, (self.player.y >> 8) as i16);
        self.lives = self.lives.saturating_sub(1);
        self.state = DYING;
        self.timer = super::player::DYING_TICKS;
        self.gauge.speed = 0;
        self.gauge.main = super::gauge::TWIN;
        self.gauge.cursor = 1;
        self.player.invuln = 0;
        self.player.invuln_kind = INV_NONE;
    }

    /// A placeholder burst at (x, y); when all are busy the oldest (least time left) is reused.
    #[inline(never)]
    fn spawn_burst(&mut self, x: i16, y: i16) {
        let mut best = 0;
        let mut i = 0;
        while i < BURSTS {
            if self.bursts[i].t < self.bursts[best].t { best = i; }
            i += 1;
        }
        self.bursts[best] = Burst { t: super::actors::BURST_TICKS, x, y };
    }
}
