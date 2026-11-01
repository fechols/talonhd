+++
id = "stage.s7"
kind = "level"
name = "STORMFRONT"
status = "approved"
revision = 3
slug = "stormfront"
stage_index = 7
title = "ABOVE THE CLOUDS"
theme = "dusk purple to night blue, white and light grey cloud, yellow lightning, red running lights on dark airship hulls"
length_screens = 32
scroll_q8 = 160
target_minutes = 3
midboss = "midboss.s7.squall"
boss = "boss.s7.anvil"
enemies = ["enemy.common.wasp", "enemy.s7.albatross", "enemy.s7.tempest", "enemy.s7.deck_turret", "enemy.s7.flak_battery", "enemy.common.kestrel", "enemy.common.lancer"]
checkpoints = [7, 17, 25]
music = { cue = "cue.stage_s7", mood = "the high-altitude dogfight; fastest theme, thunder in the drums, a desperate climb", bpm = 164, length_s = 150, loop = true }
background_ink = 6
card_line = "THEIR FLEET HOLDS THE STORM. GO THROUGH IT. THEN GO OVER IT."
no_ground = true

[[sections]]
n = 1
name = "CLOUD DECK"
screens = [0, 11]
description = "The top of a vast cloud layer at dusk seen from directly above: rolling white and light grey cumulus tops with soft lobes and deep purple-blue gaps between them, long shadows from the low sun, thin wisps drifting across, occasional holes showing dark blue depth far below, a pale violet sky glow tinting the cloud edges, and tiny cyan crystal motes glittering in the air."
inks_intent = "15 light grey cloud, 1 white tops, 4 purple dusk shade, 6 blue gaps"
beat = "WASP squadrons of 16, KESTREL pairs, the first ALBATROSS carrier launching WASPs; the most sprites on screen in the game."
teaches = "air-power showcase: there is nothing on the ground, so every threat is a sprite; mux flicker priority keeps bullets solid"
enemies = ["enemy.common.wasp", "enemy.common.kestrel", "enemy.s7.albatross", "enemy.common.lancer"]

[[sections]]
n = 2
name = "STORM CELL"
screens = [11, 21]
description = "Inside the top of a towering storm cell from directly above: dark grey and deep blue churning cloud masses with heavy rounded anvil tops, sheets of rain drawn as fine diagonal streaks, branching yellow lightning forks frozen inside the cloud, white flash-lit cloud edges, swirling dark vortices sinking into the cloud like drains, hail glinting in the gaps, and violet crystal motes caught in the updrafts."
inks_intent = "11 dark grey cloud, 6 blue depths, 7 yellow lightning, 15 flash-lit edges"
beat = "Lightning flashes the border and background; TEMPEST gunships push through the cloud; the SQUALL triangle hunts."
teaches = "reading bullets during screen flashes (bullets stay bright on both flash states); big multi-cell gunships"
enemies = ["enemy.s7.tempest", "enemy.common.wasp", "enemy.common.lancer", "enemy.s7.albatross"]

[[sections]]
n = 3
name = "THE FLEET"
screens = [21, 32]
description = "The Lattice sky fleet seen from directly above under a clear starry night sky: the long armoured upper decks of massive airship hulls passing below, dark grey riveted plating with white deck markings and walkway rails, rows of red running lights along the edges, round gun tubs, engine nacelles with spinning propellers, crystal masts of violet and light blue, and thin cloud far beneath them."
inks_intent = "11 dark grey hull plating, 12 mid grey deck panels, 2 dark red running lights, 4 purple crystal masts"
beat = "The hull decks are the bitmap layer: DECK TURRETs and FLAK BATTERYs sit on them; ALBATROSS and TEMPEST overhead; the flagship comes last."
teaches = "the hulls are the ground: ground patches return, but now they move as part of the scroll"
enemies = ["enemy.s7.deck_turret", "enemy.s7.flak_battery", "enemy.s7.albatross", "enemy.s7.tempest", "enemy.common.wasp", "enemy.common.kestrel"]

[[transitions]]
section = 3
set_piece = "ABOVE THE STORM: the TALON breaks out of the top of the storm cell; the churning dark cloud thins in three bands into a still, flat silver cloud sea under a clear night sky full of stars, and the bow of the first airship hull slides into view below."
asset = "setpiece.s7.above_storm"

