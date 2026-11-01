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
cargo run --release -- preview --t 150   # one demo frame through the model
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
- **Hand models** (`src/hand.rs`): geometry the art director builds when a Ludo model reads badly from straight
  above (bible `generator = "hand"`); `cargo run --release -- hand-models` writes their `.mesh` files. The GANTRY
  QUEEN is one.
- **Sprite format:** column segments are Run, Mix or Skip (`SEG_SKIP`: subtract a gap from DATA, so holes cost no
  REU bytes). The Rust `draw_column`, `asm/blit.S` and `tools/test_blit.py` change together.
- **Tech scene** (`src/demo.rs`, shared, integer-only; `src/models.rs`, host): S1 palette; the Ludo TALON and the
  hand-built GANTRY QUEEN (`assets/models/*.mesh`) when present, 49 roll frames, fine sine sway; procedural
  stand-ins (not game content) for the drones, the panorama and, as a fallback, the boss.
- **Boot:** the PRG erases the REU magic, then waits for magic + whole-image checksum, so `run` then `upload-reu`
  always starts on the complete new image.
- **Next:** the rest of the S1 roster from Ludo (WASP, GANNET, SKIFF, PILLBOX, HATCH, CRAWLER, KEELBREAKER),
  the GANTRY QUEEN's separate legs/arms parts, then the TALON simulation.
- **Hardware quirk:** `c64u.py run` right after `config` can reset the machine before the PRG loads; run again.
