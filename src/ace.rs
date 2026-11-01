//! ACE, the attract mode's pilot (user: "make the player look like an expert pilot even during attract
//! mode"): a small goal-oriented action planner (GOAP) over a dodging motor. It runs every tick from the
//! paged ATTRACT phase while nobody plays (`paging::ATTRACT`, out of the window during real play), on the
//! C64 and in the host's replay alike, so `compare` stays exact. Its memory (`Ace`) lives in the `Game`.
//!
//! - **Perception:** facts (bits) read from the `Game`: threats on a collision course, a GANNET diving
//!   overhead, SPARKs, the gauge, the best target and whether the guns are lined up on it.
//! - **Goals,** first unmet wins: stay safe, take an upgrade on offer, collect SPARKs, destroy the
//!   target, hold station (low and centred).
//! - **Planner:** a forward search over the actions' preconditions and effects (up to 3 steps), the
//!   cheapest plan that meets the goal. Replanned when the goal changes, a step's precondition breaks, or
//!   every 16 ticks.
//! - **Motor:** the plan's step names a point (and the ARM bit); each of the 9 stick moves is scored over
//!   a 24-tick lookahead against every nearby threat's predicted path (bullets and WASPs in straight
//!   lines, GANNETs by their dive), plus progress toward the point and a cost for changing direction,
//!   so the ship threads fans and banks smoothly. It fires only while an enemy is on screen.
//!
//! Three paged phases (one 4 KiB window would not hold it), **time-sliced, one per tick** (user: 60 FPS
//! is a hard limit; one overlay DMA a frame): tick n `see` (ATTRACT: threats, SPARKs, the target), n+1
//! `motor` (MOTOR: the lookahead for all 9 moves), n+2 `think` (ACE: facts, goal, plan, the stick). The
//! stick is held between decisions (a new one every 3 ticks, 50 ms). They share a `Mind`: the C64 keeps
//! it in `SEG_BUF` during the tick and parks it in the REU (`MIND_REU`) between ticks; the host keeps it
//! beside the `Game` (`fly`).
//!
//! 6502: 16-bit math only (the sim's Q8 positions are read once with
//! `>> 8` or `>> 2`), no multiplies (positions are accumulated), threats in parallel arrays on the stack,
//! small out-of-line functions so nothing is inlined twice, no iterator closures (their generic helpers
//! would land in resident RAM).

use crate::boss::NO_AIM;
use crate::game::Game;
use crate::ground;
use crate::sim::actors::{B_BOMB, G_CLIMB, G_DIVE, G_PULLUP, G_TELEGRAPH};
use crate::sim::collide::{GANNET_HALF_H, GANNET_HALF_W, HITBOX_DY, HITBOX_HALF, WASP_HALF_H, WASP_HALF_W};
use crate::sim::common::sway;
use crate::sim::gauge;
use crate::sim::player::{BOTTOM, LEFT_X, RIGHT_X, SPEED_DIAGONAL_Q8, SPEED_Q8};
use crate::sim::track::{CURVES, MIRROR, WASP_VX, WASP_VY};
use crate::sim::world::*;
use crate::sim::{ARM, DOWN, FIELD_H, FIELD_W, FIRE, LEFT, RIGHT, UP};

// Facts.
const SAFE: u16 = 1;
const THREAT: u16 = 2;
const DIVING: u16 = 4;
const SPARK: u16 = 8;
const NOSPARK: u16 = 16;
const CURSOR: u16 = 32;
const ARMED: u16 = 64;
const TARGET: u16 = 128;
const ALIGNED: u16 = 256;
const CLEAR: u16 = 512;
const STATION: u16 = 1024;

// Actions, indices into `PRE`, `EFF`, `COST`.
const HOLD: u8 = 0;
const EVADE: u8 = 1;
const FLANK: u8 = 2;
const TAKE: u8 = 3;
const ARM_IT: u8 = 4;
const LINE_UP: u8 = 5;
const STRAFE: u8 = 6;
const ACTIONS: u8 = 7;
static PRE: [u16; 7] = [0, THREAT, DIVING, SPARK, CURSOR, TARGET, ALIGNED];
static EFF: [u16; 7] = [STATION, SAFE, SAFE, CURSOR | NOSPARK, ARMED, ALIGNED, CLEAR];
static COST: [u8; 7] = [1, 3, 1, 2, 1, 2, 1];

