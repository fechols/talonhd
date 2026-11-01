+++
id = "enemy.s1.skiff"
kind = "enemy"
name = "SKIFF"
stage = "stage.s1"
faction = "captured"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "W"
cells = [2, 1]
expanded = false
angles = 8
anchor = "center"
identity = "an orange Accord harbour patrol boat seen from directly above: pointed bow, grey deck, one deck gun, light blue crystal on the wheelhouse, white wake"
silhouette = "pointed hull, square wheelhouse, small gun mount, trailing wake V"
proportions = "hull three times as long as wide; wheelhouse a quarter of the length; wake as long as the hull"
materials = "orange painted hull, dark grey deck and gun, light blue crystal, white foam wake"
acting = "putters along its sea lane, swings the bow toward the TALON, fires, carries on"
forbidden = ["people or crew", "national flags or markings", "sails", "eyes or a face", "submarines"]
hurtbox = { x = -20, y = -7, w = 40, h = 14 }
domain = "sea"
hp = 3
contact_damage = 1
score = 200
weak_to = "twin"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "rail_runner"
params = { speed_q8 = 192, lane = "horizontal_sea_lane", ground_locked = true, groups = 3, spacing_px = 56 }
attack = { kind = "aimed", count = 1, spread_deg = 0, speed_q8 = 320, telegraph_ticks = 14, cooldown_ticks = 100 }
placement = "S1 open sea and docks only; lanes at least 48 lines apart; never a SKIFF lane inside the lower 64 lines when it enters"
sprite_budget = { channels_on_line = 4, bullets_max = 3 }
death_vfx = "vfx.explosion_s"

[[colors]]
role = "primary"
c64 = 8
note = "orange hull (private ink)"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal on the wheelhouse (shared pair, proposed)"
required = false

[[colors]]
role = "deck"
c64 = 11
note = "dark grey deck and gun (shared pair, proposed)"
required = false

[[animations]]
id = "cruise"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "the boat bobs gently forward, its white wake rippling and trailing behind"

[[animations]]
id = "heading"
frames = 8
ticks_per_frame = 4
loop = false
model = "offline"
motion = "8 headings rotated from the master; the wake is part of each frame"

[ludo]
sprite_prompt = "an orange Accord harbour patrol boat seen from directly above: pointed bow, grey deck, one deck gun, light blue crystal on the wheelhouse, white wake, heading right, top-down view"
+++

# SKIFF

## Concept

The breakwater's patrol boats, still running their old lanes with crystal at the wheel. They are slow,
low and ride the water rather than the air, so they **move with the scroll and across it at once**. The
SKIFF teaches the first lesson of S1's open sea: not every target moves like a WASP, and a slow aimed
shot from below the middle of the screen is still a shot.

## Behaviour pattern

- Spawns at a lane's edge, travelling horizontally at 0.75 px/tick relative to the water. The water
  scrolls with the bitmap, so on screen a SKIFF drifts down-diagonally.
- In groups of up to 3 on one lane, 56 px apart.
- Every 100 ticks, once in range (lane above the player), it swings its bow one heading toward the
  player over 14 ticks and fires one aimed shot at 1.25 px/tick, then swings back.
- Leaves by the far edge, or scrolls out the bottom.

## Placement rules

- S1 open sea and the dock basins only.
- A lane holds at most 3 SKIFFs (6 channels on the lane's 21 lines), so lanes are entered above line 80
  and are 48+ lines apart. With the 0.5 px/tick scroll a lane reaches the player band after about 4 s;
  by then at most 2 SKIFFs per lane may be alive or the lane is authored to exit first.
- Not combined with a GANNET dive on the same screen.

## Readability

An orange stroke with a white V behind it: a boat, at any size. Orange on blue sea is the clearest
contrast in S1.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
