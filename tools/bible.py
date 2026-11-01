#!/usr/bin/env python3
"""TALON design-bible validator and exporter. Standard library only (Python 3.11+ for tomllib).

    python tools/bible.py check                  validate docs/story/**
    python tools/bible.py export                 write assets/story/catalog.json
    python tools/bible.py export-prompts [--all] compile Ludo requests to assets/prompts/
    python tools/bible.py export-music           collect music tables and briefs to output/music-briefs.md
    python tools/bible.py list                   print every entity id, kind and status

The bible is the single source of truth: prompts are compiled from front matter, never typed by hand.
docs/story/SCHEMA.md is authoritative for field names and legal values.

Provenance: copied from pummel's tools/bible.py at pummel commit 66b240c and adapted for TALON (a
top-down vertical shooter): sprite/bitmap/bucket/hybrid layers, the S/W/M/L/B size classes, bitmap
cell rectangles, shooter archetypes and attack kinds, weapon weaknesses, the TALON's private sprite
inks 1 and 3, music cues checked against docs/music-storyboard.txt, and talon-prompts-v1 requests.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import re
import sys
import textwrap
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
STORY = ROOT / "docs" / "story"
STORYBOARD = ROOT / "docs" / "music-storyboard.txt"
CATALOG = ROOT / "assets" / "story" / "catalog.json"
PROMPTS = ROOT / "assets" / "prompts"
MUSIC_OUT = ROOT / "output" / "music-briefs.md"
FRONT = re.compile(r"\A\+\+\+\r?\n(.*?)\r?\n\+\+\+\r?\n", re.S)
CUE_LINE = re.compile(r"^(cue\.[a-z0-9_]+)\s+-\s", re.M)

STATUSES = {"draft", "review", "approved", "frozen"}
KINDS = {"world", "story", "ship", "character", "enemy", "level", "boss", "midboss", "vfx", "style", "ui", "art"}
# Key art outside the game (album cover, box, flyer): one illustration job, then an upscale of the chosen take.
ART_FIELDS = {"image_type", "art_style", "perspective", "aspect_ratio", "subject", "forbidden", "ludo"}
VISUAL_KINDS = {"ship", "character", "enemy", "boss", "midboss"}
BOSS_KINDS = {"boss", "midboss"}
LAYERS = {"sprite", "bitmap", "bucket"}
BOSS_LAYERS = LAYERS | {"hybrid"}
PART_LAYERS = {"sprite", "bitmap", "line"}  # line: a software-drawn bitmap line, no cell_rect
SIZE_CLASSES = {"S": (1, 1), "W": (2, 1), "M": (2, 2), "L": (3, 3), "B": None}
# Class B ("big") is any grid of 1-4 columns (8 hardware sprites per line, two per column in pairs) by
# 1-8 rows (168 lines); its `cells` field gives the actual size.
MAX_COLS, MAX_ROWS = 4, 8
MAX_CELL_RECT = (40, 255)         # bitmap cell rectangles: 40 cells across; the stage scrolls, so rows may exceed 25
ANGLES = {1, 8, 16}
ANCHORS = {"center", "nose", "base"}
MODELS = {"hydra", "hydra-plus", "forge", "forge-pixel", "offline"}
LUDO_MODELS = MODELS - {"offline"}
REQUEST_FRAMES = {16, 25, 36, 49, 64}
ARCHETYPES = {"swooper", "diver", "strafer", "rail_runner", "orbiter", "carrier", "splitter", "drifter", "gunship",
              "turret", "tank", "hatch", "mortar", "emplacement", "tether_node", "hazard", "segment"}
ATTACK_KINDS = {"none", "aimed", "fixed", "spread", "ring", "burst", "lob", "beam", "ram", "launch", "hazard"}
WEAPONS = {"twin", "flank", "burst", "beam", "seeker"}
WEAKNESSES = WEAPONS | {"none"}
DOMAINS = {"air", "ground", "sea", "hull", "cable"}
# Everything scrolls down through the player band (the lower 80 lines) at stage speed, so every domain
# can put a sprite there; trim this set if a domain is ever guaranteed to stay above the band.
PLAYER_BAND_DOMAINS = {"air", "ground", "sea", "hull", "cable"}
PLAYER_BAND_CHANNELS = 2
FACTIONS = {"accord", "lattice", "captured"}
IMAGE_TYPES = {"texture", "tile", "sprite-tiling-vertical", "sprite", "parallax_layer", "vertical_scrolling_background"}
SETPIECE_IMAGE_TYPE = "vertical_scrolling_background"  # P0: `sprite` gave isolated, diagonal objects
SETPIECE_ASPECTS = {"ar_1_1": 1.0, "ar_4_3": 4 / 3, "ar_16_9": 16 / 9, "ar_19_9": 19 / 9, "ar_3_4": 3 / 4,
                    "ar_9_16": 9 / 16, "ar_9_19": 9 / 19}  # width / height of the Ludo aspect options
SETPIECE_DEFAULT_CELLS = [40, 25]  # a transition set-piece without a tile entry: one full screen
SEAMLESS = {"h", "v", "both", "none"}
VFX_LAYERS = LAYERS | {"hud"}  # hud: text-row colour effects, no sprite cost
PRIVATE_INKS = {1, 3}             # the TALON's sprite-layer inks
PRIVATE_INK_KINDS = {"ship"}      # the ship and its pods may use them as required inks
MAX_NAME, MAX_STAGE_NAME = 12, 16
MAX_BOSS_NAME = 16  # boss names appear only on the warning card and boss bar, not in 12-column HUD slots
MAX_IDENTITY_WORDS = 25
MAX_MOTION_WORDS = 60
MAX_TEXT_COLUMNS = 38
MAX_SCREEN_LINES = 8
SPRITES_PER_LINE = 8
BULLET_BUCKETS = 2  # bucket channels every band reserves for player shots and enemy bullets

VISUAL_FIELDS = {"style_swaps", "layer", "sprite_class", "cells", "expanded", "cell_rect", "angles", "anchor", "identity", "silhouette",
                 "proportions", "materials", "acting", "forbidden", "hurtbox", "colors", "animations", "ludo"}
COMMON_FIELDS = {"id", "kind", "name", "status", "revision"}
ALLOWED_FIELDS = {
    "ship": COMMON_FIELDS | VISUAL_FIELDS | {"faction", "speed_levels_q8", "hitbox_levels", "private_inks", "cell_inks",
                                             "hitbox_center", "blocks_bullets", "invulnerable"},
    "character": COMMON_FIELDS | VISUAL_FIELDS | {"role", "full_name", "pronouns", "faction", "appears", "portrait",
                                                  "portrait_layer", "voice", "glyph_charset", "glyph_inks", "avatar",
                                                  "glyphs"},
    "enemy": COMMON_FIELDS | VISUAL_FIELDS | {"stage", "faction", "domain", "hp", "contact_damage", "score", "weak_to",
                                              "resists", "drops", "behavior", "variant_of"},
    "level": COMMON_FIELDS | {"slug", "stage_index", "title", "theme", "length_screens", "scroll_q8", "target_minutes",
                              "midboss", "boss", "enemies", "checkpoints", "music", "background_ink", "card_line",
                              "twist_line", "no_ground", "cable", "sections", "transitions", "waves", "tiles"},
    "boss": COMMON_FIELDS | VISUAL_FIELDS | {"title", "stage", "faction", "domain", "hp", "contact_damage", "weak_to",
                                             "resists", "score", "time_bonus", "reward", "reward_note", "music_cue",
                                             "music_cue_phase2", "drops", "parts", "phases", "patterns", "occupancy",
                                             "variant_of"},
}
ALLOWED_FIELDS["midboss"] = ALLOWED_FIELDS["boss"]
ALLOWED_FIELDS["art"] = COMMON_FIELDS | ART_FIELDS
BEHAVIOR_FIELDS = {"archetype", "params", "attack", "placement", "sprite_budget", "death_vfx"}
BODY_HEADINGS = {
    "enemy": ("Concept", "Behaviour pattern", "Placement rules", "Readability"),
    "level": ("Setting", "Story beat", "Section beats", "Transition set piece", "Midboss and boss arena", "Music brief"),
    "boss": ("Personality and role", "Arena", "Phases and patterns", "Budget notes"),
    "midboss": ("Personality and role", "Arena", "Phases and patterns", "Budget notes"),
}


# ---------------------------------------------------------------------------
# Loading


class Problems:
    def __init__(self) -> None:
        self.errors: list[str] = []
        self.warnings: list[str] = []

    def error(self, path: Path, message: str) -> None:
        line = f"{rel(path)}: {message}"
        if line not in self.errors:
            self.errors.append(line)

    def warn(self, path: Path, message: str) -> None:
        line = f"{rel(path)}: {message}"
        if line not in self.warnings:
            self.warnings.append(line)


@dataclass
class Context:
    """Everything outside the entity files that the checks consult (overridable in tests)."""
    terms: list[str] = field(default_factory=list)
    cues: set[str] | None = None        # None: storyboard missing, cue checks become warnings
    vfx_missing: bool = False           # True: 08-vfx.md missing, death_vfx checks become warnings
    extra_texts: dict[Path, str] = field(default_factory=dict)   # READMEs etc. scanned for forbidden terms

    @classmethod
    def from_disk(cls) -> "Context":
        cues = set(CUE_LINE.findall(STORYBOARD.read_text(encoding="utf-8"))) if STORYBOARD.exists() else None
        extra = {}
        for path in sorted(STORY.rglob("*.md")):
            if path.name in {"README.md", "SCHEMA.md"}:
                extra[path] = path.read_text(encoding="utf-8")
        return cls(terms=forbidden_terms(), cues=cues, vfx_missing=not (STORY / "08-vfx.md").exists(), extra_texts=extra)


def rel(path: Path) -> str:
    try:
        return Path(path).resolve().relative_to(ROOT).as_posix()
    except ValueError:
        return str(path)


def parse_entity(text: str, path: Path) -> dict:
    """Parse one bible file's text into an entity dict with private _path/_body/_text keys."""
    match = FRONT.match(text)
    if not match:
        raise ValueError("missing +++ front matter")
    data = tomllib.loads(match.group(1))
    data["_path"] = Path(path)
    data["_body"] = text[match.end():]
    data["_text"] = text
    return data


