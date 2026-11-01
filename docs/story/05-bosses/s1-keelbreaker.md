+++
id = "midboss.s1.keelbreaker"
kind = "midboss"
name = "KEELBREAKER"
title = "the Lattice-grown destroyer in the docks"
stage = "stage.s1"
status = "approved"
revision = 1
layer = "bitmap"
hp = 72
weak_to = "beam"
score = 10000
time_bonus = { par_s = 40, max = 10000 }
drops = { spark = 3 }
music_cue = "cue.midboss"
cell_rect = [12, 32]
angles = 16
anchor = "center"
identity = "a long grey Accord destroyer seen from above, crystal growing through its deck seams, three gun turrets and a cyan-glowing faceted bridge"
silhouette = "long narrow hull, pointed bow up, square stern, three round turret rings, a raised block bridge mid-deck"
proportions = "hull eight times as long as wide; turrets a third of the beam; bridge a quarter of the length"
materials = "battleship-grey painted steel, white deck numbers worn away, hazard-striped turret rings, crystal facets in cyan and light blue along every seam"
acting = "steams ahead steadily, wake foaming behind; turrets track slowly and recoil on each shot; the bridge pulses before it fires"
forbidden = ["crew or people on deck", "national flags or markings", "real ship classes", "eyes or faces in the crystal", "organic growth"]
hurtbox = { x = -48, y = -128, w = 96, h = 256 }

[[colors]]
role = "primary"
c64 = 12
note = "mid grey hull paint"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey deck shadow, turret wells, lower-right cast shadow on the water"
required = true

[[colors]]
role = "crystal"
c64 = 3
note = "cyan crystal along the seams and on the bridge"
required = true

[[colors]]
role = "glow"
c64 = 14
note = "light blue facet shading; the bridge charge glow"
required = false

[[colors]]
role = "accent"
c64 = 7
note = "yellow hazard stripes on the turret rings"
required = false

[[parts]]
id = "turret_fore"
layer = "bitmap"
cell_rect = [4, 4]
hp = 12

[[parts]]
id = "turret_mid"
layer = "bitmap"
cell_rect = [4, 4]
hp = 12

[[parts]]
id = "turret_aft"
layer = "bitmap"
cell_rect = [4, 4]
hp = 12

[[parts]]
id = "bridge_core"
layer = "bitmap"
cell_rect = [6, 6]
hp = 36

[[phases]]
from_hp = 72
to_hp = 36
patterns = ["p1_broadside", "p1_wake_lob"]

[[phases]]
from_hp = 36
to_hp = 0
patterns = ["p2_bridge_ring", "p1_broadside"]

[[patterns]]
id = "p1_broadside"
name = "BROADSIDE"
telegraph = { ticks = 20, cue = "turret muzzle flashes cyan" }
attack = { kind = "aimed", count = 3, spread_deg = 0, speed_q8 = 320, interval_ticks = 6 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 9 }
counterplay = "Each living turret fires a 3-round aimed burst in turn, fore to aft. Drift sideways between bursts; the aft turret cannot aim past the bow, so hugging the bow side halves the fire."

[[patterns]]
id = "p1_wake_lob"
name = "WAKE MINES"
telegraph = { ticks = 30, cue = "stern hatch opens" }
attack = { kind = "lob", count = 2, spread_deg = 40, speed_q8 = 192, burst_on_land = 4 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 10 }
counterplay = "Two slow crystal mines arc out of the stern and split into a 4-way cross where they land. Their landing spots are marked by a white splash ring; leave the ring before the splash."

[[patterns]]
id = "p2_bridge_ring"
name = "BRIDGE RING"
telegraph = { ticks = 36, cue = "bridge facets brighten to white" }
attack = { kind = "ring", count = 12, spread_deg = 360, speed_q8 = 256, gap = 2 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 12 }
counterplay = "A 12-shot ring with a two-bullet gap aimed at the player's position at telegraph time. Stay still through the glow, then step into the gap; a BEAM lance down the deck keeps damage on the bridge while dodging."

[[animations]]
id = "turret_turn"
frames = 16
ticks_per_frame = 4
loop = true
model = "offline"
duration = 3
request_frames = 16
motion = "turret rotates through sixteen headings, baked over the deck patch"

[[animations]]
id = "bridge_pulse"
frames = 4
ticks_per_frame = 9
loop = true
model = "offline"
duration = 3
request_frames = 16
motion = "bridge crystal facets brighten from light blue to white and back"

[[animations]]
id = "wreck"
frames = 3
ticks_per_frame = 8
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "turret ring cracks, blackens, and becomes a smoking crater in the deck"

[ludo]
concept_prompt = "A long grey naval destroyer seen from directly above, bow pointing up, sailing through a harbour. Pale cyan and light blue faceted crystal grows through every deck seam. Three round gun turrets on hazard-striped rings, a raised square bridge block glowing cyan in the middle. Foaming white wake behind the stern. Orthographic top-down, lit from the upper left, short shadow to the lower right on dark blue water."
setpiece_prompt = "a long grey destroyer hull from directly above, bow up, crystal growth along deck seams, three empty turret rings and a faceted cyan bridge, white wake behind, on dark blue harbour water"
+++

# KEELBREAKER

## Personality and role

The first big thing the Lattice ever took from the Accord: a harbour destroyer, still under steam,
its crew long ashore and its deck seams split by crystal. It is the player's first midboss and the
game's first lesson in **ground-layer giants**: it is huge, it fills a third of the screen, and it costs
no sprites. It never threatens by contact (the TALON flies over it); its threat is fire.

## Arena

Section 2 of BREAKWATER, the docks, around screen 12. The destroyer slides out from between two piers
heading up-screen. When its bow reaches the top third of the view, the stage scroll eases from 0.5 to
**0.125 px/tick**: it reads as the ship steaming ahead at nearly the TALON's speed. It stays on screen
about 40 s; if not destroyed, the scroll returns to normal and the screen overtakes it (no penalty but
no score or sparks). Patrol SKIFFs are suppressed during the fight.

## Phases and patterns

**Phase 1 (72 → 36).** BROADSIDE from each living turret in turn, and WAKE MINES from the stern every
fourth cycle. Every turret is a separate 16-angle patch; destroying one swaps it for a crater patch.

**Phase 2 (36 → 0).** Triggered when all three turrets are down or total HP falls to 36. The bridge
core's crystal plate cracks open and it fires BRIDGE RING, alternating with BROADSIDE from any turret
still alive.

**Weakness.** BEAM pierces ground targets: a lance held along the keel line hits every part beneath it
for double damage. This is the stage's quiet hint that BEAM, the boss's reward, is worth arming.

## Budget notes

Bitmap only: zero sprite channels. All bullets go to the bucket layer and never exceed 2 bucket
channels per line (12 bullets maximum). Each part has its own patch set baked over its exact deck patch
and over the moving sea: because the hull is fixed in the strip, the "under way" illusion comes from
the slowed scroll plus 4-frame animated wake and bow-wave patches.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
