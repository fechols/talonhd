//! The simulation state (`World`), its construction and the host's whole-tick step.
//!
//! Plain fixed-size pools, no allocation, no pointers: the C64 keeps one `World` in resident RAM and the
//! two code phases step it in place. Field order is irrelevant to determinism; `digest` hashes the fields
//! one by one so the host (x86-64 layout) and the 6502 (packed layout) agree.
use super::*;

/// Live pools (`docs/design.md` 6.4: TWIN live cap 24 bolts = 12 pairs; `03-enemies/common-wasp.md`: at
/// most two strings of 6 on screen, with room for a third entering).
pub const PAIRS: usize = 12;
pub const WASPS: usize = 18;
pub const BURSTS: usize = 8;
pub const SPARKS: usize = 4;
pub const WAVES: usize = 4;
/// `03-enemies/s1-gannet.md`: one GANNET on screen at a time; 2 leaves room for one climbing out.
pub const GANNETS: usize = 2;
/// Enemy bullets live at once (`docs/design.md` 14 caps the game at 64; S1 needs far fewer).
pub const EBULLETS: usize = 32;

/// `World::state`.
pub const PLAY: u8 = 0;
/// The TALON was destroyed: the stage keeps scrolling and flying, no new waves spawn.
pub const DYING: u8 = 1;
/// The replacement TALON climbs in from below the screen (no control).
pub const LAUNCH: u8 = 2;
/// No ships left: the HUD shows GAME OVER, then a new game starts (M2 has no title screen).
pub const GAME_OVER: u8 = 3;

/// `Player::invuln_kind`: the arming flash (inks swap white <-> cyan every 4 ticks) or the respawn blink
/// (2 ticks on, 2 off), `ship-talon.md`.
pub const INV_NONE: u8 = 0;
pub const INV_ARM: u8 = 1;
pub const INV_RESPAWN: u8 = 2;

/// The TALON. `x`/`y` are the anchor (the 2x2 grid's centre), Q8.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    pub x: i32,
    pub y: i32,
    /// Ticks until the TWIN may fire again.
    pub fire_timer: u8,
    pub invuln: u8,
    pub invuln_kind: u8,
    /// ARM was held on the previous tick (ARM is edge-triggered).
    pub arm_held: bool,
}

/// A TWIN pair: the two bolts fired together at x -4 and +4, drawn as one 1x1 sprite (`render::game`).
/// `alive` bit 0 = left bolt, bit 1 = right bolt; `x` is the pair's centre and `y` the bolts' top, px.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Pair {
    pub alive: u8,
    pub x: i16,
    pub y: i16,
}

/// A WASP flying an authored curve (`track::CURVES`). Position Q8 (centre), `angle` in 1/256 turns.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Wasp {
    pub alive: bool,
    /// The last of its string: yellow core, drops a SPARK.
    pub leader: bool,
    /// Curve index with `track::MIRROR` (bit 7) for the left-right mirror.
    pub curve: u8,
    pub seg: u8,
    pub seg_t: u8,
    pub angle: u8,
    pub x: i32,
    pub y: i32,
}

/// A GANNET (diver, `03-enemies/s1-gannet.md`). Position Q8 (centre), vertical speed Q8 px per tick.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gannet {
    /// `actors::G_*` (0 = free).
    pub state: u8,
    pub hp: u8,
    pub t: u8,
    pub x: i32,
    pub y: i32,
    pub vy: i16,
}

/// An enemy bullet (`docs/design.md` 14). Position Q6 (1/64 px, centre) and velocity Q6 px per tick, so a
/// bullet is 6 bytes. `kind` 0 = free (`actors::B_*`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EBullet {
    pub kind: u8,
    pub x: i16,
    pub y: i16,
    pub vx: i8,
    pub vy: i8,
}

/// The placeholder 2x2 burst (explosion). `t` counts down from `BURST_TICKS`; 0 = free.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Burst {
    pub t: u8,
    pub x: i16,
    pub y: i16,
}

/// A SPARK pickup drifting down with a slow sway around `x0`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Spark {
    pub alive: bool,
    pub x0: i16,
    pub y: i32,
    pub t: u8,
}

/// A string being released from the spawn track: `left` WASPs still to come, one every `spacing` ticks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Wave {
    pub left: u8,
    pub timer: u8,
    pub record: u8,
}

