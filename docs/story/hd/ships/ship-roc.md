+++
id = "hd.ship.roc"
kind = "hd_ship"
name = "ROC"
prototype = 16
stage = "stage.s8"
status = "approved"
revision = 3
crystal = "heart"
identity = "a large heavy white and cobalt fighter shaped like a bird of prey with spread wings, a faceted cyan crystal heart set into its spine"
silhouette = "broad spread wings with stepped feather-like trailing edges, a strong central body, a large faceted crystal at the centre, three nozzles"
materials = "white armour, cobalt blue panels, a cyan crystal heart, light blue canopy glass, cyan exhaust glow"
feel = "everything the Accord learned: fast, strong, the widest shot pattern; the biggest player ship"
forbidden = ["real aircraft silhouette", "national markings or flags", "text or numbers on the airframe", "visible pilot", "missiles under the wings", "rounded toy-like proportions"]
speed_levels_q8 = [416, 480, 544, 608, 672, 736]
hitbox = { w = 5, h = 5 }

[model]
size_class = "M"
canvas = [60, 70]
length_px = 68
tris = 300
orient = { roll = 9, roll_max_deg = 60 }
ludo = { generator = "hero", rig = false, texture = "simple" }

[[model.materials]]
name = "hull"
source = "white armour"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "panels"
source = "cobalt blue panels"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "crystal"
source = "cyan crystal heart"
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
source = "cyan exhaust glow"
ramp = ["hot"]
floor = 0
emissive = true
+++

# ROC

Prototype 16, the airframe QUILL flies in stage 8. Hull colors: that stage's `dark` and
`light` (`02-ladder.md`).

## Feel

Everything the Accord learned: fast, strong, the widest shot pattern; the biggest player ship.

## Design note

The bookend of the TALON: white over blue again, now with the crystal of the enemy at its heart.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): canvas height raised to length_px + 2 (rev 1 canvases were shorter than the ships; a number error, no design change)
- rev 3 (2026-10-31): hull floors 1 -> 2 (shadow side stops at solid dark; art-direction review of the TALON pilot)
