+++
id = "stage.s2"
kind = "level"
name = "TERRACES"
status = "approved"
revision = 3
slug = "terraces"
stage_index = 2
title = "HILLSIDE VINEYARDS"
theme = "warm afternoon gold over terraced hills: green vines, terracotta roofs, cream stone, violet crystal creeping uphill"
length_screens = 26
scroll_q8 = 128
target_minutes = 3
midboss = "midboss.s2.campanile"
boss = "boss.s2.astrolabe"
enemies = ["enemy.common.wasp", "enemy.common.kestrel", "enemy.s2.pike", "enemy.s2.roof_hatch", "enemy.s2.bell_turret", "enemy.s2.dome_mortar", "enemy.common.pillbox"]
checkpoints = [6, 14, 20]
music = { cue = "cue.stage_s2", mood = "warm golden afternoon over rolling hills, brisk and sunny, a plucked village bridge", bpm = 140, length_s = 150, loop = true }
background_ink = 9
card_line = "THEY TURNED A TELESCOPE INTO A GUN. CLOSE THE OBSERVATORY, QUILL."

[[sections]]
n = 1
name = "VINEYARDS"
screens = [0, 9]
description = "Terraced hillside vineyards seen from directly above in warm afternoon light: long parallel rows of green vines on brown soil following the contours, low dry-stone walls in cream and grey stepping down the slope, lines of tall dark cypress trees casting long shadows, a dirt track zig-zagging between terraces, small farmhouses with red terracotta tile roofs, a stone well, and scattered violet crystal shards sprouting at the row ends."
inks_intent = "9 brown soil, 5 green vines, 8 terracotta roofs, 15 cream stone"
beat = "PIKE drones race along the vine rows; KESTREL pairs cross and fire on the turn; farmhouse ROOF HATCHes open."
teaches = "rail runners follow the ground lines (read the rows to predict them); BURST opens roofs for bonus points"
enemies = ["enemy.s2.pike", "enemy.common.kestrel", "enemy.common.wasp", "enemy.s2.roof_hatch"]

[[sections]]
n = 2
name = "HILL VILLAGE"
screens = [9, 17]
description = "A hilltop village packed tight around narrow lanes: red and orange terracotta roofs of every size, cream plaster courtyards with potted lemon trees, a round fountain plaza paved in pale stone circles, striped market awnings, laundry lines between houses, a stone bell tower rising from the central square, and violet crystal crusting the gutters and cracking through the paving toward the tower."
inks_intent = "8 terracotta roofs, 15 cream plaster, 9 brown lanes, 4 purple crystal"
beat = "BELL TURRETs on the rooftops, ROOF HATCHes across the village; the bell tower CAMPANILE crystallises and spins."
teaches = "rotating 2x2 turret patches; shooting roofs open; the first midboss that fires rings"
enemies = ["enemy.s2.bell_turret", "enemy.s2.roof_hatch", "enemy.common.kestrel", "enemy.common.wasp"]

[[sections]]
n = 3
name = "OBSERVATORY"
screens = [17, 26]
description = "A ridge-top observatory campus in late golden light: white hemispherical domes of different sizes with their slit shutters, raked gravel paths in pale yellow curving between them, clipped green hedges, a long low stone instrument hall with a copper roof gone green, a sundial court, and thick violet and cyan crystal growth swallowing the paths and climbing the dome walls."
inks_intent = "15 light grey domes, 7 yellow gravel, 4 purple crystal, 14 light blue crystal"
beat = "DOME MORTARs lob slow arcing shots from the small domes while KESTRELs and PIKEs mix overhead; the great dome opens."
teaches = "lobbed shots land where you were: keep moving; BEAM against hard ground targets"
enemies = ["enemy.s2.dome_mortar", "enemy.s2.pike", "enemy.common.kestrel", "enemy.common.pillbox", "enemy.common.wasp"]

[[transitions]]
section = 2
set_piece = "The AQUEDUCT: a long stone aqueduct of tall arches crosses a deep shadowed ravine from the left edge to the right edge; a thin silver stream runs in the ravine floor far below, the water channel on top glints, and violet crystal grows along one span."
asset = "setpiece.s2.aqueduct"

