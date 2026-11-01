+++
id = "boss.s3.trestle_titan"
kind = "boss"
name = "TRESTLE TITAN"
title = "the crystal-grown crane engine of the freight yard"
stage = "stage.s3"
status = "approved"
revision = 1
layer = "sprite"
hp = 220
weak_to = "seeker"
score = 50000
time_bonus = { par_s = 75, max = 50000 }
reward = "burst"
music_cue = "cue.boss"

sprite_class = "B"
cells = [4, 8]
expanded = false
angles = 1
anchor = "center"
identity = "a gantry crane fused to a steam locomotive boiler, straddling rails on four bogies, two crystal grab claws hanging on cables beneath its boom"
silhouette = "wide rectangular gantry, round boiler in the middle, two narrow claws dangling below"
proportions = "gantry and boiler 4 cells wide and 4 rows deep, a 2-wide firebox row, then two 1-cell claws on 3-row cables"
materials = "rust-orange girders, dark grey boiler with rivet bands, yellow hazard stripes, light blue and purple crystal ore growing from the seams, claws of solid faceted crystal"
acting = "rolls on its rails with a heavy sway, the boom lurches before each sweep, the boiler vents puff before it opens"
forbidden = ["faces", "eyes", "organic forms", "real railway liveries or logos", "visible crew"]
hurtbox = { x = -44, y = -84, w = 88, h = 100 }

[[colors]]
role = "primary"
c64 = 8
note = "rust-orange gantry girders"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey boiler and bogies"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal claws and ore growth (sprite cells avoid the TALON's private cyan)"
required = true

[[colors]]
role = "accent"
c64 = 4
note = "purple facets in the claws and boiler seams"
required = false

[[colors]]
role = "glow"
c64 = 7
note = "yellow hazard stripes and the open-boiler glow"
required = false

[[parts]]
id = "claw_left"
layer = "sprite"
sprite_class = "B"
cells = [1, 3]
hp = 50
weak = true
note = "SEEKER double damage; destroyed claws drop onto the tracks as bitmap wreck patches"

[[parts]]
id = "claw_right"
layer = "sprite"
sprite_class = "B"
cells = [1, 3]
hp = 50
weak = true

[[parts]]
id = "boiler"
layer = "sprite"
sprite_class = "B"
cells = [4, 5]
hp = 120
weak = false
note = "armoured until phase 2; the open firebox takes normal damage"

[[phases]]
from_hp = 220
to_hp = 120
patterns = ["p1_claw_sweep", "p1_claw_drop", "p1_gantry_guns"]
note = "ends when both claws are destroyed"

[[phases]]
from_hp = 120
to_hp = 0
patterns = ["p2_ore_fan", "p2_ore_rain", "p2_charge_line", "p1_gantry_guns"]
note = "the boiler door opens; the titan rolls up and down its rails"

[[patterns]]
id = "p1_claw_sweep"
name = "CLAW SWEEP"
telegraph = { ticks = 30, cue = "the boom lurches and both cables swing back" }
attack = { kind = "ram", count = 2, path = "pendulum arcs under the gantry, 120 degrees, mirrored", period_ticks = 90 }
sprite_budget = { boss_channels = 6, bucket_channels = 2, bullets_max = 6 }
counterplay = "The claws swing in mirror image and cross in the middle: sit at an edge while they swing in, cross the centre while they swing out. Each claw is fully exposed at the end of its arc."

[[patterns]]
id = "p1_claw_drop"
name = "CLAW DROP"
telegraph = { ticks = 24, cue = "one claw rises to the boom and its shadow grows on the tracks under the player" }
attack = { kind = "ram", count = 1, target = "player_x", drop_px = 64, hold_ticks = 40 }
sprite_budget = { boss_channels = 5, bucket_channels = 2, bullets_max = 4 }
counterplay = "Leave the shadow. The claw stays down for 40 ticks: the best SEEKER window, since every dart picks it."

[[patterns]]
id = "p1_gantry_guns"
name = "GANTRY GUNS"
telegraph = { ticks = 18, cue = "two muzzles on the gantry corners flash" }
attack = { kind = "aimed", count = 2, spread_deg = 0, speed_q8 = 448, volleys = 3, gap_ticks = 12 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 6 }
counterplay = "Two parallel aimed streams; a steady sideways drift beats them."

[[patterns]]
id = "p2_ore_fan"
name = "ORE SPRAY"
telegraph = { ticks = 30, cue = "boiler door swings open, yellow glow, three warning puffs" }
attack = { kind = "spread", count = 9, spread_deg = 100, speed_q8 = 352, volleys = 3, rotate_deg = 6 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 27 }
counterplay = "A 9-way fan that shifts 6 degrees per volley; the gaps widen with distance, so hang back near the bottom and step into the shifting gap."

