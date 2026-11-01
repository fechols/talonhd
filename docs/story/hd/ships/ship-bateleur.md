+++
id = "hd.ship.bateleur"
kind = "hd_ship"
name = "BATELEUR"
prototype = 12
stage = "stage.s4"
status = "approved"
revision = 3
crystal = "none"
identity = "a broad arrowhead strike fighter in safety yellow and steel grey, two engine pods slung beside the fuselage, blade wingtips angled down"
silhouette = "wide arrowhead planform, two long engine pods alongside the fuselage, blade wingtips, a short V of tail blades, a flat armoured spine"
materials = "safety yellow armour, steel grey engine pods and spine, dark seams, light blue canopy glass, cyan exhaust glow"
feel = "balanced and heavy-hitting: the TALON grown up, the best all-rounder on the continent"
forbidden = ["real aircraft silhouette", "national markings or flags", "text or numbers on the airframe", "visible pilot", "missiles under the wings", "rounded toy-like proportions"]
speed_levels_q8 = [384, 448, 512, 576, 640, 704]
hitbox = { w = 5, h = 5 }

[model]
size_class = "M"
canvas = [44, 64]
length_px = 62
tris = 240
orient = { roll = 9, roll_max_deg = 60 }
ludo = { generator = "hero", rig = false, texture = "simple" }

[[model.materials]]
name = "hull"
source = "safety yellow armour"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "pods"
source = "steel grey engine pods and spine"
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

# BATELEUR

Prototype 12, the airframe QUILL flies in stage 4. Hull colors: that stage's `dark` and
`light` (`02-ladder.md`).

## Feel

Balanced and heavy-hitting: the TALON grown up, the best all-rounder on the continent.

## Design note

Industrial and blunt, like the Sluiceworks it flies over.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): canvas height raised to length_px + 2 (rev 1 canvases were shorter than the ships; a number error, no design change)
- rev 3 (2026-10-31): hull floors 1 -> 2 (shadow side stops at solid dark; art-direction review of the TALON pilot)
