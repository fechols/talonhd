+++
id = "enemy.s3.dust_devil"
kind = "enemy"
name = "DUST DEVIL"
stage = "stage.s3"
faction = "lattice"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
expanded = false
angles = 1
anchor = "center"
identity = "a spinning three-bladed crystal rotor drone seen from directly above: purple triangular blades around a light blue hexagonal hub, a faint ring of sand beneath"
silhouette = "three-blade pinwheel, hexagonal hub, round blur ring"
proportions = "blades reach the cell edge; hub a third of the diameter"
materials = "purple faceted crystal blades, light blue hub, dark grey blade edges"
acting = "spins without pause, circling in tight groups, one peels off and darts at the TALON"
forbidden = ["eyes or a face", "propeller aircraft", "organic shapes", "more or fewer than three blades", "a tornado drawn as a funnel"]
hurtbox = { x = -8, y = -8, w = 16, h = 16 }
domain = "air"
hp = 2
contact_damage = 1
score = 150
weak_to = "burst"
resists = "none"
drops = { spark = 0, group_clear = 1 }

[behavior]
archetype = "orbiter"
params = { group = 4, radius_px = 28, angular_q8 = 6, centre_drift_q8 = 128, peel_every_ticks = 60, peel_speed_q8 = 640 }
attack = { kind = "aimed", count = 1, spread_deg = 0, speed_q8 = 384, telegraph_ticks = 8, cooldown_ticks = 0 }
placement = "S3 open desert and freight yard; groups of 4 around a centre that drifts down with the ground; centre spawns above line 40"
sprite_budget = { channels_on_line = 3, bullets_max = 4 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 4
note = "purple blades (private ink)"
required = true

[[colors]]
role = "hub"
c64 = 14
note = "light blue hub (shared pair, proposed)"
required = false

[[colors]]
role = "edge"
c64 = 11
note = "dark grey blade edges (shared pair, proposed)"
required = false

[[animations]]
id = "spin"
frames = 4
ticks_per_frame = 2
loop = true
model = "offline"
motion = "4 rotation steps of the three-blade master (120-degree symmetry) rotated by our tools"

[[animations]]
id = "flare"
frames = 2
ticks_per_frame = 4
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the hub flashes bright and the blades tilt outward"

[ludo]
sprite_prompt = "a spinning three-bladed crystal rotor drone seen from directly above: purple triangular blades around a light blue hexagonal hub, a faint ring of sand beneath, top-down view"
+++

# DUST DEVIL

## Concept

Lattice rotor drones that ride the desert thermals in knots of four. A group circles a point that
scrolls down with the ground; every second one peels away at the player and fires on the way. DUST
DEVILs teach **clusters**: a tight group is one target for a wide weapon, and BURST's 5-way flak clears
a whole knot in one press if the player dares to get close.

## Behaviour pattern

- 4 DEVILs orbit a centre point at radius 28 px, 6/256 of a turn per tick (a revolution in ~0.7 s).
  The centre drifts down at 0.5 px/tick (ground speed).
- Every 60 ticks the DEVIL nearest the player peels off: 8-tick hub flare, one aimed shot at
  1.5 px/tick, then a straight dash at 2.5 px/tick toward where the player was, exiting the screen.
- When the centre reaches line 120 the remaining DEVILs all peel at once, staggered 6 ticks.
- Killing all four drops 1 spark from the last.

## Placement rules

- S3 open desert and freight yard. At most 2 groups on screen.
- An orbit is 56+24 px wide, so a group puts up to 3 S sprites on a line; the centre reaches the player
  band only after the forced peel, so the band never holds an orbiting group.
- Not on the same lines as a VULTURE.

## Readability

A pinwheel that never stops turning: its spin says "air", its purple says "Lattice". Four in a ring read
as one object, which is the lesson.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
