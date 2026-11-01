+++
id = "stage.s1"
kind = "level"
name = "BREAKWATER"
status = "approved"
revision = 3
slug = "breakwater"
stage_index = 1
title = "COASTAL AIRBASE AT DAWN"
theme = "bright morning coast: deep blue sea, grey concrete, white runway paint, orange cranes, cyan crystal in the cracks"
length_screens = 26
scroll_q8 = 128
target_minutes = 3
midboss = "midboss.s1.keelbreaker"
boss = "boss.s1.gantry_queen"
enemies = ["enemy.common.wasp", "enemy.s1.skiff", "enemy.s1.gannet", "enemy.common.pillbox", "enemy.common.hatch", "enemy.common.crawler"]
checkpoints = [6, 14, 20]
music = { cue = "cue.stage_s1", mood = "launch-morning optimism, salt spray and jet noise; bright and heroic", bpm = 152, length_s = 150, loop = true }
background_ink = 6
card_line = "THAT'S OUR RUNWAY THEY'RE STANDING ON, CAPTAIN. GET IT BACK."

[[sections]]
n = 1
name = "OPEN SEA"
screens = [0, 9]
description = "Open ocean seen straight down at dawn: deep blue water with long rolling swells, scattered white whitecaps, the broad white V of the carrier MERIDIAN's wake fading behind the start position, a few drifting slicks of foam, small rocky islets ringed with surf and pale sand, a red-and-white marker buoy, and low morning light catching the wave crests in light blue streaks. Short soft shadows fall lower right."
inks_intent = "6 blue sea, 14 light blue crests, 1 white surf and wake, 15 light grey rocks"
beat = "Launch from the carrier wake into clean sky; WASP waves teach the S-curve, SKIFFs on the water teach that ground-level targets ignore your altitude."
teaches = "movement, TWIN fire, collecting the first sparks and stepping the gauge"
enemies = ["enemy.common.wasp", "enemy.s1.skiff", "enemy.s1.gannet"]

[[sections]]
n = 2
name = "THE DOCKS"
screens = [9, 17]
description = "A naval harbour behind a straight concrete breakwater: grey quays with white bollards and yellow hazard edges, long timber piers on dark piles, orange gantry cranes with their booms swung over the water, stacked olive and white cargo containers, fuel tanks with numbered lids, tugboats moored in rows, oily dark water in the basins, and thin cyan crystal veins climbing the crane legs and splitting the quay slabs."
inks_intent = "12 mid grey concrete, 8 orange cranes, 6 blue basins, 3 cyan crystal veins"
beat = "Docks crowded with PILLBOX turrets on the quays and SKIFFs in the basins; the destroyer KEELBREAKER pulls out of its berth and steams up the channel ahead of you."
teaches = "ground turrets aim at you; destroy them before they turn; first midboss"
enemies = ["enemy.common.pillbox", "enemy.s1.skiff", "enemy.common.wasp", "enemy.s1.gannet"]

[[sections]]
n = 3
name = "THE AIRFIELD"
screens = [17, 26]
description = "The Accord forward airbase: two broad grey concrete runways with white threshold bars, centre dashes and big painted numbers, taxiways edged in yellow, curved hangar roofs of ribbed light grey metal, sandbagged AA pits, orange windsocks on striped poles, a radar dish on a blockhouse, scorched tarmac patches, and bright cyan crystal threading every crack in the concrete toward the apron."
inks_intent = "12 mid grey runway, 1 white markings, 8 orange windsocks, 3 cyan crystal cracks"
beat = "Hangars open to launch WASPs, RUNWAY CRAWLERs grind along the taxiways, and the crystal growth thickens toward the hangar apron where the control tower waits."
teaches = "HATCH launchers (kill the hatch before it opens), tanks on fixed paths, a boss"
enemies = ["enemy.common.hatch", "enemy.common.crawler", "enemy.common.pillbox", "enemy.common.wasp", "enemy.s1.gannet"]

[[transitions]]
section = 2
set_piece = "The SEA WALL: a massive straight concrete breakwater of interlocking tetrapod blocks crosses the whole screen, surf bursting white over its top in animated patches, a lighthouse stub at the right end with a cyan crystal lamp, and calm dark harbour water beyond."
asset = "setpiece.s1.sea_wall"

[[waves]]
screen = 1
what = "6 WASPs, S-curve from the left, no fire; the leader drops a spark"

