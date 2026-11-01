# Validation

## 2026-10-31 — M0/M1 tech scene on the C64 Ultimate

Package: `talonhd.prg` sha256 `1063395fc92b31e4…`, `talonhd.reu` sha256 `aaa5c99bd76611e8…` (1,119,202 bytes, 6.7 % of
16 MiB: panorama 48,960 B; boss 9 × 512×512 × 4 phases 1,024,762 B; drone 16 frames 23,378 B; player 9 frames 21,386 B).
Device: C64 Ultimate, firmware 1.1.0, core 1.49, FPGA 122; `tools/c64u.py config` (64 MHz turbo, badline timing
disabled, 16 MB REU), not saved to flash.

Host gates: `cargo test --release` green (renderer vs reference incl. vertical scroll, 1-phase frames, global
palette; raster3d goldens), `cargo check --no-default-features`, `tools/test_blit.py` (400 random columns), bake
self-check at 11 demo times.

Hardware:
- First `run` right after `config` did not load the PRG (RAM at `$0801` empty, presumably the config reset it);
  a second `run` then `upload-reu` worked. Run `config` first, then `run`, then `upload-reu`.
- Vertical fine scroll (`$D011` YSCROLL, 24-row mode) displays correctly with badline timing disabled.
- `dump --hold` at t=833 → `talonhd compare`: **0 mismatched bytes** (bitmap, screen RAM, color RAM).
- Render time over boss sweeps: 6.9–9.9 ms per frame (background window DMA ≈ 2.3 ms); the demo runs at the
  vblank rate (~60 fps, ~55 demo ticks per 0.9 s sample) with the 512×512 boss across the screen.

## 2026-10-31 — First Ludo model on the C64 Ultimate: the S1 TALON (HD bible approved)

Bible: HD layer (`docs/story/hd`) approved by the user. Ships rev 3, palette law rev 3 (canvas and floor fixes
from this pilot). `python tools/bible_hd.py check` ok, `tools/bible.py check` ok.

