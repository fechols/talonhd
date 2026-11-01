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
pub const DRONE_FRAME: u16 = BOSS_FRAME + BOSS_BANKS;
pub const PLAYER_FRAME: u16 = DRONE_FRAME + DRONE_YAWS;
pub const FRAMES: u16 = PLAYER_FRAME + PLAYER_BANKS;

pub const BOSS_SIZE: u16 = 512;
/// Screen line of the boss canvas centre: it hovers at the top, mostly above the screen.
pub const BOSS_CENTER_Y: i16 = -60;
/// The boss slowly sinks and rises this many lines about its centre (period 2048 frames).
pub const BOSS_DIP: i16 = 40;
pub const DRONE_W: u16 = 40;
pub const DRONE_H: u16 = 40;
pub const PLAYER_W: u16 = 36;
pub const PLAYER_H: u16 = 66;

/// Lines in one period of the looping panorama (the image has 200 more, a
/// copy of the first 200, so every window is contiguous).
pub const PANO_PERIOD: u16 = 1024;
pub const DRONES: usize = 6;
pub const INSTANCES: usize = DRONES + 2;

/// Sine of `p` (256 per turn) in -127..=127 (parabolic approximation).
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
fn bank(v: i16, banks: u16) -> u16 {
    let half = (banks / 2) as i32;
    let i = (v as i32 * half + if v >= 0 { 63 } else { -63 }) / 127;
    (half + i.clamp(-half, half)) as u16
}

/// Panorama line at the top of the screen: the view climbs one line a frame.
pub fn scroll(t: u16) -> u16 {
    (PANO_PERIOD - (t % PANO_PERIOD)) % PANO_PERIOD
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

/// Fill `out` with the sprites of demo time `t`; returns how many.
pub fn scene(t: u16, out: &mut [Sprite; INSTANCES]) -> usize {
    let mut n = 0;
    // The boss: sways ±150 px about the screen centre (period 1024 frames),
    // banking into each turn through 49 roll angles; bobbing a little.
    let phase = t & 1023;
    let cx = 80 + isin_fine(phase) * 150 / 127;
    out[n] = Sprite {
        frame: BOSS_FRAME + bank(icos_fine(phase), BOSS_BANKS),
        x: cx - BOSS_SIZE as i16 / 2,
        y: BOSS_CENTER_Y - BOSS_SIZE as i16 / 2 + isin_fine((t >> 1) & 1023) * BOSS_DIP / 127,
        z: 0,
    };
    n += 1;
    // Drones: fall through the screen on sine paths, spinning.
    for i in 0..DRONES as u16 {
        let p = t.wrapping_mul(2).wrapping_add(i * 43) as u8;
        let y = ((t + i * 48) % 288) as i16 - 48;
        out[n] = Sprite {
            frame: DRONE_FRAME + (t / 3 + i * 5) % DRONE_YAWS,
            x: 80 + isin(p) * 60 / 127 - DRONE_W as i16 / 2 + (i as i16 - 3) * 8,
            y,
            z: 1,
        };
        n += 1;
    }
    // The player: weaves along the bottom, banking with its motion.
    let p = t.wrapping_mul(3) as u8;
    out[n] = Sprite {
        frame: PLAYER_FRAME + bank(icos(p), PLAYER_BANKS),
        x: 80 + isin(p) * 50 / 127 - PLAYER_W as i16 / 2,
        y: 122,
        z: 2,
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
}