[[waves]]
screen = 2
what = "6 WASPs mirrored from the right; 2 SKIFFs cross the water left to right, single aimed shot each"

[[waves]]
screen = 3
what = "2 GANNETs dive from the top corners and pull up; first aimed shots at the player"

[[waves]]
screen = 4
what = "4 SKIFFs in a line abreast moving up the screen, staggered aimed shots; 6 WASPs from the top centre split left and right"

[[waves]]
screen = 6
what = "Checkpoint 1. 8 WASPs in two S-curves crossing; the leader of each drops a spark"

[[waves]]
screen = 8
what = "SEA WALL set piece; 2 PILLBOXes on the wall top, 3 GANNETs dive in a staggered row"

[[waves]]
screen = 10
what = "Docks: 4 PILLBOXes on the quays (16-angle aimed), 3 SKIFFs leave the basins, WASP pair overhead"

[[waves]]
screen = 12
what = "MIDBOSS KEELBREAKER leaves its berth and steams up the centre channel; the scroll slows while it is on screen; 2 GANNETs join at half health"

[[waves]]
screen = 14
what = "Checkpoint 2. 6 PILLBOXes on crane bases, WASPs in a 10-wave rolling from the right; spark from the leader"

[[waves]]
screen = 16
what = "4 SKIFFs in the last basin plus 3 GANNETs diving in a V; the breakwater ends at the airfield seawall"

[[waves]]
screen = 18
what = "Airfield: 2 HATCHes on the first hangar roofs open and launch 3 WASPs each; a RUNWAY CRAWLER on the near taxiway"

[[waves]]
screen = 20
what = "Checkpoint 3. 3 RUNWAY CRAWLERs in convoy down the main runway, 4 PILLBOXes in AA pits"

[[waves]]
screen = 22
what = "8 WASPs in a pincer from both sides while 2 HATCHes launch; 2 GANNETs dive late; hidden spark under a windsock pole"

[[waves]]
screen = 24
what = "Hangar apron: crystal growth fills the frame, the scroll stops, GANTRY QUEEN rises from the apron"

[[tiles]]
id = "tileset.s1.sea"
section = 1
image_type = "texture"
subject = "open deep blue ocean seen straight down, long rolling swells running horizontally across the image, scattered small white whitecaps, light blue glints on the crests"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s1.wake"
section = 1
image_type = "sprite"
subject = "the broad white V of an aircraft carrier wake on blue water, seen from directly above, fading toward the top"
size_cells = [24, 25]
seamless = "none"

[[tiles]]
id = "tileset.s1.islets"
section = 1
image_type = "sprite"
subject = "three small rocky islets ringed with white surf and pale sand, a red and white marker buoy, top-down"
size_cells = [16, 12]
seamless = "none"

[[tiles]]
id = "setpiece.s1.sea_wall"
section = 2
image_type = "vertical_scrolling_background"
subject = "a single straight sea wall running from the left edge to the right edge of the image, once, across the middle, with no end and no turn inside the image: a massive concrete breakwater of interlocking tetrapod blocks, open blue sea with white surf bursting against it filling the whole lower half, calm dark harbour water filling the whole upper half, a short round lighthouse stub with a cyan crystal lamp standing on top of the wall near the right side while the wall carries on past it to the right edge, top-down"
size_cells = [40, 16]
seamless = "none"

[[tiles]]
id = "tileset.s1.quay"
section = 2
image_type = "texture"
subject = "grey concrete harbour quays with white bollards and yellow hazard edges, timber piers on dark piles, oily dark blue basin water, thin cyan crystal veins splitting the slabs"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s1.dock_props"
section = 2
image_type = "sprite"
subject = "orange gantry cranes with booms over the water, stacked olive and white cargo containers, round fuel tanks with numbered lids, small moored tugboats, all from directly above"
size_cells = [20, 16]
seamless = "none"

[[tiles]]
id = "tileset.s1.berth"
section = 2
image_type = "sprite"
subject = "an empty long destroyer berth: a dark water slot between two concrete quays with mooring lines left trailing and crystal growth on the bollards"
size_cells = [14, 34]
seamless = "none"

[[tiles]]
id = "tileset.s1.runway"
section = 3
image_type = "sprite-tiling-vertical"
subject = "a broad grey concrete runway seen from above with white centre dashes and edge lines, yellow taxiway borders, scorched patches, bright cyan crystal threading the cracks"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s1.airfield_props"
section = 3
image_type = "sprite"
subject = "curved ribbed light grey hangar roofs, sandbagged round AA pits, orange windsocks on striped poles, a radar dish on a blockhouse, painted runway numbers, top-down"
size_cells = [24, 20]
seamless = "none"

