+++
id = "enemy.common.crawler"
kind = "enemy"
name = "CRAWLER"
stage = "common"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [4, 4]
angles = 8
anchor = "center"
identity = "a squat light grey Accord tank seen from directly above: wide tracks, a boxy hull, one fixed forward gun, purple crystal on the engine deck"
silhouette = "rectangle hull, two track strips, one forward barrel, crystal cluster at the rear"
proportions = "hull slightly longer than wide; barrel half the hull length; tracks a quarter of the width each"
materials = "light grey painted armour, dark grey tracks, orange hazard stripe on the bow, purple crystal"
acting = "grinds along its route, stops, swings its whole hull toward the TALON, fires twice, moves on"
forbidden = ["a rotating turret separate from the hull", "people or crew", "national markings", "eyes or a face", "wheels"]
hurtbox = { x = -13, y = -13, w = 26, h = 26 }
domain = "ground"
hp = 6
contact_damage = 0
score = 400
weak_to = "burst"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "tank"
params = { speed_q8 = 96, path = "authored", path_step_px = 2, stop_every_px = 64, pivot_ticks_per_angle = 6, fire_range_lines = 150 }
attack = { kind = "burst", count = 2, spread_deg = 0, speed_q8 = 448, telegraph_ticks = 16, cooldown_ticks = 120 }
placement = "on authored road, runway and yard paths; at most 3 CRAWLERs moving on screen"
sprite_budget = { channels_on_line = 0, bullets_max = 2 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 15
note = "light grey hull"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey tracks and gun"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal on the engine deck"
required = true

[[colors]]
role = "accent"
c64 = 8
note = "orange hazard stripe on the bow"
required = false

[[animations]]
id = "drive"
frames = 2
ticks_per_frame = 8
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "tracks rolling forward steadily, hull rocking very slightly"

[[animations]]
id = "pivot"
frames = 8
ticks_per_frame = 6
loop = false
model = "offline"
motion = "8 hull headings rotated from the master, composited at every path sample"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the hull splits open, tracks thrown off, crystal dull and cracked, smoke-black scorch around it"

[ludo]
sprite_prompt = "a squat light grey Accord tank seen from directly above: wide tracks, a boxy hull, one fixed forward gun, purple crystal on the engine deck, facing straight down, top-down view"
+++

# CRAWLER

## Concept

Airfield and depot tanks the Lattice drives by rote. Their gun is fixed in the hull, so a CRAWLER must
**turn its whole body to aim**, and that turn is the telegraph. CRAWLERs teach the player to watch the
ground for intent: a hull swinging toward you means two shots in a second. In S1 they appear on the
runway (the storyboard's "RUNWAY CRAWLER" is this entity placed on the airfield), and they return in
the depots and yards of later stages.

## Behaviour pattern

1. Drives its authored path at 0.375 px/tick relative to the ground, in 2-px path samples.
2. Every 64 px of path it stops, pivots toward the player through the 8 headings (6 ticks per step),
   glows at the muzzle for 16 ticks and fires 2 shots, 8 ticks apart, at 1.75 px/tick along the hull
   heading (the nearest of 8 directions, so it can miss a player between headings).
3. Pivots back to the path heading and drives on. Holds fire once the player is above it.

## Placement rules

- **Bitmap motion:** the baker composites every heading at every 2-px path sample over the exact
  background, into a 5×5-cell rect (one cell of straddle margin). A 400-px path is about 200 samples ×
  8 headings worst case; we bake only the heading the path uses at each sample plus the pivot set at
  each stop, about 60 KiB per CRAWLER in the REU. Paths must not cross another bitmap object's rect.
- At most 3 CRAWLERs moving on screen (patch DMA cost, not sprites).
- **S8:** no CRAWLERs (no ground).

## Readability

A tank shape with one gun and no turret: wherever the barrel points, it will shoot. The orange bow
stripe shows its facing at C64 size.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