[[waves]]
screen = 1
what = "16 WASPs in two squadrons of 8 sweeping across in opposite directions; both leaders drop sparks"

[[waves]]
screen = 3
what = "KESTREL pairs cross three times in sequence; 4 LANCERs dive through the cloud lobes"

[[waves]]
screen = 5
what = "First ALBATROSS carrier crosses slowly, launching 3 WASPs every 2 seconds until killed; drops 2 sparks"

[[waves]]
screen = 7
what = "Checkpoint 1. 16 WASPs in a single rolling ribbon from the left; 2 KESTREL pairs underneath"

[[waves]]
screen = 9
what = "2 ALBATROSSes side by side with a LANCER screen; the cloud darkens toward the storm"

[[waves]]
screen = 11
what = "Storm cell: the first lightning flash ($D021 and border to white for 2 frames); a TEMPEST gunship pushes up from below the screen and turns"

[[waves]]
screen = 13
what = "8 WASPs and 4 LANCERs between flashes; second TEMPEST from the top"

[[waves]]
screen = 15
what = "MIDBOSS SQUALL: three linked gunships in a rotating triangle; the scroll keeps running through the cloud"

[[waves]]
screen = 17
what = "Checkpoint 2. TEMPEST pair; ALBATROSS launches into the gap between them; hidden spark inside a lightning fork (shoot the brightest flash point)"

[[waves]]
screen = 19
what = "16 WASPs in four diamonds of four orbiting the player's x; lightning every 3 seconds"

[[waves]]
screen = 20
what = "ABOVE THE STORM set piece: the cloud thins, stars appear, the first hull bow slides in below"

[[waves]]
screen = 22
what = "The fleet: first hull deck with 4 DECK TURRETs and 2 FLAK BATTERYs; KESTREL pairs over the deck"

[[waves]]
screen = 25
what = "Checkpoint 3. Second hull: 6 DECK TURRETs, FLAK BATTERY burst clouds, TEMPEST overhead"

[[waves]]
screen = 27
what = "Two hulls side by side with a gap of open sky between; ALBATROSS and 16 WASPs; spark from the ALBATROSS"

[[waves]]
screen = 29
what = "The flagship's bow enters; the scroll slows to 0.125 px/tick across the hull: ANVIL"

[[tiles]]
id = "tileset.s7.cloud_deck"
section = 1
image_type = "texture"
subject = "the top of a vast cloud layer at dusk from directly above: rolling white and light grey cumulus tops, deep purple-blue gaps, violet sky glow on the edges, wisps"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s7.cloud_holes"
section = 1
image_type = "sprite"
subject = "holes in a cloud layer showing dark blue depth far below, ragged soft edges, tiny glittering crystal motes, top-down"
size_cells = [16, 12]
seamless = "none"

[[tiles]]
id = "tileset.s7.storm"
section = 2
image_type = "texture"
subject = "the churning top of a storm cell from directly above: dark grey and deep blue cloud masses, rain streaks, swirling dark vortices"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s7.lightning"
section = 2
image_type = "sprite"
subject = "branching bright yellow lightning forks frozen inside dark storm cloud with white flash-lit cloud edges around them, three shapes, top-down"
size_cells = [16, 16]
seamless = "none"

[[tiles]]
id = "setpiece.s7.above_storm"
section = 3
image_type = "vertical_scrolling_background"
subject = "a storm top thinning in straight bands that run from the left edge to the right edge of the image into a still flat silver cloud sea under a clear starry night sky, the bow of a huge dark airship hull entering at the bottom, top-down"
size_cells = [40, 22]
seamless = "none"

[[tiles]]
id = "tileset.s7.night_cloud"
section = 3
image_type = "texture"
subject = "a flat silver moonlit cloud sea far below under a dark blue night sky with scattered white stars, top-down"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s7.hull_deck"
section = 3
image_type = "sprite-tiling-vertical"
subject = "the long armoured upper deck of a massive airship hull from directly above: dark grey riveted plating, white deck markings, walkway rails, red running lights along the edges, round gun tubs, violet crystal masts"
size_cells = [24, 25]
seamless = "v"

