//! The world half of a tick (C64 phase `p.world`): scroll, the spawn track, the WASP swooper archetype,
//! explosions and the drifting SPARKs.
use super::track::{Curve, CURVES, GANNET, KIND, LEADER, LOOP_LINES, MIRROR, TRACK, WASP, WASP_VX, WASP_VY};
use super::world::*;
use super::*;

/// S1 BREAKWATER's scroll: 0.5 px/tick (`04-levels/s1-breakwater.md` scroll_q8 = 128).
pub const SCROLL_Q8: u32 = 128;
/// Explosion lifetime: 3 frames of 4 ticks (placeholder burst).
pub const BURST_TICKS: u8 = 12;
/// SPARK drift (`docs/design.md` 6.1): 0.5 px/tick down the screen, swaying +-8 px over 64 ticks.
pub const SPARK_FALL: i32 = 128;
/// WASPs are removed once their centre is this far outside the playfield.
const MARGIN: i32 = 32 * PX;

/// GANNET states (`03-enemies/s1-gannet.md`): dive at 2 px/tick over the player's column, telegraph 16
/// ticks at the release line (levelling out to 0.5 px/tick, racks open), release 2 bombs, pull up over
/// 24 ticks, climb out the top at 1.5 px/tick.
pub const G_DIVE: u8 = 1;
pub const G_TELEGRAPH: u8 = 2;
pub const G_PULLUP: u8 = 3;
pub const G_CLIMB: u8 = 4;
pub const GANNET_HP: u8 = 6;
pub const GANNET_DIVE_Q8: i16 = 512;
/// Release at screen line 96 = playfield line 88 (the screen starts 8 lines above the playfield).
pub const GANNET_RELEASE_Y: i32 = 88;
pub const GANNET_TELEGRAPH: u8 = 16;
pub const GANNET_LEVEL_Q8: i16 = 128;
pub const GANNET_PULLUP: u8 = 24;
pub const GANNET_CLIMB_Q8: i16 = -384;
const GANNET_PULL_STEP: i16 = (GANNET_LEVEL_Q8 - GANNET_CLIMB_Q8) / GANNET_PULLUP as i16;
/// Bombs leave the racks under the wing elbows.
pub const BOMB_DX: i16 = 12;
pub const BOMB_DY: i16 = 8;

/// Enemy bullet kinds (`docs/design.md` 14); positions and velocities are Q6 (64 = 1 px).
pub const B_ROUND: u8 = 1;
/// The GANNET's bomb: 0.875 px/tick, 10 degrees either side of straight down, shootable (1 HP, 50 points).
pub const B_BOMB: u8 = 2;
pub const Q6: i16 = 64;
pub const BOMB_VX: i8 = 10;
pub const BOMB_VY: i8 = 55;
/// No point-blank shots: no bullet spawns within this many px of the TALON (both axes).
pub const POINT_BLANK: i16 = 32;
/// ... nor in the first 30 ticks of the respawn blink.
pub const LAUNCH_GRACE: u8 = 30;

impl World {
    /// The world half of tick `self.tick`: everything that moves without the player's input.
    #[inline(never)]
    pub fn step_world(&mut self) {
        self.tick = self.tick.wrapping_add(1);
        self.scroll = self.scroll.wrapping_add(SCROLL_Q8);
        if self.state != DYING && self.state != GAME_OVER {
            self.spawn_track();
        }
        self.release_waves();
        self.fly_wasps();
        self.fly_gannets();
        self.move_ebullets();
        let mut i = 0;
        while i < BURSTS {
            if self.bursts[i].t != 0 { self.bursts[i].t -= 1; }
            i += 1;
        }
        self.drift_sparks();
    }

    /// Start every string whose record has come due. A full wave table delays the record a tick.
    #[inline(never)]
    fn spawn_track(&mut self) {
        let lines = self.scroll_lines();
        if lines < self.track_base { return; }
        let line = lines - self.track_base;
        loop {
            let record = &TRACK[self.track_next as usize];
            if line < record.at as u32 || !matches!(record.kind & KIND, WASP | GANNET) { return; }
            let mut slot = WAVES;
            let mut w = 0;
            while w < WAVES {
                if self.waves[w].left == 0 { slot = w; break; }
                w += 1;
            }
            if slot == WAVES { return; }
            self.waves[slot] = Wave { left: record.count, timer: 0, record: self.track_next };
            self.track_next += 1;
            if self.track_next as usize == TRACK.len() {
                self.track_next = 0;
                self.track_base = self.track_base.wrapping_add(LOOP_LINES);
                return;
            }
        }
    }

    /// Release one WASP per string every `spacing` ticks; the last one is the leader if the record says so.
    #[inline(never)]
    fn release_waves(&mut self) {
        let mut w = 0;
        while w < WAVES {
            if self.waves[w].left != 0 {
                if self.waves[w].timer != 0 {
                    self.waves[w].timer -= 1;
                } else {
                    let record = &TRACK[self.waves[w].record as usize];
                    self.waves[w].left -= 1;
                    self.waves[w].timer = record.spacing - 1;
                    if record.kind & KIND == GANNET {
                        self.spawn_gannet(record.y);
                    } else {
                        let curve = &CURVES[(record.curve & !MIRROR) as usize];
                        let heading = if record.curve & MIRROR != 0 { curve.heading.wrapping_neg() } else { curve.heading };
                        let mut k = 0;
                        while k < WASPS {
                            if !self.wasps[k].alive {
                                self.wasps[k] = Wasp {
                                    alive: true,
                                    leader: self.waves[w].left == 0 && record.kind & LEADER != 0,
                                    curve: record.curve,
                                    seg: 0,
                                    seg_t: 0,
                                    angle: heading,
                                    x: (record.x2 as i32 * 2) * PX,
                                    y: record.y as i32 * PX,
                                };
                                break;
                            }
                            k += 1;
                        }
                    }
                }
            }
            w += 1;
        }
    }

