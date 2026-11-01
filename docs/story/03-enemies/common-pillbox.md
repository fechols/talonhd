+++
id = "enemy.common.pillbox"
kind = "enemy"
name = "PILLBOX"
stage = "common"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [3, 3]
angles = 16
anchor = "center"
identity = "a round concrete anti-aircraft pillbox seen from directly above: a grey turret ring with twin short barrels, purple crystal facets grown through the roof"
silhouette = "circle, inner turret ring, twin parallel barrels, crystal cluster"
proportions = "barrels reach from the centre to the rim; crystal covers a third of the roof"
materials = "mid grey weathered concrete, dark grey gun metal, purple and light blue crystal"
acting = "the turret ring turns in steady steps to follow the TALON; barrels flash before firing"
forbidden = ["people or crew", "national markings", "eyes or a face", "more than two barrels", "flags"]
hurtbox = { x = -11, y = -11, w = 22, h = 22 }
domain = "ground"
hp = 4
contact_damage = 0
score = 200
weak_to = "beam"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "turret"
params = { turn_step_ticks = 4, fire_range_lines = 160, hold_fire_when_below_player = true }
attack = { kind = "aimed", count = 1, spread_deg = 0, speed_q8 = 384, telegraph_ticks = 12, cooldown_ticks = 90 }
placement = "on concrete, docks, wall-tops and decks; at most 4 firing PILLBOXes on screen"
sprite_budget = { channels_on_line = 0, bullets_max = 1 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 12
note = "mid grey concrete"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey turret ring and barrels"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal on the roof"
required = true

[[colors]]
role = "crystal_light"
c64 = 14
note = "light blue facet highlights where the cell has a free ink"
required = false

[[animations]]
id = "aim"
frames = 16
ticks_per_frame = 4
loop = false
model = "offline"
motion = "16 turret headings rotated from the master and composited over the background patch"

[[animations]]
id = "fire"
frames = 1
ticks_per_frame = 6
loop = false
model = "offline"
motion = "muzzle flash overlay per heading, composited offline"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the turret ring caves in and the crystal cracks into dull shards, leaving a scorched crater"

[ludo]
sprite_prompt = "a round concrete anti-aircraft pillbox seen from directly above: a grey turret ring with twin short barrels, purple crystal facets grown through the roof, barrels pointing straight down, top-down view"
+++

# PILLBOX

## Concept

The airbase's own flak positions, turned. PILLBOXes are the first **bitmap-baked** enemy the player
meets: part of the ground, rotating to follow. They teach the ground half of TALON: fire comes from
fixed points you can see coming, so **kill them while they are still above you**, and lead the scroll.
BEAM pierces down through ground targets and eats rows of them.

## Behaviour pattern

- Active once its centre is within 160 lines above the player. Every 4 ticks the turret steps one of 16
  headings toward the player (a full turn takes about a second, so fast lateral moves outrun it).
- When aligned within one step: 12-tick barrel glow, then one aimed shot at 1.5 px/tick; cooldown 90.
- Holds fire once the player is above it: a pillbox that has scrolled past is harmless. This is a
  deliberate fairness rule for every bitmap turret in the game.
- Destroyed: the patch swaps to the wreck crater (both VIC buffers) and plays `vfx.ground_blast` as a
  sprite overlay.

## Placement rules

- At most 4 firing PILLBOXes on screen; space them at least 32 px apart so the baker's patch rects do
  not overlap (overlapping rects need a combined bake).
- Each instance is baked over its exact background: 16 headings + 16 fire frames + 1 wreck = 33 patches
  of 3×3 cells (about 3 KiB per instance in the REU).
- Used in S1 (airfield, docks), S5 (wall-tops) and as the base of DECK TURRET thinking in S7.

## Readability

A grey circle with two lines sticking out: the barrels point at you, so you always know who is about to
fire. The crystal patch says "Lattice-held".

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
