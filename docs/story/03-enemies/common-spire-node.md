+++
id = "enemy.common.spire_node"
kind = "enemy"
name = "SPIRE NODE"
stage = "common"
faction = "lattice"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [3, 5]
angles = 1
anchor = "base"
identity = "a tall hexagonal crystal pylon seen from directly above: a light blue six-sided spire on a purple faceted base, a tether socket on each side"
silhouette = "hexagon spire, wider faceted base, two side sockets, long cast shadow"
proportions = "base one and a half times the spire width; the shadow runs twice the spire length to the lower right"
materials = "light blue and purple faceted crystal, dark grey cast shadow, white glints"
acting = "stands rigid, its tether humming; when a partner breaks, it flares and spits a ring"
forbidden = ["organic or plant shapes", "eyes or a face", "metal or human hardware", "rounded blobs", "religious shapes"]
hurtbox = { x = -10, y = -36, w = 20, h = 36 }
domain = "ground"
hp = 10
contact_damage = 0
score = 600
weak_to = "seeker"
resists = "none"
drops = { spark = 0, last_of_group = 1 }

[behavior]
archetype = "tether_node"
params = { group_size = 3, tether = "shot_wall", tether_axis = "horizontal", tether_max_px = 120, flare_ticks = 20 }
attack = { kind = "ring", count = 6, spread_deg = 60, speed_q8 = 320, telegraph_ticks = 20, cooldown_ticks = 0 }
placement = "groups of 2-4 at one ground line, tethered left-to-right across the screen; ground targets beyond the tether are shielded"
sprite_budget = { channels_on_line = 0, bullets_max = 6 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 14
note = "light blue spire"
required = true

[[colors]]
role = "crystal_dark"
c64 = 4
note = "purple base facets"
required = true

[[colors]]
role = "shadow"
c64 = 11
note = "dark grey cast shadow"
required = false

[[colors]]
role = "glint"
c64 = 1
note = "white glints on the spire tip (non-required)"
required = false

[[animations]]
id = "hum"
frames = 4
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "light runs up the facets of the spire in a slow repeating pulse"

[[animations]]
id = "tether"
frames = 2
ticks_per_frame = 4
loop = true
model = "offline"
motion = "the tether band between nodes flickers between two baked cell rows"

[[animations]]
id = "flare"
frames = 2
ticks_per_frame = 10
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the spire flashes bright from base to tip, facets spreading open slightly"

[ludo]
sprite_prompt = "a tall hexagonal crystal pylon seen from directly above: a light blue six-sided spire on a purple faceted base, a tether socket on each side, top-down view"
+++

# SPIRE NODE

## Concept

The Lattice's own architecture: pylons that hold a humming band of energy between them. The band does
not hurt the TALON (it is ground-level); it **absorbs the TALON's shots**. Everything on the far side of
a tether is shielded until one node in the chain breaks. SPIRE NODEs teach **target priority**: break
the chain first, then the guns behind it. SEEKER darts, one per target, are the elegant answer: they fly
around the band's ends.

## Behaviour pattern

- A group of 2-4 nodes stands on one ground line, linked by tether bands (baked cell rows, two flicker
  frames, plus a "broken" frame with the band gone).
- The band is a shot wall: player shots that cross it from below are absorbed, except SEEKER darts,
  which arc over. Ground targets above the band (further up the scroll) take no damage while it stands.
- When any node dies, its tethers drop. Each surviving node in that group flares for 20 ticks and fires
  one 6-way ring at 1.25 px/tick. This happens once per death, so killing the middle node first triggers
  two rings; killing an end node first triggers one. The order is the puzzle.
- The last node of a group drops 1 spark.

## Placement rules

- Groups of 2-4 at one ground line, tether span at most 120 px per link.
- Never more than one group per screen; at most 6 ring bullets on screen from tether flares.
- Introduced in S5 BASTION's walls. S8 PRISM NODE is a separate cable-mounted relative.
- The stage builder bakes three states for each tether row: hum A, hum B and broken.

## Readability

A tall hexagon with a shadow stands up off the ground; the glowing band between two of them reads as a
fence. Light blue and purple are the Lattice colours, so nothing else in the stage looks like this.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
