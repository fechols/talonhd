# Bible file schema (TOML front matter)

Every entity file starts with `+++`, a TOML document, `+++`, then Markdown prose. Field names below are
exact; `tools/bible.py check` rejects missing required fields and illegal values. Adapted from pummel's
schema for a vertical shooter: everything is seen **from directly above**, the playfield is a
REU-streamed multicolour bitmap, and objects live on one of three layers.

Write files larger than ~6 KB with the Write tool (the Bash tool truncates long here-documents).

## Common fields (all kinds)

```toml
id = "enemy.s3.dust_devil"  # kind.scope.slug; the file name is <scope>-<slug with dashes>.md
kind = "enemy"              # world | story | ship | character | enemy | level | boss | midboss | vfx | style | ui
name = "DUST DEVIL"         # uppercase, ≤ 12 chars (stages, bosses and midbosses ≤ 16)
status = "review"           # draft | review | approved | frozen
revision = 1
```

## Layers (the key TALON concept)

```toml
layer = "sprite"   # sprite | bitmap | bucket
```

- `sprite`: drawn by the multiplexer. Size class below. Costs hardware channels on every line it covers.
- `bitmap`: ground objects fixed to the stage. Every frame and angle is composited offline over the exact
  background patch it sits on and quantized with it; the runtime DMAs cell rectangles. Costs no sprites.
  Use for turrets, tanks on fixed paths, hatches, fortress parts, wreckage, craters.
- `bucket`: tiny objects (bullets, GNAT drones, star twinkles) software-rendered each frame into shared
  24×21 "bucket" sprites; many objects cost one channel per 21-line band.

## Visual identity block (ship, character, enemy, boss, midboss, vfx)

```toml
sprite_class = "M"          # sprite layer: S=1x1 W=2x1 M=2x2 L=3x3 B=big (cells = [1-4 cols, 1-8 rows])
cells = [2, 2]              # [cols, rows]; one cell = 12x21 art px = 24x21 display px
                            # no `expanded` field: VIC-II sprite expansion ($D017/$D01D) is banned and
                            # `bible.py check` rejects expanded = true anywhere (legacy `expanded = false`
                            # lines are tolerated); bigger objects use bigger grids
cell_rect = [6, 6]          # bitmap layer instead: [cols, rows] of 8x8 display cells (4x8 art px each)
angles = 1                  # 1, 8 or 16 rotation frames generated offline from the master
anchor = "center"           # center | nose | base
identity = "…"              # ≤ 25 words, repeated verbatim in every still prompt
silhouette = "…"            # 3-6 shape keywords, read from above
proportions = "…"
materials = "…"
acting = "…"                # personality in motion; prepended to every motion prompt
forbidden = ["…"]           # never in the art
hurtbox = { x = -10, y = -10, w = 20, h = 20 }   # display px, origin = anchor

[[colors]]
role = "primary"            # primary | secondary | accent | canopy | glow | crystal …
c64 = 12                    # 0..15 (see 00-bible palette)
note = "…"
required = true             # the quantizer must keep this ink where the hue appears

[[animations]]
id = "idle"
frames = 4                  # C64 frames after curation (1..8)
ticks_per_frame = 6
loop = true
model = "hydra"             # hydra | hydra-plus | forge | offline (rotation/scroll done by our tools)
duration = 3                # seconds (hydra ≥ 3)
request_frames = 25         # Ludo --frames: 16 | 25 | 36 | 49 | 64 (≥ 2 × frames)
motion = "…"                # < 60 words, positive wording, no appearance

[ludo]
concept_prompt = "…"        # image create --image-type asset (ship, characters, bosses)
sprite_prompt = "…"         # the master still: identity + pose, no style words (prefix added)
```

## Ship (`02-characters/ship-talon.md`) and pods

As the visual block, plus:

```toml
speed_levels_q8 = [384, 448, 512, 576, 640, 704]   # px per tick × 256, level 0..5
hitbox_levels = [{ w = 4, h = 4 }, { w = 6, h = 6 }, { w = 8, h = 8 }]   # grows per arming step
private_inks = [1, 3]       # reserved on the SPRITE layer only: the shared $D025/$D026 pair may not use
                            # them and no other sprite may list them as a required identity ink. Bitmap
                            # stages may use them freely (surf, paint, Lattice crystal).
```

## Enemy

