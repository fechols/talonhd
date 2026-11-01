+++
id = "enemy.common.kestrel"
kind = "enemy"
name = "KESTREL"
stage = "common"
faction = "captured"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "W"
cells = [2, 1]
expanded = false
angles = 8
anchor = "center"
identity = "a light grey Accord interceptor jet seen from directly above: straight wide wings, twin tail fins, light blue crystal grown through the canopy"
silhouette = "short fuselage, straight wide wings, twin tail fins, crystal canopy"
proportions = "wingspan twice the fuselage length; crystal patch covers the canopy and wing roots"
materials = "light grey painted metal, dark grey panel lines and rivets, light blue crystal facets"
acting = "flies flat and fast in pairs, rolls into a sharp turn, snaps a volley on the turn"
forbidden = ["real aircraft types", "national markings or roundels", "a pilot visible", "eyes or a face", "swept delta wings like the TALON"]
hurtbox = { x = -20, y = -7, w = 40, h = 14 }
domain = "air"
hp = 3
contact_damage = 1
score = 300
weak_to = "seeker"
resists = "none"
drops = { spark = 0, pair_bonus = 500 }

[behavior]
archetype = "strafer"
params = { speed_q8 = 768, pair_offset_px = 28, cross_line = 56, turn_x = "player_x", turn_ticks = 16, exit = "reverse_up" }
attack = { kind = "spread", count = 3, spread_deg = 30, speed_q8 = 384, telegraph_ticks = 10, cooldown_ticks = 0 }
placement = "pairs enter from a side edge at line 56, cross to the player's column, turn and leave upward; never below line 96"
sprite_budget = { channels_on_line = 4, bullets_max = 6 }
death_vfx = "vfx.explosion_s"

[[colors]]
role = "primary"
c64 = 15
note = "light grey airframe (private ink)"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal at the canopy (shared pair, proposed)"
required = false

[[colors]]
role = "panel"
c64 = 11
note = "dark grey panel lines (shared pair, proposed)"
required = false

[[animations]]
id = "fly"
frames = 2
ticks_per_frame = 4
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "steady level flight, the crystal on the canopy glittering"

[[animations]]
id = "bank"
frames = 8
ticks_per_frame = 2
loop = false
model = "offline"
motion = "8 headings rotated from the master by our tools"

[ludo]
sprite_prompt = "a light grey Accord interceptor jet seen from directly above: straight wide wings, twin tail fins, light blue crystal grown through the canopy, flying to the right, top-down view"
+++

# KESTREL

## Concept

The Accord's own trainer-interceptors, left on the flight line when the Lattice came and now flown by
the crystal in their cockpits. They are the first enemy that shoots **on a schedule the player can see**:
a KESTREL pair crosses, turns where you are, and fires on the turn. The lesson is to be somewhere else
when the turn comes, then punish the exit. SEEKER darts were made for a target this fast.

## Behaviour pattern

1. A pair enters from one side edge at screen line 56, 28 px apart vertically, at 3 px/tick.
2. Each KESTREL crosses until it reaches the player's X at the moment of entry, then spends 16 ticks
   turning through the 8 headings back toward the edge it came from, climbing.
3. Mid-turn (after a 10-tick canopy flash) each fires a 3-way spread, 30 degrees, at 1.5 px/tick,
   centred on the player.
4. They leave upward-diagonally. Killing both members of a pair pays a 500 bonus.

## Placement rules

- Pairs only, never more than two pairs on screen. Both members fly above line 96, so a pair costs
  4 channels on lines well clear of the player band.
- Do not schedule a KESTREL pair while a VULTURE, ALBATROSS or TEMPEST is on screen (their lines overlap).
- **S8 variant (prose):** crystal-plated KESTREL, 6 HP, fires a 5-way spread; private ink purple 4.
  `variant_of = "enemy.common.kestrel"`.

## Readability

A wide plus-shape in pale grey with a cyan-blue crystal cockpit: plainly a captured human jet, not a
Lattice drone. The turn is the tell.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