def story_files() -> list[Path]:
    # TALON HD: the HD layer in docs/story/hd is checked by tools/bible_hd.py.
    return sorted(p for p in STORY.rglob("*.md")
                  if p.name not in {"README.md", "SCHEMA.md"} and "hd" not in p.relative_to(STORY).parts[:1])


def load_all(problems: Problems) -> dict[str, dict]:
    entities: dict[str, dict] = {}
    for path in story_files():
        try:
            data = parse_entity(path.read_text(encoding="utf-8"), path)
        except (ValueError, tomllib.TOMLDecodeError) as error:
            problems.error(path, f"cannot parse: {error}")
            continue
        entity_id = data.get("id")
        if not isinstance(entity_id, str) or not entity_id:
            problems.error(path, "missing id")
            continue
        if entity_id in entities:
            problems.error(path, f"duplicate id {entity_id} (also {rel(entities[entity_id]['_path'])})")
            continue
        entities[entity_id] = data
    return entities


def forbidden_terms(path: Path | None = None) -> list[str]:
    path = path or STORY / "forbidden-terms.txt"
    terms = []
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if line and not line.startswith("#"):
            terms.append(line.lower())
    return terms


def word_count(text: str) -> int:
    return len(text.split())


def is_int(value) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


# ---------------------------------------------------------------------------
# Common and visual checks


def check_common(problems: Problems, data: dict) -> None:
    path = data["_path"]
    for name in ("id", "kind", "name", "status", "revision"):
        if name not in data:
            problems.error(path, f"missing {name}")
    kind = data.get("kind")
    if kind not in KINDS:
        problems.error(path, f"unknown kind {kind!r}")
    if data.get("status") not in STATUSES:
        problems.error(path, f"unknown status {data.get('status')!r}")
    if not is_int(data.get("revision")) or data.get("revision", 0) < 1:
        problems.error(path, "revision must be an integer >= 1")
    name = data.get("name", "")
    if isinstance(name, str):
        if name != name.upper():
            problems.error(path, f"name must be uppercase: {name!r}")
        limit = MAX_BOSS_NAME if kind in {"boss", "midboss"} else MAX_NAME
        if kind in VISUAL_KINDS and len(name) > limit:
            problems.error(path, f"name longer than {limit} characters: {name!r} ({len(name)})")
        if kind == "level" and len(name) > MAX_STAGE_NAME:
            problems.error(path, f"stage name longer than {MAX_STAGE_NAME} characters: {name!r} ({len(name)})")
    allowed = ALLOWED_FIELDS.get(kind)
    if allowed:
        for key in sorted(k for k in data if not k.startswith("_") and k not in allowed):
            problems.warn(path, f"unknown field {key!r} for kind {kind}")
    if kind in VISUAL_KINDS | {"level"} and not re.search(r"^#+ Approval\s*$", data["_body"], re.M):
        problems.warn(path, "no '## Approval' section")
    for heading in BODY_HEADINGS.get(kind, ()):
        if not re.search(rf"^#+ {re.escape(heading)}\b", data["_body"], re.M):
            problems.warn(path, f"body lacks a '# {heading}' section")
    check_id_matches_file(problems, data)


def check_id_matches_file(problems: Problems, data: dict) -> None:
    path, entity_id, kind = data["_path"], data.get("id", ""), data.get("kind")
    stem = Path(path).stem
    parts = entity_id.split(".")
    if kind in {"world", "story", "style", "ui", "vfx"}:
        if entity_id != kind:
            problems.error(path, f"{kind} id must be {kind!r}, got {entity_id!r}")
        return
    if kind in {"ship", "character"}:
        prefixes = {"ship": {"ship", "pod"}, "character": {"char"}}[kind]
        if len(parts) != 2 or parts[0] not in prefixes:
            problems.error(path, f"{kind} id must be {'|'.join(sorted(prefixes))}.<slug>: {entity_id}")
        elif stem != f"{parts[0]}-{parts[1].replace('_', '-')}":
            problems.error(path, f"file should be {parts[0]}-{parts[1].replace('_', '-')}.md for id {entity_id}")
        return
    if kind in {"enemy", "boss", "midboss"}:
        if len(parts) != 3 or parts[0] != kind:
            problems.error(path, f"{kind} id must be {kind}.<scope>.<slug>: {entity_id}")
            return
        expected = f"{parts[1]}-{parts[2].replace('_', '-')}"
        if stem != expected:
            problems.error(path, f"file should be {expected}.md for id {entity_id}")
        return
    if kind == "art":
        if len(parts) != 2 or parts[0] != "art":
            problems.error(path, f"art id must be art.<slug>: {entity_id}")
        elif stem != f"art-{parts[1].replace('_', '-')}":
            problems.error(path, f"file should be art-{parts[1].replace('_', '-')}.md for id {entity_id}")
        return
    if kind == "level":
        if len(parts) != 2 or parts[0] != "stage" or not re.fullmatch(r"s[1-8]", parts[1]):
            problems.error(path, f"level id must be stage.s1 .. stage.s8: {entity_id}")
            return
        slug = data.get("slug", "")
        expected = f"{parts[1]}-{str(slug).replace('_', '-')}"
        if stem != expected:
            problems.error(path, f"level file should be {expected}.md for id {entity_id} and slug {slug!r}")
        if data.get("stage_index") != int(parts[1][1:]):
            problems.error(path, f"stage_index {data.get('stage_index')!r} does not match id {entity_id}")


def grid_of(record: dict) -> tuple[int, int] | None:
    """(cols, rows) of a sprite-layer record, or None when the class or cells are invalid."""
    sprite_class = record.get("sprite_class")
    if sprite_class not in SIZE_CLASSES:
        return None
    cells = list(record.get("cells") or [])
    if len(cells) != 2 or not all(is_int(c) for c in cells):
        return None
    cols, rows = cells
    if sprite_class == "B":
        return (cols, rows) if 1 <= cols <= MAX_COLS and 1 <= rows <= MAX_ROWS else None
    return (cols, rows) if (cols, rows) == SIZE_CLASSES[sprite_class] else None


