+++
id = "style"
kind = "style"
name = "LUDO STYLE GUIDE"
status = "approved"
revision = 3

[style]
art_style = "Cel-Shaded"
art_style_alternates = ["Illustration"]
perspective = "Top-Down"
concept_prefix = "1980s arcade cabinet flyer illustration of retro-future military hardware, seen from directly above in strict orthographic top-down view, thick bold dark outlines, large simple flat colour masses, chunky toy-like proportions, minimal small detail, clean flat colour fills with one shade tone, lit from the upper left, plain flat mid-grey background, single object, centred"
still_prefix = "1980s arcade game sprite art, strictly orthographic top-down view seen from directly above, the nose or front pointing straight up toward the top of the image, thick bold dark outlines, large simple flat colour masses, chunky toy-like proportions, minimal small detail, bold readable silhouette, clean flat colour fills with one shade tone, lit from the upper left with shade on the lower-right faces and a short hard shadow offset to the lower right, isolated single object, centred"
vfx_prefix = "1980s arcade game effect art seen from directly above, bold flat shapes, clean flat colour fills with one shade tone, hard edges, no soft glow, no motion blur, isolated on transparency, centred"
tile_prefix = "1980s arcade game terrain texture seen from directly above in strict orthographic top-down view, seamless top-to-bottom tiling, bold readable ground shapes, clean flat colour fills with one shade tone, lit from the upper left with short hard shadows to the lower right, no horizon, no sky, no vehicles, no characters, no text"
texture_suffix = "simple repeating surface, no objects, no islands, every pattern runs straight across the image, no diagonal layout"
setpiece_prefix = "1980s arcade game painting of a large structure seen from directly above in strict orthographic top-down view, fills the full image width edge to edge, straight orthogonal layout, each long shape runs once in the one direction the subject names, no diagonal layout, thick bold dark outlines, large simple flat colour masses, minimal small detail, clean flat colour fills with one shade tone, lit from the upper left with short hard shadows to the lower right, no horizon, no sky, no characters, no text"
motion_rules = "positive wording only; under 60 words; never restate the object's appearance; name the action, the moving part and how it settles; the camera stays directly overhead and never moves; the object stays centred and nose-up unless the motion is a roll or a turn; no frame counts or timing"
never_in_prompts = ["perspective view", "three-quarter view", "isometric", "side view", "low angle", "horizon", "sky (except S7 and S8 cloud and star tiles, which describe them as flat texture)", "transparent background (the sprite type keeps it)", "pixel art (P0 dropped the 16-Bit tier)", "gradient", "soft glow", "bloom", "lens flare", "motion blur", "depth of field", "text", "logo", "insignia", "national flag", "real aircraft or vehicle model names", "people (except the char.quill and char.brandt portraits)", "blood or injury", "weapon firing (muzzle flashes and shots are separate VFX)", "any franchise or game name from forbidden-terms.txt"]
review_size = "true C64 size at 2:1 pixel aspect, shown at 3x on a PAL-ratio preview"

# Ludo option names confirmed with `ludo image create --help` (ludo CLI 0.2.1, 2026-10-30).
[ludo_options]
perspective = "Top-Down"
image_types = { concept = "asset", still = "sprite", vfx = "sprite-vfx", texture = "texture", tile = "tile", tiling_strip = "sprite-tiling-vertical", setpiece = "vertical_scrolling_background" }
aspect_ratios = ["default", "ar_1_1", "ar_4_3", "ar_16_9", "ar_19_9", "ar_3_4", "ar_9_16", "ar_9_19"]
setpiece_aspect = "the listed aspect closest (log ratio) to size_cells width/height; 40x16 -> ar_19_9, 40x20 -> ar_19_9, 40x22 and 40x25 -> ar_16_9, 38x50 -> ar_3_4 (rev 3; rev 2 forced ar_9_16 / ar_9_19)"
n_max = { create = 8, edit = 4 }
batch_naming = "`ludo batch` saves n>1 results as <name>-1..-N; single commands with --n save <name>-v1..-vN"
resolution = "image create and image edit take no size flag; the output size is fixed by the model, so maximum resolution means the default output, never a smaller one; sprite animate always --frame-size 0"
+++

# Ludo style guide (top-down)

