+++
id = "enemy.common.hatch"
kind = "enemy"
name = "HATCH"
stage = "common"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [4, 4]
angles = 1
anchor = "center"
identity = "a square armoured hatch seen from directly above: two sliding grey doors with yellow-black hazard edges, a purple crystal nest glowing in the shaft"
silhouette = "square frame, two sliding doors, hazard-striped edges, glowing shaft"
proportions = "square; each door half the frame; shaft three quarters of the frame when open"
materials = "mid grey steel frame, light grey doors, yellow and black hazard paint, purple and light blue crystal"
acting = "doors judder, slide apart, a WASP rises out every few beats, doors slam shut"
forbidden = ["eyes or a face", "teeth or a mouth shape", "organic nest or eggs", "people", "national markings"]
hurtbox = { x = -12, y = -12, w = 24, h = 24 }
domain = "ground"
hp = 8
contact_damage = 0
score = 1000
weak_to = "beam"
resists = "twin"
drops = { spark = 1 }

[behavior]
archetype = "hatch"
params = { open_ticks = 20, launch = "enemy.common.wasp", launch_count = 3, launch_spacing_ticks = 20, cycles = 2, closed_ticks = 90, closed_damage_scale = 0.0, closed_beam_scale = 0.5 }
attack = { kind = "launch", count = 3, spread_deg = 0, speed_q8 = 512, telegraph_ticks = 20, cooldown_ticks = 90 }
placement = "on hangar floors, aprons, roofs and fortress courtyards; one HATCH open at a time"
sprite_budget = { channels_on_line = 0, bullets_max = 0 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 12
note = "mid grey frame"
required = true

[[colors]]
role = "hazard"
c64 = 7
note = "yellow hazard edges"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal nest in the shaft"
required = true

[[colors]]
role = "doors"
c64 = 15
note = "light grey door panels where the cell allows"
required = false

[[animations]]
id = "open"
frames = 4
ticks_per_frame = 5
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "the two doors judder and slide apart to the sides, light blooming up from the shaft"

[[animations]]
id = "close"
frames = 2
ticks_per_frame = 4
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
motion = "the doors buckle inward and the crystal in the shaft goes dark and cracked"

[ludo]
sprite_prompt = "a square armoured hatch seen from directly above: two sliding grey doors with yellow-black hazard edges, a purple crystal nest glowing in the shaft, doors half open, top-down view"
+++

# HATCH

## Concept

Every Lattice-held base has holes it grows drones in. A HATCH opens, sends up a string of WASPs, and
closes. It teaches **timing a window**: shut, it shrugs off TWIN fire; open, it is soft, and it holds
a spark. BEAM, which pierces into ground targets, is the one weapon that hurts it through closed doors
(at half damage), so HATCHes are where the player first learns why they might arm BEAM.

## Behaviour pattern

1. Closed until its centre crosses screen line 40.
2. Opens over 20 ticks (doors judder, shaft glows). Fully open: launches 3 WASPs on the `drop_u` curve,
   20 ticks apart. Launched WASPs are not leaders and carry no spark.
3. Closes; waits 90 ticks; repeats for 2 cycles in total, then stays shut.
4. Damage: full while open. Closed, only BEAM damages it (×0.5 instead of the ×2 weak bonus). TWIN does
   ×0.5 even when open (`resists = "twin"`): the player is nudged toward a heavier weapon.
5. Destroyed: drops 1 spark; any WASP still in the shaft is destroyed with it.

## Placement rules

- One HATCH open at a time per screen; at most 2 placed per screen.
- In S1 the **HANGAR HATCH** of the storyboard is this entity set into the hangar roofs (same art with a
  roof-panel surround baked by the stage builder; no separate file).
- S2's ROOF HATCH and S6's CANOPY HATCH are separate entities with their own twists.
- Its launched WASPs count toward the WASP limit (two strings).

## Readability

A square with hazard stripes is a door; light from inside means it is open. Yellow-black edges stay
readable on every stage palette.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
