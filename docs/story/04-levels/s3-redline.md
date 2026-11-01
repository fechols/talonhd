+++
id = "stage.s3"
kind = "level"
name = "REDLINE"
status = "approved"
revision = 3
slug = "redline"
stage_index = 3
title = "DESERT CANYON RAILWAY"
theme = "hard midday sun on orange sand and red rock, long black shadows, dark steel rails, cyan-glowing crystal ore"
length_screens = 26
scroll_q8 = 128
target_minutes = 3
midboss = "midboss.s3.sidewinder"
boss = "boss.s3.trestle_titan"
enemies = ["enemy.s3.dust_devil", "enemy.s3.vulture", "enemy.s3.rail_car", "enemy.s3.sandcrawler", "enemy.s3.water_tower", "enemy.common.wasp", "enemy.common.lancer", "enemy.common.crawler"]
checkpoints = [6, 14, 20]
music = { cue = "cue.stage_s3", mood = "hard sun and steel rails; dusty, relentless locomotive drive", bpm = 146, length_s = 150, loop = true }
background_ink = 8
card_line = "EVERY TRAIN ON THAT LINE CARRIES THEIR POWER. CUT THE LINE."

[[sections]]
n = 1
name = "OPEN DESERT"
screens = [0, 9]
description = "Open desert at noon seen from directly above: rippled orange sand dunes with sharp crests and dark lee-side shadows, flat brown rock outcrops, a lone two-lane highway with faded yellow centre lines and a rusted signpost, a single railway line of dark grey rails on pale ballast running straight up the frame, telegraph poles casting thin shadows, and small cyan crystal clusters glinting in the sand."
inks_intent = "8 orange sand, 9 brown rock, 11 dark grey rails, 7 yellow highlights"
beat = "DUST DEVILs circle out of the dunes; SANDCRAWLERs surface from the sand; the first RAIL CARs ride the line."
teaches = "circling enemies (shoot the centre of the orbit); tanks that surface from cover"
enemies = ["enemy.s3.dust_devil", "enemy.s3.sandcrawler", "enemy.s3.rail_car", "enemy.common.wasp"]

[[sections]]
n = 2
name = "MESA CANYON"
screens = [9, 17]
description = "A deep mesa canyon from directly above: red-orange rock walls on both sides falling into deep black-brown shadow, flat mesa tops in pale orange with scrub, the twin railway weaving along the canyon floor and vanishing into square tunnel mouths cut in the rock, rockfall boulders and fallen telegraph poles beside the track, a rusted signal gantry, and cyan crystal veins running through the canyon walls."
inks_intent = "8 orange mesa tops, 9 brown walls, 0 black shadow, 3 cyan veins"
beat = "Rails weave through tunnels; RAIL CARs pop out of the tunnel mouths; the armoured SIDEWINDER runs alongside."
teaches = "targets that appear from tunnels at fixed points; the first sprite midboss"
enemies = ["enemy.s3.rail_car", "enemy.s3.dust_devil", "enemy.common.lancer", "enemy.s3.vulture"]

[[sections]]
n = 3
name = "FREIGHT YARD"
screens = [17, 26]
description = "A sun-baked railway freight yard seen from above: a fan of a dozen parallel dark grey tracks with switch points and buffer stops, rows of rust-brown and grey boxcars and flatcars, tall round water towers on steel legs, a turntable pit, a corrugated engine shed, and conical heaps of glowing cyan crystal ore piled beside the tracks under the hard white sun."
inks_intent = "9 brown ballast, 11 dark grey rails, 12 mid grey cars, 3 cyan ore heaps"
beat = "WATER TOWER emplacements, RAIL CARs on every track, a VULTURE gunship over the yard; the crane at the far end stirs."
teaches = "many ground targets at once: prioritise; VULTUREs drop 2 sparks"
enemies = ["enemy.s3.water_tower", "enemy.s3.rail_car", "enemy.common.crawler", "enemy.s3.vulture", "enemy.s3.dust_devil", "enemy.common.wasp"]