def check_sprite_size(problems: Problems, path: Path, record: dict, label: str) -> None:
    sprite_class = record.get("sprite_class")
    if sprite_class not in SIZE_CLASSES:
        problems.error(path, f"{label}sprite_class {sprite_class!r} unknown (S|W|M|L|B)")
        return
    if grid_of(record) is None:
        fixed = SIZE_CLASSES[sprite_class]
        rule = f"must be {list(fixed)}" if fixed else f"must be [1-{MAX_COLS} cols, 1-{MAX_ROWS} rows]"
        problems.error(path, f"{label}cells {record.get('cells')} invalid for sprite_class {sprite_class} ({rule})")


def check_no_expansion(problems: Problems, path: Path, value, label: str = "") -> None:
    """VIC-II sprite expansion ($D017/$D01D) is banned: bigger objects use bigger grids."""
    if isinstance(value, dict):
        if "expanded" in value and value["expanded"] is not False:
            if value["expanded"] is True:
                problems.error(path, f"{label}expanded = true: VIC-II sprite expansion is banned "
                                     "($D017/$D01D stay 0); use a bigger sprite grid")
            else:
                problems.error(path, f"{label}expanded must be false or omitted")
        for key, item in value.items():
            if not str(key).startswith("_") and key != "expanded":
                check_no_expansion(problems, path, item, label if isinstance(item, list) else f"{label}{key}: ")
    elif isinstance(value, list):
        for item in value:
            item_label = f"{label}{item.get('id')}: " if isinstance(item, dict) and "id" in item else label
            check_no_expansion(problems, path, item, item_label)


def check_cell_rect(problems: Problems, path: Path, record: dict, label: str) -> None:
    rect = record.get("cell_rect")
    if (not isinstance(rect, list) or len(rect) != 2 or not all(is_int(v) for v in rect)
            or not (1 <= rect[0] <= MAX_CELL_RECT[0] and 1 <= rect[1] <= MAX_CELL_RECT[1])):
        problems.error(path, f"{label}cell_rect must be [1-{MAX_CELL_RECT[0]} cols, 1-{MAX_CELL_RECT[1]} rows], got {rect!r}")


def check_layer_geometry(problems: Problems, path: Path, record: dict, layers: set[str], label: str = "") -> None:
    """layer + sprite_class/cells (sprite) or cell_rect (bitmap)."""
    layer = record.get("layer")
    if layer not in layers:
        problems.error(path, f"{label}layer {layer!r} must be {'|'.join(sorted(layers))}")
        return
    if layer == "sprite":
        if "cell_rect" in record:
            problems.error(path, f"{label}sprite layer uses sprite_class/cells, not cell_rect")
        check_sprite_size(problems, path, record, label)
    elif layer == "bitmap":
        if "sprite_class" in record:
            problems.error(path, f"{label}bitmap layer uses cell_rect, not sprite_class")
        check_cell_rect(problems, path, record, label)
    elif layer == "hybrid":
        if "sprite_class" in record:
            check_sprite_size(problems, path, record, label)
        if "cell_rect" in record:
            check_cell_rect(problems, path, record, label)


def check_colors(problems: Problems, data: dict, colors: list, label: str = "") -> None:
    path = data["_path"]
    if not isinstance(colors, list):
        problems.error(path, f"{label}colors must be a list")
        return
    for color in colors:
        if not isinstance(color, dict):
            problems.error(path, f"{label}colors entries must be tables")
            continue
        c64 = color.get("c64")
        if not is_int(c64) or not 0 <= c64 <= 15:
            problems.error(path, f"{label}colour index out of range: {c64!r}")
        if "role" not in color:
            problems.error(path, f"{label}colour entry without role")
        if "required" in color and not isinstance(color["required"], bool):
            problems.error(path, f"{label}colour {color.get('role')!r}: required must be true or false")


def check_private_inks(problems: Problems, data: dict) -> None:
    """Inks 1 and 3 belong to the TALON on the sprite layer; other sprites may not require them."""
    if data.get("kind") in PRIVATE_INK_KINDS:
        return
    layer = data.get("layer")
    if layer not in {"sprite", "hybrid"}:
        return
    for color in data.get("colors", []):
        if isinstance(color, dict) and color.get("required") is True and color.get("c64") in PRIVATE_INKS:
            message = (f"colour {color.get('role')!r} requires ink {color['c64']}, a TALON private sprite ink "
                       f"(sprite layer only; mark it required = false or use another ink)")
            if layer == "sprite":
                problems.error(data["_path"], message)
            else:
                problems.warn(data["_path"], f"hybrid boss {message}; fine if that colour is only in bitmap parts")


def check_animations(problems: Problems, data: dict) -> None:
    path = data["_path"]
    seen = set()
    for anim in data.get("animations", []):
        if not isinstance(anim, dict):
            problems.error(path, "animations entries must be tables")
            continue
        anim_id = anim.get("id", "?")
        label = f"animation {anim_id!r}: "
        if anim_id in seen:
            problems.error(path, f"duplicate animation id {anim_id!r}")
        seen.add(anim_id)
        for key in ("id", "frames", "ticks_per_frame", "loop", "model", "motion"):
            if key not in anim:
                problems.error(path, f"{label}missing {key}")
        model = anim.get("model")
        frames = anim.get("frames")
        if model not in MODELS:
            problems.error(path, f"{label}unknown model {model!r} ({'|'.join(sorted(MODELS))})")
        if model == "offline":
            if not is_int(frames) or not 1 <= frames <= 16:
                problems.error(path, f"{label}offline frames must be 1..16")
        else:
            if not is_int(frames) or not 1 <= frames <= 8:
                problems.error(path, f"{label}frames must be 1..8")
            request = anim.get("request_frames")
            if request not in REQUEST_FRAMES:
                problems.error(path, f"{label}request_frames must be one of {sorted(REQUEST_FRAMES)}, got {request!r}")
            elif is_int(frames) and request < 2 * frames:
                problems.error(path, f"{label}request_frames {request} < 2 x frames {frames}")
            duration = anim.get("duration")
            if model in {"hydra", "hydra-plus"} and (not isinstance(duration, (int, float)) or duration < 3):
                problems.error(path, f"{label}hydra models need duration >= 3")
        motion = anim.get("motion", "")
        if isinstance(motion, str) and word_count(motion) >= MAX_MOTION_WORDS:
            problems.error(path, f"{label}motion is {word_count(motion)} words (limit {MAX_MOTION_WORDS})")


def check_visual(problems: Problems, data: dict) -> None:
    path = data["_path"]
    kind = data.get("kind")
    identity = data.get("identity")
    if not isinstance(identity, str) or not identity:
        problems.error(path, "missing identity")
    elif word_count(identity) > MAX_IDENTITY_WORDS:
        problems.error(path, f"identity is {word_count(identity)} words (limit {MAX_IDENTITY_WORDS})")
    if "forbidden" not in data:
        problems.error(path, "missing forbidden")
    check_colors(problems, data, data.get("colors", []))
    if kind == "character":
        if data.get("sprite_class") not in (None, "none"):
            check_sprite_size(problems, path, data, "portrait ")
        return
    for name in ("layer", "angles", "anchor", "silhouette", "proportions", "materials", "acting", "hurtbox", "colors"):
        if name not in data:
            problems.error(path, f"missing {name}")
    check_layer_geometry(problems, path, data, BOSS_LAYERS if kind in BOSS_KINDS else LAYERS)
    if "angles" in data and data.get("angles") not in ANGLES:
        problems.error(path, f"angles must be 1, 8 or 16, got {data.get('angles')!r}")
    if "anchor" in data and data.get("anchor") not in ANCHORS:
        problems.error(path, f"anchor must be center|nose|base, got {data.get('anchor')!r}")
    hurtbox = data.get("hurtbox")
    if hurtbox is not None and (not isinstance(hurtbox, dict) or not all(is_int(hurtbox.get(k)) for k in "xywh")):
        problems.error(path, "hurtbox needs integer x, y, w, h")
    check_animations(problems, data)
    check_private_inks(problems, data)
    ludo = data.get("ludo", {})
    if not isinstance(ludo, dict):
        problems.error(path, "[ludo] must be a table")
    else:
        for key, value in ludo.items():
            if not key.endswith("_prompt"):
                problems.warn(path, f"unknown ludo field {key!r} (prompt fields end in _prompt)")
            elif not isinstance(value, str) or not value.strip():
                problems.error(path, f"ludo.{key} must be a non-empty string")
        if "sprite_prompt" not in ludo and "concept_prompt" not in ludo:
            problems.warn(path, "no ludo.sprite_prompt or ludo.concept_prompt; no master still can be compiled")


