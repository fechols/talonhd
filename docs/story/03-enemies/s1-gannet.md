+++
id = "enemy.s1.gannet"
kind = "enemy"
name = "GANNET"
stage = "stage.s1"
faction = "captured"
status = "approved"
revision = 3
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
expanded = false
angles = 8
anchor = "center"
identity = "a Lattice-overgrown retro-future attack drone seen from directly above: gull-shaped wings like a diving seabird, light red armour, a light blue crystal eye"
silhouette = "a broad flattened M of gull wings kinked back at the elbows, a short blunt pod body, a small fan tail, two round bomb pods under the wing elbows, crystal spikes on the wing roots"
proportions = "wingspan one and a half times the length; the body pod a third of the span long and fat; the wing elbows at a third of the span from the body; tail half the body width"
materials = "light red painted armour plates, dark grey seams, bomb pods and outline, light blue faceted crystal growing from the wing roots and forming a single round sensor eye where a cockpit would be"
acting = "comes in steep and heavy, levels out, drops its load, pulls up and climbs away"
forbidden = ["real aircraft silhouette", "real-world bomber or fighter shape", "WWII-era features: radial engines, propellers, framed glazing, a cockpit canopy", "national markings or roundels", "a pilot visible", "eyes or a face", "a straight cross-shaped planform"]
hurtbox = { x = -18, y = -16, w = 36, h = 32 }
domain = "air"
hp = 6
contact_damage = 1
score = 500
weak_to = "twin"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "diver"
params = { dive_speed_q8 = 512, release_line = 96, pullup_ticks = 24, climb_speed_q8 = 384, track = "player_x_at_entry" }
attack = { kind = "fixed", count = 2, spread_deg = 20, speed_q8 = 224, telegraph_ticks = 16, cooldown_ticks = 0 }
placement = "enters from the top edge above the player's column; releases at line 96 and climbs back out the top; one GANNET at a time"
sprite_budget = { channels_on_line = 2, bullets_max = 2 }
death_vfx = "vfx.explosion_m"

[[colors]]
role = "primary"
c64 = 10
note = "light red airframe (private ink)"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal growth and the sensor eye (shared pair, proposed)"
required = false

[[colors]]
role = "panel"
c64 = 11
note = "dark grey outline, seams and bomb pods (shared pair, proposed)"
required = false

[[animations]]
id = "dive"
frames = 2
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "steady steep descent, the wingtips flex slightly and the crystal eye pulses"

[[animations]]
id = "release"
frames = 2
ticks_per_frame = 8
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the two bomb pods unclamp and drop away from the wing elbows and the drone lifts slightly"

[[animations]]
id = "heading"
frames = 8
ticks_per_frame = 3
loop = false
model = "offline"
motion = "8 headings rotated from the master for the pull-up"

[ludo]
concept_prompt = "A Lattice-overgrown retro-future attack drone seen from directly above, nose pointing to the top of the image, plainly a machine and clearly not a real aircraft or a living bird. Its wings make one broad flattened M, like a gull seen from above: from a short blunt pod body each wing angles out and forward to a raised elbow a third of the way out, then sweeps out and back to a blunt tip, so the wingspan is one and a half times the length. A small round ducted fan at the tail instead of a tail fin, and two round dark grey bomb pods clamped under the wing elbows. No cockpit, no canopy, no propellers, no engines on the wings. Light red armour in large flat plates with dark grey seams and a thick dark grey outline. Light blue faceted crystal grows in clusters from both wing roots and forms one round sensor eye on the nose where a cockpit would be. Chunky 1980s toy-model proportions."
sprite_prompt = "a Lattice-overgrown attack drone seen from directly above, nose pointing straight up, wings spread level in a broad gull-wing M like a diving seabird, light red armour with a thick dark grey outline, two round bomb pods under the wing elbows, light blue crystal on the wing roots and a single round crystal sensor eye on the nose, clearly not a real-world aircraft"
+++

# GANNET

## Concept

The carrier wing's attack drones, taken on the flight deck before the war started and now grown
through with Lattice crystal: a light red gull-winged drone (rev 2: P0's bomber read as a real
World-War-era plane, so it is now plainly a retro-future drone with no cockpit and no propellers). A GANNET comes
straight for the player's column, drops two slow bombs that fan apart, and climbs away. It teaches
**the space under a big plane is dangerous, the space beside it is safe**, and that a medium target
takes a real burst of TWIN (six shots, about 0.6 s) to bring down, so you start shooting as it enters.

## Behaviour pattern

1. Enters from the top edge, centred on the player's X at the moment of spawn, diving at 2 px/tick.
2. At screen line 96 it telegraphs for 16 ticks (racks open) and releases 2 bombs: slow shots at
   0.875 px/tick angled 10 degrees either side of straight down. Bombs are bullets (bucket layer) drawn
   larger than normal shots, and they are shootable (1 HP each, 50 points).
3. Pulls up over 24 ticks through the headings and climbs out the top at 1.5 px/tick. It never enters
   the lower 80 lines.

## Placement rules

- One GANNET on screen at a time; never with a SKIFF lane or KESTREL pair on the same lines.
- Release line 96 keeps the M sprite out of the player band, so the band budget is untouched.
- Loop 2+: releases 3 bombs (spread 20 degrees) and the bombs split into 2 on reaching line 170.

## Concept step (rev 3)

P1's direct master (no concept) kept drawing stubby red planes and never the gull-wing M. Like the
TALON, the GANNET now goes concept (`concept_prompt`, `image create --image-type asset`) and then an
`image edit` master of the best concept with the still prompt.

## Readability

A broad red gull-wing M, chunkier than anything else in the sky. When the racks open, the bombs that follow
are the largest bullets in the game, so they read as "big and slow, shoot or step around".

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): P2 pilot fixes
