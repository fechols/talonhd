+++
id = "world"
kind = "world"
name = "TALON"
status = "approved"
revision = 2
title = "TALON"
tagline = "THE SKY IS OURS AGAIN."
era = "1986 arcade-cabinet retro-future military, year 2091"
twist_after_stage = 5

[factions.accord]
id = "accord"
name = "THE MERIDIAN ACCORD"
short = "Accord"
creed = "Hold the line. Hold the sky."
emblem = "a white chevron over a horizontal bar inside a circle (abstract, no national symbols)"
colors = [1, 12, 15, 7]
vehicles = "late-1980s retro-future military: carriers, airbases, cranes, rail freight, radar domes"
naming = "short military words and birds of prey: TALON, QUILL, BRANDT, MERIDIAN, SKYHOOK"
speech = "terse radio brevity, dry humour, never cruel"

[factions.lattice]
id = "lattice"
name = "THE LATTICE"
short = "Lattice"
creed = "(it does not speak; it arranges)"
emblem = "none; it leaves hexagonal crystal growth on what it claims"
colors = [3, 14, 4, 1, 6]
vehicles = "none of its own; it overgrows and repurposes human machines, and grows faceted drones"
naming = "names given by the Accord: nature and weather words, NATO-style reporting names (WASP, GANNET, ORCHID)"
speech = "none; geometric glyph screens"

[palette]
c0 = ["black", "#000000"]
c1 = ["white", "#F7F7F7"]
c2 = ["dark red", "#8D2F34"]
c3 = ["cyan", "#6AD4CD"]
c4 = ["purple", "#9835A4"]
c5 = ["green", "#4CB442"]
c6 = ["blue", "#2C29B1"]
c7 = ["yellow", "#EFEF5D"]
c8 = ["orange", "#984E20"]
c9 = ["brown", "#5B3800"]
c10 = ["light red", "#D1676D"]
c11 = ["dark grey", "#4A4A4A"]
c12 = ["mid grey", "#7B7B7B"]
c13 = ["light green", "#9FEF93"]
c14 = ["light blue", "#6D6AEF"]
c15 = ["light grey", "#B2B2B2"]

[hardware]
screen = [320, 200]
safe_x = [8, 311]
hud_text_rows = { top = 0, bottom = 24 }
playfield = "multicolour bitmap, 24-row mode, YSCROLL fine scroll, REU-streamed view"
bitmap_cell = "4x8 art px (8x8 display); three free inks per cell plus the stage background ink"
sprite_cell_art_px = [12, 21]
size_classes = { S = [1, 1], W = [2, 1], M = [2, 2], L = [3, 3], B = [4, 8] }
sprite_expansion = "never used: VIC-II sprite expansion is banned and $D017/$D01D stay 0; bigger objects use bigger grids"
sprites_per_line = 8
shared_sprite_pair = "one static $D025/$D026 pair for the whole game, chosen by the quantizer over every frame"
hero_private_inks = [1, 3]
ticks_per_second = 60
+++

# TALON — world bible

## 1 Premise

In 2091 a crystalline machine intelligence, **the LATTICE**, arrives inside a meteor swarm. It does not
destroy what it finds; it *takes it over*. Glassy blue-violet growth threads through airbases,
vineyards, rail lines, canals and fortresses, turning each into a fortified node that sends power to a
growing orbital structure, **the CROWN**. Lattice nodes jam every guided weapon, so conventional air
forces cannot get close.

The **Meridian Accord**, the free nations' joint air command, has one answer: the **XF-9 TALON**, a
hand-built prototype flying analogue avionics the jamming cannot touch. Its modular hard-points accept
**SPARKS**, salvaged Lattice energy cells that downed nodes drop. Each spark cycles the TALON's **arming
gauge**; the pilot chooses when to commit power to a system.

## 2 Tone

A 1986 arcade cabinet brought to life. Sunny, bold, confident military hardware against cold, beautiful
alien geometry. Heroic and bright, never grim, never gory: no people are ever shown hurt, and every
Lattice unit is a machine. Explosions are big, round and yellow. The alien growth is pretty and wrong.
Humour is dry radio banter on the stage cards.

## 3 Era aesthetic rules

