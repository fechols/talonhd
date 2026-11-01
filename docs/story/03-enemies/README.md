# 03-enemies — roster

38 enemy entities: 7 common and 31 stage enemies. Every file is `status = "review"`, `revision = 1`.
Midbosses and bosses live in `05-bosses/`.

## Roster

| id | name | stage | layer | size | hp | score | weak_to |
|---|---|---|---|---|---|---|---|
| enemy.common.wasp | WASP | common | sprite | S 1×1 | 1 | 100 | twin |
| enemy.common.lancer | LANCER | common | sprite | M 2×2 | 4 | 400 | flank |
| enemy.common.kestrel | KESTREL | common | sprite | W 2×1 | 3 | 300 | seeker |
| enemy.common.pillbox | PILLBOX | common | bitmap | 3×3 cells, 16 angles | 4 | 200 | beam |
| enemy.common.crawler | CRAWLER | common | bitmap | 4×4 cells, 8 angles | 6 | 400 | burst |
| enemy.common.hatch | HATCH | common | bitmap | 4×4 cells | 8 | 1000 | beam |
| enemy.common.spire_node | SPIRE NODE | common | bitmap | 3×5 cells | 10 | 600 | seeker |
| enemy.s1.skiff | SKIFF | s1 | sprite | W 2×1 | 3 | 200 | twin |
| enemy.s1.gannet | GANNET | s1 | sprite | M 2×2 | 6 | 500 | twin |
| enemy.s2.pike | PIKE | s2 | sprite | S 1×1 | 1 | 100 | flank |
| enemy.s2.roof_hatch | ROOF HATCH | s2 | bitmap | 5×4 cells | 6 | 800 | burst |
| enemy.s2.bell_turret | BELL TURRET | s2 | bitmap | 6×6 cells, 16 angles | 10 | 500 | burst |
| enemy.s2.dome_mortar | DOME MORTAR | s2 | bitmap | 5×5 cells | 8 | 600 | flank |
| enemy.s3.dust_devil | DUST DEVIL | s3 | sprite | S 1×1 | 2 | 150 | burst |
| enemy.s3.vulture | VULTURE | s3 | sprite | L 3×3 | 40 | 3000 | burst |
| enemy.s3.rail_car | RAIL CAR | s3 | bitmap | 4×6 cells, 8 angles | 8 | 500 | beam |
| enemy.s3.sandcrawler | SANDCRAWLER | s3 | bitmap | 5×6 cells, 8 angles | 10 | 600 | burst |
| enemy.s3.water_tower | WATER TOWER | s3 | bitmap | 6×6 cells | 16 | 1000 | beam |
| enemy.s4.gnat | GNAT | s4 | bucket | 12 motes in ≤2×2 bucket windows | 1 each | 50 each | burst |
| enemy.s4.heron | HERON | s4 | sprite | W 2×1 | 4 | 300 | flank |
| enemy.s4.valve_turret | VALVE TURRET | s4 | bitmap | 3×3 cells | 4 | 300 | twin |
| enemy.s4.pressure_tank | PRESSURETANK | s4 | bitmap | 4×4 cells | 6 | 500 | seeker |
| enemy.s4.fan_pit | FAN PIT | s4 | bitmap | 6×6 cells (hazard) | indestructible | 0 (1000 jam) | none |
| enemy.s5.gallery_gun | GALLERY GUN | s5 | bitmap | 3×3 cells, rows of 6 | 3 | 300 | beam |
| enemy.s5.wall_mortar | WALL MORTAR | s5 | bitmap | 4×4 cells | 8 | 600 | burst |
| enemy.s6.moth | MOTH | s6 | sprite | M 2×2 → 3 × S | 3 | 300 | burst |
| enemy.s6.dragonfly | DRAGONFLY | s6 | sprite | W 2×1 | 2 | 300 | seeker |
| enemy.s6.bloom | BLOOM | s6 | bitmap | 5×5 cells | 6 | 500 | beam |
| enemy.s6.statue_turret | STATUE GUN | s6 | bitmap | 4×6 cells, 16 angles | 12 | 700 | twin |
| enemy.s6.canopy_hatch | CANOPY HATCH | s6 | bitmap | 5×5 cells | 8 | 1200 | beam |
| enemy.s7.albatross | ALBATROSS | s7 | sprite | L 3×3 | 50 | 5000 | twin |
| enemy.s7.tempest | TEMPEST | s7 | sprite | B 4×3 | 90 | 10000 | seeker |
| enemy.s7.deck_turret | DECK TURRET | s7 | bitmap | 4×4 cells, 16 angles | 6 | 400 | beam |
| enemy.s7.flak_battery | FLAK BATTERY | s7 | bitmap | 5×4 cells | 10 | 800 | burst |
| enemy.s8.shard | SHARD | s8 | sprite | M 2×2 → 2 × S | 4 | 400 | seeker |
| enemy.s8.mirror | MIRROR | s8 | sprite | M 2×2 | 8 | 800 | twin |
| enemy.s8.climber_pod | CLIMBER POD | s8 | bitmap | 4×5 cells | 12 | 700 | beam |
| enemy.s8.prism_node | PRISM NODE | s8 | bitmap | 4×4 cells | 14 | 1000 | twin |

