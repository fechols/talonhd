+++
id = "vfx"
kind = "vfx"
name = "VFX"
status = "approved"
revision = 2

# ================================================================ player shots
# Player-shot buckets use private ink yellow (7); the shapes are tiny, so `source = "mask"` means a
# hand-authored pixel mask in tools/talon (no Ludo job). `prompt` is kept as the reference for the
# concept sheet only. size_px is display px (multicolour pixels are 2 px wide).

[[vfx]]
id = "vfx.shot_twin"
name = "TWIN BOLT"
layer = "bucket"
size_px = [4, 10]
hitbox = { w = 4, h = 10 }
frames = 2
ticks_per_frame = 2
loop = true
source = "mask"
identity = "a slim vertical yellow bolt with a white-hot tip, two art px wide"
colors = [7]
prompt = "a slim vertical bolt of yellow energy with a bright white tip, flying straight up"
per_wings = { w0 = "2 bolts at x -4 and +4", w1 = "+2 bolts from the wingtips at x -20 and +20", w2 = "+1 bolt from each inner pod (vfx.shot_pod)", w3 = "+1 bolt from each of the 4 pods" }

[[vfx]]
id = "vfx.shot_flank"
name = "FLANK SHOT"
layer = "bucket"
size_px = [10, 4]
hitbox = { w = 10, h = 4 }
frames = 2
ticks_per_frame = 3
loop = true
angles = 4
source = "mask"
identity = "a short fat horizontal yellow dash with a white leading end; diagonal variants for the WINGS 1 wingtip shots"
colors = [7]
prompt = "a short fat horizontal dash of yellow energy with a white leading end"
per_wings = { w0 = "2 shots, horizontal left and right, from the wing roots", w1 = "+2 shots at 45 degrees forward from the wingtips", w2 = "unchanged (pods never flank)", w3 = "unchanged" }

[[vfx]]
id = "vfx.shot_burst"
name = "BURST PELLET"
layer = "bucket"
size_px = [8, 8]
hitbox = { w = 8, h = 8 }
frames = 3
ticks_per_frame = 6
loop = false
source = "mask"
identity = "a chunky yellow diamond flak pellet with a white core that shrinks as it slows: big, medium, small"
colors = [7]
prompt = "a chunky diamond-shaped flak pellet, yellow with a white core"
per_wings = { w0 = "5 pellets at 0, +-16, +-32 degrees", w1 = "+2 pellets at +-48 degrees", w2 = "+3 small pellets (vfx.shot_pod_burst) per inner pod", w3 = "+3 small pellets per pod, 4 pods" }

[[vfx]]
id = "vfx.beam_lance"
name = "BEAM LANCE"
layer = "sprite"
sprite_class = "B"
cells = [1, 6]
frames = 2
ticks_per_frame = 2
loop = true
identity = "a straight vertical lance of white energy with a yellow sheath, crackling facets along both edges, as thin as a pencil line at the base and flaring at the tip"
colors = [7]
prompt = "a long straight vertical lance of white energy with a yellow sheath and small crackling facets along both edges, flaring slightly at the top, filling the frame top to bottom"
per_wings = { w0 = "one lance 4 px wide, up to 6 cell rows (126 px); drawn row by row as it grows", w1 = "lance 8 px wide (second frame set), up to 8 rows (168 px)", w2 = "each inner pod adds a 3-row lance (vfx.beam_pod)", w3 = "each of the 4 pods adds a 3-row lance" }
budget_note = "one channel on every line the lance covers; the lance is above the TALON, never on the player band"

[[vfx]]
id = "vfx.beam_throw"
name = "LANCE THROW"
layer = "sprite"
sprite_class = "B"
cells = [1, 3]
frames = 2
ticks_per_frame = 2
loop = true
identity = "the released lance: a shorter white-and-yellow spear of energy with a bright tip and a fading tail, flying up"
colors = [7]
prompt = "a short vertical spear of white and yellow energy with a bright pointed tip and a tail that fades into three separate dashes"
per_wings = { w0 = "rows = the lance length at release (1-6)", w1 = "8 px wide frames", w2 = "pod lances are not thrown; they vanish on release", w3 = "same" }

