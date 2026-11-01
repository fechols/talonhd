"""Focused tests for tools/bible.py. Run: python -m unittest discover -s tools/tests"""
from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import bible  # noqa: E402

STORY = bible.STORY

LEVEL = '''+++
id = "stage.s1"
kind = "level"
name = "BREAKWATER"
status = "review"
revision = 1
slug = "breakwater"
stage_index = 1
+++
'''

VFX = '''+++
id = "vfx"
kind = "vfx"
name = "VFX"
status = "review"
revision = 1

[[vfx]]
id = "vfx.explosion_m"
name = "BLAST M"
layer = "sprite"
sprite_class = "M"
cells = [2, 2]
frames = 6
ticks_per_frame = 3
loop = false
identity = "a round yellow blast"
colors = [7, 8, 1]
prompt = "a round yellow explosion"
+++
'''

ENEMY = '''+++
id = "enemy.common.wasp"
kind = "enemy"
name = "WASP"
stage = "common"
faction = "lattice"
status = "review"
revision = 1
layer = "sprite"
sprite_class = "S"
cells = [1, 1]
expanded = false
angles = 16
anchor = "center"
identity = "a small faceted crystal dart drone seen from directly above"
silhouette = "arrowhead, two swept vanes"
proportions = "as long as wide"
materials = "purple faceted crystal"
acting = "darts in smooth S-curves"
forbidden = ["eyes or a face"]
hurtbox = { x = -8, y = -7, w = 16, h = 14 }
domain = "air"
hp = 1
contact_damage = 1
score = 100
weak_to = "twin"
resists = "none"
drops = { spark = 0, leader = 1 }

[behavior]
archetype = "swooper"
params = { speed_q8 = 640 }
attack = { kind = "aimed", count = 1, spread_deg = 0, speed_q8 = 384, telegraph_ticks = 8, cooldown_ticks = 0 }
placement = "enters from the top"
sprite_budget = { channels_on_line = 2, bullets_max = 1 }
death_vfx = "vfx.explosion_m"

[[colors]]
role = "crystal"
c64 = 4
note = "purple body"
required = true

[[animations]]
id = "fly"
frames = 2
ticks_per_frame = 6
loop = true
model = "hydra"
duration = 3
request_frames = 16
motion = "the core pulses brighter and dimmer"

[ludo]
sprite_prompt = "pointing straight up"
+++

# WASP

## Concept
## Behaviour pattern
## Placement rules
## Readability
## Approval
'''


def check(*texts: tuple[str, str], terms: list[str] | None = None) -> bible.Problems:
    problems = bible.Problems()
    entities = {}
    for rel_path, text in texts:
        data = bible.parse_entity(text, STORY / rel_path)
        entities[data["id"]] = data
    ctx = bible.Context(terms=terms or [], cues={"cue.stage_s1"})
    bible.run_checks(entities, problems, ctx)
    return problems


def files(enemy: str = ENEMY) -> list[tuple[str, str]]:
    return [("03-enemies/common-wasp.md", enemy), ("04-levels/s1-breakwater.md", LEVEL), ("08-vfx.md", VFX)]


def enemy_errors(problems: bible.Problems) -> list[str]:
    return [e for e in problems.errors if "common-wasp" in e]


