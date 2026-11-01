+++
id = "enemy.s2.dome_mortar"
kind = "enemy"
name = "DOME MORTAR"
stage = "stage.s2"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [5, 5]
angles = 1
anchor = "center"
identity = "a white-grey observatory dome seen from directly above, its viewing slit opened on a stubby purple crystal mortar tube, gravel ring around the base"
silhouette = "circle, straight slit across the top, short tube in the slit, gravel ring"
proportions = "dome fills four fifths of the patch; slit a fifth of the dome width"
materials = "light grey painted metal dome, dark grey slit and rivets, purple crystal tube, brown gravel"
acting = "the slit grinds open, the tube thumps, a heavy shell sails up; the slit grinds shut"
forbidden = ["telescopes pointing at the viewer like an eye", "eyes or a face", "people", "religious symbols", "national markings"]
hurtbox = { x = -14, y = -14, w = 28, h = 28 }
domain = "ground"
hp = 8
contact_damage = 0
score = 600
weak_to = "flank"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "mortar"
params = { aim = "player_pos_at_fire", flight_ticks = 90, harmless_until_tick = 70, shell_hp = 1, shell_score = 50, splash = 4, closed_damage_scale = 0.5 }
attack = { kind = "lob", count = 1, spread_deg = 0, speed_q8 = 384, telegraph_ticks = 24, cooldown_ticks = 150 }
placement = "S2 observatory grounds; on gravel paths between larger domes; at most 2 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 5 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 15
note = "light grey dome"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey slit and rivets"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple mortar tube"
required = true

[[colors]]
role = "gravel"
c64 = 9
note = "brown gravel ring (background)"
required = false

[[animations]]
id = "open"
frames = 3
ticks_per_frame = 8
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the two halves of the dome slit grind apart to reveal the crystal tube inside"

[[animations]]
id = "fire"
frames = 2
ticks_per_frame = 4
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the crystal tube kicks down and flashes bright, a puff of pale dust around the slit"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the dome caved in with its slit torn open and the crystal tube snapped"

[ludo]
sprite_prompt = "a white-grey observatory dome seen from directly above, its viewing slit opened on a stubby purple crystal mortar tube, gravel ring around the base, top-down view"
+++

# DOME MORTAR

## Concept

The observatory's lesser domes now throw crystal shells into the sky. A mortar shell **goes up before
it comes down**: for most of its flight it is a harmless, growing shape heading for where you were.
DOME MORTARs teach that **some bullets can be shot**, and that a slow threat aimed at your old position
is solved by moving, not by panicking.

## Behaviour pattern

1. Within 140 lines above the player, the slit opens (24 ticks) and the mortar fires one shell at the
   player's position at the moment of firing.
2. The shell flies for exactly 90 ticks at whatever speed reaches that point (capped at 1.5 px/tick,
   `speed_q8` is the cap), drawn by the bullet bucket in 3 growing sizes (rising)
   then 3 shrinking (falling). It is harmless to the player until tick 70 and shootable throughout
   (1 HP, 50 points).
3. On landing it splashes into 4 slow shots (diagonals, 1 px/tick).
4. Cooldown 150 ticks; the slit closes between shots. Closed, it takes half damage.
5. Fires even when the player is above it (its shell can reach), the one ground target in S2 that does.

## Placement rules

- S2 observatory grounds; at most 2 per screen, and never more than 2 shells in flight game-wide
  (5 bullets incl. splash).
- Keep the landing area clear of BELL TURRET fans so the splash reads.

## Readability

A white dome with a dark line across it: open line, incoming shell. The growing shell is the clearest
"it's in the air" signal we can draw in one bucket channel.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