[[vfx]]
id = "vfx.beam_pod"
name = "POD LANCE"
layer = "sprite"
sprite_class = "B"
cells = [1, 3]
frames = 2
ticks_per_frame = 2
loop = true
identity = "a thinner, shorter version of the BEAM lance, white with a yellow edge"
colors = [7]
prompt = "a thin straight vertical lance of white energy with a thin yellow edge, filling the frame top to bottom"

[[vfx]]
id = "vfx.shot_seeker"
name = "SEEKER DART"
layer = "bucket"
size_px = [6, 10]
hitbox = { w = 6, h = 6 }
frames = 1
ticks_per_frame = 8
loop = true
angles = 8
source = "mask"
identity = "a tiny yellow dart with a dark exhaust dot behind it; 8 headings rotated offline"
colors = [7]
prompt = "a tiny yellow homing dart pointing up with a single dark exhaust dot behind it"
per_wings = { w0 = "1 dart per target, max 4 per salvo; radial ring of 8 when no target", w1 = "max 6 per salvo", w2 = "+1 dart per inner pod at the nearest unassigned target", w3 = "+1 dart per pod, 4 pods" }

[[vfx]]
id = "vfx.shot_pod"
name = "POD BOLT"
layer = "bucket"
size_px = [2, 8]
hitbox = { w = 2, h = 8 }
frames = 1
ticks_per_frame = 8
loop = true
source = "mask"
identity = "a needle-thin yellow bolt, one art px wide"
colors = [7]
prompt = "a needle-thin vertical yellow bolt"

[[vfx]]
id = "vfx.shot_pod_burst"
name = "POD PELLET"
layer = "bucket"
size_px = [4, 4]
hitbox = { w = 4, h = 4 }
frames = 2
ticks_per_frame = 6
loop = false
source = "mask"
identity = "a small yellow diamond pellet"
colors = [7]
prompt = "a small yellow diamond pellet"

[[vfx]]
id = "vfx.muzzle"
name = "MUZZLE"
layer = "bucket"
size_px = [8, 6]
frames = 2
ticks_per_frame = 2
loop = false
source = "mask"
identity = "a short white starburst at the nose, two frames, shown on every TWIN or BURST volley"
colors = [7]
prompt = "a short white four-pointed starburst muzzle flash"

[[vfx]]
id = "vfx.hit_spark"
name = "HIT SPARK"
layer = "bucket"
size_px = [8, 8]
frames = 3
ticks_per_frame = 2
loop = false
source = "mask"
identity = "a small yellow star of sparks at the point where a shot hits; on armour it is grey instead"
colors = [7, 12]
prompt = "a small star-shaped burst of white sparks, isolated"

# ================================================================ enemy bullets
# Enemy-bullet buckets use private ink light red (10) by default; a stage may override it to white (1)
# in its level file where light red lacks contrast (the validator checks every stage). They never share
# a bucket with player shots.

[[vfx]]
id = "vfx.bullet_round"
name = "ROUND"
layer = "bucket"
size_px = [6, 6]
hitbox = { w = 2, h = 2 }
frames = 2
ticks_per_frame = 4
loop = true
source = "mask"
identity = "a round light red pellet with a dark ring and a one-pixel white centre that blinks"
colors = [10, 0]
prompt = "a small round energy pellet, light red with a dark outline ring and a bright centre"

[[vfx]]
id = "vfx.bullet_needle"
name = "NEEDLE"
layer = "bucket"
size_px = [4, 10]
hitbox = { w = 2, h = 4 }
frames = 1
ticks_per_frame = 8
loop = true
angles = 8
source = "mask"
identity = "a thin light red needle with a dark outline, pointed at both ends; 8 headings rotated offline"
colors = [10, 0]
prompt = "a thin pointed energy needle, light red with a dark outline"

[[vfx]]
id = "vfx.bullet_orb"
name = "BIG ORB"
layer = "bucket"
size_px = [12, 12]
hitbox = { w = 6, h = 6 }
hp = 2
score = 50
frames = 2
ticks_per_frame = 6
loop = true
source = "ludo"
identity = "a big slow pulsing orb: light red disc with a dark ring and a faceted crystal centre, the ring swelling and shrinking"
colors = [10, 0]
prompt = "a large round energy orb, light red disc with a dark outline ring and a small faceted crystal at its centre, pulsing"