[[waves]]
screen = 1
what = "6 WASPs in a slow S from the right; the leader drops a spark"

[[waves]]
screen = 2
what = "3 PIKEs race up the vine rows from the bottom edge, one per row, no fire; teach the rail line"

[[waves]]
screen = 3
what = "KESTREL pair crosses left to right and fires a 3-way on the turn; 2 ROOF HATCHes in a farmhouse"

[[waves]]
screen = 4
what = "2 PILLBOXes on a terrace wall; 6 WASPs swoop low across them from the left"

[[waves]]
screen = 5
what = "5 PIKEs down the rows from the top, staggered, each firing one aimed shot when level with the player"

[[waves]]
screen = 6
what = "Checkpoint 1. 2 KESTREL pairs crossing in opposite directions; 8 WASPs behind them, leader drops a spark"

[[waves]]
screen = 8
what = "AQUEDUCT set piece; 2 PILLBOXes on the arches, 4 PIKEs run along the aqueduct channel itself"

[[waves]]
screen = 10
what = "Village: 3 BELL TURRETs on rooftops, 4 ROOF HATCHes (BURST opens the roofs for 500 each)"

[[waves]]
screen = 12
what = "MIDBOSS CAMPANILE: the bell tower in the central square crystallises and spins out shard rings; the scroll stops for the fight"

[[waves]]
screen = 14
what = "Checkpoint 2. KESTREL pairs from both sides at once, 6 WASPs in a V; hidden spark in the fountain basin (shoot the fountain)"

[[waves]]
screen = 16
what = "4 BELL TURRETs on the village edge firing in sequence; 3 PIKEs down the lanes"

[[waves]]
screen = 18
what = "Observatory: 3 DOME MORTARs open in the small domes; lobbed shots arc toward the player's position"

[[waves]]
screen = 20
what = "Checkpoint 3. 6 PIKEs along the gravel paths in two directions; KESTREL pair overhead"

[[waves]]
screen = 22
what = "4 DOME MORTARs plus 2 PILLBOXes in the sundial court; 10 WASPs in a rolling wave from the left; leader drops a spark"

[[waves]]
screen = 24
what = "The great dome fills the top of the screen, the scroll stops, the shutters slide open: ASTROLABE"

[[tiles]]
id = "tileset.s2.vineyard"
section = 1
image_type = "texture"
subject = "terraced vineyard hillside from directly above: parallel rows of green vines on brown soil following contours, low dry-stone walls, warm afternoon light"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s2.farm_props"
section = 1
image_type = "sprite"
subject = "small farmhouses with red terracotta tile roofs, a stone well, lines of tall dark cypress trees with long shadows, a dirt track, top-down"
size_cells = [20, 16]
seamless = "none"

[[tiles]]
id = "setpiece.s2.aqueduct"
section = 2
image_type = "vertical_scrolling_background"
subject = "a single straight stone aqueduct of tall arches running from the left edge to the right edge of the image, once, across the middle, over a deep shadowed ravine, a thin silver stream far below, a water channel along the top, violet crystal on one span, top-down"
size_cells = [40, 20]
seamless = "none"

[[tiles]]
id = "tileset.s2.village"
section = 2
image_type = "texture"
subject = "a dense hilltop village from directly above: red and orange terracotta roofs, cream plaster courtyards, narrow brown lanes, striped awnings, laundry lines, violet crystal in the gutters"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "setpiece.s2.plaza"
section = 2
image_type = "vertical_scrolling_background"
subject = "a village square paved in pale stone circles with a round fountain and a square stone bell tower base at its centre, crystal cracking through the paving, top-down"
size_cells = [24, 20]
seamless = "none"

