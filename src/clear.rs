//! The stage clear and QUILL's promotion (`docs/design.md` 355-368, `docs/story/01-story.md` 357-368,
//! `docs/story/hd/02-ladder.md`; effects `fx.node_dark` and `fx.promotion_walk` in
//! `docs/story/hd/01-art-direction.md`):
//!
//! 1. DARK: the boss falls; the waves stop, enemy bullets clear, the ship can no longer be hit and the
//!    stick is ignored; `hot` steps down to grey, then the stage's `dark`, while her wreck drifts off.
//! 2. FLYOUT: the ship eases to the centre and climbs off the top of the screen, faster and faster.
//! 3. CARD: the stage-clear card, a line at a time (the rank, BRANDT's line, QUILL's reply), then the
//!    tally (BOSS TIME, CLEAN AIRFRAME) counting into the score at 1,000 a tick, then a 90-tick hold
//!    (Fire skips, only a human presses it: ACE does not fire without enemies).
//! 4. WALK: every role walks to the next stage's color through the luminance groups (void first, hot
//!    last, a step every 8 ticks) while the next airframe rolls out from below, already in its colors.
//! 5. STAGE: the next stage's card for 150 ticks (or Fire); then play starts at the stage's first line.
//!
//! The next stage's content is the next level (`Clear::level`) up to the last one this machine can play
//! (`Clear::top`: the host has every built level; the C64 the level of the image it booted, until the
//! per-level REU load exists); past it, the last level again in the next stage's palette.
//!
//! State lives in [`crate::game::Game`] beside the vendored `World` and replays exactly. The step is the
//! paged phase `STAGE`; the render-time helpers ([`Clear::sprites`]) and the palette are resident.

use crate::demo::{CARD_FRAME, DIGIT_FRAME, DIGIT_W, EDGE_FRAME, LINE_H};
use crate::game::INSTANCES;
use crate::render::{Palette, Sprite};
use crate::sim::world::*;
use crate::sim::FIRE;

/// `Clear::phase`.
pub const OFF: u8 = 0;
pub const DARK: u8 = 1;
pub const FLYOUT: u8 = 2;
pub const CARD: u8 = 3;
pub const WALK: u8 = 4;
pub const STAGE: u8 = 5;

/// Each stage's four colors, void, dark, light, hot (`02-ladder.md`).
pub const PALETTES: [[u8; 4]; 8] = [
    [6, 12, 1, 3],
    [9, 8, 15, 14],
    [2, 8, 7, 3],
    [11, 12, 7, 3],
    [11, 5, 15, 3],
    [6, 5, 13, 3],
    [0, 4, 15, 7],
    [0, 6, 1, 3],
];

/// The S1 GANTRY QUEEN's time bonus (`05-bosses/s1-gantry-queen.md` `time_bonus`): the full amount
/// within `PAR` ticks (60 s), falling linearly to 0 at twice that. S2's ASTROLABE: 40,000 within 70 s.
/// The tally counts in hundreds (every bonus is a multiple of 100), so a later stage's 70,000+ fits a u16.
pub const BOSS_MAX: u16 = 300;
pub const PAR: u16 = 3_600;
pub const BOSS_MAX2: u16 = 400;
pub const PAR2: u16 = 4_200;
/// CLEAN AIRFRAME: 10,000 × the stage number when no ship was lost this stage (in hundreds).
pub const CLEAN: u16 = 100;
/// Tally speed (hundreds a tick: 1,000 points) and the card's hold after it.
pub const TALLY_STEP: u16 = 10;
pub const HOLD: u16 = 90;
/// Ticks between card lines appearing, between palette steps.
pub const LINE_TICKS: u16 = 6;
pub const WALK_TICKS: u16 = 8;
/// `fx.node_dark`: a step every 16 ticks, 48 at most.
pub const DARK_TICKS: u16 = 16;
pub const STAGE_TICKS: u16 = 150;
/// The fly-out starts this long after the fall (her 120-tick explosions, then two seconds of her
/// wreck sinking).
pub const WRECK_WAIT: u16 = 240;

