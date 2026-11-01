+++
id = "ui"
kind = "ui"
name = "UI TEXT"
status = "approved"
revision = 1
columns = 40
safe_columns = 38
case = "upper"
half_width_font = "4x7 glyphs, two per character cell; used only by the gauge row (strings marked font = \"half\")"

# ---------------------------------------------------------------- title
[[strings]]
id = "title.logo"
text = "TALON"
screen = "title"
note = "bitmap logo; this text is the fallback and the attract caption"

[[strings]]
id = "title.tagline"
text = "THE SKY IS OURS AGAIN."
screen = "title"

[[strings]]
id = "title.start"
text = "START"
screen = "title"

[[strings]]
id = "title.options"
text = "OPTIONS"
screen = "title"

[[strings]]
id = "title.scores"
text = "TOP PILOTS"
screen = "title"

[[strings]]
id = "title.press_fire"
text = "PRESS FIRE TO LAUNCH"
screen = "title"

[[strings]]
id = "title.platform"
text = "FOR THE 64 MHZ C64 ULTIMATE"
screen = "title"

[[strings]]
id = "title.version"
text = "TALON001 {VERSION}"
screen = "title"
note = "package magic and build version, bottom-right, dark grey"

# ---------------------------------------------------------------- attract
[[strings]]
id = "attract.demo"
text = "DEMO FLIGHT"
screen = "attract"
note = "top row during the recorded demo replay"

[[strings]]
id = "attract.press_fire"
text = "PRESS FIRE"
screen = "attract"

[[strings]]
id = "attract.howto_title"
text = "ARMING GAUGE"
screen = "attract-howto"

[[strings]]
id = "attract.howto_1"
text = "SHOOT DOWN LEADERS FOR SPARKS."
screen = "attract-howto"

[[strings]]
id = "attract.howto_2"
text = "EACH SPARK LIGHTS THE NEXT SLOT."
screen = "attract-howto"

[[strings]]
id = "attract.howto_3"
text = "FIRE 2 ARMS THE LIT SLOT."
screen = "attract-howto"

[[strings]]
id = "attract.howto_4"
text = "TWIN BURST BEAM SEEKER: PICK ONE."
screen = "attract-howto"

[[strings]]
id = "attract.howto_5"
text = "FLANK AND BARRIER STACK WITH ANY."
screen = "attract-howto"

[[strings]]
id = "attract.howto_6"
text = "PODS BLOCK BULLETS. THEY DIE IN PAIRS."
screen = "attract-howto"

[[strings]]
id = "attract.howto_7"
text = "ARM WINGS AT WINGS 3: OVERDRIVE."
screen = "attract-howto"

[[strings]]
id = "attract.howto_8"
text = "ARMING MAKES YOU BIGGER. FLY SMART."
screen = "attract-howto"

[[strings]]
id = "attract.cast_title"
text = "KNOW YOUR ENEMY"
screen = "attract-cast"
note = "enemy roll call: one enemy sprite per line with its name from the enemy file and its score"

[[strings]]
id = "attract.cast_line"
text = "{NAME}          {SCORE} PTS"
screen = "attract-cast"

# ---------------------------------------------------------------- HUD, top row
[[strings]]
id = "hud.top"
text = "1UP 0000000     HI 0100000      @X3"
screen = "hud"
note = "@ is the custom TALON glyph; X is the custom times glyph; lives show 1-9"

[[strings]]
id = "hud.top_boss"
text = "1UP 0000000 BOSS ################ @X3"
screen = "hud"
note = "boss and midboss fights: 16 segments, each 1/16 of the current phase's HP; # is a custom bar glyph in light red, empty segments dark grey"

[[strings]]
id = "hud.top_2up_free"
text = "1UP 0000000     HI 0100000      @X3"
screen = "hud"
note = "single player only; no 2UP field"

[[strings]]
id = "hud.extend"
text = "1UP 0000000      EXTEND!        @X4"
screen = "hud"
note = "replaces the HI field for 60 ticks on an extend"

[[strings]]
id = "hud.warning_boss"
text = "WARNING  LATTICE GUARDIAN APPROACHING"
screen = "hud"
note = "replaces the whole top row for 120 ticks, light red on black, blinking 8/8"

