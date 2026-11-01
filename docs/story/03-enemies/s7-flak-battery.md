+++
id = "enemy.s7.flak_battery"
kind = "enemy"
name = "FLAK BATTERY"
stage = "stage.s7"
faction = "captured"
status = "approved"
revision = 1
layer = "bitmap"
cell_rect = [5, 4]
angles = 1
anchor = "center"
identity = "a four-tube flak launcher on an airship hull seen from directly above: square grey mount, four stubby tubes in a square, glowing purple crystal fuses"
silhouette = "square mount, four round tube mouths in a square, ammunition rails"
proportions = "mount wider than deep; each tube mouth a fifth of the mount width"
materials = "mid grey steel mount, dark grey tubes and rails, purple crystal fuses, yellow hazard corners"
acting = "the four tubes glow in turn, then thump a shell up that bursts into a cloud of shards ahead of the TALON"
forbidden = ["people or crew", "national markings", "eyes or a face", "flags", "missiles with fins"]
hurtbox = { x = -16, y = -12, w = 32, h = 24 }
domain = "hull"
hp = 10
contact_damage = 0
score = 800
weak_to = "burst"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "mortar"
params = { aim = "lead_player", lead_ticks = 30, fuse_ticks = 50, burst_count = 8, burst_speed_q8 = 256, shells_per_volley = 2, fire_range_lines = 180 }
attack = { kind = "burst", count = 8, spread_deg = 45, speed_q8 = 512, telegraph_ticks = 24, cooldown_ticks = 150 }
placement = "S7 fleet section, on hull decks near the bow and stern; at most 2 per screen"
sprite_budget = { channels_on_line = 0, bullets_max = 18 }
death_vfx = "vfx.ground_blast"

[[colors]]
role = "primary"
c64 = 12
note = "mid grey mount"
required = true

[[colors]]
role = "secondary"
c64 = 11
note = "dark grey tubes and rails"
required = true

[[colors]]
role = "crystal"
c64 = 4
note = "purple crystal fuses"
required = true

[[colors]]
role = "hazard"
c64 = 7
note = "yellow hazard corners where the cell allows"
required = false

[[animations]]
id = "load"
frames = 4
ticks_per_frame = 6
loop = false
model = "hydra"
duration = 3
request_frames = 25
motion = "the four tube mouths light up one after another in a circle"

[[animations]]
id = "fire"
frames = 2
ticks_per_frame = 4
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "two tubes thump with a puff of smoke and the mount rocks back"

[[animations]]
id = "wreck"
frames = 1
ticks_per_frame = 0
loop = false
model = "hydra"
duration = 3
request_frames = 16
motion = "the mount torn open, tubes split and splayed, crystal fuses dark"

[ludo]
sprite_prompt = "a four-tube flak launcher on an airship hull seen from directly above: square grey mount, four stubby tubes in a square, glowing purple crystal fuses, top-down view"
+++

# FLAK BATTERY

## Concept

The airship hulls' anti-air launchers, firing timed shells that burst into clouds of shards. Unlike every
other gun in the game, a FLAK BATTERY aims **where the player is going**, not where they are: each shell
flies toward a point 30 ticks ahead of the TALON's motion and bursts there. It teaches **changing
direction**: flak punishes straight lines, and a player who reverses or stops sees the clouds bloom
harmlessly ahead.

## Behaviour pattern

1. Active within 180 lines above the player. 24-tick load (tubes light in a circle).
2. Fires 2 shells, 10 ticks apart, at 2 px/tick toward the player's position extrapolated 30 ticks
   ahead (from the last 8 ticks of movement). Shells are small, unshootable bucket bullets.
3. After 50 ticks of flight each shell bursts into an 8-way ring at 1 px/tick (a "burst cloud").
4. Cooldown 150 ticks. Fires regardless of the player's position.

## Placement rules

- S7 fleet section, on hull decks near bow and stern; at most 2 per screen (at most 36 bullets is too
  many, so only one battery may be mid-volley at a time: the second waits for the first's bursts to clear,
  keeping it at 18).
- Not on screen with a TEMPEST.

## Readability

A square with four round mouths: plainly a launcher. The circling load lights count down the volley,
and the shell's burst point is always ahead of you, never on you.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
