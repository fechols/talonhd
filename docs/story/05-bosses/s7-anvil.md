+++
id = "boss.s7.anvil"
kind = "boss"
name = "ANVIL"
title = "the flagship dreadnought of the Lattice sky fleet"
stage = "stage.s7"
status = "approved"
revision = 1
layer = "hybrid"
hp = 320
weak_to = "burst"
score = 90000
time_bonus = { par_s = 100, max = 90000 }
reward = "seeker"
music_cue = "cue.boss"

# Visual identity: the hull is a bitmap two screens long; the bridge core lifts off as a sprite
cell_rect = [38, 50]
sprite_class = "B"
cells = [4, 8]
expanded = false
angles = 16
anchor = "center"
identity = "a colossal rigid airship dreadnought, armoured deck plates split by crystal ridges, rows of turret wells, a stepped bridge tower amidships"
silhouette = "a long blunt-nosed hull filling the screen, a spine of crystal ridges, a stepped bridge block; alone, a wedge-shaped bridge with a pointed keel"
proportions = "hull 304x400 display px; bridge core 4 cells wide for 4 rows, tapering to a 2-cell keel for 4 rows"
materials = "dark grey and mid grey riveted deck plates, red running lights, yellow deck markings, purple and light blue crystal ridges along the spine, white-hot engine vents"
acting = "the hull glides beneath at a crawl, turret wells iris open one by one, the bridge tower shudders, tears its moorings and rises with debris falling away"
forbidden = ["real airship names", "national markings", "faces", "eyes", "visible crew", "organic forms"]
hurtbox = { x = -44, y = -84, w = 88, h = 100 }

[[colors]]
role = "primary"
c64 = 11
note = "dark grey deck plates (hull bitmap and bridge sprite)"
required = true

[[colors]]
role = "secondary"
c64 = 12
note = "mid grey plating highlights"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal ridges; the sprite bridge uses 4 and 14, not the TALON's cyan"
required = true

[[colors]]
role = "lights"
c64 = 2
note = "dark red running lights (light red 10 on the sprite)"
required = false

[[colors]]
role = "markings"
c64 = 7
note = "yellow deck markings and turret-well rims"
required = false

[[parts]]
id = "turret_1"
layer = "bitmap"
cell_rect = [4, 4]
hp = 15
weak = true
note = "eight turret wells along the hull, 16-angle patches plus closed/open/wrecked; BURST double damage"

[[parts]]
id = "turret_2"
layer = "bitmap"
cell_rect = [4, 4]
hp = 15
weak = true

[[parts]]
id = "turret_3"
layer = "bitmap"
cell_rect = [4, 4]
hp = 15
weak = true

[[parts]]
id = "turret_4"
layer = "bitmap"
cell_rect = [4, 4]
hp = 15
weak = true

[[parts]]
id = "turret_5"
layer = "bitmap"
cell_rect = [4, 4]
hp = 15
weak = true

[[parts]]
id = "turret_6"
layer = "bitmap"
cell_rect = [4, 4]
hp = 15
weak = true

[[parts]]
id = "turret_7"
layer = "bitmap"
cell_rect = [4, 4]
hp = 15
weak = true

[[parts]]
id = "turret_8"
layer = "bitmap"
cell_rect = [4, 4]
hp = 15
weak = true

[[parts]]
id = "bridge"
layer = "sprite"
sprite_class = "B"
cells = [4, 8]
hp = 200
weak = true
note = "while docked it is part of the hull bitmap (invulnerable); after lift-off its armour plates take double damage from BURST"

[[phases]]
from_hp = 320
to_hp = 200
patterns = ["p1_broadside", "p1_flak_wall", "p1_launch_deck", "p1_spine_sweep"]
note = "the hull scrolls under at 0.125 px/tick; turrets open in pairs; turrets that scroll off unkilled are skipped (their HP is lost from the time bonus, not owed)"

