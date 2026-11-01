+++
id = "midboss.s2.campanile"
kind = "midboss"
name = "CAMPANILE"
title = "the crystallised bell tower of the hill village"
stage = "stage.s2"
status = "approved"
revision = 1
layer = "bitmap"
hp = 80
weak_to = "burst"
score = 12000
time_bonus = { par_s = 35, max = 12000 }
drops = { spark = 3 }
music_cue = "cue.midboss"
cell_rect = [10, 10]
angles = 8
anchor = "center"
identity = "a square cream-stone bell tower seen from above, its red tile roof replaced by a spinning eight-pointed violet crystal crown"
silhouette = "square stone base, octagonal crown of crystal points, four corner buttresses"
proportions = "crown fills two thirds of the tower top; points reach past the walls"
materials = "cream limestone, weathered red roof tiles at the edges, purple and light blue crystal points with white glints, an iron bell visible through the hub"
acting = "the crown turns in steady steps like a clock hand, speeds up before every ring, and chimes a flash of white"
forbidden = ["religious symbols", "crosses", "people", "eyes or faces", "organic growth"]
hurtbox = { x = -40, y = -40, w = 80, h = 80 }

[[colors]]
role = "primary"
c64 = 15
note = "light grey / cream stone"
required = true

[[colors]]
role = "secondary"
c64 = 8
note = "orange-terracotta remnant roof tiles"
required = false

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal crown"
required = true

[[colors]]
role = "glow"
c64 = 14
note = "light blue facet shading and the chime flash ramp to white"
required = true

[[parts]]
id = "crown"
layer = "bitmap"
cell_rect = [8, 8]
hp = 56

[[parts]]
id = "bell_hub"
layer = "bitmap"
cell_rect = [2, 2]
hp = 24

[[phases]]
from_hp = 80
to_hp = 40
patterns = ["p1_chime_ring", "p1_spoke"]

[[phases]]
from_hp = 40
to_hp = 0
patterns = ["p2_double_peal", "p1_spoke"]

[[patterns]]
id = "p1_chime_ring"
name = "CHIME RING"
telegraph = { ticks = 30, cue = "crown spins faster, white flash on each point" }
attack = { kind = "ring", count = 8, spread_deg = 360, speed_q8 = 224 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 16 }
counterplay = "One shard leaves each crown point. The ring is wide at range; close in, the gaps are wider than the ship, so a BURST player hugging the tower is safe between points."

[[patterns]]
id = "p1_spoke"
name = "SPOKE"
telegraph = { ticks = 20, cue = "one crown point glows yellow" }
attack = { kind = "fixed", count = 5, spread_deg = 0, speed_q8 = 384, interval_ticks = 5 }
sprite_budget = { boss_channels = 0, bucket_channels = 1, bullets_max = 5 }
counterplay = "A fast 5-shot stream along the glowing point's direction, which then rotates one step. Read the yellow point and stand off its line."

[[patterns]]
id = "p2_double_peal"
name = "DOUBLE PEAL"
telegraph = { ticks = 36, cue = "the bell hub opens, crown reverses direction" }
attack = { kind = "ring", count = 16, spread_deg = 360, speed_q8 = 192, rings = 2, ring_offset_deg = 11 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 32 }
counterplay = "Two staggered 16-shard rings, the second rotated half a gap. Slip through the first ring's gap, then sidestep half a gap for the second; the open hub takes double damage from BURST."

[[animations]]
id = "crown_turn"
frames = 8
ticks_per_frame = 6
loop = true
model = "offline"
duration = 3
request_frames = 16
motion = "the eight-pointed crystal crown turns one eighth of a step per frame"

[[animations]]
id = "hub_open"
frames = 3
ticks_per_frame = 8
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "crystal points at the centre fold back to show a dark iron bell glowing light blue"

[[animations]]
id = "wreck"
frames = 3
ticks_per_frame = 10
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "the crown shatters into dark shards and the tower top becomes a scorched open shaft"

[ludo]
concept_prompt = "A square village bell tower of cream limestone seen from directly above, its roof replaced by a spinning eight-pointed crown of faceted purple and light blue crystal, white glints, a dark iron bell visible at the hub. Weathered terracotta tiles cling to the edges. Orthographic top-down, lit from the upper left, short shadow to the lower right across a sunny village courtyard."
setpiece_prompt = "a square cream-stone bell tower from directly above with an eight-pointed purple crystal crown on top, terracotta tile edges, on a sunlit cobbled courtyard"
+++

# CAMPANILE

## Personality and role

The hill village's bell tower, now the Lattice's metronome. It never moves, it only turns and rings.
CAMPANILE teaches **reading a rotating emitter** and rewards brave players who close in with BURST.

## Arena

Section 2 of TERRACES, the hill village, around screen 12. The tower stands in the fountain plaza;
the scroll stops for the fight with the tower in the upper half of the view and the courtyard below
for room to dodge. ROOF HATCHes around the plaza stay shut during the fight.

## Phases and patterns

**Phase 1 (80 → 40).** CHIME RING every 3 s, SPOKE between rings. The crown is the main target; it
turns in eighths, each frame a baked patch.

**Phase 2 (40 → 0).** The hub opens (BURST does double damage to it), the crown reverses, and CHIME
RING becomes DOUBLE PEAL.

**Weakness.** BURST: its short-range 5-way spread lands all five pellets on the 80-px tower when the
player sits just below it, between the ring's points.

## Budget notes

Bitmap only. Eight crown rotation patches plus three hub and three wreck patches, all baked over the
tower top. DOUBLE PEAL is the stage's heaviest bullet moment: 32 shards, spread over the screen so
no line carries more than 2 bucket channels; the second ring is held until the first is 40 px out.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