[[strings]]
id = "hud.warning_midboss"
text = "WARNING  HEAVY CONTACT AHEAD"
screen = "hud"
note = "90 ticks"

[[strings]]
id = "hud.warning_crown"
text = "WARNING  THE CROWN"
screen = "hud"
note = "S8 final boss, 180 ticks"

[[strings]]
id = "hud.boss_name"
text = "{BOSS}"
screen = "hud"
note = "centred on the top row for 60 ticks after the warning, before the bar fills"

[[strings]]
id = "hud.overdrive"
text = "OVERDRIVE"
screen = "hud"
note = "replaces the HI field for the first 60 ticks of OVERDRIVE, cycling white/cyan/yellow"

# ---------------------------------------------------------------- HUD, gauge row (half-width font)
[[strings]]
id = "hud.gauge"
text = "SPEED TWIN FLANK WINGS BURST BEAM SEEKR BARIR"
screen = "hud"
font = "half"
cells = 32
note = "eight 4-cell boxes, centred: columns 3-34; 5-letter labels take 3 cells; colours per design.md 6.3"

[[strings]]
id = "gauge.speed"
text = "SPEED"
screen = "hud"
font = "half"
note = "slot 1 at SPEED 0"

[[strings]]
id = "gauge.speed_n"
text = "SPD {N}"
screen = "hud"
font = "half"
note = "slot 1 at SPEED 1-5 (SPD 1 .. SPD 5); light grey at 5"

[[strings]]
id = "gauge.twin"
text = "TWIN"
screen = "hud"
font = "half"

[[strings]]
id = "gauge.flank"
text = "FLANK"
screen = "hud"
font = "half"

[[strings]]
id = "gauge.wings"
text = "WINGS"
screen = "hud"
font = "half"
note = "slot 4 at WINGS 0"

[[strings]]
id = "gauge.wings_n"
text = "WING{N}"
screen = "hud"
font = "half"
note = "WING1, WING2 at WINGS 1 and 2"

[[strings]]
id = "gauge.overdrive"
text = "OVDRV"
screen = "hud"
font = "half"
note = "slot 4 at WINGS 3 (arming it starts OVERDRIVE); blinks during OVERDRIVE"

[[strings]]
id = "gauge.burst"
text = "BURST"
screen = "hud"
font = "half"

[[strings]]
id = "gauge.beam"
text = "BEAM"
screen = "hud"
font = "half"

[[strings]]
id = "gauge.seeker"
text = "SEEKR"
screen = "hud"
font = "half"

[[strings]]
id = "gauge.barrier"
text = "BARIR"
screen = "hud"
font = "half"
note = "slot 8 with no charges"

[[strings]]
id = "gauge.barrier_n"
text = "BAR {N}"
screen = "hud"
font = "half"
note = "BAR 3, BAR 2, BAR 1"

[[strings]]
id = "gauge.overdrive_timer"
text = "{SS}"
screen = "hud"
font = "half"
note = "right spare cells during OVERDRIVE: 10 down to 1"

# ---------------------------------------------------------------- in-play popups (sprite digits)
[[strings]]
id = "popup.chain"
text = "{VALUE}"
screen = "play"
note = "FORMATION CHAIN value, 3x5 digit sprites in a bucket, rises 16 px over 40 ticks"

[[strings]]
id = "popup.oneup"
text = "1UP"
screen = "play"

[[strings]]
id = "popup.refit"
text = "REFIT"
screen = "play"

# ---------------------------------------------------------------- stage card, stage clear
[[strings]]
id = "card.stage"
text = "STAGE {N}"
screen = "stage-card"

[[strings]]
id = "card.loop"
text = "LOOP {L}"
screen = "stage-card"

[[strings]]
id = "clear.title"
text = "{NAME} IS OURS."
screen = "stage-clear"

[[strings]]
id = "clear.dark"
text = "THE CRYSTAL GOES DARK."
screen = "stage-clear"

[[strings]]
id = "clear.boss_time"
text = "BOSS TIME              {VALUE}"
screen = "stage-clear"
note = "label left, value right-aligned to column 37"

