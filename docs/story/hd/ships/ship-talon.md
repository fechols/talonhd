+++
id = "hd.ship.talon"
kind = "hd_ship"
name = "TALON"
prototype = 9
stage = "stage.s1"
base = "ship.talon"
status = "approved"
revision = 3
crystal = "none"
identity = "a sleek predatory white retro-future interceptor, knife-edge nose, raptor-claw wingtips, narrow angular light blue canopy slit"
silhouette = "long arrowhead nose, narrow chiselled fuselage, wings swept steeply back hooking forward into claw tips, one raked tail blade, twin nozzles"
materials = "white armour plates, dark grey seams and leading edges, light blue canopy glass, cyan engine glow"
feel = "the baseline: precise, quick to bank, nothing special and nothing missing"
forbidden = ["real aircraft silhouette", "national markings or flags", "text or numbers on the airframe", "visible pilot", "missiles under the wings", "rounded toy-like proportions"]
speed_levels_q8 = [384, 448, 512, 576, 640, 704]
hitbox = { w = 4, h = 4 }

[model]
size_class = "M"
canvas = [36, 66]
length_px = 64
tris = 200
orient = { roll = 9, roll_max_deg = 60 }
ludo = { generator = "hero", rig = false, texture = "simple" }

[[model.materials]]
name = "hull"
source = "white armour"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "trim"
source = "dark grey seams and leading edges"
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
source = "cyan engine glow"
ramp = ["hot"]
floor = 0
emissive = true
+++

# TALON

Prototype 9, the airframe QUILL flies in stage 1. Hull colors: that stage's `dark` and
`light` (`02-ladder.md`).

## Feel

The baseline: precise, quick to bank, nothing special and nothing missing.

## Design note

The XF-9 from the base bible, unchanged in shape: the approved silhouette is the model brief. White over the blue sea of S1.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): canvas height raised to length_px + 2 (rev 1 canvases were shorter than the ships; a number error, no design change)
- rev 3 (2026-10-31): hull floors 1 -> 2 (shadow side stops at solid dark; art-direction review of the TALON pilot)