// Target kinds (`Ace::target` high bits).
const T_NONE: u8 = 0;
const T_WASP: u8 = 1;
const T_GANNET: u8 = 2;
const T_UNIT: u8 = 3;
const T_BOSS: u8 = 4;
const T_KEEL: u8 = 5;

/// Lookahead: `SAMPLES` samples 2 ticks apart; a move is held `HELD` ticks, then the ship stops.
const SAMPLES: usize = 12;
const HELD: u8 = 4;
/// Threats considered at once (the nearest; on the stack).
const MAX_THREATS: usize = 16;
/// Clearance (Q6) under which a near miss starts to cost, and under which the pilot calls a threat.
const MARGIN: i16 = 6 * 64;
/// Where the pilot waits: low and centred.
const HOME_X: i16 = 152;
const HOME_Y: i16 = 140;
/// Stick moves.
static MX: [i8; 9] = [0, 0, 1, 1, 1, 0, -1, -1, -1];
static MY: [i8; 9] = [0, -1, -1, 0, 1, 1, 1, 0, -1];
const NO_SPARK: u8 = 0xFF;

/// The pilot's memory (in the `Game`, so the replay carries it).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Ace {
    goal: u16,
    plan: [u8; 3],
    len: u8,
    step: u8,
    replan: u8,
    /// Target kind << 5 | index.
    target: u8,
    /// The stick move held (`MX`/`MY` index) and for how many decisions.
    held: u8,
    hold: u8,
    /// The slice due this tick (0 `see`, 1 `motor`, 2 `think`) and the stick held between decisions.
    slice: u8,
    stick: u8,
}

impl Ace {
    pub const fn new() -> Ace {
        Ace { goal: 0, plan: [0; 3], len: 0, step: 0, replan: 0, target: 0, held: 0, hold: 0, slice: 0, stick: 0 }
    }
    /// The slice due this tick: 0 `see`, 1 `motor`, 2 `think`.
    #[inline(always)]
    pub fn slice(&self) -> u8 {
        self.slice
    }
    /// The stick between decisions (ARM is a press, so it is only sent on the deciding tick).
    #[inline(always)]
    pub fn held(&self) -> u8 {
        self.stick & !ARM
    }
}

/// Where the C64 parks the `Mind` between ticks: the REU just below the music ring (`audio::RING`); level
/// images end below it (`bake`).
pub const MIND_REU: u32 = 0xFE_7000;
/// The `Mind`'s size in bytes (what the C64 parks).
pub const MIND_BYTES: u16 = core::mem::size_of::<Mind>() as u16;

/// What the phases hand on within a tick: `see` fills the threats, SPARK and target, `motor` each
/// move's clearance, end point and stick bits. Plain integers only (on the C64 it is scratch RAM that
/// `see` initialises).
pub struct Mind {
    threats: Threats,
    diving: u8,
    spark: u8,
    has_aim: u8,
    aim: Aim,
    clear: [i16; 9],
    ex: [i16; 9],
    ey: [i16; 9],
    bits: [u8; 9],
}

// The C64 keeps the Mind in the renderer's scratch buffer.
const _: () = assert!(core::mem::size_of::<Mind>() <= crate::layout::SEG_BUF_LEN as usize);

impl Mind {
    pub const fn new() -> Mind {
        Mind {
            threats: Threats { n: 0, x: [0; MAX_THREATS], y: [0; MAX_THREATS], vx: [0; MAX_THREATS], vy: [0; MAX_THREATS], rw: [0; MAX_THREATS], rh: [0; MAX_THREATS] },
            diving: 0,
            spark: NO_SPARK,
            has_aim: 0,
            aim: Aim { x: 0, y: 0, vx: 0 },
            clear: [0; 9],
            ex: [0; 9],
            ey: [0; 9],
            bits: [0; 9],
        }
    }
}

/// Nearby threats, Q6 positions and per-tick velocities, half extents plus the hitbox's.
struct Threats {
    n: usize,
    x: [i16; MAX_THREATS],
    y: [i16; MAX_THREATS],
    vx: [i16; MAX_THREATS],
    vy: [i16; MAX_THREATS],
    rw: [i16; MAX_THREATS],
    rh: [i16; MAX_THREATS],
}

/// The ship as the motor sees it: hitbox centre (Q6), straight and diagonal speed (Q6/tick), clamps.
struct Ship {
    x: i16,
    y: i16,
    v: i16,
    d: i16,
    top: i16,
}

