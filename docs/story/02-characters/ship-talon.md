+++
id = "ship.talon"
kind = "ship"
name = "XF-9 TALON"
faction = "accord"
status = "approved"
revision = 5
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
expanded = false
angles = 1
anchor = "center"
identity = "a sleek predatory white retro-future interceptor seen from directly above, nose up, knife-edge nose, raptor-claw wingtips, narrow angular light blue canopy slit"
silhouette = "a long sharp arrowhead nose, a narrow angular fuselage with chiselled facets, wings swept steeply back that hook forward at the tips into curved claw points like a raptor's talons, one short raked central tail blade on the spine, twin angular engine nozzles side by side at the base"
proportions = "40 display px wingspan by 40 px long at base; the fuselage is a sixth of the span wide; the nose is a third of the length; the canopy is a narrow slit a quarter of the fuselage length, just behind mid-length so it straddles the cell-row seam; the wings start at two fifths and reach full span at the claw tips at four fifths; WINGS 1 extends the span to 48 px"
materials = "smooth white armour in sharp angular plates, thin dark grey panel seams, dark grey leading edges on the wings and claws, a narrow light blue glass canopy slit framed in dark grey, dark grey nozzles with light blue engine glow, a thick dark grey outline"
acting = "predatory and precise; snaps into turns like a hawk banking onto prey, settles level instantly, engines always burning; never wobbles when still"
forbidden = ["rounded toy-like proportions", "bubble canopy", "any national marking or flag", "real aircraft silhouette", "real-world fighter jet silhouette", "twin tail fins", "F-14/F-15/F-22-like shapes", "variable-sweep wings or canards", "engine intakes beside the cockpit", "red or orange paint", "visible pilot", "missiles hanging under the wings", "text or numbers on the airframe", "three-quarter or side view", "cast shadow outside the silhouette"]
hurtbox = { x = -2, y = 0, w = 4, h = 4 }
speed_levels_q8 = [384, 448, 512, 576, 640, 704]
hitbox_levels = [{ w = 4, h = 4 }, { w = 6, h = 6 }, { w = 8, h = 8 }, { w = 10, h = 10 }, { w = 12, h = 12 }]
private_inks = [1, 3]
cell_inks = { top_row = 1, bottom_row = 1 }
style_swaps = { "chunky toy-like proportions" = "bold sharp predatory proportions" }
hitbox_center = { x = 0, y = 2 }

[[colors]]
role = "primary"
c64 = 1
note = "white: the whole hull (nose, fuselage, wings); private ink of BOTH cell rows (plan B)"
required = true

[[colors]]
role = "canopy"
c64 = 14
note = "light blue (the shared pair light tone): the narrow canopy slit and the twin engine glow (plan B); cyan (3) is used only by the invulnerability flash"
required = false

[[colors]]
role = "shared_light"
c64 = 14
note = "the game's shared pair light tone (proposed light blue 14; shared_pair.py decides): small panels only (wing roots, fin), never the main hull"
required = false

[[colors]]
role = "shared_dark"
c64 = 11
note = "the game's shared pair dark tone (proposed dark grey 11): the thick outline, nozzles and canopy frame"
required = false

[[animations]]
id = "idle"
frames = 2
ticks_per_frame = 3
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "hovering level in place, the twin engine exhausts flicker between short and long flames, the airframe stays perfectly still"

[[animations]]
id = "bank_left"
frames = 2
ticks_per_frame = 4
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "rolls into a gentle left bank, the left wing dips and looks narrower, the right wing lifts and looks broader, then holds the bank"

[[animations]]
id = "bank_right"
frames = 2
ticks_per_frame = 4
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "mirror of bank_left, made by flipping the curated frames horizontally"

[[animations]]
id = "wings1_idle"
frames = 2
ticks_per_frame = 3
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "hovering level in place, the twin engine exhausts flicker between short and long flames, the wingtip gun pods stay still"

[[animations]]
id = "wings1_bank_left"
frames = 2
ticks_per_frame = 4
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "rolls into a gentle left bank, the left wing and its gun pod dip, the right wing lifts, then holds the bank"

[[animations]]
id = "wings1_bank_right"
frames = 2
ticks_per_frame = 4
loop = false
model = "offline"
duration = 3
request_frames = 16
motion = "mirror of wings1_bank_left"