[[transitions]]
section = 3
set_piece = "The TRESTLE: a tall timber-and-steel trestle bridge carries the railway across a wide dry riverbed of cracked pale mud and round stones; its lattice of beams throws a criss-cross shadow on the riverbed, and a RAIL CAR crosses it as you do."
asset = "setpiece.s3.trestle"

[[waves]]
screen = 1
what = "4 DUST DEVILs spiral out of a dune and circle in place, then drift down; the last one drops a spark"

[[waves]]
screen = 2
what = "6 WASPs in a fast line from the left along the highway"

[[waves]]
screen = 3
what = "2 SANDCRAWLERs surface beside the highway and fire aimed pairs; 1 RAIL CAR rides the line up"

[[waves]]
screen = 5
what = "6 DUST DEVILs in two circles of three, crossing; 2 SANDCRAWLERs surface behind the player's x"

[[waves]]
screen = 6
what = "Checkpoint 1. 2 RAIL CARs in tandem, 8 WASPs in a V from the top; leader drops a spark"

[[waves]]
screen = 8
what = "Canyon mouth: the walls close in; 3 LANCERs dive from the mesa tops"

[[waves]]
screen = 10
what = "RAIL CARs burst out of two tunnel mouths in turn; DUST DEVILs circle in the canyon bend"

[[waves]]
screen = 12
what = "MIDBOSS SIDEWINDER: the armoured engine and 4 cars emerge from a tunnel and run up the track beside the player's lane"

[[waves]]
screen = 14
what = "Checkpoint 2. First VULTURE gunship crosses slowly firing spreads, drops 2 sparks; 4 LANCERs escort"

[[waves]]
screen = 16
what = "TRESTLE set piece; a RAIL CAR crosses the bridge with the player, 6 DUST DEVILs below over the riverbed"

[[waves]]
screen = 18
what = "Freight yard: 3 WATER TOWERs (slow 5-way fans), RAIL CARs on three tracks, 2 CRAWLERs among the boxcars"

[[waves]]
screen = 20
what = "Checkpoint 3. 2 VULTUREs side by side, 10 WASPs weaving between them"

[[waves]]
screen = 22
what = "Every track live: 5 RAIL CARs, 4 WATER TOWERs, 2 SANDCRAWLERs in the ore heaps; hidden spark in the turntable pit"

[[waves]]
screen = 24
what = "The yard ends in a gantry crane over the tracks; the scroll stops; TRESTLE TITAN wakes"

[[tiles]]
id = "tileset.s3.desert"
section = 1
image_type = "texture"
subject = "rippled orange sand dunes from directly above with sharp crests and dark lee shadows, flat brown rock outcrops, small cyan crystal clusters, hard noon light"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s3.rail_single"
section = 1
image_type = "sprite-tiling-vertical"
subject = "a single railway line of dark grey rails on pale ballast and wooden sleepers running straight up, telegraph poles beside it, top-down"
size_cells = [6, 25]
seamless = "v"

[[tiles]]
id = "tileset.s3.highway"
section = 1
image_type = "sprite-tiling-vertical"
subject = "a lone sun-bleached two-lane highway with faded yellow centre lines and sand drifting over its edges, top-down"
size_cells = [8, 25]
seamless = "v"

[[tiles]]
id = "tileset.s3.canyon"
section = 2
image_type = "texture"
subject = "a deep red rock canyon from directly above: orange mesa tops with scrub on both sides, sheer walls falling into black shadow, cyan crystal veins in the walls"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s3.tunnel"
section = 2
image_type = "sprite"
subject = "a square railway tunnel mouth cut into red canyon rock with the twin rails disappearing into darkness, rockfall boulders beside, top-down"
size_cells = [12, 10]
seamless = "none"

[[tiles]]
id = "setpiece.s3.trestle"
section = 3
image_type = "vertical_scrolling_background"
subject = "a single tall timber and steel trestle railway bridge running straight from the bottom edge to the top edge of the image, once, up the middle, over a wide dry riverbed of cracked pale mud and round stones that crosses from the left edge to the right edge, the beam lattice casting a criss-cross shadow, top-down"
size_cells = [40, 20]
seamless = "none"

