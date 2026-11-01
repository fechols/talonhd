+++
id = "char.quill"
kind = "character"
role = "pilot"
name = "QUILL"
full_name = "CAPTAIN NOOR VASQUEZ"
pronouns = "they/them"
faction = "accord"
status = "approved"
revision = 1
appears = ["launch", "twist-04", "stage-clear S2 S4 S6", "ending-03"]
portrait = "optional"
portrait_layer = "sprite"
sprite_class = "B"
cells = [4, 4]
expanded = false
anchor = "center"
identity = "a test pilot shown head and shoulders, white flight helmet with visor up, calm face, dry half-smile, olive collar with cyan callsign tab"
silhouette = "rounded helmet dome, raised visor band, headset boom microphone on one side, square shoulders with suit collar"
proportions = "head and helmet fill the upper two cell rows, shoulders fill the lower two; facing three-quarter toward screen right"
materials = "glossy white helmet shell, tinted visor, matte olive flight suit fabric, rubber oxygen-hose fitting at the collar"
acting = "calm, dry, stubborn; never shouts, never panics; a half-smile when things go wrong"
forbidden = ["national flag or insignia", "rank badges with real-world designs", "visible injury", "cigarette", "sunglasses", "any text on the helmet", "full-face mask over the face"]

[[colors]]
role = "primary"
c64 = 1
note = "white helmet"
required = true

[[colors]]
role = "visor"
c64 = 3
note = "cyan reflection on the raised visor and the callsign tab (echoes the TALON's canopy)"
required = true

[[colors]]
role = "suit"
c64 = 9
note = "olive flight suit, quantized to brown with dark grey shade"
required = false

[[colors]]
role = "skin"
c64 = 8
note = "skin tones resolved by the portrait quantizer from orange and light red"
required = false

[ludo]
concept_prompt = "Head-and-shoulders portrait of a calm retro-future test pilot in a glossy white flight helmet with the tinted visor pushed up, a headset boom microphone, a matte olive flight suit collar with a small cyan tab. Steady gaze, dry half-smile, three-quarter view facing right. 1980s arcade cabinet flyer illustration, bold readable shapes, flat colour with one shade tone, plain flat mid-grey background. No insignia, no text."
sprite_prompt = "the pilot portrait centred, helmet visor up, half-smile, three-quarter view facing right"
+++

# QUILL

## Role in the story

Captain Noor Vasquez, callsign **QUILL**, the TALON's test pilot for three years and its only pilot now.
QUILL is referred to as they/them everywhere: on cards, in this bible, in the credits.

QUILL never appears in the playfield (the TALON's canopy is too small to show a pilot). They speak only
on text screens: the launch card, a few stage-clear lines and the ending.

## Personality

Calm, dry and stubborn. QUILL talks in short radio sentences, understates everything, and makes one joke
per card at most. Never cruel, never boastful. They treat BRANDT with respect and a little teasing.

## Portrait (optional)

A 4×4-cell multicolour sprite grid (96×84 display px) shown on the left of a text screen, with the text
indented to column 13. Text screens carry no gameplay sprites, so the portrait may use its own
`$D025/$D026` pair (loaded with the screen) and up to four channels per line. If the portrait is not
produced, the card's text is centred and nothing else changes. Two frames only: neutral and half-smile
(swapped while the text types).

## Text lines

- "TALON IS HOT. SPARK RACKS EMPTY."
- "THE CITADEL WAS A RELAY. SOMETHING ABOVE US IS DRINKING THE LIGHT."
- "TELESCOPE'S CLOSED. NICE VIEW, THOUGH."
- "PUMPS ARE DRY. SO IS MY FLIGHT SUIT."
- "LAUNCH PAD'S CLEAR. RUSTY, BUT CLEAR."
- "TALON ON DECK. RACKS EMPTY."

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