[[phases]]
from_hp = 200
to_hp = 0
patterns = ["p2_lift_ring", "p2_keel_lance", "p2_engine_wash", "p2_escort_call"]
note = "the scroll stops when the bridge reaches mid-screen; the bridge tears free as a 4x8 sprite"

[[patterns]]
id = "p1_broadside"
name = "BROADSIDE"
telegraph = { ticks = 24, cue = "two facing turret wells iris open, yellow rims flash" }
attack = { kind = "aimed", count = 3, spread_deg = 12, speed_q8 = 416, turrets = 2, volleys = 3, gap_ticks = 14 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 18 }
counterplay = "Turrets open in left/right pairs; each fires a tight aimed 3-way. Hover over the open well with BURST: the well is the weak point and the spread is narrow at the source."

[[patterns]]
id = "p1_flak_wall"
name = "FLAK WALL"
telegraph = { ticks = 30, cue = "FLAK BATTERY patches on the deck edge puff grey smoke" }
attack = { kind = "burst", count = 6, speed_q8 = 224, enemy = "enemy.s7.flak_battery", burst_after_ticks = 40, shards = 6 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 36 }
counterplay = "Slow shells that burst into 6 shards after 40 ticks; the burst point is fixed at launch, so stay away from shells, not shards."

[[patterns]]
id = "p1_launch_deck"
name = "LAUNCH DECK"
telegraph = { ticks = 30, cue = "a deck elevator patch slides open, red lights chase" }
attack = { kind = "launch", count = 4, enemy = "enemy.common.wasp", interval_ticks = 10 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 8 }
counterplay = "Four WASPs climb out of the elevator on a fixed swoop; the leader drops a spark."

[[patterns]]
id = "p1_spine_sweep"
name = "SPINE SWEEP"
telegraph = { ticks = 40, cue = "crystal ridges along the spine glow in a running chain" }
attack = { kind = "fixed", count = 16, spread_deg = 180, speed_q8 = 320, order = "spine_front_to_back", interval_ticks = 3 }
sprite_budget = { boss_channels = 0, bucket_channels = 2, bullets_max = 16 }
counterplay = "A wave of shots runs down the spine: cross over the spine behind the wave."

[[patterns]]
id = "p2_lift_ring"
name = "MOORINGS"
telegraph = { ticks = 45, cue = "the bridge shudders, mooring patches snap, debris falls away" }
attack = { kind = "ring", count = 16, spread_deg = 360, speed_q8 = 256 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 16 }
counterplay = "One slow ring as the bridge lifts; slip a gap."

[[patterns]]
id = "p2_keel_lance"
name = "KEEL LANCE"
telegraph = { ticks = 36, cue = "the keel point glows, a dotted guide line drops to the bottom edge" }
attack = { kind = "beam", count = 1, width_px = 16, sweep = "follows player_x at 0.75 px/tick", duration_ticks = 90 }
sprite_budget = { boss_channels = 4, bucket_channels = 1, bullets_max = 4 }
counterplay = "The lance follows you slower than you can move: keep drifting one way and it never catches up. The bridge also drifts toward your side, so stay near an edge."

[[patterns]]
id = "p2_engine_wash"
name = "ENGINE WASH"
telegraph = { ticks = 24, cue = "white-hot vents on both bridge wings flare" }
attack = { kind = "spread", count = 7, spread_deg = 120, speed_q8 = 384, volleys = 3, alternate_offset_deg = 8 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 21 }
counterplay = "Alternating wide fans: the gaps move 8 degrees each volley; step with them."

[[patterns]]
id = "p2_escort_call"
name = "LAST ESCORT"
telegraph = { ticks = 30, cue = "red signal lamp on the bridge blinks" }
attack = { kind = "launch", count = 2, enemy = "enemy.common.lancer", side = "alternating", interval_ticks = 45 }
sprite_budget = { boss_channels = 4, bucket_channels = 2, bullets_max = 8 }
counterplay = "Only below 100 HP. Two LANCER divers drop from the bridge wings one after another, never together. Each dives at your x at the telegraph: sidestep, and they flash past."

