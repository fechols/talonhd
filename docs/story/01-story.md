+++
id = "story"
kind = "story"
name = "TALON STORY"
status = "approved"
revision = 1
twist_after_stage = 5
acts = ["backstory", "inciting", "continent", "twist", "ascent", "crown", "ending", "loop"]

[[plot_points]]
id = "pp.landfall"
trigger = "game start (after PRESS FIRE on the title)"
screen = "intro-01..03"
consequence = "launch card, then stage 1"

[[plot_points]]
id = "pp.launch"
trigger = "intro-03 dismissed, loop 1 only"
screen = "launch"
consequence = "stage card S1; the TALON is catapulted from MERIDIAN"

[[plot_points]]
id = "pp.stage_card"
trigger = "every stage start, every loop"
screen = "stage-card (template) with the stage's BRANDT line"
consequence = "stage begins at screen 0"

[[plot_points]]
id = "pp.node_dark"
trigger = "any stage boss defeated (S1-S7)"
screen = "stage-clear (template) with the stage's clear line"
consequence = "the stage's crystal palette is swapped to its dark variant during the fly-out; bonus tally; next stage card"

[[plot_points]]
id = "pp.twist"
trigger = "boss.s5.quadrant defeated"
screen = "stage-clear S5, then twist-01..04"
consequence = "stages 6-8 are 'the ascent'; the stage cards from S6 on carry the CROWN glyph strip"

[[plot_points]]
id = "pp.storm_broken"
trigger = "boss.s7.anvil defeated"
screen = "stage-clear S7, then ascent-01"
consequence = "S8 stage card"

[[plot_points]]
id = "pp.crown_heart"
trigger = "boss.s8.crown phase 1 (the Ring) defeated"
screen = "none (in-engine: crown-caption, 120 ticks over the ring breaking)"
consequence = "phase 2, the Heart; cue.final_2"

[[plot_points]]
id = "pp.ending"
trigger = "boss.s8.crown phase 2 defeated"
screen = "ending-01..04, credits"
consequence = "loop card"

[[plot_points]]
id = "pp.loop"
trigger = "credits finished"
screen = "loop-card"
consequence = "loop counter +1, rank base raised (see docs/design.md 12), stage card S1 with LOOP line; score, lives and gauge carry over"
+++

# TALON — story

## 1 Backstory

For forty years the free nations kept the peace with a joint air command they called the **Meridian
Accord**: a carrier group, a dozen coastal airbases, and a habit of building one of everything and
testing it until it broke. The **XF-9 TALON** was the last of those: a hand-built delta prototype with
analogue flight controls, modular hard-points and a pilot who refused to let the programme be cancelled.
It sat in a hangar at **BREAKWATER**, the Accord's forward airbase, waiting for a budget.

In the spring of 2091 a meteor swarm crossed the sky over three continents. Most of it burned up. What
reached the ground was not rock. It was **the LATTICE**: a crystalline machine intelligence that grows
rather than builds. It did not destroy what it found. It *took it over*. Faceted blue-violet crystal
threaded through runways, vine rows, rail lines, canal locks and fortress walls, and every place it
claimed became a **node**: armed, fortified, and humming as it sent power along the old human lines
toward something no one could yet see.

Every guided weapon the Accord fired at a node forgot where it was going. The Lattice's silence jams
electronics; radar sees snow; missiles wander. Within a month the Accord had lost its forward bases and
most of its aircraft, and the crystal had reached the mountains.

## 2 Inciting incident

BREAKWATER fell first, in a single night. The ground crew pulled one aircraft out of the hangar before
the crystal came through the doors: the TALON, on a flatbed, under a tarpaulin, towed onto the carrier
**MERIDIAN** as she left harbour.

On the carrier, the engineers found two things. First, the TALON's analogue avionics fly straight through
the Lattice's silence; the jamming has nothing to grab. Second, downed Lattice drones drop **SPARKS**:
small crystal energy cells that the TALON's hard-points can accept. A spark does not do anything by
itself. It charges the **arming gauge**, and the pilot decides which system gets the power.

