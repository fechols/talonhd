+++
id = "stage.s6"
kind = "level"
name = "VERDIGRIS"
status = "approved"
revision = 3
slug = "verdigris"
stage_index = 6
title = "JUNGLE RUINS"
theme = "humid greens and teal water, mossy grey stone, rust orange, huge glowing cyan crystal flowers"
length_screens = 26
scroll_q8 = 128
target_minutes = 3
midboss = "midboss.s6.monolith"
boss = "boss.s6.orchid"
enemies = ["enemy.s6.moth", "enemy.s6.dragonfly", "enemy.s6.bloom", "enemy.s6.statue_turret", "enemy.s6.canopy_hatch", "enemy.common.kestrel", "enemy.common.wasp", "enemy.common.crawler"]
checkpoints = [6, 14, 20]
music = { cue = "cue.stage_s6", mood = "humid, lush, mysterious; crystal flowers glowing in green shade; heroic at the gantry", bpm = 136, length_s = 150, loop = true }
background_ink = 5
card_line = "THE OLD LAUNCH SITE IS UNDER ALL THAT GREEN. FIND US A WAY UP."

[[sections]]
n = 1
name = "CANOPY"
screens = [0, 9]
description = "Dense equatorial rainforest canopy seen from directly above: rounded crowns of trees in many greens packed edge to edge, a few taller emergent trees casting deep shade, gaps where a winding teal river shows through with sandbars and fallen logs, hanging vines, white mist pooling in the hollows, and huge cyan crystal flowers opening out of the treetops with white glints at their tips."
inks_intent = "5 green canopy, 13 light green highlights, 6 blue deep shade, 3 cyan crystal flowers"
beat = "MOTHs drift and split when hit; CANOPY HATCHes hidden in the trees open; BLOOMs flower in the crowns."
teaches = "splitters (kill the fragments or let them pass); BLOOMs fire rings when open, BEAM them open for bonus"
enemies = ["enemy.s6.moth", "enemy.s6.canopy_hatch", "enemy.s6.bloom", "enemy.common.wasp", "enemy.common.kestrel"]

[[sections]]
n = 2
name = "TEMPLE RUINS"
screens = [9, 17]
description = "Overgrown ruins of stepped stone terraces seen from directly above: square platforms of mossy grey blocks stepping down on four sides, broken causeways between them, tumbled carved blocks of abstract geometric pattern, tall rectangular standing stones, roots splitting the paving, still teal pools with lily pads in sunken courts, and cyan and light blue crystal growing in hexagonal clusters over the stonework."
inks_intent = "12 mid grey stone, 5 green moss, 14 light blue crystal, 3 cyan pools and glow"
beat = "STATUE TURRETs on the terraces, DRAGONFLY strafers sweeping the courts, the MONOLITH slides open."
teaches = "fast strafers that cross the whole width; turrets hidden in the scenery"
enemies = ["enemy.s6.statue_turret", "enemy.s6.dragonfly", "enemy.s6.moth", "enemy.s6.bloom", "enemy.common.crawler"]

[[sections]]
n = 3
name = "LAUNCH APRON"
screens = [17, 26]
description = "An abandoned equatorial launch site reclaimed by jungle, seen from directly above: a vast cracked concrete pad with faded yellow and black markings, grass and saplings pushing through the seams, a rusted orange service gantry lying along one side with its cross-bracing, flame trenches choked with vines, round fuel tanks streaked with rust, and huge cyan crystal flowers rooted in the concrete."
inks_intent = "12 mid grey concrete, 8 rust orange gantry, 5 green growth, 3 cyan crystal flowers"
beat = "BLOOMs in ranks across the pad, DRAGONFLYs and KESTRELs mixed; the largest flower of all at the centre."
teaches = "density: everything learned in the stage at once"
enemies = ["enemy.s6.bloom", "enemy.s6.dragonfly", "enemy.common.kestrel", "enemy.s6.moth", "enemy.s6.statue_turret", "enemy.common.crawler"]

