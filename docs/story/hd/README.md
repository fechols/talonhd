# The TALON HD bible layer

TALON HD keeps TALON's approved bible (`docs/story`, forked from `../talon` at `95179bb`): the world, the
story, the stages, every enemy and boss, their behaviour and their names. This folder is the **HD layer**
on top of it. Where the two disagree, the HD layer wins. It covers:

| File | Holds |
|---|---|
| `00-palette-law.md` | four global colors, roles, value order, per-stage palette rules, polygon rules, size classes |
| `01-art-direction.md` | the look and the tricks, written by Claude as TALON HD's art director |
| `02-ladder.md` | the promotion ladder: eight stages, eight palettes, eight ranks, eight ships |
| `ships/ship-*.md` | the eight airframes QUILL flies, one per stage, with their `[model]` blocks |
| `models/s1.md` | `[[models]]` blocks for every S1 enemy, midboss and boss |

What the HD layer **replaces** in the base bible:
- **Layers.** `sprite` / `bitmap` / `bucket`, cell grids, `sprite_class`, `cells`, `occupancy`, `private_inks`,
  the shared `$D025/$D026` pair and every sprite-budget field are hardware-sprite concepts. In HD every
  object is a software sprite rendered from a 3D model; budgets are milliseconds and REU bytes.
- **Colors.** `[[colors]]` C64 indices and stage `inks_intent` become *intent* only. The palette a
  stage actually shows is its `02-ladder.md` palette, and an entity's colors are **roles**.
- **Art source.** `[ludo]` 2D sprite prompts become `[model]` 3D prompts (`ludo model create-image` →
  `ludo model create`).

What it **adds**: the promotion arc. QUILL is promoted at every stage clear and handed the Accord's next
prototype, in that stage's colors.

Lifecycle, IDs and approval records follow the base bible. `python tools/bible_hd.py check` validates this
folder, the palette law and the cross-references into the base bible; `export-prompts` compiles the
Ludo model requests.