[[tiles]]
id = "tileset.s2.observatory"
section = 3
image_type = "texture"
subject = "an observatory campus from directly above: pale yellow raked gravel paths curving between clipped green hedges, violet and cyan crystal swallowing the paths"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s2.small_domes"
section = 3
image_type = "sprite"
subject = "white hemispherical observatory domes of three sizes with dark slit shutters, a low stone hall with a green copper roof, a sundial court, top-down"
size_cells = [24, 16]
seamless = "none"

[[tiles]]
id = "setpiece.s2.great_dome"
section = 3
image_type = "vertical_scrolling_background"
subject = "a huge white observatory dome seen from directly above, its two shutter halves sliding apart, crystal crusting the rim, raked gravel ring around it"
size_cells = [36, 20]
seamless = "none"
+++

# Setting

A ridge of terraced vineyards in warm afternoon gold, a hill village wrapped around its bell tower, and
on the crest an old observatory whose great telescope the Lattice has turned into a sky cannon guarding
the valley. Greens (5) and brown soil (9) give way to terracotta (8) and cream stone (15), then to white
domes and pale yellow gravel (7). Lattice violet (4) creeps uphill in every section, joined by light blue
and cyan crystal (14, 3) at the observatory. The background ink is brown (9): soil, lane shadows and
cypress shade all share it.

# Story beat

The Lattice has made the observatory's telescope into a cannon that commands the whole valley; no
Accord aircraft has crossed the ridge since. Clearing it opens the road inland to the rail lines.

Stage card: **STAGE 2  TERRACES** / "THEY TURNED A TELESCOPE INTO A GUN. CLOSE THE OBSERVATORY, QUILL."

# Section beats

**1 VINEYARDS (screens 0-9).** The vine rows are the stage's teaching device: PIKE rail drones run
exactly along them, so the player learns to read ground lines to predict air paths. KESTREL pairs cross
and fire on the turn, the first enemy that punishes sitting in the centre. Farmhouse ROOF HATCHes can
be blown open with BURST for 500-point bonuses, a first hint at weapon-specific rewards. Checkpoint 1
at screen 6.

**2 HILL VILLAGE (screens 9-17).** Dense rooftops carry BELL TURRETs (2x2 rotating patches) and more
ROOF HATCHes. The CAMPANILE midboss rises in the central square at screen 12; the scroll stops for the
fight and resumes when it falls. A hidden spark sits in the fountain basin. Checkpoint 2 at screen 14.

**3 OBSERVATORY (screens 17-26).** DOME MORTARs lob slow arcing shots that land where the player was a
second ago, teaching constant movement. PIKEs follow the gravel paths, KESTRELs mix overhead.
Checkpoint 3 at screen 20. The great dome closes the stage.

# Transition set piece

The AQUEDUCT (screens 8-9): a straight stone aqueduct strides across a ravine in deep shadow from the left edge to the right edge, its
arches casting long dark bars across the ravine floor. PILLBOXes sit on two arch piers, and a run of
PIKEs uses the water channel on top as their rail, so the transition is also a small skill test. The
quiet B section of the music starts as the village roofs come in.

# Midboss and boss arena

**CAMPANILE** (screen 12): the village square, a pale stone plaza 200 px across with the tower base
at its centre and roofs on every side. The player flies over it freely; the roofs hide two ROOF HATCHes
that open during the fight's second phase.

**ASTROLABE** (screens 24-26): the observatory crest. The great dome sits in the upper two thirds of the
screen with gravel all around; the scroll stops with the dome's centre at playfield line 64. The three
telescope rings occupy the open shutter slot, so ring-gap beam lanes sweep across the full width below.

# Music brief

`cue.stage_s2`, sunny synth-pop rock at 140 BPM, 2:30, seamless loop. A warm golden afternoon: a bright
plucked FM melody like a mandolin over a bouncing synth bass, light gated drums with shaker, and FM
brass answering each phrase with the rising talon call. The vineyard main theme is brisk and sunny. The
8-bar B section drops to pad and pluck alone with lighter drums for the hill village, a breath of calm
among the rooftops. The main theme returns with brass doubling the melody and full drums as the player
climbs to the crystal-choked observatory. Warm, clean, never sleepy.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): P2 pilot fixes
