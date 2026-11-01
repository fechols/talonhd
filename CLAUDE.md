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
cargo run --release -- preview --t 150   # attract-mode tick 150 through the model
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
python tools/test_blit.py                # asm column kernel vs reference in a 6502 interpreter
python tools/test_hud_irq.py             # HUD raster IRQ vs the table contract
python tools/check_phases.py             # REU paging audit: no resident call into the window but phase entries
python tools/emu64.py --frames 160       # the real PRG in a 6502 + REU + $01 model: finds 6502-only faults
pwsh scripts/build-windows.ps1 -Features start-keel   # debug: the attract game starts just before the midboss
```

Always run `tools/test_blit.py` after touching `target-c64/asm/blit.S` (the assembler silently mis-encodes
out-of-range branches). Hardware (`C64U_HOST` = device IP): `python tools/c64u.py config`, `run dist/talonhd.prg`,
then `upload-reu dist/talonhd.reu` (after run: REU preload overwrites on reset), `dump dist/dump --hold`,
`cargo run --release -- compare --dump dist/dump`.

## Architecture

- **One renderer, two machines.** `src/render.rs` is generic over `Hw` (`src/hw.rs`); on the C64 it is volatile MMIO
  plus `asm/blit.S`, on the host `model::Machine` (byte-exact RAM + REU model). Correctness layers: `tests/render.rs`
  (model vs `reference.rs`), `tools/test_blit.py` (asm vs Rust kernel), `compare` (hardware vs model).
- **Contracts** (`src/format.rs`, `src/layout.rs`): REU image `TALONHD1`; panorama = `BG_ROWS` cell rows × 320 B
  top to bottom; frames pre-shifted ×4 (or ×1, drawn at `x & !3`), column-major with segment lists. Memory map
  as gclass except: code `$0801-$4BFF`, buffers `$4C00/$6000` and `$8C00/$A000`, HUD sprite blocks `$5000`
  (bank 1, pointers 64+) and `$8000` (bank 2, pointers 0+; never `$9000`: char ROM shadow), `SEG_BUF $C000`,
  `JOB $CA00`, `STATUS $CA80`, `HUD_TABLE $CB00`, `COL_TABLE $E000`. REU DMAs ≤ 512 bytes (`MAX_DMA`) so the
  HUD's raster IRQ always lands in its 3-line gap.
- **HUD** (`src/hud.rs`, `asm/hud.S`): 8 hires sprites multiplexed into 8 rows (64 instances) by a raster IRQ that
  reads `HUD_TABLE`; static art and few-state readouts are baked blocks chosen by pointer, only score and
  stage digits are written per frame. Damage flash white → red → green via `$D027-$D02E`. Run
  `tools/test_hud_irq.py` after touching `asm/hud.S`.
- **Vertical scroll.** `Scroll { coarse, fine }`: 25 rows DMA'd from row `coarse`; 24-row mode with YSCROLL
  `7 - fine` shows bitmap lines `fine..fine+192`; sprites are drawn `fine` lines lower.
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
- **Sprite format:** column segments are Run, Mix or Skip (`SEG_SKIP`: subtract a gap from DATA, so holes cost no
  REU bytes). The Rust `draw_column`, `asm/blit.S` and `tools/test_blit.py` change together.
- **Game** (`src/sim/`, vendored unchanged from TALON; `src/game.rs`, shared): one sim tick per frame. The
  304×176 playfield sits 8 px in and 8 lines down on the 320×192 screen; `game::scene` maps `World` to sprites
  (TALON with an eased bank, WASPs by heading, GANNETs by dive state, TWIN pairs, bullets, bursts, SPARKs),
  `game::hud_state` to the HUD. `Game::ticks` never resets (World::tick does at GAME OVER) and names the frame.
  The C64 runs the sim's autopilot (attract mode) until the stick or SPACE is touched (`STATUS+28` HUMAN), so
  `compare` replays `game::replay(SHOWN_T)` on the host.
- **Boss** (`src/boss.rs`, beside the vendored `World`, stepped after it each tick): the GANTRY QUEEN arrives at
  line 4400, the scroll holds at 4560 (the step undoes the world's scroll) with `boss_ceiling` on. Phase 1:
  arms and front legs 15 HP each (TRIPLE FAN, MAST SWEEP); phase 2: rear legs snap, `hot` goes yellow
  (`game::palette`, a color-RAM fill), the cab rim takes 120 HP (RING BURST, OFFSET RINGS, TRIPLE FAN);
  then 30,000 points, a chain of bursts, she sinks away and the stage scrolls on. Bullets go through
  `World::spawn_ebullet`, scores through `add_score`. `the_fight_runs_to_the_end` plays it through.
- **REU code paging** (`src/paging.rs`, as TALON's phases): rarely active code is a *phase* linked at the
  window `$9000-$9FFF`, fetched by `paging::page(id)` from REU `$FF0000` + 4 KiB × id when it is not
  current (banking I/O in if the step had it out). Only BOOT rides in the PRG; every other phase has its
  load address outside it (`link.ld` `phimg`), `tools/phases.py` (run by the build) extracts them to
  `dist/phases/` and stamps the PRG (`PHASE_STAMP`), `bake` embeds them in the level REU image (header
  `PHASES`, lengths, stamp), and boot (`install_phases`) copies them to the phase bank; a stamp mismatch
  is fatal (border 8). So: build the C64 program, then bake, then `run` + `upload-reu`. Phases: `BOSS` (the fight's step logic; `Boss::sprites/palette` and `placement` stay resident),
  `ATTRACT` (the autopilot), `MIDBOSS` (the KEELBREAKER's step), `BOOT` (`hud_init`, image checks,
  `install_phases`, `Game::reset`), `GROUND` (the ground units' step; paged while `Ground::busy`), `AUDIO`
  (the music producer, every frame), `TUNE` (music setup, once). A phase
  may call resident code only. Link with `--no-check-sections` (phases overlap by design); build with
  `-Zmerge-functions=disabled -Zno-jump-tables` (`target-c64/.cargo/config.toml`): LLVM folded identical
  `aim` functions into one copy in the BOSS phase and resident code called it while another phase was in
  the window. After every C64 build run `tools/check_phases.py`; add a phase = a `ph_*` region, a
  `.phase.*` section, its symbols in `main.rs`, its entry in `check_phases.py` `ENTRIES`. 6502 math: avoid
  i32 multiplies in per-frame code (turret aiming cost 3 ms each until it went 16-bit, in the step).
- **Light / time of day** (`raster3d::Light`, `02-ladder.md` rung `light`): one light per stage for its sprites
  AND its level. Shading is normalised to the S1 morning sun (`MORNING`, the light every approved model used),
  so a lower sun darkens flat ground. S2-S8 lights are drafts pending approval. Animated levels (user idea):
  N full panorama copies, frame `t mod N` picked by the background DMA (no extra per-frame cost; S1 is
  216 KB per copy).
- **Ground units** (`src/ground.rs`, paged): SKIFF, PILLBOX, HATCH, CRAWLER as sprites in stage coordinates
  (world x, `u` = stage line; screen y = 200 + lines - u), from `PLACEMENTS` (the bible waves placed on
  `s1.rs` geometry; `placements_sit_on_the_level` checks quays, pits, roofs, water with `s1::surface`).
  Up to `UNITS` (10) out at once incl. wrecks; z -3 under everything. Telegraphs are armed frames (hot
  muzzles), never blinks. 16-heading sector aim (`yaw_to`, no multiplies). Resident: `sprites`, `busy`,
  and the helpers `hits`/`shoot`/`launch` (`.text.resident.*` section names) and `burst` (`.hitext`), so
  the phase fits its 4 KiB window (4,039 B). `the_ground_units_fight_and_fall`, `a_hatch_launches_wasps_unless_killed`.
- **Music** (`src/audio.rs`; user: DLT5 from `../sueme64`, 16 kHz mono, Haas width): TALON's soundtrack
  (`../talon/assets/music/cue.*.m4a`) encoded by sueme64's encoder to `assets/music/<cue>.dlt` (3-bit codes,
  ~6.8 KB/s; THIRD-PARTY.md has the command); bake puts the stage cue in the level image (header `MUSIC`).
  The AUDIO phase (sueme64's 6502 kernel `asm/digital.S` + a mono player) decodes ~267 samples a frame into
  `STAGING` (`SEG_BUF`) and stashes them into ONE 8-bit ring at REU `$FE8000` (32 Ki samples); the C64U
  sampler's left voice plays it, the right voice the same ring 160 samples (10 ms) later (Haas). The
  producer paces itself by CIA 2 timer A against a boot calibration (TUNE: two muted one-shots, CIA timers
  A+B chained); state at `$C800-$C9FF`. No 32-bit divides on the 6502 (LLVM turns `/` + `%` into a 2.4 KB
  divmod): subtraction loops. Host: `decode_matches_the_encoder` (bit-exact vs the encoder's decode),
  `the_ring_stays_ahead`; emu64 models the sampler and checks the ring. Hardware needs C64U config "Map
  Ultimate Audio $DF20-DFFF = Enabled"; no sampler = silent (fault `NO_SAMPLER`). Next: cue changes
  (midboss, boss, stage clear are encoded), fades, sound effects.
- **Midboss** (`src/midboss.rs`, paged): the KEELBREAKER leaves its berth at line 2400 and slides up the docks
  channel; the scroll eases to 0.125 px/tick while it fights and it creeps up the screen (a line every 8
  ticks) so its later turrets come into reach. Turrets 12 HP each (BROADSIDE, WAKE MINES), then the bridge
  (36 HP, BRIDGE RING); sunk = 10,000 + 3 SPARKs; after 40 s the screen overtakes it. Hull 5 rolls, turret 16
  headings (hand-built; its deck is void/dark only so the light TALON reads over it). `the_destroyer_can_be_sunk`.
- **Invincibility** (user): no blinking; the TALON's 9 banks have 8-step energy-wave frames
  (`models::shield_frames`: waves that follow the hull's silhouette outward, `light` fading to `dark` dots).
- **Hit feedback** (user): the HUD never flashes. The border ramps black, dark grey, light grey, white, light
  red, red, light red, light grey, dark grey, black (`hud::FLASH_RAMP`, 2 frames a step); the camera shakes
  for 24 frames (`game::SHAKE_X/Y`: per axis two sines of different frequency and amplitude under a
  decaying envelope, compile-time tables, up to ±12 px / ±6 lines). Vertically the panorama window and
  sprites move; sideways whole 8-px columns shift the background copy (`Scroll::col`, wrapped-in columns
  blanked) and the sprites, and XSCROLL (38-column mode, rest 4) does the rest. The HUD sprites move the
  opposite way (`hud::xs`, `hud::build`'s Ys and IRQ lines).
- **Frames and panorama:** `demo.rs` is the frame directory (boss body, arms, legs, WASP yaws, TALON banks,
  GANNET pitches, shots, bursts, SPARKs) plus the boss's flight (`PART_XY0` + compile-time `PART_DX`; no trig
  on the C64). Her six parts have one frame per body bank sample (`hand::queen_part_frames`, rendered at bake
  time, no `.mesh` files): same roll as the body, posed by the bank (barrels lean in, legs trail), so nothing
  jumps; `queen_parts_fit_and_move_smoothly` guards it. `src/s1.rs` builds S1 BREAKWATER as 3D geometry (sea swells, islets with surf, the sea wall,
  quays/basins/piers, containers, tanks, cranes, runways and decals, hangars, bushes, crystal cracks) and
  renders it in strips with `raster3d::render_scene` under the stage light with cast shadows
  (`ShadowMap`, normal-offset lookups); ground materials void/dark only, `light` for surf and paint, `hot`
  for crystal (`output/sheets/panorama.png`).
- **C64 code space** (`target-c64/link.ld`, `opt-level = "s"`): `$0801-$4BFF` plus five regions copied at boot
  from one load image at `$4C00-$8FFF` (frame-buffer/HUD space, free until the first frame): `.hitext`
  `$E410-$FEFF` (HUD, boss flight, `sim::actors`, small game helpers), `.midtext` `$9000-$9FFF` (VIC sees
  char ROM, CPU RAM: `sim::collide/gauge`), `.lowtext` `$0200-$07FF` (`render`), `.uptext` `$5C00-$5FFF`
  (`sim::common/world`, `hw`), `.iotext` `$D000-$DFFF`, the RAM under the I/O chips (`sim::collide/gauge`):
  the game step runs with `$01 = $34`, so only pure code may live there, and `asm/hud.S` banks I/O in for
  itself. `$9000-$9FFF` is the paging window. Placement is by mangled
  name in the link script (or `#[cfg_attr(target_arch = "mos", unsafe(link_section = ".hitext"))]`). Every
  region is nearly full: check `llvm-size -A dist/talonhd.elf` after changes. The sprite list lives in
  `.lowbss` (NOLOAD, the tail of `$0200-$07FF`); main RAM had 2 bytes free after the ground units. 6502
  size: avoid i32/u32 in our code (the vendored sim's Q8 i32 positions are its own), struct-literal resets
  of big structs, and repeated store sites (`game::put` is the scene's one sprite store). `World::digest` stays host-only
  (2 KB of 6502). Keep the `/DISCARD/ .init.250` rule: it calls KERNAL CHROUT at `$FFD2`, which is our code now.
- **Boot:** the PRG erases the REU magic, then waits for magic + whole-image checksum, so `run` then `upload-reu`
  always starts on the complete new image.
- **Next:** the promotion sequence and the per-level REU load (it must not overwrite the phase bank `$FF0000`). Then animated level frames (rolling
  swells, searchlights, pulsing crystal). Memory ideas from the
  sibling repos still open: `opt-level "z"`, zero page `$90-$FF`, boot-only code in a retired section,
  borrow-and-restore overlays (impact), an IRQ static-frame audit (sueme64).
- **Hardware quirk:** `c64u.py run` right after `config` can reset the machine before the PRG loads; run again.
