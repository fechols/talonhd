+++
id = "enemy.s2.pike"
kind = "enemy"
name = "PIKE"
stage = "stage.s2"
faction = "lattice"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
expanded = false
angles = 8
anchor = "center"
identity = "a slim blue crystal rail drone seen from directly above: narrow pointed hull, light blue fin ridge, two skids clamped to a guide wire"
silhouette = "narrow needle, short ridge, two skids, pointed both ends"
proportions = "four times as long as wide; skids at one quarter and three quarters of the length"
materials = "deep blue faceted crystal, light blue ridge, dark grey skids"
acting = "streaks along the vine rows in a tight line, flicks a single shot down as it passes overhead"
forbidden = ["eyes or a face", "fish shapes or fins like a fish", "organic shapes", "wheels", "wings"]
hurtbox = { x = -9, y = -5, w = 18, h = 10 }
domain = "air"
hp = 1
contact_damage = 1
score = 100
weak_to = "flank"
resists = "none"
drops = { spark = 0, string_clear = 1 }

[behavior]
archetype = "rail_runner"
params = { speed_q8 = 1024, rail = "vine_row", ground_locked = true, string = 4, spacing_px = 22, entry_warning_ticks = 30 }
attack = { kind = "fixed", count = 1, spread_deg = 0, speed_q8 = 384, telegraph_ticks = 0, cooldown_ticks = 0 }
placement = "S2 vineyard section; strings run along authored vine-row rails, one string per row, at most 2 rows active"
sprite_budget = { channels_on_line = 4, bullets_max = 4 }
death_vfx = "vfx.crystal_shatter"

[[colors]]
role = "crystal"
c64 = 6
note = "deep blue hull (private ink)"
required = true

[[colors]]
role = "ridge"
c64 = 14
note = "light blue ridge (shared pair, proposed)"
required = false

[[colors]]
role = "skids"
c64 = 11
note = "dark grey skids (shared pair, proposed)"
required = false

[[animations]]
id = "run"
frames = 2
ticks_per_frame = 3
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "fast forward glide with light flickering along the ridge from nose to tail"

[[animations]]
id = "heading"
frames = 8
ticks_per_frame = 1
loop = false
model = "offline"
motion = "8 headings rotated from the master; only east and west are used on straight rows"

[ludo]
sprite_prompt = "a slim blue crystal rail drone seen from directly above: narrow pointed hull, light blue fin ridge, two skids clamped to a guide wire, pointing right, top-down view"
+++

# PIKE

## Concept

The Lattice has strung the vineyard's trellis wires with its own current, and PIKEs ride them like beads
on a thread. They teach **lanes**: the vine rows on the ground are rails, and a row that flickers is a
row about to fill with drones. You cannot outrun a PIKE string, but you can avoid sitting on its row,
and the FLANK guns, firing sideways, rake a whole string as it shoots past beside you.

## Behaviour pattern

- A row's wire flickers (30-tick warning baked into the row as a 2-frame patch) before the string
  enters from one edge.
- A string of 4 PIKEs runs the row at 4 px/tick, 22 px apart, locked to the ground so the row scrolls
  down while they cross it.
- Each PIKE drops one straight-down shot at 1.5 px/tick the moment it crosses the player's X. Four
  PIKEs = a short curtain that moves sideways: dodge by moving with the string or across it early.
- Killing all 4 in a string drops 1 spark from the last.

## Placement rules

- Only in S2's vineyard rows (rails are authored rows in the level map); at most 2 rows active.
- A string on one row costs up to 4 S channels on its 21 lines. A row may only start a string while it
  is above line 112, so it has crossed and left before the row reaches the player band.
- Not combined with KESTREL pairs on the same lines.

## Readability

A blue needle on a line: what it is and where it will go are the same picture. The wire flicker is the
warning; the shot drops exactly where the needle crosses you.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
