#!/usr/bin/env python3
"""The TALON HD bible layer (docs/story/hd): checks and Ludo model prompts.

    python tools/bible_hd.py check                 palette law, ladder, ships, models, cross-references
    python tools/bible_hd.py export-prompts [--all] compile Ludo model requests to assets/prompts/hd/
    python tools/bible_hd.py palettes [--out DIR]   palette cards (PPM) for every stage

The base bible (docs/story, checked by tools/bible.py) keeps identity and behaviour; this layer decides
colors (roles), palettes, the promotion ladder and how every entity is built as a 3D model.
"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import bible  # noqa: E402

ROOT = bible.ROOT
HD = bible.STORY / "hd"
PROMPTS = ROOT / "assets" / "prompts" / "hd"
ROLES = ("void", "dark", "light", "hot")
MAX_IDENTITY_WORDS = 25
CARD_COLUMNS = 38
CARD_LINES = 3
APPROVED = {"approved", "frozen"}


def load_hd(problems: bible.Problems) -> dict[str, dict]:
    out: dict[str, dict] = {}
    for path in sorted(HD.rglob("*.md")):
        if path.name == "README.md":
            continue
        try:
            data = bible.parse_entity(path.read_text(encoding="utf-8"), path)
        except Exception as error:  # noqa: BLE001
            problems.error(path, f"cannot parse: {error}")
            continue
        if data.get("id") in out:
            problems.error(path, f"duplicate id {data.get('id')}")
            continue
        out[data["id"]] = data
    return out


def wrap(text: str, width: int = CARD_COLUMNS) -> list[str]:
    lines, cur = [], ""
    for word in text.split():
        if cur and len(cur) + 1 + len(word) > width:
            lines.append(cur)
            cur = word
        else:
            cur = f"{cur} {word}" if cur else word
    if cur:
        lines.append(cur)
    return lines


class Checker:
    def __init__(self, problems: bible.Problems, base: dict[str, dict], hd: dict[str, dict]):
        self.p, self.base, self.hd = problems, base, hd
        self.rules = hd.get("hd.rules", {})
        self.luma = {int(k): v for k, v in self.rules.get("luma", {}).items()}
        self.terms = bible.forbidden_terms()

    def err(self, data: dict, msg: str) -> None:
        self.p.error(data["_path"], msg)

    def common(self, data: dict) -> None:
        for f in ("id", "kind", "name", "status", "revision"):
            if f not in data:
                self.err(data, f"missing {f}")
        if data.get("status") not in bible.STATUSES:
            self.err(data, f"bad status {data.get('status')}")
        if "## Approval" not in data["_body"]:
            self.err(data, "missing ## Approval section")
        low = data["_text"].lower()
        for term in self.terms:
            if term in low:
                self.err(data, f"forbidden term {term!r}")

    def palette(self, data: dict, rung: dict, label: str) -> None:
        pal = rung.get("palette")
        if not (isinstance(pal, list) and len(pal) == 4 and all(isinstance(c, int) and 0 <= c <= 15 for c in pal)):
            self.err(data, f"{label}: palette must be 4 C64 colors 0..15")
            return
        vr = self.rules.get("value_rules", {})
        if vr.get("distinct") and len(set(pal)) != 4:
            self.err(data, f"{label}: palette colors must be distinct {pal}")
        lv = [self.luma[c] for c in pal]
        if not lv[0] < lv[1] < lv[2]:
            self.err(data, f"{label}: value order void<dark<light fails, luma {lv[:3]}")
        if lv[3] - lv[0] < vr.get("hot_contrast", 3):
            self.err(data, f"{label}: hot luma {lv[3]} too close to void {lv[0]}")
        if len(rung.get("palette_names", [])) != 4:
            self.err(data, f"{label}: palette_names needs 4 entries")

    def ladder(self, data: dict) -> None:
        rungs = data.get("rungs", [])
        stages = sorted(k for k, v in self.base.items() if v.get("kind") == "level")
        if [r.get("stage") for r in rungs] != stages:
            self.err(data, f"rungs must cover {stages} in order")
        ranks = set()
        for i, r in enumerate(rungs):
            label = r.get("stage", f"rung {i}")
            self.palette(data, r, label)
            if r.get("rank") in ranks:
                self.err(data, f"{label}: rank {r.get('rank')} repeats")
            ranks.add(r.get("rank"))
            ship = self.hd.get(r.get("ship"))
            if not ship or ship.get("kind") != "hd_ship":
                self.err(data, f"{label}: unknown ship {r.get('ship')}")
            elif ship.get("stage") != r.get("stage"):
                self.err(data, f"{label}: ship {ship['id']} belongs to {ship.get('stage')}")
            light = r.get("light", {})
            d = light.get("dir", [])
            if (len(d) != 3 or d[2] <= 0 or not 0.0 <= light.get("ambient", -1) <= 0.5
                    or not 0.5 <= light.get("intensity", 0) <= 1.3 or not light.get("time")):
                self.err(data, f"{label}: light needs time, dir (sun above the horizon, z > 0), ambient 0-0.5, intensity 0.5-1.3")
            for key in ("clear_line", "quill_reply"):
                text = r.get(key, "")
                if not text or text != text.upper():
                    self.err(data, f"{label}: {key} must be uppercase and present")
                if len(wrap(text)) > CARD_LINES:
                    self.err(data, f"{label}: {key} wraps to more than {CARD_LINES} lines of {CARD_COLUMNS}")
        diff = self.rules.get("value_rules", {}).get("stage_change", 2)
        for a, b in zip(rungs, rungs[1:]):
            pa, pb = a.get("palette", []), b.get("palette", [])
            if len(pa) == 4 and len(pb) == 4 and sum(x != y for x, y in zip(pa, pb)) < diff:
                self.err(data, f"{a.get('stage')} -> {b.get('stage')}: palettes differ in fewer than {diff} roles")

    def model(self, data: dict, m: dict, label: str) -> None:
        classes = self.rules.get("size_classes", {})
        sc = classes.get(m.get("size_class"))
        if sc is None:
            self.err(data, f"{label}: unknown size_class {m.get('size_class')}")
            return
        canvas = m.get("canvas", [])
        if len(canvas) != 2 or max(canvas) > sc["max_canvas"] or min(canvas) < 1:
            self.err(data, f"{label}: canvas {canvas} outside size class {m.get('size_class')} (max {sc['max_canvas']})")
        length = m.get("length_px", 0)
        yawing = any(k in m.get("orient", {}) for k in ("yaw", "tumble"))
        if len(canvas) == 2 and (canvas[1] < length or (yawing and canvas[0] * 2 < length)):
            self.err(data, f"{label}: canvas {canvas} cannot hold length_px {length} (height >= length; width*2 >= length when it yaws)")
        lo, hi = sc["tris"]
        if not lo <= m.get("tris", 0) <= hi:
            self.err(data, f"{label}: tris {m.get('tris')} outside {lo}-{hi}")
        mats = m.get("materials", [])
        if not mats:
            self.err(data, f"{label}: no materials")
        for mat in mats:
            ramp = mat.get("ramp", [])
            if not ramp or any(r not in ROLES for r in ramp):
                self.err(data, f"{label}: material {mat.get('name')}: ramp roles must be from {ROLES}")
            elif [ROLES.index(r) for r in ramp] != sorted(ROLES.index(r) for r in ramp):
                self.err(data, f"{label}: material {mat.get('name')}: ramp must run darkest first")
            if mat.get("emissive") and len(ramp) != 1:
                self.err(data, f"{label}: emissive material {mat.get('name')} takes one role")
            if not mat.get("source"):
                self.err(data, f"{label}: material {mat.get('name')} needs a source paint description")
        gen = m.get("ludo", {}).get("generator")
        if gen not in ("hero", "prop", "hand"):
            self.err(data, f"{label}: ludo.generator must be hero, prop or hand (hand: built in src/hand.rs)")
        for part in m.get("parts", []):
            if len(part.get("canvas", [])) != 2 or max(part["canvas"]) > 512:
                self.err(data, f"{label}: part {part.get('id')} canvas")

    def ship(self, data: dict) -> None:
        for f in ("prototype", "stage", "identity", "silhouette", "materials", "speed_levels_q8", "hitbox", "model"):
            if f not in data:
                self.err(data, f"missing {f}")
        if len(data.get("identity", "").split()) > MAX_IDENTITY_WORDS:
            self.err(data, f"identity over {MAX_IDENTITY_WORDS} words")
        if data.get("base") and data["base"] not in self.base:
            self.err(data, f"base {data['base']} not in the base bible")
        speeds = data.get("speed_levels_q8", [])
        if len(speeds) != 6 or speeds != sorted(speeds):
            self.err(data, "speed_levels_q8: 6 rising levels")
        self.model(data, data.get("model", {}), "model")
        ladder = self.hd.get("hd.ladder", {})
        if not any(r.get("ship") == data["id"] for r in ladder.get("rungs", [])):
            self.err(data, "ship is on no rung of the ladder")

    def models(self, data: dict) -> None:
        if data.get("stage") not in self.base:
            self.err(data, f"unknown stage {data.get('stage')}")
        level = self.base.get(data.get("stage"), {})
        wanted = set(level.get("enemies", [])) | {level.get("boss"), level.get("midboss")} - {None}
        seen = set()
        for m in data.get("models", []):
            of = m.get("of")
            target = self.base.get(of)
            if target is None:
                self.err(data, f"model of unknown entity {of}")
                continue
            seen.add(of)
            self.model(data, m, of)
            for part in m.get("parts", []):
                ids = {p.get("id") for p in target.get("parts", [])}
                if ids and part.get("id") not in ids:
                    self.err(data, f"{of}: part {part.get('id')} not among the base parts {sorted(ids)}")
            if "phase2_hot" in m and not 0 <= m["phase2_hot"] <= 15:
                self.err(data, f"{of}: phase2_hot must be a C64 color")
        missing = wanted - seen
        if missing:
            self.p.warn(data["_path"], f"stage entities without a model yet: {sorted(missing)}")

    def run(self) -> None:
        for need in ("hd.rules", "hd.art", "hd.ladder"):
            if need not in self.hd:
                self.p.error(HD, f"missing {need}")
        if len(self.luma) != 16:
            self.p.error(HD, "hd.rules [luma] must list all 16 colors")
        for data in self.hd.values():
            self.common(data)
            kind = data.get("kind")
            if kind == "hd_ladder":
                self.ladder(data)
            elif kind == "hd_ship":
                self.ship(data)
            elif kind == "hd_models":
                self.models(data)


def check() -> int:
    problems = bible.Problems()
    base = bible.load_all(problems)
    hd = load_hd(problems)
    Checker(problems, base, hd).run()
    errors, warnings = problems.errors, problems.warnings
    for w in warnings:
        print(f"warning: {w}")
    for e in errors:
        print(f"error: {e}")
    print(("FAILED" if errors else "ok") + f": {len(hd)} HD entities, {len(errors)} error(s), {len(warnings)} warning(s)")
    return 1 if errors else 0


def model_request(art: dict, entity_id: str, identity: str, silhouette: str, materials: str, model: dict) -> dict:
    lm = art["ludo_model"]
    paints = "; ".join(f"{m['name']}: {m['source']}" for m in model["materials"])
    prompt = (f"{lm['image_prefix']}. Subject: {identity}. Shape: {silhouette}. Paint: {materials}. "
              f"Parts painted distinctly: {paints}. {lm['image_suffix']}")
    gen = model.get("ludo", {}).get("generator", lm["create"]["generator_default"])
    return {
        "id": entity_id,
        "create_image": {"prompt": prompt},
        "create": {
            "model": gen,
            "texture_type": model.get("ludo", {}).get("texture", lm["create"]["texture_type"]),
            "texture_size": lm["create"]["texture_size"],
            "target_num_faces": max(lm["create"]["target_faces_floor"], model["tris"] * 8),
            "rig": bool(model.get("ludo", {}).get("rig")),
        },
        "bake": {k: model[k] for k in ("size_class", "canvas", "length_px", "tris", "orient", "materials") if k in model},
    }


def export_prompts(include_all: bool) -> int:
    problems = bible.Problems()
    base = bible.load_all(problems)
    hd = load_hd(problems)
    art = hd["hd.art"]
    PROMPTS.mkdir(parents=True, exist_ok=True)
    n = 0
    for data in hd.values():
        if data.get("kind") == "hd_ship":
            if not include_all and data.get("status") not in APPROVED:
                continue
            if data["model"].get("ludo", {}).get("generator") == "hand":
                continue  # built in src/hand.rs
            req = model_request(art, data["id"], data["identity"], data["silhouette"], data["materials"], data["model"])
        elif data.get("kind") == "hd_models":
            if not include_all and data.get("status") not in APPROVED:
                continue
            for m in data.get("models", []):
                if m.get("ludo", {}).get("generator") == "hand":
                    continue  # built in src/hand.rs, nothing to ask Ludo for
                b = base[m["of"]]
                req = model_request(art, m["of"], b.get("identity", ""), b.get("silhouette", ""), b.get("materials", ""), m)
                (PROMPTS / f"{m['of']}.json").write_text(json.dumps(req, indent=2) + "\n", encoding="utf-8")
                n += 1
            continue
        else:
            continue
        (PROMPTS / f"{data['id']}.json").write_text(json.dumps(req, indent=2) + "\n", encoding="utf-8")
        n += 1
    print(f"wrote {n} request(s) to {PROMPTS.relative_to(ROOT).as_posix()}")
    return 0


C64_RGB = {0: (0, 0, 0), 1: (247, 247, 247), 2: (141, 47, 52), 3: (106, 212, 205), 4: (152, 53, 164),
           5: (76, 180, 66), 6: (44, 41, 177), 7: (239, 239, 93), 8: (152, 78, 32), 9: (91, 56, 0),
           10: (209, 103, 109), 11: (74, 74, 74), 12: (123, 123, 123), 13: (159, 239, 147),
           14: (109, 106, 239), 15: (178, 178, 178)}


def palettes(out: Path) -> int:
    """One strip per stage: void, dark, light, hot swatches (PPM)."""
    problems = bible.Problems()
    hd = load_hd(problems)
    rungs = hd["hd.ladder"]["rungs"]
    sw, sh = 48, 24
    w, h = sw * 4, sh * len(rungs)
    px = bytearray()
    for r in rungs:
        for _ in range(sh):
            for c in r["palette"]:
                px += bytes(C64_RGB[c]) * sw
    out.mkdir(parents=True, exist_ok=True)
    path = out / "palettes.ppm"
    path.write_bytes(f"P6 {w} {h} 255\n".encode() + bytes(px))
    for r in rungs:
        print(f"{r['stage']:9} {r['rank']:12} {r['ship']:20} " + "  ".join(
            f"{role}={c:2d} {name}" for role, c, name in zip(ROLES, r["palette"], r["palette_names"])))
    print(f"wrote {path.relative_to(ROOT).as_posix()}")
    return 0


def main(argv: list[str]) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("command", choices=["check", "export-prompts", "palettes"])
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--out", default=str(ROOT / "output" / "bible"))
    a = ap.parse_args(argv)
    if a.command == "check":
        return check()
    if a.command == "export-prompts":
        return export_prompts(a.all)
    return palettes(Path(a.out))


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
