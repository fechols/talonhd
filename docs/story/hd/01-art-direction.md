+++
id = "hd.art"
kind = "hd_art"
name = "ART DIRECTION"
status = "approved"
revision = 2

# Ludo 3D: `ludo model create-image` (the render to convert) then `ludo model create` (GLB).
# The image is converted as-is, so it must show the whole object, flat paint per part, nothing else.
[ludo_model]
image_prefix = "A clean 3D render of a single hard-surface game model, low-poly with crisp flat facets and sharp edges, each part painted one flat matte color, seen from above at a three-quarter angle with the whole object in view, centred, plain white background, soft even studio light, no shadow on the ground"
image_suffix = "Only the object. No text, no markings, no pilot, no people, no base or stand."
never = ["photorealistic", "weathering", "decals", "text", "logo", "insignia", "flag", "gradient paint", "glow effects", "motion blur", "background scenery"]
create = { generator_default = "hero", texture_type = "simple", texture_size = 2048, target_faces_floor = 1000 }
note = "Ludo meshes come back at >= 1000 faces; tools/model_prep.py decimates to the bible's tris and maps each paint color to a material by its `source` description."

# Palette effects: every one is a register write or a screen-RAM / color-RAM fill, applied in vblank.
[[effects]]
id = "fx.hit_flash"
trigger = "a boss part or large enemy takes a hit"
change = "void -> the stage's dark color for 2 frames"
budget = "one $D021 write"

[[effects]]
id = "fx.boss_phase"
trigger = "a boss enters its next phase"
change = "hot -> the boss's phase-2 hot color for the rest of the fight (declared per boss); a 6-frame void/hot strobe marks the change"
budget = "one color-RAM fill"

[[effects]]
id = "fx.overheat"
trigger = "the player's shield or last life is at risk (design.md gauge rules)"
change = "hot pulses between its color and light every 16 frames"
budget = "one color-RAM fill per pulse"

[[effects]]
id = "fx.node_dark"
trigger = "stage boss defeated (pp.node_dark)"
change = "hot steps down its luminance ramp to dark over 48 frames: the crystal goes dark, as in the base story"
budget = "one color-RAM fill per step"

[[effects]]
id = "fx.promotion_walk"
trigger = "stage-clear card -> next stage card (the promotion sequence)"
change = "each role walks from this stage's color to the next stage's color through the luminance groups, one step per 8 frames, void first, hot last"
budget = "one write or fill per step"

[[effects]]
id = "fx.smart_bomb"
trigger = "a screen-clearing weapon (design.md BURST at level 3)"
change = "all four roles -> light for 1 frame, then void-dark-light-hot restored over 4 frames"
budget = "4 writes"
+++

# Art direction

*Written by Claude as TALON HD's art director. Every rule here is meant to be checkable, and the reason is
given each time.*

## The look in one paragraph

Solid faceted polygons, four colors, crisp edges, no outlines, one light. Ships are hard little machines
that **turn**: they bank, roll and yaw through dozens of pre-rendered angles, and that motion is what
sells the third dimension on a machine that has none. Everything is seen from straight above. Bosses are
architecture: bigger than the screen, never fully seen at once, discovered as they sway.

## 1 The palette is a live instrument

Because all four colors are global, recoloring the whole screen costs one register write or one fill.
TALON HD uses that on purpose, and every use is declared in `[[effects]]` above:

- **Hit flash.** `void` jumps to `dark` for two frames. The whole world flinches.
- **Boss phases.** A boss's second phase changes `hot`. Every crystal, bullet and core on screen changes
  with it, so the player feels the fight change.
- **The node goes dark.** When a boss falls, `hot` steps down to `dark`: the base story's "the crystal goes
  dark", done with no new art.
- **The promotion walk.** Between stages each role walks, step by step, from this stage's color to the next
  one's. The player watches the world turn into the next theatre while QUILL's new ship rolls out.

Effects are short and sparse. The palette is the most powerful tool we have; overused it becomes noise.

## 2 Value first, hue second

Every stage palette is a luminance ramp: `void` < `dark` < `light`, with `hot` well clear of `void`. A
palette that fails this fails `check`, however pretty it is, because Lambert shading needs ordered steps.
Hue gives the stage its mood; value gives the player the shapes.

## 3 Light belongs to the player

With four shared colors, the player could vanish under a boss. The rule that prevents it:

- the player's ship is mostly `light` (at least 35 % of its pixels);
- enemies and bosses are mostly `dark` and `hot`, with `light` only as highlights (at most 15 %);
- the background is mostly `void` and `dark` (at most 10 % `light`).

The eye always finds the brightest moving shape at the bottom of the screen.

## 4 The Lattice is `hot`

Crystal, cores, enemy bullets, muzzles: anything that belongs to the Lattice is `hot`, on every stage.
Captured Accord machines are `dark` hulls with `hot` crystal growing through them. Pure Lattice drones are
`hot` facets over `dark`. The player learns one rule once: **hot hurts**. Weapons are `hot` too, even when the Accord built them: a captured cannon's barrel glows.

## 5 One light, faceted

One key light, upper left and toward the viewer, for the whole game. Faces snap to at most three tones plus
the void/dark checker. The polygon counts are deliberately low (`00-palette-law.md` size classes): facets
the player can see are the style, not a limitation.

## 6 Motion sells the third dimension

- **The player's ship** banks through 9 roll angles (±60°). It snaps into a bank in 4 frames and levels
  in 6.
- **Air enemies** turn through 16 yaw angles and bank into their turns.
- **Debris** tumbles through 8 angles.
- **Turrets** track through 16 angles.
- **Bosses** sway and bank slowly through 9 roll angles; articulated parts (arms, turrets, cores) are
  separate sprites with their own angles.

An object that never turns should be flat art, not a model.

## 7 Scale is the spectacle

Exaggerate the size classes: 24-pixel WASPs, a 36-pixel player ship, 512-pixel bosses. A boss always
enters partly off-screen and is never fully visible. Its sway reveals one wing, then the other, so the
player understands it only in pieces. Midbosses fill about two thirds of the screen width.

## 8 Dither is a material, not a gradient

The 50 % checker means something: **armour** (dark/light checker on lit plating), **glass** (light/hot
checker on canopies) and **heat** (dark/hot checker near engines and crystal). It never stands for a
smooth gradient. It is anchored in sprite space, so it travels with the object and never shimmers.

## 9 The background stays down

Panoramas are `void` and `dark`, with `light` for small markings (surf, runway paint) and `hot` only where
the Lattice has grown into the ground. Ground structures are rendered from the same models and light as
the sprites, one luminance step down. The play layer owns the contrast.

## 10 Review

For every stage, `talonhd bake` writes:
- a **palette card**: the four colors, their roles and their luminance groups;
- a **lineup sheet**: the stage's ship, every enemy and the bosses, rendered in that stage's palette at C64
  size;
- three **scene frames** at fixed times.

Nothing is accepted from Ludo until the lineup sheet has been looked at by a human at C64 size.

## Approval

- Date: 2026-10-31
- Approver: fechols (the user)
- Decision: approved (whole HD layer approved at once, "Approve all, start Ludo")
- rev 2 (2026-10-31): "weapons are hot" added to section 4 (from the GANTRY QUEEN review)
