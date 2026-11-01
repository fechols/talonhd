//! TALON's game on TALON HD's screen: the vendored simulation (`crate::sim`) stepped one tick per frame,
//! and a pure mapping from its state to software sprites, the HUD and the scroll.
//!
//! Coordinates: the sim's playfield is 304 × 176 logical (hires) pixels; it sits centred in the 320 × 192
//! visible screen, 8 px in from the left and 8 lines down. Sprite x is in multicolor pixels (hires / 2).
//! Everything here is a function of `Game` alone, which is itself a function of the input log, so the host
//! replays the C64's attract mode to any tick and renders the identical frame.

use crate::demo::{self, *};
use crate::hud;
use crate::render::Sprite;
use crate::sim::actors::{B_BOMB, G_CLIMB, G_DIVE, G_PULLUP, G_TELEGRAPH};
use crate::sim::common::sway;
use crate::sim::world::*;
use crate::sim::{LEFT, RIGHT, World};

/// Where the playfield sits on screen (hires px, lines).
pub const OX: i16 = 8;
pub const OY: i16 = 8;

/// Every sprite a frame can hold: the boss, the pools of the sim and the TALON.
pub const INSTANCES: usize = QUEEN_SPRITES + GANNETS + WASPS + SPARKS + BURSTS + EBULLETS + PAIRS + 1;

/// Draw order (low first): boss parts and body (0, 1), then these.
const Z_GANNET: i16 = 2;
const Z_WASP: i16 = 3;
const Z_SPARK: i16 = 4;
const Z_BURST: i16 = 5;
const Z_PAIR: i16 = 6;
const Z_PLAYER: i16 = 7;
const Z_BULLET: i16 = 8;

/// Presentation state that is not gameplay (TALON's digest does not cover it) but is stepped per tick,
/// so it replays exactly: the ship's bank and the HUD's damage flash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pilot {
    /// -4 (full left) ..= 4 (full right), easing one step every 2 ticks toward the stick.
    pub bank: i8,
    pub deaths: u16,
    /// Ticks since the TALON was last destroyed (saturating; 255 = long ago).
    pub since_hit: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Game {
    pub world: World,
    pub pilot: Pilot,
    /// Ticks stepped since power-on. `World::tick` restarts with every new game; this does not, so it
    /// names a frame of the attract mode across game overs (the C64's `SHOWN_T`, the host's replay).
    pub ticks: u32,
}

impl Game {
    pub const fn new() -> Game {
        Game { world: World::new(), pilot: Pilot { bank: 0, deaths: 0, since_hit: 255 }, ticks: 0 }
    }

    /// Power-on state in place (the C64 keeps `Game` zeroed in BSS and calls this at boot).
    #[inline(never)]
    pub fn reset(&mut self) {
        self.world.hi = 0;
        self.world.reset();
        self.pilot = Pilot { bank: 0, deaths: 0, since_hit: 255 };
        self.ticks = 0;
    }