[[patterns]]
id = "p2_ore_rain"
name = "ORE RAIN"
telegraph = { ticks = 36, cue = "the boiler coughs ore chunks straight up; they glint at the top of the screen" }
attack = { kind = "lob", count = 8, speed_q8 = 256, target = "random_columns_marked", marks_ticks = 30 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 8 }
counterplay = "Each chunk lands on a column marked by a shadow dot for 30 ticks; the unmarked columns are safe."

[[patterns]]
id = "p2_charge_line"
name = "FULL STEAM"
telegraph = { ticks = 40, cue = "whistle sound, wheels spin, the titan backs up to the top edge" }
attack = { kind = "ram", count = 1, speed_q8 = 512, path = "rolls down its rails to line 88 and back; only the 2-wide firebox row crosses the ceiling" }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 0 }
counterplay = "It runs on fixed rails at x 104-200: stay outside the rail lane. The ceiling clamp keeps you clear of the gantry; only the firebox noses down."

[[occupancy]]
pose = "phase1"
rows = ["1111", "1111", "1111", "1111", "0110", "1001", "1001", "1001"]
note = "rows 0-3 (lines 0-83) 4 wide above the ceiling; row 4 the 2-wide firebox; rows 5-7 the two 1-wide claws = 2 channels in the player band"

[[occupancy]]
pose = "phase2"
rows = ["1111", "1111", "1111", "1111", "0110", "0000", "0000", "0000"]

[[animations]]
id = "roll"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra-plus"
duration = 3
request_frames = 25
motion = "the crane engine rocks gently on its bogies as it rolls, the boom bobbing"

[[animations]]
id = "claw_swing"
frames = 6
ticks_per_frame = 4
loop = true
model = "offline"
duration = 0
request_frames = 0
motion = "claw master rotated offline in 6 swing angles along its cable"

[[animations]]
id = "boiler_open"
frames = 4
ticks_per_frame = 6
loop = false
model = "hydra-plus"
duration = 3
request_frames = 25
motion = "the round boiler door swings open and a yellow glow pours out with sparks"

[ludo]
concept_prompt = "Top-down orthographic view of a giant rust-orange gantry crane fused to a dark grey steam locomotive boiler, straddling two railway tracks on four bogies, yellow hazard stripes on the girders, light blue and purple crystal ore growing from the seams, two faceted crystal grab claws hanging on steel cables below the boom. Lit from the upper left, short shadow to the lower right."
sprite_prompt = "top-down view of a rust-orange gantry crane and dark grey boiler on rail bogies, crystal ore in the seams, two crystal claws on cables underneath"
+++

# TRESTLE TITAN

# Personality and role

The Lattice's railway foreman. It loaded the power trains that ran to the coast and now it blocks the
yard with its whole bulk. Heavy, loud and blunt: the claws do the grabbing, the boiler does the
shouting. It is the first boss built entirely from sprites at full 4x8 size and the showcase for the
taper rule: a broad gantry overhead, narrow claws below.

# Arena

The end of the REDLINE freight yard: four parallel tracks, switch points, crystal ore heaps and two
water towers at the edges (bitmap). The scroll stops with the titan's rails running down the middle of
the screen. The titan rolls a short distance along them (up and down) and never leaves the rail lane at
x 104-200.

# Phases and patterns

**Phase 1 (220 → 120).** The claws are the targets: 50 HP each, double damage from SEEKER. CLAW SWEEP
swings both claws in mirrored pendulum arcs; CLAW DROP slams one claw onto the player's shadow and
leaves it down for 40 ticks; GANTRY GUNS fire parallel aimed streams. A lost claw crashes onto the
tracks as a wreck patch and the remaining one swings faster.

**Phase 2 (120 → 0).** The boiler door opens. ORE SPRAY fans 9-way shifting volleys, ORE RAIN drops
marked chunks, FULL STEAM rolls the titan down its rails to the ceiling line. The open firebox takes
normal damage from everything.

**Defeat.** The boiler bursts in a column of BLAST L explosions, the gantry sags across the tracks as a
bitmap wreck, the crystal ore heaps in the yard go dark.

# Budget notes

- Lines 0-83: titan 4 + buckets 2 = 6 (the TALON is clamped below line 88).
- Line 84-104 (firebox row, 2 wide): titan 2 + ship 2 + inner pods 2 + buckets 2 = 8.
- Lines 105-167 (claw rows, two 1-wide claws): titan 2 + ship/pods 4 + buckets 2 = 8. During CLAW DROP
  the dropped claw stays 1 wide.
- WINGS 3 outer pods composite into the bucket on these lines as in the README.
- Phase 2: rows 5-7 empty, so the player band carries only the firebox when FULL STEAM reaches the ceiling.

# Weapon reward

The CORE SPARK moves the gauge cursor to **BURST**, the close-range flak that cracks the HYDRA PUMP core.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
