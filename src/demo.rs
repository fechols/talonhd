//! The M0/M1 tech scene: a 512×512 flying boss that sways and banks across
//! the top of the screen, a flight of spinning drones and the player's ship
//! weaving below it, over a looping vertical panorama.
//!
//! A pure function of demo time `t` with integer math only, so the host can
//! render exactly the frame the C64 reports showing.

use crate::hud;
use crate::render::Sprite;

/// Frame directory layout (see `models::demo_frames`).
pub const BOSS_BANKS: u16 = 49;
pub const DRONE_YAWS: u16 = 16;
pub const PLAYER_BANKS: u16 = 9;
pub const BOSS_FRAME: u16 = 0;
/// GANTRY QUEEN parts: arms (left, right) × aims × rolls, then legs (nw, ne, sw, se) × strides × rolls.
pub const ARM_AIMS: u16 = 5;
pub const LEG_STRIDES: u16 = 6;
pub const PART_ROLLS: u16 = 7;
pub const ARM_FRAME: u16 = BOSS_FRAME + BOSS_BANKS;
pub const LEG_FRAME: u16 = ARM_FRAME + 2 * ARM_AIMS * PART_ROLLS;
pub const DRONE_FRAME: u16 = LEG_FRAME + 4 * LEG_STRIDES * PART_ROLLS;
pub const PLAYER_FRAME: u16 = DRONE_FRAME + DRONE_YAWS;
pub const FRAMES: u16 = PLAYER_FRAME + PLAYER_BANKS;

pub const BOSS_SIZE: u16 = 512;
/// Screen line of the boss canvas centre: it hovers at the top, mostly above the screen.
pub const BOSS_CENTER_Y: i16 = -60;
/// The boss slowly sinks and rises this many lines about its centre (period 2048 frames).
pub const BOSS_DIP: i16 = 40;
/// The body's bank: 49 samples over ±24°, one degree apart.
pub const QUEEN_ROLL_MAX_DEG: i16 = 24;
/// Part origins in the body's (scaled) model space, hires px, x right, y forward, z up: the right arm and
/// the north-east leg (the others mirror x and/or y). `hand.rs` builds each part about this point.
pub const QUEEN_ARM_CENTER: [i16; 3] = [183, -50, 100];
pub const QUEEN_LEG_CENTER: [i16; 3] = [143, 78, 12];
/// Part canvases (multicolor px × lines).
pub const QUEEN_ARM_CANVAS: (u16, u16) = (112, 240);
pub const QUEEN_LEG_CANVAS: (u16, u16) = (168, 232);
pub const DRONE_W: u16 = 40;
pub const DRONE_H: u16 = 40;
pub const PLAYER_W: u16 = 36;
pub const PLAYER_H: u16 = 66;

/// Lines in one period of the looping panorama: S1 BREAKWATER, 26 screens (the image has 200 more, a
/// copy of the first 200, so every window is contiguous).
pub const PANO_PERIOD: u16 = 26 * 200;
pub const DRONES: usize = 6;
pub const INSTANCES: usize = DRONES + 2 + 6;

/// Sine of `p` (256 per turn) in -127..=127 (parabolic approximation).
#[inline(never)]
pub fn isin(p: u8) -> i16 {
    let x = (p & 127) as i32;
    let v = (x * (128 - x) * 127 / 4096) as i16;
    if p < 128 { v } else { -v }
}

pub fn icos(p: u8) -> i16 {
    isin(p.wrapping_add(64))
}

/// Fine sine: `p` in 1024 steps per turn, result -127..=127. Smooth enough that a
/// 150-pixel sway moves under a pixel per frame.
#[inline(never)]
pub fn isin_fine(p: u16) -> i16 {
    let x = (p & 511) as i32;
    let v = (x * (512 - x) * 127 / 65536) as i16;
    if p & 512 == 0 { v } else { -v }
}

pub fn icos_fine(p: u16) -> i16 {
    isin_fine(p.wrapping_add(256))
}

/// Bank frame index for a lateral velocity proxy in -127..=127: 0 is full
/// left, `banks/2` level, `banks-1` full right.
#[inline(never)]
fn bank(v: i16, banks: u16) -> u16 {
    let half = (banks / 2) as i32;
    let i = (v as i32 * half + if v >= 0 { 63 } else { -63 }) / 127;
    (half + i.clamp(-half, half)) as u16
}

/// Panorama line at the top of the screen: the view starts on the stage's first screen (the bottom of
/// the panorama) and climbs one line a frame, looping from the apron back to the sea.
pub fn scroll(t: u16) -> u16 {
    (2 * PANO_PERIOD - 200 - t % PANO_PERIOD) % PANO_PERIOD
}