[[transitions]]
section = 2
set_piece = "The WATERFALL GORGE: the river drops off a broad rock lip into a deep gorge, white water fanning over black rock, a rainbow of spray drawn in pale bands, the gorge walls hung with ferns and crystal, and the first temple terrace rising on the far side."
asset = "setpiece.s6.waterfall"

[[waves]]
screen = 1
what = "4 MOTHs drift down slowly; each splits into 3 when hit; the last fragment of the last MOTH drops a spark"

[[waves]]
screen = 2
what = "2 CANOPY HATCHes open in the treetops and launch 3 WASPs each"

[[waves]]
screen = 3
what = "First BLOOM pair in the crowns: they open on a cycle and fire 8-way rings; BEAM them while open for 1,000"

[[waves]]
screen = 5
what = "KESTREL pairs from both sides; 6 MOTHs in a slow V"

[[waves]]
screen = 6
what = "Checkpoint 1. 3 CANOPY HATCHes plus 4 BLOOMs along the river gap; spark from a hatch"

[[waves]]
screen = 8
what = "WATERFALL GORGE set piece: 6 MOTHs drift through the spray; 2 STATUE TURRETs on the far rock lip"

[[waves]]
screen = 10
what = "Temple: 4 STATUE TURRETs on the stepped platforms; first DRAGONFLY strafes across the full width"

[[waves]]
screen = 12
what = "MIDBOSS MONOLITH: the tall stone in the central court slides open on its crystal lens; the scroll stops"

[[waves]]
screen = 14
what = "Checkpoint 2. 3 DRAGONFLYs strafe in sequence from alternate sides; 2 CRAWLERs on the causeways"

[[waves]]
screen = 16
what = "6 STATUE TURRETs and 2 BLOOMs in a sunken court; hidden spark under a tumbled block"

[[waves]]
screen = 18
what = "Launch apron: 6 BLOOMs in two ranks opening alternately; KESTREL pair"

[[waves]]
screen = 20
what = "Checkpoint 3. 4 DRAGONFLYs plus 8 MOTHs; 2 CRAWLERs in the flame trench"

[[waves]]
screen = 22
what = "BLOOMs ring a fuel tank; STATUE TURRETs on the gantry base; 10 WASPs from the right; leader drops a spark"

[[waves]]
screen = 24
what = "The biggest crystal flower on the centre of the pad lifts off its stem; the scroll stops; ORCHID"

[[tiles]]
id = "tileset.s6.canopy"
section = 1
image_type = "texture"
subject = "dense rainforest canopy from directly above: rounded tree crowns in many greens packed edge to edge, deep shade, white mist in the hollows"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s6.river"
section = 1
image_type = "sprite-tiling-vertical"
subject = "a winding teal jungle river seen through a gap in the canopy, sandbars and fallen logs, overhanging branches, top-down"
size_cells = [16, 25]
seamless = "v"

[[tiles]]
id = "tileset.s6.crystal_flowers"
section = 1
image_type = "sprite"
subject = "huge faceted cyan crystal flowers with hexagonal petals and white glints opening out of treetops and concrete, three sizes, top-down"
size_cells = [16, 12]
seamless = "none"

[[tiles]]
id = "setpiece.s6.waterfall"
section = 2
image_type = "vertical_scrolling_background"
subject = "a jungle river pouring off a broad straight rock lip that runs from the left edge to the right edge of the image, once, across the middle, into a deep gorge, from directly above, white water fanning over black rock, pale spray bands, ferns and crystal on the gorge walls"
size_cells = [40, 20]
seamless = "none"

[[tiles]]
id = "tileset.s6.temple"
section = 2
image_type = "texture"
subject = "overgrown stepped stone terraces from directly above: mossy grey blocks, broken causeways, roots splitting the paving, teal pools in sunken courts, hexagonal crystal clusters"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s6.temple_props"
section = 2
image_type = "sprite"
subject = "tall rectangular standing stones, tumbled carved blocks with abstract geometric patterns, a square sunken pool, top-down"
size_cells = [20, 14]
seamless = "none"

