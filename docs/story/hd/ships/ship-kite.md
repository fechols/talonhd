+++
id = "hd.ship.kite"
kind = "hd_ship"
name = "KITE"
prototype = 14
stage = "stage.s6"
status = "approved"
revision = 3
crystal = "edges"
identity = "a long-winged flying-wing fighter in two greens, cyan crystal set into its leading edges, no tail, a small bubble of canopy glass"
silhouette = "wide thin flying wing, sawtooth trailing edge, cyan crystal strips along both leading edges, central hump, no tail"
materials = "light green composite skin, green panels, cyan crystal leading edges, light blue canopy glass, cyan exhaust glow"
feel = "the first hybrid: glides, drifts a little in turns, sparks pulled in from further away"
forbidden = ["real aircraft silhouette", "national markings or flags", "text or numbers on the airframe", "visible pilot", "missiles under the wings", "rounded toy-like proportions"]
speed_levels_q8 = [416, 480, 544, 608, 672, 736]
hitbox = { w = 4, h = 4 }

[model]
size_class = "M"
canvas = [60, 46]
length_px = 44
tris = 200
orient = { roll = 9, roll_max_deg = 60 }
ludo = { generator = "hero", rig = false, texture = "simple" }

[[model.materials]]
name = "hull"
source = "light green composite skin"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "panels"
source = "green panels"
ramp = ["void", "dark", "light"]
floor = 2
emissive = false

[[model.materials]]
name = "crystal"
source = "cyan crystal leading edges"
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

# KITE

Prototype 14, the airframe QUILL flies in stage 6. Hull colors: that stage's `dark` and
`light` (`02-ladder.md`).

## Feel

The first hybrid: glides, drifts a little in turns, sparks pulled in from further away.

## Design note

After the twist, the crystal of the Lattice goes into the wings: the player ship starts to share the enemy color.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): canvas height raised to length_px + 2 (rev 1 canvases were shorter than the ships; a number error, no design change)
- rev 3 (2026-10-31): hull floors 1 -> 2 (shadow side stops at solid dark; art-direction review of the TALON pilot)