/// What the pilot shoots at: playfield centre, x velocity (Q8 px/tick).
#[derive(Clone, Copy)]
struct Aim {
    x: i16,
    y: i16,
    vx: i16,
}

#[inline(never)]
fn add(t: &mut Threats, x: i16, y: i16, vx: i16, vy: i16, rw: i16, rh: i16) {
    if t.n < MAX_THREATS {
        let i = t.n;
        t.x[i] = x;
        t.y[i] = y;
        t.vx[i] = vx;
        t.vy[i] = vy;
        t.rw[i] = rw;
        t.rh[i] = rh;
        t.n = i + 1;
    }
}

#[inline(always)]
fn near(x: i16, y: i16, px: i16, py: i16) -> bool {
    (x - px).abs() < 112 && y > py - 160 && y < py + 48
}

/// The threats near the ship (`half`: the hitbox's half size, Q6); true if a GANNET dives on it.
#[inline(never)]
fn gather(g: &Game, px: i16, py: i16, half: i16, t: &mut Threats) -> bool {
    let w = &g.world;
    let mut i = 0;
    while i < EBULLETS {
        let b = &w.ebullets[i];
        if b.kind != 0 && near(b.x >> 6, b.y >> 6, px, py) {
            let h = if b.kind == B_BOMB { 128 } else { 64 } + half;
            add(t, b.x, b.y, b.vx as i16, b.vy as i16, h, h);
        }
        i += 1;
    }
    let mut i = 0;
    while i < WASPS {
        let k = &w.wasps[i];
        if k.alive && near((k.x >> 8) as i16, (k.y >> 8) as i16, px, py) {
            // WASPs fly a known turn program (`sim::track::CURVES`): predict along the heading they will
            // have halfway through the lookahead (the turn so far in this segment, up to 12 ticks), and
            // widen the box while they are still turning.
            let curve = &CURVES[(k.curve & !MIRROR) as usize];
            let mut angle = k.angle;
            let mut grow = 0;
            if (k.seg as usize) < curve.segments.len() {
                let (ticks, turn) = curve.segments[k.seg as usize];
                if ticks != 0 && turn != 0 {
                    let turn = if k.curve & MIRROR != 0 { turn.wrapping_neg() } else { turn } as u8;
                    let mut n = ticks - k.seg_t;
                    if n > 12 {
                        n = 12;
                    }
                    while n != 0 {
                        angle = angle.wrapping_add(turn);
                        n -= 1;
                    }
                    grow = 4 << 6;
                }
            }
            let h = (angle.wrapping_add(8) >> 4) as usize;
            add(t, (k.x >> 2) as i16, (k.y >> 2) as i16, WASP_VX[h] >> 2, WASP_VY[h] >> 2, (WASP_HALF_W << 6) + half + grow, (WASP_HALF_H << 6) + half + grow);
        }
        i += 1;
    }
    // A HATCH launches WASPs from its doors: never loiter over one (it scrolls down with the ground).
    let mut i = 0;
    while i < ground::UNITS {
        let u = &g.ground.units[i];
        if u.kind == ground::HATCH && u.state == ground::LIVE && near(u.px, u.py, px, py) {
            add(t, u.px << 6, u.py << 6, 0, 32, (WASP_HALF_W << 6) + half, (WASP_HALF_H << 6) + half);
        }
        i += 1;
    }
    let mut diving = false;
    let mut i = 0;
    while i < GANNETS {
        let k = &w.gannets[i];
        let kx = (k.x >> 8) as i16;
        if k.state != 0 && near(kx, (k.y >> 8) as i16, px, py) {
            add(t, (k.x >> 2) as i16, (k.y >> 2) as i16, 0, k.vy >> 2, (GANNET_HALF_W << 6) + half, (GANNET_HALF_H << 6) + half);
            if (k.state == G_DIVE || k.state == G_TELEGRAPH) && (kx - px).abs() < 48 {
                diving = true;
            }
        }
        i += 1;
    }
    diving
}

