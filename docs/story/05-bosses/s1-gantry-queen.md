+++
id = "boss.s1.gantry_queen"
kind = "boss"
name = "GANTRY QUEEN"
title = "the recrystallised control tower of Breakwater airbase"
stage = "stage.s1"
status = "approved"
revision = 2
layer = "hybrid"
hp = 180
weak_to = "burst"
score = 30000
time_bonus = { par_s = 60, max = 30000 }
reward = "beam"
music_cue = "cue.boss"

# Visual identity (the sprite core; legs and arm cannons are bitmap patches, see [[parts]])
sprite_class = "B"
cells = [4, 6]
expanded = false
angles = 1
anchor = "center"
identity = "an airfield control tower turned walker: octagonal glass cab on a ribbed concrete shaft, crystal plates bolted over it, two stubby cannon arms"
silhouette = "octagon on a square, two side arms, tapering lower block, four splayed legs"
proportions = "cab 4 cells wide and 3 rows deep, shaft tapering to 2 cells for the last 2 rows; each arm cannon a 6x6-cell bitmap patch braced on a forward leg beside the cab"
materials = "white-grey concrete, smoked glass cab ringed with purple crystal facets, light blue crystal armour plates, hazard-striped radar mast"
acting = "rises slowly with its shadow shrinking under it, plants each leg with a jolt, swivels the cab to track the TALON, shudders when plates crack"
forbidden = ["faces", "eyes", "organic limbs", "national markings", "real airport logos", "visible people"]
hurtbox = { x = -40, y = -60, w = 80, h = 100 }

[[colors]]
role = "primary"
c64 = 15
note = "light grey concrete shaft and cab frame"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal armour plates (the BURST weak parts); sprite cells use 14, never the TALON's private cyan"
required = true

[[colors]]
role = "secondary"
c64 = 4
note = "purple facet ring around the cab and crystal seams"
required = true

[[colors]]
role = "accent"
c64 = 7
note = "yellow hazard stripes on the radar mast and arm-cannon muzzles"
required = false

[[colors]]
role = "shadow"
c64 = 11
note = "dark grey glass and panel lines"
required = false

[[parts]]
id = "leg_nw"
layer = "bitmap"
cell_rect = [6, 6]
hp = 15
weak = false
note = "planted on the apron; patches: planted, flexing, cracked, broken stump"

[[parts]]
id = "leg_ne"
layer = "bitmap"
cell_rect = [6, 6]
hp = 15
weak = false

[[parts]]
id = "leg_sw"
layer = "bitmap"
cell_rect = [6, 6]
hp = 15
weak = false

[[parts]]
id = "leg_se"
layer = "bitmap"
cell_rect = [6, 6]
hp = 15
weak = false

[[parts]]
id = "arm_left"
layer = "bitmap"
cell_rect = [6, 6]
hp = 0
weak = false
note = "arm cannon braced on the forward leg's shoulder (48x48 display); invulnerable; patches: stowed, raised, muzzle flash, recoil, wreck on the apron"

[[parts]]
id = "arm_right"
layer = "bitmap"
cell_rect = [6, 6]
hp = 0
weak = false

[[parts]]
id = "plates"
layer = "sprite"
sprite_class = "B"
cells = [4, 6]
hp = 120
weak = true
note = "the core; its crystal armour plates take double damage from BURST"

[[phases]]
from_hp = 180
to_hp = 120
patterns = ["p1_triple", "p1_launch", "p1_mast_sweep"]
note = "phase ends when all four legs are destroyed (60 HP); legs take damage from any weapon"

[[phases]]
from_hp = 120
to_hp = 0
patterns = ["p2_ring", "p2_double_ring", "p2_cab_dash", "p1_triple"]
note = "legs snap off, the arm-cannon patches swap to wreck patches on the apron, the core hovers"

[[patterns]]
id = "p1_triple"
name = "TRIPLE FAN"
telegraph = { ticks = 24, cue = "both arm muzzles flash yellow" }
attack = { kind = "spread", count = 3, spread_deg = 30, speed_q8 = 384, aimed = true, volleys = 2, gap_ticks = 20 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 12 }
counterplay = "Aimed at the TALON's position at the flash: sidestep one ship width after the flash and the fan opens around you."

[[patterns]]
id = "p1_launch"
name = "HANGAR CALL"
telegraph = { ticks = 30, cue = "the two hangar-door patches either side of the apron slide open" }
attack = { kind = "launch", count = 3, enemy = "enemy.common.wasp", interval_ticks = 12 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 6 }
counterplay = "WASPs leave the doors on fixed S-curves; the leader drops a spark. Kill them at the door with FLANK or ignore them and stay low."

[[patterns]]
id = "p1_mast_sweep"
name = "MAST SWEEP"
telegraph = { ticks = 36, cue = "radar mast spins up, a dotted arc of bucket sparks traces the sweep" }
attack = { kind = "fixed", count = 12, spread_deg = 120, speed_q8 = 320, order = "left_to_right", interval_ticks = 4 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 12 }
counterplay = "A rotating fixed stream: move against the sweep direction and pass through it as the stream turns away."

[[patterns]]
id = "p2_ring"
name = "RING BURST"
telegraph = { ticks = 20, cue = "the purple facet ring glows and the core drops 4 px" }
attack = { kind = "ring", count = 12, spread_deg = 360, speed_q8 = 320 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 16 }
counterplay = "Twelve-way ring with 30-degree gaps; at the bottom of the screen the gaps are a ship width apart. Stay low and slide into a gap."

