# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

TALON HD is TALON (`../talon`, original IP) re-done as a solid-polygon vertical shooter for the 64 MHz C64 Ultimate
with a 16 MiB REU. Every ship is a flat-shaded 3D model, rendered **offline on the host** (orthographic, straight
top-down) at sampled orientations into software sprites of any size up to 512×512. The C64 only picks frames and
REU-DMAs their pixels into a double-buffered 160×200 multicolor bitmap (renderer vendored from `../gclass`).
Claude is also the **art director**: proposes and defends the look; the user approves.

## Rules

- **Four global colors.** All multicolor codes are global (`render::Palette`): 00 `$D021`, 01/10 the one screen
  RAM byte, 11 color RAM. Roles: 00 `void`, 01 `hull_dark`, 10 `hull_light`, 11 `hot`. One palette per stage; a
  write recolors everything (flashes, boss phases, stage transitions).
- **One REU image per level** (16 MiB each, art + music), auto-loaded by the PRG between stages.
- **Bible first.** `docs/story` is TALON's approved bible (forked); `docs/story/hd/` is the approved HD layer that
  wins where they differ (palette law, art direction, promotion ladder, ships, `[model]` blocks). Nothing is
  generated with Ludo for an entity that isn't approved; requests come only from `python tools/bible_hd.py
  export-prompts`. Report Ludo credits after every batch; always maximum quality (hero, 2048 textures).
- **Promotion ladder.** At every stage clear QUILL is promoted and gets a new airframe in the next stage's colors
  (`docs/story/hd/02-ladder.md`). Light belongs to the player; hot is the Lattice.
- **Original, not a clone.** The polygon look is borrowed; no names, ships, stages or story from existing games.
- **Vendored code** is recorded in `THIRD-PARTY.md` with its source commit. Never write to `../gclass`, `../talon`,
  `../sueme64`, `../impact`, `../pummel`.
- **Rust:** edition 2024, zero third-party Rust dependencies; the shared library is `no_std` without `host`.
- **C64 Ultimate:** mutating MCP calls and device runs need the user's go-ahead in the current session. Never save
  config to flash.
- **No sprite flicker, ever** (user, 2026-11-01: "we don't flicker in 2026 ... let's pretend we're 16-bit"). No
  blinking sprites (invincibility, pickups, warnings), no alternating-frame transparency, no multiplexing that
  drops or alternates sprites, no HUD races. Color shifts, palette ramps, border flashes and screen shakes are
  welcome: they are 16-bit polish, not 8-bit flicker (the energy wave, the border ramp, the camera shake).
- Commit only when asked.

## Commands

Host (run cargo from **PowerShell**: Git Bash's `link` shadows MSVC):

```sh
cargo test --release                     # renderer vs reference compositor, raster3d goldens
cargo check --no-default-features        # shared library still builds no_std
cargo run --release -- bake              # dist/talonhd.reu (+ music, + dist/phases) and previews, self-checked
cargo run --release -- bake --stage 2    # dist/talonhd_s2.reu (S2 TERRACES), output/frames_s2/, self-checked
cargo run --release -- preview --t 150   # attract-mode tick 150 through the model (--stage 2: the S2 image)
cargo run --release -- s2                # S2 TERRACES: output/sheets/panorama_s2.png, s2_*.png, anim_s2_*.png
cargo run --release --example queen_assembly   # output/lineup/queen.png: GANTRY QUEEN body + parts at 3 banks
cargo run --release --example frame_stats -- dist/talonhd.reu 512   # DMAs / blends / I/O writes per frame
```

Bible and models:

```sh
python tools/bible.py check                 # base bible (forked from TALON)
python tools/bible_hd.py check              # HD layer: palette law, ladder, ships, model blocks
python tools/bible_hd.py export-prompts     # assets/prompts/hd/<id>.json (Ludo create-image + create + bake)
python tools/model_prep.py <glb> assets/prompts/hd/<id>.json --forward z --up y   # -> assets/models/<id>.mesh
python tools/model_prep.py <glb> --axes     # find the GLB's forward/up first
cargo run --release -- bake-model assets/models/<id>.mesh --palette 6,12,1,3     # sheet + light share
```

C64 (rust-mos + llvm-mos in WSL `Ubuntu-24.04`, paths in `scripts/env.sh`):

```sh
pwsh scripts/build-windows.ps1           # -> dist/talonhd.prg, dist/talonhd.elf, dist/phases/*.bin (BEFORE bake)
python tools/test_blit.py                # asm sprite kernel draw_cols vs reference in a 6502 interpreter
python tools/test_sprite.py [n] [image]  # asm draw_sprite (cache, bands, chunks, clipping) vs reference, on a baked image
python tools/c64u.py bench               # with a `-Features bench` build: the C64U's unit costs (I/O write, DMA, stores)
python tools/test_hud_irq.py             # HUD raster IRQ vs the table contract, and its chain with the border painter
python tools/test_arith.py               # asm/arith.S's integer libcalls vs Python (every u8 pair, every u16 divisor),
                                         # the scratch registers each writes; --libcrt on an older ELF: libcrt's cycles
python tools/check_phases.py             # REU paging audit: resident code enters the window only at phase entries; a
                                         # phase's calls/jumps land only on its own functions (phases share addresses)
python tools/emu64.py --frames 160       # the real PRG in a 6502 + REU + $01 + raster IRQ + CIA1 timer A model
                                         # (add --painter for the canvas: the painted border is checked only then):
                                         # finds 6502-only faults (--level-dir dist: + the UCI, the per-level load;
                                         # --dump DIR: a `c64u.py dump` + the band set + the colours the border
                                         # painter stored, for `compare`: frame, bands and painted border vs the host)
