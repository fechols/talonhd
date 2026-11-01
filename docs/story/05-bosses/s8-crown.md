+++
id = "boss.s8.crown"
kind = "boss"
name = "CROWN"
title = "the Lattice's orbital ring station, final guardian"
stage = "stage.s8"
status = "approved"
revision = 2
layer = "hybrid"
hp = 580
weak_to = "seeker"
score = 200000
time_bonus = { par_s = 150, max = 150000 }
reward = "burst"
reward_note = "the CORE SPARK falls with the ending and carries into the next loop's stage 1"
music_cue = "cue.final_1"
music_cue_phase2 = "cue.final_2"

# Visual identity: Phase 1 the Ring (bitmap), Phase 2 the Heart (sprite + two orbiting shard columns)
cell_rect = [38, 14]
sprite_class = "B"
cells = [4, 8]
expanded = false
angles = 1
anchor = "center"
identity = "a vast orbital ring station of faceted crystal girders, eight hexagonal facet doors on the rim, a crystal Heart amid orbiting shards at its centre"
silhouette = "a screen-wide arc of crystal with eight hexagonal doors; then a tall double-pointed crystal spindle flanked by two columns of tumbling shards"
proportions = "Ring 304x112 display px; Heart 4 cells wide for 4 rows tapering to a 2-cell point for 4 rows; shard corona two 1x4 sprite columns, each 24x84, beside the wide rows"
materials = "purple and light blue crystal girders with white glints, white-hot energy seams, black space behind; the Heart is clear crystal with a white-yellow core light"
acting = "the Ring turns its facets toward the TALON one at a time like a slow lock; the Heart moves with sudden glides and stops dead, its shards trailing a beat behind"
forbidden = ["faces", "eyes", "organic heart shapes", "veins or flesh", "religious halos or symbols", "crowns with jewels in a royal style", "real space station designs"]
hurtbox = { x = -44, y = -84, w = 88, h = 120 }

[[colors]]
role = "crystal"
c64 = 4
note = "purple girders and facets (Ring bitmap and Heart sprite)"
required = true

[[colors]]
role = "crystal_light"
c64 = 14
note = "light blue planes; the Heart and shard sprites use 14, not the TALON's private cyan"
required = true

[[colors]]
role = "glints"
c64 = 3
note = "cyan seams on the Ring (bitmap ink only)"
required = false

[[colors]]
role = "energy"
c64 = 7
note = "yellow-white core light in the Heart and opening facets"
required = true

[[colors]]
role = "space"
c64 = 0
note = "black space (stage background ink)"
required = true

[[parts]]
id = "facet_1"
layer = "bitmap"
cell_rect = [6, 6]
hp = 35
weak = true
note = "eight hexagonal facet doors on the Ring rim; patches closed/opening/open/cracked/dark; SEEKER double damage; only open facets take damage"

[[parts]]
id = "facet_2"
layer = "bitmap"
cell_rect = [6, 6]
hp = 35
weak = true

[[parts]]
id = "facet_3"
layer = "bitmap"
cell_rect = [6, 6]
hp = 35
weak = true

[[parts]]
id = "facet_4"
layer = "bitmap"
cell_rect = [6, 6]
hp = 35
weak = true

[[parts]]
id = "facet_5"
layer = "bitmap"
cell_rect = [6, 6]
hp = 35
weak = true

[[parts]]
id = "facet_6"
layer = "bitmap"
cell_rect = [6, 6]
hp = 35
weak = true

[[parts]]
id = "facet_7"
layer = "bitmap"
cell_rect = [6, 6]
hp = 35
weak = true

[[parts]]
id = "facet_8"
layer = "bitmap"
cell_rect = [6, 6]
hp = 35
weak = true

[[parts]]
id = "heart"
layer = "sprite"
sprite_class = "B"
cells = [4, 8]
hp = 300
weak = true
note = "SEEKER double damage"

[[parts]]
id = "corona_left"
layer = "sprite"
sprite_class = "B"
cells = [1, 4]
hp = 0
weak = false
note = "a 24x84 column of four or five small crystal shards beside the Heart's wide rows, cycling an 8-frame orbit (shards rise up one side, wink out at the top and re-form on the other column); invulnerable; absorbs player shots and SEEKER darts that cross it"

[[parts]]
id = "corona_right"
layer = "sprite"
sprite_class = "B"
cells = [1, 4]
hp = 0
weak = false

[[phases]]
from_hp = 580
to_hp = 300
name = "THE RING"
music_cue = "cue.final_1"
patterns = ["r_facet_volley", "r_twin_facets", "r_lattice_lines", "r_prism_call"]
note = "facets open one at a time in a fixed clockwise order; after four are destroyed, two open at once"

