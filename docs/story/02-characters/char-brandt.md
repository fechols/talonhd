+++
id = "char.brandt"
kind = "character"
role = "commander"
name = "BRANDT"
full_name = "MARSHAL IDA BRANDT"
pronouns = "she/her"
faction = "accord"
status = "approved"
revision = 1
appears = ["launch", "stage-card S1-S8", "stage-clear S1 S3 S5 S7", "twist-04", "ending-03"]
portrait = "optional"
portrait_layer = "sprite"
sprite_class = "B"
cells = [4, 4]
expanded = false
anchor = "center"
identity = "an older air commander shown head and shoulders, iron-grey hair under a peaked cap, a radio headset with boom microphone, one eyebrow raised"
silhouette = "flat peaked cap, headset band over the cap, boom microphone in front of the mouth, broad shoulders with epaulettes"
proportions = "cap and head fill the upper two cell rows, shoulders fill the lower two; facing three-quarter toward screen left"
materials = "wool uniform jacket, plain cloth cap, black plastic headset, a small white chevron-and-bar collar pin"
acting = "dry, brief, certain; speaks only when it matters; the eyebrow does the joking"
forbidden = ["real-world rank insignia or medals", "national flag", "cap badge with a real emblem", "weapon", "cigar", "smiling broadly", "text"]

[[colors]]
role = "primary"
c64 = 12
note = "mid grey uniform and cap (Accord grey)"
required = true

[[colors]]
role = "hair"
c64 = 15
note = "light grey hair"
required = true

[[colors]]
role = "accent"
c64 = 1
note = "white chevron-and-bar collar pin and headset highlights"
required = false

[[colors]]
role = "skin"
c64 = 10
note = "skin tones resolved by the portrait quantizer from light red and orange"
required = false

[ludo]
concept_prompt = "Head-and-shoulders portrait of an older retro-future air force commander: short iron-grey hair under a plain mid grey peaked cap with no badge, a black radio headset with a boom microphone, a mid grey wool uniform jacket with plain epaulettes and a small white chevron-and-bar collar pin. Stern mouth, one eyebrow raised. Three-quarter view facing left. 1980s arcade cabinet flyer illustration, bold readable shapes, flat colour with one shade tone, plain flat mid-grey background. No medals, no flags, no text."
sprite_prompt = "the commander portrait centred, one eyebrow raised, three-quarter view facing left"
+++

# BRANDT

## Role in the story

Marshal Ida Brandt commands what is left of the Meridian Accord's air arm from the carrier MERIDIAN.
She is the **briefing voice**: every stage card ends with her one- or two-line brief, and half the
stage-clear cards carry her reply. She approved the TALON's first sortie because nothing else could fly.

## Personality

Dry, brief, certain. Brandt never explains twice and never wastes a word on the radio; her jokes are
deadpan ("NO MORE TRAINS. GOOD. I HATE TRAINS."). She is fond of QUILL and would never say so. She does
not know about loops: her lines are the same on every loop, which is part of the loop card's chill.

## Portrait (optional)

Same rules as QUILL's: a 4×4-cell sprite grid on the right of the card (text indented on the left,
ending at column 25), its own sprite pair, two frames (neutral, eyebrow raised). Brandt faces left and
QUILL faces right, so a card with both speakers shows them facing each other. Without a portrait the
text centres.

## Text lines

- Stage briefs: see `01-story.md` 3.
- "THEN FILL THEM ON THE WAY IN."
- "GROUND CREW IS ALREADY ON THE APRON."
- "NO MORE TRAINS. GOOD. I HATE TRAINS."
- "THE CITADEL IS DOWN. STAND BY..."
- "THEN WE GO UP AFTER IT."
- "THE FLEET IS DOWN. THE CABLE IS YOURS."
- "WELCOME HOME, CAPTAIN."

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
