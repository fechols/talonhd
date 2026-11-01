+++
id = "boss.s4.hydra_pump"
kind = "boss"
name = "HYDRA PUMP"
title = "the five-headed pump core of the Sluiceworks relay"
stage = "stage.s4"
status = "approved"
revision = 1
layer = "hybrid"
hp = 240
weak_to = "burst"
score = 60000
time_bonus = { par_s = 80, max = 60000 }
reward = "beam"
music_cue = "cue.boss"

# Visual identity: the core is a bitmap set-piece; the heads are sprites
cell_rect = [12, 10]
sprite_class = "B"
cells = [2, 3]
expanded = false
angles = 8
anchor = "base"
identity = "a round industrial pump housing in a turbine pit, five jointed steel pipe heads with crystal nozzles rising from its rim like periscopes"
silhouette = "a ringed circle with five long jointed tubes fanning upward, each ending in a hexagonal nozzle"
proportions = "core 12x10 cells; each head 2 cells wide, 3 rows long, three pipe joints and a nozzle"
materials = "steel grey pipe segments with yellow-and-black hazard collars, red warning lamps, nozzles of light blue crystal, cyan crystal veins visible through pipe gauges"
acting = "heads rise one or two at a time from their sockets, sway like searching periscopes, lock rigid before firing, whip down to strike"
forbidden = ["faces", "eyes", "mouths", "teeth", "scales", "organic necks", "real company logos"]
hurtbox = { x = -24, y = -63, w = 48, h = 63 }

