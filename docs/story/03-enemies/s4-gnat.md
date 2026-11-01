+++
id = "enemy.s4.gnat"
kind = "enemy"
name = "GNAT"
stage = "stage.s4"
faction = "lattice"
status = "approved"
revision = 1
layer = "bucket"
sprite_class = "S"
cells = [1, 1]
expanded = false
angles = 1
anchor = "center"
identity = "a tiny purple crystal mote drone seen from directly above: a four-point star of facets around a bright light blue centre"
silhouette = "four-point star, bright centre dot"
proportions = "4x4 art px each; star points equal length"
materials = "purple faceted crystal, light blue centre"
acting = "a swarm of twelve that jitters and flows after the TALON like a shoal, always slightly late"
forbidden = ["insect legs or wings", "eyes or a face", "organic shapes", "smoke or cloud shapes"]
hurtbox = { x = -3, y = -3, w = 6, h = 6 }
domain = "air"
hp = 1
contact_damage = 1
score = 50
weak_to = "burst"
resists = "none"
drops = { spark = 0, swarm_clear = 1 }

[behavior]
archetype = "drifter"
params = { swarm = 12, follow_lag_ticks = 40, max_speed_q8 = 320, jitter_px = 3, spread_px = 40, lifetime_ticks = 720, swarm_bonus = 2000 }
attack = { kind = "ram", count = 12, spread_deg = 0, speed_q8 = 320, telegraph_ticks = 0, cooldown_ticks = 0 }
placement = "S4 pipe yards and pump hall; one swarm at a time; enters from the top edge as a tight ball"
sprite_budget = { channels_on_line = 2, bullets_max = 0 }
death_vfx = "vfx.spark_pop"

[[colors]]
role = "crystal"
c64 = 4
note = "purple facets (the GNAT bucket's private ink; enemy bullet buckets use their own)"
required = true

[[colors]]
role = "centre"
c64 = 14
note = "light blue centre (shared pair, proposed)"
required = false

[[animations]]
id = "twinkle"
frames = 2
ticks_per_frame = 4
loop = true
model = "offline"
motion = "two hand-drawn 4x4 masks, star and plus, alternated per mote with a per-mote phase"

[ludo]
sprite_prompt = "a tiny purple crystal mote drone seen from directly above: a four-point star of facets around a bright light blue centre, top-down view, many copies arranged in a loose swarm"
+++

# GNAT

## Concept

The smallest thing the Lattice grows: crystal motes that hunt in a swarm of twelve. GNATs are the showcase
for the **bucket layer**: twelve enemies drawn by the CPU into one or two shared 24×21 sprites, so a whole
swarm costs what one WASP costs. They teach **area weapons**: a swarm laughs at a narrow stream and dies
to BURST's wide flak, FLANK's side guns, or a pod spread.

## Behaviour pattern

- The swarm follows a target point that chases the player's position from 40 ticks ago, capped at
  1.25 px/tick. Each mote keeps a fixed offset inside a 40-px blob plus ±3 px jitter, so the swarm
  breathes and flows like a shoal.
- No bullets. Contact with any mote kills (it is a ram swarm), but the motes are 6×6 px hurtboxes and
  the blob has holes; a calm player can pass through the edge.
- After 720 ticks (12 s) the swarm streams off the top.
- Killing all 12 pays a 2,000 bonus and drops a spark where the last mote died.

## Placement rules

- S4 pipe yards and pump hall; one swarm at a time.
- **Budget:** the swarm's bounding box is at most 48×42 px, so it fits in 2×2 bucket windows; at most
  2 channels on any line. In the player band that makes ship 2 + pods 4 + swarm 2 = 8, so while a swarm
  is in the band no other enemy sprite or enemy bullet bucket may be scheduled there (S4 waves are
  authored around this). Mux priority keeps the player and the swarm; anything else flickers.
- `sprite_class`/`cells` describe one bucket window, not one GNAT.
- Death VFX is a 1-frame bucket pop (`vfx.spark_pop`), not a sprite explosion.

## Readability

Twelve purple twinkles moving as one: a shoal, not twelve enemies. The swarm's shape tells you where the
holes are.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