[[tiles]]
id = "setpiece.s1.apron"
section = 3
image_type = "vertical_scrolling_background"
subject = "a wide concrete hangar apron overgrown with large hexagonal cyan and purple crystal plates, a square open pit in the centre where a control tower stood, broken white paint markings, top-down"
size_cells = [40, 22]
seamless = "none"
+++

# Setting

The Accord's forward airbase and home port, on a sunny coast at dawn. The stage begins in the carrier
MERIDIAN's wake over open sea, crosses a great concrete breakwater into the naval docks, and ends on
the airfield itself, where cyan crystal threads every crack in the runway and the control tower has
become something else. The palette is bright and clean: deep blue water (6) with light blue crests
(14), white surf and paint (1), mid and light grey concrete (12, 15), orange cranes and windsocks (8),
and the first cyan Lattice blooms (3). The stage background ink is blue (6), the sea, which also serves
as the darkest shadow ink on the quays and runways.

# Story beat

The Lattice took the Accord's own airbase first. QUILL launches from the carrier offshore to take it
back, and the player's first stage is literally reclaiming home ground.

Stage card: **STAGE 1  BREAKWATER** / "THAT'S OUR RUNWAY THEY'RE STANDING ON, CAPTAIN. GET IT BACK."

When the GANTRY QUEEN falls, the crystal across the airfield goes dark and grey (world rule 3) before
the stage-clear jingle.

# Section beats

**1 OPEN SEA (screens 0-9).** A safe, readable opening. WASP S-curves and SKIFF crossings on open
water teach movement and TWIN fire with almost no clutter; the first GANNET dives at screen 3 are the
first aimed shots. Formation leaders drop sparks generously here (five by screen 6) so the player sees
the gauge step and arms SPEED or WINGS before the docks. Checkpoint 1 at screen 6.

**2 THE DOCKS (screens 9-17).** The SEA WALL marks the change. PILLBOX turrets on the quays introduce
16-angle aimed ground fire; they are bitmap patches and can be killed before they turn. At screen 12
KEELBREAKER pulls out of its berth (the empty berth stays visible afterwards) and steams up the centre
channel; the scroll slows to 0.125 px/tick so the destroyer stays on screen. Checkpoint 2 at screen 14,
just after the midboss.

**3 THE AIRFIELD (screens 17-26).** Hangar HATCHes open to launch WASPs (kill the hatch before its
doors finish opening and it launches nothing); RUNWAY CRAWLERs follow the taxiway lines; PILLBOXes sit
in the AA pits. Checkpoint 3 at screen 20. A hidden spark sits under the third windsock pole, revealed
by shooting the pole. The crystal thickens until the apron fills the screen and the scroll stops.

# Transition set piece

The SEA WALL (screens 8-9): a straight breakwater of grey tetrapod blocks spans the full 304 px. Surf
bursts over it in three animated patch columns, timed with the music's B-section entry. A stub
lighthouse at the right end carries a cyan crystal lamp that pulses; shooting it pays 1,000. Beyond it
the water turns dark and still, and the first quay slides in.

# Midboss and boss arena

**KEELBREAKER** (screen 12) owns the docks' central channel: a 96-px-wide basin with quays either side
holding two PILLBOXes each, so the player must choose between lining up on the destroyer's deck and
clearing the quays.

**GANTRY QUEEN** (screen 24-26): the hangar apron, a 304 x 176 concrete square overgrown with hexagonal
crystal plates, with the open tower pit in its upper centre. The scroll stops with the pit at playfield
line 40. Four leg-socket patches around the pit are where the Queen's planted legs are baked. Hangars
left and right are scenery only, so the arena reads as open ground.

# Music brief

`cue.stage_s1`, energetic synth-rock at 152 BPM, 2:30, seamless loop. This is the first thing the player
hears in flight, so it must sound like launch morning: a galloping analog bass under an FM-brass lead
that opens with the talon call (root, fifth, octave), shimmering bell arpeggios like sunlight on water,
and gated drums. The open-sea main theme is wide and optimistic; the 8-bar B section tightens into
staccato brass stabs as the docks and turrets arrive; the main theme returns with rock power chords for
the airfield, the hook at full strength as the runway is retaken. Bright, clean and heroic, with
nothing grim in it.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): P2 pilot fixes
