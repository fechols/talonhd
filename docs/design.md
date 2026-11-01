+++
id = "design"
kind = "design"
name = "TALON DESIGN SPEC"
status = "review"
revision = 1
+++

# TALON — game design spec

Numbers are starting values, tuned on hardware in M3-M5. Everything in a table here is **data** (REU
tables built from this file and the bible); only state machines are code. Hardware facts come from
`docs/story/00-bible.md` and `docs/story/SCHEMA.md`; names come from the bible.

## 1 Controls

| Input | In play | In menus |
|---|---|---|
| Joystick (port 2), 8 ways | fly; instant velocity, no inertia | move the cursor |
| Fire (held) | fire the main weapon, FLANK and pods at their own rates (built-in autofire); hold for BEAM | confirm |
| Fire 2 (POTX, C64GS wiring) | **ARM** the highlighted gauge slot | back |
| SPACE | ARM (fallback for one-button sticks) | back |
| RUN/STOP | pause / resume | back |
| F1 | — | start a game from the title or attract |

Input is the 8-bit mask UP, DOWN, LEFT, RIGHT, FIRE, ARM, PAUSE, START. Fire 2 and SPACE OR into ARM.
ARM is edge-triggered (one arm per press); Fire is level-triggered. Hardware autofire must be off. Full
mapping and the host keys are in `docs/controls.md`.

## 2 Units, ticks, fixed point

- **One simulation tick per screen refresh: 60 Hz on NTSC (59.83), 50 Hz on PAL.** Every refresh shows
  exactly one step, so movement never hitches. All numbers here are authored in **NTSC ticks**; the
  build generates the PAL table from them, as pummel does:
  - velocities and per-tick distances × 6/5 (scroll 0.5 → 0.6 px/tick; SPEED 0 1.5 → 1.8 px/tick);
  - accelerations × 36/25;
  - durations (timers, invulnerability, fire intervals, animation holds) × 5/6, rounded to whole ticks
    (TWIN interval 6 → 5, arming invulnerability 120 → 100);
  - everything is Q8, so the scaling error stays below 1/256 px per tick.
  The simulation reads the active table (`TickDef`, chosen at boot from the measured line count).
  Replays record the standard they were captured on. Stage lengths are in pixels, so a stage takes the
  same number of seconds on both standards.
- **Logical pixel** = display pixel horizontally (320 wide) and one raster line vertically. Multicolour
  art pixels are 2 logical px wide; all gameplay math is in logical px.
- **Playfield coordinates:** x 0..303 (screen x 8..311), y 0..175 (top of the playfield = 0). World
  scroll position `scroll_y` counts lines from the bottom of the stage strip upward.
- Positions `i16` px + Q8 sub-pixel; velocities Q8 px/tick; angles in 1/64 turns (`u8`, 0 = up,
  clockwise); 16-angle art uses `angle >> 2`, 8-angle art `angle >> 3`.
- A **screen** in level files is 200 lines of travel. At the base scroll of 0.5 px/tick a screen takes
  400 ticks (6.7 s).

## 3 Movement and speed

