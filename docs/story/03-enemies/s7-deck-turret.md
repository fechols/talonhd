+++
id = "enemy.s7.deck_turret"
kind = "enemy"
name = "DECK TURRET"
stage = "stage.s7"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [4, 4]
angles = 16
anchor = "center"
identity = "a twin-gun dome turret on an airship hull seen from directly above: grey armoured dome, two long barrels, red running light, purple crystal hatch"
silhouette = "round dome on a square deck plate, two long parallel barrels, small crystal patch"
proportions = "barrels as long as the dome is wide; deck plate one cell wider than the dome"
materials = "mid grey armour dome, dark grey barrels and deck plate, light red running light, purple crystal"
acting = "tracks the TALON smoothly across the deck, fires paired shots in a steady beat"
forbidden = ["people or crew", "national markings", "eyes or a face", "flags", "more than two barrels"]
hurtbox = { x = -13, y = -13, w = 26, h = 26 }
domain = "hull"
hp = 6
contact_damage = 0
score = 400
weak_to = "beam"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "turret"
params = { turn_step_ticks = 3, fire_range_lines = 170, hold_fire_when_below_player = true, pair_gap_px = 8 }
attack = { kind = "aimed", count = 2, spread_deg = 0, speed_q8 = 448, telegraph_ticks = 10, cooldown_ticks = 80 }
placement = "S7 fleet section, on airship upper hulls in lines along the spine; at most 5 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 2 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 12
note = "mid grey dome"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey barrels and deck plate"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal over the hatch"
required = true

[[colors]]
role = "light"
c64 = 10
note = "light red running light"
required = false

[[animations]]
id = "aim"
frames = 16
ticks_per_frame = 3
loop = false
model = "offline"
motion = "16 headings of the dome and barrels rotated from the master and composited over the hull plate"

[[animations]]
id = "fire"
frames = 1
ticks_per_frame = 5
loop = false
model = "offline"
motion = "twin muzzle flash overlay per heading"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the dome blown open into a smoking ring of torn plate on the hull, barrels bent"

[ludo]
sprite_prompt = "a twin-gun dome turret on an airship hull seen from directly above: grey armoured dome, two long barrels, red running light, purple crystal hatch, barrels pointing straight down, top-down view"
+++

# DECK TURRET

## Concept

The fleet's airship hulls pass below the TALON like islands of steel in the clouds, and every hull is
studded with twin-gun domes. DECK TURRETs are the PILLBOX grown up: they track faster (a full turn in
under a second) and fire pairs. They teach **clearing a hull in a pass**: lines of turrets along a spine
are a BEAM target, and a player who parks under the spine with BEAM held cuts the whole line.

## Behaviour pattern

- Active within 170 lines above the player. Steps one of 16 headings every 3 ticks.
- Aligned: 10-tick muzzle glow, then 2 parallel shots (8 px apart) at 1.75 px/tick; cooldown 80.
- Holds fire once the player is above it.
- Loop 2+: cooldown 60 and the pair becomes a 3-way at 10 degrees.

## Placement rules

- S7 fleet section only, on the upper decks of hull bitmaps; in lines of 3-5 along the spine, 40+ px
  apart; at most 5 per screen.
- Hull bitmaps scroll at the stage's scroll rate (the hulls are the "ground"), so these bake exactly like
  PILLBOX over each hull's plate pattern.

## Readability

A grey dome with two long barrels and a red light: a warship gun. The barrel pair shows the shot pair.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
