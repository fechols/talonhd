+++
id = "hd.rules"
kind = "hd_rules"
name = "PALETTE LAW"
status = "approved"
revision = 3

# The four global multicolor codes and their roles. Every pixel on screen is one of these four.
[roles]
void = { code = 0, use = "the ground or sky: most of the background, the darkest tone, shadow side of hulls only as a dither" }
dark = { code = 1, use = "hull shade, background structure, the lower half of every lit ramp" }
light = { code = 2, use = "the player's hull, Accord friendlies, explosion cores, surf and markings in small doses" }
hot = { code = 3, use = "the Lattice: crystal, cores, enemy bullets, muzzles, engine glow" }

# C64 luminance groups (Pepto), darkest 0 to brightest 8. Used for the value rules.
[luma]
0 = 0
6 = 1
9 = 1
2 = 2
11 = 2
4 = 3
8 = 3
12 = 4
14 = 4
5 = 5
10 = 5
3 = 6
15 = 6
7 = 7
13 = 7
1 = 8

[value_rules]
order = "luma(void) < luma(dark) < luma(light): shading must read on every stage"
hot_contrast = 3          # luma(hot) - luma(void) >= 3: bullets read on the background
stage_change = 2          # consecutive stages differ in at least 2 of the 4 roles
distinct = true           # the four colors of a palette are four different C64 colors

[polygon_rules]
faces = "every face is one solid code or a 50 % checkerboard of two neighbouring ramp codes, anchored in sprite space"
light = "one key light for the whole game: upper left, toward the viewer"
outline = "none: silhouettes come from contrast against void"
floor = "a lit hull never shades to pure void; player hulls stop at solid dark, enemy hulls at the void/dark checker"
facet_size = 12           # a face should average >= 12 on-screen pixels at the object's canvas size

# Size classes: canvas in multicolor pixels (one pixel = 2 hires px wide, 1 line tall), triangle budget.
[size_classes]
S = { max_canvas = 32, tris = [40, 120] }
M = { max_canvas = 80, tris = [120, 300] }
L = { max_canvas = 128, tris = [300, 700] }
XL = { max_canvas = 256, tris = [700, 1500] }
BOSS = { max_canvas = 512, tris = [1500, 4000] }

[light_share]
player_min = 0.35         # >= 35 % of the player's opaque pixels are light (or light-checker)
enemy_max = 0.15          # <= 15 % for enemies and bosses: light belongs to the player
background_max = 0.10     # <= 10 % of a stage's panorama
+++

# Palette law

## Four colors, all global

The C64 multicolor bitmap has four pixel codes. In TALON HD **all four are global**: code 00 is `$D021`,
codes 01 and 10 are the one byte written to every screen-RAM cell, code 11 is the one value in color RAM.
Nothing ever writes per-cell attributes, so sprites and background can never clash, and a single register
write or one fill recolors the whole screen.

Each stage has exactly one palette of four C64 colors. The stage, its enemies, its bosses, every shot and
the player's ship are all drawn with those four colors. A stage's identity *is* its palette.

## Roles, not colors

Models and art are authored in **roles**, never in C64 indices:

| Role | Code | Carries |
|---|---|---|
| `void` | 00 | ground or sky, the darkest tone |
| `dark` | 01 | hull shade, background structure |
| `light` | 10 | the player's hull, friendlies, explosion cores |
| `hot` | 11 | the Lattice: crystal, cores, enemy bullets, glow |

So one model works on every stage: the GANNET is "dark hull, hot crystal eye" everywhere, and the stage
decides what dark and hot look like.

## Value first

Each palette is chosen as a **luminance ramp first and a hue second**: `void` is darker than `dark`, which
is darker than `light`. That is what makes Lambert shading read on every stage. `hot` sits at least three
luminance steps above `void`, so bullets read against the background. The luminance groups are in
`[luma]` above; `bible_hd.py check` enforces the order.

## Change between stages

Consecutive stages differ in at least two of the four roles, so a stage clear always looks like arriving
somewhere new. The palette walk between them is a promotion-sequence effect (`01-art-direction.md`).

## Light belongs to the player

`light` is the player's color. Player ships are at least 35 % light; enemies and bosses at most 15 %
(highlights only); the background at most 10 %. This is how the player stays readable under a 512×512
boss that shares the same four colors. The bake measures it and fails over budget.

## Polygons

- Every face is a solid code or a 50 % checkerboard of two neighbouring ramp codes. The checker is fixed
  to the sprite, so it travels with the ship and never crawls.
- One key light for the whole game, upper left and toward the viewer.
- No outlines. Silhouettes come from the contrast against `void`.
- A lit hull never falls to pure `void`. Player hulls stop at solid `dark` (a checker against `void` reads as
  holes when the ship banks); enemy hulls may reach the void/dark checker.
- Low polygon on purpose: the facets are the style. Each size class has a triangle budget, and a face
  should cover about 12 on-screen pixels or more.

## Size classes

| Class | Canvas (multicolor px) | Triangles | Typical |
|---|---|---|---|
| S | ≤ 32 | 40-120 | WASP, bullets' carriers, debris |
| M | ≤ 80 | 120-300 | the player's ship, GANNET, CRAWLER |
| L | ≤ 128 | 300-700 | heavy units, ground emplacements |
| XL | ≤ 256 | 700-1500 | midbosses |
| BOSS | ≤ 512 | 1500-4000 | stage bosses: always bigger than the screen |

A multicolor pixel is two hires pixels wide, so a 512-pixel canvas is 1024 hires pixels across: more than
three screens wide.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): M canvas limit 64 -> 80 so the longest player ships fit (number fix with the ship canvases)
- rev 3 (2026-10-31): player hull floor is solid dark (art-direction review of the TALON pilot)