[[vfx]]
id = "vfx.bullet_shard"
name = "SHARD"
layer = "bucket"
size_px = [8, 8]
hitbox = { w = 4, h = 4 }
frames = 4
ticks_per_frame = 4
loop = true
source = "mask"
identity = "a tumbling triangular crystal shard, light red facets with a dark edge; 4 rotation frames"
colors = [10, 0]
prompt = "a small triangular crystal shard tumbling end over end, light red facets with a dark edge"

[[vfx]]
id = "vfx.bullet_lob"
name = "MORTAR SHELL"
layer = "bucket"
size_px = [8, 8]
hitbox = { w = 4, h = 4 }
frames = 2
ticks_per_frame = 6
loop = true
source = "mask"
identity = "a round dark shell that grows as it rises and shrinks as it falls, with a separate small dark ground-shadow dot that slides to the landing point"
colors = [0, 10]
prompt = "a round dark mortar shell with a light red band"
note = "harmless in flight; hits only on its landing tick; the shadow dot is a second bucket object"

# ================================================================ explosions and ground damage

[[vfx]]
id = "vfx.explosion_s"
name = "BLAST S"
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
frames = 5
ticks_per_frame = 3
loop = false
identity = "a small round yellow fireball with a blue-white flash rim that bursts, then breaks into orange puffs, dark grey smoke and three crystal chips"
colors = [7, 8, 11, 14]
private_inks = [7, 8]
ink_plan = "identity in the private inks, chosen per frame (one cell): yellow (7) on cells holding the core, orange (8) on cells holding the fire ring; the shared pair only for dark grey (11) smoke and outline and a light blue (14) blue-white flash rim in the first frames; no white"
prompt = "a small round explosion seen from above: a bright yellow fireball with a thin pale blue flash rim bursting outward, breaking into orange fire puffs, dark grey smoke puffs and three small crystal chips"

[[vfx]]
id = "vfx.explosion_m"
name = "BLAST M"
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
frames = 6
ticks_per_frame = 3
loop = false
identity = "a big round yellow fireball with a blue-white flash rim, a ring of orange fire, dark grey smoke, panel fragments and crystal chips flying outward"
colors = [7, 8, 11, 14]
private_inks = [7, 8]
ink_plan = "identity in the private inks, chosen per cell and frame: by default per frame (P1: a top-yellow/bottom-orange row split leaves a hard seam across the ball): burst frames yellow (7) in every cell, ring and fade frames orange (8) in every cell; the shared pair only for dark grey (11) smoke and outline and a light blue (14) blue-white flash rim in the first frames; no white"
prompt = "a big round explosion seen from above: a bright yellow fireball core with a thin pale blue flash rim, a thick ring of orange fire, chunky dark grey smoke puffs, dark metal panel fragments and small crystal chips flying outward in all directions"

[[vfx]]
id = "vfx.explosion_l"
name = "BLAST L"
layer = "sprite"
sprite_class = "L"
cells = [3, 3]
frames = 7
ticks_per_frame = 3
loop = false
identity = "a huge round yellow fireball with a blue-white shock ring, three smaller orange secondary blasts at its edge, dark grey smoke, long fragments spinning away"
colors = [7, 8, 11, 14]
private_inks = [7, 8]
ink_plan = "identity in the private inks, chosen per cell and frame: yellow (7) on cells holding the core, orange (8) on cells holding the fire ring (the middle cell may stay yellow while the outer ring of cells goes orange); the shared pair only for dark grey (11) smoke and outline and a light blue (14) blue-white flash rim in the first frames; no white"
prompt = "a huge round explosion seen from above: a bright yellow fireball core with a pale blue shock ring, three smaller orange secondary fireballs at its edge, chunky dark grey smoke, long dark metal fragments spinning away"
budget_note = "3 channels; used for the TALON's death, L enemies and boss parts; may drop its outer columns on busy lines (it is last in mux priority)"

[[vfx]]
id = "vfx.explosion_boss_chain"
name = "BOSS CHAIN"
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
frames = 6
ticks_per_frame = 3
loop = false
identity = "a sequence, not new art: 12 vfx.explosion_m and 4 vfx.explosion_l at random points inside the boss's hurt area over 120 ticks, ending with a white border-and-background flash"
colors = [7, 8]
prompt = "none (sequenced from vfx.explosion_m and vfx.explosion_l)"
sequence = { duration_ticks = 120, m_blasts = 12, l_blasts = 4, max_live = 3, final_flash_ticks = 4 }

