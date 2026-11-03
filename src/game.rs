//! TALON's game on TALON HD's screen: the vendored simulation (`crate::sim`) stepped one tick per frame,
//! and a pure mapping from its state to software sprites, the HUD and the scroll.
//!
//! Coordinates: the sim's playfield is 304 × 176 logical (hires) pixels; it sits centred in the 320 × 192
//! visible screen, 8 px in from the left and 8 lines down. Sprite x is in multicolor pixels (hires / 2).
//! Everything here is a function of `Game` alone, which is itself a function of the input log, so the host
//! replays the C64's attract mode to any tick and renders the identical frame.

use crate::boss::Boss;
use crate::clear::Clear;
use crate::ground::{self, Ground};
use crate::midboss::Midboss;
use crate::demo::{self, *};
use crate::render::Palette;
use crate::hud;
use crate::render::Sprite;
use crate::sim::actors::{B_BOMB, G_CLIMB, G_DIVE, G_PULLUP, G_TELEGRAPH};
use crate::sim::common::sway;
use crate::sim::world::*;
use crate::sim::{LEFT, RIGHT, World};

/// Where the playfield sits in the bitmap (hires px, lines): with XSCROLL at rest (4) in 38-column mode
/// the visible bitmap is hires 3..306, so the 304-px field at 4 shows whole.
pub const OX: i16 = 4;
pub const OY: i16 = 8;

/// Every sprite a frame can hold: the boss, the pools of the sim and the TALON. Later levels' fights share
/// the KEELBREAKER's and QUEEN's slots (`KEEL_SPRITES + QUEEN_SPRITES`; the ASTROLABE's most is 12).
pub const INSTANCES: usize = ground::UNITS + KEEL_SPRITES + QUEEN_SPRITES + GANNETS + WASPS + SPARKS + BURSTS + EBULLETS + PAIRS + 1;
/// The KEELBREAKER's hull, three turrets and two mines.
pub(crate) const KEEL_SPRITES: usize = 6;

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
    /// The GANTRY QUEEN fight (`crate::boss`).
    pub boss: Boss,
    /// The KEELBREAKER fight (`crate::midboss`).
    pub midboss: Midboss,
    /// SKIFFs, PILLBOXes, HATCHes and CRAWLERs (`crate::ground`).
    pub ground: Ground,
    /// The attract mode's pilot (`crate::ace`): its plan and stick, replayed with the rest.
    pub ace: crate::ace::Ace,
    /// The stage clear and the promotion (`crate::clear`): the stage and ship being flown.
    pub clear: Clear,
    /// S2's midboss (`crate::campanile`).
    pub campanile: crate::campanile::Campanile,
    /// S2's boss (`crate::astrolabe`).
    pub astro: crate::astrolabe::Astrolabe,
}

impl Game {
    pub const fn new() -> Game {
        Game { world: World::new(), pilot: Pilot { bank: 0, deaths: 0, since_hit: 255 }, ticks: 0, boss: Boss { state: 0, hp: [0; 4], core: 0, broken: 0, turn: 0, pat: 0, step: 0, timer: 0, sink: 0, aim: [0; 2], wreck_frame: 0, wreck_x: 0, wreck_y: 0 }, midboss: Midboss { state: 0, y: 0, hp: [0; 3], core: 0, clock: 0, turn: 0, pat: 0, step: 0, timer: 0, mines: [crate::midboss::Mine { t: 0, x: 0, y: 0, vx: 0, vy: 0 }; 2], yaw: [0; 3], aim: [0; 2] }, ground: Ground::new(), ace: crate::ace::Ace::new(), clear: Clear::new(), campanile: crate::campanile::Campanile::new(), astro: crate::astrolabe::Astrolabe::new() }
    }

    /// Power-on state in place (the C64 keeps `Game` zeroed in BSS and calls this at boot).
    #[inline(never)]
    pub fn reset(&mut self) {
        self.world.hi = 0;
        self.world.reset();
        self.pilot = Pilot { bank: 0, deaths: 0, since_hit: 255 };
        self.ticks = 0;
        self.boss = Boss::default();
        self.midboss = Midboss::default();
        self.ground = Ground::new();
        self.ace = crate::ace::Ace::new();
        self.clear = Clear::new();
        self.campanile = crate::campanile::Campanile::new();
        self.astro = crate::astrolabe::Astrolabe::new();
    }

