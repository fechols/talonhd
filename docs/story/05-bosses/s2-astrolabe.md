+++
id = "boss.s2.astrolabe"
kind = "boss"
name = "ASTROLABE"
title = "the observatory telescope turned sky cannon"
stage = "stage.s2"
status = "approved"
revision = 1
layer = "bitmap"
hp = 180
weak_to = "beam"
score = 40000
time_bonus = { par_s = 70, max = 40000 }
reward = "seeker"
music_cue = "cue.boss"

# Visual identity (bitmap set-piece)
cell_rect = [30, 16]
angles = 16
anchor = "center"
identity = "an opened white observatory dome holding three nested brass-and-crystal rings around a faceted telescope lens, violet crystal spilling over the dome rim"
silhouette = "a circle split into two shutter halves, three concentric rings, a hexagonal lens at the centre"
proportions = "dome 30 cells wide, 16 tall; outer ring 14 cells across, middle 10, inner 6, lens 4"
materials = "cream-white dome panels, gravel collar, brass ring frames gone to crystal, purple and light blue facets, a white-hot lens"
acting = "the shutters roll apart, rings turn at different speeds and directions, gaps line up like a combination lock before each beam"
forbidden = ["faces", "eyes", "zodiac or religious symbols", "real observatory names", "real-world star charts"]
hurtbox = { x = -48, y = -40, w = 96, h = 80 }

[[colors]]
role = "primary"
c64 = 1
note = "white dome panels (bitmap ink)"
required = true

[[colors]]
role = "frame"
c64 = 8
note = "brass-orange ring frames"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal overgrowth on rings and dome rim"
required = true

[[colors]]
role = "glow"
c64 = 3
note = "cyan lens and ring gap glow (bitmap ink, not a sprite)"
required = true

[[colors]]
role = "shadow"
c64 = 9
note = "brown gravel collar and inner dome shadow (stage background ink)"
required = false

[[parts]]
id = "ring_outer"
layer = "bitmap"
cell_rect = [14, 14]
hp = 35
weak = false
note = "16 rotation patches per ring, offline-rotated from one master; damage patch set: cracked, shattered"

[[parts]]
id = "ring_middle"
layer = "bitmap"
cell_rect = [10, 10]
hp = 35
weak = false

[[parts]]
id = "ring_inner"
layer = "bitmap"
cell_rect = [6, 6]
hp = 35
weak = false

[[parts]]
id = "lens"
layer = "bitmap"
cell_rect = [4, 4]
hp = 75
weak = true
note = "BEAM does double damage; armoured (no damage) while any ring still covers it except during LENS FLARE"

[[phases]]
from_hp = 180
to_hp = 75
patterns = ["p1_lane", "p1_ring_volley", "p1_mortar_call", "p1_lens_flare"]
note = "destroy the three rings in any order; each lost ring removes its own volley and speeds the others by 1/8"

[[phases]]
from_hp = 75
to_hp = 0
patterns = ["p2_twin_lanes", "p2_starburst", "p1_lens_flare"]
note = "naked lens; the dome shutters half-close and reopen on a cycle"

[[patterns]]
id = "p1_lane"
name = "SIGHT LANE"
telegraph = { ticks = 45, cue = "the three ring gaps align and a dotted cyan guide line drops from the lens down the screen" }
attack = { kind = "beam", count = 1, width_px = 16, sweep = "left_to_right_or_reverse", sweep_q8 = 256, duration_ticks = 90 }
sprite_budget = { boss_channels = 0, bucket_channels = 1, bullets_max = 1 }
counterplay = "The lane is a vertical column sweeping sideways at 1 px/tick. Start on the side it leaves from and follow behind it, or cross through it during the 10-tick gap when the rings slip out of line."

[[patterns]]
id = "p1_ring_volley"
name = "RING VOLLEY"
telegraph = { ticks = 20, cue = "notches on one ring glow yellow" }
attack = { kind = "fixed", count = 6, spread_deg = 360, speed_q8 = 320, per_ring = true, rotate_with_ring = true }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 18 }
counterplay = "Each ring fires from its own notches in the direction it is turning; the outer ring's shots spiral one way, the middle ring's the other. Watch the glowing ring and drift with its spin."

[[patterns]]
id = "p1_mortar_call"
name = "DOME MORTARS"
telegraph = { ticks = 30, cue = "two DOME MORTAR hatches on the gravel collar flip open" }
attack = { kind = "lob", count = 2, speed_q8 = 192, enemy = "enemy.s2.dome_mortar", target = "player_x" }
sprite_budget = { boss_channels = 0, bucket_channels = 1, bullets_max = 2 }
counterplay = "Slow arcing shells land where you were at the telegraph; a shadow dot marks the landing spot for 30 ticks."

