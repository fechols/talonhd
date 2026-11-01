+++
id = "enemy.s4.pressure_tank"
kind = "enemy"
name = "PRESSURETANK"
stage = "stage.s4"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [4, 4]
angles = 1
anchor = "center"
identity = "a round industrial pressure tank seen from directly above: a domed yellow-and-black hazard-banded lid, bolted hatch, light blue crystal veins glowing through cracks"
silhouette = "circle with a concentric hazard band, small square hatch, branching vein lines"
proportions = "tank fills the patch; band a sixth of the radius; hatch a fifth of the diameter"
materials = "mid grey steel dome, yellow and black hazard band, dark grey bolts, light blue and purple crystal veins"
acting = "sits and hisses, veins pulsing faster as it is hit, then bursts in a ring of shards"
forbidden = ["eyes or a face", "people", "skull or danger symbols", "brand logos", "fire or flames before death"]
hurtbox = { x = -14, y = -14, w = 28, h = 28 }
domain = "ground"
hp = 6
contact_damage = 0
score = 500
weak_to = "seeker"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "emplacement"
params = { idle_fire = false, crack_stages = 3, chain_radius_px = 48, chain_delay_ticks = 12 }
attack = { kind = "ring", count = 12, spread_deg = 30, speed_q8 = 256, telegraph_ticks = 0, cooldown_ticks = 0 }
placement = "S4 pipe yards and pump hall; singly or in clusters of up to 4; always with a safe approach lane"
sprite_budget = { channels_on_line = 0, bullets_max = 12 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 12
note = "mid grey steel dome"
required = true

[[colors]]
role = "hazard"
c64 = 7
note = "yellow hazard band"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal veins"
required = true

[[colors]]
role = "bolts"
c64 = 11
note = "dark grey bolts and band stripes"
required = false

[[animations]]
id = "crack"
frames = 3
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the crystal veins spread wider across the dome in three stages, the lid bulging"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the tank torn open like a flower of steel petals from inside, crystal shards scattered dark"

[ludo]
sprite_prompt = "a round industrial pressure tank seen from directly above: a domed yellow-and-black hazard-banded lid, bolted hatch, light blue crystal veins glowing through cracks, top-down view"
+++

# PRESSURETANK

## Concept

(Display name `PRESSURETANK` fits the 12-character rule; the storyboard calls it "PRESSURE TANK".)

The pipe yards store crystal under pressure. A PRESSURETANK never fires while it lives; it **fires when
you kill it**: a 12-way ring of slow shards. It teaches that **when and where you kill something
matters**: shoot it from far away and the ring reaches you thin and slow; shoot it point-blank and you are
in the densest part. SEEKER darts, fired from across the screen, are the clean answer. Clusters chain.

## Behaviour pattern

- Inert. Each 2 HP of damage advances a baked crack stage (veins spread), so its remaining HP is visible.
- On death: a 12-way ring at 1 px/tick from its centre, with no telegraph beyond the third crack stage.
- Any other PRESSURETANK within 48 px of a bursting tank bursts 12 ticks later (chain), each with its
  own ring, giving a big score cascade and a dangerous bullet field.
- Does not care whether the player is above or below it.

## Placement rules

- S4 pipe yards and pump hall; singly or in clusters of up to 4.
- At most 24 ring bullets on screen from tanks: a cluster of 4 is authored with the outer two beyond chain
  range of the inner pair so a cascade is at most 2 bursts at once.
- Always leave an approach lane from which the player can kill a cluster from 120+ px away.

## Readability

A round tank with hazard stripes says "don't stand next to this when it pops". The crack stages count
down its HP without a health bar.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
