//! The player half of a tick (C64 phase `p.sim`): the TALON's movement and clamps, TWIN fire, the shots,
//! then collisions (`collide`), the gauge (`gauge`), death and respawn.
use super::world::*;
use super::*;

/// SPEED levels 0..5 in NTSC Q8 px/tick (`ship-talon.md` speed_levels_q8, `docs/design.md` 3).
pub static SPEED_Q8: [i32; 6] = [384, 448, 512, 576, 640, 704];
/// The same speeds scaled for diagonals, `speed * DIAGONAL >> 8` (a table: no multiply on the 6502).
pub static SPEED_DIAGONAL_Q8: [i32; 6] = [271, 316, 362, 407, 452, 497];
/// Floor after the (M3) modifiers.
pub const SPEED_MIN: i32 = 320;
/// Diagonal scale 181/256 (~1/sqrt 2), so diagonal speed equals straight speed.
pub const DIAGONAL: i32 = 181;
/// Anchor bounds (`docs/design.md` 3): the 2x2 TALON (48 x 42) stays inside the 304-px safe area and
/// between the HUD rows (playfield lines 0..175).
pub const LEFT_X: i32 = 24;
pub const RIGHT_X: i32 = 279;
pub const TOP: i32 = 21;
pub const BOTTOM: i32 = 155;
/// The boss ceiling: the TALON's top edge stays below playfield line 88 (anchor y >= 109) while a wide
/// sprite boss is on screen; the clamp eases in over 30 ticks (88 px / 30 ticks, Q8).
pub const CEILING_TOP: i32 = 109;
pub const CEILING_EASE: i32 = (CEILING_TOP - TOP) * PX / 30;
/// TWIN (`docs/design.md` 6.4): a pair every 6 ticks, 8 px/tick, bolts 4 x 10 at x -4 and +4.
pub const TWIN_INTERVAL: u8 = 6;
pub const TWIN_SPEED: i16 = 8;
pub const BOLT_W: i16 = 4;
pub const BOLT_H: i16 = 10;
pub const BOLT_DX: i16 = 4;
/// Invulnerability windows (`docs/design.md` 5).
pub const ARM_INVULN: u8 = 120;
pub const RESPAWN_INVULN: u8 = 180;
/// Death sequence (`docs/design.md` 9): death frames 15 + 120 ticks of flying on, then a 60-tick launch.
pub const DYING_TICKS: u16 = 135;
pub const LAUNCH_TICKS: u16 = 60;
/// The launch climbs from below the screen to the start position.
pub const LAUNCH_FROM_Y: i32 = 200;
pub const LAUNCH_STEP: i32 = (LAUNCH_FROM_Y * PX - START_Y) / LAUNCH_TICKS as i32;
/// GAME OVER shows for 5 s, then a new game starts.
pub const GAME_OVER_TICKS: u16 = 300;

impl World {
    /// The player half of the tick, with this tick's input.
    #[inline(never)]
    pub fn step_sim(&mut self, input: u8) {
        match self.state {
            PLAY => {
                self.move_player(input);
                self.arm(input);
                self.fire(input);
            }
            DYING => {
                self.timer -= 1;
                if self.timer == 0 {
                    if self.lives == 0 {
                        self.state = GAME_OVER;
                        self.timer = GAME_OVER_TICKS;
                        if self.score > self.hi { self.hi = self.score; }
                    } else {
                        self.state = LAUNCH;
                        self.timer = LAUNCH_TICKS;
                        self.player.x = START_X;
                        self.player.y = LAUNCH_FROM_Y * PX;
                        self.player.vx = 0;
                    }
                }
            }
            LAUNCH => {
                self.timer -= 1;
                // A straight climb to START_Y over LAUNCH_TICKS.
                self.player.y -= LAUNCH_STEP;
                if self.timer == 0 {
                    self.state = PLAY;
                    self.player.invuln = RESPAWN_INVULN;
                    self.player.invuln_kind = INV_RESPAWN;
                    self.player.fire_timer = 0;
                }
            }
            _ => {
                self.timer -= 1;
                if self.timer == 0 {
                    self.reset();
                    return;
                }
            }
        }
        self.move_shots();
        self.collide();
        if self.player.invuln != 0 {
            self.player.invuln -= 1;
            if self.player.invuln == 0 { self.player.invuln_kind = INV_NONE; }
        }
        self.player.arm_held = input & ARM != 0;
    }