Weakness spread: twin 8, burst 10, beam 9, seeker 6, flank 4, none 1.

## Storyboard names folded into other entities

- **HANGAR HATCH** (S1) is `enemy.common.hatch` set into hangar roofs.
- **RUNWAY CRAWLER** (S1) is `enemy.common.crawler` on the runway paths (the name is over 12 characters).
- **Crystal-plated S8 variants** of WASP, LANCER and KESTREL are prose in those files
  (`variant_of = "enemy.common.<slug>"`), not separate entities.

## Shared conventions

- **Units.** Speeds are Q8 px per tick (256 = 1 px/tick = 60 px/s at 60 ticks/s). Screen lines count from
  the top of the 200-line playfield; the "player band" is the lower 80 lines.
- **Balance.** The baseline is TWIN at 1 damage and about 10 shots/s. Popcorn has 1-2 HP, mediums 3-8,
  ground emplacements 8-16, L gunships 40-50 and the B gunship 90. `weak_to` doubles damage and `resists`
  halves it. Hatch-like enemies also scale damage while closed, as given in their params.
- **Ground fairness.** Bitmap turrets hold fire once the player is above them
  (`hold_fire_when_below_player`). Radial emplacements, mortars and lances are the deliberate exceptions.
- **Contact.** Air and sea units kill on contact (`contact_damage = 1`). Ground, hull and cable objects
  are at ground level and cannot touch the TALON (`contact_damage = 0`); only their shots and beams hurt.
- **Leaders and sparks.** `drops` uses `spark` for an unconditional drop. Conditional drops are named:
  `leader`, `string_clear`, `group_clear`, `row_clear`, `last_of_group`, `swarm_clear`.

## Decisions for the user to confirm

1. **Proposed static shared sprite pair: dark grey 11 + light blue 14.** A multicolour sprite cell has
   one private ink, so every sprite enemy here puts its identity in that ink. Lattice drones use purple 4
   or blue 6, and captured aircraft use 15, 10, 8 or 13. Crystal highlights (14) and panel lines (11)
   come from the shared pair. L and B grids use a different private ink for their inner and outer
   columns. `shared_pair.py` still makes the final choice; this is the design intent.
2. **No sprite enemy uses white 1 or cyan 3** (the TALON's private inks). No enemy anywhere has 1 or 3 as
   a required ink. White glints are listed only as non-required. 00-bible says no stage may use 1 or
   3 at all, but the Lattice faction colours include both, so that rule needs a ruling.
3. **Yellow 7 is reserved for sparks, explosions and bullets** on sprites. The only sprite use is the
   WASP leader's core, which shows the spark it carries. Bitmap hazard stripes may use yellow.
4. **Bitmap objects that move** (CRAWLER, RAIL CAR, SANDCRAWLER, CLIMBER POD) are baked at 2-px path
   samples into a rect with one cell of margin, at about 60 KiB of REU per instance. The M1 patch-DMA
   spike (d) should confirm the cost.
5. **SPIRE NODE tethers absorb player shots** rather than damaging the TALON. This keeps the rule that
   ground objects never touch the aircraft.
6. **PRISM NODE lances are bitmap line overlays**, not sprites. Spike M1(d) should also check the cost
   of restoring them.
7. **Display names cut to 12 characters:** `PRESSURETANK` (id `pressure_tank`) and `STATUE GUN`
   (id `statue_turret`).
8. **FAN PIT has `hp = 0` (indestructible).** `tools/bible.py` must exempt the `hazard` archetype from
   the HP check.
9. **GNAT uses `sprite_class`/`cells`** to describe one bucket window, not one mote.
10. **VFX IDs referenced here, to be defined in `08-vfx.md`:** `vfx.explosion_s`, `vfx.explosion_m`,
    `vfx.explosion_l`, `vfx.ground_blast` (a bitmap crater/wreck patch, with an optional small sprite flash on its first frame), `vfx.crystal_shatter`
    and `vfx.spark_pop` (a bucket-drawn pop).

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