[[tiles]]
id = "tileset.s6.apron"
section = 3
image_type = "texture"
subject = "a vast cracked concrete launch pad from directly above with faded yellow and black markings, grass and saplings through the seams, vine-choked flame trenches"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s6.gantry"
section = 3
image_type = "sprite"
subject = "a rusted orange launch service gantry lying along the edge of a pad with cross-bracing, round rust-streaked fuel tanks, top-down"
size_cells = [12, 25]
seamless = "none"

[[tiles]]
id = "setpiece.s6.flower_stem"
section = 3
image_type = "vertical_scrolling_background"
subject = "the cracked centre of a launch pad where a giant crystal flower stem is rooted, hexagonal crystal roots spreading through the concrete, top-down"
size_cells = [40, 22]
seamless = "none"
+++

# Setting

The old equatorial launch site, deep under jungle the Lattice has seeded more thickly than anywhere on
Earth. Treetop canopy with river gaps, then stepped stone ruins of abstract geometric carving, then the
cracked concrete launch apron with its rusted gantry. Humid greens (5, 13), teal and blue water and shade
(6), mossy grey stone (12), rust orange (8), and the stage's signature: huge glowing cyan crystal flowers
(3) with light blue facets (14). The background ink is green (5). The ruins carry no real-world religious
iconography: only steps, blocks and abstract geometric patterns.

# Story beat

To reach the CROWN the Accord needs a way up, and the only one left is the old equatorial launch site
at the foot of the SKYHOOK. The Lattice has grown a garden over it.

Stage card: **STAGE 6  VERDIGRIS** / "THE OLD LAUNCH SITE IS UNDER ALL THAT GREEN. FIND US A WAY UP."

# Section beats

**1 CANOPY (screens 0-9).** MOTH drifters split into three when hit, teaching the player to finish
fragments or slip past them. CANOPY HATCHes hide in the treetops until they open. BLOOMs flower in the
crowns, firing rings when open and paying a bonus when BEAM hits them open. Checkpoint 1 at screen 6.

**2 TEMPLE RUINS (screens 9-17).** STATUE TURRETs hide among the standing stones; DRAGONFLY strafers
cross the full width fast. The MONOLITH midboss opens in the central court at screen 12. Checkpoint 2
at screen 14. A hidden spark sits under a tumbled block.

**3 LAUNCH APRON (screens 17-26).** BLOOMs in ranks across the concrete, DRAGONFLYs and KESTRELs mixed
overhead, CRAWLERs in the flame trenches. Checkpoint 3 at screen 20. The largest flower waits at the
pad's centre.

# Transition set piece

The WATERFALL GORGE (screens 8-9): the river drops off a broad rock lip into a deep gorge. White water
fans across black rock in three animated patch columns, and pale spray bands drift across the gorge.
MOTHs drift through the spray, where they are hardest to read; two STATUE TURRETs on the far lip are the
first sign of the ruins. The music's B section starts as the first terrace appears.

# Midboss and boss arena

**MONOLITH** (screen 12): a sunken central court ringed by stepped platforms. The monolith stands in
the upper middle. The scroll stops; two BLOOMs on the court corners open in the second phase.

**ORCHID** (screens 24-26): the centre of the launch pad, open concrete with the flower's crystal roots
spreading through it. The scroll stops with the stem socket at playfield line 40. The flower is all
sprite: it rises above the ceiling line and its petals orbit there. The concrete below gives a plain
backdrop for reading homing petals in phase 2.

# Music brief

`cue.stage_s6`, lush jungle synth-rock at 136 BPM, 2:30, seamless loop. Humid and mysterious: a
marimba-like FM ostinato over rolling electronic hand-drum percussion, a warm analog pad, deep synth
bass and a glassy bell lead like the crystal flowers glowing in green shade. A gated snare on two and
four keeps it driving. The canopy main theme is lush and curious. The 8-bar B section drops to pad and
percussion with a distant bell for the temple ruins. Then the FM brass talon call returns over full drums
at the launch apron, the music turning heroic again as the rusted gantry comes into view.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): P2 pilot fixes