def check_weakness(problems: Problems, path: Path, data: dict) -> None:
    for key in ("weak_to", "resists"):
        if key in data and data[key] not in WEAKNESSES:
            problems.error(path, f"{key} {data[key]!r} must be {'|'.join(sorted(WEAKNESSES))}")
    if data.get("weak_to") not in (None, "none") and data.get("weak_to") == data.get("resists"):
        problems.error(path, f"weak_to and resists are both {data['weak_to']!r}")


def check_drops(problems: Problems, path: Path, drops) -> None:
    if not isinstance(drops, dict) or not all(is_int(v) and v >= 0 for v in drops.values()):
        problems.error(path, f"drops must be a table of non-negative integers, got {drops!r}")


def check_cue(problems: Problems, path: Path, cue, ctx: Context, label: str) -> None:
    if not isinstance(cue, str) or not cue.startswith("cue."):
        problems.error(path, f"{label} {cue!r} is not a cue.<slug> id")
    elif ctx.cues is None:
        problems.warn(path, f"{label} {cue}: docs/music-storyboard.txt missing, cannot check")
    elif cue not in ctx.cues:
        problems.error(path, f"{label} {cue} is not in docs/music-storyboard.txt")


def check_vfx_ref(problems: Problems, path: Path, vfx_id, vfx: set[str], ctx: Context, label: str) -> None:
    if vfx_id in (None, "none"):
        return
    if ctx.vfx_missing:
        problems.warn(path, f"{label} {vfx_id!r}: 08-vfx.md missing, cannot check")
    elif vfx_id not in vfx:
        problems.error(path, f"{label} {vfx_id!r} is not defined in 08-vfx.md")


# ---------------------------------------------------------------------------
# Kind-specific checks


def check_ship(problems: Problems, data: dict) -> None:
    path = data["_path"]
    check_visual(problems, data)
    inks = data.get("private_inks")
    if inks is not None and (not isinstance(inks, list) or not set(inks) <= PRIVATE_INKS):
        problems.error(path, f"private_inks {inks!r} must be a subset of {sorted(PRIVATE_INKS)}")
    if data.get("layer") != "sprite":
        problems.error(path, "ship and pods live on the sprite layer")
    speeds = data.get("speed_levels_q8")
    if speeds is not None and (not isinstance(speeds, list) or not all(is_int(s) and s > 0 for s in speeds)
                               or speeds != sorted(speeds)):
        problems.error(path, "speed_levels_q8 must be ascending positive integers")


def check_character(problems: Problems, data: dict) -> None:
    check_visual(problems, data)
    if data.get("faction") not in FACTIONS:
        problems.error(data["_path"], f"faction {data.get('faction')!r} unknown")


def check_enemy(problems: Problems, data: dict, entities: dict, vfx: set[str], ctx: Context) -> None:
    path = data["_path"]
    check_visual(problems, data)
    for name in ("stage", "faction", "domain", "hp", "contact_damage", "score", "weak_to", "resists", "drops", "behavior"):
        if name not in data:
            problems.error(path, f"missing {name}")
    stage = data.get("stage")
    if stage != "common" and (stage not in entities or entities[stage].get("kind") != "level"):
        problems.error(path, f"stage {stage!r} is not 'common' or a level id")
    if "faction" in data and data["faction"] not in FACTIONS - {"accord"}:
        problems.error(path, f"faction {data['faction']!r} must be lattice|captured")
    if "domain" in data and data["domain"] not in DOMAINS:
        problems.error(path, f"domain {data['domain']!r} must be {'|'.join(sorted(DOMAINS))}")
    behavior = data.get("behavior", {})
    if not isinstance(behavior, dict):
        problems.error(path, "[behavior] must be a table")
        behavior = {}
    archetype = behavior.get("archetype")
    if archetype not in ARCHETYPES:
        problems.error(path, f"unknown archetype {archetype!r}")
    hp = data.get("hp")
    min_hp = 0 if archetype == "hazard" else 1
    if "hp" in data and (not is_int(hp) or hp < min_hp):
        problems.error(path, f"hp must be an integer >= {min_hp} (hp 0 is for the hazard archetype only), got {hp!r}")
    for key in ("contact_damage", "score"):
        if key in data and (not is_int(data[key]) or data[key] < 0):
            problems.error(path, f"{key} must be a non-negative integer")
    check_weakness(problems, path, data)
    if "drops" in data:
        check_drops(problems, path, data["drops"])
    for key in ("archetype", "attack", "placement", "death_vfx"):
        if key not in behavior:
            problems.error(path, f"behavior.{key} missing")
    for key in sorted(set(behavior) - BEHAVIOR_FIELDS):
        problems.warn(path, f"unknown field behavior.{key}")
    attack = behavior.get("attack", {})
    if isinstance(attack, dict) and attack and attack.get("kind") not in ATTACK_KINDS:
        problems.error(path, f"unknown attack kind {attack.get('kind')!r}")
    check_vfx_ref(problems, path, behavior.get("death_vfx"), vfx, ctx, "death_vfx")
    budget = behavior.get("sprite_budget")
    if data.get("layer") == "sprite":
        if not isinstance(budget, dict) or not is_int(budget.get("channels_on_line")):
            problems.error(path, "sprite-layer enemies need behavior.sprite_budget.channels_on_line")
        else:
            channels = budget["channels_on_line"]
            if not 0 <= channels <= SPRITES_PER_LINE:
                problems.error(path, f"sprite_budget.channels_on_line {channels} exceeds {SPRITES_PER_LINE}")
            elif channels > PLAYER_BAND_CHANNELS and data.get("domain") in PLAYER_BAND_DOMAINS:
                problems.warn(path, f"sprite_budget.channels_on_line {channels} > {PLAYER_BAND_CHANNELS} for a "
                                    f"sprite-layer {data.get('domain')} unit that can reach the player band")
    if "variant_of" in data and data["variant_of"] not in entities:
        problems.error(path, f"variant_of {data['variant_of']!r} does not exist")