**Marshal Ida BRANDT**, commanding what is left of the Accord's air arm, approves one sortie. The test
pilot, **Captain Noor Vasquez**, callsign **QUILL**, has been waiting for this flight for three years.
At dawn the MERIDIAN turns into the wind off the coast, and the catapult fires.

## 3 Stage-card framing

The game is one continuous sortie in eight legs. Before each leg the screen shows a **stage card**: the
stage number and name, the theatre subtitle, and a one- or two-line brief from BRANDT over the radio.
BRANDT is dry, brief and never wrong about the target. After each boss falls the **stage-clear card**
shows the crystal in that stage going dark, the bonus tally, and one line of radio between QUILL and
BRANDT.

| Stage | Name | Subtitle | BRANDT's brief |
|---|---|---|---|
| S1 | BREAKWATER | COASTAL AIRBASE AT DAWN | THAT'S OUR RUNWAY THEY'RE STANDING ON, CAPTAIN. GET IT BACK. |
| S2 | TERRACES | HILLSIDE VINEYARDS | THEY TURNED A TELESCOPE INTO A GUN. CLOSE THE OBSERVATORY, QUILL. |
| S3 | REDLINE | DESERT CANYON RAILWAY | EVERY TRAIN ON THAT LINE CARRIES THEIR POWER. CUT THE LINE. |
| S4 | SLUICEWORKS | INDUSTRIAL CANAL DISTRICT | THE TRAINS FED SOMETHING IN THE CANALS. DRAIN IT. |
| S5 | BASTION | THE CITADEL WALLS | THE CITADEL IS THE LAST WALL ON THE CONTINENT. BRING IT DOWN. |
| S6 | VERDIGRIS | JUNGLE RUINS | THE OLD LAUNCH SITE IS UNDER ALL THAT GREEN. FIND US A WAY UP. |
| S7 | STORMFRONT | ABOVE THE CLOUDS | THEIR FLEET HOLDS THE STORM. GO THROUGH IT. THEN GO OVER IT. |
| S8 | SKYHOOK | THE ORBITAL ELEVATOR | THE CABLE STILL HOLDS. FOLLOW IT ALL THE WAY UP, QUILL. |

The briefs are copied verbatim from each level file's `card_line` (`04-levels/*.md`); the level file is
the source of truth and the card only wraps the line at a word boundary to fit 38 columns. S5's
`twist_line` ("THEN WE GO UP AFTER IT.") closes twist-04. The briefs are deliberately short: the stage card is on screen for 150 ticks and the player wants to
fly. On loop 2 and later the card adds a `LOOP {L}` line and BRANDT's lines are unchanged (she does not
know about loops; the Lattice does).

## 4 The continent (stages 1-4)

- **S1 BREAKWATER.** QUILL retakes the Accord's own airbase: open sea, the docks, and the runway with
  crystal threading the tarmac cracks. The control tower has been recrystallised into the walker
  GANTRY QUEEN. When it falls, the crystal across the airbase goes dark and the Accord has a runway.
- **S2 TERRACES.** Inland, the Lattice has turned a ridge-top observatory's telescope into a sky cannon
  that guards the valley. Vineyards, a hill village, the observatory: ASTROLABE.
- **S3 REDLINE.** The Lattice moves harvested power to the coast by rail. Desert, canyon, freight yard;
  the crane-and-engine TRESTLE TITAN straddles the tracks.
- **S4 SLUICEWORKS.** The trains fed a flooded industrial district where the Lattice is building a relay.
  Pipes, canal locks, the pump hall and the HYDRA PUMP.

Each cleared stage is a node gone dark. The player sees it: the crystal palette in that stage's bitmap
swaps to its dark variant during the fly-out (a colour change only, no new art).

## 5 Mid-game twist (after stage 5)