[[phases]]
from_hp = 300
to_hp = 0
name = "THE HEART"
music_cue = "cue.final_2"
patterns = ["h_hunt", "h_shard_spin", "h_plunge", "h_starfall"]
note = "the Ring shatters into drifting bitmap debris; the Heart drops free inside its shard corona and hunts"

[[patterns]]
id = "r_facet_volley"
name = "FACET VOLLEY"
telegraph = { ticks = 30, cue = "a facet door glows yellow at its seams and slides open" }
attack = { kind = "spread", count = 6, spread_deg = 60, speed_q8 = 384, aimed = true, volleys = 4, gap_ticks = 12, facet_specific = "each facet has its own volley shape: fan, ring arc, double line, or spiral" }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 24 }
counterplay = "Only the open facet fires and only the open facet can be hurt. Its door glows before opening, so be under it when it opens and SEEKER will lock on."

[[patterns]]
id = "r_twin_facets"
name = "TWIN FACETS"
telegraph = { ticks = 36, cue = "two opposite facets glow together" }
attack = { kind = "aimed", count = 4, speed_q8 = 416, facets = 2, volleys = 3, gap_ticks = 16 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 24 }
counterplay = "From the fifth facet on. The two streams converge from left and right; stay in the middle and slip down between them."

[[patterns]]
id = "r_lattice_lines"
name = "LATTICE LINES"
telegraph = { ticks = 45, cue = "dotted guide lines drop from three closed facets" }
attack = { kind = "beam", count = 2, width_px = 16, fixed_x = true, duration_ticks = 120, pattern = "lanes rotate between three positions" }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 2 }
counterplay = "Two of three marked lanes fire; the dark one is safe. The guide lines show all three for 45 ticks before two light."

[[patterns]]
id = "r_prism_call"
name = "PRISM CALL"
telegraph = { ticks = 30, cue = "PRISM NODE patches on the Ring girders blink" }
attack = { kind = "ring", count = 8, speed_q8 = 256, enemy = "enemy.s8.prism_node", nodes = 2 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 16 }
counterplay = "Two prism nodes fire slow rings; destroy them for sparks before the next facet opens."

[[patterns]]
id = "h_hunt"
name = "HUNT"
telegraph = { ticks = 20, cue = "the Heart stops dead, its shards catch up and flash" }
attack = { kind = "aimed", count = 5, spread_deg = 30, speed_q8 = 448, volleys = 2, gap_ticks = 10, movement = "glides to the TALON's x along the top band, then fires" }
sprite_budget = { boss_channels = 6, bucket_channels = 2, bullets_max = 10 }
counterplay = "The Heart glides above you, stops and fires. Move as it stops: a 5-way aimed fan misses a ship that is already moving sideways."

[[patterns]]
id = "h_shard_spin"
name = "SHARD SPIN"
telegraph = { ticks = 30, cue = "the shard columns whirl faster and glint white" }
attack = { kind = "ring", count = 12, spread_deg = 360, speed_q8 = 288, rings = 4, rotate_deg = 9, gap_ticks = 12 }
sprite_budget = { boss_channels = 6, bucket_channels = 2, bullets_max = 40 }
counterplay = "Four rotating rings; while the shards whirl they swallow every shot and SEEKER dart that curves in from the sides, so focus on dodging and fire again when they slow."

[[patterns]]
id = "h_plunge"
name = "PLUNGE"
telegraph = { ticks = 36, cue = "the shard columns lift away upward, the Heart's point glows and its shadow locks under the TALON" }
attack = { kind = "ram", count = 1, path = "the 2-wide point lances down to line 160 at the locked x, holds 30 ticks", speed_q8 = 896 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 0 }
counterplay = "Leave the lock point. While the Heart is extended its body sits right at the ceiling with its shards away: the biggest SEEKER window of the fight."

[[patterns]]
id = "h_starfall"
name = "STARFALL"
telegraph = { ticks = 40, cue = "below 100 HP: the Heart flares white, Ring debris starts to glow" }
attack = { kind = "lob", count = 12, speed_q8 = 224, source = "ring debris patches", target = "marked columns", marks_ticks = 30 }
sprite_budget = { boss_channels = 6, bucket_channels = 2, bullets_max = 24 }
counterplay = "Debris rains down marked columns while the Heart keeps hunting; the unmarked columns are safe. This is the last pattern before the end."

[[occupancy]]
pose = "heart_hunt"
rows = ["111111", "111111", "111111", "111111", "0110", "0110", "0110", "0110"]
note = "rows 0-3: shard column 1 + heart 4 + shard column 1 + buckets 2 = 8, all above line 88; rows 4-7 the 2-wide point with no shards beside it"

[[occupancy]]
pose = "plunge"
rows = ["1111", "1111", "1111", "1111", "0110", "0110", "0110", "0110"]
note = "shard columns parked above the screen edge (0 channels); the extended point reaches line 160 at 2 channels"