```toml
stage = "stage.s1"          # or "common"
faction = "lattice"         # lattice | captured (Accord hardware the Lattice has overgrown)
domain = "air"              # air | ground | sea | hull (on airship decks) | cable (S8 elevator)
hp = 4
contact_damage = 1          # 1 = kills an unshielded TALON
score = 200
weak_to = "burst"           # twin | flank | burst | beam | seeker | none
resists = "none"
drops = { spark = 1 }       # sparks dropped on death (leaders and carriers)

[behavior]
archetype = "swooper"       # swooper | diver | strafer | rail_runner | orbiter | carrier | splitter | drifter |
                            # gunship | turret | tank | hatch | mortar | emplacement | tether_node | hazard | segment
params = { speed_q8 = 512, curve = "s_left", formation = 6, spacing_ticks = 8 }
attack = { kind = "aimed", count = 1, spread_deg = 0, speed_q8 = 384, telegraph_ticks = 12, cooldown_ticks = 90 }
                            # kind: none | aimed | fixed | spread | ring | burst | lob | beam | ram | launch
placement = "…"
sprite_budget = { channels_on_line = 2, bullets_max = 2 }   # sprite layer only
death_vfx = "vfx.explosion_m"
```

Markdown body: `# Concept`, `# Behaviour pattern`, `# Placement rules`, `# Readability`, `## Approval`.

## Level

```toml
slug = "breakwater"
stage_index = 1
title = "COASTAL AIRBASE AT DAWN"
theme = "…"
length_screens = 26         # 200-line screens of travel at the base scroll speed
scroll_q8 = 128             # px per tick × 256 (128 = 0.5 px/tick)
target_minutes = 3
midboss = "midboss.s1.keelbreaker"
boss = "boss.s1.gantry_queen"
enemies = ["enemy.common.wasp", "enemy.s1.skiff"]
checkpoints = [7, 14, 21]   # screen numbers
music = { cue = "cue.stage_s1", mood = "…", bpm = 152, length_s = 150, loop = true }
background_ink = 6          # $D021 for the whole stage (the bitmap's shared colour)

[[sections]]
n = 1
name = "OPEN SEA"
screens = [0, 8]            # [first, last) screen
description = "60-100 words of concrete nouns, materials, light and scale for tile prompts"
inks_intent = "blue sea, white surf, grey hull"
beat = "what happens here"
teaches = "what the player learns"
enemies = ["enemy.common.wasp"]

[[transitions]]
section = 2
set_piece = "…"
asset = "setpiece.s1.sea_wall"

[[waves]]                   # sketch, not final placement (levels are hand-authored later)
screen = 1
what = "6 WASPs, S-curve from left, leader drops a spark"

[[tiles]]
id = "tileset.s1.sea"
section = 1
image_type = "texture"      # texture | tile | sprite-tiling-vertical | sprite | parallax_layer;
                            # setpiece.* ids, and only they, use vertical_scrolling_background
subject = "…"
size_cells = [40, 25]
seamless = "v"              # h | v | both | none
```

Markdown body: `# Setting`, `# Story beat`, `# Section beats`, `# Transition set piece`, `# Midboss and
boss arena`, `# Music brief`, `## Approval`.

## Boss and midboss

```toml
title = "the recrystallised control tower"
stage = "stage.s1"
layer = "sprite"            # sprite | bitmap | hybrid
hp = 120
weak_to = "burst"
score = 20000
time_bonus = { par_s = 60, max = 30000 }

[[parts]]                   # hybrid bosses list every destroyable part
id = "core"
layer = "sprite"
sprite_class = "B"
cells = [4, 6]
hp = 60

[[phases]]
from_hp = 120
to_hp = 60
patterns = ["p1_spread", "p1_launch"]

[[occupancy]]               # sprite bosses: one string per 21-line band, "1" = a channel, "0" or " " = none
pose = "phase1"
rows = ["1111", "1111", "0110"]   # each row + the 2 bullet buckets must fit in 8 channels
note = "…"

[[patterns]]
id = "p1_spread"
name = "TRIPLE FAN"
telegraph = { ticks = 24, cue = "glow" }
attack = { kind = "spread", count = 3, spread_deg = 30, speed_q8 = 384 }
sprite_budget = { boss_channels = 4, bullets_max = 12 }
counterplay = "…"
```

Markdown body: `# Personality and role`, `# Arena`, `# Phases and patterns`, `# Budget notes`, `## Approval`.

## VFX (`08-vfx.md`, one file)

```toml
[[vfx]]
id = "vfx.explosion_m"
name = "BLAST M"
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
frames = 6
ticks_per_frame = 3
loop = false
identity = "…"
colors = [7, 8, 1]
prompt = "…"                # vfx_prefix is added
```
