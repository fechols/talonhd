+++
id = "enemy.s8.climber_pod"
kind = "enemy"
name = "CLIMBER POD"
stage = "stage.s8"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [4, 5]
angles = 1
anchor = "center"
identity = "a white-grey elevator climber pod clamped to a vertical cable seen from directly above: roller clamps top and bottom, side gun ports, purple crystal plating"
silhouette = "rectangle around a vertical cable, roller clamps at both ends, gun ports on both sides"
proportions = "pod one and a half times as tall as wide; cable a fifth of the pod width; clamps a quarter of the height each"
materials = "light grey and white painted panels, dark grey cable and clamps, purple crystal plates, orange hazard markings"
acting = "grinds along the cable, stops, opens its side ports and fans shots out sideways, grinds on"
forbidden = ["people or crew", "national markings or logos", "eyes or a face", "windows with figures", "flags"]
hurtbox = { x = -12, y = -16, w = 24, h = 32 }
domain = "cable"
hp = 12
contact_damage = 0
score = 700
weak_to = "beam"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "tank"
params = { path = "cable", path_step_px = 2, speed_q8 = 192, direction = "up_or_down", stop_every_px = 72, stop_ticks = 50 }
attack = { kind = "spread", count = 6, spread_deg = 60, speed_q8 = 320, telegraph_ticks = 20, cooldown_ticks = 0 }
placement = "S8 upper atmosphere and orbit; on the SKYHOOK cable only; at most 2 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 6 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 15
note = "light grey panels"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey cable and clamps"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal plates"
required = true

[[colors]]
role = "hazard"
c64 = 8
note = "orange hazard markings where the cell allows"
required = false

[[animations]]
id = "climb"
frames = 2
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the roller clamps turn along the cable, the pod juddering slightly"

[[animations]]
id = "ports"
frames = 3
ticks_per_frame = 7
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "panels on both sides of the pod slide open to reveal glowing crystal gun ports"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the pod split open and stuck on the cable, panels peeled back, crystal dark"

[ludo]
sprite_prompt = "a white-grey elevator climber pod clamped to a vertical cable seen from directly above: roller clamps top and bottom, side gun ports, purple crystal plating, cable running vertically, top-down view"
+++

# CLIMBER POD

## Concept

The SKYHOOK's maintenance climbers, captured on the cable. The cable runs up the centre of the screen as
S8's only "ground", and CLIMBER PODs ride it, stopping to fire **sideways fans** out from the cable. They
teach **not to hug the centre line**: the cable looks like the safe spine of the screen and is the most
dangerous place to sit. BEAM, piercing down the cable's length, can hit several pods in a line.

## Behaviour pattern

- Rides the cable path at 0.75 px/tick relative to the cable (up or down, authored), in 2-px baked samples
  over the exact cable patch.
- Every 72 px: stops for 50 ticks. 20-tick port-opening telegraph, then fires two 3-way fans, one to each
  side, each 30 degrees wide and angled 15 degrees downward, at 1.25 px/tick.
- Fires regardless of whether the player is above it (its fans are lateral).

## Placement rules

- S8 upper atmosphere and orbit, on the cable only; at most 2 per screen, never both stopped on the same
  lines.
- The cable is a narrow strip, so pod rects are baked over cable-plus-starfield patches; the star bucket's
  twinkles avoid the pod rect.

## Readability

A box clamped around a line: obviously a lift on a cable. The side panels opening are the telegraph, and
they point exactly where the fans go.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
