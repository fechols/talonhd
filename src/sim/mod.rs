//! The deterministic `no_std` game simulation shared by the host player and the C64 Ultimate.
//!
//! Vendored unchanged from TALON (`../talon/src/sim`, commit 95179bb; `THIRD-PARTY.md`) so TALON's replay
//! digest still holds. TALON HD maps the 304 x 176 playfield onto its screen in `crate::game`. The notes
//! below about REU code phases describe TALON's C64 runtime, not TALON HD's.
//!
//! Authored in NTSC ticks (one step per 60 Hz refresh) with Q8 fixed point, as `docs/design.md` 2 and
//! pummel's simulation: positions are `i32` Q8 (256 = one logical pixel), velocities Q8 px per tick,
//! angles in 1/256 turns (0 = up, clockwise). Playfield coordinates: x 0..303, y 0..175 (top of the
//! playfield = 0), the 304-px safe area between the two HUD rows. No floats, no randomness, no host state:
//! the same input log gives the same `World` on both sides (`digest`).
//!
//! One tick is two halves, run in this order by the host (`World::step`) and by the C64, where each half is
//! a code phase paged from the REU (`docs/architecture.md`, "Code phases"):
//! - `World::step_world` (module `actors`, phase `p.world`): scroll, the hand-authored spawn track, the
//!   WASP swooper archetype, explosions and the drifting SPARKs;
//! - `World::step_sim` (modules `player`, `collide`, `gauge`, phase `p.sim`): the TALON, TWIN shots,
//!   collisions, score, pickups, the gauge, death and respawn.
//!
//! Helpers both halves call live in `common` (resident on the C64).
pub mod actors;
pub mod collide;
pub mod common;
pub mod digest;
pub mod gauge;
pub mod player;
pub mod replay;
pub mod track;
pub mod world;

pub use world::World;

#[cfg(test)]
mod tests;

/// Input bits, one byte per tick (`docs/controls.md`). Directions and FIRE match the CIA's joystick bits
/// (`!$DC00 & 31`), ARM is Fire 2 (POTX) or SPACE.
pub const UP: u8 = 1;
pub const DOWN: u8 = 2;
pub const LEFT: u8 = 4;
pub const RIGHT: u8 = 8;
pub const FIRE: u8 = 16;
pub const ARM: u8 = 32;
pub const PAUSE: u8 = 64;
pub const START: u8 = 128;

/// Playfield size in logical pixels (`docs/design.md` 2 and 4).
pub const FIELD_W: i16 = 304;
pub const FIELD_H: i16 = 176;
/// Q8 one pixel.
pub const PX: i32 = 256;
