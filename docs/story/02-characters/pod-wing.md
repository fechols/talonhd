+++
id = "pod.wing"
kind = "ship"
name = "WING POD"
faction = "accord"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
expanded = false
angles = 1
anchor = "center"
identity = "a small white armoured gun pod seen from directly above, teardrop shaped point up, a short gun barrel at the tip, two stubby side fins"
silhouette = "teardrop, point up, short barrel, two stubby fins, round engine dot at the base"
proportions = "8 art px wide (16 display px) by 16 lines; the barrel is a quarter of the length"
materials = "painted metal shell with one panel line, dark nozzle, a dark gun barrel"
acting = "loyal and eager; trails the TALON with a slight lag, bobs once when it settles"
forbidden = ["cockpit or canopy", "cyan", "wings", "red or orange paint", "markings", "side view"]
hurtbox = { x = -4, y = -6, w = 8, h = 12 }
private_inks = [1]
blocks_bullets = true

[[colors]]
role = "primary"
c64 = 1
note = "white shell (private ink)"
required = true

[[colors]]
role = "shared_light"
c64 = 14
note = "shared pair light tone (proposed light blue): fins and panel line"
required = false

[[colors]]
role = "shared_dark"
c64 = 11
note = "shared pair dark tone: outline, barrel, nozzle"
required = false

[[animations]]
id = "idle"
frames = 2
ticks_per_frame = 3
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "hovering in place, a tiny engine flame at the base flickers short and long"

[[animations]]
id = "deploy"
frames = 3
ticks_per_frame = 3
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the pod unfolds from a closed capsule, its two fins swing out and the barrel extends forward"

[ludo]
concept_prompt = "A small armoured gun pod for a retro-future fighter jet, seen from directly above, point toward the top of the image. Teardrop-shaped white shell with one panel line, a short dark gun barrel at the point, two stubby mid grey fins, a round dark engine nozzle at the base with a tiny flame. Chunky 1980s toy-model look. No markings, no cockpit."
sprite_prompt = "the gun pod centred, point straight up, fins level"
+++

# WING POD

## Role

WINGS 2 adds one pod on each side of the TALON; WINGS 3 adds a second pair. Pods are uncrewed gun
platforms slaved to the TALON: they mirror the current main weapon (TWIN, BURST, BEAM or SEEKER, never
FLANK) at the reduced pod values in `docs/design.md` 6.

## Formation

Pod slots are fixed offsets from the TALON's anchor (display px, NTSC):

| Pod | Offset | Lag |
|---|---|---|
| inner left / right (WINGS 2) | (−34, +6) / (+34, +6) | follows the slot at 1/2 of the remaining distance per tick |
| outer left / right (WINGS 3) | (−50, +30) / (+50, +30) | 1/4 per tick, so the outer pair swings wide in turns |

The inner pair sits on the TALON's own lines (2 + 2 = 4 channels, plus 2 bullet buckets = the
player-band budget of 6). The outer pair trails 30 lines lower and overlaps the TALON's bottom row, so
on every line it shares with the player band the outer pods are drawn as composites inside a bucket
channel (see `08-vfx.md`, per-line rules); below the band they are ordinary S sprites. They keep their
hurtboxes either way. This leaves 2 channels on the player band for a boss or air enemies.

Pods are clamped to the playfield; at the screen edge a pod slides inward along its row rather than
leaving the screen.

## Damage

- Pods **block** enemy bullets: a bullet that touches a pod is absorbed.
- The absorbing pod and its mirror partner are destroyed together ("pods die as a pair"), each with a
  `vfx.explosion_s`, and WINGS drops one level (3 → 2 or 2 → 1). The TALON itself is not hurt.
- Contact with an air enemy body counts the same as a bullet; ground targets do not touch pods.
- Pods are invulnerable during the TALON's arming invulnerability and while they are escorts.

## Colour notes

Private ink white (1), the TALON's top-row ink, so pods read as "part of the TALON" at a glance; no
cyan, so the player's own ship is the only white-and-cyan object.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
