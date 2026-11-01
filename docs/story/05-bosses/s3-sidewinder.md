+++
id = "midboss.s3.sidewinder"
kind = "midboss"
name = "SIDEWINDER"
title = "the armoured rail engine and its four cars"
stage = "stage.s3"
status = "approved"
revision = 1
layer = "sprite"
hp = 120
weak_to = "seeker"
score = 14000
time_bonus = { par_s = 40, max = 14000 }
drops = { spark = 3 }
music_cue = "cue.midboss"
sprite_class = "B"
cells = [2, 4]
expanded = false
angles = 1
anchor = "center"
identity = "an armoured freight locomotive seen from above, grey plate hull with crystal ribs, pulling four turret cars up a single rail line"
silhouette = "long narrow segments in a line, blunt wedge nose, a round turret on every car"
proportions = "each segment twice as long as wide; the engine a little wider at the nose"
materials = "riveted grey armour plate, yellow and black hazard nose, light blue crystal ribs along each roof, dark grey couplers"
acting = "pulls level with the player, surges ahead, brakes back; cars sway on the couplers; turrets swing smoothly"
forbidden = ["people", "smoke stacks with steam (it runs on crystal power)", "eyes or faces", "organic growth", "real rail company liveries"]
hurtbox = { x = -22, y = -40, w = 44, h = 80 }

[[colors]]
role = "primary"
c64 = 12
note = "mid grey armour"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey couplers and turret wells"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal ribs (not cyan: cyan is the TALON's private ink)"
required = true

[[colors]]
role = "accent"
c64 = 7
note = "yellow hazard nose stripes"
required = false

[[parts]]
id = "engine"
layer = "sprite"
sprite_class = "B"
cells = [2, 4]
hp = 40

[[parts]]
id = "car_1"
layer = "sprite"
sprite_class = "B"
cells = [2, 4]
hp = 20

[[parts]]
id = "car_2"
layer = "sprite"
sprite_class = "B"
cells = [2, 4]
hp = 20

[[parts]]
id = "car_3"
layer = "sprite"
sprite_class = "B"
cells = [2, 4]
hp = 20

[[parts]]
id = "car_4"
layer = "sprite"
sprite_class = "B"
cells = [2, 4]
hp = 20

[[phases]]
from_hp = 120
to_hp = 40
patterns = ["p1_car_volley", "p1_surge"]

[[phases]]
from_hp = 40
to_hp = 0
patterns = ["p2_engine_fan", "p1_surge"]

[[patterns]]
id = "p1_car_volley"
name = "CAR VOLLEY"
telegraph = { ticks = 18, cue = "car turret ring flashes light blue" }
attack = { kind = "aimed", count = 2, spread_deg = 10, speed_q8 = 352, cars_firing = 2 }
sprite_budget = { boss_channels = 2, bucket_channels = 2, bullets_max = 8 }
counterplay = "Two cars at a time fire an aimed pair. The train is in its own lane; staying one ship-width out of its lane leaves time to see each flash."

[[patterns]]
id = "p1_surge"
name = "SURGE"
telegraph = { ticks = 24, cue = "engine nose lamps blink yellow, couplers tighten" }
attack = { kind = "ram", count = 1, spread_deg = 0, speed_q8 = 768, lane = "rail" }
sprite_budget = { boss_channels = 2, bucket_channels = 0, bullets_max = 0 }
counterplay = "The whole train dashes up its rail and then falls back below the screen bottom. Never park in the rail lane during the blink."

[[patterns]]
id = "p2_engine_fan"
name = "ENGINE FAN"
telegraph = { ticks = 30, cue = "engine roof crystal rib brightens" }
attack = { kind = "spread", count = 5, spread_deg = 60, speed_q8 = 320 }
sprite_budget = { boss_channels = 2, bucket_channels = 2, bullets_max = 10 }
counterplay = "With every car gone the bare engine weaves across both lanes firing 5-way fans. Fans open toward the player's side: cross behind the engine to its far side."

[[occupancy]]
pose = "train"
note = "segments are stacked on one vertical rail; no two segments share a raster line"
rows = ["11", "11", "11", "11"]

[[animations]]
id = "run"
frames = 2
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the engine rolls straight ahead along the rail, roof crystal rib pulsing, wheels shimmering at the sides"

[[animations]]
id = "turret_turn"
frames = 8
ticks_per_frame = 4
loop = true
model = "offline"
duration = 3
request_frames = 16
motion = "car turret rotates through eight headings toward the player's side"

[[animations]]
id = "car_break"
frames = 3
ticks_per_frame = 6
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the car's armour plates burst outward and it derails into a tumbling wreck"

[ludo]
concept_prompt = "An armoured freight locomotive seen from directly above, nose pointing up, riveted grey armour plates, yellow and black hazard stripes on the wedge nose, light blue faceted crystal ribs along the roof. Behind it four armoured flat cars, each with a round gun turret. Orthographic top-down, lit from the upper left, short shadow to the lower right on desert rail."
sprite_prompt = "an armoured locomotive from directly above, nose up, grey riveted plates, yellow hazard nose, light blue crystal ribs along the roof"
+++

# SIDEWINDER

## Personality and role

The Lattice's ore express, armoured for the run to the coast. It does not wait for the player: it
**runs alongside**, matching speed on a rail line in the mesa canyon, and fights in its own lane.
SIDEWINDER teaches **multi-segment targets** and why SEEKER exists: one dart per segment.

## Arena

Section 2 of REDLINE, the mesa canyon, around screen 12. A single rail runs up the left or right third
of the canyon floor (the level alternates which side per surge). The scroll does not stop: the train
enters from the bottom and holds pace beside the player for about 40 s. Canyon walls narrow the
dodging room to roughly 200 px outside the rail lane.

## Phases and patterns

**Phase 1 (120 → 40).** Engine + four cars. CAR VOLLEY from two cars at a time, SURGE every third
cycle. Destroying a car removes its segment and the train closes up.

**Phase 2 (40 → 0).** When all four cars are gone (or HP reaches 40) the engine leaves the rail and
weaves across the canyon firing ENGINE FAN, with a SURGE back to the rail between fans.

**Weakness.** SEEKER: one dart per on-screen target, so five segments soak five darts per salvo, each
for double damage.

## Budget notes

Each segment is B 2x4 (48x84 display px). The plan's "4x2" is read as 2 columns by 4 rows so that a
segment level with the TALON costs exactly the **2 channels** the player band allows (ship 2 + inner
pods 2 + buckets 2 + segment 2 = 8). Segments are stacked on one vertical rail and never share lines,
so the whole train costs 2 channels on every line it covers. In phase 2 the engine alone is still
2 columns wide. Turret frames are part of each segment's cells; bullets are bucket-layer.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