/// Hold move `m` for `HELD` ticks (then stop) over the lookahead: where the hitbox ends up (Q6, in
/// `end`) and its worst clearance from every threat (Q6; negative = a hit, the earlier the more
/// negative).
#[inline(never)]
fn assess(t: &Threats, s: &Ship, m: usize, end: &mut (i16, i16)) -> i16 {
    let sp = if MX[m] != 0 && MY[m] != 0 { s.d } else { s.v };
    let vx = if MX[m] < 0 { -sp } else if MX[m] > 0 { sp } else { 0 };
    let vy = if MY[m] < 0 { -sp } else if MY[m] > 0 { sp } else { 0 };
    let (x0, x1) = ((LEFT_X as i16) << 6, (RIGHT_X as i16) << 6);
    let (y0, y1) = (s.top + (HITBOX_DY << 6), (BOTTOM as i16 + HITBOX_DY) << 6);
    // The hitbox every 2 ticks.
    let mut px = [0i16; SAMPLES];
    let mut py = [0i16; SAMPLES];
    let (mut x, mut y) = (s.x, s.y);
    let mut tick = 0u8;
    let mut i = 0;
    while i < SAMPLES {
        if tick < HELD {
            x = (x + vx + vx).clamp(x0, x1);
            y = (y + vy + vy).clamp(y0, y1);
        }
        tick += 2;
        px[i] = x;
        py[i] = y;
        i += 1;
    }
    *end = (x, y);
    let mut worst = i16::MAX;
    let mut k = 0;
    while k < t.n {
        let (mut tx, mut ty) = (t.x[k], t.y[k]);
        let (sx, sy) = (t.vx[k] + t.vx[k], t.vy[k] + t.vy[k]);
        let (rw, rh) = (t.rw[k], t.rh[k]);
        let mut hit = -4000i16;
        let mut i = 0;
        while i < SAMPLES {
            tx = tx.wrapping_add(sx);
            ty = ty.wrapping_add(sy);
            let cx = (tx - px[i]).abs() - rw;
            let cy = (ty - py[i]).abs() - rh;
            let c = if cx > cy { cx } else { cy };
            let c = if c < 0 { hit } else { c };
            if c < worst {
                worst = c;
            }
            hit += 300;
            i += 1;
        }
        k += 1;
    }
    worst
}

/// The cheapest sequence of up to 3 actions whose effects reach `goal` from `facts`.
#[inline(never)]
fn plan(a: &mut Ace, facts: u16, goal: u16) {
    a.goal = goal;
    a.step = 0;
    a.len = 0;
    a.replan = 16;
    let mut best = u8::MAX;
    let mut i = 0;
    while i < ACTIONS {
        let f0 = facts | EFF[i as usize];
        if facts & PRE[i as usize] == PRE[i as usize] {
            let c0 = COST[i as usize];
            if f0 & goal == goal {
                if c0 < best {
                    best = c0;
                    a.plan = [i, 0, 0];
                    a.len = 1;
                }
            } else {
                let mut j = 0;
                while j < ACTIONS {
                    let f1 = f0 | EFF[j as usize];
                    let c1 = c0 + COST[j as usize];
                    if f0 & PRE[j as usize] == PRE[j as usize] {
                        if f1 & goal == goal {
                            if c1 < best {
                                best = c1;
                                a.plan = [i, j, 0];
                                a.len = 2;
                            }
                        } else {
                            let mut k = 0;
                            while k < ACTIONS {
                                let c2 = c1 + COST[k as usize];
                                if f1 & PRE[k as usize] == PRE[k as usize] && (f1 | EFF[k as usize]) & goal == goal && c2 < best {
                                    best = c2;
                                    a.plan = [i, j, k];
                                    a.len = 3;
                                }
                                k += 1;
                            }
                        }
                    }
                    j += 1;
                }
            }
        }
        i += 1;
    }
}

