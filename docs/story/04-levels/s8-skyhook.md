+++
id = "stage.s8"
kind = "level"
name = "SKYHOOK"
status = "approved"
revision = 3
slug = "skyhook"
stage_index = 8
title = "THE ORBITAL ELEVATOR"
theme = "cobalt fading to black, white stars, the cable up the screen centre, cyan, light blue and purple crystal, white-hot energy"
length_screens = 26
scroll_q8 = 128
target_minutes = 3
midboss = "midboss.s8.tether_warden"
boss = "boss.s8.crown"
enemies = ["enemy.s8.shard", "enemy.s8.mirror", "enemy.s8.climber_pod", "enemy.s8.prism_node", "enemy.common.wasp", "enemy.common.lancer", "enemy.common.kestrel", "enemy.common.spire_node"]
checkpoints = [6, 14, 20]
music = { cue = "cue.stage_s8", mood = "climbing out of the sky into silence and stars; awe first, then resolve building to the hero's peak", bpm = 144, length_s = 150, loop = true }
background_ink = 0
card_line = "THE CABLE STILL HOLDS. FOLLOW IT ALL THE WAY UP, QUILL."
cable = { x_px = 160, width_px = 32, note = "the SKYHOOK cable runs up the screen centre in every section and is the stage's ground layer" }

[[sections]]
n = 1
name = "UPPER ATMOSPHERE"
screens = [0, 9]
description = "The upper atmosphere seen straight down along a space elevator: a thick braided grey-white cable with yellow hazard bands and service rungs running up the exact centre of the frame, cobalt blue air darkening toward black at the top, thin high cirrus far below, a few faint early stars, boxy climber pods clamped to the cable with crystal growing over their hatches, and cyan crystal sheathing creeping up the cable."
inks_intent = "6 cobalt air, 15 light grey cable, 7 yellow hazard bands, 3 cyan crystal sheath"
beat = "CLIMBER POD turrets on the cable, crystal-plated WASPs and LANCERs, the first SHARD splitters."
teaches = "the cable is the ground: every ground target is in the centre column; SHARDs split along their travel line"
enemies = ["enemy.s8.climber_pod", "enemy.s8.shard", "enemy.common.wasp", "enemy.common.lancer"]

[[sections]]
n = 2
name = "ORBIT"
screens = [9, 17]
description = "Low orbit seen from above along the cable: deep black space full of sharp white stars, the curved blue-white limb of the Earth glowing along the bottom edge with thin atmosphere haze, the cable's anchor station of grey trusses and pressure modules, wide flat solar panel wings of dark blue cells in silver frames, and violet crystal nodes growing on the truss joints."
inks_intent = "0 black space, 1 white stars, 14 light blue Earth limb, 4 purple crystal nodes"
beat = "MIRROR units reflect BEAM; PRISM NODEs on the anchor trusses; the TETHER WARDEN climbs the cable."
teaches = "do not BEAM a MIRROR; side guns for things beside you"
enemies = ["enemy.s8.mirror", "enemy.s8.prism_node", "enemy.s8.climber_pod", "enemy.s8.shard", "enemy.common.kestrel", "enemy.common.spire_node"]

[[sections]]
n = 3
name = "CROWN APPROACH"
screens = [17, 26]
description = "The approach to a vast orbital ring station of crystal seen from directly above: huge faceted girders of cyan, light blue and purple crystal radiating from the cable's top, triangular lattice frames, glowing white-hot energy channels pulsing along the girders, hexagonal hull plates beginning to close overhead, black space and stars visible through the gaps, and the curve of the ring's inner wall at the top."
inks_intent = "3 cyan crystal, 14 light blue facets, 4 purple shadow facets, 1 white-hot energy"
beat = "Everything at once: plated variants of every air type, PRISM NODE networks, SHARDs and MIRRORs; the ring hull closes overhead."
teaches = "final exam: weapon choice matters (SEEKER for the CROWN)"
enemies = ["enemy.s8.prism_node", "enemy.s8.mirror", "enemy.s8.shard", "enemy.s8.climber_pod", "enemy.common.wasp", "enemy.common.lancer", "enemy.common.kestrel", "enemy.common.spire_node"]