/// The stage-clear card's lines (frames from `CARD_FRAME`), then the next stage's card.
pub const CLEAR_LINES: u16 = 17;
pub const STAGE_LINES: u16 = 8;
/// The tally's lines on the clear card (their values are drawn in digits at the right).
const BOSS_LINE: u16 = 14;
const CLEAN_LINE: u16 = 15;
const TOTAL_LINE: u16 = 16;
/// Screen position of the card's left edge (multicolor px, a multiple of 4: the frames are stored
/// unshifted; centred on the playfield) and of a card's top line.
pub const CARD_X: i16 = 32;
/// The tally's labels start, and its values end, this far into the plate (multicolor px).
pub const TALLY_LEFT: u16 = 8;
pub const TALLY_RIGHT: u16 = 88;
const fn card_top(lines: u16) -> i16 {
    // Centred on the playfield (8 lines down, 176 tall), the edges one line outside.
    8 + (176 - (lines * LINE_H) as i16) / 2
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Clear {
    pub phase: u8,
    /// The stage being flown: 0 = S1 (also the ship: the TALON, then the PEREGRINE).
    pub stage: u8,
    /// Ticks in this phase.
    pub t: u16,
    /// Ticks of the boss fight (her phases 1 and 2), for BOSS TIME.
    pub fight: u16,
    /// `World::deaths` when this stage began (CLEAN AIRFRAME).
    pub deaths0: u16,
    pub boss_bonus: u16,
    pub clean_bonus: u16,
    /// Points already counted into the score.
    pub counted: u16,
    /// Palette steps taken (node dark, then the walk).
    pub steps: u8,
    /// The palette shown while `pal_on` (from the clear on, and for every stage after S1).
    pub pal: [u8; 4],
    pub pal_on: bool,
    /// Fire was down last tick (Fire skips on its edge).
    pub fire: bool,
    /// The ship's bank target while the sequence flies it (-4..=4).
    pub bank: i8,
    /// Climb speed (Q4 lines per tick) during the fly-out, and its fraction.
    pub vy: u8,
    pub acc: u8,
    /// The ship's line while the sequence flies it beyond the world's clamps (written to the world
    /// every tick: the world's step clamps it back to the playfield).
    pub y: i16,
    /// The level being played (its units, waves, bosses and image: 0 = S1 BREAKWATER, 1 = S2 TERRACES),
    /// the level a new game starts on and the last one this machine can play.
    pub level: u8,
    pub first: u8,
    pub top: u8,
}

/// The last level built (S2 TERRACES).
pub const LAST_LEVEL: u8 = 1;

impl Clear {
    /// A new game from S1, every level playable (the host's).
    #[inline(always)]
    pub const fn new() -> Clear {
        Clear::at(0, LAST_LEVEL)
    }

    /// A new game from level `first` (its stage, ship and palette), playable up to level `top`.
    #[inline(always)]
    pub const fn at(first: u8, top: u8) -> Clear {
        let first = first & 7;
        Clear {
            phase: OFF,
            stage: first,
            t: 0,
            fight: 0,
            deaths0: 0,
            boss_bonus: 0,
            clean_bonus: 0,
            counted: 0,
            steps: 0,
            pal: PALETTES[first as usize],
            pal_on: first != 0,
            fire: false,
            bank: 0,
            vy: 0,
            acc: 0,
            y: 0,
            level: first,
            first,
            top,
        }
    }

    /// A new game from the first level (paged: `paging::STAGE`).
    #[inline(never)]
    pub fn restart(&mut self) {
        *self = Clear::at(self.first, self.top);
    }

    /// The sequence owns the ship and the stick (the world sees no input).
    #[inline(always)]
    pub fn holds(&self) -> bool {
        self.phase != OFF
    }

    /// The palette: the sequence's while it runs and for every stage after S1, else `base`.
    #[inline(always)]
    pub fn palette(&self, base: Palette) -> Palette {
        if self.pal_on { Palette::new(self.pal[0], self.pal[1], self.pal[2], self.pal[3]) } else { base }
    }

    /// Whether a card shows (render time pages the phase in for [`Clear::sprites`]).
    #[inline(always)]
    pub fn card(&self) -> bool {
        self.phase == CARD || self.phase == STAGE
    }

    /// Render time, paged (`paging::STAGE`, only while a card shows): the cards and the tally's digits,
    /// in front of everything.
    #[inline(never)]
    pub fn sprites(&self, out: &mut [Sprite; INSTANCES], n: &mut usize) {
        let (first, lines, shown) = match self.phase {
            CARD => (0, CLEAR_LINES, self.t / LINE_TICKS),
            STAGE => (CLEAR_LINES, STAGE_LINES, self.t / LINE_TICKS),
            _ => return,
        };
        let shown = if shown > lines { lines } else { shown };
        let top = card_top(lines);
        if shown != 0 {
            put_raw(out, n, EDGE_FRAME, CARD_X, top - 1);
        }
        let mut y = top;
        let mut i = 0;
        while i < shown {
            put_raw(out, n, CARD_FRAME + first + i, CARD_X, y);
            y += LINE_H as i16;
            i += 1;
        }
        if shown == lines {
            put_raw(out, n, EDGE_FRAME, CARD_X, y);
        }
        if self.phase == CARD && shown == lines {
            // The tally: each value counts up in turn, the total with them.
            let boss = if self.counted < self.boss_bonus { self.counted } else { self.boss_bonus };
            let clean = self.counted - boss;
            number(out, n, boss, top + (BOSS_LINE * LINE_H) as i16);
            number(out, n, clean, top + (CLEAN_LINE * LINE_H) as i16);
            number(out, n, self.counted, top + (TOTAL_LINE * LINE_H) as i16);
        }
    }

    /// One tick, after the boss's (paged: `paging::STAGE`). `input` is the stick as read.
    #[inline(never)]
    pub fn step(&mut self, w: &mut World, boss_state: u8, input: u8) {
        let fire = input & FIRE != 0 && !self.fire;
        self.fire = input & FIRE != 0;
        self.t = self.t.wrapping_add(1);
        match self.phase {
            OFF => {
                if boss_state == crate::boss::DYING {
                    self.begin(w);
                }
            }
            DARK => {
                self.hold(w);
                if self.t % DARK_TICKS == 0 && self.steps < 2 {
                    // `fx.node_dark`: the yellow crystal greys out (15), then goes to the stage's `dark`.
                    self.pal[3] = if self.steps == 0 { 15 } else { self.pal[1] };
                    self.steps += 1;
                }
                // The wreck is well on its way down (it sinks on through the fly-out and the card).
                if boss_state == crate::boss::GONE || (boss_state == crate::boss::WRECK && self.t >= WRECK_WAIT) {
                    self.next(FLYOUT);
                    self.vy = 0;
                    self.acc = 0;
                    self.y = (w.player.y >> 8) as i16;
                }
            }
            FLYOUT => {
                self.hold(w);
                // Ease to the centre, then climb ever faster off the top.
                let x = w.player.x >> 8;
                let dx = CENTRE_X - x;
                let step = if dx > 2 { 2 } else if dx < -2 { -2 } else { dx };
                w.player.x += step << 8;
                self.bank = if step > 0 { 4 } else if step < 0 { -4 } else { 0 };
                if self.t > 24 {
                    if self.vy < 120 {
                        self.vy += 2;
                    }
                    self.acc += self.vy;
                    while self.acc >= 16 {
                        self.acc -= 16;
                        self.y -= 1;
                    }
                }
                w.player.y = (self.y as i32) << 8;
                if self.y < OFF_TOP {
                    self.bank = 0;
                    self.next(CARD);
                }
            }
            CARD => {
                self.hold(w);
                let total = self.boss_bonus + self.clean_bonus;
                if fire || self.t >= CLEAR_LINES * LINE_TICKS {
                    // Fire counts the rest at once and moves on.
                    let left = total - self.counted;
                    if left != 0 {
                        let add = if fire || left <= TALLY_STEP { left } else { TALLY_STEP };
                        score(w, add);
                        self.counted += add;
                        self.t = CLEAR_LINES * LINE_TICKS;
                    }
                    if fire || (left == 0 && self.t >= CLEAR_LINES * LINE_TICKS + HOLD) {
                        self.next(WALK);
                    }
                }
            }
            WALK => {
                self.hold(w);
                if self.t % WALK_TICKS == 0 {
                    self.walk_step();
                }
                if self.t == 1 {
                    // The promotion: the next stage, the next ship, below the screen.
                    self.stage += 1;
                    self.y = ROLL_FROM;
                } else if self.y > START_LINE {
                    // It climbs in from below, already in the new colors, easing in to the start line.
                    let d = (self.y - START_LINE) >> 4;
                    self.y -= if d < 1 { 1 } else { d };
                }
                w.player.x = START_X;
                w.player.y = (self.y as i32) << 8;
                if self.y == START_LINE && self.pal == next_palette(self.stage - 1) {
                    self.next(STAGE);
                }
            }
            _ => {
                // STAGE: the next stage's card, then play.
                self.hold(w);
                w.player.y = START_Y;
                if fire || self.t >= STAGE_TICKS {
                    self.start_stage(w);
                }
            }
        }
    }

    fn next(&mut self, phase: u8) {
        self.phase = phase;
        self.t = 0;
    }

    /// The boss has fallen: score the bonuses, stop the waves, clear the bullets.
    fn begin(&mut self, w: &mut World) {
        self.next(DARK);
        self.steps = 0;
        self.counted = 0;
        self.pal = PALETTES[self.stage as usize & 7];
        self.pal[3] = crate::boss::PHASE2_HOT;
        self.pal_on = true;
        self.boss_bonus = boss_time(self.fight, self.level);
        self.clean_bonus = if w.deaths == self.deaths0 { CLEAN * (self.stage as u16 + 1) } else { 0 };
        for b in w.ebullets.iter_mut() {
            b.kind = 0;
        }
        for k in 0..WASPS {
            if w.wasps[k].alive {
                w.wasps[k].alive = false;
                crate::ground::burst(w, (w.wasps[k].x >> 8) as i16, (w.wasps[k].y >> 8) as i16);
            }
        }
        for k in 0..GANNETS {
            if w.gannets[k].state != 0 {
                w.gannets[k].state = 0;
                crate::ground::burst(w, (w.gannets[k].x >> 8) as i16, (w.gannets[k].y >> 8) as i16);
            }
        }
        for v in w.waves.iter_mut() {
            v.left = 0;
        }
        self.hold(w);
    }

    /// Every tick of the sequence: no spawns, nothing can hit the ship (no blinking: the plain frames).
    fn hold(&mut self, w: &mut World) {
        w.track_base = w.scroll_lines() + 1;
        w.player.invuln = 255;
        w.player.invuln_kind = INV_NONE;
        w.boss_ceiling = false;
    }

    /// One promotion-walk step: the first role (void first, hot last) not yet at its new color moves one
    /// luminance group toward it.
    fn walk_step(&mut self) {
        let to = next_palette(self.stage - 1);
        let mut r = 0;
        while r < 4 {
            if self.pal[r] != to[r] {
                self.pal[r] = toward(self.pal[r], to[r]);
                return;
            }
            r += 1;
        }
    }

    /// Play from the stage's first line (all fights and ground units reset themselves there).
    fn start_stage(&mut self, w: &mut World) {
        self.next(OFF);
        if self.level < self.top {
            self.level += 1;
        }
        self.fight = 0;
        self.deaths0 = w.deaths;
        self.pal = PALETTES[self.stage as usize & 7];
        w.scroll = 0;
        w.track_next = 0;
        w.track_base = 0;
        w.player.x = START_X;
        w.player.y = START_Y;
        w.player.invuln = 0;
    }
}

/// The palette after stage `s`'s clear.
fn next_palette(s: u8) -> [u8; 4] {
    PALETTES[(s as usize + 1) & 7]
}

/// The ship's x at the centre of the playfield and the line above which it has left the screen.
const CENTRE_X: i32 = 152;
const OFF_TOP: i16 = -48;
/// The new ship starts below the playfield and eases in to the start line.
const ROLL_FROM: i16 = 230;
const START_LINE: i16 = (START_Y >> 8) as i16;

/// Pepto luminance group (0 darkest .. 8 white) of each C64 color (`hd/00-palette-law.md`).
pub const LUMA: [u8; 16] = [0, 8, 2, 6, 3, 5, 1, 7, 3, 1, 5, 2, 4, 4, 4, 6];

/// One step from color `a` toward `b`: straight to `b` when it is in the next group (or the same one);
/// else through the next group's grey (11, 12, 15) when it has one, else on to the group after.
fn toward(a: u8, b: u8) -> u8 {
    let (la, lb) = (LUMA[a as usize & 15], LUMA[b as usize & 15]);
    if la == lb {
        return b;
    }
    let up = lb > la;
    let mut l = la;
    loop {
        l = if up { l + 1 } else { l - 1 };
        if l == lb {
            return b;
        }
        // The greys: 11 (group 2), 12 (4), 15 (6).
        if l == 2 {
            return 11;
        } else if l == 4 {
            return 12;
        } else if l == 6 {
            return 15;
        }
    }
}

/// BOSS TIME for a fight of `t` ticks on `level`: the full bonus up to par, nothing at twice par, in
/// steps between (a subtraction loop: no divides on the 6502): S1 100 every 12 ticks, S2 200 every 21.
/// In hundreds.
fn boss_time(t: u16, level: u8) -> u16 {
    let (par, max, every, points) = if level == 0 { (PAR, BOSS_MAX, 12, 1) } else { (PAR2, BOSS_MAX2, 21, 2) };
    if t <= par {
        return max;
    }
    if t >= 2 * par {
        return 0;
    }
    let mut left = 2 * par - t;
    let mut v = 0u16;
    while left >= every {
        left -= every;
        v += points;
    }
    v
}

/// `hundreds` × 100 points into the score (no 32-bit multiply on the 6502).
#[inline(always)]
fn score(w: &mut World, mut hundreds: u16) {
    while hundreds != 0 {
        w.add_score(100);
        hundreds -= 1;
    }
}

/// Append a sprite at screen position (`x`, `y`) (multicolor px, lines), in front of everything.
#[inline(never)]
fn put_raw(out: &mut [Sprite; INSTANCES], n: &mut usize, frame: u16, x: i16, y: i16) {
    if *n < INSTANCES {
        out[*n] = Sprite { frame, x, y, z: Z_CARD };
        *n += 1;
    }
}

/// In front of everything (`game`'s draw order ends at 8).
const Z_CARD: i16 = 9;

/// `value` hundreds (its points: the digits, then `00`) right-aligned at the card's right edge on the
/// line at `y`, with thousands commas.
#[inline(never)]
fn number(out: &mut [Sprite; INSTANCES], n: &mut usize, value: u16, y: i16) {
    let mut x = CARD_X + TALLY_RIGHT as i16 - DIGIT_W as i16;
    let mut v = value;
    // 0 is a lone 0 (no hundreds' zeros).
    let mut k = if v == 0 { 2u8 } else { 0 };
    loop {
        // The digits from the right: the hundreds' two zeros, then the value's last digit by a
        // subtraction loop (no divides on the 6502).
        let mut d = 0u16;
        if k >= 2 {
            while v >= 10 {
                v -= 10;
                d += 1;
            }
            core::mem::swap(&mut v, &mut d);
        }
        put_raw(out, n, DIGIT_FRAME + d, x, y + 1);
        x -= DIGIT_W as i16 + 1;
        k += 1;
        if k > 2 && v == 0 {
            break;
        }
        if k == 3 {
            put_raw(out, n, DIGIT_FRAME + 10, x, y + 1);
            x -= DIGIT_W as i16 + 1;
        }
    }
}

/// The cards' text per level image (host: baked into the frames from `CARD_FRAME`): the level's clear,
/// then the next stage's card. A leading `~` draws the line in `dark` (speakers, the stage's subtitle);
/// everything else is `light` on a `void` plate. Lines from `01-story.md` (cards), `hd/02-ladder.md`
/// (rank, BRANDT's line, QUILL's reply), wrapped at 19 characters.
#[cfg(feature = "host")]
pub const CARD_TEXT: [[&str; (CLEAR_LINES + STAGE_LINES) as usize]; 2] = [[
    "BREAKWATER IS OURS.",
    "THE CRYSTAL GOES",
    "DARK.",
    "PROMOTED: MAJOR",
    "~BRANDT:",
    "MAJOR VASQUEZ.",
    "THE TEN IS FUELLED.",
    "TRY TO BRING THIS",
    "ONE BACK.",
    "~QUILL:",
    "IT'S A TEST",
    "PROGRAMME, MARSHAL.",
    "I'M TESTING.",
    "",
    "<BOSS TIME",
    "<CLEAN AIRFRAME",
    "<TOTAL",
    "STAGE 2",
    "TERRACES",
    "HILLSIDE VINEYARDS",
    "",
    "THEY TURNED A",
    "TELESCOPE INTO A",
    "GUN. CLOSE THE",
    "OBSERVATORY, QUILL.",
], [
    "TERRACES IS OURS.",
    "THE CRYSTAL GOES",
    "DARK.",
    "PROMOTED: LT COL.",
    "~BRANDT:",
    "LIEUTENANT COLONEL.",
    "THE ELEVEN WAS",
    "BUILT FOR THE",
    "DESERT. SO ARE YOU",
    "NOW.",
    "~QUILL:",
    "I'LL PACK",
    "SUNSCREEN.",
    "",
    "<BOSS TIME",
    "<CLEAN AIRFRAME",
    "<TOTAL",
    "STAGE 3",
    "REDLINE",
    "~DESERT CANYON",
    "~RAILWAY",
    "EVERY TRAIN ON",
    "THAT LINE CARRIES",
    "THEIR POWER. CUT",
    "THE LINE.",
]];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;

    #[test]
    fn the_walk_goes_through_the_luminance_groups() {
        // S1 -> S2 after the node went dark: void 6 -> 9, dark 12 -> 8, light 1 -> 15, hot 12 -> 14.
        assert_eq!(toward(6, 9), 9);
        assert_eq!(toward(12, 8), 8);
        assert_eq!(toward(1, 15), 15, "white skips yellow-green (no grey in group 7)");
        assert_eq!(toward(7, 12), 15, "yellow greys out through light grey");
        assert_eq!(toward(15, 12), 12);
        // Every walk between ladder palettes ends, one group at a time at most two.
        for a in 0..16u8 {
            for b in 0..16u8 {
                let mut c = a;
                let mut n = 0;
                while c != b {
                    let d = toward(c, b);
                    assert!((LUMA[d as usize] as i8 - LUMA[c as usize] as i8).abs() <= 8);
                    c = d;
                    n += 1;
                    assert!(n < 10, "{a} -> {b}");
                }
            }
        }
    }

    #[test]
    fn boss_time_falls_from_par_to_twice_par() {
        assert_eq!(boss_time(0, 0), 300);
        assert_eq!(boss_time(PAR, 0), 300);
        assert_eq!(boss_time(PAR + 1800, 0), 150);
        assert_eq!(boss_time(2 * PAR - 1, 0), 0);
        assert_eq!(boss_time(2 * PAR, 0), 0);
        assert_eq!(boss_time(PAR2, 1), 400);
        assert_eq!(boss_time(PAR2 + 1, 1), 398);
        assert_eq!(boss_time(PAR2 + 2100, 1), 200);
        assert_eq!(boss_time(2 * PAR2 - 1, 1), 0);
    }

    /// Tally values are hundreds: drawn with their two zeros and a thousands comma (a total stays under
    /// a million: 40,000 BOSS TIME + 10,000 × stage 8).
    #[test]
    fn numbers_read_in_hundreds() {
        for (v, want) in [(0, "0"), (7, "700"), (10, "1,000"), (398, "39,800"), (700, "70,000"), (1_200, "120,000")] {
            let (mut out, mut n) = ([Sprite::default(); INSTANCES], 0);
            number(&mut out, &mut n, v, 0);
            let got: String = out[..n].iter().rev().map(|s| if s.frame == DIGIT_FRAME + 10 { ',' } else { (b'0' + (s.frame - DIGIT_FRAME) as u8) as char }).collect();
            assert_eq!(got, want);
        }
    }

    /// A deathless, fast stage-3 clear on S2 is worth 70,000 (over a u16 in points): the tally counts it
    /// all into the score, tick by tick or skipped with Fire; the crystal goes dark in its stage's `dark`.
    #[test]
    fn the_tally_holds_a_stage_3_clear() {
        for skip in [false, true] {
            let mut w = World::new();
            let mut c = Clear::at(1, 1);
            c.stage = 2;
            c.step(&mut w, crate::boss::DYING, 0);
            assert_eq!(c.phase, DARK);
            let mut t = 0;
            while c.phase != WALK {
                let fire = skip && c.phase == CARD && c.t > CLEAR_LINES * LINE_TICKS && t % 2 == 0;
                c.step(&mut w, crate::boss::GONE, if fire { FIRE } else { 0 });
                if c.phase == DARK && c.steps == 2 {
                    assert_eq!(c.pal[3], PALETTES[2][1], "S3's dark");
                }
                t += 1;
                assert!(t < 5_000, "phase {}", c.phase);
            }
            assert_eq!((c.boss_bonus, c.clean_bonus), (BOSS_MAX2, 3 * CLEAN));
            assert_eq!(w.score, 70_000, "skip {skip}");
        }
    }

    /// From the GANTRY QUEEN's last hit to play on stage 2: the bonuses go into the score once, the
    /// palette walks to S2's, the PEREGRINE flies the start line and nobody died.
    #[test]
    fn the_stage_clears_and_promotes() {
        let mut g = Game::new();
        g.world.track_base = 1_000_000;
        g.world.scroll = (crate::boss::ARRIVE as u32 - 10) << 8;
        // Fight with the ship kept safe until she dies.
        let mut t = 0;
        while g.boss.state != crate::boss::DYING {
            g.world.player.invuln = 50;
            let x = (g.world.player.x >> 8) as i16;
            let aim = g.boss.aim[0] as i16;
            let input = FIRE | if aim > x + 4 { crate::sim::RIGHT } else if aim < x - 4 { crate::sim::LEFT } else { 0 };
            g.step(if t % 2 == 0 { input } else { input & !FIRE });
            t += 1;
            assert!(t < 20_000, "she never fell");
        }
        let deaths = g.world.deaths;
        let score = g.world.score;
        let mut phases = vec![];
        let mut pals = vec![];
        let mut out = [Sprite::default(); INSTANCES];
        let mut most_cards = 0;
        assert_eq!(g.clear.level, 0);
        for _ in 0..4000 {
            let was = g.clear.phase;
            g.step(0);
            // The level (and so, on the C64, the image: `crate::uci`) changes only as play starts.
            if g.clear.level != 0 {
                assert_eq!((was, g.clear.phase), (STAGE, OFF), "the next level from the stage's first line");
            }
            if phases.last() != Some(&g.clear.phase) {
                phases.push(g.clear.phase);
            }
            if g.clear.phase == WALK && pals.last() != Some(&g.clear.pal) {
                pals.push(g.clear.pal);
            }
            let n = crate::game::scene(&g, &mut out);
            most_cards = most_cards.max(out[..n].iter().filter(|s| s.frame >= CARD_FRAME).count());
            for s in &out[..n] {
                assert!(s.frame < crate::demo::FRAMES);
            }
            if g.clear.phase == OFF {
                break;
            }
        }
        assert_eq!(phases, [DARK, FLYOUT, CARD, WALK, STAGE, OFF]);
        assert_eq!(g.clear.stage, 1);
        assert_eq!(g.clear.level, 1, "S1 clears into S2 TERRACES");
        assert_eq!(g.clear.pal, PALETTES[1]);
        assert_eq!(g.world.state, PLAY);
        assert_eq!(g.world.deaths, deaths, "nobody died in the sequence");
        let bonus = (g.clear.boss_bonus + g.clear.clean_bonus) as u32 * 100;
        assert!(bonus >= CLEAN as u32 * 100 || deaths > 0);
        // Extends can add a full-stock bonus; none at these scores.
        assert_eq!(g.world.score, score + bonus);
        assert_eq!(g.world.scroll_lines(), 0, "stage 2 starts at its first line");
        assert_eq!(g.world.player.vx, 0, "no slide carries into the stage");
        assert!(pals.len() >= 3, "the walk takes steps: {pals:?}");
        assert!(most_cards >= CLEAR_LINES as usize + 2);
        // Play goes on as the PEREGRINE.
        for _ in 0..30 {
            g.step(crate::sim::UP);
        }
        let n = crate::game::scene(&g, &mut out);
        let pf = crate::demo::PEREGRINE_FRAME;
        assert!(out[..n].iter().any(|s| (pf..pf + crate::demo::PLAYER_BANKS).contains(&s.frame)), "the PEREGRINE flies");
    }

    /// A game over on a later level starts the new game on the booted level (the C64 loads its image back).
    #[test]
    fn a_game_over_goes_back_to_the_first_level() {
        let mut g = Game::new();
        g.reset_at(0, LAST_LEVEL);
        g.clear.stage = 1;
        g.clear.level = 1;
        g.clear.pal = PALETTES[1];
        g.campanile.state = crate::campanile::GONE;
        g.world.state = crate::sim::world::DYING;
        g.world.lives = 0;
        g.world.timer = 1;
        let mut t = 0;
        while g.world.state != PLAY {
            g.step(0);
            t += 1;
            assert!(t < 1_000);
        }
        assert_eq!((g.clear.level, g.clear.stage, g.clear.top), (0, 0, LAST_LEVEL));
    }

    /// A C64 that cannot load (`status::TOP` = its booted level, `game::replay_level`'s `top`) plays its
    /// level again after the clear; one that can goes on.
    #[test]
    fn the_next_level_waits_for_a_load() {
        let mut g = Game::new();
        g.reset_at(0, 0);
        g.clear.start_stage(&mut g.world);
        assert_eq!(g.clear.level, 0);
        g.reset_at(0, LAST_LEVEL);
        g.clear.start_stage(&mut g.world);
        assert_eq!(g.clear.level, 1);
        assert_eq!(crate::game::replay_level(1, 0, 0).clear.top, 1, "a top under the level is the level");
    }
}
