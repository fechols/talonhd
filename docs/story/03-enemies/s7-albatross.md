+++
id = "enemy.s7.albatross"
kind = "enemy"
name = "ALBATROSS"
stage = "stage.s7"
faction = "captured"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "L"
cells = [3, 3]
expanded = false
angles = 1
anchor = "center"
identity = "an Accord cargo flying boat seen from directly above: very long straight light grey wings, a boat hull with a purple crystal launch bay"
silhouette = "very long straight wings, slim hull, wingtip floats, glowing open bay"
proportions = "wingspan the full width; hull a quarter of the span wide; bay half the hull length"
materials = "light grey painted metal, dark grey panel lines and floats, purple and light blue crystal bay"
acting = "glides in slow and level, opens its belly, lets drones fall out in strings, soars away"
forbidden = ["real aircraft types", "a bird shape or feathers", "national markings", "eyes or a face", "propellers"]
hurtbox = { x = -16, y = -28, w = 32, h = 56 }
domain = "air"
hp = 50
contact_damage = 1
score = 5000
weak_to = "twin"
resists = "flank"
drops = { spark = 2 }

[behavior]
archetype = "carrier"
params = { enter_speed_q8 = 160, cross = "top_to_hold", hold_line = 44, hold_ticks = 480, launch = "enemy.common.wasp", launch_count = 4, launch_waves = 3, launch_gap_ticks = 140, exit_speed_q8 = 192 }
attack = { kind = "launch", count = 4, spread_deg = 0, speed_q8 = 640, telegraph_ticks = 30, cooldown_ticks = 140 }
placement = "S7 cloud deck and the fleet; one at a time; holds at line 44, never below line 108; exits upward"
sprite_budget = { channels_on_line = 3, bullets_max = 4 }
death_vfx = "vfx.explosion_l"

[[colors]]
role = "primary"
c64 = 15
note = "light grey wings (private ink of the outer columns)"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal bay (private ink of the centre column)"
required = true

[[colors]]
role = "crystal_light"
c64 = 14
note = "light blue bay glow (shared pair, proposed)"
required = false

[[colors]]
role = "panel"
c64 = 11
note = "dark grey panels and floats (shared pair, proposed)"
required = false

[[animations]]
id = "glide"
frames = 2
ticks_per_frame = 10
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "a slow steady glide, wingtips flexing very slightly, bay doors shut"

[[animations]]
id = "bay_open"
frames = 3
ticks_per_frame = 10
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "long bay doors along the hull slide apart and the crystal inside flares brightly"

[ludo]
sprite_prompt = "an Accord cargo flying boat seen from directly above: very long straight light grey wings, a boat hull with a purple crystal launch bay, nose pointing straight down, top-down view"
+++

# ALBATROSS

## Concept

The Accord's long-range cargo flying boats, now the Lattice's drone tenders in the storm layer. An
ALBATROSS fires nothing itself; it **multiplies**: every 140 ticks it drops a string of 4 WASPs. It
teaches **priority under pressure**: kill the carrier fast (5 s of TWIN focused on the narrow hull,
which is the only hurtbox; the wings are not) and the sky empties; chase the WASPs and the sky fills.

## Behaviour pattern

1. Glides in from the top at 0.625 px/tick and holds at line 44 for up to 480 ticks.
2. Every 140 ticks: bay opens (30-tick telegraph), 4 WASPs drop out on the `drop_u` curve, 10 ticks
   apart. Three launch waves in all. The fourth WASP of each launch is a leader (yellow core, spark).
3. After the hold, climbs out the top at 0.75 px/tick. If it leaves alive, its remaining WASPs stay.
4. Death: `vfx.explosion_l` chain; 2 sparks; any WASP still in the bay dies with it.

## Placement rules

- S7 cloud deck and fleet sections; one ALBATROSS at a time and never with a TEMPEST.
- An L at line 44 costs 3 channels on lines 12-75. Its WASPs count toward the 2-string WASP limit, so
  S7's "squadrons of 16" are authored as free waves only when no carrier is present.
- Hurtbox is the hull only (32 px wide): shots that strike the wings are absorbed with a ricochet flash and do no damage, while contact with the
  full silhouette is still lethal to the player (contact uses the art mask).

## Readability

The widest, thinnest thing in the sky: a long grey line with a purple slot. When the slot glows, WASPs
come out of it.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