TALON is seen from **directly above**. Pummel's guide was side-view; nothing here inherits its framing.
The exact Ludo option names, confirmed with `ludo image create --help` before P0, are recorded in
`[ludo_options]` above: `--perspective "Top-Down"`; image types `asset` (concept), `sprite` (still),
`sprite-vfx`, `texture`, `tile`, `sprite-tiling-vertical` and `vertical_scrolling_background`;
`--aspect-ratio` one of `default ar_1_1 ar_4_3 ar_16_9 ar_19_9 ar_3_4 ar_9_16 ar_9_19`. Batch quirk:
`ludo batch` names n>1 results `<name>-1..-N`, single commands `<name>-v1..-vN`.

Every prompt the compiler emits is `<prefix>. <identity>. <silhouette>. <proportions>. <materials>.
Colours: <colour list with hex>. <extra>` with the prefix chosen by the asset kind:

| Asset kind | Prefix | Ludo call |
|---|---|---|
| concept sheet (ship, characters, enemies, bosses) | `concept_prefix` | `image create --image-type asset` |
| sprite master and every sprite still | `still_prefix` | `image create/edit --image-type sprite` |
| master fix (`master_fix_prompt`, rev 3) | none: the prompt says only what fills the offline mask | masked `image edit --mask-mode replace` of the chosen master |
| explosions, shots, pickups | `vfx_prefix` | `--image-type sprite-vfx` |
| terrain textures | `tile_prefix` + `texture_suffix`, subject only (no section description) | `--image-type texture` |
| terrain tiles with objects | `tile_prefix` + section description | `--image-type` as the level declares (`sprite`, `tile`, `sprite-tiling-vertical`) |
| stage set-pieces | `setpiece_prefix` | `--image-type vertical_scrolling_background`, the aspect closest to its `size_cells` (rev 3) |
| boss set-pieces (`setpiece_prompt`) | `setpiece_prefix` | `--image-type texture` (unchanged in rev 2; revisit when the first boss is piloted) |
| motion (animations) | none | `sprite animate` on hydra / hydra-plus |

Motion prompts get no prefix: Ludo takes appearance from the image. Always `--frame-size 0` on Hydra
and Hydra+, as `CLAUDE.md` requires.

## Art style

A **1980s arcade cabinet** look: bold readable shapes, clean flat colour with **one shade tone**, crisp
dark outlines, sunny confident military hardware against cold faceted crystal. Not painterly, not
photographic, not "modern pixel art".

- Default `--art-style "Cel-Shaded" --perspective "Top-Down"`; the only alternate is "Illustration".
- **Pilot P0 result** (`output/pilots/p0`): Cel-Shaded reads best at 24×42 (widest solid masses, lowest
  quantizer error); 16-Bit averages into noise at a 20–40:1 reduction and is dropped; Illustration gives
  thinner, spikier shapes and stays as the only alternate.
- Every concept and still asks for **thick bold dark outlines, large simple flat colour masses, chunky
  toy-like proportions and minimal small detail**: at C64 size only masses and the outline survive.
- Never describe or name a real aircraft. Where a prompt must steer away from one it says
  "not a real-world fighter jet silhouette"; specific types appear only in a bible `forbidden` list.
- Everything is generated **oversized** (maximum output size) and reduced by our tools; nothing is
  generated at C64 size.

## Lighting, line, colour

- One flat key light from the **upper left**. Shade sits on the lower-right faces. A short hard shadow
  falls to the lower right.
- **Air objects:** the remap step keys the cast shadow out of sprite frames (a sprite shadow would float
  with the object); air units get no ground shadow at all. **Ground objects and set-pieces** keep the
  shadow, because they are composited into the bitmap where it reads as depth.
- Uniform dark outline; sparse interior lines (they vanish at 2:1).
- Prompts list the C64 hex values of the object's inks ("primary colour white #F7F7F7, canopy cyan
  #6AD4CD") and ask for flat fills; the quantizer does the rest.
- Lattice crystal: cyan #6AD4CD, light blue #6D6AEF and purple #9835A4 with white glints, always
  hexagonal or triangular facets.

## Orientation and rotation

- Every sprite is generated **nose up** (front toward the top of the image), centred, wings or tracks
  level.
- **Rotations are never generated in Ludo.** Turrets (16 angles), tanks (8), SEEKER darts and needles (8)
  and any enemy listed with `angles > 1` are made offline: our tool rotates the high-resolution master
  (or only the turret head over a fixed base), area-resamples each angle linear-light, then quantizes.
  One master guarantees the angles match; Ludo would draw sixteen slightly different turrets.
- Mirrored frames (bank right from bank left, symmetric enemies) are flipped offline too.