[[animations]]
id = "death"
frames = 3
ticks_per_frame = 5
loop = false
model = "hydra-plus"
duration = 3
request_frames = 25
motion = "the airframe cracks along the wing roots, the two wings and the nose section spin apart in three directions, flames burst from the engines"

[[animations]]
id = "barrier"
frames = 2
ticks_per_frame = 4
loop = true
model = "offline"
duration = 3
request_frames = 16
motion = "offline composite: a faceted hexagonal ring outline drawn around the ship inside the same 2x2 cells, alternating between two facet phases; three variants for 3, 2 and 1 charges (6, 4 and 2 facets lit)"

[ludo]
concept_prompt = "A sleek, menacing retro-future interceptor seen from directly above, nose pointing to the top of the image, clearly not a real-world fighter jet silhouette. A long knife-edge arrowhead nose, a narrow angular fuselage built from chiselled white armour plates, wings swept steeply back that hook forward at their tips into curved claw points like a bird of prey's talons, one short raked central tail blade on the spine, and two angular engine nozzles side by side at the tail glowing light blue. A narrow angular light blue glass canopy slit sits just behind the middle of the body like a narrowed eye. The whole hull is white with thin dark grey panel seams, dark grey leading edges on the wings and claws, and a thick dark grey outline around everything. A predatory, aggressive 1980s arcade hero ship. No markings, no pilot visible, no weapons hanging under the wings, no engine intakes beside the canopy."
sprite_prompt = "the interceptor centred, nose straight up, wings level, white armour plates with a thick dark grey outline, the claw wingtips sharp and hooked forward, the narrow light blue canopy slit just behind mid-body, both engines burning with short light blue flames"
wings1_prompt = "the same interceptor with its claw wingtips extended outward by a short armoured section ending in a small angular white gun pod on each tip, everything else unchanged"
+++

# XF-9 TALON

## Role in the story

The last prototype of the Meridian Accord, towed off BREAKWATER under a tarpaulin the night the airbase
fell. Analogue flight controls make it invisible to the Lattice's jamming; modular hard-points let it
burn captured SPARKS. Flown by QUILL. The ground crew call it "the kettle" because it whistles on the
catapult; nobody uses that name on the radio.

## Personality and acting

Nimble, eager, loyal. It is the hero, so it always looks in control: banks are short and crisp and it
snaps back level the moment the stick centres. The engines never stop burning. When it is hit it does
not tumble; it breaks cleanly into three pieces and is replaced by `vfx.explosion_l`.

## Silhouette and proportions

