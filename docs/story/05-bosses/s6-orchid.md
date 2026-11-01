+++
id = "boss.s6.orchid"
kind = "boss"
name = "ORCHID"
title = "the colossal crystal flower over the launch apron"
stage = "stage.s6"
status = "approved"
revision = 2
layer = "hybrid"
hp = 280
weak_to = "seeker"
score = 80000
time_bonus = { par_s = 90, max = 80000 }
reward = "burst"
music_cue = "cue.boss"

# Visual identity: the core is a 4x4 sprite grid; the six petals are bitmap patches on the apron
sprite_class = "B"
cells = [4, 4]
angles = 1
anchor = "center"
identity = "a colossal flower of faceted crystal: a hexagonal prism core ringed by six angular kite-shaped petal pods, all hard planes and sharp edges"
silhouette = "a hexagon centre with six kite-shaped blades around it, like a geometric star"
proportions = "core 96x84 display px (4x4 sprite grid); each petal pod 48x40 (a 6x5-cell bitmap patch; its drifting seed form an M 2x2 sprite), ring about 208 px across and 120 px tall"
materials = "purple and light blue crystal with white glints, a bright yellow-white pulse inside the core, thin dark grey metal veins from the launch gantry it grew on"
acting = "the petal ring clicks round like a dial, the core pulses before each volley, detached petals glide slowly and turn their points toward the TALON"
forbidden = ["organic textures", "leaves or stems", "pollen", "faces", "eyes", "mouths", "soft curves"]
hurtbox = { x = -40, y = -36, w = 80, h = 72 }

[[colors]]
role = "crystal"
c64 = 4
note = "purple facets (core and petals)"
required = true

[[colors]]
role = "crystal_light"
c64 = 14
note = "light blue facet planes; sprites use 14, never the TALON's private cyan"
required = true

[[colors]]
role = "glow"
c64 = 7
note = "yellow-white pulse in the core before volleys"
required = true

[[colors]]
role = "veins"
c64 = 11
note = "dark grey metal veins (from the rusted gantry)"
required = false

[[parts]]
id = "core"
layer = "sprite"
sprite_class = "B"
cells = [4, 4]
hp = 160
weak = false
note = "takes 1/2 damage while four or more petals remain"

[[parts]]
id = "petal_1"
layer = "bitmap"
cell_rect = [6, 5]
hp = 20
weak = true
note = "SEEKER double damage; all six petals share this block. Patches per socket: closed, tip lit, cracked, empty socket"

[[parts]]
id = "petal_2"
layer = "bitmap"
cell_rect = [6, 5]
hp = 20
weak = true

[[parts]]
id = "petal_3"
layer = "bitmap"
cell_rect = [6, 5]
hp = 20
weak = true

[[parts]]
id = "petal_4"
layer = "bitmap"
cell_rect = [6, 5]
hp = 20
weak = true

[[parts]]
id = "petal_5"
layer = "bitmap"
cell_rect = [6, 5]
hp = 20
weak = true

[[parts]]
id = "petal_6"
layer = "bitmap"
cell_rect = [6, 5]
hp = 20
weak = true

[[parts]]
id = "seed"
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
hp = 0
weak = true
note = "the sprite form of a detached petal (SEED DRIFT): hp 0 here, it carries the remaining HP and SEEKER weakness of the petal it came from; never more than one at a time"

[[phases]]
from_hp = 280
to_hp = 180
patterns = ["p1_dial", "p1_petal_fans", "p1_bloom_call"]
note = "the ring of six petals turns on the dial; core shielded to half damage"

[[phases]]
from_hp = 180
to_hp = 0
patterns = ["p2_drift", "p2_core_rings", "p2_prism_lance"]
note = "surviving petals tear free one at a time as seed sprites and home slowly; the core takes full damage"

[[patterns]]
id = "p1_dial"
name = "DIAL TURN"
telegraph = { ticks = 20, cue = "the core pulses yellow once; a ticking chime" }
attack = { kind = "fixed", count = 1, speed_q8 = 384, per_petal = true, direction = "outward along each petal point" }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 6 }
counterplay = "The ring clicks 60 degrees and every visible petal fires one shot outward along its point. The shots fan away from the centre: the space directly under the core is safe."

[[patterns]]
id = "p1_petal_fans"
name = "PETAL FANS"
telegraph = { ticks = 30, cue = "the two lower petals light at their tips" }
attack = { kind = "spread", count = 5, spread_deg = 40, speed_q8 = 352, aimed = true, petals = 2, volleys = 2 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 20 }
counterplay = "Two aimed fans from the lower petals; the fans cross above you, so move sideways early rather than late."

[[patterns]]
id = "p1_bloom_call"
name = "BLOOM CALL"
telegraph = { ticks = 40, cue = "BLOOM flower patches on the apron begin to open" }
attack = { kind = "ring", count = 8, speed_q8 = 256, enemy = "enemy.s6.bloom", blooms = 2 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 16 }
counterplay = "Two ground BLOOMs open and fire rings. Hit them with BEAM while open for the bonus, or stay between them."

[[patterns]]
id = "p2_drift"
name = "SEED DRIFT"
telegraph = { ticks = 30, cue = "one petal flashes white and springs out of its socket, leaving the socket patch empty" }
attack = { kind = "ram", count = 1, homing_q8 = 6, speed_q8 = 160, life_ticks = 360, fires = { kind = "aimed", count = 1, speed_q8 = 320, interval_ticks = 60 } }
sprite_budget = { boss_channels = 6, bucket_channels = 2, bullets_max = 10 }
counterplay = "One petal at a time glides toward you as a seed, slow and turning wide. Lead it in a circle and SEEKER darts break it (its remaining HP, double damage); after 360 ticks it settles back into its socket."

