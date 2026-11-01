+++
id = "stage.s4"
kind = "level"
name = "SLUICEWORKS"
status = "approved"
revision = 3
slug = "sluiceworks"
stage_index = 4
title = "INDUSTRIAL CANAL DISTRICT"
theme = "steel greys and canal blue, safety yellow and red warning lamps, cyan crystal veins glowing inside the pipes"
length_screens = 26
scroll_q8 = 128
target_minutes = 3
midboss = "midboss.s4.lockmaster"
boss = "boss.s4.hydra_pump"
enemies = ["enemy.s4.gnat", "enemy.s4.heron", "enemy.s4.valve_turret", "enemy.s4.pressure_tank", "enemy.s4.fan_pit", "enemy.common.wasp", "enemy.common.kestrel", "enemy.common.pillbox"]
checkpoints = [6, 14, 20]
music = { cue = "cue.stage_s4", mood = "pressurised machinery, syncopated pistons, cool blue water, a sleek melody above the clank", bpm = 132, length_s = 150, loop = true }
background_ink = 11
card_line = "THE TRAINS FED SOMETHING IN THE CANALS. DRAIN IT."

[[sections]]
n = 1
name = "PIPE YARDS"
screens = [0, 9]
description = "An industrial pipe yard from directly above: dense tangles of grey steel pipe of every diameter running in parallel bundles and elbow bends, red hand-wheel valves, bolted flanges, squat round pressure tanks with yellow hazard bands and gauges, steel catwalks with grating, rust-streaked puddles of oily water reflecting the sky, and veins of cyan crystal glowing through cracked pipe joints and seams."
inks_intent = "12 mid grey pipes, 11 dark grey shadow, 7 yellow hazard bands, 3 cyan crystal glow"
beat = "VALVE TURRETs on the pipe junctions, PRESSURE TANKs that burst into rings when destroyed, the first GNAT swarm."
teaches = "do not kill a PRESSURE TANK when standing next to it; bucket swarms of tiny drones"
enemies = ["enemy.s4.valve_turret", "enemy.s4.pressure_tank", "enemy.s4.gnat", "enemy.common.wasp"]

[[sections]]
n = 2
name = "CANAL LOCKS"
screens = [9, 17]
description = "A stepped canal of lock chambers seen from above: bright blue water channels between tall blocks of grey machine-panel concrete with yellow and black hazard stripes along every edge, massive steel lock gates with walkways across their tops, red warning lamps on posts, mooring rings, a lock-keeper's control hut with a green-glowing panel, and cyan crystal creeping along the waterline."
inks_intent = "6 blue canal water, 12 mid grey concrete, 7 yellow hazard stripes, 2 dark red lamps"
beat = "HERON hover-skimmers ride the canals; the lock gates of the LOCKMASTER close in from both sides."
teaches = "enemies that follow water channels; a midboss that shrinks the safe space"
enemies = ["enemy.s4.heron", "enemy.s4.valve_turret", "enemy.s4.gnat", "enemy.common.kestrel"]

[[sections]]
n = 3
name = "PUMP HALL"
screens = [17, 26]
description = "The roof-off interior of a vast pump hall seen from directly above: rows of round turbine pits with spinning impellers, huge vent fans in square steel frames, thick bundled cables in trays, yellow overhead crane rails, chequer-plate steel floor with painted walkways, red warning lamps, and the crystal veins now thick, cyan and pulsing as they converge on the central pump."
inks_intent = "11 dark grey floor, 12 mid grey machinery, 7 yellow rails, 3 cyan crystal"
beat = "FAN PITs drag player shots aside; PRESSURE TANKs and VALVE TURRETs in rows; every crystal vein leads to the pump."
teaches = "shot-bending hazards: line up so the drag helps; prioritise under pressure"
enemies = ["enemy.s4.fan_pit", "enemy.s4.pressure_tank", "enemy.s4.valve_turret", "enemy.s4.gnat", "enemy.s4.heron", "enemy.common.pillbox"]

