# TALON level format (`talon-level-v1`)

A stage is hand-authored in `assets/levels/<stage>.level.json` (S1: `assets/levels/s1.level.json`). Ludo
supplies textures, set-pieces and sprites only; every layer, ground object, wave and checkpoint is placed
here by hand. `python tools/stage_build.py check` validates a level against the bible;
`python tools/stage_build.py build` bakes it (see "Build" below).

## Coordinates

- **x**: display pixels from the left edge of the 320-px bitmap (0..319). The multicolour art pixel is 2
  display pixels wide, so x is effectively quantised to 2 px; ground objects are cell aligned (x % 8 = 0).
- **y**: *stage lines from the start*, measured upward: line 0 is the bottom of the opening view, line
  `lines` the end of the stage. A placement's `y` is its **bottom** edge. A wave's `line` is the scroll
  position (`World::scroll_lines`) at which it starts. Screen *n* of the bible is lines `n*200 ..
  (n+1)*200`.
- The baked image is stored with the stage end at the top (image row 0 = line `lines`); the REU block is
  rotated so that the opening view is distinct rows 0..21 (`src/format.rs`).

## Top level

| Field | Meaning |
|---|---|
| `format` | `"talon-level-v1"` |
| `stage`, `story_file`, `story_revision` | the bible entry (must be approved) and the revision the level was authored against |
| `lines` | `length_screens * 200` (S1: 5,200 = 650 cell rows); `format.rs` `STAGE_ROWS` must equal `lines / 8` |
| `background_ink` | `$D021` inside the playfield, equal to the bible's `background_ink` (`format.rs` `STAGE_BACKGROUND`) |
| `reu` | `{"stage": <REU address>, "patches": <REU address>}` (`format.rs` `STAGE_BASE`, `PATCHES`) |
| `art` | quantizer settings: `weights` (vote weight per ink, favouring thin bright accents) and `keep_isolated` (inks the despeckle may not remove) |
| `sections` | `{n, name, lines: [from, to)}`, equal to the bible's sections (`screens * 200`) |
| `checkpoints` | `{n, screen, line}`, the bible's checkpoint screens |
| `sources` | named source files (below) |
| `layers` | background layers, painted in order, bottom first (below) |
| `object_art` | per bitmap-layer entity: how its master is cut and inked (below) |
| `ground_objects` | bitmap-layer enemy instances (below) |
| `waves` | the air and ground spawn list (below) |
| `preview_frames` | scroll lines of the 320x200 review frames |

## Sources

`{"file", "sha256", "tile" | "entity", "ludo": {"request_id", "job_id", "take"}}`. Files are the
byte-exact Ludo outputs in LFS (`assets/tiles/<stage>/`, `assets/sprites/source/<entity>/`); `check`
verifies the hash. `tile` names a `[[tiles]]` id of the stage's bible entry (its prompt came from
`bible.py export-prompts`), `entity` an approved bitmap-layer entity.

## Layers

