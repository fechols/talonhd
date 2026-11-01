+++
id = "midboss.s4.lockmaster"
kind = "midboss"
name = "LOCKMASTER"
title = "the twin canal lock gates that close on the player"
stage = "stage.s4"
status = "approved"
revision = 1
layer = "bitmap"
hp = 90
weak_to = "flank"
score = 16000
time_bonus = { par_s = 40, max = 16000 }
drops = { spark = 3 }
music_cue = "cue.midboss"
cell_rect = [14, 10]
angles = 16
anchor = "center"
identity = "two massive steel canal lock gates seen from above, hazard-striped leading edges, crystal-veined machinery decks, three gun turrets on each"
silhouette = "two broad rectangles sliding toward each other, sawtooth inner edges, round turret rings in a row"
proportions = "each half 112 px wide and 80 px deep; turrets 32 px across"
materials = "riveted dark steel gate plates, yellow and black hazard leading edges, red warning lamps, cyan crystal veins through hydraulic pipes"
acting = "grinds inward one step at a time, pauses with a hiss, turrets track; lamps blink red before every step"
forbidden = ["people", "real company logos", "eyes or faces", "organic growth"]
hurtbox = { x = -56, y = -40, w = 112, h = 80 }

[[colors]]
role = "primary"
c64 = 11
note = "dark grey steel plate"
required = true

[[colors]]
role = "secondary"
c64 = 12
note = "mid grey machinery decks"
required = true

[[colors]]
role = "accent"
c64 = 7
note = "yellow hazard edges"
required = true

[[colors]]
role = "lamp"
c64 = 2
note = "red warning lamps"
required = false

[[colors]]
role = "crystal"
c64 = 3
note = "cyan crystal veins in the pipes"
required = false

[[parts]]
id = "gate_left"
layer = "bitmap"
cell_rect = [14, 10]
hp = 45

[[parts]]
id = "gate_right"
layer = "bitmap"
cell_rect = [14, 10]
hp = 45

[[parts]]
id = "turrets"
layer = "bitmap"
cell_rect = [4, 4]
count = 6
hp = 8
note = "three per gate; their HP counts toward their gate"

[[phases]]
from_hp = 90
to_hp = 45
patterns = ["p1_step_close", "p1_cross_fire"]

[[phases]]
from_hp = 45
to_hp = 0
patterns = ["p1_step_close", "p2_lone_gate"]

[[patterns]]
id = "p1_step_close"
name = "STEP CLOSE"
telegraph = { ticks = 24, cue = "red lamps blink along the leading edge, hiss" }
attack = { kind = "hazard", count = 1, step_px = 8, min_gap_px = 64, reopen_ticks = 240 }
sprite_budget = { boss_channels = 0, bucket_channels = 0, bullets_max = 0 }
counterplay = "Both gates step 8 px inward per blink until a 64-px channel remains, then reopen to full width after 4 s. The gates are ground: the TALON flies over them, but the turrets get closer. Keep to the centre while they close and let FLANK guns rake both edges."

[[patterns]]
id = "p1_cross_fire"
name = "CROSS FIRE"
telegraph = { ticks = 16, cue = "turret barrel flashes white" }
attack = { kind = "aimed", count = 1, spread_deg = 0, speed_q8 = 384, turrets_firing = 6, stagger_ticks = 8 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 12 }
counterplay = "Turrets fire in a left-right zigzag, one shot each. The shots cross in front of the player: move up or down, not sideways."

[[patterns]]
id = "p2_lone_gate"
name = "LONE GATE"
telegraph = { ticks = 30, cue = "the surviving gate's lamps all light red" }
attack = { kind = "spread", count = 3, spread_deg = 24, speed_q8 = 320, turrets_firing = 3 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 9 }
counterplay = "When one gate falls the other steps all the way across in 8-px moves while its turrets fire 3-way fans. Stay on the wrecked side; it never reaches the far wall."

[[animations]]
id = "edge_step"
frames = 2
ticks_per_frame = 6
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "the gate's leading edge column redrawn one 8-px step further in, water churning white along it"

[[animations]]
id = "turret_turn"
frames = 16
ticks_per_frame = 4
loop = true
model = "offline"
duration = 3
request_frames = 16
motion = "turret rotates through sixteen headings, baked over its deck patch"

[[animations]]
id = "gate_wreck"
frames = 3
ticks_per_frame = 10
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "the gate's plates buckle, crystal veins go dark, and it sinks into foaming blue water"

[ludo]
concept_prompt = "Two massive steel canal lock gates seen from directly above, closing toward each other across a bright blue canal. Riveted dark grey plates, yellow and black hazard stripes on the leading edges, red warning lamps, three round gun turrets in a row on each gate, cyan crystal veins in the hydraulic pipes. Orthographic top-down, lit from the upper left, short shadows to the lower right."
setpiece_prompt = "one half of a steel canal lock gate from directly above, dark riveted plates, yellow hazard leading edge, red lamps, three empty turret rings, cyan crystal veins in pipes, on blue canal water"
+++

# LOCKMASTER

## Personality and role

The canal's own machinery turned into a vice. LOCKMASTER teaches the stage's theme, **pressure**:
the space to dodge shrinks on a timer, and the side guns of FLANK pay off when targets close in from
the sides.

## Arena

Section 2 of SLUICEWORKS, the canal locks, around screen 12. The scroll stops with the lock chamber
filling the view; the gates start fully open against the chamber walls (each 112 px wide), leaving
the central 80 px of canal plus their decks. GNAT swarms do not spawn during the fight.

## Phases and patterns

**Phase 1 (90 → 45).** STEP CLOSE runs continuously; CROSS FIRE every 2 s. Each turret is an
independent 16-angle patch; destroying one leaves a smoking well.

**Phase 2 (45 → 0).** Starts when one gate reaches 0 HP (its half then sinks). The survivor runs LONE
GATE until destroyed.

**Weakness.** FLANK: its side guns hit both gates at once while the player holds the centre.

## Budget notes

Bitmap only. Sliding is done without moving a picture: the gate body is a repeating plate texture, so
each 8-px step DMAs just the new leading-edge column (1x10 cells) into both buffers and restores the
water behind the trailing column. Turret patches are re-based on each step (the baker stores each
turret at every one of its 8 step positions). Bullets: at most 2 bucket channels per line.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