Ludo (credits: 2 for four `model create-image` renders + 15 for `model create --model hero` = **17**; account
1,245 → 1,228 available):
- `bible_hd.py export-prompts` → `ludo model create-image --n 4`. The art director picked v2, the only render with
  the approved single tail blade (v1, v3 and v4 had twin fins, which the TALON's entry forbids).
- `ludo model create --model hero --target-num-faces 1600 --texture-type simple`: 1,576 triangles, 2048² JPEG
  texture, axis-aligned (nose +z, up +y). Sources kept in `assets/ludo/hd.ship.talon/` (GLB sha256
  `e8d5f2e2aebb79eb…`).
- `tools/model_prep.py … --forward z --up y`: paint classes by normalized luminance + chroma
  (hull 791 / trim 717 / canopy 23 / glow 45 faces), QEM decimation 1,576 → 200 triangles,
  `assets/models/hd.ship.talon.mesh` (sha256 `9d633460ab9444b6…`).
- `talonhd bake-model`: 9 bank frames 36×66, 22,188 REU bytes, code share void 0 % / dark 50 % / light 49 % /
  hot 1 % (light ≥ 35 %: passes).

Art-direction fixes made during the review: lighting normalized so up-facing faces are solid light; the
checker only covers the middle 40 % of each step; albedo from the paint's luminance; player hull floor = solid
dark; the stand-in boss and background follow the light-share and "background one step down" rules.

Hardware (`talonhd.prg` `104fd394a15c519c…`, `talonhd.reu` `75496d0b5cee21ae…`, S1 palette 6/12/1/3):
- The first `run` again left the machine on garbage (status block full of the `$55` fill pattern); a second
  `run` + `upload-reu` worked. Same symptom as the M0 run.
- `dump --hold` at t=705 → `compare`: **0 mismatched bytes**; 7.4–7.6 ms per frame.

## 2026-10-31 — The green helmet HUD (64 multiplexed hardware sprites) on the C64 Ultimate

Package `talonhd.prg` `8f656672d89d9281…`, `talonhd.reu` `8ea313e84db57c07…`.

- 8 hires sprites (4 left, 4 right, unexpanded), 8 rows of 21 lines + 3-line gaps = 64 sprite instances per
  frame. The raster IRQ `hud_irq` (`asm/hud.S`) sets each row's Y and pointers in the gap; REU DMAs are capped
  at 512 bytes (`MAX_DMA`) so no transfer delays the IRQ past its gap.
- `tools/test_hud_irq.py`: `hud_irq` matches the table contract over 160 IRQs (Y, both banks' pointers, row
  counter, next line, registers preserved, no stray writes). `tools/test_blit.py` still passes.
- Code size: drawing the HUD per pixel (32-bit masks, division) cost ~11 KB of 6502 code and overflowed
  `$0801-$3FFF`. Readouts with few states (lives, gauge, speed) are now pre-drawn variants picked by pointer.
  Score and stage digits are byte-aligned glyph copies, and digits come from subtraction, not division.
  Program region `$0801-$4BFF` (end `$4554`).
- **Found on hardware:** HUD blocks at `$9000` showed character-ROM glyphs on every other frame (the VIC sees
  the char ROM at `$1000-$1FFF` of banks 0 and 2), which read as flicker and "doubled" sprites. Bank 2's
  blocks now sit at `$8000` (pointers 0–47), bank 1's at `$5000` (pointers 64–111); the IRQ ORs `$40` into
  bank 1's pointers. After the fix, 12 consecutive captures were clean.
- **Found on hardware:** the PRG accepted a stale REU image (the previous upload's magic) before the new
  upload arrived, and copied header bytes as HUD art. A second `run` after the upload worked. The boot check
  should verify the image checksum, not just the magic (todo).
- Render 7.8–11.6 ms per frame with the HUD; `dump --hold` at t=1316 → `compare`: **0 mismatched bytes**
  (bitmap, screen, color RAM; sprites are not part of the dump).

## 2026-10-31 — The GANTRY QUEEN (Ludo, 512×512 flying boss) and a race-free boot

Ludo: 2 credits for four `model create-image` renders, then 16 for `model create --model hero --rig`
(24,000-face target) = **18** (pilot total 35). The art director picked v2: the only render with exactly two
hazard-striped cannons, four splayed legs, the smoked cab and the purple facet ring. GLB: 24,892 triangles,
rigged (one skin), tower along +y. Sources in `assets/ludo/boss.s1.gantry_queen/` (GLB sha256
`f0fca665cd38ed24…`).

- `model_prep --forward z --up y`: concrete 16,282 / plates 1,930 / cab 5,716 / arms 964 faces → 3,000 triangles
  (5.9 s). The tower stands toward the viewer: 478 × 512 hires px footprint, 775 px tall.
  `assets/models/boss.s1.gantry_queen.mesh` (sha256 `06df22ff1dc97dd4…`).
- 9 roll frames (±24°) at 512×512 × 4 phases: 1,004,950 REU bytes. Code share void 15 / dark 65 / light 12 /
  hot 9 % (enemy light ≤ 15 %: passes).
- Review fixes:
  - Material albedo is now relative to the model's brightest paint (absolute albedo put the whole concrete
    hull in the void/dark checker).
  - The cab and arm paint sources name their colors (S1 models rev 2).
  - The arms' ramp changed light → hot, and "weapons are hot" was added to the art direction (S1 models rev 3,
    art direction rev 2).
  - The canvas centre sits at screen line −50, so the crown hangs over the top third of the screen.
- C64U (`talonhd.prg` `95e0afbf7bbfd2e3…`): 8.6–14.2 ms per frame with the boss across the screen; held frame
  t=1185 → **0 mismatched bytes**.