/// Target `kind`/`i` if it can be shot now (in `out`) and its score (higher is better), else `i16::MIN`.
#[inline(never)]
fn rate(g: &Game, kind: u8, i: usize, px: i16, py: i16, out: &mut Aim) -> i16 {
    let w = &g.world;
    let (x, y, vx, bonus) = match kind {
        T_WASP => {
            let k = &w.wasps[i];
            if !k.alive {
                return i16::MIN;
            }
            let h = (k.angle.wrapping_add(8) >> 4) as usize;
            ((k.x >> 8) as i16, (k.y >> 8) as i16, WASP_VX[h], 0)
        }
        T_GANNET => {
            let k = &w.gannets[i];
            if k.state != G_PULLUP && k.state != G_CLIMB {
                return i16::MIN;
            }
            ((k.x >> 8) as i16, (k.y >> 8) as i16, 0, 80)
        }
        T_UNIT => {
            let u = &g.ground.units[i];
            if u.state != ground::LIVE {
                return i16::MIN;
            }
            (u.px, u.py, 0, 30)
        }
        T_BOSS => {
            let b = &g.boss;
            if (b.state != crate::boss::PHASE1 && b.state != crate::boss::PHASE2) || b.aim[1] == NO_AIM {
                return i16::MIN;
            }
            (b.aim[0], b.aim[1], 0, 400)
        }
        T_KEEL => {
            let b = &g.midboss;
            if (b.state != crate::midboss::PHASE1 && b.state != crate::midboss::PHASE2) || b.aim[1] == NO_AIM {
                return i16::MIN;
            }
            (b.aim[0], b.aim[1], 0, 400)
        }
        _ => return i16::MIN,
    };
    if y <= 4 || y >= py - 24 || x < 8 || x >= FIELD_W - 8 {
        return i16::MIN;
    }
    *out = Aim { x, y, vx };
    y - ((x - px).abs() >> 1) + bonus
}

/// Whether any enemy is on screen: a WASP or GANNET in the playfield, a live ground unit in view, or a
/// boss fight.
#[inline(never)]
fn enemies_on_screen(g: &Game) -> bool {
    let w = &g.world;
    let mut i = 0;
    while i < WASPS {
        let k = &w.wasps[i];
        let (x, y) = ((k.x >> 8) as i16, (k.y >> 8) as i16);
        if k.alive && x >= 0 && x < FIELD_W && y >= 0 && y < FIELD_H {
            return true;
        }
        i += 1;
    }
    let mut i = 0;
    while i < GANNETS {
        let k = &w.gannets[i];
        let y = (k.y >> 8) as i16;
        if k.state != 0 && y >= 0 && y < FIELD_H {
            return true;
        }
        i += 1;
    }
    let mut i = 0;
    while i < ground::UNITS {
        let u = &g.ground.units[i];
        if u.state == ground::LIVE && u.py >= 0 && u.py < FIELD_H {
            return true;
        }
        i += 1;
    }
    let (b, k) = (g.boss.state, g.midboss.state);
    b == crate::boss::PHASE1 || b == crate::boss::PHASE2 || k == crate::midboss::PHASE1 || k == crate::midboss::PHASE2
}

/// The target worth shooting now, kept while it stays good: a boss's weak part first, then the lowest
/// enemy above the ship, nearer columns preferred. False if there is none.
#[inline(never)]
fn choose(g: &mut Game, px: i16, py: i16, aim: &mut Aim) -> bool {
    let mut best = i16::MIN;
    let mut id = 0u8;
    let mut a = Aim { x: 0, y: 0, vx: 0 };
    // (kind, count) to scan.
    static SCAN: [(u8, u8); 5] = [(T_BOSS, 1), (T_KEEL, 1), (T_WASP, WASPS as u8), (T_GANNET, GANNETS as u8), (T_UNIT, ground::UNITS as u8)];
    let mut s = 0;
    while s < SCAN.len() {
        let (kind, count) = SCAN[s];
        let mut i = 0;
        while i < count {
            let v = rate(g, kind, i as usize, px, py, &mut a);
            if v > best {
                best = v;
                id = kind << 5 | i;
                *aim = a;
            }
            i += 1;
        }
        s += 1;
    }
    // Keep the current target unless the best is clearly better (no dithering between two).
    let held = g.ace.target;
    if held >> 5 != T_NONE && held != id {
        let v = rate(g, held >> 5, (held & 31) as usize, px, py, &mut a);
        if v != i16::MIN && best < v + 40 {
            best = v;
            id = held;
            *aim = a;
        }
    }
    g.ace.target = if best == i16::MIN { 0 } else { id };
    best != i16::MIN
}

/// The stick for this tick: this tick's slice, then the stick (host; the C64 pages the slice's phase
/// and parks `m` in the REU between ticks). `m` must persist from tick to tick.
pub fn fly(g: &mut Game, m: &mut Mind) -> u8 {
    match g.ace.slice {
        0 => see(g, m),
        1 => motor(g, m),
        _ => think(g, m),
    }
}