class BibleTests(unittest.TestCase):
    def test_minimal_valid_enemy(self) -> None:
        problems = check(*files())
        self.assertEqual(enemy_errors(problems), [])
        self.assertEqual([w for w in problems.warnings if "common-wasp" in w], [])

    def test_illegal_layer(self) -> None:
        problems = check(*files(ENEMY.replace('layer = "sprite"', 'layer = "overlay"')))
        self.assertTrue(any("layer 'overlay'" in e for e in enemy_errors(problems)), problems.errors)

    def test_hybrid_layer_is_for_bosses_only(self) -> None:
        problems = check(*files(ENEMY.replace('layer = "sprite"', 'layer = "hybrid"')))
        self.assertTrue(any("layer 'hybrid'" in e for e in enemy_errors(problems)), problems.errors)

    def test_forbidden_term(self) -> None:
        text = ENEMY.replace("darts in smooth S-curves", "darts like a Galaga bee")
        problems = check(*files(text), terms=["galaga"])
        hits = [e for e in enemy_errors(problems) if "forbidden term 'galaga'" in e]
        self.assertTrue(hits, problems.errors)
        # The acting line feeds the animation prompt, so the compiled prompt is caught too.
        self.assertTrue(any("compiled prompt" in e for e in hits), hits)

    def test_required_private_ink_on_sprite(self) -> None:
        problems = check(*files(ENEMY.replace("c64 = 4", "c64 = 3")))
        self.assertTrue(any("private sprite ink" in e for e in enemy_errors(problems)), problems.errors)

    def test_private_ink_allowed_on_bitmap(self) -> None:
        bitmap = (ENEMY.replace("c64 = 4", "c64 = 3")
                  .replace('layer = "sprite"\nsprite_class = "S"\ncells = [1, 1]\nexpanded = false',
                           'layer = "bitmap"\ncell_rect = [3, 3]'))
        problems = check(*files(bitmap))
        self.assertFalse(any("private sprite ink" in e for e in problems.errors), problems.errors)

    def test_hazard_may_have_zero_hp(self) -> None:
        zero = ENEMY.replace("hp = 1", "hp = 0")
        self.assertTrue(any("hp must be" in e for e in enemy_errors(check(*files(zero)))))
        hazard = zero.replace('archetype = "swooper"', 'archetype = "hazard"')
        self.assertFalse(any("hp must be" in e for e in enemy_errors(check(*files(hazard)))))

    def test_sprite_expansion_is_banned(self) -> None:
        problems = check(*files(ENEMY.replace("expanded = false", "expanded = true")))
        self.assertTrue(any("VIC-II sprite expansion is banned" in e for e in enemy_errors(problems)), problems.errors)
        # Nested records (boss parts, vfx entries) are checked too; expanded = false stays legal.
        vfx = VFX.replace("cells = [2, 2]\nframes", "cells = [2, 2]\nexpanded = true\nframes")
        problems = check(("03-enemies/common-wasp.md", ENEMY), ("04-levels/s1-breakwater.md", LEVEL), ("08-vfx.md", vfx))
        self.assertTrue(any("vfx.explosion_m: expanded = true" in e for e in problems.errors), problems.errors)
        self.assertFalse(any("expan" in e for e in enemy_errors(problems)), problems.errors)

    def test_unknown_death_vfx(self) -> None:
        problems = check(*files(ENEMY.replace('death_vfx = "vfx.explosion_m"', 'death_vfx = "vfx.nope"')))
        self.assertTrue(any("vfx.nope" in e for e in enemy_errors(problems)), problems.errors)


    def test_setpiece_and_texture_export(self) -> None:
        level = {"id": "stage.s1", "kind": "level", "name": "BREAKWATER",
                 "sections": [{"n": 1, "name": "OPEN SEA", "description": "a red buoy and islets"}],
                 "tiles": [{"id": "tileset.s1.sea", "section": 1, "image_type": "texture", "subject": "open sea",
                            "size_cells": [40, 25]},
                           {"id": "setpiece.s1.wall", "section": 1, "image_type": "vertical_scrolling_background",
                            "subject": "a wall", "size_cells": [40, 16]},
                           {"id": "setpiece.s1.hull", "section": 1, "image_type": "vertical_scrolling_background",
                            "subject": "a hull", "size_cells": [38, 50]}]}
        style = {"style": {"tile_prefix": "TILE", "texture_suffix": "PLAIN SURFACE", "setpiece_prefix": "SETPIECE"}}
        requests = {r["step"]: r for r in bible.compile_entity(level, {}, style)}
        sea, wall, hull = requests["tile:sea"], requests["tile:wall"], requests["tile:hull"]
        self.assertEqual(sea["args"]["image_type"], "texture")
        self.assertEqual(sea["prompt"], "TILE. Breakwater, Open Sea: open sea. PLAIN SURFACE.")
        self.assertNotIn("buoy", sea["prompt"])  # textures leave out the section's objects
        self.assertEqual(wall["args"]["image_type"], "vertical_scrolling_background")
        self.assertEqual(wall["args"]["aspect_ratio"], "ar_19_9")  # 40x16 cells is 2.5:1
        self.assertTrue(wall["prompt"].startswith("SETPIECE. "))
        self.assertEqual(hull["args"]["aspect_ratio"], "ar_3_4")  # 38x50 cells

    def test_setpiece_image_type_is_checked(self) -> None:
        tiles = ('[[sections]]\nn = 1\nname = "OPEN SEA"\nscreens = [0, 9]\n\n[[tiles]]\n'
                 'id = "setpiece.s1.wall"\nsection = 1\nimage_type = "sprite"\nsubject = "a wall"\n'
                 'size_cells = [40, 16]\nseamless = "none"\n')
        level = LEVEL.replace("stage_index = 1\n", "stage_index = 1\n" + tiles)
        problems = check(*[(name, level if name.startswith("04") else text) for name, text in files()])
        self.assertTrue(any("set-pieces, and only set-pieces" in e for e in problems.errors), problems.errors)
        fixed = level.replace('image_type = "sprite"', 'image_type = "vertical_scrolling_background"')
        problems = check(*[(name, fixed if name.startswith("04") else text) for name, text in files()])
        self.assertFalse(any("set-pieces, and only set-pieces" in e for e in problems.errors), problems.errors)

if __name__ == "__main__":
    unittest.main()
