+++
id = "stage.s5"
kind = "level"
name = "BASTION"
status = "approved"
revision = 3
slug = "bastion"
stage_index = 5
title = "THE CITADEL WALLS"
theme = "late-day siege: olive and khaki stone, long dark grey shadows, violet crystal domes pulsing inside the walls"
length_screens = 26
scroll_q8 = 128
target_minutes = 3
midboss = "midboss.s5.portcullis"
boss = "boss.s5.quadrant"
enemies = ["enemy.s5.gallery_gun", "enemy.s5.wall_mortar", "enemy.common.spire_node", "enemy.common.wasp", "enemy.common.lancer", "enemy.common.pillbox", "enemy.common.crawler", "enemy.common.hatch"]
checkpoints = [6, 14, 20]
music = { cue = "cue.stage_s5", mood = "the siege: heavy, martial, determined, rising toward the revelation", bpm = 128, length_s = 150, loop = true }
background_ink = 11
card_line = "THE CITADEL IS THE LAST WALL ON THE CONTINENT. BRING IT DOWN."
twist_line = "THEN WE GO UP AFTER IT."

[[sections]]
n = 1
name = "MOAT AND GLACIS"
screens = [0, 9]
description = "The outer works of a vast star-shaped fortress in low late-day light, seen from directly above: a broad grassy glacis sloping toward a wide still moat of dark green water, angular earthwork ravelins with stone-faced edges, a long timber drawbridge on chains across the moat, a cobbled approach road with abandoned Accord trucks, sandbag lines, and violet crystal spikes standing in rows on the earthworks."
inks_intent = "9 brown earth, 5 green glacis, 11 dark grey shadows, 4 purple crystal"
beat = "PILLBOXes and CRAWLERs hold the earthworks; SPIRE NODE pylons appear, linked by beams; the drawbridge is raised."
teaches = "SPIRE NODE tethers: break one pylon and the linked beam drops; ground-target density ramps up"
enemies = ["enemy.common.pillbox", "enemy.common.crawler", "enemy.common.spire_node", "enemy.common.wasp", "enemy.common.lancer"]

[[sections]]
n = 2
name = "THE WALLS"
screens = [9, 17]
description = "The great curtain walls and arrow-head bastions of the citadel from directly above: massive khaki and olive stone ramparts many metres thick with crenellated parapets, long dark diagonal shadows across inner courts, rows of square gun embrasures along each face, stone stair ramps, a fortified gatehouse with an iron grate, rubble heaps from old shelling, and violet crystal sheathing the wall faces like glazing."
inks_intent = "15 light stone tops, 12 mid grey faces, 11 dark grey shadows, 4 purple crystal sheath"
beat = "GALLERY GUNS in rows of six, WALL MORTARs behind the parapets, LANCERs diving; the gatehouse PORTCULLIS."
teaches = "ground-target showcase: sweep rows, BEAM for long lines of patches"
enemies = ["enemy.s5.gallery_gun", "enemy.s5.wall_mortar", "enemy.common.lancer", "enemy.common.wasp", "enemy.common.spire_node"]

[[sections]]
n = 3
name = "INNER KEEP"
screens = [17, 26]
description = "The citadel's inner keep from directly above: a paved parade ground of large grey flagstones ringed by barrack roofs and an inner wall, four great round crystal domes of faceted violet and light blue glass pulsing with light in a square formation, power conduits of cyan crystal running between them through the paving, broken flagpoles, abandoned grey supply crates, and long evening shadows."
inks_intent = "12 mid grey flagstones, 4 purple domes, 14 light blue facets, 3 cyan conduits"
beat = "SPIRE NODEs guard the conduits; HATCHes in the barracks launch WASPs; the four domes wake."
teaches = "break the tether network in the right order; prepare for the boss"
enemies = ["enemy.common.spire_node", "enemy.common.hatch", "enemy.s5.wall_mortar", "enemy.s5.gallery_gun", "enemy.common.lancer", "enemy.common.wasp"]