[[transitions]]
section = 2
set_piece = "The SPACE LINE: the last cobalt of the sky drops away in a few sharp bands to pure black, the cable's sheathing changes from weathered grey to clean white, and a star field drawn through a bucket of twinkling sprites switches on all at once."
asset = "setpiece.s8.space_line"

[[waves]]
screen = 1
what = "2 CLIMBER PODs on the cable fire aimed pairs; 6 plated WASPs (2 HP) in an S around the cable"

[[waves]]
screen = 2
what = "First SHARD: one large crystal that splits into 2, then 4, along its travel line when hit"

[[waves]]
screen = 4
what = "4 plated LANCERs dive either side of the cable; 3 CLIMBER PODs stacked on the cable"

[[waves]]
screen = 6
what = "Checkpoint 1. 3 SHARDs in a row; 8 plated WASPs; leader drops a spark"

[[waves]]
screen = 8
what = "SPACE LINE set piece: the sky turns black, the bucket star field switches on; 2 CLIMBER PODs at the line"

[[waves]]
screen = 10
what = "Orbit: first MIRROR pair; BEAM shots reflect back down their line; 2 PRISM NODEs on the anchor trusses"

[[waves]]
screen = 12
what = "MIDBOSS TETHER WARDEN climbs the cable from below and overtakes, then holds above the player"

[[waves]]
screen = 14
what = "Checkpoint 2. KESTREL pairs plated; 4 PRISM NODEs on solar panel frames linked by SPIRE NODE beams; hidden spark on the anchor station's dish"

[[waves]]
screen = 16
what = "4 MIRRORs in a square around the cable, 3 SHARDs; Earth's limb drops out of the bottom edge"

[[waves]]
screen = 18
what = "Crown approach: crystal girders; 6 PRISM NODEs firing crossing lines; 8 plated WASPs"

[[waves]]
screen = 20
what = "Checkpoint 3. 4 SHARDs and 2 MIRRORs; plated LANCERs dive down the girders"

[[waves]]
screen = 22
what = "Everything at once: CLIMBER PODs, PRISM NODEs, a KESTREL pair, 10 plated WASPs; leader drops a spark"

[[waves]]
screen = 24
what = "The ring hull closes overhead and the cable ends at a docking collar; the scroll stops; CROWN, the Ring"

[[tiles]]
id = "tileset.s8.cable"
section = 1
image_type = "sprite-tiling-vertical"
subject = "a thick braided grey-white space elevator cable seen from directly above running straight up, yellow hazard bands, service rungs, cyan crystal sheathing creeping over it"
size_cells = [4, 25]
seamless = "v"

[[tiles]]
id = "tileset.s8.atmosphere"
section = 1
image_type = "texture"
subject = "the upper atmosphere seen straight down: cobalt blue air darkening toward black, thin high cirrus far below, a few faint stars"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s8.climber_props"
section = 1
image_type = "sprite"
subject = "boxy grey cargo climber pods clamped to a cable with crystal over their hatches, a cable service ring, top-down"
size_cells = [10, 12]
seamless = "none"

[[tiles]]
id = "setpiece.s8.space_line"
section = 2
image_type = "vertical_scrolling_background"
subject = "the edge of space: cobalt sky dropping away in a few sharp bands to pure black, a cable running up the centre whose sheathing turns from weathered grey to clean white, top-down"
size_cells = [40, 16]
seamless = "none"

[[tiles]]
id = "tileset.s8.orbit"
section = 2
image_type = "texture"
subject = "deep black space with sharp white stars and the curved glowing blue-white limb of the Earth along the bottom edge, top-down"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s8.anchor"
section = 2
image_type = "sprite"
subject = "a space elevator anchor station of grey trusses and pressure modules with wide flat dark blue solar panel wings in silver frames, violet crystal nodes on the truss joints, top-down"
size_cells = [40, 20]
seamless = "none"

