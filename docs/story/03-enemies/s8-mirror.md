+++
id = "enemy.s8.mirror"
kind = "enemy"
name = "MIRROR"
stage = "stage.s8"
faction = "lattice"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
expanded = false
angles = 8
anchor = "center"
identity = "a flat hexagonal crystal mirror drone seen from directly above: a polished light grey face in a light blue faceted frame, a dark grey keel"
silhouette = "flat hexagon plate, thick faceted rim, short keel on one side"
proportions = "plate fills the cell grid; rim a sixth of the width; keel a third of the plate length"
materials = "polished light grey mirror face, light blue crystal frame, dark grey keel"
acting = "glides on a slow orbit, always turning its bright face toward the TALON, flashing as it catches light"
forbidden = ["eyes or a face", "a human reflection or figure in the mirror", "organic shapes", "frames with ornaments", "yellow"]
hurtbox = { x = -16, y = -16, w = 32, h = 32 }
domain = "air"
hp = 8
contact_damage = 1
score = 800
weak_to = "twin"
resists = "beam"
drops = { spark = 1 }

[behavior]
archetype = "orbiter"
params = { orbit_radius_px = 48, angular_q8 = 2, centre_drift_q8 = 96, face_player = true, face_step_ticks = 8, reflect = "beam", reflect_damage_to_enemies = 2, front_arc_deg = 90, front_damage_scale = 0.25 }
attack = { kind = "aimed", count = 3, spread_deg = 20, speed_q8 = 352, telegraph_ticks = 18, cooldown_ticks = 130 }
placement = "S8 orbit and Crown approach; one or two MIRRORs per screen, orbiting points above line 96"
sprite_budget = { channels_on_line = 2, bullets_max = 3 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "mirror"
c64 = 15
note = "light grey polished face (private ink)"
required = true

[[colors]]
role = "frame"
c64 = 14
note = "light blue crystal frame (shared pair, proposed)"
required = false

[[colors]]
role = "keel"
c64 = 11
note = "dark grey keel (shared pair, proposed)"
required = false

[[animations]]
id = "glint"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "a bright band of reflected light sweeps slowly across the mirror face from edge to edge"

[[animations]]
id = "face"
frames = 8
ticks_per_frame = 8
loop = false
model = "offline"
motion = "8 facings rotated from the master so the face can turn toward the player"

[[animations]]
id = "reflect"
frames = 2
ticks_per_frame = 2
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the whole mirror face blazes bright, flickering, as if struck by a beam"

[ludo]
sprite_prompt = "a flat hexagonal crystal mirror drone seen from directly above: a polished light grey face in a light blue faceted frame, a dark grey keel, top-down view"
+++

# MIRROR

## Concept

The CROWN's defence against the weapon that has served the player best on the ground. A MIRROR turns its
face to the TALON and **reflects BEAM**: the lance bends off the mirror at the reflected angle. It teaches
that **no weapon is universal** in the final stage, and offers a trick for those who learn the geometry:
a reflected BEAM damages other enemies it crosses. TWIN works fine; the edges and the keel take full
damage from anything.

## Behaviour pattern

- Orbits a centre point (radius 48 px, slow) that drifts down at 0.375 px/tick. Every 8 ticks it steps
  its face one of 8 facings toward the player.
- **Front arc (90 degrees around the face):** player shots other than BEAM do ×0.25. BEAM does nothing
  and is reflected: the lance's continuation leaves at the mirrored angle (one of 8 directions) and deals
  2 damage per tick to enemies on that line. Never to the player.
- **Sides and keel:** full damage from everything (TWIN ×2 as its weakness).
- Every 130 ticks: 18-tick glint telegraph, then a 3-way aimed spread at 1.375 px/tick.
- Drops 1 spark.

## Placement rules

- S8 orbit and Crown approach; one or two per screen, orbit centres above line 96; never in the player
  band.
- Pairs are placed so a reflected BEAM from one can strike the other or a SHARD group: a designed trick
  shot.

## Readability

A bright flat hexagon that always turns to face you. Its glint sweep is the "mirror" read even at C64
size, and its blazing reflect frame tells the BEAM player exactly why their lance just bent.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