| Field | Meaning |
|---|---|
| `id`, `source` | layer name; source name |
| `crop` | `[x, y, w, h]` in source pixels (cut inside Ludo's ragged transparent margins, or one wall out of a set-piece) |
| `flip` | `"x"`, `"y"` or `"xy"` (mirrored reuse of a prop block) |
| `mask` | `"ellipse"`: keep only the inscribed ellipse of the crop (the lighthouse cut from the sea wall) |
| `inks` | the inks this layer may become (the section's `inks_intent` plus what the art needs) |
| `k` | k-means clusters for the ink segmentation (default 10) |
| `overrides` | `[{"near": "#rrggbb", "ink": n}]`: the cluster nearest that colour becomes ink n (hand fixes: dark harbour water -> blue 6, rust -> brown 9) |
| `inkmap` | reuse another layer's clusters and inks (mirrored copies stay identical) |
| `drop_inks` | inks that become transparent (an islet tile keeps its rocks and surf, its own sea is dropped onto the stage's sea) |
| `x`, `y`, `width`, `height` | placement in display px / stage lines |
| `tile: "v"`, `from`, `to`, `phase` | repeat the `width x height` block vertically over lines `[from, to)` (seamless textures) |

## Ground objects (bitmap layer)

`{"id", "entity", "source", "x", "y", "note"}`: x and y cell aligned (multiples of 8), the rect is the
entity's `cell_rect` (PILLBOX 3x3 cells), inside the stage, above the opening view (y >= 176: the
opening rows are stored twice), no overlaps. The stage is baked with every object at frame 0.

`object_art.<entity>`: `source`, `crop`, `inks`, `k`, `overrides` as a layer, plus the offline frame
parameters: `barrel_ink`, `barrel_start`, `barrel_reach`, `barrel_gap`, `barrel_width` (fractions of the
object size: two barrel lines drawn over the rotated master, because 1-px painted barrels vanish at 12
art px). Frames follow the bible's animations: `angles` headings rotated from the master (heading 0 =
barrels straight down, clockwise), the same headings with a muzzle flash (`fire`), then `wreck`
(an offline crater stand-in until the bible's hydra wreck take is accepted).

## Waves

`{"line", "entities", "count", "path", "formation", "spacing_ticks", "fire", "spark", "track", "bible", ...}`;
sorted by `line`; entities must be the stage's enemies, midboss or boss.

`track` is the hand placement the simulation runs: a list of strings, each
`{"curve", "field_x", "y", "count", "spark"}`. `curve` is an authored WASP curve (`s_left`, `s_right`,
`hook_left`, `hook_right`, `drop_u`, `drop_u_mirror`); `field_x`/`y` are the entry point in **playfield**
pixels (x 0..304, even; y negative = above the top edge); `spark` makes the last of the string the leader
that drops a SPARK. The counts add up to the wave's `count`; strings release one enemy every
`spacing_ticks` (default 10) from the tick the scroll reaches `line`.

`python tools/stage_build.py track` compiles every wave with `track` into `src/sim/track_<stage>.rs`
(8-byte `Spawn` records, `kind` bit 7 = leader), which `src/sim/track.rs` includes into the `p.world`
phase; a test fails if the file is stale. Waves without `track` (entities with no runner yet) are listed
in the generated file's header and do not spawn. The track repeats every `lines` lines, in step with the
looping stage strip, until the boss stop exists.

`talon budget` (also a `cargo test` and a `build.sh` step) flies the whole track with an invulnerable,
non-firing TALON and fails if the air layer needs more than 6 sprite channels on any board line, or more than
2 in the player zone (the lower 80 screen lines), `docs/design.md` 13. Two strings flying mirrored
horizontal legs at the same height are the usual breach: enter one a sprite height (>= 21 lines) apart.

## Build (`tools/stage_build.py`, `tools/talon/stage_art.py`)

1. Each layer's source is cropped, resampled in linear light (premultiplied alpha; box filter when
   shrinking) to 4x the display resolution and segmented into its inks (Oklab k-means, chroma-boosted
   nearest ink, then the overrides). Labels, not colours, are composed on a 1280 x `lines*4` canvas.
2. Every art pixel (2 x 1 display px = 8 x 4 canvas px) takes the ink with the largest weighted area
   share. No colours are averaged, no dithering.
3. Despeckle: an art pixel sharing its ink with none of its 8 neighbours takes the neighbours' majority.
4. Legalise: `$D021` is free; a 4x8 cell with more than three other inks keeps the triple with the least
   weighted Oklab error and moves the other pixels to their nearest kept ink.
5. Pack (codes 00 `$D021`, 01 screen high nibble, 10 screen low nibble, 11 colour RAM; a cell's inks in
   ascending order) into the REU stage block, check it decodes back to the ink map, and write
   `assets/generated/<stage>/stage.bin`, `patches.bin` and `manifest.json` (hashes, sizes, inks per layer),
   plus previews under `output/stage/<n>/`.

## Patch block (`patches.bin`, REU `PATCHES`)

Header (16 bytes): magic `TPAT`, version 1, object count, entry size 16, reserved, total bytes u32,
reserved u32. Then one 16-byte entry per object: entity code (1 PILLBOX, 2 HATCH, 3 CRAWLER), frame
count, cell columns, cell rows, cell column, reserved, distinct top row u16 (stage row in the REU's
rotated numbering), REU address of frame 0 u32, frame stride u16 (`cols*rows*10`), reserved u16. Each
frame is bitmap (`cols*rows*8`, row-major cells, VIC cell order), screen (`cols*rows`), colour
(`cols*rows`). Each frame is quantized over the object's exact background (a one-cell margin of
context); frame 0 equals the stage's own cells, which `talon` checks.

Runtime (next step, not in M4): a heading change DMAs the frame's cells into both VIC banks and colour
RAM where the rect is in view, and stashes them into the REU stage planes (both stored copies when the
rect lies in rows 0..21) so the back-buffer fill keeps the current frame.
