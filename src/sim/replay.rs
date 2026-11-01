//! A deterministic autopilot that produces input logs for the replay tests and the host player's demo.
//! It reads only the `World` it is driving, so the log it records replays identically anywhere: the
//! host tests pin the digest it reaches, and `tools/test_native_sim.py` feeds the same log to the C64
//! runtime's joystick port and compares digests.
use super::world::*;
use super::*;

/// The input for the next tick: fire continuously, stay low, line up under the lowest WASP or GANNET (or
/// chase a SPARK; beside a diving GANNET), sidestep any WASP or bullet closing in on the band, and ARM SPEED or TWIN when the
/// cursor reaches them.
pub fn autopilot(world: &World) -> u8 {
    let mut input = FIRE;
    let px = (world.player.x >> 8) as i16;
    let py = (world.player.y >> 8) as i16;
    let mut target_x = 152i16;
    let mut target_y = 140i16;
    let mut best = i16::MIN;
    for w in world.wasps.iter().filter(|w| w.alive) {
        let (wx, wy) = ((w.x >> 8) as i16, (w.y >> 8) as i16);
        if wy > best && wy < py - 30 && (0..FIELD_W).contains(&wx) {
            best = wy;
            target_x = wx;
        }
    }
    // A GANNET: stand beside it while it dives and drops (its bombs fan straight down), shoot it as it climbs.
    for g in world.gannets.iter().filter(|g| g.state != 0) {
        let (gx, gy) = ((g.x >> 8) as i16, (g.y >> 8) as i16);
        if g.state <= super::actors::G_TELEGRAPH {
            target_x = if gx > FIELD_W / 2 { gx - 56 } else { gx + 56 };
            best = i16::MAX;
            // Hold fire while still under it, so this replay sees the release (a player would shoot).
            if (gx - px).abs() < 40 { input &= !FIRE; }
        } else if gy > best && gy < py - 30 && (0..FIELD_W).contains(&gx) {
            best = gy;
            target_x = gx;
        }
    }
    if let Some(s) = world.sparks.iter().find(|s| s.alive && (s.y >> 8) as i16 > 40) {
        target_x = s.x0;
        target_y = ((s.y >> 8) as i16).clamp(110, 150);
    }
    // Danger: a WASP within 56 px above or beside the ship moves us away from it.
    for w in world.wasps.iter().filter(|w| w.alive) {
        let (wx, wy) = ((w.x >> 8) as i16, (w.y >> 8) as i16);
        if (wy - py).abs() < 56 && (wx - px).abs() < 36 {
            target_x = if wx > px { px - 48 } else { px + 48 };
            target_y = 150;
        }
    }
    // A bullet falling toward the ship within 64 px above it: step out of its column.
    for b in world.ebullets.iter().filter(|b| b.kind != 0) {
        let (bx, by) = (b.x >> 6, b.y >> 6);
        if by < py && py - by < 64 && (bx - px).abs() < 14 {
            target_x = if bx > px { px - 40 } else { px + 40 };
        }
    }
    if target_x < px - 2 { input |= LEFT } else if target_x > px + 2 { input |= RIGHT }
    if target_y < py - 2 { input |= UP } else if target_y > py + 2 { input |= DOWN }
    if (world.gauge.cursor == gauge::SPEED && world.gauge.speed < 3 || world.gauge.cursor == gauge::TWIN) && world.tick & 1 == 0 {
        input |= ARM;
    }
    input
}

/// Record `ticks` ticks of the autopilot from a new game: the input log and the digest after each tick.
#[cfg(feature = "host")]
pub fn record(ticks: usize) -> (Vec<u8>, Vec<u32>) {
    let mut world = World::new();
    let mut inputs = Vec::with_capacity(ticks);
    let mut digests = Vec::with_capacity(ticks);
    for _ in 0..ticks {
        let input = autopilot(&world);
        world.step(input);
        inputs.push(input);
        digests.push(world.digest());
    }
    (inputs, digests)
}