[[colors]]
role = "primary"
c64 = 12
note = "mid grey pipe steel (core and heads)"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal nozzles on the sprite heads (not the TALON's private cyan)"
required = true

[[colors]]
role = "accent"
c64 = 7
note = "yellow hazard collars"
required = true

[[colors]]
role = "warning"
c64 = 2
note = "dark red warning lamps that light during a telegraph"
required = false

[[colors]]
role = "veins"
c64 = 3
note = "cyan crystal veins inside the core's gauges (bitmap ink only)"
required = false

[[parts]]
id = "head_1"
layer = "sprite"
sprite_class = "B"
cells = [2, 3]
hp = 24
weak = false
note = "when not raised a head rests in its socket as a bitmap patch; destroyed heads flood lane 1 (x 8-64)"

[[parts]]
id = "head_2"
layer = "sprite"
sprite_class = "B"
cells = [2, 3]
hp = 24
weak = false

[[parts]]
id = "head_3"
layer = "sprite"
sprite_class = "B"
cells = [2, 3]
hp = 24
weak = false

[[parts]]
id = "head_4"
layer = "sprite"
sprite_class = "B"
cells = [2, 3]
hp = 24
weak = false

[[parts]]
id = "head_5"
layer = "sprite"
sprite_class = "B"
cells = [2, 3]
hp = 24
weak = false

[[parts]]
id = "core"
layer = "bitmap"
cell_rect = [12, 10]
hp = 120
weak = true
note = "BURST double damage; armoured until its iris shutter opens in phase 2; patches: closed, opening, open, cracked"

[[phases]]
from_hp = 240
to_hp = 120
patterns = ["p1_periscope", "p1_strike", "p1_pressure_ring"]
note = "heads only; each head destroyed floods its lane for 180 ticks"

[[phases]]
from_hp = 120
to_hp = 0
patterns = ["p2_geyser_chase", "p2_core_vent", "p2_last_heads"]
note = "the core shutter opens; any surviving heads keep fighting"

[[patterns]]
id = "p1_periscope"
name = "PERISCOPE"
telegraph = { ticks = 24, cue = "two heads rise from their sockets and their red lamps light" }
attack = { kind = "aimed", count = 3, speed_q8 = 384, heads = 2, volleys = 2, gap_ticks = 16 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 12 }
counterplay = "Only two heads are up at once, both above the ceiling. Each fires a 3-shot aimed burst: stream sideways and shoot whichever head is lit."

[[patterns]]
id = "p1_strike"
name = "PIPE STRIKE"
telegraph = { ticks = 30, cue = "one head rears back to the top edge, its lane flashes a yellow dotted outline on the floor" }
attack = { kind = "ram", count = 1, path = "whips straight down its lane to line 150, holds 30 ticks, retracts", speed_q8 = 768 }
sprite_budget = { boss_channels = 2, bucket_channels = 2, bullets_max = 4 }
counterplay = "Leave the marked lane. The extended head lies across the player band for 30 ticks and is the easiest target in the fight."

[[patterns]]
id = "p1_pressure_ring"
name = "BLOWDOWN"
telegraph = { ticks = 20, cue = "core gauges spin and hiss, steam puffs at the rim" }
attack = { kind = "ring", count = 10, spread_deg = 360, speed_q8 = 288 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 10 }
counterplay = "One slow ring from the core; slip through a gap near the bottom."

[[patterns]]
id = "p2_geyser_chase"
name = "GEYSER CHASE"
telegraph = { ticks = 30, cue = "drain grates on the pit floor bubble under the player's lane" }
attack = { kind = "hazard", count = 3, target = "player_lane", interval_ticks = 40, duration_ticks = 60 }
sprite_budget = { boss_channels = 0, bucket_channels = 0, bullets_max = 0 }
counterplay = "Three geyser columns (bitmap patches) erupt one after another in the lane you are in: change lane each time the grate bubbles."

[[patterns]]
id = "p2_core_vent"
name = "CORE VENT"
telegraph = { ticks = 24, cue = "the iris shutter widens and glows cyan" }
attack = { kind = "spread", count = 7, spread_deg = 90, speed_q8 = 416, volleys = 4, alternate_offset_deg = 6 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 28 }
counterplay = "Alternating 7-way fans; get close under the core and fire BURST between fans: the fan is narrow at its root."

[[patterns]]
id = "p2_last_heads"
name = "LAST HEADS"
telegraph = { ticks = 24, cue = "surviving heads rise together" }
attack = { kind = "aimed", count = 1, speed_q8 = 512, heads = 2, volleys = 5, gap_ticks = 10 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 10 }
counterplay = "Skipped if no heads remain. Faster single shots: keep moving."

[[occupancy]]
pose = "two_raised"
rows = ["11 11", "11 11", "11 11"]
note = "two 2-wide heads share lines 0-62 above the ceiling = 4 channels; the other three heads are bitmap socket patches"

[[occupancy]]
pose = "strike"
rows = ["11", "11", "11"]
note = "one striking head below line 88, 2 channels in the player band; the second raised head retracts first"

[[animations]]
id = "head_rise"
frames = 4
ticks_per_frame = 5
loop = false
model = "hydra-plus"
duration = 3
request_frames = 25
motion = "a jointed steel pipe head unfolds upward out of a round socket, joint by joint, and locks straight"

[[animations]]
id = "head_sway"
frames = 8
ticks_per_frame = 6
loop = true
model = "offline"
duration = 0
request_frames = 0
motion = "head master rotated offline through 8 aim angles"

[[animations]]
id = "core_iris"
frames = 4
ticks_per_frame = 8
loop = false
model = "offline"
duration = 0
request_frames = 0
motion = "bitmap patch sequence: the core's iris shutter opens in four steps"

[ludo]
concept_prompt = "Top-down orthographic view of a giant round industrial pump in a flooded turbine pit, steel grey housing with gauges showing cyan crystal veins inside, an iris shutter in the middle, and five jointed steel pipe arms with yellow-and-black hazard collars and hexagonal light blue crystal nozzles rising from sockets around its rim. Lit from the upper left, short shadow to the lower right."
setpiece_prompt = "top-down view of a round grey pump housing with an iris shutter, gauges, five empty pipe sockets around the rim, in a turbine pit of blue water and yellow-striped steel walkways"
sprite_prompt = "top-down view of one jointed steel pipe arm with three hazard-striped joints and a hexagonal light blue crystal nozzle"
+++

# HYDRA PUMP

# Personality and role

The heart of the SLUICEWORKS relay: the machine that pressurises the canals to push Lattice power down
the pipes. Busy, twitchy and loud, all hiss and clank. Five heads search the sky like periscopes; the
core never moves. It is the first hybrid boss where the player chooses targets: every head killed makes
the arena smaller for a while.

# Arena

The pump hall at the end of the stage: a round turbine pit of canal-blue water, yellow-striped steel
walkways, giant vent fans in the corners. The scroll stops with the core at the top middle. Five lanes
run down from the five sockets (lane 1 x 8-64 to lane 5 x 248-304), marked on the walkway floor by
faint hazard lines.

# Phases and patterns

**Phase 1 (240 → 120).** Heads only, 24 HP each. PERISCOPE raises two heads that fire aimed bursts;
PIPE STRIKE whips one head down its marked lane and leaves it lying across the player band; BLOWDOWN
vents a slow ring from the core. A destroyed head bursts and **floods its lane**: a white-water geyser
patch fills the lane for 180 ticks and kills on contact, so killing heads in neighbouring lanes squeezes
the player. Phase 1 ends at 120 HP even if heads remain.

**Phase 2 (120 → 0).** The core's iris opens. GEYSER CHASE erupts geysers in the player's lane, CORE VENT
fires alternating fans, LAST HEADS uses whatever heads survived. BURST does double damage to the core:
the fan is narrow at its root, so the brave play is close.

**Defeat.** The iris jams open, the pit drains in four patch steps, and the cyan veins in every pipe in
the district go dark.

# Budget notes

- Never more than two heads are raised as sprites: 2 x 2 = 4 channels on lines 0-62, + buckets 2 = 6.
  Lowered heads are bitmap socket patches.
- PIPE STRIKE: only one head below line 88 (2 channels): head 2 + ship 2 + inner pods 2 + buckets 2 = 8.
  The second raised head retracts during a strike.
- Core, geysers and wreckage are bitmap: 0 channels.

# Weapon reward

The CORE SPARK moves the gauge cursor to **BEAM**, which outranges the QUADRANT's dome cannons in stage 5.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