[[transitions]]
section = 3
set_piece = "The LOCK GATE: a final pair of massive steel mitre gates swings open as the player approaches while the water level in the chamber drops in animated steps, exposing wet green-black walls, a drained channel floor and the pump hall's intake grilles beyond."
asset = "setpiece.s4.lock_gate"

[[waves]]
screen = 1
what = "6 WASPs in an S over the pipes; leader drops a spark; 2 VALVE TURRETs on a junction"

[[waves]]
screen = 2
what = "First GNAT swarm: 12 drones drawn in bucket sprites, a loose cloud that drifts toward the player; one shot kills each"

[[waves]]
screen = 3
what = "3 PRESSURE TANKs in a row with a VALVE TURRET behind them: shoot the tanks from range to clear the turret with their rings"

[[waves]]
screen = 5
what = "2 GNAT swarms from opposite corners; 4 VALVE TURRETs along a pipe bundle"

[[waves]]
screen = 6
what = "Checkpoint 1. KESTREL pairs cross twice; 3 PRESSURE TANKs; spark from the second pair's leader"

[[waves]]
screen = 8
what = "The pipes end at the first lock; 3 HERONs launch along the channel"

[[waves]]
screen = 10
what = "HERONs ride two parallel canals in opposite directions firing sideways pairs; 2 VALVE TURRETs on the gate walkways"

[[waves]]
screen = 12
what = "MIDBOSS LOCKMASTER: two gate halves close in from the sides with 3 turrets each; the scroll stops"

[[waves]]
screen = 14
what = "Checkpoint 2. GNAT swarm plus 4 HERONs; hidden spark on the lock-keeper's hut roof (shoot the panel)"

[[waves]]
screen = 16
what = "LOCK GATE set piece; the chamber drains in steps; 4 VALVE TURRETs on the walls fire as the water drops"

[[waves]]
screen = 18
what = "Pump hall: 2 FAN PITs either side of a line of 3 PRESSURE TANKs; shots drag toward the fans"

[[waves]]
screen = 20
what = "Checkpoint 3. 2 GNAT swarms and 6 WASPs at once; 2 PILLBOXes in the turbine pits"

[[waves]]
screen = 22
what = "Rows of 4 FAN PITs and 4 VALVE TURRETs; KESTREL pair; HERONs in the cable trays"

[[waves]]
screen = 24
what = "The crystal veins meet at the central pump pit; the scroll stops; HYDRA PUMP raises its heads"

[[tiles]]
id = "tileset.s4.pipes"
section = 1
image_type = "texture"
subject = "dense bundles of grey steel pipes from directly above with elbow bends, bolted flanges and red hand-wheel valves, cyan crystal glowing through cracked joints"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s4.pipe_props"
section = 1
image_type = "sprite"
subject = "squat round pressure tanks with yellow hazard bands and dial gauges, steel grating catwalks, oily puddles, top-down"
size_cells = [20, 16]
seamless = "none"

[[tiles]]
id = "tileset.s4.canal"
section = 2
image_type = "sprite-tiling-vertical"
subject = "bright blue canal channels between tall grey machine-panel concrete blocks with yellow and black hazard stripes on every edge, mooring rings, red lamps on posts, top-down"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s4.lock_props"
section = 2
image_type = "sprite"
subject = "massive steel lock gates with a walkway on top, a lock-keeper's control hut with a green glowing panel, bollards, top-down"
size_cells = [24, 14]
seamless = "none"

[[tiles]]
id = "setpiece.s4.lock_gate"
section = 3
image_type = "vertical_scrolling_background"
subject = "a pair of huge steel mitre lock gates spanning the canal from the left edge to the right edge of the image, once, across the middle, swinging open over a draining chamber, wet green-black walls, a drained channel floor and intake grilles beyond, top-down"
size_cells = [40, 20]
seamless = "none"