[[strings]]
id = "clear.tailored"
text = "TAILORED KILL              X2"
screen = "stage-clear"

[[strings]]
id = "clear.clean"
text = "CLEAN AIRFRAME         {VALUE}"
screen = "stage-clear"

[[strings]]
id = "clear.sweep"
text = "SECTION SWEEP          {VALUE}"
screen = "stage-clear"

[[strings]]
id = "clear.gold"
text = "GOLD SPARK             {VALUE}"
screen = "stage-clear"

[[strings]]
id = "clear.light"
text = "LIGHT TRAVELLER        {VALUE}"
screen = "stage-clear"

[[strings]]
id = "clear.total"
text = "TOTAL                  {VALUE}"
screen = "stage-clear"

# ---------------------------------------------------------------- pause
[[strings]]
id = "pause.title"
text = "PAUSED"
screen = "pause"
note = "centred on the top row; the playfield freezes, music and SFX stop"

[[strings]]
id = "pause.resume"
text = "RUN/STOP RESUMES"
screen = "pause"

[[strings]]
id = "pause.quit"
text = "HOLD FIRE 2 TO ABORT THE SORTIE"
screen = "pause"
note = "held 60 ticks, returns to the title; shown on the gauge row in the normal font"

# ---------------------------------------------------------------- game over, continue
[[strings]]
id = "gameover.title"
text = "GAME OVER"
screen = "game-over"

[[strings]]
id = "gameover.sub"
text = "THE TALON IS DOWN."
screen = "game-over"

[[strings]]
id = "continue.title"
text = "CONTINUE?"
screen = "continue"

[[strings]]
id = "continue.count"
text = "PRESS FIRE    {N}"
screen = "continue"
note = "countdown 9 to 0, one step per 60 ticks; Fire 2 speeds it up"

[[strings]]
id = "continue.warning"
text = "YOUR SCORE STARTS AGAIN FROM ZERO."
screen = "continue"

[[strings]]
id = "continue.left"
text = "CONTINUES LEFT {N}"
screen = "continue"

# ---------------------------------------------------------------- name entry
[[strings]]
id = "name.title"
text = "NEW HIGH SCORE"
screen = "name-entry"

[[strings]]
id = "name.prompt"
text = "ENTER YOUR CALLSIGN"
screen = "name-entry"

[[strings]]
id = "name.charset"
text = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.-"
screen = "name-entry"
note = "38 glyphs on a ring; the ring shows 13 at a time, so the line itself is never printed whole"

[[strings]]
id = "name.end"
text = "END"
screen = "name-entry"

[[strings]]
id = "name.delete"
text = "DEL"
screen = "name-entry"

[[strings]]
id = "name.hint"
text = "FIRE PICKS  FIRE 2 DELETES"
screen = "name-entry"

[[strings]]
id = "name.timer"
text = "TIME {SS}"
screen = "name-entry"
note = "30 seconds; on time-out the entered letters are kept, padded with dots"

[[strings]]
id = "name.rank"
text = "YOU RANK {ORD}"
screen = "name-entry"
note = "1ST 2ND 3RD 4TH .. 10TH"

# ---------------------------------------------------------------- high-score table
[[strings]]
id = "scores.title"
text = "TOP PILOTS"
screen = "scores"

[[strings]]
id = "scores.header"
text = "RANK   SCORE     NAME   STAGE"
screen = "scores"

[[strings]]
id = "scores.row"
text = " {ORD}  {SCORE}   {NAME}   {L}-{S}"
screen = "scores"
note = "stage reached as loop-stage, e.g. 1-5; ALL for a cleared loop"

[[high_scores]]
rank = 1
name = "QLL"
score = 100000
stage = "1-8"

[[high_scores]]
rank = 2
name = "IDA"
score = 90000
stage = "1-7"

[[high_scores]]
rank = 3
name = "MRD"
score = 80000
stage = "1-6"

[[high_scores]]
rank = 4
name = "XF9"
score = 70000
stage = "1-5"

[[high_scores]]
rank = 5
name = "HOK"
score = 60000
stage = "1-4"

