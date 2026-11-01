+++
id = "enemy.s6.moth"
kind = "enemy"
name = "MOTH"
stage = "stage.s6"
faction = "lattice"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
expanded = false
angles = 1
anchor = "center"
identity = "a broad drifting crystal drone seen from directly above: four wide purple triangular panels around a light blue hexagonal hub, seams where it will split"
silhouette = "wide four-panel kite, hexagonal hub, visible split seams"
proportions = "as wide as long; each panel a quarter; hub a fifth of the width"
materials = "purple faceted crystal panels, light blue hub and seam glow, dark grey edges"
acting = "drifts lazily on the air, panels slowly tilting; when struck it cracks along its seams into three"
forbidden = ["insect bodies, antennae or legs", "eyes or a face", "wing patterns like eyespots", "organic or feathery shapes", "butterfly shapes"]
hurtbox = { x = -16, y = -15, w = 32, h = 30 }
domain = "air"
hp = 3
contact_damage = 1
score = 300
weak_to = "burst"
resists = "none"
drops = { spark = 0, all_children_bonus = 1000 }

[behavior]
archetype = "splitter"
params = { drift_q8 = 160, wobble_px = 16, wobble_period_ticks = 120, split_on = "first_hit", children = 3, child_class = "S", child_hp = 1, child_score = 100, child_spread_deg = 120, child_speed_q8 = 224, child_lifetime_ticks = 300 }
attack = { kind = "aimed", count = 1, spread_deg = 0, speed_q8 = 320, telegraph_ticks = 12, cooldown_ticks = 120 }
placement = "S6 canopy and temple ruins; enters from the top; parents never below line 104; at most 2 parents on screen"
sprite_budget = { channels_on_line = 3, bullets_max = 4 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 4
note = "purple panels (private ink, parent and children)"
required = true

[[colors]]
role = "hub"
c64 = 14
note = "light blue hub and seam glow (shared pair, proposed)"
required = false

[[colors]]
role = "edge"
c64 = 11
note = "dark grey edges (shared pair, proposed)"
required = false

[[animations]]
id = "drift"
frames = 4
ticks_per_frame = 8
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "the four panels tilt slowly in turn as it floats, the hub glowing softly"

[[animations]]
id = "split"
frames = 3
ticks_per_frame = 4
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "bright cracks race along the seams and the drone breaks apart into three triangular pieces flying outward"

[[animations]]
id = "child"
frames = 2
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "a single purple triangular crystal shard tumbling slowly as it drifts"

[ludo]
sprite_prompt = "a broad drifting crystal drone seen from directly above: four wide purple triangular panels around a light blue hexagonal hub, seams where it will split, top-down view"
+++

# MOTH

## Concept

The jungle drones the Lattice grows in its crystal flowers. A MOTH is fragile and slow, and the first hit
**splits it into three** tumbling shards that drift apart. It teaches **wide coverage after a split**:
one narrow stream leaves shards flying everywhere; a BURST spread catches the parent and all three
children together, and clearing all three pays a bonus.

## Behaviour pattern

- Drifts down at 0.625 px/tick with a ±16 px sine wobble (120-tick period).
- Every 120 ticks (rank 1+) the hub flares 12 ticks and fires one aimed shot at 1.25 px/tick.
- First hit of any weapon: the parent splits (12 ticks) into 3 S shards spread 120 degrees apart (the
  first one pointing away from the player), each moving at 0.875 px/tick, 1 HP, 100 points. Shards never
  fire, but contact kills. Shards live 300 ticks then fade.
- Killing all 3 shards of one MOTH pays a 1,000 bonus.

## Placement rules

- S6 canopy and temple ruins; at most 2 parents on screen and at most 6 shards alive.
- Parents are kept above line 104, so their split happens above the player band; shards fan outward and
  one S shard per line at most reaches the band.
- The split costs: parent M (2 channels) becomes up to 3 S on the same lines briefly (3 channels).

## Readability

A big purple kite with glowing cracks already drawn on it: the seams show where it will break. The
shards keep the parent's colour so the player knows they belong to it.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