/// Off duty (dying, launching, game over): start the next decision afresh, fire held.
#[inline(always)]
fn idle(a: &mut Ace) {
    a.len = 0;
    a.slice = 0;
    a.stick = 0;
}

/// ATTRACT phase: the threats near the ship, the SPARK to take and the target, into `m`; the held stick.
#[inline(never)]
pub fn see(g: &mut Game, m: &mut Mind) -> u8 {
    m.threats.n = 0;
    m.diving = 0;
    m.spark = NO_SPARK;
    m.has_aim = 0;
    if g.world.state != PLAY {
        idle(&mut g.ace);
        return 0;
    }
    g.ace.slice = 1;
    let px = (g.world.player.x >> 8) as i16;
    let py = (g.world.player.y >> 8) as i16;
    let half = HITBOX_HALF[g.world.hitbox_step()] << 6;
    m.diving = gather(g, px, py, half, &mut m.threats) as u8;
    let mut i = 0;
    while i < SPARKS {
        let s = &g.world.sparks[i];
        let sy = (s.y >> 8) as i16;
        if s.alive && sy > 40 && sy < BOTTOM as i16 && m.spark == NO_SPARK {
            m.spark = i as u8;
        }
        i += 1;
    }
    m.has_aim = choose(g, px, py, &mut m.aim) as u8;
    g.ace.held()
}

/// MOTOR phase: every stick move's lookahead (clearance, end point) and its stick bits, into `m`; the
/// held stick.
#[inline(never)]
pub fn motor(g: &mut Game, m: &mut Mind) -> u8 {
    if g.world.state != PLAY {
        idle(&mut g.ace);
        return 0;
    }
    g.ace.slice = 2;
    let w = &g.world;
    let px = (w.player.x >> 8) as i16;
    let py = (w.player.y >> 8) as i16;
    // The threats were seen a tick ago: bring them up to now, and keep only those whose whole path over
    // the lookahead comes within `MARGIN` of anywhere the ship can reach (most bullets never do; the
    // lookahead's cost is per threat).
    let (sx, sy) = (px << 6, (py + HITBOX_DY) << 6);
    let reach = (SPEED_Q8[w.gauge.speed as usize] >> 2) as i16 * HELD as i16 + MARGIN;
    let t = &mut m.threats;
    let (mut k, mut kept) = (0, 0);
    while k < t.n {
        let (x, y, vx, vy) = (t.x[k].wrapping_add(t.vx[k]), t.y[k].wrapping_add(t.vy[k]), t.vx[k], t.vy[k]);
        // Where it is at the end of the lookahead (24 ticks: x + 16 vx + 8 vx).
        let (x1, y1) = (x + (vx << 4) + (vx << 3), y + (vy << 4) + (vy << 3));
        let (lx, hx) = if x1 < x { (x1, x) } else { (x, x1) };
        let (ly, hy) = if y1 < y { (y1, y) } else { (y, y1) };
        let (rw, rh) = (t.rw[k] + reach, t.rh[k] + reach);
        if lx - rw < sx && hx + rw > sx && ly - rh < sy && hy + rh > sy {
            t.x[kept] = x;
            t.y[kept] = y;
            t.vx[kept] = vx;
            t.vy[kept] = vy;
            t.rw[kept] = t.rw[k];
            t.rh[kept] = t.rh[k];
            kept += 1;
        }
        k += 1;
    }
    t.n = kept;
    let speed = w.gauge.speed as usize;
    let ship = Ship {
        x: px << 6,
        y: (py + HITBOX_DY) << 6,
        v: (SPEED_Q8[speed] >> 2) as i16,
        d: (SPEED_DIAGONAL_Q8[speed] >> 2) as i16,
        top: (w.ceiling >> 2) as i16,
    };
    let mut end = (0i16, 0i16);
    let mut mv = 0;
    while mv < 9 {
        m.clear[mv] = assess(&m.threats, &ship, mv, &mut end);
        m.ex[mv] = end.0;
        m.ey[mv] = end.1;
        let mut b = 0;
        if MX[mv] < 0 {
            b |= LEFT;
        } else if MX[mv] > 0 {
            b |= RIGHT;
        }
        if MY[mv] < 0 {
            b |= UP;
        } else if MY[mv] > 0 {
            b |= DOWN;
        }
        m.bits[mv] = b;
        mv += 1;
    }
    g.ace.held()
}