- **Accord hardware:** late-1980s retro-future. White, light grey and mid grey paint, hazard stripes,
  chunky panel lines and rivets, CRT-green instrument glow, orange windsocks and cranes. No real
  aircraft types, no national markings, no real logos.
- **Lattice growth:** faceted crystal in cyan, light blue and purple with white glints. Always
  hexagonal or triangular, always crisp-edged. Never organic, never fleshy, never eyes or faces.
- **Captured machines:** Accord shapes with crystal grown through seams, panels replaced by facets,
  running lights turned cyan.
- **Places:** every stage is a recognisable human place being overgrown: an airbase, vineyards, a
  railway, canals, a star fort, jungle ruins, a sky fleet, a space elevator.
- **Viewpoint:** everything is seen from directly above (orthographic top-down), lit from the upper left,
  with short cast shadows to the lower right.

## 4 Factions

### The Meridian Accord
The joint air command of the free nations. Practical, outgunned, stubborn. They launch the TALON from
the carrier MERIDIAN and talk to the pilot only by radio. Emblem: a white chevron over a bar in a circle.

### The Lattice
A distributed crystal intelligence with no centre on Earth; its nodes relay power to the CROWN in orbit.
It grows drones (WASPs, GNATs, MOTHs) from crystal and overgrows human machines (tanks, ships, cranes,
airships) to make the rest. It never communicates except through geometric glyph screens shown at
the twist and the ending.

## 5 World rules

1. The Lattice jams guided weapons, except the TALON's SEEKER darts, which home on the Lattice's own
   power using captured sparks.
2. Every destroyed node releases its stored power as a SPARK; formation leaders, carriers and ground
   nodes carry them.
3. Captured places can be freed: when a stage boss falls, the crystal in that stage goes dark.
4. The CROWN draws power through the relays; break the relays (stages 1-5) to expose it, climb to it
   (stages 6-8) to end it.
5. After the CROWN falls, the loop card reads "THE LATTICE REMEMBERS. IT HAS GROWN.": loop 2+ is the
   same war against a stronger remnant.

## 6 Glossary

| Term | Meaning |
|---|---|
| TALON | the XF-9 prototype fighter; also the game |
| QUILL | callsign of Captain Noor Vasquez, the TALON's pilot |
| BRANDT | Marshal Ida Brandt, Accord air commander, the briefing voice |
| MERIDIAN | the Accord carrier the TALON launches from |
| SPARK | a captured Lattice energy cell; advances the arming gauge |
| ARMING GAUGE | the eight-slot bar at the bottom of the screen |
| ARM | commit the highlighted gauge slot (Fire 2) |
| POD | a wing pod added by WINGS 2 and 3 |
| ESCORT | a pod promoted to a full fighter during OVERDRIVE |
| OVERDRIVE | ARM WINGS at WINGS 3: escorts plus double fire rate for 10 s |
| NODE | any Lattice ground installation |
| RELAY | a node that sends power to the CROWN |
| CROWN | the Lattice's orbital ring station; the final boss |
| SKYHOOK | the equatorial space elevator |

## 7 Art-direction constraints (hardware)

- Sprite pixels are 2 display px wide × 1 line tall (multicolour). A cell is 12×21 art pixels.
- A sprite cell has transparent + the game's shared pair + one private ink. A character's identity lives
  in its private inks. The TALON reserves white (1) and cyan (3) as private inks.
- Bitmap cells are 4×8 art pixels with three inks plus the stage's background ink. Ground objects are
  composited onto their background before quantization, so they share the cell's three inks.
- Read at a glance from above: every enemy needs a distinct silhouette and one bright accent.
- Enemy bullets are small bright shapes (white/yellow/light red) that must read on every stage's
  background; the quantizer checks contrast per stage.
- Nothing is generated at C64 size: Ludo renders at maximum resolution, our tools downsample.

## 8 IP-safety rules

- TALON is an original game in the 1980s vertical-shooter genre. It shares genre conventions (a
  selectable power-up gauge, wing pods, ground and air targets, checkpoints, loops) and nothing else.
- No names, stage layouts, logos, sprites or music from any existing game. `forbidden-terms.txt` is
  checked on every file and every prompt.
- No real-world aircraft, militaries, flags, insignia or religious iconography.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-31): sprite expansion removed at the user's request; bigger grids instead
