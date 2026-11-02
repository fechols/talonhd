//! The level image's frame directory and the GANTRY QUEEN's flight: a 512×512 body that sways and banks
//! across the top of the screen with its arms and legs as separate sprites. Integer math only, so the host
//! renders exactly the frame the C64 reports showing. The game scene (`crate::game`) places everything.
//!
//! On the C64 the boss code lives in `.hitext` (RAM under the KERNAL, copied there at boot).

use crate::render::Sprite;

/// Frame directory layout (see `models::demo_frames`).
pub const BOSS_BANKS: u16 = 49;
pub const DRONE_YAWS: u16 = 16;
pub const PLAYER_BANKS: u16 = 9;
pub const BOSS_FRAME: u16 = 0;
/// GANTRY QUEEN parts: arms (left, right), then legs (nw, ne, sw, se), each one frame per body bank
/// sample: posed for that bank (barrels lean into the turn, legs trail it) at the body's own roll.
pub const ARM_FRAME: u16 = BOSS_FRAME + BOSS_BANKS;
pub const LEG_FRAME: u16 = ARM_FRAME + 2 * BOSS_BANKS;
pub const DRONE_FRAME: u16 = LEG_FRAME + 4 * BOSS_BANKS;
pub const PLAYER_FRAME: u16 = DRONE_FRAME + DRONE_YAWS;
/// GANNET: nose down the screen at 5 pitches (+50° dive .. -50°), then nose up at the same 5.
pub const GANNET_PITCHES: u16 = 5;
pub const GANNET_FRAME: u16 = PLAYER_FRAME + PLAYER_BANKS;
/// TWIN pairs (left bolt only, right only, both), enemy round, GANNET bomb, 3 explosion frames, 4 SPARK
/// spin frames.
pub const PAIR_FRAME: u16 = GANNET_FRAME + 2 * GANNET_PITCHES;
pub const ROUND_FRAME: u16 = PAIR_FRAME + 3;
pub const BOMB_FRAME: u16 = ROUND_FRAME + 1;
pub const BURST_FRAMES: u16 = 3;
pub const BURST_FRAME: u16 = BOMB_FRAME + 1;
pub const SPARK_FRAMES: u16 = 4;
pub const SPARK_FRAME: u16 = BURST_FRAME + BURST_FRAMES;
/// The invincible TALON (respawn and arming): each bank wrapped in energy waves that follow its
/// silhouette outward, 8 steps per loop.
pub const SHIELD_STEPS: u16 = 8;
pub const SHIELD_FRAME: u16 = SPARK_FRAME + SPARK_FRAMES;
/// The KEELBREAKER: hull at 5 roll samples, then its turret at 16 headings (`crate::midboss`).
pub const KEEL_ROLLS: u16 = 5;
pub const KEEL_FRAME: u16 = SHIELD_FRAME + PLAYER_BANKS * SHIELD_STEPS;
pub const TURRET_YAWS: u16 = 16;
pub const TURRET_FRAME: u16 = KEEL_FRAME + KEEL_ROLLS;
/// Ground and sea units (`crate::ground`): SKIFF at 16 headings; PILLBOX and CRAWLER at 16 headings, then
/// the same 16 armed (muzzles glowing `hot`); HATCH door poses closed to open; wrecks (small, large).
pub const UNIT_YAWS: u16 = 16;
pub const SKIFF_FRAME: u16 = TURRET_FRAME + TURRET_YAWS;
pub const PILLBOX_FRAME: u16 = SKIFF_FRAME + UNIT_YAWS;
pub const HATCH_POSES: u16 = 6;
pub const HATCH_FRAME: u16 = PILLBOX_FRAME + 2 * UNIT_YAWS;
pub const CRAWLER_FRAME: u16 = HATCH_FRAME + HATCH_POSES;
pub const WRECK_FRAME: u16 = CRAWLER_FRAME + 2 * UNIT_YAWS;
/// The GANTRY QUEEN's wreck (user: "show it in a damaged state and have it scroll offscreen smoothly"):
/// her body banked left, level and right (`QUEEN_WRECK_BANKS`), the Lattice dead, the hull scorched and
/// holed; 4 phases, as it glides to the screen's centre while it drifts down.
pub const QUEEN_WRECK_FRAME: u16 = WRECK_FRAME + 2;
pub const QUEEN_WRECKS: u16 = 3;
pub const QUEEN_WRECK_BANKS: [u16; 3] = [8, 24, 40];
/// Body sprite x of the wreck centred on the screen (canvas centre at multicolor x 80).
pub const QUEEN_WRECK_X: i16 = 80 - BOSS_SIZE as i16 / 2;
pub const FRAMES: u16 = QUEEN_WRECK_FRAME + QUEEN_WRECKS;

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
pub const QUEEN_LEG_CENTER: [i16; 3] = [165, 102, 12];
/// Part canvases (multicolor px × lines).
pub const QUEEN_ARM_CANVAS: (u16, u16) = (96, 216);
pub const QUEEN_LEG_CANVAS: (u16, u16) = (152, 200);
pub const DRONE_W: u16 = 40;
pub const DRONE_H: u16 = 40;
pub const PLAYER_W: u16 = 36;
pub const PLAYER_H: u16 = 66;
pub const GANNET_W: u16 = 36;
pub const GANNET_H: u16 = 72;
pub const PAIR_W: u16 = 8;
pub const PAIR_H: u16 = 10;
pub const ROUND_W: u16 = 3;
pub const ROUND_H: u16 = 5;
pub const BOMB_W: u16 = 3;
pub const BOMB_H: u16 = 8;
pub const BURST_W: u16 = 24;
pub const BURST_H: u16 = 44;
pub const SPARK_W: u16 = 6;
pub const SPARK_H: u16 = 10;
pub const KEEL_W: u16 = 32;
pub const KEEL_H: u16 = 256;
pub const TURRET_W: u16 = 16;
pub const TURRET_H: u16 = 32;
pub const SKIFF_W: u16 = 28;
pub const SKIFF_H: u16 = 56;
pub const PILLBOX_W: u16 = 22;
pub const PILLBOX_H: u16 = 44;
pub const HATCH_W: u16 = 22;
pub const HATCH_H: u16 = 44;
pub const CRAWLER_W: u16 = 32;
pub const CRAWLER_H: u16 = 64;
pub const SHIELD_W: u16 = 48;
pub const SHIELD_H: u16 = 88;