def check_level(problems: Problems, data: dict, entities: dict, ctx: Context) -> None:
    path = data["_path"]
    for name in ("slug", "stage_index", "title", "theme", "length_screens", "scroll_q8", "target_minutes", "midboss",
                 "boss", "enemies", "checkpoints", "music", "background_ink", "sections", "tiles"):
        if name not in data:
            problems.error(path, f"missing {name}")
    level_id = data["id"]
    for key in ("midboss", "boss"):
        ref = data.get(key)
        if ref is None:
            continue
        if ref not in entities or entities[ref].get("kind") != key:
            problems.error(path, f"{key} {ref!r} does not exist")
        elif entities[ref].get("stage") != level_id:
            problems.error(path, f"{key} {ref} lists stage {entities[ref].get('stage')!r}, not {level_id}")
    level_enemies = data.get("enemies", [])
    for enemy in level_enemies:
        if enemy not in entities or entities[enemy].get("kind") != "enemy":
            problems.error(path, f"enemy {enemy!r} does not exist")
        elif entities[enemy].get("stage") not in ("common", level_id):
            problems.warn(path, f"enemy {enemy} belongs to {entities[enemy].get('stage')}")
    ink = data.get("background_ink")
    if ink is not None and (not is_int(ink) or not 0 <= ink <= 15):
        problems.error(path, f"background_ink must be 0..15, got {ink!r}")
    length = data.get("length_screens")
    sections = data.get("sections", [])
    if [s.get("n") for s in sections] != list(range(1, len(sections) + 1)):
        problems.error(path, "sections must be numbered 1..N in order")
    cursor = 0
    for section in sections:
        screens = section.get("screens")
        label = f"section {section.get('n')}"
        if not isinstance(screens, list) or len(screens) != 2 or not all(is_int(s) for s in screens) or screens[0] >= screens[1]:
            problems.error(path, f"{label} screens must be [first, last) with first < last, got {screens!r}")
            continue
        if screens[0] != cursor:
            problems.error(path, f"{label} starts at screen {screens[0]}, expected {cursor} (sections must be contiguous)")
        cursor = screens[1]
        name = section.get("name", "")
        if isinstance(name, str) and len(name) > MAX_STAGE_NAME:
            problems.warn(path, f"{label} name {name!r} longer than {MAX_STAGE_NAME} characters")
        for key in ("name", "description", "beat", "enemies"):
            if key not in section:
                problems.error(path, f"{label} missing {key}")
        for enemy in section.get("enemies", []):
            if enemy not in level_enemies:
                problems.error(path, f"{label} uses {enemy} which is not in the level enemies list")
    if sections and is_int(length) and cursor != length:
        problems.error(path, f"sections end at screen {cursor}, length_screens is {length}")
    for checkpoint in data.get("checkpoints", []):
        if not is_int(checkpoint) or not (is_int(length) and 0 < checkpoint < length):
            problems.error(path, f"checkpoint {checkpoint!r} is not a screen inside the stage")
    section_numbers = {s.get("n") for s in sections}
    for transition in data.get("transitions", []):
        if transition.get("section") not in section_numbers:
            problems.error(path, f"transition section {transition.get('section')!r} unknown")
    for wave in data.get("waves", []):
        screen = wave.get("screen")
        if not is_int(screen) or (is_int(length) and not 0 <= screen <= length):
            problems.error(path, f"wave screen {screen!r} outside the stage")
    tile_ids = set()
    for tile in data.get("tiles", []):
        tile_id = tile.get("id")
        if tile_id in tile_ids:
            problems.error(path, f"duplicate tile id {tile_id}")
        tile_ids.add(tile_id)
        if not isinstance(tile_id, str) or not tile_id.startswith(("tileset.", "setpiece.")):
            problems.error(path, f"tile id {tile_id!r} must be tileset.<scope>.<slug> or setpiece.<scope>.<slug>")
        if tile.get("section") not in section_numbers:
            problems.error(path, f"tile {tile_id} section {tile.get('section')!r} unknown")
        if tile.get("image_type") not in IMAGE_TYPES:
            problems.error(path, f"tile {tile_id} image_type {tile.get('image_type')!r} unknown")
        elif isinstance(tile_id, str):
            is_setpiece = tile_id.startswith("setpiece.")
            if is_setpiece != (tile.get("image_type") == SETPIECE_IMAGE_TYPE):
                problems.error(path, f"tile {tile_id}: set-pieces, and only set-pieces, use image_type "
                                     f"{SETPIECE_IMAGE_TYPE!r}")
        if tile.get("seamless", "none") not in SEAMLESS:
            problems.error(path, f"tile {tile_id} seamless {tile.get('seamless')!r} unknown")
        size = tile.get("size_cells")
        if not isinstance(size, list) or len(size) != 2 or not all(is_int(v) and v > 0 for v in size):
            problems.error(path, f"tile {tile_id} size_cells must be [cols, rows]")
        if not tile.get("subject"):
            problems.error(path, f"tile {tile_id} needs a subject")
    music = data.get("music", {})
    for key in ("cue", "mood", "bpm", "length_s", "loop"):
        if key not in music:
            problems.error(path, f"music.{key} missing")
    if "cue" in music:
        check_cue(problems, path, music["cue"], ctx, "music.cue")
    card = data.get("card_line")
    if isinstance(card, str):
        check_text_line(problems, path, card, "card_line", wrap_lines=2)
    twist = data.get("twist_line")
    if isinstance(twist, str):
        check_text_line(problems, path, twist, "twist_line")


def check_text_line(problems: Problems, path: Path, text: str, label: str, wrap_lines: int = 1) -> None:
    """Uppercase, ASCII, and fits in `wrap_lines` lines of 38 columns when word-wrapped."""
    if text != text.upper():
        problems.error(path, f"{label} must be uppercase: {text!r}")
    if not all(ord(ch) < 128 for ch in text):
        problems.error(path, f"{label} has non-ASCII characters: {text!r}")
    lines = textwrap.wrap(text, MAX_TEXT_COLUMNS, break_long_words=False) if wrap_lines > 1 else [text]
    if len(lines) > wrap_lines or any(len(line) > MAX_TEXT_COLUMNS for line in lines):
        limit = f"{wrap_lines} lines of {MAX_TEXT_COLUMNS}" if wrap_lines > 1 else f"{MAX_TEXT_COLUMNS} columns"
        problems.error(path, f"{label} does not fit {limit} ({len(text)} chars): {text!r}")


def check_boss(problems: Problems, data: dict, entities: dict, ctx: Context) -> None:
    path = data["_path"]
    kind = data.get("kind")
    check_visual(problems, data)
    for name in ("title", "stage", "hp", "weak_to", "score", "phases", "patterns"):
        if name not in data:
            problems.error(path, f"missing {name}")
    stage = data.get("stage")
    if stage not in entities or entities[stage].get("kind") != "level":
        problems.error(path, f"stage {stage!r} does not exist")
    elif entities[stage].get(kind) != data["id"]:
        problems.error(path, f"{stage} names {kind} {entities[stage].get(kind)!r}, not {data['id']}")
    check_weakness(problems, path, data)
    if "reward" in data and data["reward"] not in WEAKNESSES:
        problems.error(path, f"reward {data['reward']!r} must be a weapon ({'|'.join(sorted(WEAPONS))}) or none")
    if "drops" in data:
        check_drops(problems, path, data["drops"])
    for key in ("music_cue", "music_cue_phase2"):
        if key in data:
            check_cue(problems, path, data[key], ctx, key)
    hp = data.get("hp")
    if not is_int(hp) or hp < 1:
        problems.error(path, f"hp must be a positive integer, got {hp!r}")
    if data.get("layer") == "hybrid" and not data.get("parts"):
        problems.error(path, "hybrid bosses must list their [[parts]]")
    part_ids = set()
    for part in data.get("parts", []):
        part_id = part.get("id")
        if part_id in part_ids:
            problems.error(path, f"duplicate part id {part_id!r}")
        part_ids.add(part_id)
        check_layer_geometry(problems, path, part, PART_LAYERS, f"part {part_id}: ")
        if not is_int(part.get("hp")) or part.get("hp") < 0:
            problems.error(path, f"part {part_id}: hp must be a non-negative integer")
    pattern_ids = set()
    for pattern in data.get("patterns", []):
        pattern_id = pattern.get("id")
        label = f"pattern {pattern_id}: "
        if pattern_id in pattern_ids:
            problems.error(path, f"duplicate pattern id {pattern_id!r}")
        pattern_ids.add(pattern_id)
        for key in ("name", "telegraph", "attack", "sprite_budget", "counterplay"):
            if key not in pattern:
                problems.error(path, f"{label}missing {key}")
        name = pattern.get("name", "")
        if isinstance(name, str) and len(name) > MAX_STAGE_NAME:
            problems.warn(path, f"{label}name {name!r} longer than {MAX_STAGE_NAME} characters")
        attack = pattern.get("attack", {})
        if attack.get("kind") not in ATTACK_KINDS:
            problems.error(path, f"{label}unknown attack kind {attack.get('kind')!r}")
        enemy = attack.get("enemy")
        if enemy is not None and (enemy not in entities or entities[enemy].get("kind") != "enemy"):
            problems.error(path, f"{label}attack.enemy {enemy!r} does not exist")
        budget = pattern.get("sprite_budget", {})
        channels = budget.get("boss_channels", 0) + budget.get("bucket_channels", 0)
        if channels > SPRITES_PER_LINE:
            problems.error(path, f"{label}boss_channels + bucket_channels = {channels} > {SPRITES_PER_LINE}")
    previous_to = None
    for phase in data.get("phases", []):
        for pattern in phase.get("patterns", []):
            if pattern not in pattern_ids:
                problems.error(path, f"phase references unknown pattern {pattern!r}")
        from_hp, to_hp = phase.get("from_hp"), phase.get("to_hp")
        if not is_int(from_hp) or not is_int(to_hp) or from_hp <= to_hp:
            problems.error(path, f"phase from_hp must exceed to_hp: from_hp={from_hp!r}, to_hp={to_hp!r}")
        elif previous_to is None and is_int(hp) and from_hp != hp:
            problems.warn(path, f"first phase starts at {from_hp} HP, boss hp is {hp}")
        elif previous_to is not None and from_hp != previous_to:
            problems.warn(path, f"phase starting at {from_hp} HP does not continue from {previous_to}")
        previous_to = to_hp if is_int(to_hp) else None
        if "music_cue" in phase:
            check_cue(problems, path, phase["music_cue"], ctx, "phase music_cue")
    for occupancy in data.get("occupancy", []):
        rows = occupancy.get("rows", [])
        pose = occupancy.get("pose")
        if not isinstance(rows, list) or not rows or not all(isinstance(r, str) for r in rows):
            problems.error(path, f"occupancy {pose!r}: rows must be a list of strings")
            continue
        if any(set(r) - {"0", "1", " "} for r in rows):
            problems.error(path, f"occupancy {pose!r}: rows may contain only 0, 1 and spaces")
        if len(rows) > MAX_ROWS:
            problems.error(path, f"occupancy {pose!r}: {len(rows)} rows (limit {MAX_ROWS})")
        for row in rows:
            # Every band also carries the two bullet buckets (one player-shot, one enemy-bullet).
            if row.count("1") + BULLET_BUCKETS > SPRITES_PER_LINE:
                problems.error(path, f"occupancy {pose!r}: row {row!r} + {BULLET_BUCKETS} bullet buckets needs "
                                     f"more than {SPRITES_PER_LINE} channels")