[[vfx]]
id = "vfx.ground_blast"
name = "GROUND BLAST"
layer = "bitmap"
cell_rect = [4, 4]
frames = 4
ticks_per_frame = 4
loop = false
identity = "a flat yellow-orange blast spreading over the ground from a destroyed ground target, with a dark scorch ring and thrown rubble; every frame is baked over the exact background patch"
colors = [7, 8, 0]
prompt = "a flat explosion on the ground seen from directly above: a yellow and orange burst spreading outward with a dark scorch ring and thrown grey rubble"
note = "per-site patch frames; sizes 2x2, 4x4 and 6x6 cells; the 4x4 is the default. The death VFX of every ground, hull and cable target: the blast frames play as patches, the last frame is the target's vfx.crater / wreck patch, and an optional vfx.explosion_s (sprite) may flash over the first frame when the line budget allows"

[[vfx]]
id = "vfx.crater"
name = "CRATER"
layer = "bitmap"
cell_rect = [4, 4]
frames = 1
ticks_per_frame = 60
loop = false
identity = "the final state of a destroyed ground target: a shallow dark scorched crater with broken plates and dull, dark crystal stubs; baked over the exact background"
colors = [0, 11, 9]
prompt = "a shallow scorched crater seen from directly above: a dark burnt ring, broken grey metal plates and a few dull dark crystal stubs, short shadows to the lower right"
note = "replaces the target's patch permanently; the stage remembers it across checkpoint rewinds"

[[vfx]]
id = "vfx.debris"
name = "DEBRIS"
layer = "bucket"
size_px = [4, 4]
frames = 2
ticks_per_frame = 4
loop = true
source = "mask"
identity = "tiny grey and light red chips flung from air kills, 4 to 6 per kill, fading after 20 ticks"
colors = [12, 10]
prompt = "a tiny tumbling metal chip"

[[vfx]]
id = "vfx.crystal_shatter"
name = "SHATTER"
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
frames = 4
ticks_per_frame = 3
loop = false
identity = "a small Lattice drone bursting into faceted glass: a purple flash, then six triangular crystal shards flying outward and fading; no fire, no smoke"
colors = [4, 14]
prompt = "a small crystal object shattering seen from above: a purple flash at the centre and six sharp triangular glass shards flying outward, light blue facets with purple edges"
note = "death VFX of grown Lattice drones (WASP, LANCER, PIKE, DUST DEVIL, SPIRE NODE and others); purple private ink, light blue from the shared pair; the shards continue as 2-4 vfx.debris chips for 16 ticks"

[[vfx]]
id = "vfx.spark_pop"
name = "SPARK POP"
layer = "bucket"
size_px = [8, 8]
frames = 3
ticks_per_frame = 3
loop = false
source = "mask"
identity = "a tiny burst for bucket-drawn objects (GNAT motes): a light red plus-shaped pop that opens into four dots and vanishes"
colors = [10]
prompt = "a tiny plus-shaped pop opening into four dots"
note = "drawn in the enemy-bullet bucket that held the object, so it costs no channel"

# ================================================================ pickups

[[vfx]]
id = "vfx.spark"
name = "SPARK"
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
frames = 4
ticks_per_frame = 5
loop = true
identity = "a yellow double-chevron energy cell pointing up with a white core, slowly spinning on its vertical axis (narrowing and widening)"
colors = [7]
prompt = "a glowing energy cell shaped like a yellow double chevron pointing up, with a bright white core, turning slowly on its vertical axis"

[[vfx]]
id = "vfx.spark_gold"
name = "GOLD SPARK"
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
frames = 4
ticks_per_frame = 4
loop = true
identity = "the SPARK chevron in orange-gold with a yellow core and a twinkle on alternate frames (hidden bonus)"
colors = [8, 7]
prompt = "an energy cell shaped like an orange-gold double chevron pointing up, with a yellow core and a small twinkle, turning slowly"