[[occupancy]]
pose = "bridge_lifted"
rows = ["1111", "1111", "1111", "1111", "0110", "0110", "0110", "0110"]
note = "rows 0-3 (lines 0-83) 4 wide above the ceiling; rows 4-7 the 2-wide keel"

[[animations]]
id = "turret_well"
frames = 16
ticks_per_frame = 3
loop = true
model = "offline"
duration = 0
request_frames = 0
motion = "turret master rotated offline through 16 angles and baked into each well"

[[animations]]
id = "bridge_lift"
frames = 4
ticks_per_frame = 8
loop = false
model = "hydra-plus"
duration = 4
request_frames = 25
motion = "the bridge tower tears free of the deck and rises, debris falling away, its shadow sliding down-right"

[[animations]]
id = "bridge_hover"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "hovers with a slow roll, engine vents pulsing white"

[ludo]
concept_prompt = "Top-down orthographic view of a colossal rigid airship dreadnought seen from directly above, filling the frame: riveted dark and mid grey armoured deck plates, a spine of purple and light blue crystal ridges, rows of round turret wells with yellow rims, red running lights, a stepped bridge tower amidships, white-hot engine vents at the stern, wisps of cloud at its edges. Lit from the upper left."
setpiece_prompt = "top-down view of a long section of airship hull deck, dark grey riveted plates, a crystal spine ridge, round turret wells with yellow rims, red running lights, seamless top and bottom"
sprite_prompt = "top-down view of a wedge-shaped airship bridge tower with a pointed keel, dark grey armour, purple crystal ridges, white engine vents on both wings"
+++

# ANVIL

# Personality and role

The flagship. Every other boss defended a place; the ANVIL *is* the place, a fortress that flies. It is
slow, vast and certain of itself: the hull does not attack so much as present its guns one pair at a
time. When the hull is stripped, the bridge does what no airship should and rips itself free to fight
alone. It is the last guardian before the climb.

# Arena

The end of STORMFRONT: the cloud deck below, a still starry night above the storm, and the hull of the
ANVIL sliding underneath the TALON, as the bitmap layer for two full screens. The scroll slows to
0.125 px/tick as the bow appears (about 53 s for the hull) and stops when the bridge reaches mid-screen.

# Phases and patterns

**Phase 1 (320 → 200).** Eight turret wells scroll past in pairs: BROADSIDE, FLAK WALL from the deck
edge batteries, LAUNCH DECK WASPs from an elevator, SPINE SWEEP down the crystal ridge. BURST does double
damage into an open well. Turrets that scroll off unkilled are lost. The phase ends when the bridge
reaches mid-screen and the scroll stops (any turret HP left counts as destroyed).

**Phase 2 (200 → 0).** MOORINGS: the bridge tears free as a 4x8 sprite and rings once. KEEL LANCE drags a
slow beam after the TALON, ENGINE WASH fans alternate, and below 100 HP LAST ESCORT drops two LANCER
divers, one at a time.

**Defeat.** The bridge drops back into its own hull and the whole ANVIL breaks in two, both halves
sinking into the clouds in a long chain of BLAST L explosions. Above: only stars, and the cable.

# Budget notes

- Phase 1 hull and turrets are bitmap: 0 channels. WASPs (1 channel each) and bullets (2 buckets) only.
- Phase 2 bridge: rows 0-3 (lines 0-83) bridge 4 + buckets 2 = 6 (+ up to 2 for a WASP); the TALON is
  clamped at line 88.
- Rows 4-7 the 2-wide keel: keel 2 + ship 2 + inner pods 2 + buckets 2 = 8.
- LAST ESCORT launches one LANCER (M, 2 channels) at a time from the bridge wings and only while the
  KEEL LANCE is not firing; a LANCER sharing the keel's lines is lowest mux priority and flickers first.
  A 4-wide TEMPEST is deliberately not used here: it would break the player-band rule.

# Weapon reward

The CORE SPARK moves the gauge cursor to **SEEKER**, whose darts track the CROWN's opening facets and its
Heart.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