[[transitions]]
section = 3
set_piece = "The WALL-TOP: about two screens of solid rampart walkway fill the whole width, a broad stone road along the top of the curtain wall with crenellations at both edges, gun galleries opening in its surface, and the drop into the inner keep's shadow at the end."
asset = "setpiece.s5.wall_top"

[[waves]]
screen = 1
what = "6 WASPs from the left over the glacis; leader drops a spark; 2 PILLBOXes on a ravelin"

[[waves]]
screen = 2
what = "3 CRAWLERs along the approach road in convoy; 4 LANCERs dive from the top"

[[waves]]
screen = 4
what = "First SPIRE NODE pair linked by a beam across the moat; destroying either drops the beam"

[[waves]]
screen = 5
what = "3 SPIRE NODEs in a triangle of beams plus 4 PILLBOXes on the earthworks; spark hidden in the drawbridge winch"

[[waves]]
screen = 6
what = "Checkpoint 1. 8 WASPs in two lines crossing; 2 LANCERs dive late"

[[waves]]
screen = 8
what = "The drawbridge and the first bastion: 2 WALL MORTARs lob from behind the parapet"

[[waves]]
screen = 10
what = "A row of 6 GALLERY GUNS fires in a rippling sequence; 4 LANCERs; a BEAM down the row clears it"

[[waves]]
screen = 12
what = "MIDBOSS PORTCULLIS: the gatehouse windows open in turn; the scroll stops"

[[waves]]
screen = 14
what = "Checkpoint 2. Two rows of 6 GALLERY GUNS on facing bastions with WALL MORTARs between; 10 WASPs"

[[waves]]
screen = 15
what = "WALL-TOP set piece: two screens of rampart road with gallery hatches opening in the walkway and LANCERs diving along it"

[[waves]]
screen = 18
what = "Inner keep: 4 SPIRE NODEs in a square of beams around the first conduit; 2 HATCHes launch WASPs"

[[waves]]
screen = 20
what = "Checkpoint 3. 6 LANCERs in pairs; 3 WALL MORTARs on the inner wall"

[[waves]]
screen = 22
what = "6 SPIRE NODEs in a hexagon of beams; GALLERY GUNS on the barrack roofs; 8 WASPs; leader drops a spark"

[[waves]]
screen = 24
what = "The four domes in a square fill the screen; the scroll stops; QUADRANT"

[[tiles]]
id = "tileset.s5.glacis"
section = 1
image_type = "texture"
subject = "a grassy glacis slope and angular earthworks with stone-faced edges from directly above, sandbag lines, rows of violet crystal spikes, long late-day shadows"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s5.moat"
section = 1
image_type = "sprite"
subject = "a wide still moat of dark green water with a long timber drawbridge on chains and a cobbled approach road with abandoned grey trucks, top-down"
size_cells = [40, 16]
seamless = "none"

[[tiles]]
id = "tileset.s5.walls"
section = 2
image_type = "sprite-tiling-vertical"
subject = "massive khaki and olive stone ramparts and arrow-head bastions from directly above with crenellated parapets, square gun embrasures, inner courts in deep diagonal shadow, violet crystal glazing the faces"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s5.wall_props"
section = 2
image_type = "sprite"
subject = "stone stair ramps, a corner watch turret, a gun gallery row of six square embrasures, rubble heaps, top-down"
size_cells = [24, 14]
seamless = "none"

[[tiles]]
id = "setpiece.s5.wall_top"
section = 3
image_type = "vertical_scrolling_background"
subject = "a broad straight stone rampart walkway running from the bottom edge to the top edge of the image and filling its full width, seen from directly above, crenellations along the left and right edges, square hatches in the paving, long shadows"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "setpiece.s5.gatehouse"
section = 2
image_type = "vertical_scrolling_background"
subject = "a single straight stone curtain wall running from the left edge to the right edge of the image, once, across the middle, with a fortified gatehouse at its centre, from directly above, four deep window slots in its roof face, an iron portcullis grate over the gate passage"
size_cells = [32, 16]
seamless = "none"

