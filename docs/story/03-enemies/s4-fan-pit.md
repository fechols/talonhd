+++
id = "enemy.s4.fan_pit"
kind = "enemy"
name = "FAN PIT"
stage = "stage.s4"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [6, 6]
angles = 1
anchor = "center"
identity = "a square vent pit seen from directly above: a six-bladed grey turbine fan spinning inside a yellow-black hazard-striped frame, purple crystal on the hub"
silhouette = "square frame, big circle, six curved blades, crystal hub"
proportions = "fan fills the frame; hub a fifth of the diameter; frame border one cell"
materials = "mid grey steel blades, yellow and black hazard frame, dark grey shaft depth, purple crystal hub"
acting = "spins without end, roaring; its draught bends anything flying over it"
forbidden = ["eyes or a face", "people", "brand logos", "fire or flames", "more or fewer than six blades"]
hurtbox = { x = -6, y = -6, w = 12, h = 12 }
domain = "ground"
hp = 0
contact_damage = 0
score = 0
weak_to = "none"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "hazard"
params = { field_radius_px = 40, deflect_q8 = 48, direction = "authored_cw_or_ccw", affects = "all_shots", beam_jam_ticks = 180, jam_score = 1000 }
attack = { kind = "none", count = 0, spread_deg = 0, speed_q8 = 0, telegraph_ticks = 0, cooldown_ticks = 0 }
placement = "S4 pump hall floor; in rows of 2-3 with gaps; never under a midboss or boss arena"
sprite_budget = { channels_on_line = 0, bullets_max = 0 }
death_vfx = "none"

[[colors]]
role = "primary"
c64 = 12
note = "mid grey blades"
required = true

[[colors]]
role = "hazard"
c64 = 7
note = "yellow hazard frame"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal hub"
required = true

[[colors]]
role = "depth"
c64 = 11
note = "dark grey shaft depth behind the blades"
required = false

[[animations]]
id = "spin"
frames = 4
ticks_per_frame = 2
loop = true
model = "offline"
motion = "4 rotation steps of the six-blade master (60-degree symmetry) composited over the frame"

[[animations]]
id = "jammed"
frames = 2
ticks_per_frame = 8
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the fan stalls and shudders in place, the crystal hub cracked and flickering"

[ludo]
sprite_prompt = "a square vent pit seen from directly above: a six-bladed grey turbine fan spinning inside a yellow-black hazard-striped frame, purple crystal on the hub, top-down view"
+++

# FAN PIT

## Concept

The pump hall's turbine vents roar so hard they bend bullets. A FAN PIT is a **hazard**, not an enemy: it
cannot be destroyed and never fires, but every shot that crosses its field, the player's and the
enemy's, is pushed sideways in the direction the fan turns. It teaches **the screen itself can be used**:
your stream curls away from a turret behind a fan, and an enemy's aimed shot curls away from you if you
put a fan between you.

## Behaviour pattern

- Field: a 40-px radius around the hub. Every tick, every bullet and player shot inside it gains
  48/256 px/tick of tangential velocity (clockwise or counter-clockwise, authored per fan) and keeps it
  after leaving. SEEKER darts are also bent but re-acquire their target.
- The fan takes no damage. A BEAM held on the **hub** (the 12×12 hurtbox) for 30 ticks jams it: the fan
  stalls for 180 ticks (field off) and pays 1,000 points once. BEAM's piercing lance is the only weapon
  that reaches down into the shaft.
- The player ship is not pushed: TALON's analogue controls are faithful.

## Placement rules

- S4 pump hall floor; in rows of 2-3 with gaps of at least 48 px; adjacent fans spin in opposite
  directions so their fields read as a pattern, not noise.
- Never under a midboss or boss arena, and never with a GNAT swarm in its field (motes are not bent;
  mixing the two confuses the rule).
- `hp = 0` means indestructible; `bible.py` treats hazard archetypes as exempt from the HP check.

## Readability

Hazard stripes and a spinning blade: everyone knows not to put their hand in it. The curl of the first
bent shot explains the rest.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
