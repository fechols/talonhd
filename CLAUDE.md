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
- Commit only when asked.

## Commands

Host (run cargo from **PowerShell**: Git Bash's `link` shadows MSVC):

```sh
cargo test --release                     # renderer vs reference compositor, raster3d goldens
cargo check --no-default-features        # shared library still builds no_std
cargo run --release -- bake              # dist/talonhd.reu + output/{sheets,frames}/*.png, self-checked
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
pwsh scripts/build-windows.ps1           # -> dist/talonhd.prg, dist/talonhd.elf
python tools/test_blit.py                # asm column kernel vs reference in a 6502 interpreter
python tools/test_hud_irq.py             # HUD raster IRQ vs the table contract
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
  `compare` replays `game::replay(SHOWN_T)` on the host. The GANTRY QUEEN flies over the airfield
  (`demo::queen`, lines 4400-5200) as scenery: not fought yet.
- **Frames and panorama:** `demo.rs` is the frame directory (boss body, arms, legs, WASP yaws, TALON banks,
  GANNET pitches, shots, bursts, SPARKs) plus the boss's flight (`PART_XY0` + compile-time `PART_DX`; no trig
  on the C64). Her six parts have one frame per body bank sample (`hand::queen_part_frames`, rendered at bake
  time, no `.mesh` files): same roll as the body, posed by the bank (barrels lean in, legs trail), so nothing
  jumps; `queen_parts_fit_and_move_smoothly` guards it. `src/s1.rs` paints the S1 BREAKWATER panorama (26 screens; ground void/dark only, flat concrete
  50 % checker, open ground 25 %, `light` for surf and paint, `hot` for crystal; `output/sheets/panorama.png`).
- **C64 code space** (`target-c64/link.ld`, `opt-level = "s"`): `$0801-$4BFF` plus three regions copied at boot
  from one load image at `$4C00-$7BFF` (frame-buffer space, free until the first frame): `.hitext`
  `$E500-$FEFF` (HUD, boss, `sim::actors`), `.midtext` `$9000-$9FFF` (VIC sees char ROM, CPU RAM:
  `sim::collide/common/world`), `.lowtext` `$0200-$07FF` (`sim::gauge`, `render`). Placement is by mangled
  name in the link script (or `#[cfg_attr(target_arch = "mos", unsafe(link_section = ".hitext"))]`). Every
  region is nearly full: check `llvm-size -A dist/talonhd.elf` after changes. `World::digest` stays host-only
  (2 KB of 6502). Keep the `/DISCARD/ .init.250` rule: it calls KERNAL CHROUT at `$FFD2`, which is our code now.
- **Boot:** the PRG erases the REU magic, then waits for magic + whole-image checksum, so `run` then `upload-reu`
  always starts on the complete new image.
- **Next:** fight the GANTRY QUEEN and the KEELBREAKER (sim runners for the S1 waves TALON has no runner for:
  SKIFF, PILLBOX, HATCH, CRAWLER), the promotion sequence and the per-level REU load. More code will need REU
  code paging (as TALON) or a leaner memory map.
- **Hardware quirk:** `c64u.py run` right after `config` can reset the machine before the PRG loads; run again.
