+++
id = "hd.ship.augur"
kind = "hd_ship"
name = "AUGUR"
prototype = 13
stage = "stage.s5"
status = "approved"
revision = 3
crystal = "none"
identity = "an armoured siege fighter in stone and olive, a broad flying wedge, a raised sensor spine, four square engine nozzles across the rear edge"
silhouette = "flat broad wedge, raised sensor spine with a disc at its peak, four square nozzles in a row, short clipped wingtips"
materials = "light stone armour plates, olive panels, dark seams, light blue canopy glass, cyan exhaust glow"
feel = "the toughest airframe: one extra hit before a life is lost, slowest bank on the ladder"
forbidden = ["real aircraft silhouette", "national markings or flags", "text or numbers on the airframe", "visible pilot", "missiles under the wings", "rounded toy-like proportions"]
speed_levels_q8 = [352, 416, 480, 544, 608, 672]
hitbox = { w = 6, h = 6 }

[model]
size_class = "M"
canvas = [48, 60]
length_px = 58
tris = 220
orient = { roll = 9, roll_max_deg = 60 }
ludo = { generator = "hero", rig = false, texture = "simple" }

[[model.materials]]
name = "hull"
source = "light stone armour plates"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "panels"
source = "olive panels"
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

# AUGUR

Prototype 13, the airframe QUILL flies in stage 5. Hull colors: that stage's `dark` and
`light` (`02-ladder.md`).

## Feel

The toughest airframe: one extra hit before a life is lost, slowest bank on the ladder.

## Design note

The last Accord-only design: a brick with wings for breaking the Citadel.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): canvas height raised to length_px + 2 (rev 1 canvases were shorter than the ships; a number error, no design change)
- rev 3 (2026-10-31): hull floors 1 -> 2 (shadow side stops at solid dark; art-direction review of the TALON pilot)