def check_ui(problems: Problems, data: dict, entities: dict) -> None:
    path = data["_path"]
    columns = data.get("columns", 40)
    safe = data.get("safe_columns", MAX_TEXT_COLUMNS)
    seen = set()
    for entry in data.get("strings", []):
        string_id = entry.get("id")
        if string_id in seen:
            problems.error(path, f"duplicate string id {string_id}")
        seen.add(string_id)
        text = entry.get("text", "")
        width = (len(text) + 1) // 2 if entry.get("font") == "half" else len(text)
        if width > columns:
            problems.error(path, f"string {string_id} is {width} columns (screen is {columns})")
        elif width > safe and entry.get("screen") != "hud":
            problems.error(path, f"string {string_id} is {width} columns, beyond the {safe}-column safe area")
        if text != text.upper():
            problems.error(path, f"string {string_id} must be uppercase")
        if not all(ord(ch) < 128 for ch in text):
            problems.error(path, f"string {string_id} has non-ASCII characters")
    for stage in data.get("stages", []):
        level = entities.get(stage.get("id"))
        if not level or level.get("kind") != "level":
            problems.error(path, f"stages entry {stage.get('id')!r} is not a level")
            continue
        for key, expected in (("name", level.get("name")), ("subtitle", level.get("title")),
                              ("midboss", entities.get(level.get("midboss"), {}).get("name")),
                              ("boss", entities.get(level.get("boss"), {}).get("name"))):
            if key in stage and expected is not None and stage[key] != expected:
                problems.warn(path, f"stages {stage['id']} {key} {stage[key]!r} differs from the bible's {expected!r}")
        subtitle = stage.get("subtitle", "")
        if isinstance(subtitle, str) and len(subtitle) > MAX_TEXT_COLUMNS:
            problems.error(path, f"stages {stage['id']} subtitle is {len(subtitle)} columns")


def check_story(problems: Problems, data: dict) -> None:
    path = data["_path"]
    blocks = re.findall(r"(#+ [^\n]*)\n```[^\n]*\n(.*?)```", data["_body"], re.S)
    for heading, block in blocks:
        lines = block.splitlines()
        if len(lines) > MAX_SCREEN_LINES and "credits" not in heading.lower():
            problems.warn(path, f"text screen {heading.strip('# ')!r} has {len(lines)} lines (limit {MAX_SCREEN_LINES})")
        for line in lines:
            if len(line) > MAX_TEXT_COLUMNS:
                problems.error(path, f"text screen line is {len(line)} columns: {line!r}")
            if line != line.upper():
                problems.error(path, f"text screen line must be uppercase: {line!r}")


def vfx_ids(entities: dict) -> set[str]:
    ids = set()
    for entity in entities.values():
        if entity.get("kind") == "vfx":
            ids |= {v.get("id") for v in entity.get("vfx", []) if isinstance(v, dict)}
    return ids


def check_vfx(problems: Problems, data: dict) -> None:
    path = data["_path"]
    seen = set()
    for vfx in data.get("vfx", []):
        vfx_id = vfx.get("id")
        label = f"vfx {vfx_id}: "
        if vfx_id in seen:
            problems.error(path, f"duplicate vfx id {vfx_id}")
        seen.add(vfx_id)
        if not isinstance(vfx_id, str) or not vfx_id.startswith("vfx."):
            problems.error(path, f"vfx id {vfx_id!r} must be vfx.<slug>")
        name = vfx.get("name", "")
        if not isinstance(name, str) or name != name.upper():
            problems.error(path, f"{label}name must be uppercase")
        elif len(name) > MAX_NAME:
            problems.warn(path, f"{label}name {name!r} longer than {MAX_NAME} characters")
        check_layer_geometry(problems, path, vfx, VFX_LAYERS, label)
        angles = vfx.get("angles", 1)
        if angles not in ANGLES and vfx.get("source") != "mask":
            problems.error(path, f"{label}angles must be 1, 8 or 16 (hand masks excepted), got {angles!r}")
        frames = vfx.get("frames")
        if not is_int(frames) or not 1 <= frames <= 16:
            problems.error(path, f"{label}frames must be 1..16")
        for c64 in vfx.get("colors", []):
            if not is_int(c64) or not 0 <= c64 <= 15:
                problems.error(path, f"{label}colour {c64!r} out of range")
        if not vfx.get("prompt"):
            problems.error(path, f"{label}needs a prompt")


def check_forbidden_text(problems: Problems, path: Path, text: str, terms: list[str], label: str = "") -> None:
    lowered = text.lower()
    for term in terms:
        for match in re.finditer(re.escape(term), lowered):
            start = lowered.rfind("\n", 0, match.start()) + 1
            end = lowered.find("\n", match.end())
            line = lowered[start:end if end != -1 else None].strip()
            # The rule may name its own file without tripping it.
            if "forbidden-terms" in line or "forbidden_terms" in line:
                continue
            problems.error(path, f"{label}forbidden term {term!r} in: {line[:80]!r}")


def check_forbidden(problems: Problems, entities: dict, ctx: Context) -> None:
    for entity in entities.values():
        check_forbidden_text(problems, entity["_path"], entity.get("_text", ""), ctx.terms)
    for path, text in ctx.extra_texts.items():
        check_forbidden_text(problems, path, text, ctx.terms)
    style = entities.get("style", {})
    for entity_id in sorted(entities):
        entity = entities[entity_id]
        for request in compile_entity(entity, entities, style):
            check_forbidden_text(problems, entity["_path"], request["prompt"], ctx.terms,
                                 f"compiled prompt {request['step']}: ")


def run_checks(entities: dict, problems: Problems, ctx: Context | None = None) -> None:
    ctx = ctx or Context.from_disk()
    vfx = vfx_ids(entities)
    if ctx.vfx_missing is False and not any(e.get("kind") == "vfx" for e in entities.values()):
        ctx.vfx_missing = True
    for entity in entities.values():
        check_common(problems, entity)
        check_no_expansion(problems, entity["_path"], entity)
    for entity in entities.values():
        kind = entity.get("kind")
        if kind == "ship":
            check_ship(problems, entity)
        elif kind == "character":
            check_character(problems, entity)
        elif kind == "enemy":
            check_enemy(problems, entity, entities, vfx, ctx)
        elif kind == "level":
            check_level(problems, entity, entities, ctx)
        elif kind in BOSS_KINDS:
            check_boss(problems, entity, entities, ctx)
        elif kind == "ui":
            check_ui(problems, entity, entities)
        elif kind == "story":
            check_story(problems, entity)
        elif kind == "vfx":
            check_vfx(problems, entity)
    if ctx.vfx_missing:
        problems.warn(STORY / "08-vfx.md", "missing: death_vfx references were not checked")
    check_forbidden(problems, entities, ctx)


