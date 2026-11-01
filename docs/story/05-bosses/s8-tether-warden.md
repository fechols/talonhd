+++
id = "midboss.s8.tether_warden"
kind = "midboss"
name = "TETHER WARDEN"
title = "the crystal climbing machine on the SKYHOOK cable"
stage = "stage.s8"
status = "approved"
revision = 1
layer = "sprite"
hp = 150
weak_to = "flank"
score = 25000
time_bonus = { par_s = 45, max = 25000 }
drops = { spark = 3 }
music_cue = "cue.midboss"
sprite_class = "B"
cells = [4, 5]
expanded = false
angles = 1
anchor = "center"
identity = "a six-legged faceted crystal climbing machine seen from above, angular light blue limbs clamped around the elevator cable, a hexagonal purple body"
silhouette = "a hexagon body with six straight jointed legs splayed outward, two grip clamps at the bottom on the cable"
proportions = "96 px across the legs, 105 px long; the body a third of the width; the grip 48 px wide"
materials = "faceted purple crystal body, light blue crystal limbs with light grey metal joints, white glints, yellow charge ports"
acting = "climbs in mechanical strides, two legs at a time; hunches before striking; the grip clamps squeal against the cable"
forbidden = ["organic spider textures", "hair", "fangs", "eyes or faces", "abdomen shapes", "people", "cyan (the TALON's private ink)"]
hurtbox = { x = -24, y = -48, w = 48, h = 96 }

[[colors]]
role = "primary"
c64 = 4
note = "purple crystal body"
required = true

[[colors]]
role = "secondary"
c64 = 14
note = "light blue crystal legs"
required = true

[[colors]]
role = "accent"
c64 = 15
note = "light grey metal joints and grip clamps"
required = false

[[colors]]
role = "glow"
c64 = 7
note = "yellow charge ports"
required = false

[[parts]]
id = "body"
layer = "sprite"
sprite_class = "B"
cells = [4, 4]
hp = 110

[[parts]]
id = "grip"
layer = "sprite"
sprite_class = "W"
cells = [2, 1]
hp = 40
note = "row 4 of the grid; destroying it forces phase 2"

[[phases]]
from_hp = 150
to_hp = 70
patterns = ["p1_leg_volley", "p1_stride_drop"]

[[phases]]
from_hp = 70
to_hp = 0
patterns = ["p2_spin_climb", "p1_leg_volley"]

[[patterns]]
id = "p1_leg_volley"
name = "LEG VOLLEY"
telegraph = { ticks = 20, cue = "the six leg tips glow yellow" }
attack = { kind = "aimed", count = 6, spread_deg = 0, speed_q8 = 320, interval_ticks = 3 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 12 }
counterplay = "One aimed shot from each leg tip, outermost first, so the stream converges from both sides. Step diagonally once the fourth shot leaves."

[[patterns]]
id = "p1_stride_drop"
name = "STRIDE DROP"
telegraph = { ticks = 30, cue = "the machine hunches and the grip clamps flash" }
attack = { kind = "lob", count = 4, spread_deg = 80, speed_q8 = 192, burst_on_land = 3 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 12 }
counterplay = "It slides down the cable to the ceiling line and flings four crystal seeds sideways; each splits into 3 where it stops. The TALON's side lanes beside the cable are where the seeds land: centre under the grip is safe for the drop, then move back out."

[[patterns]]
id = "p2_spin_climb"
name = "SPIN CLIMB"
telegraph = { ticks = 36, cue = "the body rotates on the cable, legs fold in" }
attack = { kind = "spread", count = 8, spread_deg = 360, speed_q8 = 256, sweeps = 3, rotate_deg = 15 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 24 }
counterplay = "With the grip gone it spirals around the cable, firing three 8-way bursts each rotated 15 degrees. Stay far out in a side lane, where the bursts thin out, and keep FLANK guns on it as it passes."

[[occupancy]]
pose = "climb"
note = "rows 0-3 are 4 columns wide and stay above line 88; row 4, the grip clamp on the cable, tapers to 2 columns and is the only row that can share lines with the player band"
rows = ["1111", "1111", "1111", "1111", "0110"]

[[occupancy]]
pose = "stride_drop"
note = "the drop lowers the grid until row 3's bottom touches line 88; still only row 4 (2 columns) reaches below"
rows = ["1111", "1111", "1111", "1111", "0110"]

[[animations]]
id = "climb"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "climbs upward along a vertical cable in mechanical strides, two straight legs moving at a time, grip clamps sliding"

[[animations]]
id = "hunch"
frames = 2
ticks_per_frame = 15
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "pulls its six legs in close and lowers its body against the cable, charge ports brightening"

[[animations]]
id = "spin"
frames = 4
ticks_per_frame = 4
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "rotates around the cable with its legs folded inward, facets flashing"

[[animations]]
id = "break"
frames = 3
ticks_per_frame = 8
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the legs snap off one by one, the body cracks along its facets and falls away down the cable"

[ludo]
concept_prompt = "A six-legged climbing machine made of faceted crystal, seen from directly above, clamped onto a thick vertical elevator cable: a hexagonal purple crystal body, six straight angular light blue crystal legs with light grey metal joints, two grip clamps around the cable, yellow charge ports. Purely geometric and mechanical, not an animal. Orthographic top-down, lit from the upper left, against a dark cobalt sky."
sprite_prompt = "a six-legged faceted crystal climbing machine from directly above, hexagonal purple body, straight light blue crystal legs, grey metal joints, two clamps around a vertical cable"
+++

# TETHER WARDEN

## Personality and role

The SKYHOOK's caretaker, rebuilt by the Lattice into a sentry that climbs the cable alongside anything
that tries to follow it up. Mechanical, patient, relentless. It is the last midboss and the only one that
moves *with* the player along the stage's central "ground", the cable.

## Arena

Section 2 of SKYHOOK, orbit, around screen 12. The cable runs up the screen centre; the scroll continues
at stage speed. The Warden climbs down to meet the TALON and then holds station at the top of the view,
matching the scroll. CLIMBER POD turrets on the cable are paused during the fight. The boss ceiling
(line 88) applies.

## Phases and patterns

**Phase 1 (150 → 70).** LEG VOLLEY every 2 s; STRIDE DROP every third cycle. The grip clamp (row 4)
is a separate part: destroy it to force phase 2 early.

**Phase 2 (70 → 0).** The Warden loses its hold and spirals: SPIN CLIMB alternates with LEG VOLLEY.

**Weakness.** FLANK: its side guns fire left and right; a player in a side lane level with the grip
hits the hanging clamp, and in phase 2 the spiralling body passes through FLANK fire every turn. Double
damage.

## Budget notes

B 4x5 (96x105). Rows 0-3 are 4 columns wide (4 channels) and stay above line 88, where the budget is
4 boss + 2 bucket = 6 of 8. Row 4 is the grip clamp, 2 columns wide, the only row that can share lines
with the player band (ship 2 + inner pods 2 + buckets 2 + grip 2 = 8). The cable itself is bitmap.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
