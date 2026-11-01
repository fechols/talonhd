+++
id = "enemy.s3.vulture"
kind = "enemy"
name = "VULTURE"
stage = "stage.s3"
faction = "captured"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "L"
cells = [3, 3]
expanded = false
angles = 1
anchor = "center"
identity = "a heavy Accord gunship seen from directly above: broad light grey wings with two engine pods each, a fuselage split by a purple crystal core"
silhouette = "broad straight wings, four engine pods, thick fuselage, crystal core in the centre"
proportions = "as wide as long; fuselage a third of the width; the crystal core a third of the fuselage length"
materials = "light grey painted armour, dark grey engine pods and panel lines, purple and light blue crystal core"
acting = "slides into position like a heavy freighter, hangs, rolls its guns across the screen, lumbers out"
forbidden = ["real aircraft types", "national markings or roundels", "propellers", "eyes or a face", "a bird shape"]
hurtbox = { x = -30, y = -26, w = 60, h = 52 }
domain = "air"
hp = 40
contact_damage = 1
score = 3000
weak_to = "burst"
resists = "none"
drops = { spark = 2 }

[behavior]
archetype = "gunship"
params = { enter_speed_q8 = 192, hold_line = 48, sway_px = 48, sway_q8 = 64, stay_ticks = 600, exit_speed_q8 = 256 }
attack = { kind = "spread", count = 5, spread_deg = 60, speed_q8 = 352, telegraph_ticks = 20, cooldown_ticks = 90 }
placement = "S3 one per section; holds at line 48 and never descends below line 112; exits upward"
sprite_budget = { channels_on_line = 3, bullets_max = 10 }
death_vfx = "vfx.explosion_l"

[[colors]]
role = "primary"
c64 = 15
note = "light grey wings (private ink of the outer two columns)"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal core (private ink of the centre column)"
required = true

[[colors]]
role = "crystal_light"
c64 = 14
note = "light blue facets (shared pair, proposed)"
required = false

[[colors]]
role = "panel"
c64 = 11
note = "dark grey engine pods and panels (shared pair, proposed)"
required = false

[[animations]]
id = "hover"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "hangs heavily in place, engine pods glowing and dimming in turn, crystal core pulsing slowly"

[[animations]]
id = "telegraph"
frames = 2
ticks_per_frame = 10
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "gun ports along the wing leading edges slide open and the crystal core flares bright"

[[animations]]
id = "damaged"
frames = 2
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "one engine pod trails dark smoke and sparks, the wing panel beside it torn open"

[ludo]
sprite_prompt = "a heavy Accord gunship seen from directly above: broad light grey wings with two engine pods each, a fuselage split by a purple crystal core, nose pointing straight down, top-down view"
+++

# VULTURE

## Concept

The Accord's heavy desert gunships, captured at their depot and now riding escort on the power trains.
The VULTURE is the first **large** air unit: a 3×3 sprite (72×63 px) with 40 HP, four seconds of
continuous TWIN fire. It teaches **commitment and reward**: stay under it with BURST at close range
and it melts in about a second and a half, but its 5-way fan is densest right there. It drops two sparks,
the first time a single kill moves the gauge twice.

## Behaviour pattern

1. Enters from the top edge at 0.75 px/tick, settles at line 48.
2. Sways ±48 px at 0.25 px/tick around its entry X.
3. Every 90 ticks: 20-tick telegraph (wing ports open, core flares), then a 5-way spread over 60
   degrees at 1.375 px/tick, aimed at the player. On alternate volleys it adds 2 aimed shots from the
   engine pods, 10 ticks later.
4. Below half HP it switches to the `damaged` loop and fires every 70 ticks.
5. After 600 ticks it climbs out the top at 1 px/tick. Escaped VULTUREs pay nothing.
6. Death: `vfx.explosion_l` with a 3-blast chain, 2 sparks scatter left and right.

## Placement rules

- One VULTURE per section of S3 (3 in the stage), never with KESTREL pairs or DUST DEVIL groups above
  line 112.
- An L costs 3 channels on its 63 lines, all above line 112; player band unaffected.
- **Private inks:** outer two columns light grey 15, centre column purple 4, so the core lives in its
  own hardware sprite.

## Readability

The biggest thing in the sky so far, with a purple heart: the target is literally the coloured middle.
Its wing-port telegraph gives every volley a visible cue.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