# ---------------------------------------------------------------------------
# Export


def strip_private(data: dict) -> dict:
    return {k: v for k, v in data.items() if not k.startswith("_")}


def canonical(data) -> str:
    return json.dumps(data, sort_keys=True, ensure_ascii=False, separators=(",", ":"))


def file_sha(entity: dict) -> str:
    return hashlib.sha256(entity.get("_text", "").encode("utf-8")).hexdigest()


def export_catalog(entities: dict) -> dict:
    catalog = {"format": "talon-catalog-v1", "entities": {}}
    for entity_id in sorted(entities):
        entity = entities[entity_id]
        record = strip_private(entity)
        record["_file"] = rel(entity["_path"])
        record["_sha256"] = hashlib.sha256(entity["_path"].read_bytes()).hexdigest()
        catalog["entities"][entity_id] = record
    catalog["hash"] = hashlib.sha256(canonical(catalog["entities"]).encode()).hexdigest()
    CATALOG.parent.mkdir(parents=True, exist_ok=True)
    CATALOG.write_text(json.dumps(catalog, indent=1, sort_keys=True, ensure_ascii=False) + "\n", encoding="utf-8")
    return catalog


# ---------------------------------------------------------------------------
# Prompt compilation


def palette_names(entities: dict) -> dict[int, tuple[str, str]]:
    world = entities.get("world", {})
    names = {}
    for key, value in world.get("palette", {}).items():
        names[int(key[1:])] = (value[0], value[1])
    return names


def colour_phrase(colors: list, palette: dict[int, tuple[str, str]]) -> str:
    parts = []
    for color in colors:
        if not isinstance(color, dict):
            continue
        name, hex_value = palette.get(color.get("c64"), ("?", "?"))
        parts.append(f"{color.get('role')} colour {name} ({hex_value}): {color.get('note', '')}".strip().rstrip(":"))
    return "; ".join(parts)


def request_id(entity_id: str, step: str, prompt: str) -> str:
    digest = hashlib.sha256(f"{entity_id}|{step}|{prompt}".encode()).hexdigest()[:8]
    return f"talon-{entity_id.replace('.', '-')}-{step.replace(':', '-').replace('.', '-')}-{digest}"


def anim_cost(model: str, duration: float) -> float:
    rate = {"hydra": 3.0, "hydra-plus": 4.0, "forge": 1.5, "forge-pixel": 1.5}.get(model, 0.0)
    return round(rate * duration, 1)


def setpiece_aspect(size_cells) -> str:
    """The Ludo aspect closest (in log ratio) to the set-piece's own shape in 8x8 display cells.

    Style rev 3 (P2): a 40x16 band requested at ar_9_16 came back as a tall, vertical composition every time.
    """
    cells = size_cells if isinstance(size_cells, list) and len(size_cells) == 2 and all(
        is_int(v) and v > 0 for v in size_cells) else SETPIECE_DEFAULT_CELLS
    target = math.log(cells[0] / cells[1])
    return min(SETPIECE_ASPECTS, key=lambda k: abs(math.log(SETPIECE_ASPECTS[k]) - target))


def join(*parts: str) -> str:
    """Join prompt fragments with '. ', skipping empty ones and doubled full stops."""
    cleaned = [p.strip().rstrip(".") for p in parts if isinstance(p, str) and p.strip()]
    return ". ".join(cleaned) + "." if cleaned else ""


def compile_entity(entity: dict, entities: dict, style: dict) -> list[dict]:
    palette = palette_names(entities)
    requests: list[dict] = []
    entity_id = entity["id"]
    kind = entity.get("kind")
    prefixes = dict(style.get("style", {}))
    # Per-entity phrase swaps in the shared prefixes (ship.talon rev 5: a predator, not a toy).
    swaps = entity.get("style_swaps", {})
    if isinstance(swaps, dict) and swaps:
        for key, value in list(prefixes.items()):
            if isinstance(value, str):
                for old, replacement in swaps.items():
                    value = value.replace(old, replacement)
                prefixes[key] = value
    art_style, perspective = prefixes.get("art_style", "Cel-Shaded"), prefixes.get("perspective", "Top-Down")

    def add(step, command, args, prompt, inputs=None, cap=0.5):
        requests.append({"step": step, "command": command, "args": args, "prompt": prompt,
                         "prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(), "inputs": inputs or [],
                         "cap_credits": cap, "request_id": request_id(entity_id, step, prompt)})

    if kind in VISUAL_KINDS:
        ludo = entity.get("ludo", {}) if isinstance(entity.get("ludo"), dict) else {}
        colours = colour_phrase(entity.get("colors", []), palette)
        details = join(entity.get("identity", ""), entity.get("silhouette", ""), entity.get("proportions", ""),
                       entity.get("materials", ""))
        concept = ludo.get("concept_prompt")
        sprite = ludo.get("sprite_prompt")
        if concept:
            add("concept", "image create", {"image_type": "asset", "art_style": art_style, "perspective": perspective,
                                             "aspect_ratio": "ar_1_1", "n": 4},
                join(prefixes.get("concept_prefix", ""), concept, f"Colours: {colours}" if colours else ""), cap=2.0)
        master = None
        if sprite or (concept and kind != "character") or (kind == "character" and entity.get("sprite_class")):
            prompt = join(prefixes.get("still_prefix", ""), details, sprite or "",
                          f"Colours: {colours}" if colours else "")
            if concept:
                add("master", "image edit", {"image_type": "sprite", "n": 4}, prompt,
                    inputs=[{"role": "image", "from_step": "concept"}], cap=2.0)
            else:
                add("master", "image create", {"image_type": "sprite", "art_style": art_style, "perspective": perspective,
                                               "n": 4}, prompt, cap=2.0)
            master = "master"
            fix = ludo.get("master_fix_prompt")
            if fix:
                # A masked edit of the chosen master (mask drawn offline, white = redraw); the prompt says
                # only what fills the mask, so no still prefix. The fixed master feeds stills and animations.
                add("master_fix", "image edit", {"image_type": "sprite", "mask_mode": "replace", "n": 4},
                    join(fix, f"Colours: {colours}" if colours else ""),
                    inputs=[{"role": "image", "from_step": "master"}, {"role": "mask", "from_step": "offline"}],
                    cap=4.0)
                master = "master_fix"
        for key in sorted(ludo):
            if key in {"concept_prompt", "sprite_prompt", "master_fix_prompt"} or not key.endswith("_prompt"):
                continue
            slug = key[:-len("_prompt")]
            if slug == "setpiece":  # boss set-pieces: unchanged in style rev 2, see 06-ludo-style-guide.md
                add("setpiece", "image create", {"image_type": "texture", "art_style": art_style,
                                                 "perspective": perspective, "n": 2},
                    join(prefixes.get("setpiece_prefix", ""), ludo[key]), cap=1.0)
            elif master:
                add(f"still:{slug}", "image edit", {"image_type": "sprite", "n": 4},
                    join(prefixes.get("still_prefix", ""), ludo[key], f"Colours: {colours}" if colours else ""),
                    inputs=[{"role": "image", "from_step": master}], cap=2.0)
        stills = {r["step"].split(":", 1)[1] for r in requests if r["step"].startswith("still:")}
        if master:
            for anim in entity.get("animations", []):
                anim_id = str(anim.get("id", ""))
                # An animation named <still>_<action> (wings1_idle) starts from that alternate still.
                source = next((f"still:{s}" for s in sorted(stills) if anim_id.startswith(s + "_")), master)
                model = anim.get("model", "hydra")
                if model not in LUDO_MODELS:
                    continue  # rotations, mirrors and composites are made by our tools
                prompt = join(entity.get("acting", ""), anim.get("motion", ""))
                args = {"model": model, "duration": anim.get("duration", 3), "frames": anim.get("request_frames", 16),
                        "frame_size": 0, "crop": False, "gif": True, "individual_frames": True,
                        "loop": bool(anim.get("loop", False))}
                add(f"anim:{anim_id}", "sprite animate", args, prompt,
                    inputs=[{"role": "initial_image", "from_step": source}], cap=anim_cost(model, args["duration"]))
    elif kind == "level":
        sections = {s.get("n"): s for s in entity.get("sections", [])}
        stage_name = str(entity.get("name", "")).title()
        for tile in entity.get("tiles", []):
            section = sections.get(tile.get("section"), {})
            image_type = tile.get("image_type")
            subject = f"{stage_name}, {str(section.get('name', '')).title()}: {tile.get('subject', '')}"
            aspect = "default"
            if str(tile.get("id", "")).startswith("setpiece."):
                # A full-width band of the vertically scrolling stage: tall, axis-aligned, subject only.
                image_type = SETPIECE_IMAGE_TYPE
                prompt = join(prefixes.get("setpiece_prefix", ""), subject)
                aspect = setpiece_aspect(tile.get("size_cells"))
            elif image_type == "texture":
                # A plain repeating surface: the section description lists objects that have their own tiles.
                prompt = join(prefixes.get("tile_prefix", ""), subject, prefixes.get("texture_suffix", ""))
            else:
                prompt = join(prefixes.get("tile_prefix", ""), subject, section.get("description", ""))
            add(f"tile:{str(tile.get('id', '?')).split('.')[-1]}", "image create",
                {"image_type": image_type, "art_style": art_style, "perspective": perspective,
                 "aspect_ratio": aspect, "n": 2}, prompt, cap=1.0)
        tile_slugs = {str(t.get("id", "")).split(".")[-1] for t in entity.get("tiles", [])}
        for transition in entity.get("transitions", []):
            slug = str(transition.get("asset", "setpiece")).split(".")[-1]
            if slug in tile_slugs or not transition.get("set_piece"):
                continue
            add(f"setpiece:{slug}", "image create", {"image_type": SETPIECE_IMAGE_TYPE, "art_style": art_style,
                                                     "perspective": perspective, "aspect_ratio": setpiece_aspect(None),
                                                     "n": 2},
                join(prefixes.get("setpiece_prefix", ""), transition["set_piece"]), cap=1.0)
    elif kind == "art":
        ludo = entity.get("ludo", {}) if isinstance(entity.get("ludo"), dict) else {}
        if ludo.get("art_prompt"):
            add("art", "image create", {"image_type": entity.get("image_type", "art"),
                                        "art_style": entity.get("art_style", "Illustration"),
                                        "perspective": entity.get("perspective", "Any perspective"),
                                        "aspect_ratio": entity.get("aspect_ratio", "ar_1_1"), "n": 4},
                ludo["art_prompt"], cap=2.0)
            # The chosen take, upscaled to Ludo's maximum (2048 px on the longest side); no prompt.
            add("upscale", "image upscale", {}, "", inputs=[{"role": "image", "from_step": "art"}], cap=0.5)
    elif kind == "vfx":
        for vfx in entity.get("vfx", []):
            if vfx.get("source") == "mask" or not vfx.get("prompt"):
                continue  # hand-authored pixel masks need no Ludo job
            prompt = join(prefixes.get("vfx_prefix", ""), vfx.get("prompt", ""))
            add(f"vfx:{str(vfx.get('id', '?')).split('.')[-1]}", "image create",
                {"image_type": "sprite-vfx", "art_style": art_style, "perspective": perspective, "n": 4}, prompt, cap=2.0)
    return requests


