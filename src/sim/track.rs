//! The stage's spawn track and the WASP curves. No procedural placement (CLAUDE.md: levels are
//! hand-authored): every record comes from a wave's `track` list in `assets/levels/*.level.json`, compiled
//! into `track_<stage>.rs` by `tools/stage_build.py track` and linked into the `p.world` phase.
//!
//! Units: `at` is the scroll line (lines of stage travelled, `World::scroll_lines`) relative to the start
//! of the current pass of the track, which loops every `LOOP_LINES`. At the S1 scroll of 0.5 px/tick a
//! line is two ticks, so a 200-line "screen" is 6.7 s.

/// One spawn record (8 bytes in the level format): a string of `count` enemies of `kind` entering at
/// playfield (`x2 * 2`, `y`), one every `spacing` ticks, flying curve `curve` (bit 7 = mirrored).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spawn {
    pub at: u16,
    pub kind: u8,
    pub curve: u8,
    pub x2: u8,
    pub y: i8,
    pub count: u8,
    pub spacing: u8,
}

/// Enemy kinds of the spawn records (`kind & KIND`), and the leader flag: the last of the string is the
/// leader (yellow core) and drops a SPARK.
pub const WASP: u8 = 1;
/// A GANNET string: one diver every `spacing` ticks over the player's x at its spawn (`x2` unused).
pub const GANNET: u8 = 2;
pub const KIND: u8 = 0x7F;
pub const LEADER: u8 = 0x80;

/// Curve ids (`CURVES`), and the mirror flag (left-right: angles negated, turns reversed).
pub const S_CURVE: u8 = 0;
pub const HOOK: u8 = 1;
pub const DROP: u8 = 2;
pub const MIRROR: u8 = 0x80;

/// A curve: the entry heading and up to 8 segments of (`ticks`, `turn` per tick in 1/256 turns). After the
/// last segment the WASP flies straight on until it leaves the screen ("WASPs never stop").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Curve {
    pub heading: u8,
    pub segments: [(u8, i8); 8],
}

/// WASP curves (`03-enemies/common-wasp.md`: S-curves whose horizontal legs stay above screen line 120,
/// vertical inside the 80-line player band). A turn of 4 per tick is a quarter turn in 16 ticks (a
/// 25-px radius at 2.5 px/tick).
pub static CURVES: [Curve; 3] = [
    // S_CURVE (s_left): down from the top, a leg to the right at about y 30, down, a leg back to the left
    // at about y 95, then straight down through the player band.
    Curve { heading: 128, segments: [(6, 0), (16, -4), (30, 0), (16, 4), (4, 0), (16, 4), (30, 0), (16, -4)] },
    // HOOK (hook_left): in from the left edge heading right, then a hook down and out the bottom.
    Curve { heading: 64, segments: [(28, 0), (16, 4), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)] },
    // DROP (drop_u): a short drop, a dog-leg toward the player's half, then down and out.
    Curve { heading: 128, segments: [(20, 0), (16, 4), (14, 0), (16, -4), (0, 0), (0, 0), (0, 0), (0, 0)] },
];

// The stage's spawn track, generated from `assets/levels/s1.level.json` by `tools/stage_build.py track`
// (`STAGE_LINES`, `TRACK`). It repeats with the stage: the scroller loops the stage strip every
// `STAGE_LINES` lines until the boss stop exists.
include!("track_s1.rs");

/// The track repeats every `LOOP_LINES` scroll lines, in step with the stage strip.
pub const LOOP_LINES: u32 = STAGE_LINES;

/// WASP velocity per heading (16 headings, 0 = up, clockwise) at `speed_q8 = 640` (2.5 px/tick):
/// `640 * sin`, `-640 * cos` of `h * 22.5` degrees, rounded. Q8 px per tick.
pub static WASP_VX: [i16; 16] = [0, 245, 453, 591, 640, 591, 453, 245, 0, -245, -453, -591, -640, -591, -453, -245];
pub static WASP_VY: [i16; 16] = [-640, -591, -453, -245, 0, 245, 453, 591, 640, 591, 453, 245, 0, -245, -453, -591];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn velocity_tables_have_constant_speed() {
        for h in 0..16 {
            let (x, y) = (WASP_VX[h] as f64, WASP_VY[h] as f64);
            assert!(((x * x + y * y).sqrt() - 640.0).abs() < 1.0, "heading {h}");
            // Mirror symmetry: heading 16 - h is the left-right mirror.
            assert_eq!(WASP_VX[h], -WASP_VX[(16 - h) & 15]);
            assert_eq!(WASP_VY[h], WASP_VY[(16 - h) & 15]);
        }
    }

    #[test]
    fn track_is_sorted_and_in_the_playfield() {
        for pair in TRACK.windows(2) { assert!(pair[0].at <= pair[1].at); }
        assert!((TRACK[TRACK.len() - 1].at as u32) < LOOP_LINES);
        for s in &TRACK {
            assert!((s.x2 as i16) * 2 <= crate::sim::FIELD_W);
            assert!(s.count >= 1 && s.count as usize <= crate::sim::world::WASPS && matches!(s.kind & KIND, WASP | GANNET));
            assert!(s.spacing >= 1);
        }
    }
}