    /// A GANNET over the player's column (the TALON's x at the spawn), entering at line `y`.
    #[inline(never)]
    fn spawn_gannet(&mut self, y: i8) {
        let mut k = 0;
        while k < GANNETS {
            if self.gannets[k].state == 0 {
                self.gannets[k] = Gannet { state: G_DIVE, hp: GANNET_HP, t: 0, x: self.player.x, y: y as i32 * PX, vy: GANNET_DIVE_Q8 };
                return;
            }
            k += 1;
        }
    }

    /// The diver archetype (`s1-gannet.md` behaviour 1-3).
    #[inline(never)]
    fn fly_gannets(&mut self) {
        let mut k = 0;
        while k < GANNETS {
            let mut g = self.gannets[k];
            if g.state != 0 {
                if g.state == G_DIVE {
                    if g.y >= GANNET_RELEASE_Y * PX {
                        g.state = G_TELEGRAPH;
                        g.t = GANNET_TELEGRAPH;
                        g.vy = GANNET_LEVEL_Q8;
                    }
                } else if g.state == G_TELEGRAPH {
                    g.t -= 1;
                    if g.t == 0 {
                        let (x, y) = ((g.x >> 8) as i16, (g.y >> 8) as i16 + BOMB_DY);
                        self.spawn_ebullet(B_BOMB, x - BOMB_DX, y, -BOMB_VX, BOMB_VY);
                        self.spawn_ebullet(B_BOMB, x + BOMB_DX, y, BOMB_VX, BOMB_VY);
                        g.state = G_PULLUP;
                        g.t = GANNET_PULLUP;
                    }
                } else if g.state == G_PULLUP {
                    g.vy -= GANNET_PULL_STEP;
                    g.t -= 1;
                    if g.t == 0 {
                        g.state = G_CLIMB;
                        g.vy = GANNET_CLIMB_Q8;
                    }
                }
                g.y += g.vy as i32;
                if g.y < -2 * MARGIN { g.state = 0; }
                self.gannets[k] = g;
            }
            k += 1;
        }
    }

    /// Spawn an enemy bullet at playfield (`x`, `y`) px with velocity (`vx`, `vy`) Q6, unless it would be
    /// point-blank, the TALON is relaunching or has just relaunched, or the pool is full (a spawn beyond the
    /// cap is dropped, never the oldest bullet).
    #[inline(never)]
    pub(crate) fn spawn_ebullet(&mut self, kind: u8, x: i16, y: i16, vx: i8, vy: i8) {
        if self.state == LAUNCH { return; }
        let p = &self.player;
        if p.invuln_kind == INV_RESPAWN && p.invuln > super::player::RESPAWN_INVULN - LAUNCH_GRACE { return; }
        let (px, py) = ((p.x >> 8) as i16, (p.y >> 8) as i16);
        if (x - px).abs() < POINT_BLANK && (y - py).abs() < POINT_BLANK { return; }
        let mut i = 0;
        while i < EBULLETS {
            if self.ebullets[i].kind == 0 {
                self.ebullets[i] = EBullet { kind, x: x * Q6, y: y * Q6, vx, vy };
                return;
            }
            i += 1;
        }
    }

    #[inline(never)]
    fn move_ebullets(&mut self) {
        let mut i = 0;
        while i < EBULLETS {
            let b = &mut self.ebullets[i];
            if b.kind != 0 {
                b.x = b.x.wrapping_add(b.vx as i16);
                b.y = b.y.wrapping_add(b.vy as i16);
                if b.x < -8 * Q6 || b.x > (FIELD_W + 8) * Q6 || b.y < -16 * Q6 || b.y > (FIELD_H + 8) * Q6 {
                    b.kind = 0;
                }
            }
            i += 1;
        }
    }

    /// The swooper archetype: follow the curve's turn program, move along the nearest of 16 headings.
    #[inline(never)]
    fn fly_wasps(&mut self) {
        let mut k = 0;
        while k < WASPS {
            let wasp = &mut self.wasps[k];
            if wasp.alive {
                let curve: &Curve = &CURVES[(wasp.curve & !MIRROR) as usize];
                if (wasp.seg as usize) < curve.segments.len() {
                    let (ticks, turn) = curve.segments[wasp.seg as usize];
                    if ticks != 0 {
                        let turn = if wasp.curve & MIRROR != 0 { turn.wrapping_neg() } else { turn };
                        wasp.angle = wasp.angle.wrapping_add(turn as u8);
                        wasp.seg_t += 1;
                        if wasp.seg_t == ticks {
                            wasp.seg += 1;
                            wasp.seg_t = 0;
                        }
                    }
                }
                let heading = (wasp.angle.wrapping_add(8) >> 4) as usize;
                wasp.x += WASP_VX[heading] as i32;
                wasp.y += WASP_VY[heading] as i32;
                if wasp.x < -MARGIN || wasp.x > FIELD_W as i32 * PX + MARGIN || wasp.y < -2 * MARGIN
                    || wasp.y > FIELD_H as i32 * PX + MARGIN
                {
                    wasp.alive = false;
                }
            }
            k += 1;
        }
    }

    #[inline(never)]
    fn drift_sparks(&mut self) {
        let mut i = 0;
        while i < SPARKS {
            let spark = &mut self.sparks[i];
            if spark.alive {
                spark.y += SPARK_FALL;
                spark.t = spark.t.wrapping_add(1);
                if spark.y > (FIELD_H as i32 + 12) * PX { spark.alive = false; }
            }
            i += 1;
        }
    }
}
