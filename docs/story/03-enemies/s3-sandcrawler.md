+++
id = "enemy.s3.sandcrawler"
kind = "enemy"
name = "SANDCRAWLER"
stage = "stage.s3"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [5, 6]
angles = 8
anchor = "center"
identity = "a sand-brown Accord desert tank seen from directly above: broad treads, a low hull with a dozer blade, a three-barrel crystal launcher on the back"
silhouette = "wide low hull, front dozer blade, broad treads, three short barrels at the rear"
proportions = "as wide as long; blade the full width; barrels a third of the hull length"
materials = "brown and orange desert paint, dark grey treads and blade edge, purple crystal launcher"
acting = "bulges up out of the dune, shrugs off sand, ploughs forward, swings round, fires a fan"
forbidden = ["people or crew", "national markings", "eyes or a face", "a giant insect or worm", "wheels without treads"]
hurtbox = { x = -16, y = -18, w = 32, h = 36 }
domain = "ground"
hp = 10
contact_damage = 0
score = 600
weak_to = "burst"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "tank"
params = { emerge_ticks = 40, tell_ticks = 60, speed_q8 = 128, path = "authored", path_step_px = 2, stop_every_px = 80, pivot_ticks_per_angle = 5 }
attack = { kind = "spread", count = 3, spread_deg = 40, speed_q8 = 384, telegraph_ticks = 18, cooldown_ticks = 120 }
placement = "S3 open desert dunes; emerges from authored dune patches; at most 2 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 6 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 9
note = "brown desert paint"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey treads and blade"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal launcher"
required = true

[[colors]]
role = "sand"
c64 = 8
note = "orange sand spill and paint highlights"
required = false

[[animations]]
id = "tell"
frames = 2
ticks_per_frame = 15
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "a dune ripples and bulges upward, sand trickling down its sides"

[[animations]]
id = "emerge"
frames = 4
ticks_per_frame = 10
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "a wide tank pushes up out of the dune blade first, sand pouring off its hull"

[[animations]]
id = "pivot"
frames = 8
ticks_per_frame = 5
loop = false
model = "offline"
motion = "8 hull headings rotated from the master, composited per path sample"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "a burnt-out hull half-buried in sand, launcher shattered, treads split"

[ludo]
sprite_prompt = "a sand-brown Accord desert tank seen from directly above: broad treads, a low hull with a dozer blade, a three-barrel crystal launcher on the back, facing straight down, top-down view"
+++

# SANDCRAWLER

## Concept

Desert engineering tanks the Lattice parks under the dunes as ambushers. The SANDCRAWLER teaches **reading
the ground before it acts**: a dune that ripples is a tank about to surface. Buried, it cannot be hurt;
surfacing, it is slow and exposed. The player who watches the sand gets a free kill; the one who does not
gets a 3-way fan from below their ship.

## Behaviour pattern

1. Buried: invisible and immune. When its dune crosses line 24, the dune patch ripples (`tell`, 60 ticks).
2. Emerges over 40 ticks (immune until the last frame), then drives its authored path at 0.5 px/tick.
3. Every 80 px of path: stops, pivots toward the player through 8 headings (5 ticks each), 18-tick
   launcher glow, fires a 3-way spread (20 degrees apart) at 1.5 px/tick. Cooldown 120.
4. Holds fire once the player is above it.

## Placement rules

- S3 open desert only, emerging from authored dune patches; at most 2 per screen.
- Bakes as CRAWLER (5×6 rect plus a one-cell straddle margin), plus the dune tell and emerge frames
  over each dune's exact sand.
- Never emerge directly below a VULTURE's hold position: the player must be free to move to it.

## Readability

The widest tank in the game, with a dozer blade: it reads as "pushes through sand". The ripple tell uses
only background inks, so it is subtle by design but always the same shape.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