[[high_scores]]
rank = 6
name = "ACC"
score = 50000
stage = "1-4"

[[high_scores]]
rank = 7
name = "BRK"
score = 40000
stage = "1-3"

[[high_scores]]
rank = 8
name = "SKY"
score = 30000
stage = "1-2"

[[high_scores]]
rank = 9
name = "KTL"
score = 20000
stage = "1-2"

[[high_scores]]
rank = 10
name = "DWN"
score = 10000
stage = "1-1"

# ---------------------------------------------------------------- options
[[strings]]
id = "options.title"
text = "OPTIONS"
screen = "options"

[[strings]]
id = "options.difficulty"
text = "DIFFICULTY      {EASY|NORMAL|HARD}"
screen = "options"

[[strings]]
id = "options.ships"
text = "SHIPS           {3|5}"
screen = "options"

[[strings]]
id = "options.fire2"
text = "FIRE 2 (POTX)   {ON|OFF}"
screen = "options"
note = "OFF ignores POTX for sticks that report a false second button; SPACE always arms"

[[strings]]
id = "options.music"
text = "MUSIC           {ON|OFF}"
screen = "options"

[[strings]]
id = "options.sound_test"
text = "SOUND TEST      {NN}"
screen = "options"
note = "Left/Right picks an effect, Fire plays it"

[[strings]]
id = "options.video"
text = "VIDEO           {PAL 50 HZ|NTSC 60 HZ}"
screen = "options"
note = "read-only; measured at boot"

[[strings]]
id = "options.reset_scores"
text = "RESET TOP PILOTS"
screen = "options"
note = "asks options.confirm first"

[[strings]]
id = "options.confirm"
text = "ARE YOU SURE?  FIRE = YES"
screen = "options"

[[strings]]
id = "options.exit"
text = "EXIT"
screen = "options"

[[strings]]
id = "value.easy"
text = "EASY"
screen = "options"

[[strings]]
id = "value.normal"
text = "NORMAL"
screen = "options"

[[strings]]
id = "value.hard"
text = "HARD"
screen = "options"

# ---------------------------------------------------------------- boot faults (impact-style fatal screens)
[[strings]]
id = "boot.turbo"
text = "C64U TURBO REQUIRED"
screen = "boot"
note = "the turbo enable did not read back or the timed loop ran too slowly"

[[strings]]
id = "boot.turbo_hint"
text = "LOAD TALON.CFG, THEN RUN TALON AGAIN"
screen = "boot"

[[strings]]
id = "boot.reu"
text = "REU 16 MB REQUIRED"
screen = "boot"

[[strings]]
id = "boot.reu_hint"
text = "SET REU SIZE TO 16 MB AND LOAD"
screen = "boot"

[[strings]]
id = "boot.reu_hint2"
text = "TALON.REU AT ADDRESS ZERO"
screen = "boot"

[[strings]]
id = "boot.package"
text = "PRG AND REU PACKAGES DO NOT MATCH"
screen = "boot"

[[strings]]
id = "boot.package_hint"
text = "EXPECTED TALON001 {BUILD} GOT {BUILD}"
screen = "boot"
note = "{BUILD} is 4 hex digits so the filled line is 31 columns"

[[strings]]
id = "boot.music"
text = "INVALID RESIDENT MUSIC DATA"
screen = "boot"

[[strings]]
id = "boot.code"
text = "FAULT {CODE}. PLEASE POWER CYCLE."
screen = "boot"
note = "generic fatal code screen, as impact"

# ---------------------------------------------------------------- stage and boss names
[[stages]]
id = "stage.s1"
name = "BREAKWATER"
subtitle = "COASTAL AIRBASE AT DAWN"
midboss = "KEELBREAKER"
boss = "GANTRY QUEEN"

[[stages]]
id = "stage.s2"
name = "TERRACES"
subtitle = "HILLSIDE VINEYARDS"
midboss = "CAMPANILE"
boss = "ASTROLABE"

[[stages]]
id = "stage.s3"
name = "REDLINE"
subtitle = "DESERT CANYON RAILWAY"
midboss = "SIDEWINDER"
boss = "TRESTLE TITAN"

