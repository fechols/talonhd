+++
id = "enemy.s2.roof_hatch"
kind = "enemy"
name = "ROOF HATCH"
stage = "stage.s2"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [5, 4]
angles = 1
anchor = "center"
identity = "a red clay-tile village roof seen from directly above, a square hatch in its ridge where purple crystal glows through gaps in the tiles"
silhouette = "rectangular pitched roof, central ridge line, square hatch, cracked tiles"
proportions = "roof wider than deep; hatch a third of the roof width, centred on the ridge"
materials = "orange-red clay tiles, brown ridge beams, purple and light blue crystal light"
acting = "tiles lift and rattle, the hatch slides open, WASPs rise; or the roof blows apart when struck hard"
forbidden = ["people", "religious symbols", "chimneys with smoke faces", "eyes or a face", "national flags"]
hurtbox = { x = -14, y = -12, w = 28, h = 24 }
domain = "ground"
hp = 6
contact_damage = 0
score = 800
weak_to = "burst"
resists = "twin"
drops = { spark = 0, bonus_medal = 1 }

[behavior]
archetype = "hatch"
params = { open_ticks = 24, launch = "enemy.common.wasp", launch_count = 2, launch_spacing_ticks = 20, cycles = 2, closed_ticks = 100, roof_hp_burst = 2, bonus_medal_score = 2000 }
attack = { kind = "launch", count = 2, spread_deg = 0, speed_q8 = 512, telegraph_ticks = 24, cooldown_ticks = 100 }
placement = "S2 hill village; on the ridge of authored roof tiles; at most 3 per screen, one opening at a time"
sprite_budget = { channels_on_line = 0, bullets_max = 0 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 8
note = "orange clay tiles"
required = true

[[colors]]
role = "secondary"
c64 = 9
note = "brown ridge beams"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple light through the gaps"
required = true

[[colors]]
role = "tile_highlight"
c64 = 10
note = "light red sunlit tile edges where the cell allows"
required = false

[[animations]]
id = "open"
frames = 4
ticks_per_frame = 6
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "tiles rattle and lift, then a square panel in the ridge slides aside to show glowing crystal below"

[[animations]]
id = "blown"
frames = 3
ticks_per_frame = 5
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "the roof bursts outward, tiles flying off the frame, leaving an open rafter grid with a gold bonus medal resting inside"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "a roofless house with scorched rafters and dark cracked crystal in the room below"

[ludo]
sprite_prompt = "a red clay-tile village roof seen from directly above, a square hatch in its ridge where purple crystal glows through gaps in the tiles, top-down view"
+++

# ROOF HATCH

## Concept

The village is full of drone nests the Lattice has grown under people's roofs. A ROOF HATCH works like
the common HATCH (`variant_of = "enemy.common.hatch"` in prose), with a twist that teaches **weapon
choice for points**: BURST, the heaviest short-range weapon, can blow the whole roof off in two hits
**before** it opens. A blown roof reveals a 2,000-point bonus medal and the nest dies without launching.

## Behaviour pattern

1. Closed; tiles glow faintly. While closed, only BURST affects it: 2 BURST hits blow the roof
   (`blown` patch), destroy the nest and expose the bonus medal (a pickup sprite, 2,000 points).
2. Otherwise, at line 48 it opens over 24 ticks and launches 2 WASPs (`drop_u`), 20 ticks apart; closes;
   100 ticks later repeats once.
3. While open it takes normal damage (TWIN ×0.5), 6 HP; destroyed open, it pays 800 and no medal.

## Placement rules

- S2 hill village only, on authored roofs. At most 3 per screen, one opening at a time.
- Roofs are placed in clusters of 2-3 so a BURST player can sweep a street; leave a gap row between
  clusters so WASP launches never overlap a PIKE row.
- Bakes: closed, 4 open frames, 3 blown frames, wreck, all over each roof's exact background.

## Readability

A red roof among red roofs, except this one glows purple through the cracks. The glow is the only
difference, and that is the point: the observant player finds the medals.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
