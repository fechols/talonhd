+++
id = "enemy.common.lancer"
kind = "enemy"
name = "LANCER"
stage = "common"
faction = "lattice"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
expanded = false
angles = 1
anchor = "center"
identity = "a long crystal lance drone seen from directly above: deep blue spearhead body, four short rear fins, a light blue spine ridge"
silhouette = "long spearhead, four rear fins, central ridge, pointed tail"
proportions = "twice as long as wide; spearhead half the length; fins a quarter"
materials = "deep blue faceted crystal, light blue spine ridge, dark grey facet seams"
acting = "hangs still at the top of the screen, shivers, then drops like a thrown spear"
forbidden = ["eyes or a face", "organic shapes", "wings wider than the body", "engines or exhaust flames", "yellow"]
hurtbox = { x = -14, y = -18, w = 28, h = 36 }
domain = "air"
hp = 4
contact_damage = 1
score = 400
weak_to = "flank"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "diver"
params = { enter_speed_q8 = 256, hover_line = 40, hover_ticks = 30, dive_speed_q8 = 1024, track = "column_at_telegraph", max_alive = 2 }
attack = { kind = "ram", count = 1, spread_deg = 0, speed_q8 = 1024, telegraph_ticks = 24, cooldown_ticks = 0 }
placement = "drops in from the top edge to hover at line 40; at most one LANCER diving at a time"
sprite_budget = { channels_on_line = 2, bullets_max = 0 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 6
note = "deep blue body (private ink)"
required = true

[[colors]]
role = "spine"
c64 = 14
note = "light blue ridge (shared pair, proposed)"
required = false

[[colors]]
role = "seam"
c64 = 11
note = "dark grey seams (shared pair, proposed)"
required = false

[[animations]]
id = "hover"
frames = 2
ticks_per_frame = 8
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "hangs in place with a faint rocking, the spine ridge brightening slowly"

[[animations]]
id = "telegraph"
frames = 2
ticks_per_frame = 3
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "shivers rapidly side to side, fins spreading wide and the ridge flashing"

[[animations]]
id = "dive"
frames = 1
ticks_per_frame = 4
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "fins fold flat against the body, perfectly straight and still"

[ludo]
sprite_prompt = "a long crystal lance drone seen from directly above: deep blue spearhead body, four short rear fins, a light blue spine ridge, pointing straight down, top-down view"
+++

# LANCER

## Concept

A Lattice drone that is one weapon: itself. LANCERs appear in BASTION, where the ground fire is heavy,
and teach the player that **a telegraph is a promise**: the shiver marks the column, the dive follows,
and stepping one ship-width aside is enough. The FLANK guns, which fire sideways, punish a LANCER as it
streaks past, so FLANK pays for itself the first time a diver misses.

## Behaviour pattern

1. Enters from the top edge at 1 px/tick and stops at screen line 40.
2. Hovers 30 ticks, then telegraphs for 24 ticks (shiver plus spine flash, Lattice charge-up shimmer
   SFX). At the end of the telegraph it locks the player's current X.
3. Dives straight down at 4 px/tick (`dive_speed_q8 = 1024`) in the locked column and leaves through the
   bottom edge. It does not steer once committed.
4. If killed during the hover it shatters in place; killed in the dive, its fragments keep falling for
   8 ticks (harmless).

On loop 2+ the dive accelerates from 3 to 5 px/tick, so the gap closes faster but the column is the same.

## Placement rules

- At most two LANCERs alive, and only one in the dive at a time; the second waits until the first has
  left the screen.
- A diving M crosses the player band for about 20 ticks: during a dive no other sprite enemy may be
  scheduled below line 120, so the band stays at ship 2 + pods 4 + LANCER 2 = 8 channels. If a bullet
  bucket shares those lines, the mux priority rule applies: the LANCER flickers, never the bullets.
- **S8 variant (prose):** crystal-plated LANCER, 8 HP, purple 4 private ink, and the dive re-locks once
  at mid-screen. `variant_of = "enemy.common.lancer"`.

## Readability

The longest, thinnest shape in the roster: a spear pointing at you. The shiver is the whole lesson.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