**S5 BASTION** is the continental stronghold, a star fort grown over with crystal walls, and every
power line the TALON has cut so far ran here. When QUADRANT's last dome falls, the stage-clear card
plays as usual. Then the domes go dark, and one does not: it fires a single pencil of light straight
up and keeps firing.

The Lattice speaks for the first time, on a **glyph screen**: rows of hexagonal and triangular glyphs
assembling into an arrow that points up. The MERIDIAN's signals crew decodes one line of it. The
Citadel was never the heart of anything. It was a **relay**, and all the power of the captured continent
has been going **up**, to a structure in orbit that the Accord now names the **CROWN**.

BRANDT: "Then we go up after it."

From this point the stage cards carry a thin strip of Lattice glyphs under the stage name, and the
in-game sky is a little darker every stage.

## 6 The ascent and the CROWN (stages 6-8)

- **S6 VERDIGRIS.** To reach orbit the Accord needs the old equatorial launch site, deep in jungle ruins
  the Lattice has seeded thickly with crystal flowers. The colossal crystal flower ORCHID guards the
  rusted launch gantry.
- **S7 STORMFRONT.** A Lattice sky fleet of captured airships holds the storm layer above the elevator.
  There is no ground: the cloud deck and the decks of the airship hulls take its place. The flagship
  ANVIL is the last thing between the TALON and the cable.
- **S8 SKYHOOK.** The TALON climbs the SKYHOOK, the equatorial space elevator, flying straight up its
  cable. The sky goes cobalt, then black; the stars come out; Earth's curve falls away below. At the top
  hangs the **CROWN**, a ring station of Lattice crystal drinking the light of every relay at once.
  - Phase 1, **the Ring**: eight facets open in turn and fire.
  - Phase 2, **the Heart**: the ring breaks, and the crystal heart inside hunts the TALON directly.

## 7 Ending

The Heart shatters. The CROWN breaks into a thousand pieces that fall as shooting stars over the dawn
side of the Earth. Across the world, node by node, the crystal goes dark: the runway at BREAKWATER, the
observatory, the rail yard, the canals, the Citadel. At BREAKWATER the ground crew run the Accord flag
back up the pole. The TALON comes down out of the morning and lands on the MERIDIAN's deck with its
spark racks empty. BRANDT is waiting on the deck. The last card reads **THE SKY IS OURS AGAIN.**

After the credits the loop card plays: **THE LATTICE REMEMBERS. IT HAS GROWN.** Somewhere a fragment of
the CROWN did not burn up. Loop 2 is the same war against a stronger remnant: faster bullets, more of
them, and crystal-plated enemies (see `docs/design.md` 12).

## 8 Plot-point table

See the front matter `[[plot_points]]`. Every trigger is an event the engine already emits (game start,
stage start, boss defeated, boss phase change, credits finished); the story adds no new game states.

| Plot point | Trigger | Screen |
|---|---|---|
| pp.landfall | game start | intro-01..03 |
| pp.launch | loop 1, before S1 | launch |
| pp.stage_card | every stage start | stage-card |
| pp.node_dark | boss S1-S7 defeated | stage-clear |
| pp.twist | QUADRANT defeated | stage-clear S5, twist-01..04 |
| pp.storm_broken | ANVIL defeated | stage-clear S7, ascent-01 |
| pp.crown_heart | CROWN ring defeated | crown-caption (in-engine) |
| pp.ending | CROWN heart defeated | ending-01..04, credits |
| pp.loop | credits finished | loop-card |

## 9 Text-screen scripts

Rules (identical for every screen):

- 38 columns, uppercase, PETSCII-safe, at most 8 lines per screen.
- 150 ticks per screen or Fire to skip one screen; RUN/STOP skips the whole sequence.
- A speaker's name sits on its own line, followed by a colon.
- Stage directions are in parentheses and are shown, not spoken.
- Glyph lines are written as `#0`..`#F` tokens: each token is one Lattice glyph from the glyph charset
  in `02-characters/char-lattice.md`, two columns wide and one row tall (so the written width is the
  displayed width), drawn in cyan and purple. A glyph line holds at most 19 glyphs.
