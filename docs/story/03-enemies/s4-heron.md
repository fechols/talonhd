+++
id = "enemy.s4.heron"
kind = "enemy"
name = "HERON"
stage = "stage.s4"
faction = "captured"
status = "approved"
revision = 1
layer = "sprite"
sprite_class = "W"
cells = [2, 1]
expanded = false
angles = 8
anchor = "center"
identity = "a light green Accord canal hover-skimmer seen from directly above: long rounded skirt, stern fan housing, light blue crystal on the cabin roof"
silhouette = "long rounded oblong, round fan at the stern, small cabin, spray fans at the sides"
proportions = "three times as long as wide; fan housing a third of the width; cabin a quarter of the length"
materials = "light green painted hull, dark grey skirt and fan, light blue crystal, white spray"
acting = "skims the canals flat out, slews round at locks, rakes a burst along the water"
forbidden = ["people or crew", "national markings", "eyes or a face", "propellers under water", "wings"]
hurtbox = { x = -20, y = -6, w = 40, h = 12 }
domain = "sea"
hp = 4
contact_damage = 1
score = 300
weak_to = "flank"
resists = "none"
drops = { spark = 0 }

[behavior]
archetype = "rail_runner"
params = { speed_q8 = 640, rail = "canal", ground_locked = true, slew_ticks = 12, pair = 2 }
attack = { kind = "burst", count = 3, spread_deg = 0, speed_q8 = 448, telegraph_ticks = 12, cooldown_ticks = 90 }
placement = "S4 canal locks; rides authored canal channels, horizontal or vertical; at most 2 on screen"
sprite_budget = { channels_on_line = 4, bullets_max = 6 }
death_vfx = "vfx.explosion_s"

[[colors]]
role = "primary"
c64 = 13
note = "light green hull (private ink)"
required = true

[[colors]]
role = "crystal"
c64 = 14
note = "light blue crystal (shared pair, proposed)"
required = false

[[colors]]
role = "skirt"
c64 = 11
note = "dark grey skirt and fan (shared pair, proposed)"
required = false

[[animations]]
id = "skim"
frames = 4
ticks_per_frame = 4
loop = true
model = "hydra"
duration = 3
request_frames = 25
motion = "skims fast across water, stern fan spinning, white spray fanning from both sides"

[[animations]]
id = "heading"
frames = 8
ticks_per_frame = 2
loop = false
model = "offline"
motion = "8 headings rotated from the master for slews at canal corners"

[ludo]
sprite_prompt = "a light green Accord canal hover-skimmer seen from directly above: long rounded skirt, stern fan housing, light blue crystal on the cabin roof, heading right, top-down view"
+++

# HERON

## Concept

The canal authority's hover-skimmers, crewed now by crystal. They run the channels between the lock
blocks, so their routes are **the water you can see**. HERONs teach the player to read the map as a
route network: a channel that turns toward you is a HERON that will turn toward you. Running alongside
the player, they are FLANK's best target in the game.

## Behaviour pattern

- Pairs ride authored canal channels at 2.5 px/tick (ground-locked, so a vertical channel adds the
  scroll). At channel corners they slew through the headings in 12 ticks.
- Every 90 ticks, if the player is within 32 px of the channel's line ahead of it: 12-tick spray-flash
  telegraph, 3 shots along its heading, 6 ticks apart, at 1.75 px/tick. It never fires sideways, so
  standing beside a channel is safe.
- Channels end at locks; HERONs that reach a closed lock gate turn back once, then exit.

## Placement rules

- S4 canal locks; at most 2 HERONs on screen.
- Horizontal channels are kept above line 112 while a HERON runs them (4 channels on its 21 lines).
  Vertical channels may enter the player band: one W there costs 2 channels.

## Readability

A long green lozenge with spray fans: the only green moving thing in a grey-and-blue stage. Its shots
only ever go the way its nose points.

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
