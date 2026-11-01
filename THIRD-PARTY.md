# Third-party and vendored code

Code copied from sibling projects (same author). The source repositories are never written to from here.

| Source | Commit | Files here | Changes |
|---|---|---|---|
| `../gclass` | `a5775bf` | `src/{format,hw,layout,render,encode,model,reference,palette}.rs`, `src/bake.rs` (allocator, frame layout), `tests/render.rs`, `examples/frame_stats.rs`, `target-c64/**`, `scripts/*`, `tools/{c64u,native_cpu,test_blit}.py`, `config/talonhd.cfg` (= `gclass.cfg`) | Vertical panorama (one contiguous 25-row DMA, `$D011` YSCROLL, 24-row mode); all four multicolor codes global (`Palette`, `set_palette`); per-frame phase count (1 or 4); magic `TALONHD1`; no `image` dependency (own PNG writer); renamed. |

Planned (not yet copied): `../talon` `95179bb` (simulation, level format, bible tooling), `../sueme64`
(REU digital music).