[[tiles]]
id = "tileset.s4.pump_floor"
section = 3
image_type = "texture"
subject = "roof-off pump hall floor from directly above: chequer-plate steel with painted yellow walkways, cable trays, overhead crane rails, thick cyan crystal veins"
size_cells = [40, 25]
seamless = "v"

[[tiles]]
id = "tileset.s4.fans"
section = 3
image_type = "sprite"
subject = "huge vent fans in square steel frames and round turbine pits with impellers, red warning lamps on the corners, top-down"
size_cells = [16, 16]
seamless = "none"

[[tiles]]
id = "setpiece.s4.pump_pit"
section = 3
image_type = "vertical_scrolling_background"
subject = "a vast round central pump pit with five pipe sockets around its rim, crystal veins converging into it from every direction, steel floor around, top-down"
size_cells = [40, 22]
seamless = "none"
+++

# Setting

A flooded industrial district where the rail freight ended. Pipe yards in grey steel, a staircase of
canal locks in bright blue between hazard-striped machine blocks, and a roofless pump hall where every
cyan crystal vein converges on the relay the Lattice is building. Steel greys (11, 12), canal blue (6),
safety yellow (7) and dark red warning lamps (2) carry the human side; the crystal (3) glows from inside
the pipes, the stage's cleverest image: the Lattice is literally in the plumbing. The background ink is
dark grey (11).

# Story beat

The power trains fed this district. The Lattice is building a relay inside the pumping station, using
the canals to move its power. QUILL is sent to drain it.

Stage card: **STAGE 4  SLUICEWORKS** / "THE TRAINS FED SOMETHING IN THE CANALS. DRAIN IT."

# Section beats

**1 PIPE YARDS (screens 0-9).** VALVE TURRETs sit on junctions; PRESSURE TANKs burst into radial rings
when destroyed, which can clear nearby turrets for you or kill you if you are close. GNAT swarms (12
drones drawn through bullet-bucket composites) appear for the first time: a cloud that costs few sprite
channels and dies to one shot each. Checkpoint 1 at screen 6.

**2 CANAL LOCKS (screens 9-17).** HERON hover-skimmers ride the canal channels exactly, firing sideways
pairs. The LOCKMASTER midboss at screen 12 closes two gate halves from the sides, shrinking the safe
space column by column. Checkpoint 2 at screen 14; hidden spark on the lock-keeper's hut.

**3 PUMP HALL (screens 17-26).** FAN PITs drag player shots sideways as they pass over, so players learn
to fire from the fan's upwind side. Rows of tanks and turrets test priorities. Checkpoint 3 at screen
20. The crystal veins converge and pulse faster toward the central pit.

# Transition set piece

The LOCK GATE (screens 16-17): the final chamber's mitre gates swing open as the player arrives, and
the water level drops in four animated patch steps (12 ticks each), exposing wet dark walls and the
drained channel floor. VALVE TURRETs on the chamber walls are revealed as the water falls, so the
player fights through the transition. The music's B section ends here.

# Midboss and boss arena

**LOCKMASTER** (screen 12): a wide lock chamber, full width, with the gate recesses in both side walls.
The scroll stops. The gate halves close in 8-px steps to leave a 96-px gap, then reopen as they take
damage.

**HYDRA PUMP** (screens 24-26): the central pump pit, its core in the upper half of the screen with five
pipe sockets around the rim. The scroll stops with the core at playfield line 48. Five floor lanes
radiate from the pit; when a head dies its lane floods with a timed hazard patch.

# Music brief

`cue.stage_s4`, industrial synth-funk at 132 BPM, 2:30, seamless loop. Pressurised machinery with a cool
head: a syncopated slap-style synth bass, metallic clanking percussion like pistons, and hissing noise
hi-hats under a sleek FM bell lead that glides above the clank. FM brass stabs carry the rising talon
call. The pipe-yard main theme is mechanical and confident. The 8-bar B section filters the bass and adds
water-drip arpeggios for the canal locks. The main theme returns with brass stabs and full drive for the
pump hall. Cool blue, precise, a gated snare that hits like a valve slamming shut.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): P2 pilot fixes
