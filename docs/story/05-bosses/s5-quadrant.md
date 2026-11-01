+++
id = "boss.s5.quadrant"
kind = "boss"
name = "QUADRANT"
title = "the four crystal domes of the Citadel keep"
stage = "stage.s5"
status = "approved"
revision = 2
layer = "hybrid"
hp = 310
weak_to = "beam"
score = 70000
time_bonus = { par_s = 90, max = 70000 }
reward = "seeker"
music_cue = "cue.boss"

# Visual identity: four bitmap domes; the survivor becomes a 4x4 sprite core
cell_rect = [10, 10]
sprite_class = "B"
cells = [4, 4]
angles = 16
anchor = "center"
identity = "four great faceted violet crystal domes set in khaki stone bastions, each crowned by a rotating twin cannon, pulsing with inner light"
silhouette = "four hexagonal domes in a square, twin barrels on each crown; alone, a flying shield-shaped gem: hexagonal crown above, torn crystal root tapering to a point below"
proportions = "each dome 10x10 cells with a 4x4 cannon crown; the lifted core a 4x4 sprite grid (96x84 display px): two 4-wide crown rows over a 2-wide root point"
materials = "purple and light blue crystal facets with white glints, khaki and brown stone collars, dark grey gun barrels"
acting = "the domes pulse in sequence like a slow heartbeat of light, cannons turn in unison, the last dome tears itself free with a ring of stone debris"
forbidden = ["faces", "eyes", "religious architecture", "real fortress names or flags", "organic forms"]
hurtbox = { x = -40, y = -36, w = 80, h = 72 }

[[colors]]
role = "crystal"
c64 = 4
note = "purple dome facets (bitmap and the sprite core)"
required = true

[[colors]]
role = "crystal_light"
c64 = 14
note = "light blue facet highlights; sprite core uses 14, not the TALON's cyan"
required = true

[[colors]]
role = "stone"
c64 = 9
note = "brown-khaki stone collars"
required = true

[[colors]]
role = "barrels"
c64 = 11
note = "dark grey cannon barrels (stage background ink)"
required = false

[[colors]]
role = "glow"
c64 = 3
note = "cyan pulse inside the domes (bitmap ink only)"
required = false

[[parts]]
id = "dome_nw"
layer = "bitmap"
cell_rect = [10, 10]
hp = 55
weak = true
note = "BEAM double damage; patches: 16 cannon angles x intact/cracked, plus a shattered crater"

[[parts]]
id = "dome_ne"
layer = "bitmap"
cell_rect = [10, 10]
hp = 55
weak = true

[[parts]]
id = "dome_sw"
layer = "bitmap"
cell_rect = [10, 10]
hp = 55
weak = true

[[parts]]
id = "dome_se"
layer = "bitmap"
cell_rect = [10, 10]
hp = 55
weak = true

[[parts]]
id = "core"
layer = "sprite"
sprite_class = "B"
cells = [4, 4]
hp = 90
weak = false
note = "the last surviving dome lifts off; its remaining dome HP is discarded and the core starts at 90"

[[phases]]
from_hp = 310
to_hp = 90
patterns = ["p1_quarter_turn", "p1_crossfire", "p1_pulse_chain", "p1_wall_call"]
note = "four domes; each loss makes the survivors faster by 1/8 and adds one shot to their volleys"

[[phases]]
from_hp = 90
to_hp = 0
patterns = ["p2_lift_ring", "p2_spiral", "p2_dive"]
note = "the fourth dome tears free as a 4x4 sprite grid and hunts along the top band; the TALON is held below line 88"

[[patterns]]
id = "p1_quarter_turn"
name = "QUARTER TURN"
telegraph = { ticks = 24, cue = "all crowns click a quarter turn with a stone grinding sound" }
attack = { kind = "fixed", count = 2, speed_q8 = 384, per_dome = true, volleys = 4, gap_ticks = 10 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 32 }
counterplay = "Every dome fires twin streams along its barrel direction; the four streams form a pinwheel that turns 90 degrees per telegraph. Sit in a quadrant the barrels point away from."

[[patterns]]
id = "p1_crossfire"
name = "CROSSFIRE"
telegraph = { ticks = 30, cue = "two diagonal domes glow, dotted lines connect them across the screen" }
attack = { kind = "aimed", count = 5, spread_deg = 24, speed_q8 = 416, domes = 2 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 10 }
counterplay = "Two aimed 5-way fans from opposite corners cross at your position; move along the dotted line's perpendicular."

[[patterns]]
id = "p1_pulse_chain"
name = "PULSE CHAIN"
telegraph = { ticks = 20, cue = "domes light one after another clockwise" }
attack = { kind = "ring", count = 8, spread_deg = 360, speed_q8 = 288, sequence = "clockwise", gap_ticks = 15 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 32 }
counterplay = "Four staggered 8-way rings; the gaps line up on the diagonals between domes."

[[patterns]]
id = "p1_wall_call"
name = "WALL CALL"
telegraph = { ticks = 30, cue = "WALL MORTAR hatches open on the keep's outer wall" }
attack = { kind = "lob", count = 4, enemy = "enemy.s5.wall_mortar", speed_q8 = 224, target = "player_x" }
sprite_budget = { boss_channels = 0, bucket_channels = 1, bullets_max = 4 }
counterplay = "Marked landing shadows for 30 ticks; keep moving between them."