[[tiles]]
id = "tileset.s7.hull_ends"
section = 3
image_type = "sprite"
subject = "the rounded bow and the tail fins of a huge dark grey airship hull, engine nacelles with propellers on outriggers, red running lights, top-down"
size_cells = [24, 20]
seamless = "none"

[[tiles]]
id = "setpiece.s7.flagship_hull"
section = 3
image_type = "vertical_scrolling_background"
subject = "the full-width upper hull of a flagship dreadnought airship from directly above, bow toward the top, running straight from the bottom edge to the top edge of the image: heavy armour plates, many gun turret sockets, a raised bridge tower near the stern, crystal growth along the spine"
size_cells = [38, 50]
seamless = "none"
+++

# Setting

Above the clouds there is no ground. The stage climbs from a cloud deck at dusk through a storm cell
into still night air above it, where the Lattice sky fleet holds the high altitudes. Dusk purple (4) and
night blue (6) carry the sky; white (1) and light grey (15) the cloud; yellow (7) the lightning; dark
red (2) the running lights. In the third section the **airship hulls are the bitmap layer**: dark grey
plated decks (11, 12) that pass beneath the TALON carry the ground patches (DECK TURRETs, FLAK
BATTERYs). This stage is longer (32 screens) and scrolls faster (0.625 px/tick) so it still runs about
2:50 while feeling faster than any other. The background ink is blue (6).

# Story beat

A Lattice sky fleet of overgrown airships holds the storm layer between the launch site and the
SKYHOOK's upper reaches. The TALON must go through the weather and then over it.

Stage card: **STAGE 7  STORMFRONT** / "THEIR FLEET HOLDS THE STORM. GO THROUGH IT. THEN GO OVER IT."

# Section beats

**1 CLOUD DECK (screens 0-11).** The air-power showcase: WASP squadrons of 16, KESTREL pairs, LANCER
dives and the ALBATROSS carrier, which launches WASPs until killed. With no ground layer, the
multiplexer carries everything. Formations are spaced vertically so no line exceeds the budget, and mux
priority (player > boss > player shots > enemy bullets > enemies) means enemies flicker first.
Checkpoint 1 at screen 7.

**2 STORM CELL (screens 11-21).** Lightning flashes switch `$D021` and the border to white for 2 frames
every few seconds. Enemy bullets use outlined shapes that read on both states. TEMPEST gunships (B 4x3)
push through the cloud. The SQUALL midboss at screen 15 fights on a running scroll. Checkpoint 2 at
screen 17.

**3 THE FLEET (screens 21-32).** Above the storm, airship hulls pass below as bitmap strips with sky
and cloud between them. DECK TURRETs and FLAK BATTERYs bring ground patches back, ALBATROSS and TEMPEST
fly overhead. Checkpoint 3 at screen 25. The flagship ANVIL closes the stage.

# Transition set piece

ABOVE THE STORM (screens 20-21): the churning storm thins in three bands of cloud and the lightning
stops. A still, flat silver cloud sea opens under a clear sky full of stars, and the bow of the first
airship hull slides in below. It is the only quiet moment in the stage: four seconds with no enemies,
timed to the music's return from the B section.

# Midboss and boss arena

**SQUALL** (screen 15): open storm cloud on a running scroll. The midboss holds its triangle near the
top of the screen, so the lower two thirds of the playfield stay free for dodging. Lightning keeps
flashing during the fight.

**ANVIL** (screens 29-32): the flagship's upper hull fills the full 304-px width for about two screens.
The scroll slows to 0.125 px/tick across it, so the turret patches come into range a few at a time.
When the bridge tower reaches mid-screen the scroll stops and the bridge core lifts off as a sprite.

# Music brief

`cue.stage_s7`, high-speed synth-rock at 164 BPM, 2:30, seamless loop: the fastest stage theme in the
game. It is a high-altitude dogfight with a racing sixteenth-note synth bass, a screaming FM brass lead,
rock guitar power chords and string-like synth pads for altitude. The cloud-deck main theme soars and
urges. The 8-bar B section brings thunderous gated tom fills and darker chords for the storm cell,
landing with the lightning. The final main theme has guitar and brass at full drive over the fleet. The
rising talon call should sound like the climb itself, out of the storm and over the enemy.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): P2 pilot fixes