Read from above at 24×42 art px (48×42 display) the TALON must be a **white raptor**: a long knife-edge
nose and narrow chiselled fuselage in the top row, wings swept hard back in the bottom row whose tips
hook forward into claws (the ship's name, readable at 2-3 art px), one raked tail blade and two engine
dots at the base. Rev 5 (2026-10-31, the user: "The game is called Talon. Can the main spaceship look a
bit more menacing?") replaces rev 2-4's friendly toy dart with this predator; ink plan B, the grid, the
anchor and the hitbox are unchanged. Rev 2 (after P0, whose concepts all drifted toward real
twin-tail fighters): one central fin or none, never two; no intakes; a big bubble canopy; rounded,
chunky, plainly a toy and never a real-world fighter silhouette. Base art occupies
20 art px of the 24-px-wide grid (40 display px span) so two art px of margin on each side stay free for
the barrier ring; **WINGS 1** fills the full 24 art px with the wingtip gun pods. The big canopy
sits just behind mid-body in the shared light blue tone, like the engine glow (plan B).

## Ink plan (two private inks on a 2x2 grid)

A multicolour sprite cell has transparent, the game's shared pair, and one private ink. The TALON's two
cell rows use different private inks:

| Row | Private ink | Carries |
|---|---|---|
| top (lines 0-20) | white (1) | the hull: nose, forward fuselage, wing roots |
| bottom (lines 21-41) | white (1) | the rear hull and wings; canopy and engine glow use shared light blue |

Light blue (shared) is kept to small panels (wing roots, fin) and dark grey (shared) to the thick outline,
nozzles and canopy frame, so the TALON reads
correctly whatever the quantizer picks for the pair, as long as the pair holds one dark and one mid
tone. **Ink ruling:** inks 1 and 3 are reserved on the **sprite layer only**: the shared sprite pair
never uses them and no other sprite (enemy, boss part, pickup, effect) uses them as its private ink,
except the TALON's own pods and escorts (white). Bitmap stages may use white and cyan freely (surf,
runway paint, Lattice crystal), which is why the TALON's identity is carried by its silhouette and the
all-white hull with a light blue canopy, not by the inks alone.

Register effects, which cost no extra frames:
- **Arming flash / invulnerability:** the two row inks swap (white ↔ cyan) every 4 ticks for the 120
  invulnerability ticks.
- **Respawn:** the whole ship blinks 2 ticks on, 2 ticks off for 180 ticks (this also frees channels).
- **OVERDRIVE:** both rows cycle white → cyan → yellow (7) every 2 ticks for the first 30 ticks.

**Ink plan B (decided 2026-10-30 by the user).** P1 showed that a white hull in both rows and a cyan
bottom-row private ink cannot coexist (one private ink per cell). Of the three legal plans in
`output/pilots/p1/ship_talon_ink_plans.png` the user chose **B**: white private ink in both rows; the
canopy and engine glow fall to the shared light blue. Cyan remains reserved for the invulnerability
flash (row inks swap white ↔ cyan).

## Master fix (rev 4)

P1's concepts and master kept the canopy high in the nose (about 20-50% of the length), in the top
cell row. `master_fix_prompt` is a masked `image edit` of the chosen master: the mask (drawn by our
tools, white = redraw) covers the fuselage from below the nose cone to the nozzles, and the prompt
describes what fills it: the canopy just behind mid-body and the light blue engine glow (plan B). The
fixed master replaces the master as the source of the stills and animations.

## Frames

| Set | Frames | Source |
|---|---|---|
| idle | 2 | hydra from the master; only the bottom row really changes (exhaust length) |
| bank_left / bank_right | 2 + 2 | hydra, then mirrored offline |
| wings1_idle, wings1_bank_left/right | 2 + 2 + 2 | `image edit` of the master with `wings1_prompt`, then the same motions |
| death | 3 | hydra-plus |
| barrier | 2 × 3 charge variants, composited over idle and both banks of both widths | offline compositor (no Ludo) |

Banking is chosen by the stick: holding left for ≥ 6 ticks shows frame 1, ≥ 12 ticks frame 2; releasing
returns through frame 1 in 4 ticks. All frames share the anchor and the 2×2 grid; no frame may exceed
it (`toolbox sprite compare` ≤ 5% scale drift).

## Speed, size and hitbox

See `docs/design.md` 3 and 5. `speed_levels_q8` is SPEED 0..5 in NTSC px/tick × 256. The hitbox starts at
4×4 display px centred 2 px below the anchor (on the canopy) and grows one step for every armed system
that makes the ship heavier, up to 12×12 (`hitbox_levels`). The hitbox never covers the wings at base
level: bullets that cross a wingtip miss.

## Colour notes

The all-white hull with a light blue canopy is the TALON's signature and are reused on everything the
player owns: pods (`pod.wing`, white), escorts (`pod.escort`, white over white), the barrier ring and
the relaunch streak (white and cyan). Player shots are yellow (`08-vfx.md`).

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-30): user chose ink plan B (all-white hull, light blue canopy and glow)
- rev 4 (2026-10-31): P2 pilot fixes
- rev 5 (2026-10-31): the user asked for a more menacing ship; raptor silhouette, claw wingtips, narrow
  canopy slit; same inks, grid and hitbox. The rev 4 master below stays in the game until a rev 5 master
  is reviewed and accepted. `master_fix_prompt` was dropped (it fixed the rev 4 master only).
- rev 5 master accepted (2026-10-31, the user: "Let's go with the ship master 3!"): pilot P5 master take 3
  (image edit of concept take 1), mirrored from its lit left half, converted with the dark-armour settings
  (`shade_split` 0.85, `chroma_min` 0.03); `tools/talon/ship_master.py`, `assets/generated/ship.talon/manifest.json`.
  The rev 4 entry below is history.
- master accepted (2026-10-31, the user): P2 master fix take 2 with the detached nose cap removed offline
  (pilot P3); sources in `assets/sprites/source/ship.talon/`, the cleaned master and the idle cells in
  `assets/generated/ship.talon/` (`tools/talon/ship_master.py`, `manifest.json`). Every later frame starts
  from that master.