/// The arming gauge (`docs/design.md` 6): cursor 0 (nothing lit) or slot 1..8; SPEED level 0..5; the main
/// weapon (always TWIN in M2).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Gauge {
    pub cursor: u8,
    pub speed: u8,
    pub main: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World {
    /// Ticks since the game started.
    pub tick: u32,
    /// Lines of stage travelled, Q8 (the scroll position; `docs/design.md` 2).
    pub scroll: u32,
    pub state: u8,
    pub timer: u16,
    pub player: Player,
    pub pairs: [Pair; PAIRS],
    pub wasps: [Wasp; WASPS],
    pub bursts: [Burst; BURSTS],
    pub sparks: [Spark; SPARKS],
    pub gannets: [Gannet; GANNETS],
    pub ebullets: [EBullet; EBULLETS],
    pub waves: [Wave; WAVES],
    /// Next spawn record and the scroll line where the current pass of the looping test track began.
    pub track_next: u8,
    pub track_base: u32,
    pub gauge: Gauge,
    pub score: u32,
    pub hi: u32,
    pub lives: u8,
    pub next_extend: u32,
    /// A wide sprite boss is on screen: the TALON's top edge is held below playfield line 88
    /// (`docs/design.md` 3). Set by boss code (none in M2) and the tests.
    pub boss_ceiling: bool,
    /// The current upper clamp of the TALON's anchor, Q8; eases toward its target over 30 ticks.
    pub ceiling: i32,
    /// Statistics for the tests and the HUD (not gameplay).
    pub kills: u16,
    pub pickups: u16,
    pub deaths: u16,
    pub arms: u16,
}

/// High score shown before any game beats it (`07-ui-text.md` hud.top: `HI 0100000`).
pub const DEFAULT_HI: u32 = 100_000;
pub const START_LIVES: u8 = 3;
pub const MAX_LIVES: u8 = 9;
/// Respawn position of the anchor (bottom centre).
pub const START_X: i32 = 152 * PX;
pub const START_Y: i32 = 140 * PX;

impl World {
    pub const fn new() -> World {
        World {
            tick: 0,
            scroll: 0,
            state: PLAY,
            timer: 0,
            player: Player { x: START_X, y: START_Y, fire_timer: 0, invuln: 0, invuln_kind: INV_NONE, arm_held: false },
            pairs: [Pair { alive: 0, x: 0, y: 0 }; PAIRS],
            wasps: [Wasp { alive: false, leader: false, curve: 0, seg: 0, seg_t: 0, angle: 0, x: 0, y: 0 }; WASPS],
            bursts: [Burst { t: 0, x: 0, y: 0 }; BURSTS],
            sparks: [Spark { alive: false, x0: 0, y: 0, t: 0 }; SPARKS],
            gannets: [Gannet { state: 0, hp: 0, t: 0, x: 0, y: 0, vy: 0 }; GANNETS],
            ebullets: [EBullet { kind: 0, x: 0, y: 0, vx: 0, vy: 0 }; EBULLETS],
            waves: [Wave { left: 0, timer: 0, record: 0 }; WAVES],
            track_next: 0,
            track_base: 0,
            gauge: Gauge { cursor: 0, speed: 0, main: gauge::TWIN },
            score: 0,
            hi: DEFAULT_HI,
            lives: START_LIVES,
            next_extend: gauge::FIRST_EXTEND,
            boss_ceiling: false,
            ceiling: player::TOP * PX,
            kills: 0,
            pickups: 0,
            deaths: 0,
            arms: 0,
        }
    }

    /// Start a new game in place, keeping the high score (the C64 calls this from the sim phase).
    #[inline(never)]
    pub fn reset(&mut self) {
        let hi = self.hi;
        // All-zero is a valid World (bools false, counters 0, pools free); a memset plus the non-zero
        // fields avoids a 400-byte constant image in the C64's resident read-only data.
        // SAFETY: every field is an integer or a bool, for which zero is a valid value.
        unsafe { core::ptr::write_bytes(self as *mut World, 0, 1) };
        self.player.x = START_X;
        self.player.y = START_Y;
        self.gauge.main = gauge::TWIN;
        self.hi = if hi == 0 { DEFAULT_HI } else { hi };
        self.lives = START_LIVES;
        self.next_extend = gauge::FIRST_EXTEND;
        self.ceiling = player::TOP * PX;
    }

    /// One whole tick: the world half, then the player half (the order the C64's phases run in).
    pub fn step(&mut self, input: u8) {
        self.step_world();
        self.step_sim(input);
    }

    /// Whole scroll lines travelled.
    pub fn scroll_lines(&self) -> u32 { self.scroll >> 8 }
}

impl Default for World {
    fn default() -> Self { World::new() }
}