Boot race fixed (`talonhd.prg` `d89ff0078331f118…`): the PRG erases the REU magic at boot (`invalidate_image`),
then starts only when the magic is back **and** the whole-image checksum matches (`verify_image`; a test
checks that a half-uploaded image doesn't verify). Two consecutive run + upload cycles each came up on the
first try (previously about half the first runs failed); held frame t=897 → 0 mismatched bytes.

## 2026-10-31 — Smoother boss: 49 roll frames and a fine sway

The user found the boss's motion too jumpy. Cause: 9 roll samples 6° apart move the roof of the 775-px tower
by ~40 hires px per step, and the sway ran on a 256-step sine (3-4 px jumps every other frame).
- 49 roll samples (1°, S1 models rev 4): 5,419,584 REU bytes for the boss, level image 5,518,505 bytes (32.9 %
  of 16 MiB, 11 REU banks; `bank_wraps` 0).
- Sway and bank driven by a 1024-step sine (`demo::isin_fine`): at most ~1.17 px per frame.
- C64U (`talonhd.prg` `912d620acfbd7bfe…`, `talonhd.reu` `75fa4fe338780aa2…`): the 6.4 s upload is accepted on the
  first run (checksum wait); 8.7–14.4 ms per frame; held frame t=940 → **0 mismatched bytes**.

## 2026-10-31 — The GANTRY QUEEN by hand; Skip segments

The user found the Ludo boss model didn't look good from above and asked the art director to build it. The
Ludo GLB stays in `assets/ludo/boss.s1.gantry_queen/` as a record; the bible block now says
`generator = "hand"` (S1 models rev 5; palette law rev 4: BOSS triangle floor 500).
- `src/hand.rs` `gantry_queen()`, 698 triangles:
  - a faceted dome cut like a gem: broad ~30° rings, each showing 8 differently lit facets;
  - a crystal crown band and eight crystal spikes;
  - a hazard mast and beacon;
  - two cannon arms with forward twin barrels (hot, glowing muzzles);
  - four legs splayed sideways with knee crystals and foot pads;
  - a short ribbed shaft.

  Authored at full size and shown at 0.6 (`QUEEN_SCALE`): crown ≈ one screen wide, legs and cannons ≈ two.
  Written by `talonhd hand-models` to `assets/models/boss.s1.gantry_queen.mesh` (sha256 `34266a41d119d460…`).
- Raster3d: faces turned toward the key light may exceed a flat top (lambert up to 1/l.z), so light-facing
  facets catch a highlight. Golden hashes regenerated (boss and drones changed; the player's stand-in did not).
- **Skip segments** (`SEG_SKIP`, b1 bit 6): a column's REU data stores only run bytes; a Skip before the run
  after a gap ≥ 8 lines subtracts the gap from DATA. They changed together:
  - the Rust reference `draw_column`;
  - `asm/blit.S` (which now rewrites `JOB+DATA`);
  - `tools/test_blit.py` (random Skips; DATA writable).

  400 random columns match. On the full-size hand model, Skip cut 11.5 MB → 8.8 MB.
- Staging: canvas centre at line −60, dipping ±40 lines over 2048 frames; 49 roll frames (1°).
- Level image 3,885,437 bytes (23.2 % of 16 MiB; boss ~3.8 MB). Code share void 11 / dark 66 / light 7 /
  hot 16 % (at full scale).
- C64U (`talonhd.prg` `7dbe3ef8b5218e52…`, `talonhd.reu` `42719c0a3ed191b7…`): accepted on the first run; 9.7–14.6
  ms per frame; held frame t=540 → **0 mismatched bytes**.

## 2026-10-31 — The S1 roster, hand-built

Bible: S1 models rev 6 (every S1 entity `generator = "hand"`; canvases sized to hold each object at any yaw;
KEELBREAKER 256 × ~52, parts resized); palette law rev 5 (triangle floors lowered for hand models).

`src/hand.rs`:

| Model | Triangles | Frames | REU bytes | Light share |
|---|---|---|---|---|
| WASP | 16 | 48 (16 yaw × 3 bank) | 34,474 | 0 % |
| GANNET | 84 | 80 (16 yaw × 5 dive pitch) | 157,046 | 13 % |
| SKIFF | 81 | 16 | 23,961 | 11 % |
| PILLBOX | 152 | 16 | 28,526 | 10 % |
| HATCH | 98 | 6 door poses | ~2,000 each | 0 % |
| CRAWLER | 70 | 16 | 37,688 | 3 % |
| KEELBREAKER hull | 201 | 5 (roll) | 54,905 | 11 % |
| KEELBREAKER turret | 54 | 16 | 12,928 | — |
| TALON (the S1 ship) | 88 | 9 | 21,383 | 60 % |

Every enemy is within the ≤ 15 % light rule.

- `every_model_fits_at_every_orientation`: no model touches its canvas edge at any orientation. It caught
  four clipping models and a HATCH bug (the doors slid out past the frame).
- Art direction from the lineup review (`talonhd lineup` → `output/lineup/s1.png`):
  - Void/dark-checker hull tops read as holes over S1's blue sea. Main hulls are now void/dark/light at
    albedo 0.6 with a solid-dark floor; secondary parts are solid dark.
  - Flat decks read flat, so decks and domes are faceted (raised centre fans).
- Tech scene: the hand WASP flies the drone paths. C64U (`talonhd.reu` `a8f8fa575110fba5…`): 9.1–9.6 ms per frame;
  held frame t=222 → **0 mismatched bytes**.
- The user picked the hand-built TALON over the Ludo model (`hd.ship.talon` rev 4, `generator = "hand"`); it is
  now `assets/models/hd.ship.talon.mesh` and the tech scene's player. Bake self-check: model matches the
  reference compositor; `talonhd.reu` `c093208f78bb704c…` (3.88 MB, 23.1 % of 16 MiB). Renderer and PRG
  unchanged, not re-run on the C64U.

## 2026-11-01 — GANTRY QUEEN parts, CRAWLER roof, the S1 panorama

Bible: S1 models rev 7.

- **GANTRY QUEEN parts** (`hand::queen_parts`): the body (414 triangles, 49 rolls) loses its arms and legs to
  separate sprites: 2 arms × 5 aims (±18° about the shoulder, toward the player) and 4 legs × 6 paddling
  strides, each with 7 roll samples (8° apart). Canvases measured over every pose and roll: arms 112 × 240,
  legs 168 × 232. The scene puts each part at the body's rolled image of its origin
  (x' = x cos + z sin, Q14 Taylor in a `const` table); `examples/queen_assembly.rs` checks the fit at full
  left, level and full right (`output/lineup/queen.png`).
- **CRAWLER**: a hipped armour roof and bevelled track guards (72 triangles).
- **S1 panorama** (`src/s1.rs`, 26 screens, 5200 lines, 675 cell rows): void 76 %, dark 23 %, light 0.8 %,
  hot 0.4 % (`the_background_stays_down`: light ≤ 10 %, hot ≤ 3 %). The scene starts at the stage's
  first screen and loops.
- REU image `talonhd.reu` `9436e22699ec4491…`: 4.77 MB (28.5 % of 16 MiB): body 2.24 MB, arms 0.78 MB, legs
  1.50 MB, panorama 216 KB. Self-check: model matches the reference compositor.
- C64 code: the scene had grown to 5.2 KB (unrolled loops, 32-bit sine math) and overflowed `$4BFF` by 3 KB.
  Now compact: compile-time part tables, `#[inline(never)]` helpers; 17,394 of 17,407 bytes used.
- `test_blit.py` and `test_hud_irq.py` pass. C64U (PRG `9e11f916e34ac69c…`): held frame t=253, render
  11.5 ms, background 2.7 ms → **0 mismatched bytes**.

## 2026-11-01 — More code space; TALON's game on the C64U

- **Relocated code.** The C64 program grew past `$4BFF`. Code now also runs from three regions copied at boot
  (`talonhd_boot`) from one load image at `$4C00-$7BFF` (frame-buffer space, free until the first frame):
  `.hitext` `$E500-$FEFF` (under the KERNAL), `.midtext` `$9000-$9FFF` (VIC bank 2 sees char ROM there, the
  CPU sees RAM), `.lowtext` `$0200-$07FF`. Sections take attributed functions or whole modules by
  mangled-name prefix. Build at `opt-level = "s"` (O2 inlined the sim to ~31 KB; "s" ~24 KB).
- First boot hung: rewriting `link.ld` had dropped the `/DISCARD/ .init.250` rule, so the SDK's charset
  switch called KERNAL CHROUT at `$FFD2`, now relocated sim code. Found by diffing C64 RAM against the PRG
  and the section images (all intact), then disassembling the init chain.
- **Simulation** vendored unchanged from TALON `95179bb` (`src/sim`): all its unit tests and the replay
  digest `0x780fe9b1` pass here.
- **Game layer** (`src/game.rs`): one sim tick per frame; the 304 × 176 playfield centred on screen; sprites
  for the TALON (bank eased per tick), WASPs (16 yaws by heading), GANNETs (dive/level/pull-up/climb poses),
  TWIN pairs, rounds, bombs, 3 burst frames, 4 SPARK frames; HUD from score, lives, gauge cursor, speed,
  deaths. Attract mode: the sim's autopilot flies until the stick or SPACE is touched. `Game::ticks` never
  resets (`World::tick` does at GAME OVER, which first hung the bake's replay). The GANTRY QUEEN flies over
  the airfield as scenery (lines 4400-5200; the autopilot's games end near line 4700, so she arrives early).
- Bake self-check: model = reference compositor at 13 attract ticks incl. a GANNET dive and the boss (t=9200).
  REU image 4.80 MB (28.6 %).
- C64U: about 50 frames per second, render 3.8-4.3 ms (background 2.2 ms). Held frame tick 2612 (score 2,600):
  **0 mismatched bytes** against the host replay; tick 9388 (boss over the airfield, render 9.4 ms): **0 mismatched
  bytes**.

## 2026-11-01 — GANTRY QUEEN parts follow her bank (no more jumping legs)

User: the legs "are moving far too many pixels per frame". Cause: 6 paddle poses changing every 8 frames (feet
jumped 5-10 px) and only 7 roll samples per part (each 8° switch lurched a leg up to ~10 px). Now each arm
and leg has one frame per body bank sample (49, 1° apart), rendered at the body's roll and posed by the
bank: barrels lean 12° into the turn, legs trail 12° like pendulums (`hand::queen_part_frames`; bible S1
models rev 8). `queen_parts_fit_and_move_smoothly`: no part touches its canvas, and no part's centroid moves
1.5 px or more between neighbouring samples (the bank steps at most once every ~6 frames). Canvases
re-measured: arms 96 × 216, legs 152 × 200. Arms 1.07 MB, legs 1.72 MB; image 5.31 MB (31.7 %). The 34
per-pose `.mesh` files are gone (frames come straight from `hand.rs`); C64 code shrank ~900 bytes.
C64U: held frame tick 9380 (boss over the airfield, render 9.5 ms): **0 mismatched bytes**.

## 2026-11-01 — The GANTRY QUEEN fight; border flash, camera shake, counter-shaking HUD

- **Fight** (`src/boss.rs`, state beside the vendored `World`): she arrives at line 4400; the scroll holds at
  4560. Phase 1 targets are what the TALON can reach in the flying staging: both arm cannons and the two
  front legs, 15 HP each (the rear legs hang above the screen); TRIPLE FAN and MAST SWEEP. Phase 2: rear
  legs snap, `hot` turns yellow (bible `phase2_hot = 7`), the cab rim has 120 HP; RING BURST, OFFSET RINGS,
  TRIPLE FAN. Defeat: 30,000 points, bursts, she sinks away, the stage scrolls on.
  `the_fight_runs_to_the_end` plays it through on the host (the attract autopilot reaches the fight but
  never breaks all four parts, so phase 2 is covered by that test, not by the hardware replay).
- **Hit feedback** (user): no HUD flashing; the border ramps 0, 11, 15, 1, 10, 2, 10, 15, 11, 0 (2 frames
  each); the view shakes for 16 frames (vertical through the panorama window and sprites, horizontal
  through XSCROLL in 38-column mode, rest 4) and the HUD sprites move the opposite way.
- **Code space:** +4.9 KB (fight, shake). Two more boot-copied regions: `$5C00-$5FFF` and the RAM under
  the I/O chips, `$D000-$DFFF`, for the boss code: the game step runs with `$01 = $34` and `asm/hud.S`
  now saves `$01`, banks I/O in and restores it (`test_hud_irq.py` checks it from a `$34` state).
  `.hitext` starts at `$E410`. Low RAM has 485 bytes left.
- Bake self-check: model = reference at 14 ticks incl. a camera shake (3509) and the fight (9320).
- C64U: held frame tick 9580 (mid-fight, render 8.1 ms): **0 mismatched bytes**.

## 2026-11-01 — HUD multiplexer race fixed

User saw HUD lines flicker (e.g. under LIFE). Four C64U captures diffed: in one, the last line of every row
in sprite 5's column showed the next row's last line (LIFE's underline gone, ARM's a row too high). Cause:
a sprite at Y shows on lines Y+1..Y+21 and the VIC fetches that last line for sprites 3-7 early in line
Y+21, exactly when the row IRQ switched pointers; at 64 MHz the handler sometimes won. Fix: the row IRQ fires
at Y+22 (`hud::build`), still before the next row's Y compare at Y+24, and `MAX_DMA` is 256 bytes (~1 line
of CPU stall) to keep that slack. Background restore 2.24 -> 2.47 ms (more DMA commands). After the fix,
8 captures: HUD green pixels identical in all of them outside the two scrolling tapes (dark specks inside
glyphs are the panorama showing through transparent sprite pixels).

## 2026-11-01 — Harder, two-harmonic camera shake

User: a harder shake, as two sines of different frequency and amplitude. `game::SHAKE_X/Y` are compile-time
tables (24 frames): x = env·(9 sin(2πt/5.5 + 2) + 4 sin(2πt/2.6 + 1)), y = env·(4.5 sin(2πt/6.5 + 1) +
2 sin(2πt/3.1 + 0.3)), env = (1 - t/24)², clamped to ±12 px / ±6 lines. x: 12, -1, -8, -2, -1, 7, 3, -4 ...
Beyond XSCROLL's ±4 the background DMA starts 8 bytes per column earlier or later (`Scroll::col`), the
wrapped-in edge columns are filled with code 00, and sprites move 4 multicolor px per column; the reference
compositor models both. Bake self-check adds the shake's +12 (two columns) and -8 (one column) frames.
C64U: held frame tick 3556 (render 3.1 ms): **0 mismatched bytes**.