    /// 8-way instant velocity at the current SPEED, diagonals normalised, then the clamps.
    #[inline(never)]
    fn move_player(&mut self, input: u8) {
        let level = self.gauge.speed as usize;
        let diagonal = input & (LEFT | RIGHT) != 0 && input & (UP | DOWN) != 0;
        // TALON HD (`ship`): the airframe's speed, and with a slide, sideways a quarter of the way to the
        // stick's velocity a tick (the TALON: all of it).
        let speed = if diagonal { SPEED_DIAGONAL_Q8[level] as i16 + self.diagonal_bias } else { SPEED_Q8[level] as i16 + self.speed_bias };
        let slide = self.slide;
        let p = &mut self.player;
        let want = if input & LEFT != 0 { -speed } else if input & RIGHT != 0 { speed } else { 0 };
        // The quarter rounds toward zero and the last of it snaps, so the slide settles exactly both ways.
        let d = want - p.vx;
        let q = (if d < 0 { d + 3 } else { d }) >> 2;
        p.vx = if slide == 0 || q == 0 { want } else { p.vx + q };
        p.x += p.vx as i32;
        if input & UP != 0 { p.y -= speed as i32 } else if input & DOWN != 0 { p.y += speed as i32 }
        // The boss ceiling eases toward its target; the anchor never goes above it.
        let target = if self.boss_ceiling { CEILING_TOP * PX } else { TOP * PX };
        if self.ceiling < target {
            self.ceiling += CEILING_EASE;
            if self.ceiling > target { self.ceiling = target; }
        } else if self.ceiling > target {
            self.ceiling -= CEILING_EASE;
            if self.ceiling < target { self.ceiling = target; }
        }
        // TALON HD: a wall stops the slide too (`ship`).
        if p.x < LEFT_X * PX {
            p.x = LEFT_X * PX;
            p.vx = 0;
        } else if p.x > RIGHT_X * PX {
            p.x = RIGHT_X * PX;
            p.vx = 0;
        }
        if p.y < self.ceiling { p.y = self.ceiling } else if p.y > BOTTOM * PX { p.y = BOTTOM * PX }
    }

    /// Fire held: a TWIN pair from the nose every TWIN_INTERVAL ticks (built-in autofire).
    #[inline(never)]
    fn fire(&mut self, input: u8) {
        if self.player.fire_timer != 0 { self.player.fire_timer -= 1; }
        if input & FIRE == 0 || self.player.fire_timer != 0 { return; }
        // TALON HD (`ship`): an airframe with a spread fires two pairs, `spread` either side of the nose,
        // or none (a lopsided shot never: `room`).
        if !self.room() { return; }
        let x = (self.player.x >> 8) as i16;
        let spread = self.spread;
        let mut dx = -spread;
        let mut i = 0;
        while i < PAIRS {
            if self.pairs[i].alive == 0 {
                self.pairs[i] = Pair { alive: 3, x: x + dx, y: (self.player.y >> 8) as i16 - 21 - BOLT_H + 4 };
                self.player.fire_timer = TWIN_INTERVAL;
                if dx >= 0 { return; }
                dx = spread;
            }
            i += 1;
        }
    }

    /// TALON HD (`ship`): room for a shot: always for one pair (`fire` finds a slot or not), two free
    /// slots for a spread shot. Pure and step-only, so on the C64 it lives in the RAM under the I/O chips
    /// (`.hitext` has no room).
    #[inline(never)]
    #[cfg_attr(target_arch = "mos", unsafe(link_section = ".iotext.room"))]
    fn room(&self) -> bool {
        if self.spread == 0 {
            return true;
        }
        let mut free = 0u8;
        let mut i = 0;
        while i < PAIRS {
            if self.pairs[i].alive == 0 {
                free += 1;
            }
            i += 1;
        }
        free >= 2
    }

    #[inline(never)]
    fn move_shots(&mut self) {
        let extra = self.bolt_speed_extra;
        let mut i = 0;
        while i < PAIRS {
            let pair = &mut self.pairs[i];
            if pair.alive != 0 {
                pair.y -= TWIN_SPEED + extra;
                if pair.y + BOLT_H <= 0 { pair.alive = 0; }
            }
            i += 1;
        }
    }
}