pwsh scripts/build-windows.ps1 -Features start-keel   # debug: the attract game starts just before the midboss
pwsh scripts/build-windows.ps1 -Features start-boss   # debug (S2 image, bake it after): just before the ASTROLABE
pwsh scripts/build-windows.ps1 -Features start-s2     # debug: boot S1's image, load S2's at once (emu64 --level-dir)
```

Always run `tools/test_blit.py` and `tools/test_sprite.py` (after a bake) after touching `target-c64/asm/blit.S`; write
named conditional branches as `bxx 90f / jmp label / 90:` (the assembler silently mis-encodes
out-of-range branches; `tools/branch-audit.py`, from typo, runs on every C64 build and fails it), and never
add a numeric local label between a branch and the label it means (`beq 2f` then a new `2:` takes it). Hardware (`C64U_HOST` = device IP): `python tools/c64u.py config`, `upload-levels`
(both level images to `/Usb0/talonhd` over FTP, for the per-level load), `run dist/talonhd.prg`,
then `upload-reu dist/talonhd.reu` (after run: REU preload overwrites on reset), `dump dist/dump --hold`,
`cargo run --release -- compare --dump dist/dump` (it picks the image of the level the replay reached).

## Architecture

- **One renderer, two machines.** `src/render.rs` is generic over `Hw` (`src/hw.rs`); on the C64 it is volatile MMIO
  plus `asm/blit.S`, on the host `model::Machine` (byte-exact RAM + REU model). Correctness layers: `tests/render.rs`
  (model vs `reference.rs`), `tools/test_blit.py` and `tools/test_sprite.py` (asm vs Rust), `compare` (hardware vs model).
- **Contracts** (`src/format.rs`, `src/layout.rs`): REU image `TALONHD1`; panorama = `BG_ROWS` cell rows × 320 B
  top to bottom; frames pre-shifted ×4 (or ×1, drawn at `x & !3`), banded column records (below). Memory map
  as gclass except: code `$0801-$4BFF`, buffers `$4C00/$6000` and `$8C00/$A000`, HUD sprite blocks `$5000`
  (bank 1, pointers 64+) and `$8000` (bank 2, pointers 0+; never `$9000`: char ROM shadow), `SEG_BUF $C000`,
  `JOB $CA00`, sprite job `$CAA0` (`draw_sprite_asm`; `SP_HEIGHT $CAD7`), `transpose_row` job
  `$CAD8`, `STATUS $CA80`, the border painter's block `$CAE0` (`layout::painter`), `HUD_TABLE $CB00` (+ the live
  slots' `HUD_SHOWN`), the small-sprite cache `$E000-$E40F`, `CHUNK`/`OFFS` in `SEG_BUF`; the painter's staging
  `$5800-$5BFF`, decode tables `$8700-$8AFF`, code `$8600` and `$8B00`; the band sets at REU `$FFF000`. REU DMAs ≤
  256 bytes (`MAX_DMA`) so the HUD's raster IRQ always lands in its 3-line gap.
- **HUD** (`src/hud.rs`, `asm/hud.S`): 8 hires sprites multiplexed into 8 rows (64 instances) by a raster IRQ that
  reads `HUD_TABLE`. 22 sprite slots per VIC bank (`hud::block`); a few-state readout (tapes, ladders, lives,
  speed) is one LIVE slot that `render::hud_update` refreshes from its variant in the REU (`hud::variant`, 63-byte
  DMA) when the state changes, in the bank just shown (the other bank catches up when shown); the pointer table
  is static (fetched at boot) but for the gauge's 4. Only score and stage digits are drawn per frame. Run
  `tools/test_hud_irq.py` after touching `asm/hud.S`; `the_hud_table_and_live_slots_follow_the_state`.
- **The canvas** (user: "real vertical-scroller aspect ratio"; typo's border painter, `THIRD-PARTY.md`; view only:
  the sim's playfield is unchanged). A runtime option, **off by default** (user: solid 60 fps; it costs
  ~6 ms a frame and the busiest stretches drop to 30 with it): **F1** flips `status::CANVAS` (main's
  `canvas()`; `c64u.py canvas on|off`, emu64 `--painter`). Off: the same 24-row scrolled bitmap, plain
  border, no band rows, band blits or stash (`restore_background(.., bands)`), the plain HUD chain; turning
  off stops the painter and waits for the chain to settle, turning on shows one frame the old way, then
  `painter::ON`. On: 160×240, the 24-row bitmap (raster 55..246) scrolled by the hardware
  (`$D011` YSCROLL = 7 - `fine`) and 24 lines painted through `$D020` above it (block T, raster 31..54) and
  below (block B, 247..262, 0..7). `Scroll { coarse, fine }` (`render.rs`): `coarse` is the panorama row of
  bitmap row 0, sprites are drawn `fine` lines lower; 40 columns always (the shake is vertical only).
  `render::restore_background` (BANDS phase) fetches the bitmap's 25 cell rows in one chained DMA and the
  band rows -3..-1 and 25..27 into `layout::BANDS` ($9800, the window's tail: AUDIO is re-paged every
  frame); `draw_canvas` draws a sprite into the bitmap and again into each band it reaches
  (`draw_sprite_asm` takes a target and `SP_HEIGHT`, leaves `SP_H`); `stash_bands` transposes each block's 4
  cell rows line-major (`Hw::transpose_row`, `asm/bands.S`) into `SEG_BUF` and stashes the 24 lines from
  line `fine` to the buffer's set (`layout::band_set`: REU $FFF000 + $800 per buffer, T then B, 960 B each).
  Previews and `compare` show 320×240 (`model::canvas`, `band_codes`).
- **Border painter** (`asm/border.S`, `asm/hud.S`): the HUD IRQ chain is row 0 @20, the DMA gate @27, block T
  @30, rows 1..7, the gate @243, block B @246. A block waits for CIA1 timer A's underflow (one line, phased
  once by `border_sync`), spins to `TA`, re-aligns with a `$D031` toggle, trims (`DF`, 1-cycle sled) and
  delays (`DL`), then the kernel: 40 bytes from the staging ($5800, line-major, 6 lines a page), 63 cycles a
  byte, stores at +0/16/32/48 through `dec0..3` (rebuilt by `border_tables` on every palette write), the
  border colour (`painter::BORDER`, the hit flash) at the corner. The line on a badline (block T: line 17 +
  YSCROLL, none for 7; block B: line 0 for YSCROLL 7) runs `border_kbad` (62 cycles a byte, its own delays
  `DLB`/`DFB`, 71/4: the VIC's character fetches steal a cycle per 8 bytes). Then the block refills the
  staging with the other block's lines (four 240-byte DMAs; refilling in each line's idle tail made lines
  with HUD sprites in the border late) and opens the gate; block B also does the main loop's flip
  (`painter::FLIP`: bank, `$D016`, `$D011` = `PAINTER_D011` with the frame's YSCROLL, band set, together).
  Every foreground DMA waits while the gate is closed and is atomic (php/sei, all registers, trigger, plp:
  the C64U's REU registers do not read back reliably, so the IRQ never saves them). Timing code lives in the
  one-page `.ptext` ($8600): nothing in it can cross a page. Off (`painter::ON` 0) until the first frame is
  shown the old way. Calibrated on the C64U (`TA/DL/DF/AOFS/SYNC` 37/73/2/26/100, poke-able at `$CAE6`):
  every painted row pixel-exact at every YSCROLL; `LATEBLK` counts skipped blocks. The painted blocks cost
  ~3.8 ms of every frame (IRQ clock below).
- **Frame timing** (`-Features timing`, `profile`): laps into the status block (`layout::status`: RENDER,
  BACKGROUND, AUDIO, STEP, ACE, HUD, WAIT = slack before the flip; `profile`: the window's and the bands'
  sprite blits and the sprite count), all but RENDER/BACKGROUND/STEP leaving out IRQ time: `asm/hud.S`
  `irq_clock_*` adds every raster IRQ's microseconds to `status::IRQ_US` (always on). `python
  tools/timing_log.py log 180 t.csv` on the C64U, `summary t.csv` per 300 ticks; `examples/frame_census.rs`
  joins a log with the model's work; `tools/emu64.py --profile FLIP` charges cycles (plus the C64U's REU-DMA
  and I/O-write costs) to functions, phases resolved by the one paged in.
- **3D bake** (`src/raster3d.rs`, host): mesh + `Orient` (roll, pitch, yaw) + `View` → `SpriteImage`. 4×4
  supersampled z-buffer, one key light (`LIGHT`), per-face Lambert quantized in half steps along each
  material's code ramp (half step = sprite-space checkerboard), emissive materials. Units: 1 = one hires pixel.
- **Models** (`src/mesh.rs`): `.mesh` text files from `tools/model_prep.py` (Ludo GLB → paint classes by
  normalized luminance + chroma → QEM decimation to the bible's triangle count → model space, x right, y nose,
  z up, hires px). Materials carry roles and an albedo from their paint.
- **Hand models** (`src/hand.rs`): the default for TALON HD (the user prefers them to Ludo's). The GANTRY QUEEN and
  the whole S1 roster, TALON included, are hand-built; `cargo run --release -- hand-models` writes their `.mesh` files, `lineup`
  renders the review sheet, and `every_model_fits_at_every_orientation` guards against clipping. Enemy hulls:
  void/dark/light at albedo ~0.6 with a solid-dark floor (no void checker over the sea); faceted tops.
- **Sprite format and renderer** (rebuilt for 60 FPS on measured C64U costs, `docs/validation.md`: an I/O
  register write is 1.26 us = 16 RAM stores; a REU fetch ~9 us + 0.25 us/B; a CPU byte copy 0.14 us). Each phase
  is one block: band table, then bands (32 lines for tall sprites, one band otherwise) of a column-offset row
  and column records (`u8` item count; Run = line, length, inline bytes; Mix = line, bits, keep). A frame record
  is 32 B (`format::frame`). Drawing (`render::draw_sprite`, the reference; `asm/blit.S` `draw_sprite_asm` on
  the C64): small one-band blocks (bullets, shots, SPARKs) come from the RAM cache at `$E000` (8 slots, no REU
  access on a hit); others fetch the record, then per visible band its on-screen column offsets and the records,
  a `CHUNK` (1.75 KB) at a time, as chained 256-byte DMAs; the CPU kernel `draw_cols_asm` scatters runs a cell
  row at a time and blends mixes. `render::draw_sprite`/`draw_cols`, `blit.S`, `test_blit.py` and
  `test_sprite.py` change together. Profiling builds: `-Features timing` (AUDIO_US, STEP_US in the status
  block), `bench` (unit costs); `examples/frame_census.rs` joins a hardware timing log with the model's work.
- **Game** (`src/sim/`, vendored from TALON, changed only by `sim::ship`; `src/game.rs`, shared): one sim tick per frame. The
  304×176 playfield sits 8 px in and 8 lines down on the 320×192 screen; `game::scene` maps `World` to sprites
  (TALON with an eased bank, WASPs by heading, GANNETs by dive state, TWIN pairs, bullets, bursts, SPARKs),
  `game::hud_state` to the HUD. `Game::ticks` never resets (World::tick does at GAME OVER) and names the frame.
  The C64 runs ACE (`src/ace.rs`, the attract pilot) until the stick or SPACE is touched (`STATUS+28` HUMAN), so
  `compare` replays `game::replay(SHOWN_T)` on the host.
- **Boss** (`src/boss.rs`, beside the vendored `World`, stepped after it each tick): the GANTRY QUEEN arrives at
  line 4400, the scroll holds at 4560 (the step undoes the world's scroll) with `boss_ceiling` on. Phase 1:
  arms and front legs 15 HP each (TRIPLE FAN, MAST SWEEP); phase 2: rear legs snap, `hot` goes yellow
  (`game::palette`, a color-RAM fill), the cab rim takes 120 HP (RING BURST, OFFSET RINGS, TRIPLE FAN);
  then 30,000 points and a chain of bursts; at its end (user: "show it in a damaged state and have it scroll
  offscreen smoothly") she becomes a WRECK: a damaged body frame (`demo::QUEEN_WRECK_FRAME`, banked left/level/
  right, 4 phases, `hand::queen_wreck_frames`: the Lattice dead, hull scorched, plates blown out) that glides
  to the screen's centre and drifts down (easing to a line a tick) while the stage scrolls on, GONE only once
  it has left the bottom (`her_wreck_drifts_off_smoothly`). Her aim is 16-bit octant + tangent thresholds
  (`aim_finds_the_nearest_direction`: the exact nearest of 48 directions 98% of the time; the old 32-bit dot
  product managed 71%). Bullets go through
  `World::spawn_ebullet`, scores through `add_score`. `the_fight_runs_to_the_end` plays it through.
- **REU code paging** (`src/paging.rs`, as TALON's phases): rarely active code is a *phase* linked at the
  window `$9000-$9FFF`, fetched by `paging::page(id)` from REU `$FF0000` + 4 KiB × id when it is not
  current (banking I/O in if the step had it out). Only BOOT rides in the PRG; every other phase has its
  load address outside it (`link.ld` `phimg`), `tools/phases.py` (run by the build) extracts them to
  `dist/phases/` and stamps the PRG (`PHASE_STAMP`), `bake` embeds them in the level REU image (header
  `PHASES`, lengths in 256-byte pages (u8: up to 15 phases; byte 63 is the image's `LEVEL`), stamp at 34), and boot (`install_phases`) copies them to the phase bank; a stamp mismatch
  is fatal (border 8). So: build the C64 program, then bake, then `run` + `upload-reu`. Phases: `BOSS` (the fight's step logic; `Boss::sprites/palette` and `placement` stay resident),
  `ATTRACT` (ACE's `see`), `MIDBOSS` (the KEELBREAKER's step), `BOOT` (`boot_image`: image checks,
  `install_phases`, `hud_init`, `Game::reset`; `read_image`), `GROUND` (the ground units' step; paged while `Ground::busy`), `AUDIO`
  (the music producer, every frame), `TUNE` (music setup, at boot and per level; also
  `boot_irq` and the per-level load's `load_level` + `uci`), `ACE` and `MOTOR` (ACE's `think` and
  `motor`; ACE also holds the `bench` feature's `asm/bench.S`, run once after `start_music`; ATTRACT, MOTOR and ACE are each paged once every attract tick), `STAGE` (the stage clear's
  step, and at render time `Clear::sprites` while a card shows), and S2's three: `S2` (`stage2::waves`:
  placements and WASP strings, paged only while one is due), `S2GND` (`gnd2::step`, while `Ground::gnd`) and
  `S2AIR` (`air2::step`, while `Ground::air`), paged by the resident `stage2::run`; and `ASTRO` (the
  ASTROLABE's step, from `Game::step`, and its render-time `sprites`, from `game::scene`). A phase
  may call resident code only. Link with `--no-check-sections` (phases overlap by design); build with
  `-Zmerge-functions=disabled -Zno-jump-tables` (`target-c64/.cargo/config.toml`): LLVM folded identical
  `aim` functions into one copy in the BOSS phase and resident code called it while another phase was in
  the window. After every C64 build run `tools/check_phases.py`; add a phase = a `ph_*` region, a
  `.phase.*` section, its symbols in `main.rs`, its entry in `check_phases.py` `ENTRIES`. 6502 math: avoid
  i32 multiplies in per-frame code (turret aiming cost 3 ms each until it went 16-bit, in the step).
- **Stage clear and promotion** (`src/clear.rs`, `Game::clear`, paged `STAGE`; user: "Let's make the next
  stage!"): when the boss falls, `hot` greys out to the stage's own `dark` (`fx.node_dark`), waves and bullets stop, the stick is
  ignored; the ship eases to centre and climbs off the top; the stage-clear card (rank, BRANDT's line,
  QUILL's reply, BOSS TIME + CLEAN AIRFRAME counting into the score at 1,000 a tick, Fire skips; the
  tally is kept in hundreds, so a later stage's 70,000+ fits a u16, and banked an `add_score(100)` at a
  time: no 32-bit multiply); the palette walks to the next stage's colors (`fx.promotion_walk`, `clear::toward`: through the luminance
  groups' greys, void first, hot last, a step every 8 ticks) while the next airframe (the hand-built
  PEREGRINE, `hand::peregrine`) climbs in; the next stage's card; then play from
  line 0 (`start_stage`), on the next level (past the last built one, the last again in the next stage's
  palette, `clear::PALETTES`).
  Card text is baked: one frame per line (`font.rs`, a condensed 3×7 font: 6×7 on screen; lines of at most 19
  characters, inside the HUD's ladders; a `void` plate, stored unshifted),
  digits as small frames. The PEREGRINE's energy waves are one 8-step field drawn under its bank frame
  (the image had no room for 72 more frames). `the_stage_clears_and_promotes`, `examples/clear_trace.rs`.
- **Light / time of day** (`raster3d::Light`, `02-ladder.md` rung `light`): one light per stage for its sprites
  AND its level. Shading is normalised to the S1 morning sun (`MORNING`, the light every approved model used),
  so a lower sun darkens flat ground. S2 and S3 lights are approved; S4-S8 are drafts pending approval.
- **Animated levels** (user): a level is a loop of whole panorama copies (`s1::ANIM_FRAMES` 32 ×
  216 KB, `1 << ANIM_SHIFT` = 4 ticks each, a 2.1 s loop), each a full 3D render of `s1::level_at(t)` with
  its own shadow map, so moving things shade and cast shadows. The REU image starts with a frame table at
  `format::BG_TABLE` (right after the header; header `BG_MASK`, `BG_SHIFT`); `render::background_at`
  picks frame `(ticks as u8 >> shift) & mask` (a 4-byte REU read, no u32 math) before the background DMA,
  on the C64 and in `model::render_game`. Everything that moves repeats a whole number of times a loop and
  is keyed to world position or stable hashes (`the_animation_loops_without_a_seam`), never to the level's
  RNG stream (the static draws are skipped so the rest of the level is unchanged). S1: swells rolling in
  (crest noise over world position), whitecaps riding them, breakers on the sea wall and the islets
  (`breaker`: crest, burst, backwash), searchlights (two on the sea wall sweeping the sea, pairs on the
  channel quays crossing over the channel in mirror, one beside each AA pit sweeping a runway), the lighthouse
  lamp turning, the radar antenna turning, runway edge chase lights, the Lattice pulsing up the cracks and
  creeping (each vein reaches out and draws back), the MERIDIAN's wake streaming back, buoys bobbing in
  foam rings, the dock cranes' booms swinging, and the carrier MERIDIAN at the point of her wake (`meridian`:
  level geometry, deck paint and the Accord emblem, island with a turning radar, bow wave and stern churn
  streaming aft; approved, `docs/story/hd/models/s1.md` rev 10). Not yet: cloud shadows (a 2.1 s loop is too short for a
  slow drift).
  Searchlights are real lights (`raster3d::Spot`, `render_scene_lit`): cone + facing per sample, soft
  shadows traced over the lens per pixel, lifting void → dark → light. Review: `output/sheets/anim_*.png`
  (frames side by side). Budget: ~48 frames fit under the music ring at `$FE8000`; `bake` refuses an image
  that reaches it.
- **S2 TERRACES** (`src/s2.rs`, host; bible `docs/story/hd/models/s2.md`, approved with S2's afternoon light):
  built like S1 under `s2::SUN` (upper right: shadows fall lower left), palette brown/terracotta/cream/light
  blue. Contour terraces of flat-topped vine rows (a gust crosses them once a loop), dry-stone walls,
  cypress lines (swaying), farmhouses, a zig-zag track; the AQUEDUCT over its ravine (glints running along
  the channel, keyed to position; geometry must stay inside x -8..328 or the shadow map's bounds move and the
  loop seams); the hill village (hipped roofs, courtyards, laundry, the fountain plaza with ripples and
  awnings, the CAMPANILE's tower at `s2::TOWER` in its square, cracks converging on it); the observatory
  (domes with creeping shutter slits, gravel paths, the sundial, the great dome at `s2::GREAT_DOME`).
  In the game as level 1 (below).
- **Levels** (`Clear::level`: 0 S1, 1 S2; `first`/`top`): every level image has the same frame ids; the COMMON
  frames (WASP, ships, shots, bursts, SPARKs, cards) sit at their S1 ids rendered under the stage's light, and
  the stage block reuses the slots of S1's bosses (`demo::S2_*` from 0; `models::stage_frames`). Header byte
  `LEVEL` (63) names the image's level; boot calls `Game::reset_at(img.level, top)`: a C64 starts on the level
  of the image it booted and goes on to `LAST_LEVEL` when it can load the others (below), else stays on it;
  the host's `Game::new()` plays S1 then S2. `game::replay_level` (preview, compare) replays an image's
  attract mode on a C64 that loads. S2's
  waves are `stage2::PLACEMENTS` (6-byte `P2`, from the bible's 14 waves, `s2_placements_sit_on_the_level`):
  PIKEs on `air2::RAILS` (the level's vine rows, aqueduct channel, lanes and a gravel path, sampled every 32
  px; `the_rails_follow_the_level` regenerates them from `s2.rs`), KESTREL pairs, ROOF HATCHes on farmhouses
  and village roofs, BELL TURRETs, DOME MORTARs (shells lobbed at the ship's position, harmless until their
  last 20 ticks, then the sim's own shootable `B_BOMB`), S2's PILLBOX, WASP strings along the sim's own
  curves. The vendored sim's S1 track is held off (`track_base`); air units ram the ship by putting a WASP on
  it (the sim's own collision). The world's scroll wraps at `PANO_PERIOD` (`Game::step`), so the stage line
  is the scroll, never a 32-bit modulo. Music: `assets/music/cue.stage_s2.dlt`. After S2's clear the C64
  plays TERRACES again as stage 3 in S3's colors (no S3 yet).
  A game over on a later stage sets S2's fights GONE (not stepped at their hold behind the GAME OVER;
  below their lines the new game resets them, `hot` its own).
  Tests: `stage2::tests` (ACE flies S2 through both fights and the clear as the PEREGRINE: 1 death, 13,377 ticks).
- **CAMPANILE** (`src/campanile.rs`, S2 midboss; its step and render-time `sprites` live in the S2 phase, the
  sprites paged by `game::scene` only while it is shown): the tower is level geometry, the crown (8 spin
  samples, a clock-hand step every 6 ticks, every 2 winding up for a ring, reversed in phase 2) and the hub
  are sprites. The scroll holds at line 2368 (a constant store) until it falls. Phase 1: CHIME RING (8
  shards from the points) and SPOKE (a glowing point, then 5 shots). Phase 2: the hub opens and DOUBLE
  PEAL. The chime flips `Clear::pal`'s `hot` to `light` for 2 frames. HP: the bible's 56 + 24, tripled for
  TWIN (ACE took 4 s at bible HP; now 12 s; the user approved the tripling, 2026-11-03). 12,000 points, 3 SPARKs. `the_campanile_can_be_silenced`;
  the S2 bake probes the fight.
- **ASTROLABE** (`src/astrolabe.rs`, S2 boss, paged `ASTRO`: step and render-time `sprites`, 4,091 of
  4,096 B): the great dome is level geometry with its shutters shut; the fight draws the body (8 shutter
  poses), the live rings over it (16 spins each) and the lens, only while the shutters are apart
  (`shown`: `pose != 0`). The scroll holds at 4872 (dome centre on playfield line 64). TWIN bolts hit the
  first live band they fly into (squared distance, 16-bit: `part_at`). Phase 1: rings 105 HP each (the
  bible's 35, tripled as the CAMPANILE's); SIGHT LANE (rings align, dotted guide, a lane of rounds
  sweeping 96 px from the ship's outer side toward the centre, so it stays on the field), RING VOLLEY (6 from a ring's notches, turned with its spin), LENS FLARE (an 8-round
  cone; the lens open 60 ticks, down to two thirds: clamped after each tick's bolts); DOME MORTARS are the arena's own units. Phase 2 (all
  rings down): `Clear::pal`'s `hot` yellow, lens 225 HP, the shutters half-close between attacks; PINCER
  LANES, STARBURST, LENS FLARE. 40,000 points, then the shutters close; its state uses the QUEEN's numbers,
  so `Game::step` hands the clear whichever boss the level has (BOSS TIME: 40,000 within 70 s on S2).
  S2's card lines (`clear::CARD_TEXT[1]`: TERRACES' clear, LT COLONEL, S3's card) are baked into the S2
  image's card frames. `the_astrolabe_can_be_closed` (through the clear into stage 3); ACE fights it for
  ~34 s; the S2 bake probes opening, lane, volley, phase 2, fall, closing and the card. Not yet: shattered
  ring frames (a destroyed ring vanishes in bursts).
- **Ground units** (`src/ground.rs`, paged): SKIFF, PILLBOX, HATCH, CRAWLER as sprites in stage coordinates
  (world x, `u` = stage line; screen y = 200 + lines - u), from `PLACEMENTS` (the bible waves placed on
  `s1.rs` geometry; `placements_sit_on_the_level` checks quays, pits, roofs, water with `s1::surface`).
  Up to `UNITS` (10) out at once incl. wrecks; z -3 under everything. Telegraphs are armed frames (hot
  muzzles), never blinks. 16-heading sector aim (`yaw_to`, no multiplies). Resident: `sprites`, `busy` (in `.iotext`: pure, step-only),
  and the helpers `hits`/`shoot`/`launch` (`.text.resident.*` section names) and `burst` (`.hitext`), so
  the phase fits its 4 KiB window (4,039 B). `the_ground_units_fight_and_fall`, `a_hatch_launches_wasps_unless_killed`.
- **Music** (`src/audio.rs`; user: DLT5 from `../sueme64`, 16 kHz mono, Haas width): TALON's soundtrack
  (`../talon/assets/music/cue.*.m4a`) encoded by sueme64's encoder to `assets/music/<cue>.dlt` (3-bit codes,
  ~6.8 KB/s; THIRD-PARTY.md has the command); bake puts the stage cue in the level image (header `MUSIC`).
  The AUDIO phase (sueme64's 6502 kernel `asm/digital.S` + a mono player) decodes ~267 samples a frame into
  `STAGING` (`SEG_BUF`) and stashes them into ONE 8-bit ring at REU `$FE8000` (32 Ki samples); the C64U
  sampler's left voice plays it, the right voice the same ring 160 samples (10 ms) later (Haas). The
  producer paces itself by CIA 2 timer A against a boot calibration (TUNE: two muted one-shots, CIA timers
  A+B chained); state at `$C800-$C9FF`. Music is not game state, so the C64 schedules it by cost: every frame
  runs the service, but a frame whose render took over `AUDIO_HEAVY_US` (7 ms) decodes up to `HEAVY_BUDGET`
  (200 samples) and a light one up to `BUDGET` (400; ~267 a frame on average), unless the ring's lead is under
  `SAFE_LEAD` (skipping services made 4 ms catch-up spikes). No 32-bit divides on the 6502 (LLVM turns `/` + `%` into a 2.4 KB
  divmod): subtraction loops. Host: `decode_matches_the_encoder` (bit-exact vs the encoder's decode),
  `the_ring_stays_ahead`; emu64 models the sampler and checks the ring. Hardware needs C64U config "Map
  Ultimate Audio $DF20-DFFF = Enabled"; no sampler = silent (fault `NO_SAMPLER`). Next: cue changes
  (midboss, boss, stage clear are encoded), fades, sound effects.
- **ACE, the attract pilot** (`src/ace.rs`; user: "make the player look like an expert pilot even during
  attract mode", runtime GOAP on the C64). Perception (fact bits: threat, GANNET diving, SPARK, cursor,
  armed, target, aligned, station) → goal (first unmet: safe, armed, collect, clear, station) → a forward
  search over 7 actions' preconditions/effects (HOLD, EVADE, FLANK, TAKE, ARM, LINE_UP, STRAFE; cheapest
  plan of up to 3, replanned when the goal changes, a precondition breaks or every 16 ticks) → the step's
  point → the motor: all 9 stick moves scored over a 24-tick lookahead against up to 16 nearby threats'
  predicted paths (bullets in straight lines, WASPs along the heading their turn program gives them mid-lookahead,
  GANNETs by their dive, live HATCHes as zones it never loiters over: they launch WASPs), progress, and a cost
  for changing or reversing direction (~1.5 stick changes a second; on station it holds still,
  `the_ace_holds_station_calmly`). It fires only while an enemy is on screen. Expert, not perfect (user: a
  pilot that dies now and then says "I could do better than that!"). Leads targets by the bolts'
  flight; bosses publish their live weak part (`Boss::aim`, `Midboss::aim`). Three phases a tick (ATTRACT
  `see`, MOTOR `motor`, ACE `think`), sharing a `Mind` in `SEG_BUF`; its memory (`Ace`, 12 B) is in the
  `Game`, so the replay and `compare` stay exact. Time-sliced: one phase a tick (see, motor, think; a decision
  every 3 ticks, the Mind parked in the REU at `ace::MIND_REU` between ticks, but not fetched before `see`, which initialises it: nothing may carry from `think` to the next `motor`, `the_mind_needs_nothing_from_before_see`; a `coast` set in `think` made the C64 diverge from the replay). Host: `the_ace_outflies_the_autopilot`
  (16,000 ticks: 1 death, both bosses; the vendored autopilot: 10
  deaths, 3,100), `examples/ace_death.rs` (the world before any death),
  `the_ace_flies_smoothly`; `cargo run --release --example ace_trace` prints its flight log. The vendored
  `sim::replay::autopilot` stays for the sim's pinned tests.
- **Midboss** (`src/midboss.rs`, paged): the KEELBREAKER leaves its berth at line 2400 and slides up the docks
  channel; the scroll eases to 0.125 px/tick while it fights and it creeps up the screen (a line every 8
  ticks) so its later turrets come into reach. Turrets 12 HP each (BROADSIDE, WAKE MINES), then the bridge
  (36 HP, BRIDGE RING); sunk = 10,000 + 3 SPARKs; after 40 s the screen overtakes it. Hull 5 rolls, turret 16
  headings (hand-built; its deck is void/dark only so the light TALON reads over it). `the_destroyer_can_be_sunk`.
- **Airframes** (`src/sim/ship.rs`; user: "Let's make each ship unique!"): each rung's ship flies its
  bible `feel`. `ship::fly` (ACE phase, only when `of_stage(Clear::stage)` changes; `ship::BUILT` = airframes
  with models, later stages fly the last) sets `World` fields that are all zero for the TALON, so a reset
  World is the TALON and TALON's digest holds. TALON: the baseline. PEREGRINE: SPEED +64 Q8 (the bible's
  448-768), sideways velocity easing a quarter of the way to the stick's a tick (`Player::vx`; rounded
  toward zero and snapped, so it settles exactly both ways; a wall zeroes it; up/down at once), bolts 10
  px/tick, the visual bank a step every 4 ticks (`Game::step`, from `World::ship`). CARACARA (when built):
  SPEED -64, hitbox 6 x 6 (`hitbox_half`; S2's rams too), each shot two pairs 12 px apart or none
  (`World::spread`, `room`; the hit code is unchanged). ACE plans with the ship's speeds, starts a sliding
  ship's moves where its slide coasts it (`ace::coast`, 3 vx, from `think`: MOTOR is full) and leads
  10 px bolts. 6502: plain fields, no per-tick table lookups (a `Ship` struct table cost 1.5 KB); the
  sim's `fire` lives in `.hitext`, its `room` in `.iotext`. `sim::ship::tests`.
- **Invincibility** (user): no blinking; the TALON's 9 banks have 8-step energy-wave frames
  (`models::shield_frames`: waves that follow the hull's silhouette outward, `light` fading to `dark` dots).
- **Hit feedback** (user): the HUD never flashes. The border ramps black, dark grey, light grey, white, light
  red, red, light red, light grey, dark grey, black (`hud::FLASH_RAMP`, 2 frames a step); the camera shakes
  for 24 frames, vertically only (user: no 38-column squeeze; `game::SHAKE_Y`: two sines of different
  frequency and amplitude under a decaying envelope, a compile-time table, up to ±6 lines): the panorama
  window and the sprites move. The HUD sprites move the opposite way (`hud::build`'s Ys and IRQ lines).
- **Frames and panorama:** `demo.rs` is the frame directory (boss body, arms, legs, WASP yaws, TALON banks,
  GANNET pitches, shots, bursts, SPARKs) plus the boss's flight (`PART_XY0` + compile-time `PART_DX`; no trig
  on the C64). Her six parts have one frame per body bank sample (`hand::queen_part_frames`, rendered at bake
  time, no `.mesh` files): same roll as the body, posed by the bank (barrels lean in, legs trail), so nothing
  jumps; `queen_parts_fit_and_move_smoothly` guards it. `src/s1.rs` builds S1 BREAKWATER as 3D geometry (sea swells, islets with surf, the sea wall,
  quays/basins/piers, containers, tanks, cranes, runways and decals, hangars, bushes, crystal cracks) and
  renders it in strips with `raster3d::render_scene` under the stage light with cast shadows
  (`ShadowMap`, normal-offset lookups); ground materials void/dark only, `light` for surf and paint, `hot`
  for crystal (`output/sheets/panorama.png`).
- **C64 code space** (`target-c64/link.ld`, `opt-level = "s"`; `"z"` makes every region bigger): `$0801-$4BFF` plus regions copied at boot
  from one load image at `$4C00-$8FFF` (frame-buffer/HUD space, free until the first frame): `.hitext`
  `$E410-$FEFF` (HUD, boss flight, `sim::actors`, small game helpers), `.midtext` `$9000-$9FFF` (VIC sees
  char ROM, CPU RAM: `sim::collide/gauge`), `.lowtext` `$0200-$07FF` (`render`), `.uptext` `$5C00-$5FFF`
  (`sim::common/world`, `hw`), `.iotext` `$D000-$DFFF`, the RAM under the I/O chips (`sim::collide/gauge`):
  the game step runs with `$01 = $34`, so only pure code may live there, and `asm/hud.S` banks I/O in for
  itself; `.vtext` `$5600-$57FF` (`hud::live`, `level_image`), `.ptext` `$8600` and `.p2text` `$8B00` (the border
  painter), in the HUD space the live slots freed. `$9000-$9FFF` is the paging window. Placement is by mangled
  name in the link script (or `#[cfg_attr(target_arch = "mos", unsafe(link_section = ".hitext"))]`). Every
  region is nearly full: check `llvm-size -A dist/talonhd.elf` after changes. The sprite list lives in
  `.lowbss` (NOLOAD, the tail of `$0200-$07FF`); main RAM had 2 bytes free after the ground units. 6502
  size: avoid i32/u32 in our code (the vendored sim's Q8 i32 positions are its own), struct-literal resets
  of big structs, and repeated store sites (`game::put` is the scene's one sprite store). `World::digest` stays host-only
  (2 KB of 6502). `asm/mem.S` replaces compiler_builtins' 536-byte `memcpy` (a page loop, ~50 bytes; it freed the RAM the
  stage clear needed). Keep the `/DISCARD/ .init.250` rule: it calls KERNAL CHROUT at `$FFD2`, which is our code now.
- **Per-level load** (`src/uci.rs`; user chose it after S2): when the step changes `Clear::level` (a clear's
  `start_stage`, or a game over back to `first`), `main` loads that level's image file (`uci::LEVEL_FILES`,
  in `uci::LEVEL_DIR`: `TALONHD_DIR` at build time, default `/Usb0/talonhd`) before drawing anything from
  it: the Ultimate Command Interface's DOS target (`$DF1C-$DF1F`, C64U config "Command Interface"
  Enabled), `CHANGE_DIR`, `OPEN_FILE`, `LOAD_REU` at 0 capped at `ace::MIND_REU` (the Ultimate reads the file
  straight into REU memory; images never reach the Mind, the ring or the phase bank, and carry the same
  phases), `CLOSE_FILE`; `00` statuses mean the whole file arrived (no checksum: seconds at 16 MB). The
  front buffer holds the last frame (the next stage's card, or GAME OVER), the HUD runs on, the music is
  muted and restarted on the new cue (`start_music`), the sprite cache emptied (common frames differ per
  stage light). The main loop is per level: outer loop music + cache, inner frame loop while the level
  matches. Every wait is bounded (`uci::SHORT` ~2 s, `LONG` ~33 s for `LOAD_REU`: 65,536-poll units, a
  guess until measured) and a timeout ABORTs; an interface found mid-command is aborted first, a refused
  push (`ERROR`) aborted and sent once more; replies are drained packet by packet (`DATA_MORE`); paths
  and names end in a NUL. `load_image`'s outcome: `LOADED`; `UNCHANGED` (cd/open failed: nothing written,
  the game stays on the loaded level, `first`/`top` = it); `DAMAGED` (`LOAD_REU` failed: part of the file
  may be in the REU, so `load_level` sets an impossible level and `level_image` stops, border 9, as for a
  wrong level or stamp). No UCI (`uci::present`: id `$C9` at `$DF1D`): `top` is the booted level.
  `status::TOP` (byte 3) holds `Clear::top` (boot, and a fallback); `compare` replays to it
  (`game::replay_level(level, top, t)`; 0 in an older dump: no load; `preview --top N`, default
  `LAST_LEVEL`). Not replayable: a fallback mid-run from a level other than the booted one. Host:
  `model::Uci` (`chunked`, `stalled_at`, `mid_command`; `uci::tests`: load, missing file, split reply,
  timeouts, stuck interface); emu64's `Uci` with `--level-dir` answers only `--device-dir` (the PRG's
  `TALONHD_DIR`, default `/Usb0/talonhd`) and splits replies into 16-byte packets (`start-s2`: load,
  header level 1, S2's cue in the ring). The C64-side register bits come from the Ultimate's C API, not
  yet proven on the C64U; load time not yet measured.
- **Boot:** the PRG erases the REU magic, then waits for magic + whole-image checksum, so `run` then `upload-reu`
  always starts on the complete new image.
- **6502 size:** never u32/i32 math in our code (user: "massive code bloat"); a u32 `%` in `lines()` cost
  over 400 bytes of resident RAM, and one i32 divide in `demo::bank` pulled libcrt's `__divsi3` +
  `__udivsi3` (~580 bytes; now 16-bit, gone). Check `llvm-nm` for `si3` helpers after changes. Check `llvm-size -A dist/talonhd.elf` and the link map after changes.
  `asm/arith.S` replaces libcrt's u8/u16 divides, remainders and multiplies (one u8 and one u16 divide
  core, shift-and-add multiplies: 248 bytes of main RAM for libcrt's 967), linked with `-Wl,--wrap=__X`
  (`build.rs`: libcrt keeps every divide in one archive member, so defining `__X` is a duplicate symbol); a
  libcall it does not wrap (`__mulsi3`, `__divsi3`, ...) brings libcrt's back. `demo::isin`/`isin_fine` are
  16-bit (exactly the i32 forms: `sine_and_bank`), so `__mulsi3` is gone too. Run `tools/test_arith.py` after
  touching `arith.S`.
  LLVM inlines single-caller functions into `main` (resident RAM): `game::view` and `render::sort_by_z`
  are `#[inline(never)]` to stay in their regions; never pass `&mut` to main's `Image` (it moves to the
  soft stack, every field access indirect): return it by value.
- **Next:** 60 fps with the painter (S1's busy stretches and fights run at 30: see `docs/validation.md`), then
  the per-level load on the C64U (`upload-levels`, the `start-s2` build, its load time; then ACE's
  attract run through S1's clear into S2). Then S3 REDLINE (`docs/story/hd/models/s3.md`, approved): per-level
  phase slots first (15 of 15 phase ids are used, the last by BANDS; S3 needs ~4), then `src/s3.rs`, the hand models, the CARACARA
  (`ship::BUILT` = 3), its waves and fights; and animation loops for the other stages. Memory ideas from the
  sibling repos still open: `opt-level "z"`, zero page `$90-$FF`, boot-only code in a retired section,
  borrow-and-restore overlays (impact), an IRQ static-frame audit (sueme64).
- **Hardware quirks:** `c64u.py run` right after `config` can reset the machine before the PRG loads; run again.
  After a C64U restart run `config` before `run` (else the sampler is missing: `NO_SAMPLER`/`TIMEOUT`). Boot
  takes ~30 s after `upload-reu` (the whole-image checksum): poll `status` for ticks before judging a hang.
  RAM survives a reset (the startup clears the painter block: a stale gate bit hung the main loop).