## Per-size framing rules

One cell = 12×21 art px = 24×21 display px. Multicolour pixels are 2:1, so a generated image is framed
at the **display** aspect (48×42 → 8:7 for M) and the downsampler halves horizontal resolution.

- **S (1×1, 12×21 art px):** one dominant shape and one accent; nothing thinner than 2 art px; the
  silhouette must read as a solid mask. Swoopers, drones, pods, pickups.
- **W (2×1, 24×21):** wide and flat: boats, crossing pairs, skimmers. Detail along the leading (top)
  edge; the trailing edge stays simple.
- **M (2×2, 24×42):** the TALON's class. A clear nose in the top row, mass in the bottom row; one accent
  per cell row (each row has its own private ink).
- **L (3×3, 36×63):** heavy gunships and carriers. A readable "face" (bridge, gun cluster) in the
  middle cell; armour in the outer ring of cells; leave transparent corners where possible so the
  corner cells can be dropped on busy lines.
- **B (up to 4×8, 48×168):** bosses and segments. Design in rows: each cell row can be shown or dropped
  independently, so weak points and guns sit in rows that stay visible, and transparent rows are
  welcome (they free channels for bullets). Never a solid rectangle.
- **No sprite expansion (rev 3).** VIC-II X/Y expansion (`$D017/$D01D`) is banned in TALON: no size
  class is doubled. Big enemies are made only from bigger sprite grids (up to 4×8, class B) or from
  bitmap patches.
- **Bitmap patches (`cell_rect`, 8×8 display cells, 4×8 art px each):** ground objects. Generated
  isolated with `still_prefix` and the shadow kept, then **composited by the baker over the exact
  background patch** at full resolution before quantization. Each 8×8 cell holds three inks plus the
  stage background, so patches avoid more than three local colours per cell; outlines align to the
  8-px grid where possible. Rotating parts (turret heads) are separate layers so only the head turns.
- **Textures** (`image_type = "texture"`): `tile_prefix` + `texture_suffix` with the tile's `subject` only.
  The section description is left out, because it lists objects (islets, buoys, wakes) that belong in
  their own tiles; patterns such as swells run horizontally across the image.
- **Tiles:** `tile_prefix`, generated as a vertical strip that tiles top-to-bottom seamlessly, 320
  display px wide in proportion (the stage builder resamples to 320). Large shapes (runways, rails,
  canals) run vertically so scrolling reads.
- **Set-pieces:** `setpiece_prefix`, `vertical_scrolling_background` at the aspect closest to `size_cells` (rev 3; P0's `sprite` type
  returned an isolated, diagonal object on transparency), painted full width (40 cells) and as tall as the level file says;
  the baker slices them into the stage strip. Destroyable parts are painted twice: intact and wrecked.
  Rev 3 (after P1, where Ludo read "aligned to the vertical axis" as "run the wall vertically"): the
  prefix names no axis. Each set-piece `subject` states the direction of its long shape in so many
  words, for example "a single straight sea wall running from the left edge to the right edge of the
  image, once, across the middle", or "running straight from the bottom edge to the top edge" for a
  structure the player flies along. A subject never says "diagonally". P2 showed the wording alone is
  not enough: at `ar_9_16` a 40x16 band still came back as a tall vertical composition with transparent
  side margins, so rev 3 also requests each set-piece at the listed aspect closest to its own shape.

## Review checklist (before any sheet is accepted)

Every candidate is reviewed **at true C64 size, 2:1 pixel aspect**, on the quantized result, next to
the TALON and on its stage background.

1. Strictly top-down: no visible side faces, no horizon, nose up, no perspective skew.
2. Identity matches the concept master (silhouette, colours, accent).
3. Lighting from the upper left; air sprites carry no cast shadow after remap; ground patches keep it.
4. The silhouette reads as a solid mask at C64 size, and the object is distinct from every other object
   in its stage.
5. Frame scale stable within the clip (`toolbox sprite compare` under 5%), anchor stable.
6. Inks: the object's private ink lands where the bible says, the shared pair is not overused, and no
   object other than the TALON shows a white row over a cyan row.
7. Enemy bullets and the SPARK remain readable over the object (contrast check per stage).
8. For B and L classes: dropping any single cell row on a busy line still leaves the object readable.
9. No text, logos, insignia, flags or real-world vehicle shapes crept in.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- rev 3 (2026-10-31): P2 pilot fixes; VIC-II sprite expansion banned (user)
