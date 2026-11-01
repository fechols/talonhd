+++
id = "hd.ship.peregrine"
kind = "hd_ship"
name = "PEREGRINE"
prototype = 10
stage = "stage.s2"
status = "approved"
revision = 3
crystal = "none"
identity = "a slim long-range interceptor in cream and terracotta, a needle nose, cranked delta wings, a long dorsal spine, one big round exhaust"
silhouette = "needle nose, cranked delta with a sharp kink at half span, dorsal spine ridge, single large round nozzle, small wingtip fins"
materials = "cream ceramic armour, terracotta wing panels, dark seams, light blue canopy glass, cyan exhaust glow"
feel = "faster in a straight line than the TALON, slower to bank: a lance"
forbidden = ["real aircraft silhouette", "national markings or flags", "text or numbers on the airframe", "visible pilot", "missiles under the wings", "rounded toy-like proportions"]
speed_levels_q8 = [448, 512, 576, 640, 704, 768]
hitbox = { w = 4, h = 4 }

[model]
size_class = "M"
canvas = [32, 72]
length_px = 70
tris = 180
orient = { roll = 9, roll_max_deg = 60 }
ludo = { generator = "hero", rig = false, texture = "simple" }

[[model.materials]]
name = "hull"
source = "cream ceramic armour"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "panels"
source = "terracotta wing panels"
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

# PEREGRINE

Prototype 10, the airframe QUILL flies in stage 2. Hull colors: that stage's `dark` and
`light` (`02-ladder.md`).

## Feel

Faster in a straight line than the TALON, slower to bank: a lance.

## Design note

Long and narrow where the TALON is broad: the first new shape tells the player the ladder is real.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): canvas height raised to length_px + 2 (rev 1 canvases were shorter than the ships; a number error, no design change)
- rev 3 (2026-10-31): hull floors 1 -> 2 (shadow side stops at solid dark; art-direction review of the TALON pilot)