[[patterns]]
id = "p2_double_ring"
name = "OFFSET RINGS"
telegraph = { ticks = 24, cue = "facet ring glows twice" }
attack = { kind = "ring", count = 10, spread_deg = 360, speed_q8 = 288, rings = 2, offset_deg = 18, gap_ticks = 16 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 20 }
counterplay = "The second ring is rotated half a gap: after slipping the first ring, step half a gap sideways."

[[patterns]]
id = "p2_cab_dash"
name = "CAB DASH"
telegraph = { ticks = 30, cue = "core drifts to one side, shadow stretches toward the other wall" }
attack = { kind = "ram", count = 1, speed_q8 = 768, path = "horizontal across the top band, above line 88" }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 4 }
counterplay = "The dash stays above the boss ceiling and cannot touch you; it is a punish window: the core stops at the far wall for 40 ticks with its plates exposed."

[[occupancy]]
pose = "phase1"
rows = ["1111", "1111", "1111", "1111", "0110", "0110"]
note = "rows 0-3 (lines 0-83) are 4 wide above line 88 (the arm cannons beside rows 1-2 are bitmap, 0 channels); rows 4-5 taper to the 2-wide shaft below line 88"

[[occupancy]]
pose = "phase2_hover"
rows = ["1111", "1111", "1111", "1111", "0110", "0000"]

[[animations]]
id = "rise"
frames = 4
ticks_per_frame = 8
loop = false
model = "hydra-plus"
duration = 4
request_frames = 25
motion = "rises straight up out of a square pit, the shadow beneath shrinking and sliding down-right"

[[animations]]
id = "cab_turn"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the octagonal cab swivels slowly left and right, crystal plates catching the light"

[[animations]]
id = "hover"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "floats in place with a gentle bob, loose crystal chips falling from the broken shaft"

[[animations]]
id = "leg_break"
frames = 4
ticks_per_frame = 4
loop = false
model = "offline"
duration = 0
request_frames = 0
motion = "bitmap patch sequence: leg flexes, cracks along the crystal seam, snaps and lies on the apron"

[ludo]
concept_prompt = "Top-down orthographic view of an airfield control tower turned into a four-legged walking fortress by alien crystal: an octagonal smoked-glass cab on a ribbed light grey concrete shaft, light blue crystal armour plates bolted over it, a purple faceted ring around the cab, two stubby cannon arms, a yellow hazard-striped radar mast, four splayed concrete-and-crystal legs planted on a grey apron. Lit from the upper left, short shadow to the lower right."
sprite_prompt = "top-down view of an octagonal control tower cab on a tapering concrete shaft, light blue crystal armour plates, purple facet ring, yellow-striped radar mast, without legs or arms"
setpiece_prompt = "top-down view of four splayed walker legs of light grey concrete with crystal seams planted on a cracked grey hangar apron with white markings, each leg ending in a square foot pad, two stubby cannon arms with yellow-striped muzzles braced on the two forward legs"
+++

# GANTRY QUEEN

# Personality and role

The first guardian, and the first proof that the Lattice keeps what it takes. Breakwater's control tower
used to talk every Accord pilot home; now it stands up on four legs and guards the runway against them.
It is stately and slow, almost proud: a lighthouse that learned to walk. It teaches the boss grammar of
the whole game: break the outer parts, then fight the exposed core.

# Arena

The hangar apron at the end of the BREAKWATER airfield: grey concrete with white taxi lines, two hangar
doors left and right (bitmap patches that open for HANGAR CALL), crystal threading the cracks. The scroll
stops with the apron filling the screen. A square pit in the upper middle splits open and the QUEEN rises
out of it. Its four leg patches are baked into the apron around the pit, so the walker reads as standing
on the ground while its cab is a sprite above them. The two arm cannons are bitmap patches too, braced on
the forward legs' shoulders either side of the cab: once the cab has risen they swing up into place.

# Phases and patterns

**Phase 1 (180 → 120).** The legs are the targets: 15 HP each, any weapon. The cab rotates TRIPLE FAN
(two aimed 3-way fans), HANGAR CALL (three WASPs from the side doors) and MAST SWEEP (a rotating stream).
Each broken leg turns into a stump patch and tilts the cab one step, so the player sees progress. Shots
on the plates also count toward the core but the phase does not end until the fourth leg goes.

**Phase 2 (120 → 0).** The last leg snaps, both arm cannons crash onto the apron (their patches swap to
wreckage), and the core lifts into a hover. RING BURST and OFFSET RINGS fill the screen with readable rings; CAB DASH
throws the core across the top band and leaves it stalled against a wall for 40 ticks. TRIPLE FAN
returns from the core's own muzzle. The crystal plates take double damage from BURST: get close under
the hover between rings.

**Defeat.** The plates shatter in a chain of BLAST M explosions, the cab drops into the pit, the crystal
in the apron cracks go dark, and the CORE SPARK rises from the pit.

# Budget notes

- Phase 1, rows 0-3 (lines 0-83): core 4 + buckets 2 = 6. The arm cannons are fixed, so they are bitmap
  patches beside rows 1-2 and cost no channels. The player is clamped at line 88, so the ship never shares
  these lines.
- Rows 4-5 taper to the 2-wide shaft (lines 84-125): shaft 2 + ship 2 + inner pods 2 + buckets 2 = 8.
- Legs, arm cannons, hangar doors and wreckage are bitmap patches: 0 channels.
- HANGAR CALL WASPs (S, 1 channel) leave the side doors below line 88; even beside the cab they find the
  2 spare channels (core 4 + buckets 2 + 2 WASPs = 8). They are lowest mux priority and flicker first.
- Phase 2: core 4 above the ceiling + buckets 2 = 6.

# Weapon reward

The CORE SPARK moves the gauge cursor to **BEAM**, the lance that pierces the ASTROLABE's lens in stage 2.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-31): sprite expansion removed at the user's request; bigger grids instead
