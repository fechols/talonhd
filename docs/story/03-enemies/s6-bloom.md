+++
id = "enemy.s6.bloom"
kind = "enemy"
name = "BLOOM"
stage = "stage.s6"
faction = "lattice"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [5, 5]
angles = 1
anchor = "center"
identity = "a large crystal flower seen from directly above: six faceted light blue triangular petals around a purple hexagonal core"
silhouette = "closed: hexagonal bud; open: six-point star of triangular petals around a hexagon"
proportions = "open petals fill the patch; core a quarter of the open diameter; closed bud half the diameter"
materials = "light blue faceted crystal petals, purple core, dark grey facet edges over jungle green"
acting = "sits closed and glittering, unfolds its petals in a slow bloom, fires a ring, folds shut"
forbidden = ["organic petals, stems or leaves", "eyes or a face", "a mouth or teeth", "real flower species", "religious symbols"]
hurtbox = { x = -12, y = -12, w = 24, h = 24 }
domain = "ground"
hp = 6
contact_damage = 0
score = 500
weak_to = "beam"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "emplacement"
params = { closed_ticks = 120, open_frames = 4, open_hold_ticks = 50, closed_damage_scale = 0.25, beam_open_bonus = 1500, fire_range_lines = 160 }
attack = { kind = "ring", count = 8, spread_deg = 45, speed_q8 = 288, telegraph_ticks = 24, cooldown_ticks = 170 }
placement = "S6 canopy clearings and temple terraces; singly or in beds of 3; at most 4 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 8 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 14
note = "light blue petals"
required = true

[[colors]]
role = "core"
c64 = 4
note = "purple core"
required = true

[[colors]]
role = "edge"
c64 = 11
note = "dark grey facet edges and shadow"
required = false

[[colors]]
role = "glint"
c64 = 1
note = "white glints on petal tips when open (non-required)"
required = false

[[animations]]
id = "open"
frames = 4
ticks_per_frame = 6
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "a pointed crystal bud unfolds its six faceted petals outward until they lie flat, the core blazing"

[[animations]]
id = "close"
frames = 4
ticks_per_frame = 6
loop = false
model = "offline"
motion = "the open frames reversed"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the flower shattered flat into dull dark crystal fragments on the ground"

[ludo]
sprite_prompt = "a large crystal flower seen from directly above: six faceted light blue triangular petals around a purple hexagonal core, petals opened flat, top-down view"
+++

# BLOOM

## Concept

The Lattice's jungle growth at its most beautiful and wrong: huge crystal flowers that open to fire. A
BLOOM is armoured while shut (quarter damage) and soft while open, and **killing it with BEAM while it is
open pays a 1,500 bonus**. BLOOMs teach **the open window and the risky reward**: the bonus means
facing the ring at the exact moment it fires.

## Behaviour pattern

- Closed for 120 ticks (quarter damage from everything).
- Opens over 24 ticks (the telegraph), holds open 50 ticks: at the start of the hold fires an 8-way ring
  at 1.125 px/tick, each ring rotated 22.5 degrees from the last.
- Closes over 24 ticks. Cycle length 218 ticks.
- Killed while open by a BEAM: +1,500 and a bigger shatter flash. Killed open by anything else: 500.
- Fires regardless of the player's position (radial).

## Placement rules

- S6 canopy clearings and temple terraces; singly or in beds of 3 with staggered phases so one is always
  opening; at most 4 per screen (at most 32 ring bullets, capped by the bucket budget at 24: a fourth
  BLOOM skips a ring if the cap is hit).

## Readability

The bud and the open star are two different silhouettes, so the state reads instantly. The flower is the
Lattice's own shape: hexagon and triangles, never a real flower.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