[[tiles]]
id = "tileset.s8.girders"
section = 3
image_type = "texture"
subject = "huge faceted crystal girders of cyan, light blue and purple radiating outward, triangular lattice frames, white-hot energy channels, black space and stars through the gaps, top-down"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "setpiece.s8.ring_hull"
section = 3
image_type = "vertical_scrolling_background"
subject = "the underside of a vast faceted crystal ring station closing overhead, eight large hexagonal facet panels in an arc, a docking collar where a cable ends, white energy seams, top-down"
size_cells = [40, 22]
seamless = "none"
+++

# Setting

The SKYHOOK: the equatorial space elevator, its cable running straight up the **centre of the screen**
as the stage's ground layer. Every ground target in the stage sits on the cable or on structures hung
from it. The sky fades from cobalt (6) to black (0); white stars (1) appear at the space line as a bucket
of twinkling sprites; Earth's light blue limb (14) glows below in orbit; and at the top the CROWN's
faceted crystal in cyan (3), light blue (14) and purple (4) closes overhead with white-hot energy
seams. The background ink is black (0); the cobalt of section 1 is carried by cell inks over it.

# Story beat

The CROWN hangs at the top of the SKYHOOK. Every relay below feeds it; destroying it collapses the
Lattice everywhere. QUILL climbs the cable all the way up.

Stage card: **STAGE 8  SKYHOOK** / "THE CABLE STILL HOLDS. FOLLOW IT ALL THE WAY UP, QUILL."

On victory the ending plays: the CROWN collapses into falling stars over the dawn Earth.

# Section beats

**1 UPPER ATMOSPHERE (screens 0-9).** CLIMBER POD turrets are clamped to the cable, so every ground threat
is in the centre column and the open sky either side is the safe air. Crystal-plated WASPs and LANCERs
(2 HP, light blue plating) mix with the first SHARDs, which split along their travel line. Checkpoint 1
at screen 6.

**2 ORBIT (screens 9-17).** MIRROR units reflect BEAM back down its line: the stage tells the player to
change weapon. PRISM NODEs grow on the anchor trusses. The TETHER WARDEN climbs the cable at screen 12;
FLANK's side guns hit it as it passes. Checkpoint 2 at screen 14. Earth's limb drops away below.

**3 CROWN APPROACH (screens 17-26).** Crystal girders radiate from the cable's top. Every enemy type of
the stage at once, with PRISM NODE networks firing crossing lines. Checkpoint 3 at screen 20. The ring
hull closes overhead, and the cable ends at a docking collar.

# Transition set piece

The SPACE LINE (screens 8-9): the last cobalt drops away in four sharp bands to pure black in eight
cell rows. The cable's sheathing changes from weathered grey to clean white, and the star field (a
bucket of twinkling sprites, one channel per 21-line band, never on the player band) switches on all at
once with the music's B section. Two CLIMBER PODs sit exactly at the line.

# Midboss and boss arena

**TETHER WARDEN** (screen 12): the cable in open orbit, black space either side and Earth's limb at the
bottom. The Warden grips the cable, so the safe air is the two flanks of the screen, which is exactly
where FLANK's side guns reach it.

**CROWN** (screens 24-26): the scroll stops under the ring hull. Phase 1, **the Ring**, is the
screen-wide bitmap fortress across the top of the screen, eight facets in an arc that open in turn. The
cable's docking collar sits under its centre. Phase 2, **the Heart**: the ring shatters into falling
facet patches and a 4x8 crystal heart, flanked by two columns of orbiting crystal shards, comes down
into open space to hunt the player.

# Music brief

`cue.stage_s8`, cosmic synth-rock at 144 BPM, 2:30, seamless loop. The climb out of the sky: a pulsing
arpeggiated analog bass, wide shimmering synth pads and a crystalline bell lead for the upper
atmosphere, awed and weightless. The 8-bar B section opens the pads wide with sparse drums as the space
line is crossed and the stars switch on, the quietest moment in the score. The final main theme brings
the FM brass talon call back, stronger with each phrase, then rock guitar power chords and full gated
drums for the CROWN approach, building resolve to the hero's peak right before `cue.final_1`.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): sprite expansion removed at the user's request; bigger grids instead