[[tiles]]
id = "tileset.s5.keep"
section = 3
image_type = "texture"
subject = "a parade ground of large grey flagstones from directly above, cyan crystal power conduits running through the paving, broken flagpoles, evening shadows"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "setpiece.s5.four_domes"
section = 3
image_type = "vertical_scrolling_background"
subject = "four great round faceted crystal domes of violet and light blue glass in a square formation on grey flagstones, cyan conduits joining them, top-down"
size_cells = [40, 22]
seamless = "none"
+++

# Setting

The continental stronghold: a vast star-fort whose stone walls the Lattice has sheathed in violet
crystal like glazing. Outer earthworks and a moat, then the great curtain walls and bastions, then the
inner keep where four crystal domes pulse in a square. This is the ground-target showcase, the stage
where bitmap-baked patches carry most of the threat. Late-day light throws long dark shadows (11) across
olive and khaki stone, rendered as brown (9), mid grey (12) and light grey (15) since the palette has no
olive. The violet crystal (4) with light blue facets (14) owns the keep. The background ink is dark grey
(11), the shadow colour, which the long shadows make the most common ink.

# Story beat

The Citadel is the last Lattice stronghold on the continent, and the Accord believes it is the centre.
When QUADRANT falls, the glyph screen reveals that the Citadel was only a **relay** beaming power up.

Stage card: **STAGE 5  BASTION** / "THE CITADEL IS THE LAST WALL ON THE CONTINENT. BRING IT DOWN."

After the twist cards ("THE CITADEL WAS A RELAY." / "SOMETHING ABOVE US IS DRINKING THE LIGHT."),
Brandt answers: "THEN WE GO UP AFTER IT."

# Section beats

**1 MOAT AND GLACIS (screens 0-9).** Earthworks hold PILLBOXes and CRAWLER convoys. SPIRE NODEs appear:
pylons linked by damaging beams that drop when either end dies, so the player learns to cut a network
at its weak point. The drawbridge is raised; a hidden spark sits in its winch. Checkpoint 1 at screen 6.

**2 THE WALLS (screens 9-17).** Rows of six GALLERY GUNS fire in rippling sequences; WALL MORTARs lob
from behind parapets; LANCERs dive along the wall faces. BEAM, which pierces ground targets, rewards
lining up on a row. The PORTCULLIS midboss holds the gatehouse at screen 12. Checkpoint 2 at screen 14.

**3 INNER KEEP (screens 17-26).** After two screens of wall-top, the keep's parade ground: SPIRE NODE
networks guard the conduits, barrack HATCHes launch WASPs. Checkpoint 3 at screen 20. The domes wake.

# Transition set piece

The WALL-TOP (screens 15-17): about two screens of solid rampart road fill the full width, the longest
continuous set piece in the game. Gallery hatches open in the walkway paving, LANCERs dive along its
length, and the music's breakdown (toms and bass ostinato alone) plays across it as siege tension. At
its end the wall drops away into the keep's shadow.

# Midboss and boss arena

**PORTCULLIS** (screen 12): the gatehouse sits in the upper half of the screen, built into the curtain
wall that crosses the full width. The courtyard below is open flagstone. The scroll stops until it falls,
then the grate rises and the scroll carries the player through the gate.

**QUADRANT** (screens 24-26): the four domes in a square fill the screen. The scroll stops with the
upper pair at the top edge. The cyan conduits between them pulse to show which dome is charging. When
the last dome lifts off as a sprite core, the floor below becomes an open arena.

# Music brief

`cue.stage_s5`, martial synth-rock at 128 BPM, 2:30, seamless loop. The siege: marching snare and deep
gated toms, a thick low synth-bass ostinato, a minor-key FM brass melody and distorted guitar power
chords. The main theme over the walls is heavy and determined, the slowest and weightiest stage tempo.
The 8-bar breakdown strips to toms and the bass ostinato alone for the wall-top crossing. The final main
theme brings guitar and brass together for the domes, and the rising talon call turns from minor to
major at the end of the loop: the music hints at the revelation before the glyph screen states it.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): P2 pilot fixes