8-way digital movement with instant velocity (no acceleration; the genre's "on rails" feel). Diagonals
are scaled by 181/256 so diagonal speed equals straight speed.

| SPEED level | Q8 px/tick | px/tick | px/s (NTSC) | Crosses 304 px in |
|---|---|---|---|---|
| 0 (start) | 384 | 1.50 | 90 | 3.4 s |
| 1 | 448 | 1.75 | 105 | 2.9 s |
| 2 | 512 | 2.00 | 120 | 2.5 s |
| 3 | 576 | 2.25 | 135 | 2.3 s |
| 4 | 640 | 2.50 | 150 | 2.0 s |
| 5 (max) | 704 | 2.75 | 165 | 1.8 s |

Modifiers (added in Q8, applied after the table; result clamped to ≥ 320):

| Condition | Q8 | Why |
|---|---|---|
| WINGS ≥ 1 (widened airframe) | −48 | the wingtip extensions add drag: "bigger is slower" |
| BEAM held | −64 | the lance drags; aiming a beam is a commitment |
| OVERDRIVE | +64 | the escorts pull the TALON along |

- Arming SPEED at level 5 **wraps to level 1**, so a player who overshoots can come back down without
  dying (a gauge of only ups is otherwise a trap).
- Bounds: the TALON's anchor stays in x 24..279 and y 21..155. Pods and escorts are clamped separately
  (see `02-characters/pod-*.md`).
- **Boss ceiling:** while a sprite boss with any row wider than 2 columns is on screen, the TALON's
  top edge is clamped at playfield line 88 (anchor y ≥ 109), and the boss keeps its wide rows above
  that line (`05-bosses/README.md`, rule 2). The clamp eases in over 30 ticks so it never snaps.
- Banking frames follow the stick (`ship-talon.md`); banking has no effect on movement.
- The scroll never stops for the player. It slows only where a level file says so (ANVIL's hull).

## 4 Screen layout

```
row 0   1UP 0000000     HI 0100000      @x3      <- text row (hires), score row
        ┌──────────────────────────────────────┐
        │                                      │
        │   playfield: multicolour bitmap,     │ <- 304 x 176 logical px
        │   24-row mode, REU-streamed view,    │    (22 cell rows visible)
        │   YSCROLL fine scroll                │
        │                                      │
        └──────────────────────────────────────┘
row 24     SPEED TWIN FLANK WINGS BURST BEAM SEEKR BARIR   <- text row, the arming gauge
```

- 320×200 screen; the **304-px safe area** is x 8..311 (38 columns); the outer 8 px each side are
  border-coloured by the 38-column mode, so objects enter from the sides hidden.
- **Top row (text, hires):** score, high score, lives. During boss and midboss fights the HI field is
  replaced by the boss bar (`07-ui-text.md`).
- **Playfield:** the bitmap in 24-row mode between the two text rows, 176 visible lines. The exact
  split raster lines are fixed by the M1 spike (a) and recorded in `docs/architecture.md`; until then
  the playfield is specified as 176 lines.
- **Bottom row (text, hires):** the **arming gauge**, eight slots drawn with a half-width HUD font
  (two 4-px glyphs per character cell), so each 5-letter label fits in 3 cells and each slot is a 4-cell
  box: 8 × 4 = 32 cells, centred in the 38-column safe area with 3 cells spare on each side. The right
  spare shows the OVERDRIVE countdown.
- Background ink `$D021` is the stage's shared bitmap colour; the HUD rows use black behind white text
  with the gauge colours in section 6.

## 5 Hitbox, invulnerability, damage

- **One-hit death.** Any enemy bullet or air-enemy body touching the hitbox destroys the TALON unless
  BARRIER absorbs it. Ground, hull and cable objects never touch the TALON (it flies over them): their
  `contact_damage` is always 0, and only their bullets are dangerous. Air enemies have
  `contact_damage = 1` (kills an unshielded TALON).
- **Hitbox:** a square centred 2 px below the anchor (on the canopy). It grows one step per *heavy*
  system:

  `step = (main weapon ≠ TWIN ? 1 : 0) + (FLANK ? 1 : 0) + WINGS level`, clamped to 0..4.

  | Step | Hitbox (display px) |
  |---|---|
  | 0 | 4 × 4 |
  | 1 | 6 × 6 |
  | 2 | 8 × 8 |
  | 3 | 10 × 10 |
  | 4 | 12 × 12 |

  SPEED and BARRIER do not grow the hitbox. OVERDRIVE does not change it.
- **Invulnerability windows** (the ship's inks swap or blink, `ship-talon.md`):

  | Event | Ticks | Notes |
  |---|---|---|
  | ARM (any slot, including a refit) | 120 | white ↔ cyan swap every 4 ticks |
  | Respawn | 180 | 2-on / 2-off blink; firing allowed |
  | Boss defeated | until stage clear | the player cannot die after the last hit |
  | BARRIER absorbs a hit | 30 | so one burst cannot strip all three charges |

  Invulnerability windows do not stack; a new one replaces the remaining time if it is longer.
- Enemies have no invulnerability; they flash (private ink → white) for 2 ticks per hit. Bosses flash
  for 2 ticks and never have invulnerability windows except between phases (60 ticks).

## 6 The arming gauge and weapons

### 6.1 SPARKS and the cursor

- **SPARKS** are yellow chevron pickups (`vfx.spark`) dropped by formation leaders, carriers, gunships
  and destroyed ground nodes (`drops = { spark = n }` in each enemy file). A spark drifts down at
  0.5 px/tick relative to the screen with a slow ±8 px sway, lives until it leaves the bottom, and is
  collected by touching it with a 16×16 pickup box (larger than the hitbox). Collecting scores 100.
- The **cursor** `c` is 0 (nothing lit) or a slot 1..8. Each spark advances it one slot:
  `c = c mod 8 + 1`. Wrapping from 8 to 1 scores a **WRAP bonus** of 2,000 (the player passed on
  every system).
- **ARM** (Fire 2 / SPACE) while `c > 0` commits slot `c`, then sets `c = 0`. Arming is ignored while
  `c = 0`, during death, respawn launch and stage clear.

### 6.2 Slots

| Slot | Label | ARM effect | When already at max / already held |
|---|---|---|---|
| 1 | SPEED | SPEED +1 | at 5: wraps to 1 |
| 2 | TWIN | main weapon = TWIN | refit |
| 3 | FLANK | FLANK on (stacks with any main weapon) | refit |
| 4 | WINGS | 0→1 widen, 1→2 inner pods, 2→3 outer pods | at 3: **OVERDRIVE** (section 7) |
| 5 | BURST | main weapon = BURST | refit |
| 6 | BEAM | main weapon = BEAM | refit |
| 7 | SEEKER | main weapon = SEEKER | refit |
| 8 | BARRIER | charges = 3, drain timer reset | at 3 charges: refit |

- TWIN, BURST, BEAM and SEEKER are **mutually exclusive**: the main weapon slot holds one of them. The
  game starts with TWIN.
- **Refit** (original rule): arming something already held spends the sparks for the 120-tick arming
  invulnerability and 500 points. It turns the gauge into a panic button, at the cost of progress.
- Every ARM plays `sfx.arm`, flashes the slot (`vfx.gauge_arm_flash`) and swaps the TALON's inks.

### 6.3 Gauge display

| State | Colour | Notes |
|---|---|---|
| cursor slot | reverse yellow (7), blinking 8 on / 8 off | the slot ARM would commit |
| active system (held weapon, FLANK on, BARRIER charged) | white (1) | |
| maxed (SPEED 5) | light grey (15) | label shows the state, see `07-ui-text.md` |
| available | mid grey (12) | |
| ARM flash | yellow ↔ white every 2 ticks for 16 ticks | the whole bottom row turns yellow for 4 ticks |

Labels change with state (SPEED shows its level, WINGS shows OVDRV at 3, BARRIER shows its charges):
the full label set is in `07-ui-text.md`.

### 6.4 Weapon table (NTSC ticks; damage in enemy HP)

All player shots are drawn on the **bucket** layer (player-shot buckets, never mixed with enemy
bullets) except BEAM, which is a sprite column. Every weapon hits both air and ground targets.

| Weapon | Damage | Interval (ticks) | Speed (px/tick) | Range | Base pattern (W0) | WINGS 1 adds (wingtips) | Each pod (W2: 2 pods, W3: 4 pods) |
|---|---|---|---|---|---|---|---|
| TWIN | 1 / bolt | 6 | 8.0 | to the top edge | 2 bolts at x ±4, straight up | +2 bolts at x ±20, straight up | 1 bolt straight up, dmg 1, interval 8 |
| FLANK | 1 / shot | 10 | 6.0 | 112 px (19 ticks) | 2 shots horizontally left and right from the wing roots (x ±16) | +2 shots at ±45° forward from the wingtips | — (pods never flank) |
| BURST | 3 / pellet | 20 | 5.0, slowing 0.125/tick | ≈ 72 px (18 ticks) | 5 pellets at 0°, ±16°, ±32° | +2 pellets at ±48° | 3 pellets at 0°, ±16°, dmg 2, interval 24 |
| BEAM | 1 per 4 ticks per target touched | continuous while Fire is held | lance grows 12 px/tick | 126 px (6 cell rows) | one lance 4 px wide from the nose | lance 8 px wide, max 168 px | a 63 px lance, 1 per 8 ticks |
| SEEKER | 5 / dart | 36 per salvo | 3.0, +0.125/tick to 6.0 | 150-tick life | one dart per on-screen target (nearest first), max 4; no targets: radial 8 darts at 45° steps, straight, dmg 3 | max 6 darts per salvo | +1 dart per pod at the nearest unassigned target, dmg 3 |

Weapon details:

- **TWIN** is the dependable default: the fastest bolts, full-screen range, weakest per hit. Live cap
  24 bolts.
- **FLANK** answers enemies that enter from the sides and ground turrets beside the flight line.
  Live cap 12.
- **BURST** is wide 5-way flak with the strongest damage and a slow reload; pellets slow down as they
  fly, so it rewards flying close. Live cap 21.
- **BEAM** is hold-to-extend:
  - holding Fire grows a lance from the nose at 12 px/tick to its maximum; it moves with the ship and
    damages everything it overlaps every 4 ticks;
  - it **pierces ground targets** and unarmoured air units; an **armoured** air unit
    (`resists = "beam"`) stops the lance at its hurtbox and takes only 1 per 8 ticks;
  - **releasing Fire throws the lance**: it detaches and flies up at 10 px/tick, dealing 3 once to
    everything it passes, and a new lance can start 16 ticks later. Tapping Fire throws short lances,
    which is the BEAM's answer to armour;
  - MIRROR units (S8) reflect the lance (enemy file).
- **SEEKER** fires one homing dart per target. Darts turn 1/32 turn per 2 ticks toward their target, are
  re-targeted to the nearest target when theirs dies, and fly straight when none remain. Ground and
  air targets count equally. With no target on screen the salvo becomes a radial ring of 8 straight
  darts (dmg 3). Darts never cancel enemy bullets. Live cap 12.
- **Pods** mirror the main weapon at the pod values above; they fire with the TALON (same trigger, own
  interval). Pods add no FLANK.

### 6.5 Hidden damage multipliers

- Weakness (`weak_to`): **×2 damage**. For bosses and midbosses it applies to their weak parts (other
  parts take normal damage), as `05-bosses/README.md`; ordinary enemies take ×2 on their whole
  hurtbox. Boss weaknesses S1-S8: BURST, BEAM, SEEKER, BURST, BEAM, SEEKER, BURST, SEEKER.
- Resistance (`resists`): **×0.5**, rounded down, with odd remainders carried so every second hit of 1
  still lands 1.
- OVERDRIVE halves every interval (TWIN 3, FLANK 5, BURST 10, BEAM damage tick 2, SEEKER 18, pods
  likewise). Damage per hit is unchanged.

## 7 WINGS, pods and OVERDRIVE

- **WINGS 1** swaps the TALON to its widened frames (`wings1_*`), adds the wingtip shots in the table,
  costs −48 Q8 speed and one hitbox step.
- **WINGS 2** adds the inner pod pair; **WINGS 3** the outer pair (`pod-wing.md` for slots and lag).
- **Pods block enemy bullets.** A pod hit by a bullet or an air-enemy body absorbs it and is destroyed
  **together with its mirror partner**; WINGS drops one level (3 → 2, 2 → 1). The TALON is unharmed.
  Pods are a shield that costs power.
- **OVERDRIVE** (original twist): arming WINGS at WINGS 3.
  - The four pods unfold into full-size **escort fighters** (`pod-escort.md`, 12-tick unfold) in a
    two-band wedge.
  - Lasts **600 ticks** (10 s). The gauge's right spare cells count down the seconds.
  - Every gun (TALON, FLANK, escorts) fires at **double rate**; escorts fire the TALON's base pattern at
    full damage.
  - Escorts are invulnerable, absorb bullets and ram air enemies (2 per 4 ticks).
  - `cue.overdrive` (a 10-s sting) plays over the music, which is ducked to half volume; `sfx.overdrive`
    roars on a centred chip pair.
  - The TALON gets +64 Q8 speed and the usual 120 arming ticks of invulnerability.
  - **When it ends** (timer, or the TALON dies) the escorts fold back into pods, the trail pair peels off
    and **WINGS drops to 2**. OVERDRIVE is a spend, not a permanent upgrade: the player must rebuild
    WINGS 3 and find another WINGS spark to repeat it.
  - Arming during OVERDRIVE works normally; arming WINGS again during OVERDRIVE is a refit.

## 8 BARRIER

- ARM BARRIER sets **3 charges**. Each charge absorbs one bullet or air-enemy contact anywhere on the
  hitbox, then gives 30 ticks of invulnerability (`vfx.barrier_hit`, `sfx.barrier_hit`).
- **Drain:** the barrier also loses one charge every **1,200 ticks** (20 s) of play, so it is a
  temporary safety, not a permanent extra life. The drain timer resets when BARRIER is armed.
- The barrier is drawn as a faceted ring baked into alternate TALON frames (`ship-talon.md`): 6, 4 or 2
  facets lit for 3, 2 or 1 charges. It costs no extra sprite channels.
- BARRIER does not grow the hitbox and does not protect pods.

## 9 Death, checkpoints, respawn, continue

- **Death strips everything:** SPEED 0, TWIN, FLANK off, WINGS 0 (pods lost), BARRIER 0, OVERDRIVE ends.
- **Sequence:** death frames (15 ticks) → `vfx.explosion_l` + `sfx.player_down` → 120 ticks in which the
  stage keeps scrolling and enemies keep flying but no new waves spawn → the view rewinds to the last
  checkpoint passed (the REU view is simply re-fetched) → the TALON launches up from below the screen
  over 60 ticks (`vfx.respawn`) → control returns with 180 ticks of invulnerability.
- **Resupply:** after a respawn the gauge cursor is set to slot 1 (SPEED highlighted), so the first
  ARM is free. It is the only gift the game gives.
- **Checkpoints:** 3 per stage (`checkpoints` in each level file); the third sits just before the boss.
  Ground objects destroyed before the checkpoint stay destroyed; waves after it respawn.
- **Bosses and midbosses keep their damage** when the player dies during the fight; the player
  relaunches at the boss checkpoint into the same fight. (The boss is not reset; the fight is not free.)
- **Lives:** 3 to start (OPTIONS: 3 or 5), maximum 9 in stock.
- **Game over:** if the score qualifies, NAME ENTRY first; then CONTINUE with a 10-s countdown (loop 1
  only, at most 9 continues). A continue gives 3 lives, **resets the score to 0** and records the number
  of continues in the score's last digit (scores are always multiples of 10, so the digit is free). The
  player relaunches at the last checkpoint passed with the resupply rule.

## 10 Scoring, extends and hidden bonuses

- Score counts in tens; the **counterstop** is 9,999,990.
- **Extends** at 50,000 and 150,000, then every 200,000 (350,000, 550,000, …). With 9 ships in stock an
  extend pays 20,000 instead. `sfx.extend` plays.
- **Enemy score guidance** (exact values live in each enemy file):

  | Class | Score |
  |---|---|
  | S air | 100-300 |
  | W air | 300-500 |
  | M air | 500-1,500 |
  | L air / gunship | 2,000-5,000 |
  | ground turret, hatch, mortar | 300-800 |
  | tank, rail car | 500-1,000 |
  | big orb shot down | 50 |
  | midboss | 10,000-20,000 |
  | boss | 30,000-60,000, plus time bonus |

- **Boss scores** (`05-bosses/README.md`): midbosses 10,000 rising 2,000 per stage (S8 25,000);
  bosses 30,000 rising 10,000 per stage; the CROWN 200,000 (100,000 per phase).
- **Boss time bonus:** `max` if the fight takes ≤ `par_s` seconds, then falling linearly to zero at
  `2 × par_s`; `time_bonus = { par_s, max }` is in each boss file.
- **CORE SPARK** (the weapon-reward cycle): every stage boss releases a large gold CORE SPARK
  (`vfx.spark_core`). Collecting it moves the gauge cursor straight to the slot of the weapon the
  **next** boss is weak to, without arming it, and scores 10,000, or 50,000 if that weapon is already
  armed. The cursor survives the stage-clear screen, so the player can ARM on the next stage. Rewards:
  GANTRY QUEEN → BEAM, ASTROLABE → SEEKER, TRESTLE TITAN → BURST, HYDRA PUMP → BEAM, QUADRANT →
  SEEKER, ORCHID → BURST, ANVIL → SEEKER, CROWN → BURST (next loop). Midbosses drop 3 ordinary sparks.
- **Crystal goes dark:** when a stage boss falls, every enemy still on screen is destroyed and scores
  normally (a chain of `vfx.explosion_s`), and every enemy bullet becomes a 10-point star that flies to
  the score.

### Stage-clear tally

| Line | Value |
|---|---|
| BOSS TIME | from the boss file |
| CLEAN AIRFRAME | 10,000 × stage number if no ship was lost in the stage |
| SECTION SWEEP | 5,000 per section whose every ground node was destroyed |
| GOLD SPARK | 10,000 if found |

### Hidden bonuses (original; never explained in game, only in the tally)

1. **GOLD SPARK.** One per stage, under an ordinary-looking ground decal chosen in the level file.
   Four hits on the spot reveal a gold chevron (`vfx.spark_gold`); collecting it advances the cursor two
   slots and is worth 10,000.
2. **SECTION SWEEP.** Destroy every ground node a level file tags to a section: 5,000 per section,
   paid in the tally.
3. **FORMATION CHAIN.** Destroying every member of a formation pays the chain value; consecutive
   complete formations double it: 200, 400, 800, 1,600, 3,200, then 6,400 for every further one. A
   formation that escapes with any member alive resets the chain to 200. The value pops up as a small
   number where the last member died.
4. **TAILORED KILL.** Landing the killing hit on a boss with the weapon it is weak to doubles its time
   bonus.
5. **LIGHT TRAVELLER.** Clearing a stage without arming anything (refits included): 30,000. Rank stays
   low as a side effect.
6. **MERIDIAN TAG** (hidden extend). Two per loop, placed in the S3 and S6 level files: an Accord
   supply crate in the bitmap that only a BEAM lance (held or thrown) can open. It drops a white flight
   tag; collecting it is a 1UP.
7. **WRAP** (section 6.1, 2,000) and **REFIT** (section 6.2, 500) are small visible bonuses that teach the
   gauge.

## 11 Stage, midboss and boss flow

`StageCard (150 or Fire) → Launch (60, loop 1 S1 only from the carrier) → Section 1 → CP1 → Section 2
→ Midboss → CP2 → transition set-piece → Section 3 → CP3 → BossWarning → Boss → StageClear`.

- **Midboss:** `WARNING` caption on the top row for 90 ticks with `sfx.warning`, then `cue.midboss`.
  The scroll continues unless the level file pins it. Midbosses that leave the screen unbeaten escape
  (no points, no penalty).
- **Boss:** the scroll slows to the level file's arena speed (usually 0, or a slow crawl);
  `BossWarning` shows the warning card on the top row for 120 ticks (`07-ui-text.md`) with the border
  flashing light red every 8 ticks and `sfx.warning`; the boss bar fills (1 segment per 2 ticks,
  `sfx.gauge_step`); `cue.boss` starts. Phases follow the boss file. Between phases the boss is
  invulnerable for 60 ticks.
- **Defeat:** the boss freezes for 30 ticks flashing, then `vfx.explosion_boss_chain` (120 ticks); the
  crystal goes dark (palette swap of the stage's crystal inks to the dark variant over 32 ticks); the
  player is invulnerable; StageClear tally; next StageCard.

## 12 Rank and loops

Rank `R` (0..255) drives enemy aggression. It is computed every tick from three parts:

- **Survival `S`:** +1 per 240 ticks alive (cap 128). A death subtracts 40 (minimum 0).
- **Power `P`:** `2 × SPEED + 4 × (main ≠ TWIN) + 3 × FLANK + 5 × WINGS + 2 × (BARRIER > 0)`, 0..34.
- **Loop base `L`:** `min(144, 48 × (loop − 1))`; on every 8th loop (8, 16, …) `L = 0`, a **rest loop**.
- Difficulty adds −32 (EASY), 0 (NORMAL) or +48 (HARD).

`R = clamp(S + 2P + L + difficulty, 0, 255)`

| Effect | Formula |
|---|---|
| enemy bullet speed | × (1 + R / 512) × loop factor, loop factor = 1 + min(loop − 1, 4) / 16 (1 on rest loops); capped at × 1.75 |
| enemy fire cooldowns | × (1 − R / 640) (down to 60%) |
| aiming | R ≥ 96: aimed shots lead the TALON by half its velocity |
| rank upgrades | R ≥ 192: enemies whose file allows `rank_upgrade` fire their next pattern (single → 3-way) |
| extra waves | R ≥ 128: the level file's `rank_extra` waves spawn |
| **ECHO SHARDS** | loop ≥ 2: Lattice-grown enemies flagged `echo` release one slow crystal shard aimed at the TALON on death, with probability R / 256 |

**Loops.** After the CROWN, the ending and the credits, the loop card plays and S1 begins again with
loop + 1. Score, lives and the gauge carry over. Loop 2+ uses crystal-plated variants where enemy files
list them (+50% HP, same art with the crystal ink swapped in). There is no final loop.

## 13 Render layers and budgets

Each object class lives on the layer that costs the least (plan "Rendering architecture").

| Layer | Holds | Cost | Budget |
|---|---|---|---|
| **bitmap** | the stage strip; ground enemies, turrets, tanks on fixed paths, hatches, fortresses, wreckage, craters, set-pieces | zero sprites; REU DMA of cell rectangles into both VIC banks | ≤ 100 cell writes per frame (both buffers) outside the coarse-scroll refill; further patches queue to the next frame; every patch frame is pre-composited over its exact background |
| **sprite** | the TALON, pods, escorts, air enemies, bosses' sprite parts, SPARKs, explosions, BEAM lance | hardware channels on every line the object covers | 8 channels per line; ≥ 48 virtual objects per frame through the multiplexer |
| **bucket** | player shots, enemy bullets, GNAT drones, debris, star twinkles | one channel per 24×21 window, re-rendered by the CPU each frame | ≤ 16 bucket sprites per frame; player-shot and enemy-bullet buckets never mix |

- **Per-line rule:** 8 channels. The **player band** (the 42 lines the TALON covers) is the worst case:
  TALON 2 + inner pods 2 + bullet buckets 2 (one player-shot, one enemy-bullet) = 6. WINGS 3's outer
  pods are composited into a bucket channel whenever they share lines with the band (they keep their
  hurtboxes). Anything else on the player band, a boss included, may use **at most 2 channels**.
- **Boss ceiling:** a sprite boss's rows wider than 2 columns stay above playfield line 88 and the TALON
  cannot fly above line 88 while such a boss is on screen (section 3; `05-bosses/README.md`).
- **Mux priority** (highest first): TALON > inner pods > boss > SPARK / CORE SPARK > player-shot
  buckets > enemy-bullet buckets > escorts > air enemies > explosions. Outside boss fights escorts rank
  directly after the TALON; during a boss fight they are the first friendly objects to flicker. Under
  overload, explosions drop first, then enemies flicker at 30 Hz; **enemy bullets within 48 px of the
  TALON never flicker**.
- **Yellow (7) on the sprite layer** is reserved for sparks, bullets (player shots) and explosions; no
  enemy or boss sprite uses it as a private ink.
- **Static shared sprite pair** `$D025/$D026` for the whole game, proposed **dark grey (11) + light
  blue (14)** (final choice by `shared_pair.py` over every
  frame of every sprite, excluding the TALON's private inks 1 and 3, which are reserved on the sprite
  layer only; bitmap stages may use them); never changed mid-line. Text screens may
  load their own pair for portraits.
- **Collision** is reject-first: objects are bucketed by 16-line bands; only same-band pairs are tested,
  bullets against the TALON first.

## 14 Enemy bullets

| Bullet | VFX | Hitbox | Base speed | Notes |
|---|---|---|---|---|
| round | `vfx.bullet_round` | 2 × 2 | 1.5 px/tick | the common aimed shot |
| needle | `vfx.bullet_needle` | 2 × 4 | 2.25 | fast, drawn in 8 angles; used by strafers and lines |
| big orb | `vfx.bullet_orb` | 6 × 6 | 1.0 | slow; can be shot down (2 HP, 50 points) |
| crystal shard | `vfx.bullet_shard` | 4 × 4 | 1.25 | tumbling; ECHO SHARDS and Lattice rings |
| mortar shell | `vfx.bullet_lob` | 4 × 4 on landing | arcs over 60-90 ticks | harmless in flight; hits only on its landing tick, marked by a ground shadow |

- Live cap 64 enemy bullets. A spawn beyond the cap is dropped (never the oldest).
- Bullets are destroyed by BARRIER, pods and escorts; player shots pass through them except the big orb.
- Enemy bullets never spawn within 32 px of the TALON (no point-blank shots) and never in the 30 ticks
  after a respawn launch ends.

## 15 Audio

- **Music:** streamed digital music in DLT5 mono16k3 from the REU on the Ultimate's sampler, as impact;
  the cue table (title, briefing, 8 stage themes, midboss, boss, the two CROWN phases, OVERDRIVE sting,
  stage clear, game over, name entry, ending) and the Suno prompts are in `docs/music-storyboard.txt`.
  Stage themes are ≤ 2:30 loops; the boss cue is shared S1-S7. Total music ≤ 13.5 MiB.
- **Ducking:** the OVERDRIVE sting plays over the stage music with the music at half volume for its 10
  s; boss warnings cut the stage music.
- **Sound effects:** the four-SID H.A.V.O.C. engine (`docs/sfx-design.txt`), one whole chip per effect,
  panned to the object's side of the screen. Music never touches the SIDs.

## 16 Data versus code

| Data (REU tables built from the bible, level files and this spec) | Code (shared `no_std` sim; native stage overlays) |
|---|---|
| `TickDef` (NTSC/PAL tables), `SpeedDef`, hitbox steps, invulnerability windows | player state machine, gauge, arming rules, death/respawn |
| `WeaponDef` × 5 + pod and OVERDRIVE variants: damage, interval, speed, life, pattern, caps | weapon kinds (straight, side, fan, lance, homing) and the shot pools |
| `EnemyDef` per enemy: layer, class, hp, hurtbox, archetype + params, attack, score, drops, weak/resist, echo, rank_upgrade | behaviour archetypes (swooper, diver, strafer, rail_runner, orbiter, carrier, splitter, drifter, gunship, turret, tank, hatch, mortar, emplacement, tether_node, hazard, segment) |
| `StageDef`: REU strip address, scroll speeds, checkpoints, sections, spawn tracks (8-byte records), patch tables, palettes, cues, gold spark and tag sites | the spawner, scroll and REU view streaming, patch DMA |
| `BulletDef` per bullet kind; rank tables | bullet pools, bucket renderer, rank computation |
| boss tuning tables (phases, patterns, telegraphs, budgets) | boss state machines, one per boss, in that stage's overlay |
| UI strings, text screens, glyph charset, high-score defaults | menus, HUD text rows, text-screen player, name entry |
| SFX patch catalog | the H.A.V.O.C. allocator and SID driver |

## Approval

- Date:
- Approver:
- Decision:
- catalog.json hash:
