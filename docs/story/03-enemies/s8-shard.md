+++
id = "enemy.s8.shard"
kind = "enemy"
name = "SHARD"
stage = "stage.s8"
faction = "lattice"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
expanded = false
angles = 1
anchor = "center"
identity = "a sharp crystal octahedron drone seen from directly above: a purple diamond of facets crossed by light blue fracture lines"
silhouette = "diamond, inner cross lines, four sharp points"
proportions = "as long as wide; fracture lines run point to point"
materials = "purple faceted crystal, light blue fracture glow, dark grey edges"
acting = "turns slowly as it descends; when destroyed it bursts into two needles that hurl themselves at the TALON"
forbidden = ["eyes or a face", "organic shapes", "rounded blobs", "gems with jewellery settings", "yellow"]
hurtbox = { x = -16, y = -16, w = 32, h = 32 }
domain = "air"
hp = 4
contact_damage = 1
score = 400
weak_to = "seeker"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "splitter"
params = { descend_q8 = 224, weave_px = 24, split_on = "death", children = 2, child_class = "S", child_hp = 1, child_score = 100, child_telegraph_ticks = 16, child_speed_q8 = 768, child_aim = "player_at_launch" }
attack = { kind = "ram", count = 2, spread_deg = 0, speed_q8 = 768, telegraph_ticks = 16, cooldown_ticks = 0 }
placement = "S8 all sections; drifts down from the top; at most 3 parents on screen; parents never below line 112"
sprite_budget = { channels_on_line = 2, bullets_max = 0 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 4
note = "purple facets (private ink, parent and needles)"
required = true

[[colors]]
role = "fracture"
c64 = 14
note = "light blue fracture glow (shared pair, proposed)"
required = false

[[colors]]
role = "edge"
c64 = 11
note = "dark grey edges (shared pair, proposed)"
required = false

[[animations]]
id = "turn"
frames = 4
ticks_per_frame = 8
loop = true
model = "offline"
motion = "4 rotation steps of the diamond master (90-degree symmetry) rotated by our tools"

[[animations]]
id = "needle"
frames = 2
ticks_per_frame = 4
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "a thin purple crystal needle trembling and glowing at its tip, about to launch"

[ludo]
sprite_prompt = "a sharp crystal octahedron drone seen from directly above: a purple diamond of facets crossed by light blue fracture lines, top-down view"
+++

# SHARD

## Concept

At the edge of space the Lattice grows its drones as raw crystal. A SHARD is MOTH's lesson turned around:
MOTH splits **when hit** into harmless drifters; SHARD splits **when killed**, into two needles that aim
themselves at the TALON. It teaches **kill range**: destroy a SHARD far above you and the needles have
time to be dodged; destroy it close and they are on you at once. SEEKER darts, one per target, kill it
high and then send fresh darts after each needle.

## Behaviour pattern

- Descends at 0.875 px/tick weaving ±24 px, turning slowly.
- On death: 2 S needles appear at its left and right points, tremble for 16 ticks (the telegraph), then
  launch at 3 px/tick toward the player's position at launch, in straight lines. 1 HP each, 100 points.
- A needle that misses leaves the screen.

## Placement rules

- All S8 sections; at most 3 parents and 4 needles alive.
- Parents are kept above line 112 so the split happens above the player band; needles are S (one channel
  each) and cross the band one at a time in most cases.
- S8 also uses **crystal-plated variants** of WASP, LANCER and KESTREL (see their files) alongside SHARDs.

## Readability

A purple diamond with a glowing cross drawn on it: the cross is where it breaks, and the two side points
are where the needles come from.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
