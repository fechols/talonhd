//! The M0/M1 tech scene: a 512×512 flying boss that sways and banks across
//! the top of the screen, a flight of spinning drones and the player's ship
//! weaving below it, over a looping vertical panorama.
//!
//! A pure function of demo time `t` with integer math only, so the host can
//! render exactly the frame the C64 reports showing.

use crate::render::Sprite;

/// Frame directory layout (see `models::demo_frames`).
pub const BOSS_BANKS: u16 = 9;
pub const DRONE_YAWS: u16 = 16;
pub const PLAYER_BANKS: u16 = 9;
pub const BOSS_FRAME: u16 = 0;
pub const DRONE_FRAME: u16 = BOSS_FRAME + BOSS_BANKS;
pub const PLAYER_FRAME: u16 = DRONE_FRAME + DRONE_YAWS;
pub const FRAMES: u16 = PLAYER_FRAME + PLAYER_BANKS;

pub const BOSS_SIZE: u16 = 512;
pub const DRONE_W: u16 = 40;
pub const DRONE_H: u16 = 40;
pub const PLAYER_W: u16 = 36;
pub const PLAYER_H: u16 = 56;

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

/// Fill `out` with the sprites of demo time `t`; returns how many.
pub fn scene(t: u16, out: &mut [Sprite; INSTANCES]) -> usize {
    let mut n = 0;
    // The boss: sways ±150 px about the screen centre (period 512 frames),
    // banking into each turn; bobbing a little.
    let phase = (t >> 1) as u8;
    let cx = 80 + isin(phase) * 150 / 127;
    out[n] = Sprite {
        frame: BOSS_FRAME + bank(icos(phase), BOSS_BANKS),
        x: cx - BOSS_SIZE as i16 / 2,
        y: -40 - BOSS_SIZE as i16 / 2 + isin(phase.wrapping_mul(3)) * 6 / 127,
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
        y: 132,
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
    }
}
