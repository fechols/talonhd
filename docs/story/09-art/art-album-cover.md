+++
id = "art.album_cover"
kind = "art"
name = "TALON ORIGINAL SOUNDTRACK"
status = "approved"
revision = 1
image_type = "art"
art_style = "Illustration"
perspective = "Any perspective"
aspect_ratio = "ar_1_1"
subject = "album cover for the MP3 soundtrack: the XF-9 TALON climbing over a dawn coastal airbase overgrown by the Lattice, the Crown glinting in orbit"
forbidden = ["text or lettering", "logos", "national markings or flags", "real-world aircraft silhouette", "twin tail fins", "people", "grim or dark mood"]

[ludo]
art_prompt = "Square album cover illustration in the style of a mid-1980s arcade cabinet flyer. A chunky toy-like white retro-future jet fighter, clearly not a real-world aircraft, with a fat rounded fuselage, a rounded bullet nose, stubby cranked-arrow wings with blunt square tips, one single short central tail fin and a big pale-blue glass bubble canopy, climbs heroically up and to the right through a bright dawn sky, two light blue engine trails behind it. Far below, a coastal airbase and its sea wall in morning light, deep blue sea with white surf, grey concrete runways with white markings, and glassy blue-violet and cyan crystal growth spreading across the runways and the docks. High in the sky a faceted crystal ring station glints faintly. Bold, sunny, heroic mood; bold readable shapes, strong cel shading, warm orange sunrise against deep blue, clean composition with open sky in the upper third. No text, no lettering, no logos, no national markings, no people."
+++

# TALON Original Soundtrack: album cover

## Purpose

Cover art embedded in the MP3 edition of the soundtrack (`assets/music/mp3/`), requested by the user on
2026-10-31. It is not a game asset and is never converted for the C64. The game itself keeps the `.m4a`
recordings.

## Composition

A cabinet-flyer illustration rather than the in-game top-down view: the TALON climbing into a dawn sky
over BREAKWATER (stage 1) with the Lattice crystal spreading below and the CROWN, the final goal,
glinting in orbit. It promises the whole journey of the game on one square. The upper third is open
sky. No lettering is drawn: the track and album titles live in the MP3 tags.

## Pipeline

`image create` (art, Illustration, 1:1, 4 takes), the user's or lead's pick of one take, then `image
upscale` to Ludo's maximum (2048 px), embedded as a JPEG front cover (`tools/music_mp3.py`).

## Approval

- Date: 2026-10-31
- Approver: fechols (the user), by direct request ("use Ludo to generate high-res album art and embed it
  into all of the MP3 tracks")
- Decision: approved
