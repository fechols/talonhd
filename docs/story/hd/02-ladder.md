+++
id = "hd.ladder"
kind = "hd_ladder"
name = "PROMOTION LADDER"
status = "approved"
revision = 3

# One rung per stage: the palette the stage shows, QUILL's rank while flying it, and the airframe.
# palette = [void, dark, light, hot] as C64 color indices (00-bible.md [palette]).
# The ship's two hull colors are always this stage's dark and light.
# light = the stage's time of day (raster3d::Light): dir toward the sun (x right, y up the screen, z toward
# the viewer), ambient fill, sun intensity. One light per stage lights its sprites AND its 3D level geometry.

[[rungs]]
stage = "stage.s1"
light = { time = "morning", dir = [-0.45, 0.55, 0.70], ambient = 0.18, intensity = 1.00, note = "the morning sun the whole S1 roster was approved under: upper left, shadows short to the lower right" }
palette = [6, 12, 1, 3]
palette_names = ["blue sea", "mid grey concrete", "white surf and hull", "cyan crystal"]
rank = "CAPTAIN"
ship = "hd.ship.talon"
clear_line = "MAJOR VASQUEZ. THE TEN IS FUELLED. TRY TO BRING THIS ONE BACK."
quill_reply = "IT'S A TEST PROGRAMME, MARSHAL. I'M TESTING."

[[rungs]]
stage = "stage.s2"
light = { time = "afternoon", dir = [0.55, 0.35, 0.65], ambient = 0.20, intensity = 1.00, note = "warm afternoon from the upper right: the terraces step into their own shade" }
palette = [9, 8, 15, 14]
palette_names = ["brown soil", "terracotta", "cream stone", "light blue crystal"]
rank = "MAJOR"
ship = "hd.ship.peregrine"
clear_line = "LIEUTENANT COLONEL. THE ELEVEN WAS BUILT FOR THE DESERT. SO ARE YOU NOW."
quill_reply = "I'LL PACK SUNSCREEN."

[[rungs]]
stage = "stage.s3"
light = { time = "hard sun", dir = [-0.70, 0.30, 0.45], ambient = 0.08, intensity = 1.10, note = "a hard low sun on the canyon: long black shadows, almost no fill" }
palette = [2, 8, 7, 3]
palette_names = ["red rock", "orange sand", "yellow sun", "cyan ore"]
rank = "LT COLONEL"
ship = "hd.ship.caracara"
clear_line = "COLONEL. THE TWELVE HAS NEVER FLOWN. NEITHER HAD THE OTHERS."
quill_reply = "THAT'S WHAT I LIKE ABOUT THEM."

[[rungs]]
stage = "stage.s4"
light = { time = "overcast", dir = [-0.20, 0.50, 0.84], ambient = 0.35, intensity = 0.70, note = "overcast over the canals: soft light, short pale shadows" }
palette = [11, 12, 7, 3]
palette_names = ["dark grey steel", "mid grey concrete", "safety yellow", "cyan pipe glow"]
rank = "COLONEL"
ship = "hd.ship.bateleur"
clear_line = "BRIGADIER. THE THIRTEEN IS OUR LAST CONTINENTAL AIRFRAME. MAKE IT COUNT."
quill_reply = "I ALWAYS MAKE THEM COUNT. I JUST DON'T RETURN THEM."

[[rungs]]
stage = "stage.s5"
light = { time = "late day", dir = [-0.80, 0.15, 0.45], ambient = 0.15, intensity = 0.95, note = "the late-day siege: a low sun from the left throws the walls' shadows across the courts" }
palette = [11, 5, 15, 3]
palette_names = ["dark grey shadow", "olive glacis", "light stone", "cyan conduits"]
rank = "BRIGADIER"
ship = "hd.ship.augur"
clear_line = "MAJOR GENERAL. THE FOURTEEN HAS THEIR CRYSTAL IN ITS WINGS. IT LISTENS NOW."
quill_reply = "THEN IT CAN LISTEN TO ME."

[[rungs]]
stage = "stage.s6"
light = { time = "humid", dir = [0.30, 0.60, 0.74], ambient = 0.30, intensity = 0.80, note = "humid jungle light, diffuse through the haze" }
palette = [6, 5, 13, 3]
palette_names = ["deep blue shade", "green canopy", "light green leaves", "cyan crystal flowers"]
rank = "MAJ GENERAL"
ship = "hd.ship.kite"
clear_line = "LIEUTENANT GENERAL. THE FIFTEEN CLIMBS FASTER THAN ANYTHING WE HAVE."
quill_reply = "GOOD. I'M IN A HURRY."

