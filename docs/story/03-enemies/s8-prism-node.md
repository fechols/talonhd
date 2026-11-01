+++
id = "enemy.s8.prism_node"
kind = "enemy"
name = "PRISM NODE"
stage = "stage.s8"
faction = "lattice"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [4, 4]
angles = 1
anchor = "center"
identity = "a triangular crystal prism node on a girder seen from directly above: a light blue prism with purple facets, three beam emitters at its corners"
silhouette = "equilateral triangle, inner triangle facet, three corner emitters"
proportions = "triangle fills the patch; inner facet half the size; emitters a tenth of a side"
materials = "light blue and purple faceted crystal, dark grey girder below, white glints at the emitters"
acting = "charges slowly with light pooling at its corners, draws thin guide lines, then fires three lances at once"
forbidden = ["eyes or a face", "organic shapes", "religious or occult symbols", "metal hardware on the prism", "rounded blobs"]
hurtbox = { x = -14, y = -14, w = 28, h = 28 }
domain = "cable"
hp = 14
contact_damage = 0
score = 1000
weak_to = "twin"
resists = "beam"
drops = { spark = 1 }

[behavior]
archetype = "emplacement"
params = { beams = 3, beam_aim = "corner_rotating", rotate_deg_per_volley = 20, guide_ticks = 40, beam_ticks = 24, beam_width_px = 4, fire_range_lines = 200 }
attack = { kind = "beam", count = 3, spread_deg = 120, speed_q8 = 0, telegraph_ticks = 40, cooldown_ticks = 160 }
placement = "S8 orbit (cable anchor) and Crown approach (crystal girders); at most 2 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 0 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 14
note = "light blue prism"
required = true

[[colors]]
role = "crystal_dark"
c64 = 4
note = "purple facets"
required = true

[[colors]]
role = "girder"
c64 = 11
note = "dark grey girder"
required = false

[[colors]]
role = "glint"
c64 = 1
note = "white glints at the emitters (non-required)"
required = false

[[animations]]
id = "charge"
frames = 4
ticks_per_frame = 10
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "light pools and brightens at the three corners of the crystal prism, facets flickering"

[[animations]]
id = "rotate"
frames = 6
ticks_per_frame = 4
loop = true
model = "offline"
motion = "6 rotation steps of 20 degrees (120-degree symmetry) rotated from the master"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the prism cracked into three dull pieces lying on the girder"

[ludo]
sprite_prompt = "a triangular crystal prism node on a girder seen from directly above: a light blue prism with purple facets, three beam emitters at its corners, top-down view"
+++

# PRISM NODE

## Concept

The CROWN's relays, in the final climb. A PRISM NODE does not shoot bullets; it fires **three lances of
light** from its corners after drawing thin guide lines where they will go. It teaches the final-stage
grammar of **telegraphed lines**, which the CROWN's facets use at scale. Its own crystal drinks BEAM
(×0.5); TWIN shatters it. Each one drops a spark for the last fight.

## Behaviour pattern

1. Within 200 lines above the player. Charge: 40 ticks; three thin guide lines (1 px, flickering, drawn by
   the bitmap renderer as a cell-row overlay restored afterwards) show the lance directions.
2. Fire: three 4-px-wide lances along the guide lines for 24 ticks. Contact with a lance kills an
   unshielded TALON. Lances end at the screen edge.
3. The prism rotates 20 degrees per volley (6 baked frames, 120-degree symmetry), so the safe wedges
   between lances turn. Cooldown 160 ticks.
4. Destroyed during the charge: no volley. Drops 1 spark.

## Placement rules

- S8 orbit (on the cable anchor) and Crown approach (on crystal girders); at most 2 per screen, placed
  so their lance sets leave at least one 48-px safe wedge at all rotations.
- Lances are not bullets: they are drawn into the bitmap back buffer as line overlays and cost no
  sprites; the M1 spike must confirm the overlay restore cost.

## Readability

A triangle with glowing corners: three corners, three lances. The guide lines are the clearest warning
in the game, and they teach the player to trust them before the CROWN.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
