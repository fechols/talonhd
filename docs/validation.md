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