    /// Power-on state on the level of the image in the REU (`format::header::LEVEL`): new games start
    /// there; stage clears go on up to level `top` (C64 boot, paged with `reset`: `clear::LAST_LEVEL` when
    /// the machine can load the other levels' images, `crate::uci`, else the booted level again).
    #[inline(never)]
    pub fn reset_at(&mut self, level: u8, top: u8) {
        self.reset();
        self.clear = Clear::at(level, top);
    }

    /// One tick with this tick's input.
    #[inline(never)]
    pub fn step(&mut self, input: u8) {
        self.ticks = self.ticks.wrapping_add(1);
        // The stage clear flies the ship itself: the world sees no stick.
        self.world.step(if self.clear.holds() { 0 } else { input });
        // The midboss and boss fight logic are paged phases (`crate::paging`), paged in only while active.
        let at = lines(self);
        if self.clear.level != 0 {
            // Another level's fights (none yet): S1's stay where they were.
        } else if at < crate::midboss::ARRIVE {
            if self.midboss.state != crate::midboss::ABSENT {
                self.midboss = Midboss::default();
            }
        } else if self.midboss.state != crate::midboss::GONE {
            crate::paging::page(crate::paging::MIDBOSS);
            self.midboss.step(&mut self.world);
        }
        // The fight logic is a paged phase (`crate::paging`): only paged in while she is around.
        if self.clear.level != 0 {
            // S2's boss (stepped through the stage clear too: the clear follows its fall).
            self.world.boss_ceiling = false;
            let a = &mut self.astro;
            if (at >= crate::astrolabe::ARRIVE && a.state != crate::astrolabe::GONE) || (at < crate::astrolabe::ARRIVE && a.state != crate::astrolabe::ABSENT) {
                crate::paging::page(crate::paging::ASTRO);
                a.step(&mut self.world, &mut self.clear, at);
            }
        } else if lines(self) < crate::boss::ARRIVE {
            if self.boss.state != crate::boss::ABSENT {
                self.boss = Boss::default();
            }
            self.world.boss_ceiling = false;
        } else if self.boss.state != crate::boss::GONE {
            crate::paging::page(crate::paging::BOSS);
            self.boss.step(&mut self.world);
            if matches!(self.boss.state, crate::boss::PHASE1 | crate::boss::PHASE2) {
                self.clear.fight += 1;
            }
        } else {
            // She is gone until the next pass: no paging, no step.
            self.world.boss_ceiling = false;
        }
        // The stage clear (paged): from the boss's fall to the next stage's first line.
        let boss = if self.clear.level != 0 { self.astro.state } else { self.boss.state };
        if self.clear.holds() || boss == crate::boss::DYING {
            crate::paging::page(crate::paging::STAGE);
            self.clear.step(&mut self.world, boss, input);
        }
        if self.world.state == GAME_OVER && self.clear.stage != self.clear.first {
            // A new game starts on the first level with its ship (and nothing of a later level's fights).
            crate::paging::page(crate::paging::STAGE);
            self.clear.restart();
            // S2's fights stop where they are (GONE: not stepped at their hold behind the GAME OVER);
            // below their lines the new game resets them, with `hot` its own.
            self.campanile.state = crate::campanile::GONE;
            self.astro.state = crate::astrolabe::GONE;
            self.astro.pose = 0;
        }
        // The ground units' step is a paged phase too, paged in only while one is out or due (frozen
        // through the stage clear, which starts the next stage at its first line). Later levels run
        // their own units and waves (`crate::stage2`, which holds the vendored sim's S1 track off).
        let at = lines(self);
        if !self.clear.holds() {
            if self.clear.level != 0 {
                crate::stage2::run(self, at);
            } else if self.ground.busy(at) {
                crate::paging::page(crate::paging::GROUND);
                self.ground.step(&mut self.world, at);
            }
        }
        // The stage loops: the world's scroll back to line 0 (its fraction kept), so the scroll is the
        // stage line. S1's spawn track loops with the stage (its base is a period by now: 0 is the same
        // place); S2's waves hold it off again on their next step. Byte stores only: no 32-bit math.
        if lines(self) >= PANO_PERIOD {
            self.world.scroll &= 0xFF;
            self.world.track_base = 0;
        }
        let p = &mut self.pilot;
        let target = if self.clear.holds() {
            self.clear.bank
        } else if self.world.state != PLAY {
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

/// Stage lines travelled, within the looping stage (`Game::step` keeps the world's scroll inside one
/// period: no 32-bit modulo on the 6502).
fn lines(g: &Game) -> u16 {
    g.world.scroll_lines() as u16
}

/// The camera shake after the TALON is hit, per frame: on each axis two sines of different frequency
/// and amplitude added together, under an envelope that decays to rest over `SHAKE_FRAMES`. Evaluated
/// at compile time; the C64 only reads the two tables.
pub const SHAKE_FRAMES: usize = 24;
pub const SHAKE_MAX_X: i8 = 12;
pub const SHAKE_MAX_Y: i8 = 6;
const fn harmonic(t: usize, a1: f64, p1: f64, ph1: f64, a2: f64, p2: f64, ph2: f64, max: i8) -> i8 {
    let tau = 2.0 * core::f64::consts::PI;
    let u = 1.0 - t as f64 / SHAKE_FRAMES as f64;
    let env = u * u;
    let v = env * (a1 * demo::const_sin(tau * t as f64 / p1 + ph1) + a2 * demo::const_sin(tau * t as f64 / p2 + ph2));
    let r = (if v < 0.0 { v - 0.5 } else { v + 0.5 }) as i32;
    (if r > max as i32 { max as i32 } else if r < -(max as i32) { -(max as i32) } else { r }) as i8
}
const fn shake_table(x: bool) -> [i8; SHAKE_FRAMES] {
    let mut out = [0i8; SHAKE_FRAMES];
    let mut t = 0;
    while t < SHAKE_FRAMES {
        // x: 9 px with a 5.5-frame period plus 4 px with a 2.6-frame period;
        // y: 4.5 lines every 6.5 frames plus 2 lines every 3.1.
        out[t] = if x {
            harmonic(t, 9.0, 5.5, 2.0, 4.0, 2.6, 1.0, SHAKE_MAX_X)
        } else {
            harmonic(t, 4.5, 6.5, 1.0, 2.0, 3.1, 0.3, SHAKE_MAX_Y)
        };
        t += 1;
    }
    out
}
pub const SHAKE_X: [i8; SHAKE_FRAMES] = shake_table(true);
pub const SHAKE_Y: [i8; SHAKE_FRAMES] = shake_table(false);

/// This frame's camera shake (hires px right, lines down).
pub fn shake(g: &Game) -> (i8, i8) {
    let t = g.pilot.since_hit as usize;
    if t < SHAKE_FRAMES { (SHAKE_X[t], SHAKE_Y[t]) } else { (0, 0) }
}

/// The shake's sideways part split into whole 8-px columns (background copy and sprites) and the
/// XSCROLL remainder (-4..=3 about rest).
fn shake_cols(sx: i8) -> (i8, i8) {
    let col = (sx as i16 + 4).div_euclid(8) as i8;
    (col, sx - 8 * col)
}

/// The view this frame: panorama window, column shift and XSCROLL, moved by the shake (sprites follow
/// in `scene`). Never inlined: it lives in `.hitext`, not main RAM (`target-c64/link.ld`).
#[inline(never)]
pub fn view(g: &Game, img: &crate::render::Image) -> crate::render::Scroll {
    let (sx, sy) = shake(g);
    let line = (demo::scroll(lines(g)) as i16 - sy as i16).rem_euclid(PANO_PERIOD as i16) as u16;
    let mut v = crate::render::Scroll::new(img, line);
    let (col, fine) = shake_cols(sx);
    v.col = col;
    v.xs = (crate::layout::XSCROLL_REST as i8 + fine) as u8;
    v
}

/// Panorama line at the top of the screen (no shake).
pub fn scroll(g: &Game) -> u16 {
    demo::scroll(lines(g))
}

/// A sprite of `w` × `h` (multicolor px × lines) centred on playfield (`x`, `y`) (hires px, lines).
#[inline(never)]
fn place(frame: u16, x: i16, y: i16, w: u16, h: u16, z: i16) -> Sprite {
    Sprite { frame, x: ((x + OX) >> 1) - (w >> 1) as i16, y: y + OY - (h >> 1) as i16, z }
}

/// Append `place(..)` to the frame's sprites (one store site for the whole scene: 6502 code size).
#[inline(never)]
pub(crate) fn put(out: &mut [Sprite; INSTANCES], n: &mut usize, frame: u16, x: i16, y: i16, w: u16, h: u16, z: i16) {
    if *n < INSTANCES {
        out[*n] = place(frame, x, y, w, h, z);
        *n += 1;
    }
}

/// The stage palette with the fight's recolor (phase 2 turns `hot` yellow), or the stage clear's (the
/// crystal going dark, the walk to the next stage's colors, and every later stage's).
pub fn palette(g: &Game, base: Palette) -> Palette {
    g.clear.palette(g.boss.palette(base))
}

/// Fill `out` with this frame's sprites; returns how many.
#[inline(never)]
pub fn scene(g: &Game, out: &mut [Sprite; INSTANCES]) -> usize {
    let w = &g.world;
    let mut n = 0;
    let (px, py) = ((w.player.x >> 8) as i16, (w.player.y >> 8) as i16);
    g.ground.sprites(out, &mut n);
    if g.campanile.shown() {
        crate::paging::page(crate::paging::S2);
        g.campanile.sprites(out, &mut n);
    }
    if g.astro.shown() {
        crate::paging::page(crate::paging::ASTRO);
        g.astro.sprites(out, &mut n);
    }
    n += g.midboss.sprites(w, &mut out[n..n + KEEL_SPRITES]);
    n += g.boss.sprites(w, &mut out[n..n + QUEEN_SPRITES]);
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
            put(out, &mut n, GANNET_FRAME + pose, (gn.x >> 8) as i16, (gn.y >> 8) as i16, GANNET_W, GANNET_H, Z_GANNET);
        }
    }
    for k in 0..WASPS {
        let wp = &w.wasps[k];
        if wp.alive {
            // Sim heading h (0 up, clockwise, 16 steps); the frames turn counter-clockwise from nose-down.
            let h = (wp.angle.wrapping_add(8) >> 4) as u16;
            let frame = DRONE_FRAME + (8u16.wrapping_sub(h) & 15);
            put(out, &mut n, frame, (wp.x >> 8) as i16, (wp.y >> 8) as i16, DRONE_W, DRONE_H, Z_WASP);
        }
    }
    for k in 0..SPARKS {
        let s = &w.sparks[k];
        if s.alive {
            let frame = SPARK_FRAME + ((s.t >> 3) & 3) as u16;
            put(out, &mut n, frame, s.x0 + sway(s.t), (s.y >> 8) as i16, SPARK_W, SPARK_H, Z_SPARK);
        }
    }
    for k in 0..BURSTS {
        let b = &w.bursts[k];
        if b.t != 0 {
            let age = (crate::sim::actors::BURST_TICKS - b.t) as u16 / 4;
            put(out, &mut n, BURST_FRAME + age.min(BURST_FRAMES - 1), b.x, b.y, BURST_W, BURST_H, Z_BURST);
        }
    }
    for k in 0..PAIRS {
        let p = &w.pairs[k];
        if p.alive != 0 {
            put(out, &mut n, PAIR_FRAME + p.alive as u16 - 1, p.x, p.y + PAIR_H as i16 / 2, PAIR_W, PAIR_H, Z_PAIR);
        }
    }
    let p = &w.player;
    if w.state == PLAY || w.state == LAUNCH {
        let bank = (g.pilot.bank + 4) as u16;
        // The ship of this stage: the TALON, then (promoted) the PEREGRINE. Invincible (respawn or
        // arming): the energy-wave frames instead of a blink.
        let promoted = g.clear.stage != 0;
        let step = (w.tick as u16 / 3) % SHIELD_STEPS;
        let (frame, sw, sh) = if promoted {
            if p.invuln_kind != INV_NONE {
                put(out, &mut n, PEREGRINE_SHIELD_FRAME + step, px, py, SHIELD_W, SHIELD_H, Z_PLAYER);
            }
            (PEREGRINE_FRAME + bank, PEREGRINE_W, PEREGRINE_H)
        } else if p.invuln_kind != INV_NONE {
            (SHIELD_FRAME + bank * SHIELD_STEPS + step, SHIELD_W, SHIELD_H)
        } else {
            (PLAYER_FRAME + bank, PLAYER_W, PLAYER_H)
        };
        put(out, &mut n, frame, px, py, sw, sh, Z_PLAYER);
    }
    for k in 0..EBULLETS {
        let b = &w.ebullets[k];
        if b.kind != 0 {
            let (frame, bw, bh) = if b.kind == B_BOMB { (BOMB_FRAME, BOMB_W, BOMB_H) } else { (ROUND_FRAME, ROUND_W, ROUND_H) };
            put(out, &mut n, frame, b.x >> 6, b.y >> 6, bw, bh, Z_BULLET);
        }
    }
    if g.clear.card() {
        crate::paging::page(crate::paging::STAGE);
        g.clear.sprites(out, &mut n);
    }
    // The camera shake moves everything drawn with the panorama: whole columns here, the rest is XSCROLL.
    let (sx, sy) = shake(g);
    let (dx, dy) = (4 * shake_cols(sx).0 as i16, sy as i16);
    if dx != 0 || dy != 0 {
        for s in out[..n].iter_mut() {
            s.x += dx;
            s.y += dy;
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
        stage: g.clear.stage + 1,
        bank: g.pilot.bank,
        tape_l: w.tick as u16,
        tape_r: lines(g),
        since_hit: g.pilot.since_hit,
        shake: shake(g),
        border: 0,
    }
}

/// The attract mode's game after `ticks` ticks of the autopilot from a new game (host: replays what the
/// C64 shows when nobody touches the stick).
#[cfg(feature = "host")]
pub fn replay(ticks: u32) -> Game {
    replay_level(0, crate::clear::LAST_LEVEL, ticks)
}

/// [`replay`] on a C64 booted with level `level`'s image (`Game::reset_at`) that goes on up to level
/// `top` (`clear::LAST_LEVEL` when it loads the other levels' images as it reaches them, `crate::uci`;
/// its booted level when it cannot: the C64's `status::TOP`). A `top` under `level` means `level`.
#[cfg(feature = "host")]
pub fn replay_level(level: u8, top: u8, ticks: u32) -> Game {
    let mut g = Game::new();
    g.reset_at(level, top.max(level));
    let mut mind = crate::ace::Mind::new();
    for _ in 0..ticks {
        let input = crate::ace::fly(&mut g, &mut mind);
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
        // The attract autopilot rarely survives to the apron (the midboss fight slows the stage): start a
        // game near her line instead.
        let mut g = Game::new();
        g.world.track_base = 1_000_000;
        g.world.scroll = (crate::boss::ARRIVE as u32 - 20) << 8;
        for _ in 0..600 {
            g.step(0);
            let n = scene(&g, &mut out);
            boss_seen |= out[..n].iter().any(|s| s.frame < demo::ARM_FRAME);
        }
        assert!(boss_seen, "the GANTRY QUEEN shows over the apron");
        assert!(most > 10);
    }

    #[test]
    fn the_shake_is_hard_then_settles() {
        let peak_x = SHAKE_X.iter().map(|v| v.abs()).max().unwrap();
        let peak_y = SHAKE_Y.iter().map(|v| v.abs()).max().unwrap();
        assert!(peak_x >= 9 && peak_x <= SHAKE_MAX_X, "{SHAKE_X:?}");
        assert!(peak_y >= 4 && peak_y <= SHAKE_MAX_Y, "{SHAKE_Y:?}");
        assert!(SHAKE_X[SHAKE_FRAMES - 3..].iter().all(|v| v.abs() <= 1));
        for sx in -SHAKE_MAX_X..=SHAKE_MAX_X {
            let (col, fine) = shake_cols(sx);
            assert_eq!(col as i16 * 8 + fine as i16, sx as i16);
            assert!((-4..=3).contains(&fine));
        }
        println!("x {SHAKE_X:?}\ny {SHAKE_Y:?}");
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
