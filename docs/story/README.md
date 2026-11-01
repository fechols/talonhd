# The TALON design bible

Everything the game shows, says, draws or plays starts here. These files are written by Claude as game
and level designer and approved by the user. No Ludo credit is spent on an entity whose file is not
`status = "approved"`, and no prompt is typed by hand: `python tools/bible.py export-prompts` compiles
prompts from the fields below, prefixed with the global style strings in `06-ludo-style-guide.md`.
The format, lifecycle and tooling follow pummel's bible (`../pummel/docs/story`, commit 66b240c).

## Files

| File | Holds |
|---|---|
| `00-bible.md` | premise, tone, era rules, factions, world rules, glossary, palette, hardware facts, IP rules |
| `01-story.md` | backstory, inciting incident, stage cards, twist, final arc, ending, every text screen |
| `02-characters/*.md` | the XF-9 TALON (ship, pods, escorts), QUILL, Marshal Brandt, the LATTICE |
| `03-enemies/*.md` | common air and ground enemies (`common-*`) and stage enemies (`s1-*` … `s8-*`) |
| `04-levels/*.md` | stages `s1`-`s8`: biome sections, transition set-piece, checkpoints, waves, music brief |
| `05-bosses/*.md` | eight stage bosses and eight midbosses; `05-bosses/README.md` holds the roster table |
| `06-ludo-style-guide.md` | global prompt prefixes (top-down), framing rules per size class, review checklist |
| `07-ui-text.md` | every on-screen string, 38 columns, uppercase |
| `08-vfx.md` | player shots per weapon and level, enemy bullets, explosions, sparks, pickups |
| `forbidden-terms.txt` | words that may never appear in any story file or prompt |
| `../design.md` | game design spec: units, movement, gauge, weapons, scoring, rank, loops |
| `../music-storyboard.txt` | per-cue mood brief and copy-ready Suno prompts |
| `../sfx-design.txt` | the four-SID effect palette |

## Lifecycle

`draft` (Claude writing) → `review` (ready for the user) → `approved` (prompts may be exported) →
`frozen` (art generated from this revision; changes bump `revision`, which marks sheets stale).

## ID scheme

`world`, `story`, `ship.talon`, `pod.wing`, `pod.escort`, `char.quill`, `char.brandt`, `char.lattice`,
`enemy.common.<slug>`, `enemy.s3.<slug>`, `stage.s1` … `stage.s8`, `boss.s1.<slug>`, `midboss.s1.<slug>`,
`tileset.s1.<slug>`, `setpiece.s1.<slug>`, `vfx.<slug>`, `cue.<slug>`.

File names drop the kind prefix: `03-enemies/s3-dust-devil.md` holds `enemy.s3.dust_devil`.

## Approval record

Every file ends with an `## Approval` section: date, approver, decision, and the `catalog.json` hash that
was exported from it. `python tools/bible.py check` fails on schema errors, unknown IDs, illegal palette
indices, sprite-budget violations and forbidden terms.