[[stages]]
id = "stage.s4"
name = "SLUICEWORKS"
subtitle = "INDUSTRIAL CANAL DISTRICT"
midboss = "LOCKMASTER"
boss = "HYDRA PUMP"

[[stages]]
id = "stage.s5"
name = "BASTION"
subtitle = "THE CITADEL WALLS"
midboss = "PORTCULLIS"
boss = "QUADRANT"

[[stages]]
id = "stage.s6"
name = "VERDIGRIS"
subtitle = "JUNGLE RUINS"
midboss = "MONOLITH"
boss = "ORCHID"

[[stages]]
id = "stage.s7"
name = "STORMFRONT"
subtitle = "ABOVE THE CLOUDS"
midboss = "SQUALL"
boss = "ANVIL"

[[stages]]
id = "stage.s8"
name = "SKYHOOK"
subtitle = "THE ORBITAL ELEVATOR"
midboss = "TETHER WARDEN"
boss = "CROWN"
+++

# UI text

All strings are uppercase, PETSCII-safe, and fit the 38-column safe area; `tools/bible.py check`
measures widths, counting `font = "half"` strings at half width (two glyphs per cell) and placeholders
at their maximum filled width. Story screens (intro, stage briefs, twist, ending, credits, loop card)
live in `01-story.md`.

## Screens

- **Title:** the bitmap TALON logo over a top-down MERIDIAN deck with the TALON on the catapult; tagline;
  menu START / OPTIONS / TOP PILOTS; PRESS FIRE TO LAUNCH blinks 30/30; platform line; package and build
  version bottom right in dark grey. `cue.title` plays. 20 s idle → attract.
- **Attract cycle:** title → DEMO FLIGHT (a recorded 40-s replay of S1, then of a random cleared stage)
  → ARMING GAUGE how-to (eight lines typed one per 60 ticks, with the real gauge row animating below
  them) → KNOW YOUR ENEMY roll call (six enemies per page) → TOP PILOTS → title. Fire or F1 starts a
  game from anywhere in the cycle.
- **HUD:** two hires text rows. Top: score, high score, lives; replaced by the warning, the boss name
  and the boss bar during fights, and by EXTEND! / OVERDRIVE briefly. Bottom: the arming gauge in the
  half-width font (labels change with state; colours in `docs/design.md` 6.3).
- **Stage card / stage clear:** full text screens, templates and lines in `01-story.md`; the tally
  lines above count up at 1,000 per tick with `sfx.gauge_step`.
- **Pause:** RUN/STOP. PAUSED on the top row, RUN/STOP RESUMES and HOLD FIRE 2 TO ABORT THE SORTIE on
  the gauge row; everything freezes; music and SFX stop.
- **Game over → name entry → continue:** GAME OVER / THE TALON IS DOWN. over the frozen playfield for
  180 ticks; NEW HIGH SCORE if the score qualifies; then CONTINUE? with the countdown on loop 1.
- **Name entry:** three-letter callsign picked from a horizontal glyph ring with the stick; Fire picks,
  Fire 2 deletes, END confirms. `cue.name_entry` plays. 30-s limit.
- **Top pilots:** ten rows, the default table above (invented callsigns), stage reached as loop-stage.
  Scores are kept in the REU for the session only (the C64U has no save path in v1).
- **Options:** cursor Up/Down, Left/Right changes values, Fire on EXIT returns to the title.
- **Boot faults:** the strings above on a black screen with a dark red border; the game does not start.
  Order of checks: turbo, REU size, package magic and build, music data.

## Width notes

- `hud.top`: 35 columns. `hud.top_boss`: 37. `hud.warning_boss`: 38 (the maximum).
- `hud.gauge`: 45 glyphs in the half-width font = 32 cells including the box padding.
- The longest stage-clear line is 38 (`01-story.md`).

## Approval

- Date: 2026-10-30
- Approver: fechols (the user)
- Decision: approved (whole bible approved at once)
- catalog.json hash: be5543749ede05461d50080ef94f882e62805c84319be3507f6bd7f8c7a15776 (export of the approved front matter, before this record was filled in)
