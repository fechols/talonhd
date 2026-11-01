+++
id = "bosses.roster"
kind = "story"
name = "BOSS ROSTER"
status = "approved"
revision = 2
+++

# Bosses, midbosses and the weapon-reward cycle

Every stage has one **midboss** at about 45 % of its length (always in section 2, near screen 12) and
one **boss** at the end. Midbosses play `cue.midboss`; bosses S1-S7 play `cue.boss`; the CROWN plays
`cue.final_1` (the Ring) then `cue.final_2` (the Heart).

## Roster

| Stage | Midboss | Layer | Size | HP | Weak to | Music |
|---|---|---|---|---|---|---|
| s1 BREAKWATER | KEELBREAKER (Lattice-grown destroyer under way) | bitmap | 12x32 cells, 3 turret patches + bridge | 72 | BEAM | cue.midboss |
| s2 TERRACES | CAMPANILE (crystallised spinning bell tower) | bitmap | 10x10 cells, 8 animation patches | 80 | BURST | cue.midboss |
| s3 REDLINE | SIDEWINDER (armoured rail engine + 4 cars) | sprite | engine + 4 cars, B 2x4 each | 120 | SEEKER | cue.midboss |
| s4 SLUICEWORKS | LOCKMASTER (two closing gate halves) | bitmap | 2 x 14x10 cells, 3 turrets each | 90 | FLANK | cue.midboss |
| s5 BASTION | PORTCULLIS (gatehouse with cycling windows) | bitmap | 24x12 cells, 4 window shooters | 100 | BURST | cue.midboss |
| s6 VERDIGRIS | MONOLITH (temple stone opening on a crystal lens) | bitmap | 10x14 cells, 16-angle lens patch | 110 | BEAM | cue.midboss |
| s7 STORMFRONT | SQUALL (three linked gunships in a triangle) | sprite | 3 x L 3x3 + linking beam | 135 | SEEKER | cue.midboss |
| s8 SKYHOOK | TETHER WARDEN (crystal climbing machine on the cable) | sprite | B 4x5 | 150 | FLANK | cue.midboss |

| Stage | Boss | Layer | Size | HP | Weak to | Rewards | Music |
|---|---|---|---|---|---|---|---|
| s1 BREAKWATER | GANTRY QUEEN | hybrid | core B 4x6 + 2 bitmap arm-cannon patches 6x6 cells + 4 bitmap leg patches | 180 | BURST | BEAM | cue.boss |
| s2 TERRACES | ASTROLABE | bitmap | 30x16 cells, 3 ring patches + lens | 180 | BEAM | SEEKER | cue.boss |
| s3 REDLINE | TRESTLE TITAN | sprite | B 4x8 (claw rows taper to 2 cols) | 220 | SEEKER | BURST | cue.boss |
| s4 SLUICEWORKS | HYDRA PUMP | hybrid | bitmap core 12x10 cells + 5 heads 2x3 | 240 | BURST | BEAM | cue.boss |
| s5 BASTION | QUADRANT | hybrid | 4 bitmap domes 10x10 cells + core B 4x4 (root rows taper to 2 cols) | 310 | BEAM | SEEKER | cue.boss |
| s6 VERDIGRIS | ORCHID | hybrid | core B 4x4 + 6 bitmap petal patches 6x5 cells (one drifting seed M 2x2) | 280 | SEEKER | BURST | cue.boss |
| s7 STORMFRONT | ANVIL | hybrid | hull bitmap 38x50 cells (2 screens) + bridge core B 4x8 | 320 | BURST | SEEKER | cue.boss |
| s8 SKYHOOK | CROWN | hybrid | Ring bitmap 38x14 cells (8 facets) + Heart B 4x8 + 2 shard columns B 1x4 | 580 | SEEKER | BURST (next loop) | cue.final_1, cue.final_2 |