[[animations]]
id = "facet_open"
frames = 4
ticks_per_frame = 6
loop = false
model = "offline"
duration = 0
request_frames = 0
motion = "bitmap patch sequence: a hexagonal facet door glows at its seams and slides open on a yellow-white interior"

[[animations]]
id = "ring_shatter"
frames = 6
ticks_per_frame = 6
loop = false
model = "offline"
duration = 0
request_frames = 0
motion = "bitmap patch sequence: the Ring cracks from facet to facet and breaks into drifting debris"

[[animations]]
id = "heart_pulse"
frames = 4
ticks_per_frame = 5
loop = true
model = "hydra-plus"
duration = 3
request_frames = 25
motion = "the crystal spindle pulses with a white-yellow light that runs from the centre to both points"

[[animations]]
id = "shards_orbit"
frames = 8
ticks_per_frame = 4
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "the small pieces climb slowly in a column, each one tumbling and flashing as it turns, the top piece fading out as a new one forms at the bottom"

[[animations]]
id = "collapse"
frames = 6
ticks_per_frame = 6
loop = false
model = "hydra-plus"
duration = 4
request_frames = 36
motion = "the crystal spindle cracks, goes dark from the points inward and falls apart into many small glittering stars"

[ludo]
concept_prompt = "Top-down orthographic view of a vast ring-shaped space station made of faceted purple and light blue crystal girders with white glints and white-hot energy seams, eight hexagonal facet doors around its rim, black space and white stars behind, and at the centre a tall double-pointed crystal spindle glowing white-yellow, flanked by two columns of small tumbling crystal shards. All crisp geometric planes. Lit from the upper left."
setpiece_prompt = "top-down view of a wide arc of a crystal ring station, purple and light blue girders, eight hexagonal doors along its edge, cyan seams, on black space"
sprite_prompt = "top-down view of a tall double-pointed faceted crystal spindle, clear crystal with purple and light blue planes, a white-yellow light at its centre"
shards_prompt = "top-down view of a narrow vertical column of five small sharp light blue crystal shards, evenly spaced, isolated"
+++

# CROWN

# Personality and role

The Lattice's only face, and it has none. The CROWN is the thing every relay fed: a ring of crystal in
orbit that gathered the world's stolen light. It does not rage; it arranges. Phase 1 is a lock being
picked one facet at a time; phase 2 is what was behind the lock: a single bright Heart that has never
had to fight for itself, hunting the one aircraft its jamming could not stop.

# Arena

The top of the SKYHOOK. The cable ends in a docking collar at the bottom of the Ring. The scroll stops
with the Ring across the top 112 lines of the screen, Earth's limb curving across the bottom of the
background, black space and stars between. After the Ring falls, its debris stays as drifting bitmap
patches that STARFALL uses.

# Phases and patterns

**Phase 1, THE RING (580 → 300), `cue.final_1`.** Eight facets open clockwise, one at a time; only the
open facet fires and takes damage (35 HP each, double from SEEKER). FACET VOLLEY gives each facet its own
volley shape so the player learns the dial; after four are gone TWIN FACETS opens opposite pairs.
LATTICE LINES drops two of three marked beam lanes; PRISM CALL wakes two PRISM NODEs on the girders.

**Phase 2, THE HEART (300 → 0), `cue.final_2`.** The Ring shatters; the Heart drops free inside its
shard corona: two columns of small crystal shards that orbit beside its wide upper body, climbing one side
and re-forming on the other. HUNT glides it over the TALON to fire aimed fans; SHARD SPIN throws four
rotating rings while the whirling shards swallow shots and SEEKER darts from the sides; PLUNGE lances the 2-wide point down at the TALON's locked position and leaves the
body exposed at the ceiling; below 100 HP STARFALL rains Ring debris down marked columns.

**Ending.** The Heart goes dark from its points inward and breaks into hundreds of small falling stars
(bucket twinkles) that drift down past the cable toward the dawn line on Earth's limb. The stage-clear
sting is replaced by `cue.ending`.

# Budget notes

- The Ring is bitmap: 0 channels. Bullets and lanes use at most 2 bucket channels.
- Heart, lines 0-83: heart 4 + two 1x4 shard columns 2 + buckets 2 = 8. The TALON is clamped at
  line 88. The shards are drawn into the two column sprites frame by frame, so a whole corona of them
  costs one channel per side.
- Heart rows 4-7 are the 2-wide point: point 2 + ship 2 + inner pods 2 + buckets 2 = 8.
- PLUNGE: the shard columns park above the screen edge, and only the 2-wide point enters the player band.
- Ending stars are bucket twinkles: 2 bucket channels, no enemy bullets exist by then.

# Weapon reward

The CORE SPARK rises out of the falling stars and moves the gauge cursor to **BURST**. The cursor carries
through the ending and the loop card into loop 2, where BURST is the answer to the GANTRY QUEEN again.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-31): sprite expansion removed at the user's request; bigger grids instead