[[tiles]]
id = "tileset.s3.yard"
section = 3
image_type = "texture"
subject = "a freight yard from directly above: a fan of parallel dark grey tracks with switch points on brown ballast, buffer stops, oil stains"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s3.yard_props"
section = 3
image_type = "sprite"
subject = "rust-brown and grey boxcars and flatcars, a round railway turntable pit, a corrugated engine shed roof, conical heaps of glowing cyan crystal ore, top-down"
size_cells = [24, 20]
seamless = "none"

[[tiles]]
id = "setpiece.s3.crane_yard"
section = 3
image_type = "vertical_scrolling_background"
subject = "the end of a freight yard where all tracks run up from the bottom edge and converge under a huge gantry crane footprint, crystal ore spilled across the sleepers, deep shadows, top-down"
size_cells = [40, 22]
seamless = "none"
+++

# Setting

A desert railway carrying harvested Lattice power to the coast. Open dunes and a lone highway under
hard noon sun, then a mesa canyon whose walls drop into black shadow and whose rails duck through
tunnels, then a sprawling freight yard of boxcars, water towers and glowing ore heaps. The palette is
the hottest in the game: orange sand (8), brown rock (9), yellow sun highlights (7), dark grey rails (11)
and black shadow (0), with cyan crystal ore (3) as the only cool colour. The background ink is orange (8).

# Story beat

The Lattice moves its harvested power by rail. Every train that reaches the coast feeds a relay. QUILL
is sent to cut the line at the yard where the trains are made up.

Stage card: **STAGE 3  REDLINE** / "EVERY TRAIN ON THAT LINE CARRIES THEIR POWER. CUT THE LINE."

# Section beats

**1 OPEN DESERT (screens 0-9).** DUST DEVILs spiral out of dunes and circle before drifting down: shoot
the centre of the orbit. SANDCRAWLERs surface from the sand (a bitmap patch sequence of sand parting),
always telegraphed by a dust ring two seconds before. The single line carries the first RAIL CARs.
Checkpoint 1 at screen 6.

**2 MESA CANYON (screens 9-17).** The walls narrow the visual field without narrowing the playfield (the
TALON flies above the canyon), but the rails now weave and vanish into tunnels; RAIL CARs appear from
tunnel mouths at fixed points. The SIDEWINDER midboss bursts from a tunnel at screen 12 and runs up the
track beside the player's lane. Checkpoint 2 at screen 14, followed by the first VULTURE, a slow
gunship worth 2 sparks.

**3 FREIGHT YARD (screens 17-26).** After the trestle, the rails fan out into a dozen tracks. Ground
targets crowd the screen: WATER TOWER emplacements, RAIL CARs on several tracks, CRAWLERs among the
boxcars. The lesson is prioritising. Checkpoint 3 at screen 20. A hidden spark in the turntable pit.

# Transition set piece

The TRESTLE (screens 16-17): the canyon opens onto a dry riverbed, and the railway crosses on a tall
timber-and-steel trestle. Its criss-cross shadow on the pale mud is the most striking graphic in the
stage, drawn in black (0) and brown (9). A RAIL CAR rides the bridge at scroll speed beside the player,
a last duel before the yard.

# Midboss and boss arena

**SIDEWINDER** (screen 12): the canyon floor's twin track runs up the left third of the screen; the
train uses the left track going up, so the right two thirds stay open for dodging. Tunnel mouths top and
bottom bracket the fight.

**TRESTLE TITAN** (screens 24-26): the convergence of the yard tracks under the crane's footprint. The
scroll stops with the crane's rail beams at the top edge. Ore heaps at both sides are scenery; the
claw-sweep lanes cross open sleepers so the player always reads the danger against plain ballast.

# Music brief

`cue.stage_s3`, desert synth-rock at 146 BPM, 2:30, seamless loop. Hard sun and steel rails: a chugging
train-rhythm synth bass locked with palm-muted guitar, twangy lead guitar phrases, FM brass stabs and
gated drums with shuffle hi-hats. The open-desert main theme is dusty and relentless, a locomotive that
never brakes. The 8-bar breakdown pulls down to low toms and muted guitar for the canyon's deep shadow.
Then brass and guitar charge together through the freight yard, the rising three-note talon call
riding over the rail rhythm. Punchy and dry, with a tight low end and short tails.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): P2 pilot fixes