def export_prompts(entities: dict, include_all: bool) -> int:
    style = entities.get("style", {})
    PROMPTS.mkdir(parents=True, exist_ok=True)
    written = 0
    index = {}
    for entity_id in sorted(entities):
        entity = entities[entity_id]
        if entity.get("kind") not in VISUAL_KINDS | {"level", "vfx", "art"}:
            continue
        if not include_all and entity.get("status") not in {"approved", "frozen"}:
            continue
        requests = compile_entity(entity, entities, style)
        if not requests:
            continue
        record = {"format": "talon-prompts-v1", "entity": entity_id, "kind": entity.get("kind"),
                  "revision": entity.get("revision"), "story_file": rel(entity["_path"]),
                  "story_sha256": hashlib.sha256(entity["_path"].read_bytes()).hexdigest(),
                  "style_sha256": hashlib.sha256(canonical(strip_private(style)).encode()).hexdigest(),
                  "requests": requests}
        path = PROMPTS / f"{entity_id}.json"
        path.write_text(json.dumps(record, indent=1, ensure_ascii=False) + "\n", encoding="utf-8")
        index[entity_id] = {"file": rel(path), "requests": len(requests),
                            "cap_credits": round(sum(r["cap_credits"] for r in requests), 1)}
        written += 1
    (PROMPTS / "index.json").write_text(json.dumps(index, indent=1, sort_keys=True) + "\n", encoding="utf-8")
    for stale in PROMPTS.glob("*.json"):
        if stale.name != "index.json" and stale.stem not in index:
            stale.unlink()  # only approved (or --all) entities keep compiled prompts
    return written


def export_music(entities: dict) -> Path:
    MUSIC_OUT.parent.mkdir(parents=True, exist_ok=True)
    lines = ["# Music briefs collected from the bible", ""]
    levels = sorted((e for e in entities.values() if e.get("kind") == "level"), key=lambda e: e.get("stage_index", 99))
    for entity in levels:
        music = entity.get("music", {})
        lines.append(f"## {entity.get('name')} ({entity['id']}) — {music.get('cue')}")
        lines.append(f"mood: {music.get('mood')}; bpm {music.get('bpm')}; {music.get('length_s')} s; loop {music.get('loop')}")
        lines.append("sections: " + " → ".join(str(s.get("name", "")) for s in entity.get("sections", [])))
        brief = re.search(r"^#+ Music brief\s*\n(.*?)(?=^#+ |\Z)", entity["_body"], re.S | re.M)
        lines.append(brief.group(1).strip() if brief else "(no music brief section)")
        lines.append("")
    cues: dict[str, list[str]] = {}
    for entity in entities.values():
        if entity.get("kind") in BOSS_KINDS:
            for key in ("music_cue", "music_cue_phase2"):
                if key in entity:
                    cues.setdefault(entity[key], []).append(f"{entity['id']} ({key})")
            for phase in entity.get("phases", []):
                if "music_cue" in phase:
                    cues.setdefault(phase["music_cue"], []).append(f"{entity['id']} (phase {phase.get('name', '')})")
    if cues:
        lines += ["## Boss and midboss cues", ""]
        for cue in sorted(cues):
            lines.append(f"- {cue}: " + ", ".join(sorted(set(cues[cue]))))
        lines.append("")
    MUSIC_OUT.write_text("\n".join(lines), encoding="utf-8")
    return MUSIC_OUT


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("command", choices=["check", "export", "export-prompts", "export-music", "list"])
    parser.add_argument("--all", action="store_true", help="export-prompts: include draft/review entities")
    parser.add_argument("--quiet", action="store_true", help="hide warnings")
    args = parser.parse_args(argv)
    problems = Problems()
    entities = load_all(problems)
    run_checks(entities, problems)
    if args.command == "list":
        for entity_id in sorted(entities):
            entity = entities[entity_id]
            print(f"{entity_id:40} {entity.get('kind', '?'):10} {entity.get('status', '?'):9} rev {entity.get('revision', '?')}")
        return 0
    for warning in problems.warnings:
        if not args.quiet:
            print(f"warning: {warning}")
    for error in problems.errors:
        print(f"error: {error}")
    if args.command == "export-music":
        # Briefs are working notes for the music storyboard; collect them even while the bible has errors.
        print(f"wrote {rel(export_music(entities))}")
        return 1 if problems.errors else 0
    if problems.errors:
        print(f"{len(problems.errors)} error(s), {len(problems.warnings)} warning(s) in {len(entities)} entities")
        return 1
    if args.command == "export":
        catalog = export_catalog(entities)
        print(f"wrote {rel(CATALOG)} with {len(catalog['entities'])} entities, hash {catalog['hash'][:16]}")
    elif args.command == "export-prompts":
        count = export_prompts(entities, args.all)
        print(f"wrote {count} prompt files to {rel(PROMPTS)}")
    elif args.command == "check":
        print(f"ok: {len(entities)} entities, {len(problems.warnings)} warning(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