[[patterns]]
id = "p2_lift_ring"
name = "LIFT-OFF"
telegraph = { ticks = 40, cue = "the last dome cracks around its collar; stone debris ring; the ceiling line flashes across the screen" }
attack = { kind = "ring", count = 16, spread_deg = 360, speed_q8 = 256 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 16 }
counterplay = "One slow ring as the core rises. Drop below the flashing ceiling line during the telegraph: from lift-off the TALON is held below line 88, and the ring's gaps are widest down there."

[[patterns]]
id = "p2_spiral"
name = "GEM SPIRAL"
telegraph = { ticks = 20, cue = "a ripple of light runs round the core's facets and it brightens" }
attack = { kind = "fixed", count = 3, speed_q8 = 352, spiral_deg_per_shot = 17, duration_ticks = 120, interval_ticks = 6 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 60 }
counterplay = "A triple spiral that rotates one way: drift sideways with its turn near the bottom of the screen, where the arms are furthest apart."

[[patterns]]
id = "p2_dive"
name = "HAMMER DIVE"
telegraph = { ticks = 30, cue = "core slides along the top band to the TALON's x; a column of shadow locks under its point" }
attack = { kind = "ram", count = 1, target = "player_x_at_lock", path = "drops until its crown rows touch line 88, driving the 2-wide root point down to line 130", speed_q8 = 640 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 0 }
counterplay = "Move out of the locked shadow column (or stay below line 130). The core hangs at the ceiling for 40 ticks after the stab: the main punish window."

[[occupancy]]
pose = "core"
rows = ["1111", "1111", "0110", "0110"]
note = "crown rows 0-1 are 4 wide and stay above line 88 (core 4 + buckets 2 = 6); root rows 2-3 are 2 wide and may enter the player band (root 2 + ship 2 + inner pods 2 + buckets 2 = 8)"

[[animations]]
id = "cannon_turn"
frames = 16
ticks_per_frame = 3
loop = true
model = "offline"
duration = 0
request_frames = 0
motion = "twin cannon crown rotated offline in 16 angles and baked onto each dome"

[[animations]]
id = "core_ripple"
frames = 4
ticks_per_frame = 5
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "holds still and level while a ripple of light runs round its facets from the crown down to the point"

[[animations]]
id = "lift"
frames = 4
ticks_per_frame = 6
loop = false
model = "hydra-plus"
duration = 3
request_frames = 25
motion = "the crystal dome tears free of its stone collar and rises, shadow sliding away below"

[ludo]
concept_prompt = "Top-down orthographic view of four great faceted purple and light blue crystal domes arranged in a square inside a khaki stone star-fort keep, each dome crowned by a dark grey rotating twin cannon, cyan light pulsing inside, long late-day shadows. Lit from the upper left, shadows to the lower right."
setpiece_prompt = "top-down view of one hexagonal purple crystal dome in a brown-khaki stone collar with a dark grey twin cannon crown, isolated on stone paving"
sprite_prompt = "top-down view of a flying shield-shaped purple crystal gem with light blue highlights, a wide hexagonal crown with two stubby dark grey barrels and a broken stone ring, its torn crystal root tapering to a point below"
+++

# QUADRANT

# Personality and role

The Citadel's keep, and the relay that changes the war. Four domes that breathe light in turn, steady
and ceremonial, like a guard of honour. When three fall, the fourth abandons its post and hunts: the
moment the player first sees a Lattice fortress *leave the ground*, foreshadowing the twist that follows.

# Arena

The inner keep of BASTION: four stone bastion collars in a square (208x176 px), courtyards of khaki
paving between them, the outer wall at the edges where WALL MORTARs sit. The scroll stops with all four
domes on screen. The TALON flies freely over the whole fortress until the survivor lifts off; from then
on it is held below line 88.

# Phases and patterns

**Phase 1 (310 → 90).** Four domes, 55 HP each, double damage from BEAM, which reaches the far domes from
safe ground. QUARTER TURN forms a pinwheel of streams, CROSSFIRE fires fans from opposite corners,
PULSE CHAIN staggers four rings clockwise, WALL CALL lobs mortars. Each dome lost speeds the others.

**Phase 2 (90 → 0).** The survivor tears free as a 96x84 gem: a wide hexagonal crown over a tapering
crystal root. It hunts along the top band. LIFT-OFF rings once while the ceiling closes, GEM SPIRAL turns
a triple spiral, and HAMMER DIVE slides over the TALON's x and drops, stabbing the root point down to line
130, then hangs at the ceiling for 40 ticks.

**Defeat.** The gem shatters over its crater. The glyph screen follows: the Citadel was a relay; the
light went **up**.

# Budget notes

- Phase 1 is all bitmap: 0 boss channels; bullets 2 bucket channels max.
- Phase 2 core: a 4x4 grid. Its two crown rows are 4 wide and stay above line 88, so the ceiling clamp
  applies from LIFT-OFF: crown 4 + buckets 2 = 6. The two root rows taper to 2 columns, so even at the
  bottom of HAMMER DIVE (root point at line 130): root 2 + ship 2 + inner pods 2 + buckets 2 = 8.

# Weapon reward

The CORE SPARK moves the gauge cursor to **SEEKER**, which picks off the ORCHID's petal pods in stage 6.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-31): sprite expansion removed at the user's request; bigger grids instead