[[patterns]]
id = "p1_lens_flare"
name = "LENS FLARE"
telegraph = { ticks = 30, cue = "lens brightens to white, rings stop" }
attack = { kind = "burst", count = 8, spread_deg = 70, speed_q8 = 448, aimed = true }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 8 }
counterplay = "The punish window: for 60 ticks after the burst the lens is open even with rings intact. Dodge the cone sideways, then hold BEAM on the lens."

[[patterns]]
id = "p2_twin_lanes"
name = "PINCER LANES"
telegraph = { ticks = 45, cue = "two dotted guide lines appear at the screen edges" }
attack = { kind = "beam", count = 2, width_px = 16, sweep = "edges_to_centre_then_back", sweep_q8 = 192, duration_ticks = 120 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 2 }
counterplay = "Two lanes close toward the centre and stop 48 px apart before reopening. Sit dead centre under the lens and fire up."

[[patterns]]
id = "p2_starburst"
name = "STARBURST"
telegraph = { ticks = 24, cue = "lens pulses three times" }
attack = { kind = "ring", count = 16, spread_deg = 360, speed_q8 = 288, rings = 3, rotate_deg = 11, gap_ticks = 14 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 32 }
counterplay = "Three rotating rings; the rotation is fixed clockwise, so step left one bullet-gap per ring."

[[animations]]
id = "shutter_open"
frames = 6
ticks_per_frame = 8
loop = false
model = "offline"
duration = 0
request_frames = 0
motion = "bitmap patch sequence: the two dome shutter halves slide apart to reveal the rings"

[[animations]]
id = "ring_turn"
frames = 16
ticks_per_frame = 4
loop = true
model = "offline"
duration = 0
request_frames = 0
motion = "each ring rotated offline in 16 steps from its master and baked over the dome interior"

[ludo]
concept_prompt = "Top-down orthographic view of a white hilltop observatory dome whose two shutter halves have rolled apart, revealing three nested rotating rings of brass frames overgrown with purple and light blue crystal facets around a glowing hexagonal telescope lens, a gravel path collar and violet crystal spilling over the dome rim. Lit from the upper left, short shadows lower right."
setpiece_prompt = "top-down view of an opened white observatory dome with three concentric brass-and-purple-crystal rings around a bright cyan hexagonal lens, on a brown gravel collar"
sprite_prompt = "top-down view of one brass ring frame with purple crystal facets and three gaps, isolated, for offline rotation"
+++

# ASTROLABE

# Personality and role

A patient marksman. The Lattice found an instrument built to watch the sky and made it shoot at it. The
ASTROLABE does not move; it aims. Every attack is a combination lock: rings turn, gaps line up, and the
player learns to read the alignment before the lane falls. It is the game's first pure bitmap fortress
and the first boss the BEAM reward from stage 1 was made for.

# Arena

The observatory grounds at the top of the TERRACES ridge: gravel paths in a star pattern, low hedges,
two smaller domes housing DOME MORTARs, everything threaded with violet crystal. The scroll stops with
the great dome in the upper two-thirds of the screen; the TALON flies freely over it (it is ground).

# Phases and patterns

**Phase 1 (180 → 75).** The three rings are separate targets (35 HP each). SIGHT LANE sweeps a vertical
beam column across the screen after a long, honest telegraph; RING VOLLEY spirals shots from the ring
notches; DOME MORTARS lob shells from the side domes; LENS FLARE fires an aimed cone and leaves the lens
open for 60 ticks. Destroying a ring removes its volley and swaps its patch set to the shattered frame.

**Phase 2 (75 → 0).** The lens is bare. The shutters half-close between attacks, hiding the lens for
40 ticks at a time. PINCER LANES close from both edges and leave a centre pocket; STARBURST throws three
rotating rings. BEAM pierces straight into the lens for double damage.

**Defeat.** The lens cracks into a star of white light, the rings fall flat, the shutters close for the
last time and the violet crystal on the ridge goes grey.

# Budget notes

- The whole boss is bitmap: 0 sprite channels.
- Beam lanes are drawn as a column of bucket sprites, one 24-px bucket per lane per 21-line band: one
  lane = 1 channel per line, PINCER LANES = 2. Bullets are suppressed on lane frames (`bullets_max`).
- Player band worst case: ship 2 + inner pods 2 + buckets 2 = 6; nothing else on the line.

# Weapon reward

The CORE SPARK moves the gauge cursor to **SEEKER**, whose darts find the TRESTLE TITAN's swinging claws.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