[[vfx]]
id = "vfx.spark_core"
name = "CORE SPARK"
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
frames = 4
ticks_per_frame = 5
loop = true
identity = "a large gold triple chevron with a faceted crystal heart and a white core, released by every stage boss; it hangs in place and slowly drifts toward the TALON"
colors = [7, 8]
prompt = "a large glowing energy core shaped like a triple chevron pointing up, gold and yellow, with a faceted crystal heart and a white-hot centre, turning slowly"
note = "moves the gauge cursor to the next boss's weakness; 10,000 or 50,000 (05-bosses/README.md)"

[[vfx]]
id = "vfx.tag"
name = "MERIDIAN TAG"
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
frames = 2
ticks_per_frame = 10
loop = true
identity = "a light grey rectangular flight tag on a short chain with a small chevron-and-bar emblem, swinging (the hidden 1UP)"
colors = [15, 12]
prompt = "a small light grey rectangular flight tag on a short bead chain, embossed with a simple chevron over a bar, swinging gently"

# ================================================================ TALON state effects

[[vfx]]
id = "vfx.gauge_arm_flash"
name = "ARM FLASH"
layer = "hud"
frames = 8
ticks_per_frame = 2
loop = false
identity = "register effect, no art: the armed slot alternates yellow and white for 16 ticks, the whole gauge row flashes yellow for the first 4, and the TALON's two row inks swap for the 120-tick invulnerability"
colors = [7, 1]
prompt = "none (colour-register effect)"

[[vfx]]
id = "vfx.barrier_shimmer"
name = "BARRIER"
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
frames = 2
ticks_per_frame = 4
loop = true
identity = "a faceted hexagonal ring baked into alternate TALON frames, 6, 4 or 2 facets lit for 3, 2 or 1 charges, the two facet phases alternating as a shimmer; costs no extra channel"
colors = [1, 3]
prompt = "none (offline composite over the TALON frames, see ship-talon.md)"

[[vfx]]
id = "vfx.barrier_hit"
name = "BARRIER HIT"
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
frames = 3
ticks_per_frame = 2
loop = false
identity = "the barrier ring flares fully lit for one frame, then one facet cracks off and falls away; plus a white flash of the TALON's inks"
colors = [1, 3]
prompt = "none (offline composite over the TALON frames)"

[[vfx]]
id = "vfx.overdrive_flash"
name = "OVERDRIVE"
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
frames = 4
ticks_per_frame = 3
loop = false
identity = "the pods' unfold into escorts (pod.escort unfold frames) plus a register flash: border and background white for 2 ticks, the TALON's inks cycling white, cyan, yellow for 30 ticks"
colors = [1, 3, 7]
prompt = "none (uses pod.escort unfold frames and colour registers)"

[[vfx]]
id = "vfx.respawn"
name = "RELAUNCH"
layer = "sprite"
sprite_class = "B"
cells = [1, 3]
frames = 2
ticks_per_frame = 2
loop = true
identity = "a long white-and-cyan afterburner streak under the TALON as it climbs into the screen from below, shrinking to normal exhaust over 60 ticks"
colors = [1, 3]
prompt = "a long straight vertical afterburner streak seen from above, a white core inside a cyan flame, tapering to a point at the bottom"
note = "drawn in the TALON's column below the ship; the only object on screen allowed to use ink 3 besides the TALON"

[[vfx]]
id = "vfx.star_twinkle"
name = "STAR"
layer = "bucket"
size_px = [2, 2]
frames = 2
ticks_per_frame = 12
loop = true
source = "mask"
identity = "a single light grey dot that blinks; S7 night sky and S8 space star field (its own bucket, private ink light grey)"
colors = [15]
prompt = "a single white star dot"
+++

# VFX catalogue

## Layers

Every effect sits on the cheapest layer that can show it (`docs/design.md` 13):

- **bucket**: shots, bullets, debris, stars, muzzle flashes, hit sparks. Shapes are 2 to 12 display px,
  far too small for Ludo to draw usefully, so they are hand-authored pixel masks (`source = "mask"`)
  and blitted by the CPU into 24×21 bucket sprites every frame. The one Ludo bucket asset is the big
  orb (`source = "ludo"`), downsampled like any sprite.
- **sprite**: explosions, pickups, the BEAM lance and relaunch streak. Generated with `vfx_prefix`.
- **bitmap**: ground blasts and craters, baked per site over the background (patch frames).
- **hud**: colour-register effects with no art.

## Per-line sprite budget rules