[[patterns]]
id = "p2_core_rings"
name = "PRISM RINGS"
telegraph = { ticks = 24, cue = "core pulses brighter and faster" }
attack = { kind = "ring", count = 12, spread_deg = 360, speed_q8 = 288, rings = 2, offset_deg = 15, gap_ticks = 18 }
sprite_budget = { boss_channels = 6, bucket_channels = 2, bullets_max = 24 }
counterplay = "Two offset rings; take the first gap, then step half a gap."

[[patterns]]
id = "p2_prism_lance"
name = "PRISM LANCE"
telegraph = { ticks = 40, cue = "the core's lower facet glows and a dotted guide line drops to the TALON's x" }
attack = { kind = "beam", count = 1, width_px = 16, aimed = true, duration_ticks = 45 }
sprite_budget = { boss_channels = 6, bucket_channels = 1, bullets_max = 1 }
counterplay = "A vertical lance at your x when the telegraph ends; step out of the guide line and shoot the core while it fires."

[[occupancy]]
pose = "dial"
rows = ["1111", "1111", "1111", "1111"]
note = "core centre at line 42: its four 4-wide rows fill lines 0-83, above the ceiling; the six petals are bitmap (0 channels): core 4 + buckets 2 = 6"

[[occupancy]]
pose = "drift_beside_core"
rows = ["111111", "111111", "1111", "1111"]
note = "the one seed sprite (M 2x2) beside the core: core 4 + seed 2 + buckets 2 = 8 on its two bands"

[[occupancy]]
pose = "drift_low"
rows = ["11", "11"]
note = "the seed below line 88: seed 2 + ship 2 + inner pods 2 + buckets 2 = 8"

[[animations]]
id = "core_pulse"
frames = 4
ticks_per_frame = 5
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the hexagonal crystal core pulses with a light that swells and fades from the centre outward"

[[animations]]
id = "petal_spin"
frames = 8
ticks_per_frame = 4
loop = true
model = "offline"
duration = 0
request_frames = 0
motion = "petal master rotated offline in 8 angles so its point can face the TALON while drifting"

[[animations]]
id = "petal_unlock"
frames = 3
ticks_per_frame = 4
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "a kite-shaped crystal blade flashes and springs free from its socket"

[ludo]
concept_prompt = "Top-down orthographic view of a colossal geometric flower made entirely of faceted purple and light blue crystal: a hexagonal prism core glowing yellow-white inside, ringed by six sharp kite-shaped crystal blades like a star, thin dark grey metal veins at the base, hanging over an overgrown concrete launch pad. All hard planes and crisp edges, nothing organic. Lit from the upper left."
sprite_prompt = "top-down view of a hexagonal faceted purple and light blue crystal prism core with a yellow-white glow inside, crisp edges"
petal_prompt = "top-down view of one sharp kite-shaped faceted purple and light blue crystal blade, point facing up"
+++

# ORCHID

# Personality and role

The most beautiful thing the Lattice ever made, and the most patient. It grew from the old launch gantry
over the VERDIGRIS apron and holds the way up. It never hurries: the ring clicks round like a dial,
the petals glide. Phase 2 turns that grace into a slow hunt, and SEEKER, the reward from the QUADRANT,
answers it directly.

# Arena

The launch apron at the end of VERDIGRIS: cracked concrete overgrown with jungle, the rusted gantry
folded under the flower, ground BLOOMs in the cracks between the slabs. The six petals lie open on the
apron as bitmap patches around the gantry's head, and the core floats just above them as a sprite. The
scroll stops with the flower's centre at line 42: the core fills lines 0-83, the upper petal pair straddles
the top edge, the side pair flanks the core, and the lower pair lies below the ceiling, where the TALON
flies right over them.

# Phases and patterns

**Phase 1 (280 → 180).** The six petals sit on a hexagonal dial around the core. DIAL TURN clicks the
ring 60 degrees (every socket swaps its patch under a sparkle streak in the bucket, so cracked petals and
gaps visibly move round) and each visible petal fires outward;
PETAL FANS sends aimed fans from the lower pair; BLOOM CALL wakes two ground BLOOMs. The core takes half
damage while four or more petals remain; petals take double from SEEKER.

**Phase 2 (180 → 0).** The surviving petals spring out of their sockets one at a time and drift toward
the TALON as seed sprites (SEED DRIFT), slow and wide-turning, firing a single aimed shot every second. The core fires PRISM RINGS and PRISM
LANCE at full vulnerability. If every petal is gone, SEED DRIFT is skipped.

**Defeat.** The core cracks along every facet, the petals fall flat on the apron as dark crystal, and
the gantry beneath is revealed: the Accord's way up.

# Budget notes

- The core is a 4x4 grid, 4 wide on every row, so it stays on lines 0-83 and the TALON is clamped at
  line 88: core 4 + buckets 2 = 6.
- The six petals on the dial are bitmap patches: 0 channels wherever they sit, including the lower pair
  under the TALON. A dial click rewrites six 6x5-cell patches (180 cells), so it is spread over two
  frames (the DMA budget is 100 cells per frame) under the sparkle streak.
- SEED DRIFT: only one seed sprite exists at a time. Beside the core: core 4 + seed 2 + buckets 2 = 8.
  Below line 88: seed 2 + ship 2 + inner pods 2 + buckets 2 = 8. No band ever carries the core and more
  than 2 petal channels.

# Weapon reward

The CORE SPARK moves the gauge cursor to **BURST**, which shreds the ANVIL's hull turrets at close range.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-31): sprite expansion removed at the user's request; bigger grids instead