    /// One tick with this tick's input.
    #[inline(never)]
    pub fn step(&mut self, input: u8) {
        self.ticks = self.ticks.wrapping_add(1);
        self.world.step(input);
        let p = &mut self.pilot;
        let target = if self.world.state != PLAY {
            0
        } else if input & LEFT != 0 {
            -4
        } else if input & RIGHT != 0 {
            4
        } else {
            0
        };
        if self.world.tick & 1 == 0 {
            if p.bank < target {
                p.bank += 1;
            } else if p.bank > target {
                p.bank -= 1;
            }
        }
        if self.world.deaths != p.deaths {
            p.deaths = self.world.deaths;
            p.since_hit = 0;
        } else if p.since_hit != 255 {
            p.since_hit += 1;
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}

/// Stage lines travelled, within the looping stage.
fn lines(g: &Game) -> u16 {
    (g.world.scroll_lines() % PANO_PERIOD as u32) as u16
}

/// Panorama line at the top of the screen.
pub fn scroll(g: &Game) -> u16 {
    demo::scroll(lines(g))
}

/// A sprite of `w` × `h` (multicolor px × lines) centred on playfield (`x`, `y`) (hires px, lines).
#[inline(never)]
fn place(frame: u16, x: i16, y: i16, w: u16, h: u16, z: i16) -> Sprite {
    Sprite { frame, x: ((x + OX) >> 1) - (w >> 1) as i16, y: y + OY - (h >> 1) as i16, z }
}

/// The GANTRY QUEEN's approach (the stage boss; not yet fought, `docs/validation.md`): she descends into
/// view over the hangar apron and lifts away before the stage loops. Lines above her hover line, or None
/// while she is off screen.
fn queen_lift(lines: u16) -> Option<i16> {
    const ARRIVE: u16 = 4400;
    const HOVER: u16 = 4560;
    const LEAVE: u16 = 5040;
    if lines < ARRIVE {
        None
    } else if lines < HOVER {
        Some(((HOVER - lines) * 2) as i16)
    } else if lines < LEAVE {
        Some(0)
    } else {
        Some(((lines - LEAVE) * 4) as i16)
    }
}

/// Fill `out` with this frame's sprites; returns how many.
#[inline(never)]
pub fn scene(g: &Game, out: &mut [Sprite; INSTANCES]) -> usize {
    let w = &g.world;
    let mut n = 0;
    let (px, py) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16);
    if let Some(lift) = queen_lift(lines(g)) {
        demo::queen(w.tick as u16, lift, &mut out[..QUEEN_SPRITES]);
        n = QUEEN_SPRITES;
    }
    for k in 0..GANNETS {
        let gn = &w.gannets[k];
        if gn.state != 0 {
            let pose = match gn.state {
                G_DIVE => 0,
                G_TELEGRAPH => if gn.t > 8 { 1 } else { 2 },
                G_PULLUP => 2 + (GANNET_PULLUP_TICKS - gn.t as u16) / 9,
                G_CLIMB => GANNET_PITCHES + 1,
                _ => 2,
            };
            out[n] = place(GANNET_FRAME + pose, (gn.x >> 8) as i16, (gn.y >> 8) as i16, GANNET_W, GANNET_H, Z_GANNET);
            n += 1;
        }
    }
    for k in 0..WASPS {
        let wp = &w.wasps[k];
        if wp.alive {
            // Sim heading h (0 up, clockwise, 16 steps); the frames turn counter-clockwise from nose-down.
            let h = (wp.angle.wrapping_add(8) >> 4) as u16;
            let frame = DRONE_FRAME + (8u16.wrapping_sub(h) & 15);
            out[n] = place(frame, (wp.x >> 8) as i16, (wp.y >> 8) as i16, DRONE_W, DRONE_H, Z_WASP);
            n += 1;
        }
    }
    for k in 0..SPARKS {
        let s = &w.sparks[k];
        if s.alive {
            let frame = SPARK_FRAME + ((s.t >> 3) & 3) as u16;
            out[n] = place(frame, s.x0 + sway(s.t), (s.y >> 8) as i16, SPARK_W, SPARK_H, Z_SPARK);
            n += 1;
        }
    }
    for k in 0..BURSTS {
        let b = &w.bursts[k];
        if b.t != 0 {
            let age = (crate::sim::actors::BURST_TICKS - b.t) as u16 / 4;
            out[n] = place(BURST_FRAME + age.min(BURST_FRAMES - 1), b.x, b.y, BURST_W, BURST_H, Z_BURST);
            n += 1;
        }
    }
    for k in 0..PAIRS {
        let p = &w.pairs[k];
        if p.alive != 0 {
            out[n] = place(PAIR_FRAME + p.alive as u16 - 1, p.x, p.y + PAIR_H as i16 / 2, PAIR_W, PAIR_H, Z_PAIR);
            n += 1;
        }
    }
    let p = &w.player;
    let blink = p.invuln_kind == INV_RESPAWN && p.invuln & 2 != 0;
    if (w.state == PLAY || w.state == LAUNCH) && !blink {
        let frame = PLAYER_FRAME + (g.pilot.bank + 4) as u16;
        out[n] = place(frame, px, py, PLAYER_W, PLAYER_H, Z_PLAYER);
        n += 1;
    }
    for k in 0..EBULLETS {
        let b = &w.ebullets[k];
        if b.kind != 0 {
            let (frame, bw, bh) = if b.kind == B_BOMB { (BOMB_FRAME, BOMB_W, BOMB_H) } else { (ROUND_FRAME, ROUND_W, ROUND_H) };
            out[n] = place(frame, b.x >> 6, b.y >> 6, bw, bh, Z_BULLET);
            n += 1;
        }
    }
    n
}

const GANNET_PULLUP_TICKS: u16 = crate::sim::actors::GANNET_PULLUP as u16;

/// What the HUD shows.
#[inline(never)]
pub fn hud_state(g: &Game) -> hud::State {
    let w = &g.world;
    hud::State {
        score: w.score,
        lives: w.lives,
        // The sim's cursor is 0 (none) or slot 1..8; the HUD's is slot 0..7 or 8 (none).
        gauge: if w.gauge.cursor == 0 { 8 } else { w.gauge.cursor - 1 },
        speed: w.gauge.speed,
        stage: 1,
        bank: g.pilot.bank,
        tape_l: w.tick as u16,
        tape_r: lines(g),
        since_hit: g.pilot.since_hit,
    }
}

/// The attract mode's game after `ticks` ticks of the autopilot from a new game (host: replays what the
/// C64 shows when nobody touches the stick).
#[cfg(feature = "host")]
pub fn replay(ticks: u32) -> Game {
    let mut g = Game::new();
    for _ in 0..ticks {
        let input = crate::sim::replay::autopilot(&g.world);
        g.step(input);
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_scene_follows_the_sim() {
        let mut g = Game::new();
        let mut out = [Sprite::default(); INSTANCES];
        let mut most = 0;
        let mut boss_seen = false;
        for t in 0..10_400u32 {
            let input = crate::sim::replay::autopilot(&g.world);
            g.step(input);
            let n = scene(&g, &mut out);
            most = most.max(n);
            boss_seen |= out[..n].iter().any(|s| s.frame < demo::ARM_FRAME);
            for s in &out[..n] {
                assert!(s.frame < demo::FRAMES, "tick {t}: frame {}", s.frame);
            }
        }
        assert!(boss_seen, "the GANTRY QUEEN shows over the apron");
        assert!(most > 10);
    }

    #[test]
    fn reset_from_zero_is_a_new_game() {
        // SAFETY: Game is integers and bools only.
        let mut g: Game = unsafe { core::mem::zeroed() };
        g.reset();
        assert_eq!(g, Game::new());
    }

    #[test]
    fn bank_eases_and_flash_follows_deaths() {
        let mut g = Game::new();
        g.world.track_base = 1_000_000;
        for _ in 0..20 {
            g.step(RIGHT);
        }
        assert_eq!(g.pilot.bank, 4);
        for _ in 0..3 {
            g.step(0);
        }
        assert!(g.pilot.bank < 4 && g.pilot.bank > 0);
        assert_eq!(g.pilot.since_hit, 255);
    }

    #[test]
    fn wasp_heading_maps_to_its_yaw_frame() {
        // Heading 0 (up) is frame 8 (rendered nose up); heading 8 (down) frame 0; 4 (right) frame 4.
        for (h, f) in [(0u16, 8u16), (8, 0), (4, 4), (12, 12)] {
            assert_eq!(8u16.wrapping_sub(h) & 15, f);
        }
    }
}
