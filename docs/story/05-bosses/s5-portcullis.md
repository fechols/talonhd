+++
id = "midboss.s5.portcullis"
kind = "midboss"
name = "PORTCULLIS"
title = "the Citadel gatehouse and its four cycling windows"
stage = "stage.s5"
status = "approved"
revision = 1
layer = "bitmap"
hp = 100
weak_to = "burst"
score = 18000
time_bonus = { par_s = 45, max = 18000 }
drops = { spark = 3 }
music_cue = "cue.midboss"
cell_rect = [24, 12]
angles = 8
anchor = "center"
identity = "a squat star-fort gatehouse seen from above, two round khaki towers flanking an iron grate, four crystal-shuttered gun windows along its roof"
silhouette = "wide rectangle with two round corner towers, a dark grate slot in the middle, four square hatches in a row"
proportions = "192 px wide, 96 px deep; towers a third of the depth across; hatches 24 px square"
materials = "khaki and brown dressed stone, dark grey iron grate, purple crystal shutters with light blue facets, white glints"
acting = "stern and patient; shutters open in turn like a slow drum roll, the grate shudders before it rises"
forbidden = ["people", "flags or heraldry", "religious symbols", "eyes or faces", "organic growth"]
hurtbox = { x = -96, y = -48, w = 192, h = 96 }

[[colors]]
role = "primary"
c64 = 9
note = "brown / khaki stone"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey grate and long late-day shadows"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal shutters"
required = true

[[colors]]
role = "glow"
c64 = 14
note = "light blue inside an open window"
required = false

[[parts]]
id = "window_1"
layer = "bitmap"
cell_rect = [3, 3]
hp = 20

[[parts]]
id = "window_2"
layer = "bitmap"
cell_rect = [3, 3]
hp = 20

[[parts]]
id = "window_3"
layer = "bitmap"
cell_rect = [3, 3]
hp = 20

[[parts]]
id = "window_4"
layer = "bitmap"
cell_rect = [3, 3]
hp = 20

[[parts]]
id = "grate"
layer = "bitmap"
cell_rect = [6, 4]
hp = 20

[[phases]]
from_hp = 100
to_hp = 40
patterns = ["p1_roll_call", "p1_tower_mortar"]

[[phases]]
from_hp = 40
to_hp = 0
patterns = ["p2_grate_rise", "p1_roll_call"]

[[patterns]]
id = "p1_roll_call"
name = "ROLL CALL"
telegraph = { ticks = 20, cue = "a crystal shutter slides back, light blue glow" }
attack = { kind = "spread", count = 3, spread_deg = 30, speed_q8 = 320, open_ticks = 60 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 12 }
counterplay = "Windows open one at a time, left to right, each firing one 3-way fan then staying open for a second. A window only takes damage while open, so follow the cycle along the wall."

[[patterns]]
id = "p1_tower_mortar"
name = "TOWER MORTAR"
telegraph = { ticks = 40, cue = "tower roof hatch lifts, shell shadow appears at the target" }
attack = { kind = "lob", count = 2, spread_deg = 0, speed_q8 = 160, burst_on_land = 6 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 12 }
counterplay = "Each corner tower lobs a shell at the player's current position; a dark ring marks the landing point 40 ticks ahead. Leave the ring; the 6-way burst is short-lived."

[[patterns]]
id = "p2_grate_rise"
name = "GRATE RISE"
telegraph = { ticks = 45, cue = "the iron grate shudders and lifts a cell at a time" }
attack = { kind = "beam", count = 1, spread_deg = 0, width_px = 32, duration_ticks = 60, direction = "down" }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 8 }
counterplay = "The raised grate reveals a crystal core that fires a 32-px beam straight down for one second. Stay out of the gate's column; the core takes damage only while the grate is up."

[[animations]]
id = "shutter_open"
frames = 3
ticks_per_frame = 6
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "a purple crystal shutter slides aside revealing a glowing light blue gun port"

[[animations]]
id = "grate_rise"
frames = 4
ticks_per_frame = 10
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "the dark iron grate lifts out of its slot, revealing a faceted crystal core below"

[[animations]]
id = "wreck"
frames = 3
ticks_per_frame = 10
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "windows blow out in turn, the roof collapses into rubble and the crystal goes dark"

[ludo]
concept_prompt = "A squat star-fort gatehouse seen from directly above: two round khaki stone towers flanking a dark iron grate in the middle, four square purple crystal shutters in a row along the flat roof, light blue facets with white glints. Late-day light from the upper left with long shadows to the lower right across a moat bridge."
setpiece_prompt = "a stone gatehouse from directly above, two round corner towers, a dark iron grate slot, four closed purple crystal hatches in a row, khaki and brown stone, long shadows"
+++

# PORTCULLIS

## Personality and role

The Citadel's front door. PORTCULLIS is about **timing**: every target is armoured until it opens.
It is the siege made into a duel, and it prepares the player for QUADRANT, where the domes also open.

## Arena

Section 2 of BASTION, the walls, around screen 12. The drawbridge leads to the gatehouse; the scroll
stops with the gatehouse in the upper half of the view and the drawbridge and moat below. GALLERY GUNs
on the flanking walls stay silent while it lives.

## Phases and patterns

**Phase 1 (100 → 40).** ROLL CALL cycles the four windows; TOWER MORTAR every 4 s.

**Phase 2 (40 → 0).** Starts when all four windows are destroyed or HP reaches 40. GRATE RISE
alternates with a faster ROLL CALL from any windows still alive. Destroying the core breaks the gate and
the scroll resumes through the archway.

**Weakness.** BURST: all five pellets fit an open 24-px window from close range, double damage.

## Budget notes

Bitmap only. Every window has closed, opening, open and wrecked patches; the grate has four lift
patches. The GRATE RISE beam is drawn as a column of bucket sprites (one channel, 32 px wide, stacked
down the column), so it costs 1 bucket channel on each line it crosses; the remaining bucket channel
carries ordinary bullets.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