Size notation: sprite sizes are `[cols, rows]` of 24x21 display-px sprite cells; bitmap sizes are
`cell_rect` in 8x8 display cells. HP is in TWIN-shot units (one base TWIN bullet = 1). Weak parts take
double damage from the listed weapon; other parts take normal damage.

Scores: midbosses 10,000 rising 2,000 per stage (S8 25,000); bosses 30,000 rising 10,000 per stage, the
CROWN 200,000 (100,000 per phase). Every boss also pays a time bonus that falls linearly from `max` at
`par_s` seconds to zero at twice `par_s`.

## The weapon-reward cycle

Each boss is weak to one exclusive gauge weapon, and each boss **rewards the weapon the next boss is
weak to**. The reward is a **CORE SPARK**: a large gold spark released when the boss falls. Collecting
it moves the gauge cursor straight to the reward slot (it does not arm it; the player still chooses
when to ARM) and scores 10,000. If the reward weapon is already armed, the CORE SPARK is worth 50,000
instead. The gauge cursor survives the stage-clear screen, so a player who keeps it can ARM the right
weapon on the next stage card.

| Boss | Weak to | Drops CORE SPARK for | Feeds |
|---|---|---|---|
| GANTRY QUEEN | BURST | BEAM | ASTROLABE |
| ASTROLABE | BEAM | SEEKER | TRESTLE TITAN |
| TRESTLE TITAN | SEEKER | BURST | HYDRA PUMP |
| HYDRA PUMP | BURST | BEAM | QUADRANT |
| QUADRANT | BEAM | SEEKER | ORCHID |
| ORCHID | SEEKER | BURST | ANVIL |
| ANVIL | BURST | SEEKER | CROWN |
| CROWN | SEEKER | BURST | GANTRY QUEEN, next loop |

Spread: BURST is rewarded three times, SEEKER three times, BEAM twice. BEAM is never the answer in S8,
where MIRROR units reflect it. Midbosses drop 3 ordinary sparks and no CORE SPARK; their weaknesses
(BEAM, BURST, SEEKER, FLANK) teach the weapons before the bosses test them.

## Sprite budget rules for bosses

The hard limit is 8 hardware sprites on any raster line. The playfield is about 176 lines tall
(between the score row and the gauge row).

1. **Player band** (the 42 lines the TALON covers, wherever it is): ship 2 + inner pods 2 + bullet
   buckets 2 (WINGS 3 outer pods are composited into the bucket when they share lines) = 6. Anything
   else on those lines may use **at most 2 channels**.
2. **Boss ceiling.** While a sprite boss with any row wider than 2 columns is on screen, those wide rows
   stay above playfield line 88, and the TALON's top edge is clamped at line 88. Above the ceiling a
   boss may use 4 columns + 2 more channels (side parts such as the CROWN's shard columns, one drifting
   ORCHID seed, or escorts) + 2 bucket channels = 8.
3. **Tapering.** Rows of a sprite boss that reach below line 88 are at most 2 columns wide (claws,
   points, a single head). Every 4x8 boss in the game is drawn to that silhouette.
4. **Bitmap bosses cost no sprites.** The TALON flies over ground fortresses, so they may fill the screen
   and need no ceiling. Their bullets and any sprites they launch still count.
5. **All boss bullets are bucket-layer.** No boss pattern may need more than 2 bucket channels on a line;
   `bullets_max` in each pattern is the on-screen total.
6. **Mux priority:** player > boss > player shots > enemy bullets > enemies. Escorts launched during a
   boss fight are the first to flicker.

Scroll: the stage scroll stops when a boss arena reaches its rest position. Exceptions: KEELBREAKER
(the scroll slows to 0.125 px/tick while the destroyer steams ahead) and ANVIL (the scroll slows to
0.125 px/tick across the hull and stops when the bridge reaches mid-screen).

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
- rev 2 (2026-10-31): sprite expansion removed at the user's request; bigger grids instead