/// What the HUD shows at demo time `t`: a running score, the gauge stepping,
/// the bank of the player's ship, tapes rolling with the scroll and a
/// damage flash every 400 frames.
pub fn hud_state(t: u16) -> hud::State {
    let p = t.wrapping_mul(3) as u8;
    hud::State {
        score: t as u32 * 30,
        lives: 3 - ((t / 700) % 3) as u8,
        gauge: ((t / 40) % 9) as u8,
        speed: ((t / 90) % 6) as u8,
        stage: 1,
        bank: bank(icos(p), PLAYER_BANKS) as i8 - 4,
        tape_l: t,
        tape_r: t / 2,
        since_hit: (t % 400).min(255) as u8,
    }
}

/// Sine and cosine of `deg` (|deg| <= 30) in Q14, by Taylor series: exact to a part in 10^4.
pub const fn sin_cos_deg(deg: i16) -> (i32, i32) {
    let a = deg as i32 * 286; // radians in Q14
    let a2 = a * a >> 14;
    let a3 = a2 * a >> 14;
    let a4 = a2 * a2 >> 14;
    (a - a3 / 6, 16384 - a2 / 2 + a4 / 24)
}

/// Screen offset (multicolor px, lines) of a part origin `c` from the body's canvas centre at roll
/// `deg`: the body rolls about its forward axis, so x' = x cos + z sin; y is unchanged (and points up).
pub const fn part_offset(c: [i16; 3], deg: i16) -> (i16, i16) {
    let (s, co) = sin_cos_deg(deg);
    let x = c[0] as i32 * co + c[2] as i32 * s; // hires, Q14
    (((x + (1 << 14)) >> 15) as i16, -c[1])
}

/// Part origins with their mirror signs: arms left, right; legs nw, ne, sw, se.
pub const PART_CENTERS: [[i16; 3]; 6] = {
    let (a, l) = (QUEEN_ARM_CENTER, QUEEN_LEG_CENTER);
    [
        [-a[0], a[1], a[2]],
        [a[0], a[1], a[2]],
        [-l[0], l[1], l[2]],
        [l[0], l[1], l[2]],
        [-l[0], -l[1], l[2]],
        [l[0], -l[1], l[2]],
    ]
};
pub const PART_CANVAS: [(u16, u16); 6] =
    [QUEEN_ARM_CANVAS, QUEEN_ARM_CANVAS, QUEEN_LEG_CANVAS, QUEEN_LEG_CANVAS, QUEEN_LEG_CANVAS, QUEEN_LEG_CANVAS];

/// Each part's sprite position relative to the body's canvas centre with the body level.
pub const PART_XY0: [(i16, i16); 6] = {
    let mut t = [(0i16, 0i16); 6];
    let mut i = 0;
    while i < 6 {
        let (dx, dy) = part_offset(PART_CENTERS[i], 0);
        t[i] = (dx - PART_CANVAS[i].0 as i16 / 2, dy - PART_CANVAS[i].1 as i16 / 2);
        i += 1;
    }
    t
};

/// How far each part slides sideways as the body banks, per bank sample (relative to level): evaluated at
/// compile time, so the C64 does no trigonometry. y never changes (the roll axis is y).
pub const PART_DX: [[i8; 6]; BOSS_BANKS as usize] = {
    let mut t = [[0i8; 6]; BOSS_BANKS as usize];
    let mut b = 0;
    while b < BOSS_BANKS as usize {
        let mut i = 0;
        while i < 6 {
            let d = part_offset(PART_CENTERS[i], b as i16 - QUEEN_ROLL_MAX_DEG).0 - part_offset(PART_CENTERS[i], 0).0;
            assert!(d >= -128 && d <= 127);
            t[b][i] = d as i8;
            i += 1;
        }
        b += 1;
    }
    t
};

/// First frame of each part's pose set, and its pose count.
const PART_FRAME: [u16; 6] = [
    ARM_FRAME,
    ARM_FRAME + ARM_AIMS * PART_ROLLS,
    LEG_FRAME,
    LEG_FRAME + LEG_STRIDES * PART_ROLLS,
    LEG_FRAME + 2 * LEG_STRIDES * PART_ROLLS,
    LEG_FRAME + 3 * LEG_STRIDES * PART_ROLLS,
];

/// The player's x (screen centre of the ship) at demo time `t`.
#[inline(never)]
fn player_x(t: u16) -> i16 {
    80 + isin(t.wrapping_mul(3) as u8) * 50 / 127
}