The VIC-II shows 8 hardware sprites on any raster line. These rules keep every line legal; the mux
priority (`docs/design.md` 13) decides what flickers when a rule is broken by bad luck.

1. **Player band = 6.** The 42 lines the TALON covers carry the TALON (2), the inner pods (2) and two
   bucket channels (one player-shot bucket, one enemy-bullet bucket). WINGS 3's outer pods are
   composited into a bucket channel on every line they share with the band. Everything else on the band
   (air enemies, a boss's tapering rows, explosions, a SPARK) gets **at most 2 channels**.
2. **Buckets are separate per owner.** Player shots and enemy bullets never share a bucket, so their
   inks never mix: player shots yellow, enemy bullets light red (or the stage's override).
3. **No more than 2 bucket channels per line** for enemy bullets in any pattern; patterns are designed
   so their bullets cluster into 24×21 windows (rings and fans spread outward, so they cross few windows
   per line once they leave the emitter). Player shots get 2 bucket channels above the player band.
4. **BEAM** costs 1 channel on every line its lance covers, always above the TALON; pod lances add 1 each
   on their own lines. With BEAM at WINGS 3 the lines above the ship carry up to 5 lances, so BEAM
   pods are 3 rows long and start 21 px above their pods to stagger.
5. **Explosions are last in priority.** An L blast (3 channels) on a busy line drops its outer columns
   first; an M blast its right column. A dropped explosion column is never noticed; a flickering bullet
   is.
6. **Pickups outrank enemies.** A SPARK or CORE SPARK never flickers while an enemy is drawn on its
   line.
7. **Boss ceiling.** While a sprite boss with rows wider than 2 columns is on screen, those rows stay
   above playfield line 88 and the TALON cannot fly above line 88 (`05-bosses/README.md`).
8. **Bitmap is free.** Ground blasts, craters and every ground target's frames are bitmap patches; they
   cost DMA time (≤ 100 cell writes per frame), never channels. When a ground target dies, its blast and
   crater are patches, and only the air debris chips are buckets.

## Colours

- **Ink ruling:** white (1) and cyan (3) are the TALON's private inks and are reserved on the sprite
  layer (sprites and buckets) for the TALON family only: the ship, pods, escorts, barrier and relaunch
  streak. Every other sprite or bucket effect has exactly one private ink (the first entry of
  `colors`); further entries are shared-pair or HUD hints. Prompts may still say "white core" for a
  hot centre: `colour_remap` maps it to the effect's lightest legal ink (yellow for shots and blasts).
  Bitmap effects (ground blast, crater) may use any ink, white and cyan included.
- **Yellow (7) on the sprite layer is reserved for sparks, bullets (player shots) and explosions.** No
  enemy body, boss part or enemy-bullet bucket uses yellow as its private ink, so yellow always means
  "pick it up", "that is my fire" or "that just blew up".
- The proposed static shared sprite pair is **dark grey (11) + light blue (14)**; `shared_pair.py`
  makes the final choice over every frame. Effects are drawn so they read with that pair (dark
  outlines, light blue crystal facets) and degrade gracefully if it changes.
- Enemy bullets are **light red (10)** with a dark ring by default; a level file may override the
  enemy-bullet bucket ink to white (1) where its background makes light red weak. The contrast check
  runs per stage on the quantized strip.
- Explosions are yellow and orange (7, 8) in the **private inks**, chosen per cell and frame (rev 2,
  after P0: with the static pair (11, 14) and one fixed private ink, the orange ring fell to dark grey
  and the white core dithered to light blue). The shared pair carries only dark grey smoke and outline
  and a light blue "blue-white" flash rim; white is never asked for. Big, round and friendly, never
  gory. Crystal chips are the only Lattice colour in them (they quantize to light blue).

## Generation

Ludo assets use `vfx_prefix` with `--image-type sprite-vfx` at maximum size, animated on hydra for 3 s
(`--no-loop` for one-shots), curated to the frame counts above and downsampled at the display aspect.
Rotated sets (`angles > 1`) are rotated offline from one master. Bitmap effects are generated isolated
with `still_prefix` (shadow kept) and composited per site by the patch baker.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-30): P0 pilot fixes, see output/pilots/p1
- 2026-10-30: user chose ORANGE (8) for the explosion fire ring (not light red, which stays the enemy-bullet colour)
