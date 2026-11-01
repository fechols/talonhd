+++
id = "hd.ship.lanner"
kind = "hd_ship"
name = "LANNER"
prototype = 15
stage = "stage.s7"
status = "approved"
revision = 3
crystal = "spine"
identity = "a storm interceptor in cloud grey and purple, a W-shaped wing, two tall blade fins, a glowing crystal spine down its back"
silhouette = "W-shaped wing planform, slim fuselage, two tall swept blade fins, a ridge of crystal along the spine, twin nozzles"
materials = "cloud grey armour, purple panels and fins, yellow-glowing crystal spine, light blue canopy glass, yellow exhaust glow"
feel = "the fastest bank on the ladder and a thin hitbox: built to thread lightning"
forbidden = ["real aircraft silhouette", "national markings or flags", "text or numbers on the airframe", "visible pilot", "missiles under the wings", "rounded toy-like proportions"]
speed_levels_q8 = [448, 512, 576, 640, 704, 768]
hitbox = { w = 3, h = 5 }

[model]
size_class = "M"
canvas = [44, 62]
length_px = 60
tris = 240
orient = { roll = 9, roll_max_deg = 60 }
ludo = { generator = "hero", rig = false, texture = "simple" }

[[model.materials]]
name = "hull"
source = "cloud grey armour"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "panels"
source = "purple panels and fins"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "crystal"
source = "glowing crystal spine"
ramp = ["dark", "hot"]
floor = 0
emissive = false

[[model.materials]]
name = "canopy"
source = "light blue canopy glass"
ramp = ["dark", "light", "hot"]
floor = 1
emissive = false

[[model.materials]]
name = "glow"
source = "exhaust glow"
ramp = ["hot"]
floor = 0
emissive = true
+++

# LANNER

Prototype 15, the airframe QUILL flies in stage 7. Hull colors: that stage's `dark` and
`light` (`02-ladder.md`).

## Feel

The fastest bank on the ladder and a thin hitbox: built to thread lightning.

## Design note

In S7 the hot role is yellow, so the crystal spine burns like the storm.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): canvas height raised to length_px + 2 (rev 1 canvases were shorter than the ships; a number error, no design change)
- rev 3 (2026-10-31): hull floors 1 -> 2 (shadow side stops at solid dark; art-direction review of the TALON pilot)
