+++
id = "enemy.s3.rail_car"
kind = "enemy"
name = "RAIL CAR"
stage = "stage.s3"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [4, 6]
angles = 8
anchor = "center"
identity = "a grey flatcar on desert rails seen from directly above, carrying a squat turret with twin barrels, purple crystal ore heaped around the turret base"
silhouette = "long rectangle on two rails, round turret mid-deck, twin barrels, ore heaps"
proportions = "car one and a half times as long as wide; turret half the car width"
materials = "mid grey steel flatcar, dark grey rails and turret, purple and light blue crystal ore"
acting = "rolls steadily down the line, its turret swinging to track, twin barrels firing in pairs"
forbidden = ["people or crew", "steam locomotive faces", "eyes or a face", "national markings", "real railway logos"]
hurtbox = { x = -12, y = -20, w = 24, h = 40 }
domain = "ground"
hp = 8
contact_damage = 0
score = 500
weak_to = "beam"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "tank"
params = { speed_q8 = 256, path = "rail", path_step_px = 2, turret_angles = 8, turn_step_ticks = 6, fire_range_lines = 160, train = 3 }
attack = { kind = "burst", count = 2, spread_deg = 0, speed_q8 = 448, telegraph_ticks = 14, cooldown_ticks = 100 }
placement = "S3 desert line, canyon tunnels and freight yard; trains of up to 3 cars on authored rail paths"
sprite_budget = { channels_on_line = 0, bullets_max = 6 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 12
note = "mid grey flatcar"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey rails and turret"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal ore"
required = true

[[colors]]
role = "ore_glow"
c64 = 14
note = "light blue glow on the ore where the cell allows"
required = false

[[animations]]
id = "aim"
frames = 8
ticks_per_frame = 6
loop = false
model = "offline"
motion = "8 turret headings rotated from the master and composited at every rail sample"

[[animations]]
id = "roll"
frames = 2
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the flatcar rolls forward along the rails, ore glittering, wheels flickering at the edges"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the flatcar derailed and tilted, turret blown off, dark cracked ore spilled across the sand"

[ludo]
sprite_prompt = "a grey flatcar on desert rails seen from directly above, carrying a squat turret with twin barrels, purple crystal ore heaped around the turret base, rails running vertically, top-down view"
+++

# RAIL CAR

## Concept

The power trains the stage exists to cut. RAIL CARs are turrets that **travel**: a train of up to three
flatcars rolls down (or up) the track while the guns on them track the player. They teach that ground
targets on a path can **outpace the scroll** and arrive faster than the terrain says. BEAM, which
pierces through ground targets, can skewer a whole train when the track runs straight down the screen.

## Behaviour pattern

- Trains of 1-3 cars move along the authored rail path at 1 px/tick relative to the ground, in 2-px
  samples (baked like CRAWLER: each car's turret heading at each sample over the exact rail patch).
- Each turret steps through 8 headings every 6 ticks toward the player. Aligned: 14-tick telegraph,
  2 shots 8 ticks apart at 1.75 px/tick; cooldown 100, staggered 33 ticks between cars so a train fires
  a steady rhythm rather than a wall.
- Holds fire once the player is above it.
- A destroyed car stays on the track as a wreck patch; cars behind it stop (the train is cut), which is
  the story of the stage in miniature.

## Placement rules

- S3 only, on authored rail paths. At most 2 trains (6 cars) on screen.
- Rails running toward the player (upward travel) are used sparingly, once per section, because the
  closing speed is 1.5 px/tick.
- Paths may enter canyon tunnels: a car inside a tunnel patch is hidden and immune.

## Readability

A rectangle on rails always reads as a rail car; the round turret on it reads as a gun. Purple ore on the
deck matches the cargo piles in the yard, tying the enemy to the stage's story.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
