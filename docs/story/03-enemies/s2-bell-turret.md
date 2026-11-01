+++
id = "enemy.s2.bell_turret"
kind = "enemy"
name = "BELL TURRET"
stage = "stage.s2"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [6, 6]
angles = 16
anchor = "center"
identity = "a round bronze-brown gun cupola seen from directly above, shaped like an upturned bell, a cream stone collar, three purple crystal barrels in a fan"
silhouette = "circle in a square collar, flared bell rim, three-barrel fan, crystal ring"
proportions = "cupola two thirds of the collar; barrels reach to the collar edge; fan 30 degrees wide"
materials = "brown bronze-like metal, cream and light grey stone collar, purple crystal barrels, light blue facet edges"
acting = "swings its fan of barrels around in slow steps, rings out a three-way volley"
forbidden = ["religious symbols", "people", "eyes or a face", "a real bell tower", "national markings"]
hurtbox = { x = -16, y = -16, w = 32, h = 32 }
domain = "ground"
hp = 10
contact_damage = 0
score = 500
weak_to = "burst"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "turret"
params = { turn_step_ticks = 6, fire_range_lines = 150, hold_fire_when_below_player = true }
attack = { kind = "spread", count = 3, spread_deg = 30, speed_q8 = 352, telegraph_ticks = 18, cooldown_ticks = 110 }
placement = "S2 hill village and plaza; on courtyard corners and wall junctions; at most 2 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 6 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 9
note = "brown bronze cupola"
required = true

[[colors]]
role = "stone"
c64 = 15
note = "light grey / cream stone collar"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal barrels"
required = true

[[colors]]
role = "rim"
c64 = 8
note = "orange sunlit rim highlight where the cell allows"
required = false

[[animations]]
id = "aim"
frames = 16
ticks_per_frame = 6
loop = false
model = "offline"
motion = "16 headings of the cupola rotated from the master and composited over the patch"

[[animations]]
id = "ring"
frames = 2
ticks_per_frame = 9
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the bell rim shudders and the crystal barrels flash brighter from root to tip"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the cupola cracked in half and tipped, the crystal barrels shattered on the stone collar"

[ludo]
sprite_prompt = "a round bronze-brown gun cupola seen from directly above, shaped like an upturned bell, a cream stone collar, three purple crystal barrels in a fan, barrels pointing straight down, top-down view"
+++

# BELL TURRET

## Concept

The village's bronze has been melted and regrown into gun cupolas on every plaza corner. The BELL TURRET
is the first ground target that fires a **spread** instead of a single shot, and its 2×2-sprite-sized
patch (48×48 px) makes it the first ground target big enough to need a commitment: ten TWIN hits, or
five BURST hits up close. It teaches that a fan has gaps, and that the gap is widest near the gun.

## Behaviour pattern

- Active within 150 lines above the player. Turns one of 16 headings every 6 ticks toward the player.
- Aligned: 18-tick ring telegraph (rim shudder, barrels flash), then a 3-way spread, 15 degrees apart,
  at 1.375 px/tick. Cooldown 110 ticks.
- Holds fire once the player is above it.
- Loop 2+: a second volley 12 ticks after the first, offset by half the spread angle, which closes the
  near gaps and opens far ones.

## Placement rules

- S2 village and fountain plaza; at most 2 per screen, at least 64 px apart horizontally so their fans
  never overlap into a wall.
- Bakes: 16 headings + 2 telegraph frames per heading (offline brightening) + wreck, 6×6 cells each, about
  15 KiB per instance.

## Readability

The plan's "2×2 rotating patch": a big round cupola with three crystal fingers pointing your way. The
three barrels show the three shots before they fire.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