[[rungs]]
stage = "stage.s7"
light = { time = "dusk", dir = [0.75, -0.20, 0.40], ambient = 0.22, intensity = 0.80, note = "dusk above the clouds: the last sun low from the right, from below the horizon line of the screen" }
palette = [0, 4, 15, 7]
palette_names = ["night", "purple dusk", "light grey cloud", "yellow lightning"]
rank = "LT GENERAL"
ship = "hd.ship.lanner"
clear_line = "GENERAL. THE SIXTEEN IS EVERYTHING WE LEARNED. BRING HER HOME."
quill_reply = "YES, MARSHAL. ALL THE WAY UP AND BACK."

[[rungs]]
stage = "stage.s8"
light = { time = "orbit", dir = [-0.50, 0.65, 0.57], ambient = 0.05, intensity = 1.15, note = "unfiltered sunlight in orbit: hard edges, black shade" }
palette = [0, 6, 1, 3]
palette_names = ["space", "cobalt", "white-hot", "cyan crystal"]
rank = "GENERAL"
ship = "hd.ship.roc"
clear_line = "AIR MARSHAL VASQUEZ. WELCOME HOME."
quill_reply = "SAME RANK AS YOU, MARSHAL. WHO SALUTES FIRST?"

[ending]
rank = "AIR MARSHAL"
note = "after the CROWN falls QUILL reaches BRANDT's rank; the ending card adds the line under THE SKY IS OURS AGAIN."
+++

# Promotion ladder

## The mechanic

At every stage clear QUILL is **promoted**, and BRANDT hands them the Accord's next prototype. The Accord
"built one of everything and tested it until it broke" (base story, backstory). Now every one of those
untested airframes, sitting in the MERIDIAN's hold, is the only aircraft that can fly through the Lattice.
Each one comes out of the hold for exactly one theatre.

The promotion is the stage-clear reward, and it is visible three ways at once:
1. **A new rank** on the stage-clear card, with one line from BRANDT and one dry reply from QUILL.
2. **A new ship**: a new model with a new silhouette and its own feel (gameplay stats per ship in
   `ships/*.md`).
3. **A new world**: the palette walks from this stage's four colors to the next stage's four
   (`fx.promotion_walk`). The new ship rolls out already in the new colors, because its hull colors
   *are* the new stage's `dark` and `light`.

Carried over between ships: score, lives, the arming gauge and its armed levels (design.md). The
airframes share the TALON's hard-points; that's why the sparks still fit.

## The story beat

- **S1-S4, the continent.** Ships 1-4 are pure Accord hardware, with no crystal.
- **The twist (after S5).** The S5 airframe, the AUGUR, is the last Accord-only design. When the Citadel
  turns out to be a relay, the MERIDIAN's engineers start building captured Lattice crystal into the next
  airframes so they can climb.
- **S6-S8, the ascent.** Ships 6-8 carry `hot` crystal in their structure, more each time: the KITE's
  leading edges, the LANNER's spine, and the ROC's heart. The player's ship starts to look a little like
  the enemy. That is on purpose: it is the price of going up.

## Ranks

CAPTAIN → MAJOR → LT COLONEL → COLONEL → BRIGADIER → MAJ GENERAL → LT GENERAL → GENERAL → AIR MARSHAL
(after the CROWN). The words are generic; no real-world insignia is ever drawn. The joke that BRANDT is
outranked by the ending is the game's last line of dialogue.

## Palettes

| Stage | void | dark | light | hot | Ship (hull: dark / light) |
|---|---|---|---|---|---|
| S1 BREAKWATER | 6 blue | 12 mid grey | 1 white | 3 cyan | TALON: white / grey |
| S2 TERRACES | 9 brown | 8 terracotta | 15 cream | 14 light blue | PEREGRINE: cream / terracotta |
| S3 REDLINE | 2 red rock | 8 orange | 7 yellow | 3 cyan | CARACARA: yellow / orange |
| S4 SLUICEWORKS | 11 dark grey | 12 mid grey | 7 yellow | 3 cyan | BATELEUR: yellow / grey |
| S5 BASTION | 11 dark grey | 5 olive | 15 stone | 3 cyan | AUGUR: stone / olive |
| S6 VERDIGRIS | 6 deep blue | 5 green | 13 light green | 3 cyan | KITE: light green / green, cyan edges |
| S7 STORMFRONT | 0 night | 4 purple | 15 cloud grey | 7 yellow | LANNER: grey / purple, yellow spine |
| S8 SKYHOOK | 0 space | 6 cobalt | 1 white | 3 cyan | ROC: white / cobalt, cyan heart |

S1 and S8 are bookends: white over blue, cyan crystal, the TALON and the ROC. S7 is the only stage where
the Lattice is yellow: inside the storm, the crystal glows like the lightning.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-11-01): a `light` (time of day) per stage, lighting both its sprites and its 3D level geometry (user: "each level could have a different time of day"). S1's is the light every approved model already used (no visual change); S2-S8 are drafts pending approval.
- rev 3 (2026-11-02): S2's light (afternoon, upper right) approved with `hd/models/s2.md` ("Approve, build TERRACES"); S3-S8 remain drafts.
