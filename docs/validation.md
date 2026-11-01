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
