+++
id = "midboss.s6.monolith"
kind = "midboss"
name = "MONOLITH"
title = "the temple stone that opens on a crystal lens"
stage = "stage.s6"
status = "approved"
revision = 1
layer = "bitmap"
hp = 110
weak_to = "beam"
score = 20000
time_bonus = { par_s = 40, max = 20000 }
drops = { spark = 3 }
music_cue = "cue.midboss"
cell_rect = [10, 14]
angles = 16
anchor = "center"
identity = "a tall mossy grey stone slab seen from above, carved with abstract stepped grooves, splitting open to reveal a rotating faceted crystal lens"
silhouette = "long rectangle splitting lengthwise into two halves, a hexagonal lens in the gap"
proportions = "80 px wide, 112 px long; the lens 48 px across when fully open"
materials = "mossy mid-grey stone, light green moss, vine tangles, stepped abstract grooves, a cyan and light blue faceted lens with a white iris shutter of six blades"
acting = "still as a stone, then grinds open; the lens turns to face the player in sixteen steps and its shutter blades snap open before firing"
forbidden = ["religious iconography", "carved faces or figures", "eyes of any kind (the lens is a geometric shutter, never an eye)", "people", "organic growth on the crystal"]
hurtbox = { x = -40, y = -56, w = 80, h = 112 }

[[colors]]
role = "primary"
c64 = 12
note = "mid grey stone"
required = true

[[colors]]
role = "secondary"
c64 = 13
note = "light green moss and vines"
required = true

[[colors]]
role = "crystal"
c64 = 3
note = "cyan lens facets"
required = true

[[colors]]
role = "glow"
c64 = 14
note = "light blue facet shading; white glint on the shutter blades"
required = false

[[parts]]
id = "slab"
layer = "bitmap"
cell_rect = [10, 14]
hp = 0
note = "invulnerable stone; only the lens takes damage"

[[parts]]
id = "lens"
layer = "bitmap"
cell_rect = [6, 6]
hp = 110

[[phases]]
from_hp = 110
to_hp = 55
patterns = ["p1_aperture", "p1_scatter"]

[[phases]]
from_hp = 55
to_hp = 0
patterns = ["p2_sweep_lance", "p1_scatter"]

[[patterns]]
id = "p1_aperture"
name = "APERTURE"
telegraph = { ticks = 24, cue = "the six shutter blades open one by one" }
attack = { kind = "aimed", count = 6, spread_deg = 0, speed_q8 = 384, interval_ticks = 4 }
sprite_budget = { boss_channels = 0, bucket_channels = 1, bullets_max = 6 }
counterplay = "One blade opens per tick group; when all six are open the lens fires a 6-shot aimed stream. Count the blades, then sidestep once on the sixth."

[[patterns]]
id = "p1_scatter"
name = "SCATTER"
telegraph = { ticks = 30, cue = "the slab halves grind shut to a slit, the lens flashes white" }
attack = { kind = "ring", count = 10, spread_deg = 360, speed_q8 = 224, split_on_age = 40, split_count = 2 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 20 }
counterplay = "A 10-shard ring whose shards each split in two after 40 ticks. Stay close to the stone and dodge the ring before it splits, or far away where the split pairs are wide."

[[patterns]]
id = "p2_sweep_lance"
name = "SWEEP LANCE"
telegraph = { ticks = 40, cue = "a dotted guide line appears along the lens heading" }
attack = { kind = "beam", count = 1, spread_deg = 90, width_px = 16, duration_ticks = 90, sweep_deg_per_tick = 1 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 12 }
counterplay = "The lens fires a narrow lance and sweeps it across a 90-degree arc toward the player's side. The dotted guide shows the start; cross the lance's path behind it, never ahead."

[[animations]]
id = "slab_open"
frames = 4
ticks_per_frame = 8
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "the stone splits lengthwise and both halves slide apart, moss tearing, revealing the crystal lens"

[[animations]]
id = "lens_turn"
frames = 16
ticks_per_frame = 3
loop = true
model = "offline"
duration = 3
request_frames = 16
motion = "the hexagonal lens turns through sixteen headings, facets catching the light"

[[animations]]
id = "shutter"
frames = 4
ticks_per_frame = 4
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "six white shutter blades rotate open around the lens centre"

[ludo]
concept_prompt = "A tall rectangular grey stone slab lying in jungle ruins seen from directly above, covered in moss and vines, carved with abstract stepped geometric grooves. It is split lengthwise and slid open, revealing a large hexagonal crystal lens of cyan and light blue facets with a six-bladed white iris shutter. No faces, no eyes, no figures. Orthographic top-down, lit from the upper left, short shadows to the lower right."
setpiece_prompt = "a mossy grey stone slab from directly above, split open lengthwise, a hexagonal cyan crystal lens with a six-bladed shutter in the gap, abstract stepped grooves, vines, jungle stone floor"
+++

# MONOLITH

## Personality and role

The quietest thing in the jungle until it opens. MONOLITH is a single armoured target with one
vulnerable centre: it teaches **waiting for the window** and rewards BEAM, the weapon that pierces down
into ground targets.

## Arena

Section 2 of VERDIGRIS, the temple ruins, around screen 12. The slab lies on a stepped terrace; the
scroll stops with it in the upper half. Vine-choked statues (abstract geometric shapes) frame the
terrace; STATUE TURRETs stay dormant during the fight.

## Phases and patterns

**Phase 1 (110 → 55).** APERTURE and SCATTER alternate. The lens is hittable only while the slab is
open (it closes to a slit for SCATTER).

**Phase 2 (55 → 0).** The slab opens fully and stays open; SWEEP LANCE alternates with SCATTER.

**Weakness.** BEAM: the held lance strikes the lens through the slit even while the slab is nearly
closed, for double damage.

## Budget notes

Bitmap only. The plan called the target a "crystal eye"; the bible forbids eyes on Lattice things, so
it is a hexagonal **lens** with a six-bladed iris shutter. 16 lens headings x 3 shutter states are baked
over the opened slab. The SWEEP LANCE is drawn in bucket sprites stepped along its line; as it rotates
its per-line cost stays at 1 bucket channel, leaving 1 for ordinary shards.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
