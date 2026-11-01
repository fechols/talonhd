+++
id = "enemy.s7.tempest"
kind = "enemy"
name = "TEMPEST"
stage = "stage.s7"
faction = "captured"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "B"
cells = [4, 3]
expanded = false
angles = 1
anchor = "center"
identity = "an Accord storm gunship seen from directly above: light grey flying-wing hull, two red engine nacelles, two gun pods, light blue crystal spine plates"
silhouette = "broad flying wing, two inner nacelles, two outer gun pods, crystal spine"
proportions = "span four cells; chord three cells; nacelles a quarter of the span apart; gun pods at the wingtips"
materials = "light grey armour, light red nacelles and running lights, dark grey gun pods, purple and light blue crystal plates"
acting = "rolls in like weather, hangs over the fleet, swings its gun pods in sweeping arcs, retreats into cloud"
forbidden = ["real aircraft types", "national markings", "eyes or a face", "propellers", "a stealth bomber silhouette copied from reality"]
hurtbox = { x = -42, y = -26, w = 84, h = 52 }
domain = "air"
hp = 90
contact_damage = 1
score = 10000
weak_to = "seeker"
resists = "beam"
drops = { spark = 3 }

[behavior]
archetype = "gunship"
params = { enter_speed_q8 = 128, hold_line = 40, drift_px = 32, stay_ticks = 900, pod_sweep_deg = 90, pod_sweep_ticks = 120, phase2_at_hp = 45 }
attack = { kind = "spread", count = 7, spread_deg = 90, speed_q8 = 352, telegraph_ticks = 24, cooldown_ticks = 120 }
placement = "S7 storm cell and fleet; one per section; holds at line 40 and never descends below line 104; no other L or B air unit on screen"
sprite_budget = { channels_on_line = 4, bullets_max = 16 }
death_vfx = "vfx.explosion_l"

[[colors]]
role = "primary"
c64 = 15
note = "light grey wing (private ink of the outer two columns)"
required = true

[[colors]]
role = "nacelles"
c64 = 10
note = "light red nacelles and running lights (private ink of the inner two columns)"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal plates (shared pair, proposed)"
required = false

[[colors]]
role = "panel"
c64 = 11
note = "dark grey gun pods and panels (shared pair, proposed)"
required = false

[[colors]]
role = "crystal_dark"
c64 = 4
note = "purple facets on the spine, only where an inner column can spare its private ink (non-required)"
required = false

[[animations]]
id = "hover"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "hangs heavily in the wind, running lights blinking in sequence, crystal plates glinting"

[[animations]]
id = "pods"
frames = 4
ticks_per_frame = 8
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "the two wingtip gun pods swivel slowly from side to side in opposite directions"

[[animations]]
id = "damaged"
frames = 2
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "one nacelle trails dark smoke and sparks, crystal plates cracked and flickering"

[ludo]
sprite_prompt = "an Accord storm gunship seen from directly above: light grey flying-wing hull, two red engine nacelles, two gun pods, light blue crystal spine plates, nose pointing straight down, top-down view"
+++

# TEMPEST

## Concept

The sky fleet's heavy gunships: a 4×3 sprite grid (96×63 px), as big as a midboss and treated as a
regular enemy in S7, the air-power showcase. TEMPEST teaches the hard truth about BEAM: **armoured air
units shrug it off** (×0.5). It rewards SEEKER, whose darts, one per target, all converge on a target
this size, and it drops 3 sparks: the biggest gauge payout of any regular enemy.

## Behaviour pattern

1. Enters from the top at 0.5 px/tick and holds at line 40, drifting ±32 px.
2. **Phase 1 (90-46 HP):** every 120 ticks, 24-tick telegraph (nacelles flare), then a 7-way spread over
   90 degrees at 1.375 px/tick from the nose. Between spreads the two wingtip pods sweep 90-degree arcs
   in opposite directions over 120 ticks, each firing a shot every 15 ticks (a pair of crossing streams).
3. **Phase 2 (45-0 HP):** `damaged` loop; spreads every 90 ticks; pod streams every 10 ticks.
4. After 900 ticks it climbs away into cloud (no score).
5. Death: a 4-blast `vfx.explosion_l` chain across the wing; 3 sparks.

## Placement rules

- One per S7 section, alone: no other L or B air unit, no KESTREL pairs. Small WASP strings may enter
  below it only after line 104.
- A B 4×3 costs 4 channels on lines 9-72. With up to 2 bullet-bucket channels near it, those lines peak
  at 6 of 8; the player band is untouched.
- **Private inks:** outer columns light grey 15, inner columns light red 10, matching the symmetric art.

## Readability

A grey wing as wide as a third of the screen, with two red engines that flare before every spread. The
crossing pod streams are telegraphed by the pods' visible swivel.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
