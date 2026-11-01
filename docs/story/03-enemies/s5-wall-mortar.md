+++
id = "enemy.s5.wall_mortar"
kind = "enemy"
name = "WALL MORTAR"
stage = "stage.s5"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [4, 4]
angles = 1
anchor = "center"
identity = "a squat mortar pit on a fortress bastion seen from directly above: a round stone ring, a fat upward-pointing crystal tube, ammunition crates beside it"
silhouette = "stone ring, fat round tube mouth, two small crates"
proportions = "tube mouth half the ring diameter; crates a fifth of the ring each"
materials = "khaki stone ring, dark grey tube rim, purple crystal tube, brown wooden crates"
acting = "the tube glows, thumps, and a target ring appears on the ground where the shell will fall"
forbidden = ["people or soldiers", "flags", "eyes or a face", "skull markings", "national markings"]
hurtbox = { x = -13, y = -13, w = 26, h = 26 }
domain = "ground"
hp = 8
contact_damage = 0
score = 600
weak_to = "burst"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "mortar"
params = { aim = "player_pos_at_fire", flight_ticks = 75, marker = "ground_ring", shell_shootable = false, splash = 6, fire_range_lines = 180 }
attack = { kind = "lob", count = 1, spread_deg = 0, speed_q8 = 512, telegraph_ticks = 20, cooldown_ticks = 160 }
placement = "S5 bastion tops and the inner keep approach; at most 2 per screen, so at most 2 landing markers at once"
sprite_budget = { channels_on_line = 0, bullets_max = 7 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 9
note = "khaki-brown stone ring"
required = true

[[colors]]
role = "rim"
c64 = 11
note = "dark grey tube rim"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal tube"
required = true

[[colors]]
role = "crates"
c64 = 8
note = "orange-brown crates where the cell allows"
required = false

[[animations]]
id = "load"
frames = 2
ticks_per_frame = 10
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the crystal tube glows from its base up to the mouth"

[[animations]]
id = "fire"
frames = 2
ticks_per_frame = 4
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the tube thumps down and a ring of pale dust bursts out around the pit"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the stone ring split, tube shattered into dark shards, crates scattered and burnt"

[ludo]
sprite_prompt = "a squat mortar pit on a fortress bastion seen from directly above: a round stone ring, a fat upward-pointing crystal tube, ammunition crates beside it, top-down view"
+++

# WALL MORTAR

## Concept

The fortress's heavy mortars, a step up from S2's DOME MORTAR. Their shells are too high to shoot; instead
the ground **marks where they will land**. WALL MORTARs teach **reading markers**: a flashing ring on the
ground is a promise of a 6-way burst there in a second. Since the stage is the ground-target showcase,
they share the screen with GALLERY GUN curtains, and the player has to weigh which to fear more.

## Behaviour pattern

1. Within 180 lines above the player: 20-tick load glow, then fires (thump).
2. A target marker (a 2-frame flashing ring drawn by the bullet bucket at the player's position at the
   moment of firing) appears at once. The shell itself is drawn rising then falling as in DOME MORTAR
   but is not shootable.
3. After 75 ticks the shell lands on the marker: a 6-way splash at 1.25 px/tick, plus the 8×8 landing
   point itself is lethal for 4 ticks.
4. Cooldown 160 ticks. Fires regardless of the player being above it.

## Placement rules

- S5 bastion tops and the keep approach; at most 2 per screen, so at most 2 markers at once.
- Do not place markers' likely landing zones (the player band) on top of a GALLERY GUN curtain's
  densest beat; authors check the ripple phase.

## Readability

A fat tube in a stone ring; the marker is a bright ring that pulses. One glance at the floor tells the
player where not to be in a second's time.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
