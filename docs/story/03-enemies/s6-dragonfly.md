+++
id = "enemy.s6.dragonfly"
kind = "enemy"
name = "DRAGONFLY"
stage = "stage.s6"
faction = "lattice"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "W"
cells = [2, 1]
expanded = false
angles = 8
anchor = "center"
identity = "a long thin crystal strafing drone seen from directly above: a deep blue needle body crossed by two pairs of narrow light blue blade vanes"
silhouette = "needle body, two crossed pairs of narrow straight vanes, pointed tail"
proportions = "vanes span twice the body width at the front pair, one and a half at the rear; body a sixth of the span thick"
materials = "deep blue faceted crystal body, light blue translucent-looking vanes, dark grey seams"
acting = "zips in a straight line, stops dead, snaps a shot, zips off at an angle"
forbidden = ["insect heads, eyes or legs", "a face", "organic or veined wings", "real aircraft", "yellow"]
hurtbox = { x = -18, y = -6, w = 36, h = 12 }
domain = "air"
hp = 2
contact_damage = 1
score = 300
weak_to = "seeker"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "strafer"
params = { dash_speed_q8 = 1024, dash_ticks = 24, stop_ticks = 20, legs = 3, group = 3, group_spacing_ticks = 16, lowest_line = 120 }
attack = { kind = "aimed", count = 1, spread_deg = 0, speed_q8 = 448, telegraph_ticks = 8, cooldown_ticks = 0 }
placement = "S6 river gaps and launch apron; groups of 3 enter from a side edge; their legs stay above line 120"
sprite_budget = { channels_on_line = 4, bullets_max = 3 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 6
note = "deep blue body (private ink)"
required = true

[[colors]]
role = "vanes"
c64 = 14
note = "light blue vanes (shared pair, proposed)"
required = false

[[colors]]
role = "seam"
c64 = 11
note = "dark grey seams (shared pair, proposed)"
required = false

[[animations]]
id = "hover"
frames = 2
ticks_per_frame = 2
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "vanes shimmer rapidly in place, body perfectly still"

[[animations]]
id = "heading"
frames = 8
ticks_per_frame = 1
loop = false
model = "offline"
motion = "8 headings rotated from the master"

[ludo]
sprite_prompt = "a long thin crystal strafing drone seen from directly above: a deep blue needle body crossed by two pairs of narrow light blue blade vanes, pointing right, top-down view"
+++

# DRAGONFLY

## Concept

Fast Lattice strafers that hunt the river gaps in threes. A DRAGONFLY moves in **stop-start legs**: dash,
stop, shoot, dash. It teaches **the rhythm of the stop**: they are nearly impossible to track while
moving and helpless while stopped, so the player learns to fire at where they will stop, or to arm SEEKER
and let one dart per DRAGONFLY do the tracking.

## Behaviour pattern

1. A group of 3 enters from a side edge, 16 ticks apart.
2. Each makes 3 legs: dash 24 ticks at 4 px/tick toward a point chosen from the player's position (never
   below line 120), stop 20 ticks (vanes shimmer, 8-tick telegraph), fire one aimed shot at
   1.75 px/tick.
3. After the third shot it dashes off the nearest side edge.

## Placement rules

- S6 river gaps and launch apron; at most one group (3 W = up to 4 channels on a line in practice, since
  stop points are spread at least 24 lines apart).
- Never with a MOTH split on the same lines.

## Readability

A blue cross that is all straight lines. Its stillness, vanes shimmering in place, is the tell.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