/// Part `i` of the boss with the body's canvas centre at x `bx` (bank slide applied) and line `cy`.
#[inline(never)]
fn part(i: usize, t: u16, bx: i16, cy: i16, r: u16, px: i16) -> Sprite {
    let (x0, y0) = PART_XY0[i];
    let x = bx + x0;
    let pose = if i < 2 {
        // Arms: barrels turned toward the player.
        ((px - x - QUEEN_ARM_CANVAS.0 as i16 / 2) / 40 + 2).clamp(0, ARM_AIMS as i16 - 1) as u16
    } else {
        // Legs paddle, diagonal pairs (nw + se, ne + sw) in step.
        (t / 8 + if i == 2 || i == 5 { 0 } else { LEG_STRIDES / 2 }) % LEG_STRIDES
    };
    Sprite { frame: PART_FRAME[i] + pose * PART_ROLLS + r, x, y: cy + y0, z: 0 }
}

/// Drone `i` at demo time `t`: falling through the screen on a sine path, spinning.
#[inline(never)]
fn drone(t: u16, i: u16) -> Sprite {
    let p = t.wrapping_mul(2).wrapping_add(i * 43) as u8;
    Sprite {
        frame: DRONE_FRAME + (t / 3 + i * 5) % DRONE_YAWS,
        x: 80 + isin(p) * 60 / 127 - DRONE_W as i16 / 2 + (i as i16 - 3) * 8,
        y: ((t + i * 48) % 288) as i16 - 48,
        z: 2,
    }
}

/// Fill `out` with the sprites of demo time `t`; returns how many.
#[inline(never)]
pub fn scene(t: u16, out: &mut [Sprite; INSTANCES]) -> usize {
    // The boss: sways ±150 px about the screen centre (period 1024 frames),
    // banking into each turn through 49 roll angles; bobbing a little.
    let phase = t & 1023;
    let cx = 80 + isin_fine(phase) * 150 / 127;
    let cy = BOSS_CENTER_Y + isin_fine((t >> 1) & 1023) * BOSS_DIP / 127;
    let b = bank(icos_fine(phase), BOSS_BANKS);
    // Parts (drawn under the body) take the nearest of their 7 roll samples, 8 degrees apart.
    let r = (b + 4) / 8;
    let px = player_x(t);
    let dxs = &PART_DX[b as usize];
    for i in 0..6 {
        out[i] = part(i, t, cx + dxs[i] as i16, cy, r, px);
    }
    out[6] = Sprite { frame: BOSS_FRAME + b, x: cx - BOSS_SIZE as i16 / 2, y: cy - BOSS_SIZE as i16 / 2, z: 1 };
    let mut n = 7;
    // Drones: fall through the screen on sine paths, spinning.
    for i in 0..DRONES as u16 {
        out[n] = drone(t, i);
        n += 1;
    }
    // The player: weaves along the bottom, banking with its motion.
    out[n] = Sprite {
        frame: PLAYER_FRAME + bank(icos(t.wrapping_mul(3) as u8), PLAYER_BANKS),
        x: px - PLAYER_W as i16 / 2,
        y: 122,
        z: 3,
    };
    n += 1;
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_and_bank() {
        assert_eq!(isin(0), 0);
        assert_eq!(isin(64), 127);
        assert_eq!(isin(192), -127);
        assert_eq!(bank(0, 9), 4);
        assert_eq!(bank(127, 9), 8);
        assert_eq!(bank(-127, 9), 0);
        assert_eq!(bank(127, 49), 48);
        assert_eq!(isin_fine(256), 127);
        assert_eq!(isin_fine(768), -127);
        // The sway tops out at ~1.17 px per frame: integer steps of at most 2.
        let x = |t: u16| isin_fine(t & 1023) as i32 * 150 / 127;
        assert!((0..2048u16).all(|t| (x(t + 1) - x(t)).abs() <= 2));
    }

    #[test]
    fn roll_offsets() {
        for deg in -24..=24i16 {
            let (s, c) = sin_cos_deg(deg);
            let r = (deg as f64).to_radians();
            assert!((s as f64 / 16384.0 - r.sin()).abs() < 2e-4 && (c as f64 / 16384.0 - r.cos()).abs() < 2e-4);
        }
        assert_eq!(part_offset([200, 50, 0], 0), (100, -50));
        // Rolling right lifts the right side toward the viewer and slides high parts right.
        assert!(part_offset([0, 0, 100], 24).0 > 0);
    }
}
