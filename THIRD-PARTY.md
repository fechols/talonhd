# Third-party and vendored code

Code copied from sibling projects (same author). The source repositories are never written to from here.

| Source | Commit | Files here | Changes |
|---|---|---|---|
| `../gclass` | `a5775bf` | `src/{format,hw,layout,render,encode,model,reference,palette}.rs`, `src/bake.rs` (allocator, frame layout), `tests/render.rs`, `examples/frame_stats.rs`, `target-c64/**`, `scripts/*`, `tools/{c64u,native_cpu,test_blit}.py`, `config/talonhd.cfg` (= `gclass.cfg`) | Vertical panorama (one contiguous 25-row DMA, `$D011` YSCROLL, 24-row mode); all four multicolor codes global (`Palette`, `set_palette`); per-frame phase count (1 or 4); magic `TALONHD1`; no `image` dependency (own PNG writer); renamed. |
| `../talon` | `95179bb` | `docs/story/**` (except `docs/story/hd/`), `docs/design.md`, `docs/level-format.md`, `docs/music-storyboard.txt`, `tools/bible.py`, `tools/tests/test_bible.py` | The base bible is unchanged; `bible.py` skips `docs/story/hd/` (checked by `tools/bible_hd.py`). |
| `../talon` | `95179bb` | `src/sim/**` (world, player, actors, collide, gauge, common, track, `track_s1.rs`, digest, replay, tests) | Unchanged code (the module doc notes the vendoring); TALON's replay digest `0x780fe9b1` still holds. TALON HD maps it to the screen in `src/game.rs`; the C64 link script places its modules in the relocated code regions by mangled name. |

| `../sueme64` | `7eab801` | `target-c64/asm/digital.S` (the DLT5 decode kernel `digital_decode_run`) | Unchanged but for its header and section name. MIT. The DLT5 format, the sampler register use (`$DF20/$DF40`, the right voice's 10 ms lag on the same ring) and the calibration method follow its `target-c64/src/digital.rs`; TALON HD's mono-only player (`src/audio.rs`) is new code. |
| `../sueme64` | `7eab801` | (tool, not copied) `tools/encode-digital-music` | Run from its own manifest with `CARGO_TARGET_DIR=target/encoder` (writes nothing there): `encode-digital-music --profile mono16k3 --codebooks <seconds / 0.02092> --training combined --beam-width 128 <in> assets/music/<cue>.dlt output/audio/<cue>.preview.wav`. Unlicense; needs ffmpeg. |
| `../talon` | `95179bb` | `assets/music/*.dlt` (encoded from `assets/music/cue.*.m4a`) | TALON's soundtrack (same author), encoded to DLT5 16 kHz mono. |

Planned (not yet copied): `../talon` level tooling (`stage_build.py`).
