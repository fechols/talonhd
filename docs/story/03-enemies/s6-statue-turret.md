+++
id = "enemy.s6.statue_turret"
kind = "enemy"
name = "STATUE GUN"
stage = "stage.s6"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [4, 6]
angles = 16
anchor = "base"
identity = "an abstract stepped stone statue seen from directly above: stacked mossy grey blocks, vines, crowned by a rotating purple crystal prism with one barrel"
silhouette = "stepped square tiers, vine lines, prism on top, single barrel, long shadow"
proportions = "three tiers each smaller than the last; prism a third of the base; shadow one base length to the lower right"
materials = "mossy mid grey stone, green vines, purple crystal prism, dark grey shadow"
acting = "the prism on top grinds round to face the TALON and fires a quick line of three shots"
forbidden = ["human or animal figures", "faces or eyes", "religious iconography or idols", "real-world monuments", "writing or glyphs"]
hurtbox = { x = -12, y = -40, w = 24, h = 32 }
domain = "ground"
hp = 12
contact_damage = 0
score = 700
weak_to = "twin"
resists = "flank"
drops = { spark = 0 }

[behavior]
archetype = "turret"
params = { turn_step_ticks = 5, fire_range_lines = 170, hold_fire_when_below_player = true, stream_gap_ticks = 6 }
attack = { kind = "burst", count = 3, spread_deg = 0, speed_q8 = 480, telegraph_ticks = 16, cooldown_ticks = 100 }
placement = "S6 temple terraces; on terrace corners and stair heads; at most 3 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 3 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 12
note = "mossy mid grey stone"
required = true

[[colors]]
role = "moss"
c64 = 5
note = "green vines and moss"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal prism"
required = true

[[colors]]
role = "shadow"
c64 = 11
note = "dark grey cast shadow"
required = false

[[animations]]
id = "aim"
frames = 16
ticks_per_frame = 5
loop = false
model = "offline"
motion = "16 headings of the prism rotated from the master and composited on the statue top"

[[animations]]
id = "charge"
frames = 2
ticks_per_frame = 8
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the crystal prism glows from within and its barrel brightens to the tip"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the top tier broken away, prism shattered on the steps, stone scorched"

[ludo]
sprite_prompt = "an abstract stepped stone statue seen from directly above: stacked mossy grey blocks, vines, crowned by a rotating purple crystal prism with one barrel, barrel pointing straight down, top-down view"
+++

# STATUE GUN

## Concept

(Display name `STATUE GUN` fits the 12-character rule; the ID keeps the storyboard's `statue_turret`.)

The ruins' abstract stepped statues are pure geometry, stacked blocks with no figures, and the Lattice has
crowned each with a gun prism. A STATUE GUN fires **a quick stream of three** along one line. It teaches
**the single sidestep**: a stream is aimed once, so one clean move sideways clears all three, while
weaving back and forth walks into the second or third shot. Its stone body shrugs off FLANK fire.

## Behaviour pattern

- Active within 170 lines above the player. The prism steps one of 16 headings every 5 ticks.
- Aligned: 16-tick charge glow, then 3 shots along the same line, 6 ticks apart, at 1.875 px/tick.
  Cooldown 100.
- Holds fire once the player is above it.
- Loop 2+: 5 shots per stream.

## Placement rules

- S6 temple terraces, on corners and stair heads; at most 3 per screen, never two firing along the same
  line at the player (offset them by at least 48 px horizontally).
- Tall object: its patch includes the long cast shadow; hurtbox is on the top tiers.

## Readability

A stepped grey block with a purple jewel on top: the jewel turns, the jewel fires. No figure, no face,
only geometry, as the bible demands.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
