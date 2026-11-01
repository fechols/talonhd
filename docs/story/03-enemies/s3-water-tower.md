+++
id = "enemy.s3.water_tower"
kind = "enemy"
name = "WATER TOWER"
stage = "stage.s3"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [6, 6]
angles = 1
anchor = "base"
identity = "a railway water tower seen from directly above: round tank lid on four splayed legs, split open by glowing purple crystal, long shadow"
silhouette = "big circle lid, four leg tips at the corners, crystal cluster in the lid, long cast shadow"
proportions = "lid fills two thirds of the patch; shadow runs one lid width to the lower right"
materials = "brown weathered wood and orange rust bands, dark grey shadow, purple and light blue crystal"
acting = "stands silent, the crystal in its tank pulses brighter and brighter, then spits a ring of shards"
forbidden = ["people", "railway logos", "eyes or a face", "water spraying like a fountain", "national markings"]
hurtbox = { x = -18, y = -36, w = 36, h = 36 }
domain = "ground"
hp = 16
contact_damage = 0
score = 1000
weak_to = "beam"
resists = "none"
drops = { spark = 1 }

[behavior]
archetype = "emplacement"
params = { pulse_frames = 3, collapse_frames = 3, fire_range_lines = 170 }
attack = { kind = "ring", count = 8, spread_deg = 45, speed_q8 = 288, telegraph_ticks = 30, cooldown_ticks = 120 }
placement = "S3 freight yard and desert halts, beside rails; one per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 16 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 9
note = "brown wooden tank"
required = true

[[colors]]
role = "rust"
c64 = 8
note = "orange rust bands"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal cluster"
required = true

[[colors]]
role = "crystal_light"
c64 = 14
note = "light blue pulse highlights"
required = false

[[animations]]
id = "pulse"
frames = 3
ticks_per_frame = 10
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the crystal cluster in the tank brightens in three steps until it blazes"

[[animations]]
id = "collapse"
frames = 3
ticks_per_frame = 8
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "the tank lid cracks and the whole tower folds sideways onto its shadow, crystal shattering"

[ludo]
sprite_prompt = "a railway water tower seen from directly above: round tank lid on four splayed legs, split open by glowing purple crystal, long shadow, top-down view"
+++

# WATER TOWER

## Concept

The railway's water towers are full of crystal now, not water, and each one is a small power node. A
WATER TOWER is the first **emplacement**: a tall, tough ground target (16 HP) that does not aim, just
fills the air with slow rings. It teaches **ring patterns**: rings have lanes, and lanes widen with
distance, so the safest place to face a ring is far away; the most rewarding is close with BEAM, which
pierces straight into it. It holds a spark.

## Behaviour pattern

- Active within 170 lines above the player. Every 120 ticks: 30-tick pulse telegraph (crystal brightens in
  three steps), then an 8-way ring at 1.125 px/tick. Each ring is rotated 22.5 degrees from the last.
- Loop 2+: a second ring 20 ticks after the first, offset by half a lane.
- Fires even when the player is above it (rings are radial).
- Death: collapses onto its shadow over 24 ticks (a baked 3-frame patch), dropping 1 spark.

## Placement rules

- S3 freight yard and desert halts, beside rails; one per screen.
- Rings are capped at 16 bullets per tower; never place a tower where its ring overlaps a VULTURE spread.

## Readability

A big round lid with a purple heart and a long shadow stands up off the ground. The three-step pulse
counts down the volley in plain sight.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
