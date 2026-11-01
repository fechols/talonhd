+++
id = "enemy.common.wasp"
kind = "enemy"
name = "WASP"
stage = "common"
faction = "lattice"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
expanded = false
angles = 16
anchor = "center"
identity = "a small faceted crystal dart drone seen from directly above: purple arrowhead body, two swept triangular vanes, light blue facet edges, a hexagonal core"
silhouette = "arrowhead, two swept vanes, notched tail, hexagonal core"
proportions = "as long as wide; vanes reach back to the tail notch; core one third of the body"
materials = "purple faceted crystal, light blue facet edges, dark grey seam lines"
acting = "darts in smooth S-curves, banks hard on the turn, never hovers"
forbidden = ["eyes or a face", "insect legs or antennae", "organic or fleshy shapes", "propellers", "yellow stripes"]
hurtbox = { x = -8, y = -7, w = 16, h = 14 }
domain = "air"
hp = 1
contact_damage = 1
score = 100
weak_to = "twin"
resists = "none"
drops = { spark = 0, leader = 1 }

[behavior]
archetype = "swooper"
params = { speed_q8 = 640, curve = "s_left", formation = 6, spacing_ticks = 10, leader = "last", fire_from_rank = 2 }
attack = { kind = "aimed", count = 1, spread_deg = 0, speed_q8 = 384, telegraph_ticks = 8, cooldown_ticks = 0 }
placement = "enters from the top or a side edge; the S-curve's horizontal legs stay above screen line 120; inside the player band the string runs vertically"
sprite_budget = { channels_on_line = 2, bullets_max = 1 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 4
note = "purple body facets (private ink)"
required = true

[[colors]]
role = "leader_core"
c64 = 7
note = "the leader's private ink: its core shows the yellow spark it carries"
required = false

[[colors]]
role = "edge"
c64 = 14
note = "light blue facet edges (shared pair, proposed)"
required = false

[[colors]]
role = "seam"
c64 = 11
note = "dark grey seams (shared pair, proposed)"
required = false

[[colors]]
role = "glint"
c64 = 1
note = "single white glint on the nose facet; dropped if the quantizer needs the slot"
required = false

[[animations]]
id = "fly"
frames = 2
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the core pulses brighter and dimmer while the vanes flex slightly"

[[animations]]
id = "turn"
frames = 16
ticks_per_frame = 1
loop = false
model = "offline"
motion = "16 headings rotated from the master by our tools"

[ludo]
sprite_prompt = "a small faceted crystal dart drone seen from directly above: purple arrowhead body, two swept triangular vanes, light blue facet edges, a hexagonal core, pointing straight up, top-down view"
+++

# WASP

## Concept

The Lattice's commonest grown drone: a sliver of crystal shaped for one job, flying in a line. WASPs are
the first thing the player shoots in TALON and the last thing they stop seeing. They teach the core
rhythm of the genre: a wave enters, you read its curve, you sweep the line, and **the last one, the
leader with the yellow core, carries the spark**. Kill the leader and the gauge moves.

## Behaviour pattern

- A wave of 6 enters as a string, 10 ticks apart, at 2.5 px/tick (`speed_q8 = 640`), following one of
  the authored curves (`s_left`, `s_right`, `hook_left`, `hook_right`, `drop_u`). Each WASP picks the
  nearest of 16 headings every tick.
- The leader flies last. Its private ink is yellow 7 instead of purple 4, so its core reads as a spark.
- From rank 2 upward (and always on loop 2+), each WASP fires one aimed shot (1.5 px/tick) when it
  crosses screen line 80, after an 8-tick core flare. One shot per WASP per pass, never more.
- WASPs never stop. Survivors leave by the edge the curve points to.
- Launched WASPs (from HATCH, ALBATROSS and similar) use the `drop_u` curve: a short drop, then a turn
  toward the player's half of the screen, then out the bottom.

## Placement rules

- At most two WASP strings on screen at once (12 S sprites).
- Curves are authored so the horizontal legs sit above screen line 120. In the lower 80 lines (the
  player band) a string runs vertically, so it costs at most 2 channels on any line.
- Never launch a wave while an M or larger air unit is in the player band.
- **S8 variant (prose only, not a separate entity):** crystal-plated WASPs take 2 hits, fire from rank 0,
  and use light blue 14 as their private ink with purple edges. `variant_of = "enemy.common.wasp"`.

## Readability

An arrowhead with a hexagon in it. Purple means "shoot it", yellow means "shoot it first". The 16
headings make the curve itself readable, so the player learns to aim where the string is going.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