/// ACE phase: facts, goal and plan, then the stick from the lookahead.
#[inline(never)]
pub fn think(g: &mut Game, m: &mut Mind) -> u8 {
    if g.world.state != PLAY {
        idle(&mut g.ace);
        return 0;
    }
    let px = (g.world.player.x >> 8) as i16;
    let py = (g.world.player.y >> 8) as i16;
    let top = (g.world.ceiling >> 2) as i16;

    // Perception.
    let mut facts = 0u16;
    if m.clear[0] < MARGIN {
        facts |= THREAT;
    }
    if m.diving != 0 {
        facts |= DIVING;
    }
    if facts & (THREAT | DIVING) == 0 {
        facts |= SAFE;
    }
    let spark = m.spark;
    facts |= if spark != NO_SPARK { SPARK } else { NOSPARK };
    let gauge = g.world.gauge;
    let wanted = |slot: u8| slot == gauge::TWIN || slot == gauge::SPEED && gauge.speed < 3;
    let next = if gauge.cursor >= gauge::SLOTS { 1 } else { gauge.cursor + 1 };
    if gauge.cursor != 0 && wanted(gauge.cursor) {
        facts |= CURSOR;
    } else if !(spark != NO_SPARK && wanted(next)) {
        facts |= ARMED;
    }
    let aim = m.aim;
    let mut lead = 0;
    if m.has_aim != 0 {
        facts |= TARGET;
        // Bolts climb 8 px a tick: lead the target by its drift over the bolts' flight.
        // (+2: the target was seen two ticks ago.)
        let mut t = ((py - aim.y) >> 3) + 2;
        let mut drift = 0i16;
        while t > 0 {
            drift += aim.vx;
            t -= 1;
        }
        lead = aim.x + (drift >> 8);
        if (lead - px).abs() <= 4 {
            facts |= ALIGNED;
        }
    } else {
        facts |= CLEAR;
    }
    if (px - HOME_X).abs() < 8 && (py - HOME_Y).abs() < 8 {
        facts |= STATION;
    }

    // Goal: the first one not met; plan for it when the plan is spent, broken or old.
    let goal = if facts & SAFE == 0 {
        SAFE
    } else if facts & ARMED == 0 {
        ARMED
    } else if facts & SPARK != 0 {
        NOSPARK
    } else if facts & TARGET != 0 {
        CLEAR
    } else {
        STATION
    };
    let a = &mut g.ace;
    let mut stale = a.step >= a.len || goal != a.goal || a.replan == 0;
    if !stale {
        let act = a.plan[a.step as usize] as usize;
        if facts & EFF[act] == EFF[act] {
            a.step += 1;
            stale = a.step >= a.len;
        } else if facts & PRE[act] != PRE[act] {
            stale = true;
        }
    }
    if stale {
        plan(a, facts, goal);
    }
    a.replan = a.replan.wrapping_sub(1);
    let action = if a.step < a.len { a.plan[a.step as usize] } else { HOLD };

    // The action's point.
    let w = &g.world;
    // Fire only with an enemy on screen: an expert doesn't spray an empty sky.
    let mut input = if enemies_on_screen(g) { FIRE } else { 0 };
    let (mut gx, mut gy) = (HOME_X, HOME_Y);
    let mut pull = 2;
    if action == EVADE {
        (gx, gy) = (px, 150);
        pull = 1;
    } else if action == FLANK {
        let mut i = 0;
        while i < GANNETS {
            let k = &w.gannets[i];
            if k.state == G_DIVE || k.state == G_TELEGRAPH {
                let kx = (k.x >> 8) as i16;
                gx = if kx > FIELD_W / 2 { kx - 56 } else { kx + 56 };
                gy = 140;
            }
            i += 1;
        }
    } else if action == TAKE && spark != NO_SPARK {
        let s = &w.sparks[spark as usize];
        gx = s.x0 + sway(s.t);
        gy = ((s.y >> 8) as i16).clamp(96, 150);
    } else if action == ARM_IT {
        (gx, gy) = (px, py);
    } else if (action == LINE_UP || action == STRAFE) && facts & TARGET != 0 {
        gx = lead;
        // Stay well below it, rising to snipe what sits high.
        gy = (aim.y + 72).clamp((top >> 6) + 4, 150);
        pull = 3;
    }
    // An upgrade on the cursor is always taken (it costs nothing).
    if facts & CURSOR != 0 && w.tick & 1 == 0 {
        input |= ARM;
    }

    // Motor: the best of the 9 moves.
    let a = &mut g.ace;
    let (mut best, mut pick) = (i16::MIN, 0u8);
    let mut mv = 0u8;
    while mv < 9 {
        let clear = m.clear[mv as usize];
        let end = (m.ex[mv as usize], m.ey[mv as usize]);
        let mut score: i16 = 0;
        if clear < 0 {
            // A hit: the later the better, never preferred to a miss.
            score = -16000 - clear.max(-4000);
        } else if clear < MARGIN {
            score = -((MARGIN - clear) >> 2);
        }
        let dist = (((end.0 >> 6) - gx).abs() + ((end.1 >> 6) - HITBOX_DY - gy).abs()).min(400);
        let mut k = 0;
        while k < pull {
            score -= dist;
            k += 1;
        }
        if mv != a.held {
            score -= if a.hold < 3 { 24 } else { 8 };
            // Reversing either axis costs more.
            let (h, n) = (m.bits[a.held as usize], m.bits[mv as usize]);
            if h & LEFT != 0 && n & RIGHT != 0 || h & RIGHT != 0 && n & LEFT != 0 || h & UP != 0 && n & DOWN != 0 || h & DOWN != 0 && n & UP != 0 {
                score -= 24;
            }
        }
        if score > best {
            best = score;
            pick = mv;
        }
        mv += 1;
    }
    if pick == a.held {
        a.hold = a.hold.saturating_add(1);
    } else {
        a.held = pick;
        a.hold = 0;
    }
    input |= m.bits[pick as usize];
    a.slice = 0;
    a.stick = input;
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::replay::autopilot;

    /// Fly a new game for `ticks`: (deaths, score, kills, the tick of each death, the KEELBREAKER sunk,
    /// the GANTRY QUEEN beaten).
    fn flight(ticks: u32, ace: bool) -> (u16, u32, u16, Vec<u32>, bool, bool) {
        let mut g = Game::new();
        let mut m = Mind::new();
        let mut deaths = vec![];
        let (mut keel, mut queen) = (false, false);
        let mut was = g.world.state;
        for t in 0..ticks {
            let input = if ace { fly(&mut g, &mut m) } else { autopilot(&g.world) };
            g.step(input);
            if g.world.state == DYING && was != DYING {
                deaths.push(t);
            }
            was = g.world.state;
            keel |= g.midboss.state == crate::midboss::SINKING;
            queen |= g.boss.state == crate::boss::DYING;
        }
        (deaths.len() as u16, g.world.score, g.world.kills, deaths, keel, queen)
    }

    #[test]
    fn the_ace_outflies_the_autopilot() {
        let ticks = 16_000;
        let old = flight(ticks, false);
        let new = flight(ticks, true);
        println!("autopilot: deaths {} score {} kills {} at {:?} keel {} queen {}", old.0, old.1, old.2, old.3, old.4, old.5);
        println!("ace:       deaths {} score {} kills {} at {:?} keel {} queen {}", new.0, new.1, new.2, new.3, new.4, new.5);
        assert!(new.0 <= 1, "the ace died {} times (ticks {:?})", new.0, new.3);
        assert!(new.4, "the ace sinks the KEELBREAKER");
        assert!(new.5, "the ace beats the GANTRY QUEEN");
        assert!(new.1 > old.1, "the ace outscores the autopilot");
    }

    #[test]
    fn the_ace_flies_smoothly() {
        let mut g = Game::new();
        let mut mind = Mind::new();
        let (mut changes, mut flips, mut last) = (0u32, 0u32, (0i8, 0i8));
        let ticks = 6_000;
        for _ in 0..ticks {
            let input = fly(&mut g, &mut mind);
            g.step(input);
            let m = (
                if input & LEFT != 0 { -1 } else if input & RIGHT != 0 { 1 } else { 0 },
                if input & UP != 0 { -1 } else if input & DOWN != 0 { 1 } else { 0 },
            );
            if m != last {
                changes += 1;
                if m.0 * last.0 < 0 {
                    flips += 1;
                }
            }
            last = m;
        }
        let per_s = changes as f64 * 60.0 / ticks as f64;
        println!("stick changes {per_s:.1}/s, left-right flips {flips}");
        assert!(per_s < 6.0, "{per_s:.1} stick changes a second: twitchy");
    }
}