- Portrait cards (QUILL, BRANDT) are optional art; with no portrait the text centres.
- Each screen types on at 2 characters per tick with `sfx.text_type`.

### intro-01
```
2091.

THE LATTICE FELL WITH THE METEORS.
```

### intro-02
```
IT GREW THROUGH OUR AIRBASES,
OUR VINEYARDS, OUR RAILWAYS,
OUR CITIES.

IT DID NOT DESTROY. IT TOOK.
```

### intro-03
```
ITS SILENCE JAMS EVERY GUIDED WEAPON.

ONE AIRCRAFT CAN STILL FLY THROUGH
ITS SILENCE:

THE XF-9 TALON.
```

### launch (loop 1 only, before the first stage card)
```
CARRIER MERIDIAN. 0500 HOURS.

QUILL:
TALON IS HOT. SPARK RACKS EMPTY.

BRANDT:
THEN FILL THEM ON THE WAY IN.
```

### stage-card (template; filled from the level file)
```
{LOOP_LINE}
STAGE {N}
{NAME}
{SUBTITLE}
{GLYPH_STRIP}
BRANDT:
{BRIEF_1}
{BRIEF_2}
```
- `{LOOP_LINE}` is blank on loop 1 and `LOOP {L}` from loop 2 on.
- `{GLYPH_STRIP}` is blank for S1-S5 and `#A#B#C#B#A` (the CROWN glyph) from S6 on.
- `{BRIEF_2}` may be blank. The card holds 150 ticks or until Fire; `cue.briefing` plays under it.

### stage-card lines

#### S1
```
STAGE 1
BREAKWATER
COASTAL AIRBASE AT DAWN

BRANDT:
THAT'S OUR RUNWAY THEY'RE STANDING
ON, CAPTAIN. GET IT BACK.
```

#### S2
```
STAGE 2
TERRACES
HILLSIDE VINEYARDS

BRANDT:
THEY TURNED A TELESCOPE INTO A GUN.
CLOSE THE OBSERVATORY, QUILL.
```

#### S3
```
STAGE 3
REDLINE
DESERT CANYON RAILWAY

BRANDT:
EVERY TRAIN ON THAT LINE CARRIES
THEIR POWER. CUT THE LINE.
```

#### S4
```
STAGE 4
SLUICEWORKS
INDUSTRIAL CANAL DISTRICT

BRANDT:
THE TRAINS FED SOMETHING IN THE
CANALS. DRAIN IT.
```

#### S5
```
STAGE 5
BASTION
THE CITADEL WALLS

BRANDT:
THE CITADEL IS THE LAST WALL ON THE
CONTINENT. BRING IT DOWN.
```

#### S6
```
STAGE 6
VERDIGRIS
JUNGLE RUINS
#A#B#C#B#A
BRANDT:
THE OLD LAUNCH SITE IS UNDER ALL
THAT GREEN. FIND US A WAY UP.
```

#### S7
```
STAGE 7
STORMFRONT
ABOVE THE CLOUDS
#A#B#C#B#A
BRANDT:
THEIR FLEET HOLDS THE STORM. GO
THROUGH IT. THEN GO OVER IT.
```

#### S8
```
STAGE 8
SKYHOOK
THE ORBITAL ELEVATOR
#A#B#C#B#A
BRANDT:
THE CABLE STILL HOLDS. FOLLOW IT
ALL THE WAY UP, QUILL.
```

### stage-clear (template)
```
{NAME} IS OURS.
THE CRYSTAL GOES DARK.

{SPEAKER}:
{CLEAR_LINE}

{BONUS_TALLY}
```
`{BONUS_TALLY}` is the tally block from `07-ui-text.md` (boss time bonus, no-miss, sweep, chain). The
card holds until the tally finishes counting plus 90 ticks, or Fire. `cue.stage_clear` plays under it.

### stage-clear lines

