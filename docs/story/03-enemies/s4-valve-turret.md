+++
id = "enemy.s4.valve_turret"
kind = "enemy"
name = "VALVE TURRET"
stage = "stage.s4"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [3, 3]
angles = 1
anchor = "center"
identity = "an industrial pipe valve seen from directly above: a red handwheel with four spokes tipped with purple crystal nozzles, on a grey flange"
silhouette = "round wheel, four straight spokes, four nozzle tips, square flange"
proportions = "wheel fills the flange; spokes reach the rim; nozzles one tenth of the wheel"
materials = "light red painted cast iron wheel, mid grey steel flange and bolts, purple crystal nozzles"
acting = "the handwheel cranks round a notch, the four nozzles spit together, it cranks again"
forbidden = ["eyes or a face", "people", "brand logos", "more or fewer than four spokes", "steam faces"]
hurtbox = { x = -11, y = -11, w = 22, h = 22 }
domain = "ground"
hp = 4
contact_damage = 0
score = 300
weak_to = "twin"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "turret"
params = { pattern = "cross", rotate_deg_per_volley = 22, fire_range_lines = 150, wheel_frames = 4 }
attack = { kind = "fixed", count = 4, spread_deg = 90, speed_q8 = 320, telegraph_ticks = 12, cooldown_ticks = 70 }
placement = "S4 pipe yards; on pipe junctions in grids of 2-4; at most 4 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 4 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 10
note = "light red handwheel"
required = true

[[colors]]
role = "flange"
c64 = 12
note = "mid grey flange"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal nozzles"
required = true

[[colors]]
role = "bolts"
c64 = 11
note = "dark grey bolts and shadow"
required = false

[[animations]]
id = "crank"
frames = 4
ticks_per_frame = 3
loop = false
model = "offline"
motion = "4 wheel positions 22.5 degrees apart (90-degree symmetry), rotated from the master"

[[animations]]
id = "spit"
frames = 1
ticks_per_frame = 6
loop = false
model = "offline"
motion = "nozzle flash overlay composited at each wheel position"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the handwheel snapped off its spindle, spokes bent, nozzles dark and cracked"

[ludo]
sprite_prompt = "an industrial pipe valve seen from directly above: a red handwheel with four spokes tipped with purple crystal nozzles, on a grey flange, top-down view"
+++

# VALVE TURRET

## Concept

The pipe district's valves are crystal now, and the Lattice cranks them like guns. A VALVE TURRET does
**not aim**: it fires a fixed cross of four shots, then turns a notch and fires again. It teaches
**pattern reading over reaction**: the gaps are predictable, the rotation is visible on the wheel, and
the player who stops dodging and starts reading slips through grids of them. Four TWIN hits kill one.

## Behaviour pattern

- Active within 150 lines above the player. Every 70 ticks: 12-tick nozzle flash, then 4 shots along the
  spokes at 1.25 px/tick.
- After each volley the wheel cranks 22.5 degrees (one of 4 baked positions; the cross has 90-degree
  symmetry) so the next cross is rotated.
- Fires whether or not the player is above it (it is radial and does not aim).
- Grids of 2-4 valves are phase-locked: the level author sets each valve's start phase so the combined
  pattern leaves a readable diagonal lane.

## Placement rules

- S4 pipe yards on pipe junctions; at most 4 per screen, 32+ px apart.
- 16 bullets max from valves on screen (4 valves × 4).

## Readability

A red wheel with four spokes is a valve to anyone; the spokes are the shot lines. Light red on steel grey
is S4's warning-lamp colour, which is the point.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
