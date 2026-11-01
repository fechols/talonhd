+++
id = "enemy.s5.gallery_gun"
kind = "enemy"
name = "GALLERY GUN"
stage = "stage.s5"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [3, 3]
angles = 1
anchor = "center"
identity = "a square stone gun embrasure in a fortress wall seen from directly above, a short dark barrel poking out, purple crystal lining the opening"
silhouette = "square notch in a thick wall, one short barrel, crystal rim"
proportions = "embrasure a third of the wall thickness; barrel reaches just past the wall face"
materials = "olive and khaki stone blocks, dark grey barrel and embrasure shadow, purple crystal rim"
acting = "the guns along the wall fire one after another in a rolling ripple, left to right and back"
forbidden = ["people or soldiers", "flags or banners", "eyes or a face", "religious symbols", "national markings"]
hurtbox = { x = -10, y = -10, w = 20, h = 20 }
domain = "ground"
hp = 3
contact_damage = 0
score = 300
weak_to = "beam"
resists = "none"
drops = { spark = 0, row_clear = 1 }

[behavior]
archetype = "turret"
params = { row = 6, spacing_px = 40, ripple_ticks = 8, ripple = "ping_pong", fire_range_lines = 160, row_bonus = 2000 }
attack = { kind = "fixed", count = 1, spread_deg = 0, speed_q8 = 384, telegraph_ticks = 10, cooldown_ticks = 96 }
placement = "S5 walls section; rows of 6 along the inner face of a bastion wall; at most 2 rows on screen"
sprite_budget = { channels_on_line = 0, bullets_max = 6 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 9
note = "khaki-brown stone"
required = true

[[colors]]
role = "shadow"
c64 = 11
note = "dark grey embrasure and barrel"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal rim"
required = true

[[colors]]
role = "stone_light"
c64 = 5
note = "olive green moss and sunlit stone where the cell allows"
required = false

[[animations]]
id = "fire"
frames = 2
ticks_per_frame = 5
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the crystal rim flashes bright and the barrel kicks back into the embrasure"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the embrasure blown open into a jagged gap, barrel gone, stone blackened"

[ludo]
sprite_prompt = "a square stone gun embrasure in a fortress wall seen from directly above, a short dark barrel poking out, purple crystal lining the opening, barrel pointing straight down, top-down view"
+++

# GALLERY GUN

## Concept

BASTION's walls are lined with gun galleries, six to a row. Each gun is weak (3 HP) and only fires
straight down, but a row ripples its shots left to right and back, sweeping a **moving curtain** across
the screen. GALLERY GUNs teach **timing a gap in a sweep** and reward BEAM, which pierces along a wall
when the player lines up beneath a row and sweeps across it. Clearing a full row drops a spark.

## Behaviour pattern

- A row of 6 guns, 40 px apart, activates when it is within 160 lines above the player.
- Ripple: gun 1 fires, then gun 2 eight ticks later, through gun 6, then back from 6 to 1 (ping-pong).
  Each gun telegraphs 10 ticks (rim flash) and fires one shot straight down at 1.5 px/tick. A full sweep
  takes 96 ticks, so each gun fires every 96 ticks.
- Destroyed guns leave a gap in the ripple (the timing continues; dead guns skip their beat).
- Holds fire once the row is below the player's line.
- Clearing all 6 in a row pays 2,000 and drops 1 spark from the last gun.

## Placement rules

- S5 walls section, along the inner face of authored wall strips; at most 2 rows on screen, staggered
  so their sweeps run in opposite directions.
- Each gun is its own 3×3 patch; row patches must not overlap.

## Readability

A row of dark notches with purple rims along a pale wall: the gun line reads before it fires, and the
ripple of flashes shows the sweep direction.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