| Stage | Speaker | Line |
|---|---|---|
| S1 | BRANDT | `GROUND CREW IS ALREADY ON THE APRON.` |
| S2 | QUILL | `TELESCOPE'S CLOSED. NICE VIEW, THOUGH.` |
| S3 | BRANDT | `NO MORE TRAINS. GOOD. I HATE TRAINS.` |
| S4 | QUILL | `PUMPS ARE DRY. SO IS MY FLIGHT SUIT.` |
| S5 | BRANDT | `THE CITADEL IS DOWN. STAND BY...` |
| S6 | QUILL | `LAUNCH PAD'S CLEAR. RUSTY, BUT CLEAR.` |
| S7 | BRANDT | `THE FLEET IS DOWN. THE CABLE IS YOURS.` |

S8 has no stage-clear card: the ending follows the CROWN directly.

### twist-01 (after stage-clear S5)
```
(FOUR DOMES GO DARK.
ONE DOES NOT.

A PENCIL OF LIGHT CLIMBS
STRAIGHT UP FROM THE CITADEL,
AND KEEPS CLIMBING.)
```

### twist-02 (glyph screen; the glyphs assemble one row per 20 ticks)
```
#1#1#1#1#1#1#1#1#1
   #4#5#6#5#4
      #7#8#7
         #9

#A#B#C#B#A
```
Visual: the rows form an upward-pointing arrow of hexagons with the CROWN glyph under it. No caption.
The screen flashes the border light blue once as the last glyph lands. `sfx.lattice_shimmer` plays.

### twist-03
```
MERIDIAN SIGNALS, DECODED:

"RELAY FIVE OF FIVE: LOST.
 THE CROWN IS FED. THE CROWN
 DOES NOT NEED THE GROUND."
```

### twist-04
```
QUILL:
THE CITADEL WAS A RELAY.
SOMETHING ABOVE US IS DRINKING
THE LIGHT.

BRANDT:
THEN WE GO UP AFTER IT.
```

### ascent-01 (after stage-clear S7)
```
THE STORM BREAKS.
ABOVE IT THE SKY IS STILL, AND
FULL OF STARS.

ONE CABLE RUNS STRAIGHT UP INTO
THE DARK.
```

### crown-caption (in-engine over the Ring breaking, 120 ticks, no skip)
```
#C#C#C#C#C#C#C#C#C
```
A single glyph row across the top text row, flickering cyan and purple while the ring breaks apart in
the playfield; then the HUD returns and the Heart fight begins.

### ending-01
```
THE CROWN BROKE INTO A THOUSAND
FALLING STARS.

ACROSS THE WORLD, NODE BY NODE,
THE CRYSTAL WENT DARK.
```

### ending-02
```
AT BREAKWATER THE GROUND CREW RAN
THE ACCORD FLAG BACK UP THE POLE.

NOBODY GAVE THE ORDER.
```

### ending-03
```
CARRIER MERIDIAN. DAWN.

QUILL:
TALON ON DECK. RACKS EMPTY.

BRANDT:
WELCOME HOME, CAPTAIN.
```

### ending-04 (held 300 ticks, no skip)
```
THE SKY IS OURS AGAIN.
```

### credits (scroll at 0.5 rows per 8 ticks; names supplied by the user)
```
TALON

AN ORIGINAL GAME FOR THE
64 MHZ C64 ULTIMATE

DESIGN AND STAGES
{NAME}

CODE
{NAME}

ARTWORK GENERATED WITH LUDO.AI
REDUCED AND PLACED BY {NAME}

MUSIC
{NAME}

SOUND EFFECTS
{NAME}

TESTED BY
{NAME}

THANK YOU FOR FLYING.
```

### loop-card (after the credits; 240 ticks, no skip)
```
#A#B#C#B#A

THE LATTICE REMEMBERS.
IT HAS GROWN.

LOOP {L}
```
After the card, the S1 stage card plays with its `LOOP {L}` line. The launch card is not repeated.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
