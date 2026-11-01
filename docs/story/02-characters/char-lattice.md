+++
id = "char.lattice"
kind = "character"
role = "antagonist"
name = "THE LATTICE"
faction = "lattice"
status = "approved"
revision = 1
appears = ["twist-02", "stage-card S6-S8 glyph strip", "crown-caption", "loop-card"]
portrait = "none"
voice = "glyph screen"
glyph_charset = "16 glyphs, #0..#F, each 2 columns x 1 row of 8x8 hires characters (16x8 display px)"
glyph_inks = { primary = 3, secondary = 4, flash = 14, background = 0 }
avatar = "boss.s8.crown"
identity = "a distributed crystalline machine intelligence with no face, no body and no voice; it arranges hexagonal and triangular crystal into patterns"
forbidden = ["a face or eyes", "a mouth or speech", "organic shapes, tendrils or flesh", "a humanoid avatar", "any written human language in its glyphs", "religious or occult symbols", "real alphabets or runes"]

[[colors]]
role = "primary"
c64 = 3
note = "cyan glyph strokes"
required = true

[[colors]]
role = "secondary"
c64 = 4
note = "purple glyph fills and the second flicker phase"
required = true

[[colors]]
role = "flash"
c64 = 14
note = "light blue border flash when a glyph row completes"
required = false

[[glyphs]]
id = "#0"
meaning = "blank cell (spacing)"
shape = "empty"

[[glyphs]]
id = "#1"
meaning = "ground / a held place"
shape = "a flat hexagon lying on a horizontal bar"

[[glyphs]]
id = "#2"
meaning = "line / power flowing sideways"
shape = "three small triangles in a row pointing right"

[[glyphs]]
id = "#3"
meaning = "node"
shape = "a hexagon with a dot in the centre"

[[glyphs]]
id = "#4"
meaning = "rising, left edge of an arrow"
shape = "a hexagon with its upper-right facet open"

[[glyphs]]
id = "#5"
meaning = "rising, flank of an arrow"
shape = "two stacked hexagons leaning right"

[[glyphs]]
id = "#6"
meaning = "rising, centre of an arrow"
shape = "a vertical chain of three facets"

[[glyphs]]
id = "#7"
meaning = "converging"
shape = "two triangles meeting tip to tip"

[[glyphs]]
id = "#8"
meaning = "up"
shape = "a tall triangle pointing up, hollow"

[[glyphs]]
id = "#9"
meaning = "the point / the summit"
shape = "a small solid triangle pointing up"

[[glyphs]]
id = "#A"
meaning = "crown, left arc"
shape = "a quarter ring of facets curving up-right"

[[glyphs]]
id = "#B"
meaning = "crown, facet"
shape = "a hexagon with a short spike on top"

[[glyphs]]
id = "#C"
meaning = "crown, centre / the CROWN itself"
shape = "a hexagon ringed by six small triangles"

[[glyphs]]
id = "#D"
meaning = "growth"
shape = "three hexagons in a branching cluster"

[[glyphs]]
id = "#E"
meaning = "breaking"
shape = "a hexagon split by a diagonal crack"

[[glyphs]]
id = "#F"
meaning = "remembering"
shape = "a hexagon inside a hexagon"

[ludo]
concept_prompt = "A sheet of sixteen abstract geometric glyphs made of hexagons and triangles, crisp faceted crystal strokes in cyan with purple fills on a black background, arranged in a four by four grid. Purely geometric, no letters, no runes, no symbols from any real alphabet or religion. Cold, precise, beautiful."
+++

# THE LATTICE

## Role in the story

The antagonist has no face, no voice and no single body. The LATTICE is a distributed crystalline
machine intelligence that arrived with the meteors in 2091; it grows rather than builds, and it takes
human places and machines instead of destroying them. Its only "body" on screen is crystal: the
growth in the bitmap, the drones it grows, the machines it overgrows, and finally its avatar, the
CROWN (`boss.s8.crown`).

## How it "speaks"

The Lattice never uses words. It communicates through **glyph screens**: rows of hexagonal and
triangular glyphs drawn from a 16-glyph custom charset, appearing one row at a time on a black screen
with a soft ring-modulated shimmer (`sfx.lattice_shimmer`). The Accord's signals crew decode its
meaning, so the only "translation" the player sees is the MERIDIAN's decoded line on the next card.

Glyphs are hand-drawn 8×8 hires characters (two characters per glyph), not Ludo art; the concept prompt
above is only a mood reference for drawing them. Each glyph is built only from hexagons, triangles and
straight facets, never curves that suggest letters.

Glyph screens and their placements:

| Screen | Glyphs | Meaning |
|---|---|---|
| twist-02 | an upward arrow of `#1`/`#4`-`#9` over the CROWN `#A#B#C#B#A` | "the power goes up, to the CROWN" |
| stage cards S6-S8 | `#A#B#C#B#A` strip | "the CROWN is watching" |
| crown-caption | nineteen `#C` glyphs | the CROWN's ring breaking |
| loop-card | `#A#B#C#B#A` above the text | "it remembers" |

## Presentation rules

- Glyph strokes cyan (3), fills purple (4), on black. When a row completes, the border flashes light
  blue (14) for 2 ticks. The glyphs flicker cyan ↔ purple every 8 ticks while on screen.
- Glyph rows appear one per 20 ticks; the screen cannot be skipped until the last row has landed.
- Never a face, never eyes, never a mouth, never a voice sample. The Lattice is pretty and wrong, not
  monstrous.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
