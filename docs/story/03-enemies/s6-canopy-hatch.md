+++
id = "enemy.s6.canopy_hatch"
kind = "enemy"
name = "CANOPY HATCH"
stage = "stage.s6"
faction = "lattice"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [5, 5]
angles = 1
anchor = "center"
identity = "a hidden launch shaft in the jungle treetops seen from directly above: a ring of green leaves parting to show a hexagonal purple crystal well"
silhouette = "closed: dense round treetop; open: leaf ring around a hexagonal glowing well"
proportions = "treetop fills the patch; the well half the diameter when open"
materials = "dark and light green leaves, brown branches, purple and light blue crystal well"
acting = "a single treetop shivers, the leaves peel back, drones rise from the glow, the leaves close"
forbidden = ["eyes or a face", "animals", "organic nest or eggs", "people", "a mouth shape"]
hurtbox = { x = -12, y = -12, w = 24, h = 24 }
domain = "ground"
hp = 8
contact_damage = 0
score = 1200
weak_to = "beam"
resists = "twin"
drops = { spark = 1 }

[behavior]
archetype = "hatch"
params = { tell_ticks = 30, open_ticks = 20, launch = "enemy.s6.dragonfly", launch_count = 3, launch_spacing_ticks = 16, cycles = 1, closed_damage_scale = 0.0, closed_beam_scale = 1.0 }
attack = { kind = "launch", count = 3, spread_deg = 0, speed_q8 = 1024, telegraph_ticks = 30, cooldown_ticks = 0 }
placement = "S6 canopy section; among ordinary treetops; at most 2 per screen, one opening at a time"
sprite_budget = { channels_on_line = 0, bullets_max = 0 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "foliage"
c64 = 5
note = "green leaves"
required = true

[[colors]]
role = "foliage_dark"
c64 = 9
note = "brown branches and deep shade"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal well"
required = true

[[colors]]
role = "foliage_light"
c64 = 13
note = "light green sunlit leaf tips where the cell allows"
required = false

[[animations]]
id = "tell"
frames = 2
ticks_per_frame = 5
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "a single round treetop shivers and rustles while the trees around it stay still"

[[animations]]
id = "open"
frames = 4
ticks_per_frame = 5
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "the leaves peel back outward from the centre revealing a glowing hexagonal crystal well"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "a burnt gap in the canopy with a cracked dark crystal well at the bottom"

[ludo]
sprite_prompt = "a hidden launch shaft in the jungle treetops seen from directly above: a ring of green leaves parting to show a hexagonal purple crystal well, top-down view"
+++

# CANOPY HATCH

## Concept

The jungle's version of the hatch: a well of crystal hidden under one treetop among hundreds. It is a
Lattice growth, not a captured door (`variant_of = "enemy.common.hatch"` in prose). It teaches
**noticing the one thing that moves**: a single shivering treetop means a DRAGONFLY group in a second
and a half. And BEAM, which pierces down through the canopy, can kill it at full damage before it ever
opens, the stage's reward for the player who armed BEAM for the BLOOMs.

## Behaviour pattern

1. Indistinguishable from the canopy until its centre crosses line 32.
2. Tell: the treetop shivers for 30 ticks (only BEAM can damage it now, at full damage).
3. Opens over 20 ticks and launches 3 DRAGONFLIES, 16 ticks apart, which then fly their normal legs.
   One cycle only.
4. Stays open afterwards and takes damage from everything (TWIN at ×0.5). Drops 1 spark.
5. Killed before launching: no DRAGONFLIES and a +1,000 early bonus.

## Placement rules

- S6 canopy section, among ordinary treetop tiles of the same style; at most 2 per screen.
- Its launches count as that screen's DRAGONFLY group: no other DRAGONFLY wave on the same screen.

## Readability

Hidden by design, but never unfair: the shiver is a 2-frame motion on an otherwise static canopy, and
the purple well appears before anything launches.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
