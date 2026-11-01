+++
id = "hd.ship.caracara"
kind = "hd_ship"
name = "CARACARA"
prototype = 11
stage = "stage.s3"
status = "approved"
revision = 3
crystal = "none"
identity = "a heavy twin-boom ground-attack craft in yellow and orange, a broad straight wing, a central armoured pod, two tail booms joined by a tailplane"
silhouette = "short blunt central pod, broad straight wing, two parallel tail booms, a crossbar tailplane, two engine pods on the booms"
materials = "yellow armour, orange booms and wing panels, dark seams, light blue canopy glass, cyan exhaust glow"
feel = "slow and wide, wider shot spread, a bigger hitbox: built for the canyon and the trains"
forbidden = ["real aircraft silhouette", "national markings or flags", "text or numbers on the airframe", "visible pilot", "missiles under the wings", "rounded toy-like proportions"]
speed_levels_q8 = [320, 384, 448, 512, 576, 640]
hitbox = { w = 6, h = 6 }

[model]
size_class = "M"
canvas = [52, 58]
length_px = 56
tris = 220
orient = { roll = 9, roll_max_deg = 60 }
ludo = { generator = "hero", rig = false, texture = "simple" }

[[model.materials]]
name = "hull"
source = "yellow armour"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "booms"
source = "orange booms and wing panels"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "canopy"
source = "light blue canopy glass"
ramp = ["dark", "light", "hot"]
floor = 1
emissive = false

[[model.materials]]
name = "glow"
source = "cyan exhaust glow"
ramp = ["hot"]
floor = 0
emissive = true
+++

# CARACARA

Prototype 11, the airframe QUILL flies in stage 3. Hull colors: that stage's `dark` and
`light` (`02-ladder.md`).

## Feel

Slow and wide, wider shot spread, a bigger hitbox: built for the canyon and the trains.

## Design note

The first wide ship. The open middle between the booms is a silhouette no enemy has.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): canvas height raised to length_px + 2 (rev 1 canvases were shorter than the ships; a number error, no design change)
- rev 3 (2026-10-31): hull floors 1 -> 2 (shadow side stops at solid dark; art-direction review of the TALON pilot)