/// Lines in one period of the looping panorama: S1 BREAKWATER, 26 screens (the image has 200 more, a
/// copy of the first 200, so every window is contiguous).
pub const PANO_PERIOD: u16 = 26 * 200;
/// Sprites the boss adds to a frame: six parts and the body.
pub const QUEEN_SPRITES: usize = 7;

/// Sine of `p` (256 per turn) in -127..=127 (parabolic approximation).
#[inline(never)]
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".hitext"))]
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
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".hitext"))]
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
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".hitext"))]
pub fn bank(v: i16, banks: u16) -> u16 {
    let half = (banks / 2) as i32;
    let i = (v as i32 * half + if v >= 0 { 63 } else { -63 }) / 127;
    (half + i.clamp(-half, half)) as u16
}

/// Panorama line at the top of the screen after `lines` lines of stage travelled: the view starts on the
/// stage's first screen (the bottom of the panorama) and climbs, looping from the apron back to the sea.
pub fn scroll(lines: u16) -> u16 {
    (2 * PANO_PERIOD - 200 - lines % PANO_PERIOD) % PANO_PERIOD
}

/// Sine for compile-time tables (no float code reaches the C64): reduce to [-pi, pi], then a Taylor
/// series to x^13 (error < 1e-6 there).
pub const fn const_sin(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let mut x = x;
    while x > pi {
        x -= 2.0 * pi;
    }
    while x < -pi {
        x += 2.0 * pi;
    }
    let (mut term, mut sum, mut n) = (x, x, 1);
    while n < 7 {
        term = -term * x * x / ((2 * n) as f64 * (2 * n + 1) as f64);
        sum += term;
        n += 1;
    }
    sum
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

/// The GANTRY QUEEN at time `t`, `lift` lines above her hover line: sways ±150 px about the screen centre
/// (period 1024 ticks), banking into each turn through 49 roll angles, bobbing a little. Her arms and legs
/// are posed by the same bank sample. Writes the parts (z 0) and body (z 1) to `out[..QUEEN_SPRITES]`.
#[inline(never)]
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".hitext"))]
pub fn queen(t: u16, lift: i16, out: &mut [Sprite]) {
    let phase = t & 1023;
    let cx = 80 + isin_fine(phase) * 150 / 127;
    let cy = BOSS_CENTER_Y - lift + isin_fine((t >> 1) & 1023) * BOSS_DIP / 127;
    let b = bank(icos_fine(phase), BOSS_BANKS);
    let dxs = &PART_DX[b as usize];
    for i in 0..6 {
        let (x0, y0) = PART_XY0[i];
        out[i] = Sprite { frame: ARM_FRAME + i as u16 * BOSS_BANKS + b, x: cx + x0 + dxs[i] as i16, y: cy + y0, z: 0 };
    }
    out[6] = Sprite { frame: BOSS_FRAME + b, x: cx - BOSS_SIZE as i16 / 2, y: cy - BOSS_SIZE as i16 / 2, z: 1 };
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
