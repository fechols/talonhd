+++
id = "midboss.s7.squall"
kind = "midboss"
name = "SQUALL"
title = "three linked gunships turning in a beam triangle"
stage = "stage.s7"
status = "approved"
revision = 1
layer = "sprite"
hp = 135
weak_to = "seeker"
score = 22000
time_bonus = { par_s = 40, max = 22000 }
drops = { spark = 3 }
music_cue = "cue.midboss"
sprite_class = "L"
cells = [3, 3]
expanded = false
angles = 8
anchor = "center"
identity = "a squat triangular Lattice gunship seen from above, faceted purple crystal hull, three stub wings, a light blue beam emitter at each wingtip"
silhouette = "equilateral triangle with clipped corners, a round emitter on each corner, a raised hexagonal core"
proportions = "72 px across; core a third of the width; emitters 12 px"
materials = "faceted purple and light blue crystal plates, light grey seams, white glints, yellow emitter flare"
acting = "the three hold formation like a spinning top, turning together; emitters brighten in sequence before the beam links"
forbidden = ["people", "Accord markings", "eyes or faces", "organic forms", "cyan (the TALON's private ink)"]
hurtbox = { x = -30, y = -26, w = 60, h = 52 }

[[colors]]
role = "primary"
c64 = 4
note = "purple crystal hull"
required = true

[[colors]]
role = "secondary"
c64 = 14
note = "light blue facets"
required = true

[[colors]]
role = "accent"
c64 = 15
note = "light grey seams"
required = false

[[colors]]
role = "glow"
c64 = 7
note = "yellow emitter flare and the linking beam core"
required = true

[[parts]]
id = "ship_a"
layer = "sprite"
sprite_class = "L"
cells = [3, 3]
hp = 45

[[parts]]
id = "ship_b"
layer = "sprite"
sprite_class = "L"
cells = [3, 3]
hp = 45

[[parts]]
id = "ship_c"
layer = "sprite"
sprite_class = "L"
cells = [3, 3]
hp = 45

[[parts]]
id = "link_beam"
layer = "line"
hp = 0
note = "three edges between the ships; damaging, indestructible; drawn as a bitmap software line"

[[phases]]
from_hp = 135
to_hp = 45
patterns = ["p1_turnstile", "p1_corner_shots"]

[[phases]]
from_hp = 45
to_hp = 0
patterns = ["p2_last_ship", "p1_corner_shots"]

[[patterns]]
id = "p1_turnstile"
name = "TURNSTILE"
telegraph = { ticks = 30, cue = "emitters brighten in sequence, beam edges flicker on" }
attack = { kind = "beam", count = 3, spread_deg = 0, rotate_deg_per_tick = 1, radius_px = 44, duration_ticks = 240 }
sprite_budget = { boss_channels = 6, bucket_channels = 2, bullets_max = 6 }
counterplay = "The triangle turns with its beam edges live for 4 s. The centre of the triangle is a safe pocket only when entered between flickers; otherwise stay out of the swept circle. The TALON is held below line 88, so the edges dip at most to the ceiling."

[[patterns]]
id = "p1_corner_shots"
name = "CORNER SHOTS"
telegraph = { ticks = 18, cue = "the corner emitter flares yellow" }
attack = { kind = "aimed", count = 2, spread_deg = 12, speed_q8 = 352 }
sprite_budget = { boss_channels = 6, bucket_channels = 2, bullets_max = 12 }
counterplay = "With the beam off, each living ship fires an aimed pair. Short weaves beat it; this is the window for SEEKER salvos."

[[patterns]]
id = "p2_last_ship"
name = "LAST SHIP"
telegraph = { ticks = 24, cue = "the survivor spins in place, emitters white" }
attack = { kind = "ring", count = 9, spread_deg = 360, speed_q8 = 256, rings = 3, ring_interval_ticks = 20 }
sprite_budget = { boss_channels = 3, bucket_channels = 2, bullets_max = 27 }
counterplay = "The last ship darts to a new spot along the ceiling and fires three 9-shot rings, each rotated 20 degrees. Hold position under its gaps; move only between rings."

[[occupancy]]
pose = "triangle"
note = "the three ships turn on a circle of radius 44 centred at line 12; their centres are never closer than 66 lines top to bottom (1.5 x radius), so the 63-line ships put at most two on any line (3 + 3 = 6) plus 2 bucket channels = 8; the lowest ship bottom reaches line 87, above the ceiling; a ship rotating over the top is clipped under the score row"
rows = ["111", "111", "111"]

[[animations]]
id = "hover"
frames = 2
ticks_per_frame = 8
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the gunship hovers in place, its hexagonal core pulsing and wing emitters flickering"

[[animations]]
id = "emitter_charge"
frames = 3
ticks_per_frame = 6
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the three wingtip emitters brighten one after another to a yellow flare"

[[animations]]
id = "break"
frames = 3
ticks_per_frame = 6
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the crystal hull cracks along its facets and bursts into tumbling shards"

[ludo]
concept_prompt = "A squat triangular alien gunship seen from directly above, a hull of faceted purple and light blue crystal plates with light grey seams and white glints, three stub wings ending in round emitters glowing yellow, a raised hexagonal core in the centre. Hard-edged geometric, never organic. Orthographic top-down, lit from the upper left, on a dusk purple cloud deck."
sprite_prompt = "a triangular crystal gunship from directly above, faceted purple and light blue plates, three stub wings with yellow-glowing round emitters, hexagonal core"
+++

# SQUALL

## Personality and role

Three ships that fight as one storm. SQUALL is the stage's air-power showcase in miniature: big
sprites, a moving hazard, and a reason to use SEEKER (three targets, three darts, double damage).

## Arena

Section 2 of STORMFRONT, the storm cell, around screen 12. The scroll keeps moving over the cloud
deck; lightning flashes (`$D021`/border) continue. SQUALL holds the upper half: the triangle's centre
stays at line 12 (the top ship dips under the score row as it turns over) and drifts left and right with the player. The boss ceiling (line 88) applies.

## Phases and patterns

**Phase 1 (135 → 45).** TURNSTILE (beams on, 4 s) alternates with CORNER SHOTS (beams off, 3 s).
Killing one ship breaks the two beam edges that touched it; the remaining pair is joined by one edge
that swings like a bar.

**Phase 2 (45 → 0).** When one ship remains (or HP reaches 45) the beam is gone and LAST SHIP repeats,
the survivor darting along the ceiling between rings.

**Weakness.** SEEKER: each ship takes double damage from darts, and with three targets every salvo
splits evenly.

## Budget notes

Each ship is L 3x3 (72x63, 3 channels). On a circle of radius 44 centred at line 12 the ships can put
two on one line but never three, so the worst line is 6 boss + 2 bucket = 8, entirely above the
ceiling. The linking beam is **not** a sprite: it is drawn as a bitmap software line through the M1(c)
bitmap-bullet save/restore path and restored each frame. If M1(c) fails, the fallback draws an edge
as a dotted bucket line only while it is within 30 degrees of vertical (one bucket channel per band)
and blanks the other edges with a flicker telegraph.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
