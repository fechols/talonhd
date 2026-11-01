+++
id = "pod.escort"
kind = "ship"
name = "ESCORT"
faction = "accord"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
expanded = false
angles = 1
anchor = "center"
identity = "an uncrewed white escort fighter seen from directly above, nose up, a slim arrowhead with long straight wings, no canopy, a dark nose sensor slot"
silhouette = "long needle nose, slim arrowhead wings with square tips, single tail fin, one wide engine nozzle"
proportions = "36 display px span by 40 px long; noticeably narrower than the TALON so the player never confuses the two"
materials = "painted metal panels with chunky panel lines, a dark glass sensor slot, one heat-stained nozzle"
acting = "fierce and precise; holds formation rigidly, snaps to its slot, fires in perfect sync with the TALON"
forbidden = ["canopy or pilot", "cyan", "twin engines", "delta wings like the TALON", "markings", "red or orange paint", "side view"]
hurtbox = { x = -14, y = -16, w = 28, h = 32 }
private_inks = [1]
cell_inks = { top_row = 1, bottom_row = 1 }
invulnerable = true

[[colors]]
role = "primary"
c64 = 1
note = "white upper surfaces in both cell rows (private ink)"
required = true

[[colors]]
role = "shared_light"
c64 = 14
note = "shared pair light tone (proposed light blue): wing panels, tail fin"
required = false

[[colors]]
role = "shared_dark"
c64 = 11
note = "shared pair dark tone: outline, sensor slot, nozzle"
required = false

[[animations]]
id = "unfold"
frames = 4
ticks_per_frame = 3
loop = false
model = "hydra-plus"
duration = 3
request_frames = 25
motion = "a small teardrop pod splits open, long wings slide out from its sides, the nose extends forward and the engine lights, ending as a slim fighter"

[[animations]]
id = "idle"
frames = 2
ticks_per_frame = 3
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "holding position, the single engine flame flickers short and long, the airframe perfectly still"

[[animations]]
id = "fold"
frames = 4
ticks_per_frame = 3
loop = false
model = "offline"
duration = 3
request_frames = 25
motion = "unfold played in reverse"

[ludo]
concept_prompt = "An uncrewed escort fighter drone for a retro-future air force, seen from directly above, nose pointing to the top of the image. A long needle nose with a dark sensor slot, slim arrowhead wings with square tips, one tail fin, one wide engine nozzle with a short flame. White upper surfaces, mid grey wing panels, dark grey panel lines. Chunky 1980s toy-model proportions, slimmer than a delta fighter. No cockpit, no markings."
sprite_prompt = "the escort drone centred, nose straight up, wings level, engine burning"
+++

# ESCORT

## Role

The OVERDRIVE showcase. Arming WINGS while already at WINGS 3 unfolds all four pods into full-size
escort fighters for 600 ticks (10 s NTSC). The screen suddenly holds five 2×2 fighters in formation, all
firing at double rate: the moment the game exists to show that this machine can draw "bigger and more".

## Formation and channel budget

Escorts fly in a **wedge split across two bands**, so no raster line ever carries more than two 2×2
fighters:

| Escort | Offset from the TALON (display px) | Lines relative to the TALON |
|---|---|---|
| lead left / right | (−52, −46) / (+52, −46) | −67 .. −25 (above the TALON) |
| trail left / right | (−60, +46) / (+60, +46) | +25 .. +67 (below the TALON) |

- TALON band: 2 channels. Lead band: 4. Trail band: 4. Each band leaves room for the player-shot bucket
  and one or two enemy-bullet buckets inside 8.
- Escorts snap to their slots (1/2 of the remaining distance per tick); at the playfield edge the whole
  wedge compresses inward rather than leaving the screen. Near the top or bottom edge the lead or trail
  pair swaps to the free side so it stays on screen.
- Mux priority during OVERDRIVE: player > escorts > player shots > enemy bullets > enemies. During a
  boss fight the boss outranks the escorts, and escorts are the first friendly objects to flicker
  (`05-bosses/README.md`, rule 6). While a sprite-boss ceiling is active (TALON below line 88) the lead
  pair flies beside the TALON at (±56, 0) as 1×1 pods instead of unfolding, so the band stays legal.

## Behaviour

- Each escort fires the current main weapon with the **TALON's own base pattern and damage** (not the
  reduced pod values), at the OVERDRIVE double rate. FLANK stays on the TALON only.
- Escorts are invulnerable. They absorb enemy bullets without harm and ram air enemies for 2 damage per
  4 ticks of contact. Ground targets do not touch them.
- When OVERDRIVE ends (or the TALON is destroyed) every escort plays `fold`, the trail pair is lost and
  WINGS drops to 2 (see `docs/design.md` 7).

## Colour notes

White in both cell rows and no cyan: the escorts are clearly "ours" (white) and clearly not the player
(no cyan canopy, slimmer silhouette). The player must always be able to find the TALON in the wedge.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
