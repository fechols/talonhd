#!/usr/bin/env python3
"""Ludo GLB -> TALON HD mesh (`assets/models/<id>.mesh`).

    python tools/model_prep.py <model.glb> <request.json> [--out assets/models] [--forward -z] [--up y]
    python tools/model_prep.py <model.glb> --axes          print bounds per axis (to pick --forward/--up)

Steps:
1. Parse the glTF binary (JSON + BIN chunks; POSITION, TEXCOORD_0, indices, node matrices).
2. Sample the texture under each face (centroid and vertices) for its paint color.
3. Classify each face into the request's materials by their `source` paint description
   (color words -> reference RGB; nearest wins).
4. Decimate by quadric edge collapse to the bible's triangle target; material borders cost extra so
   paint regions keep their shape; collapses that flip a face are rejected.
5. Re-axis to TALON HD model space (x right, y forward/nose, z up toward the viewer) and scale so the
   object is `length_px` hires pixels long nose to tail.

The .mesh file is plain text, read by `talonhd bake-model` (no dependencies on either side):
    TMESH1
    name <id>
    canvas <w> <h>
    orient <key>=<value> ...
    material <name> <emissive 0|1> <floor> <albedo> <role> [<role> ...]   (albedo from the paint's luminance, relative to the brightest paint)
    v <x> <y> <z>
    f <a> <b> <c> <material index>
"""
from __future__ import annotations

import argparse
import heapq
import io
import json
import struct
import sys
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]

# Reference paint colors for `source` descriptions (longest phrase first wins).
PAINT = {
    "light blue": (110, 160, 235), "dark grey": (60, 60, 64), "dark glass": (55, 60, 75), "smoked": (55, 60, 75), "steel grey": (110, 115, 120),
    "light grey": (190, 190, 190), "mid grey": (128, 128, 128), "cloud grey": (190, 190, 200),
    "light green": (160, 235, 150), "light red": (220, 110, 110), "light stone": (215, 205, 180),
    "safety yellow": (240, 210, 40), "cobalt blue": (40, 60, 190),
    "white": (240, 240, 240), "grey": (128, 128, 128), "cream": (235, 225, 190), "terracotta": (185, 90, 55),
    "yellow": (235, 220, 60), "orange": (220, 120, 40), "olive": (120, 125, 60), "green": (70, 160, 70),
    "purple": (150, 60, 170), "cyan": (90, 225, 225), "blue": (60, 70, 200), "red": (170, 50, 50),
    "black": (20, 20, 20), "dark": (60, 60, 64), "brown": (100, 65, 30), "stone": (200, 190, 165),
    "concrete": (140, 140, 140), "glass": (110, 160, 235), "crystal": (90, 225, 225), "glow": (90, 225, 225),
}


def paint_of(source: str) -> tuple[int, int, int]:
    s = source.lower()
    for word in sorted(PAINT, key=len, reverse=True):
        if word in s:
            return PAINT[word]
    raise SystemExit(f"no known paint color in {source!r}; add one to PAINT")


# --------------------------------------------------------------------------- glTF

COMPONENT = {5120: "b", 5121: "B", 5122: "h", 5123: "H", 5125: "I", 5126: "f"}
WIDTH = {"SCALAR": 1, "VEC2": 2, "VEC3": 3, "VEC4": 4, "MAT4": 16}


def read_glb(path: Path):
    data = path.read_bytes()
    magic, _version, _length = struct.unpack_from("<4sII", data, 0)
    assert magic == b"glTF", "not a GLB"
    off, js, binary = 12, None, b""
    while off < len(data):
        n, kind = struct.unpack_from("<II", data, off)
        chunk = data[off + 8: off + 8 + n]
        if kind == 0x4E4F534A:
            js = json.loads(chunk)
        elif kind == 0x004E4942:
            binary = chunk
        off += 8 + n
    return js, binary


def accessor(js, binary, index):
    a = js["accessors"][index]
    view = js["bufferViews"][a["bufferView"]]
    comp = COMPONENT[a["componentType"]]
    width = WIDTH[a["type"]]
    start = view.get("byteOffset", 0) + a.get("byteOffset", 0)
    item = struct.calcsize("<" + comp) * width
    stride = view.get("byteStride", item)
    out = np.empty((a["count"], width), dtype=np.float64)
    for i in range(a["count"]):
        out[i] = struct.unpack_from("<" + comp * width, binary, start + i * stride)
    return out


def node_matrices(js):
    """World matrix per mesh index (first instance)."""
    def local(node):
        if "matrix" in node:
            return np.array(node["matrix"], dtype=np.float64).reshape(4, 4).T
        m = np.eye(4)
        if "scale" in node:
            m = np.diag(list(node["scale"]) + [1.0]) @ m
        if "rotation" in node:
            x, y, z, w = node["rotation"]
            r = np.array([[1 - 2 * (y * y + z * z), 2 * (x * y - z * w), 2 * (x * z + y * w)],
                          [2 * (x * y + z * w), 1 - 2 * (x * x + z * z), 2 * (y * z - x * w)],
                          [2 * (x * z - y * w), 2 * (y * z + x * w), 1 - 2 * (x * x + y * y)]])
            rm = np.eye(4)
            rm[:3, :3] = r
            m = rm @ m
        if "translation" in node:
            t = np.eye(4)
            t[:3, 3] = node["translation"]
            m = t @ m
        return m

    out = {}
    nodes = js.get("nodes", [])

    def walk(i, parent):
        m = parent @ local(nodes[i])
        if "mesh" in nodes[i] and nodes[i]["mesh"] not in out:
            out[nodes[i]["mesh"]] = m
        for c in nodes[i].get("children", []):
            walk(c, m)

    scene = js.get("scenes", [{}])[js.get("scene", 0)]
    for root in scene.get("nodes", range(len(nodes))):
        walk(root, np.eye(4))
    return out


def load_mesh(path: Path):
    """Positions (n,3), uvs (n,2), faces (m,3), texture image or None."""
    js, binary = read_glb(path)
    mats = node_matrices(js)
    pos_all, uv_all, faces_all, offset = [], [], [], 0
    for mi, mesh in enumerate(js["meshes"]):
        world = mats.get(mi, np.eye(4))
        for prim in mesh["primitives"]:
            p = accessor(js, binary, prim["attributes"]["POSITION"])
            p = (np.c_[p, np.ones(len(p))] @ world.T)[:, :3]
            uv = accessor(js, binary, prim["attributes"]["TEXCOORD_0"]) if "TEXCOORD_0" in prim["attributes"] else np.zeros((len(p), 2))
            idx = accessor(js, binary, prim["indices"]).astype(np.int64).reshape(-1, 3) if "indices" in prim else np.arange(len(p)).reshape(-1, 3)
            pos_all.append(p)
            uv_all.append(uv)
            faces_all.append(idx + offset)
            offset += len(p)
    tex = None
    if js.get("images"):
        im = js["images"][0]
        view = js["bufferViews"][im["bufferView"]]
        raw = binary[view.get("byteOffset", 0): view.get("byteOffset", 0) + view["byteLength"]]
        tex = np.asarray(Image.open(io.BytesIO(raw)).convert("RGB"), dtype=np.float64)
    return np.vstack(pos_all), np.vstack(uv_all), np.vstack(faces_all), tex


# --------------------------------------------------------------------------- color

def face_colors(pos, uv, faces, tex):
    if tex is None:
        return np.full((len(faces), 3), 200.0)
    h, w, _ = tex.shape
    out = np.zeros((len(faces), 3))
    for fi, f in enumerate(faces):
        samples = [uv[f].mean(axis=0)] + [uv[f[k]] * 0.6 + uv[f].mean(axis=0) * 0.4 for k in range(3)]
        acc = np.zeros(3)
        for s in samples:
            x = int(np.clip((s[0] % 1.0) * w, 0, w - 1))
            y = int(np.clip((s[1] % 1.0) * h, 0, h - 1))
            acc += tex[y, x]
        out[fi] = acc / len(samples)
    return out


def classify(colors, materials):
    """Nearest paint by luminance (normalized to the texture's own 5-95 % range, since Ludo textures
    carry baked shading) and chroma (the color's offset from grey, compared as-is)."""
    refs = np.array([paint_of(m["source"]) for m in materials], dtype=np.float64)

    def lum(c):
        return c @ np.array([0.299, 0.587, 0.114])

    L = lum(colors)
    lo, hi = np.percentile(L, 5), np.percentile(L, 95)
    Ln = np.clip((L - lo) / max(hi - lo, 1.0), 0.0, 1.0) * 220.0 + 20.0
    chroma = colors - L[:, None]
    ref_L = lum(refs)
    ref_c = refs - ref_L[:, None]
    # Texture chroma is muted; scale it up to the paints' saturation scale.
    chroma = chroma * 2.0
    d = (Ln[:, None] - ref_L[None, :]) ** 2 + 1.5 * ((chroma[:, None, :] - ref_c[None, :, :]) ** 2).sum(axis=2)
    return d.argmin(axis=1)


# --------------------------------------------------------------------------- decimation

def plane_quadric(a, b, c):
    n = np.cross(b - a, c - a)
    area = np.linalg.norm(n)
    if area < 1e-12:
        return np.zeros((4, 4))
    n /= area
    p = np.append(n, -n.dot(a))
    return np.outer(p, p) * area


def decimate(pos, faces, mat, target, border_weight=50.0):
    pos = pos.copy()
    faces = [list(f) for f in faces]
    alive = [True] * len(faces)
    vfaces = [set() for _ in range(len(pos))]
    for fi, f in enumerate(faces):
        for v in f:
            vfaces[v].add(fi)
    Q = [np.zeros((4, 4)) for _ in range(len(pos))]
    for fi, f in enumerate(faces):
        q = plane_quadric(pos[f[0]], pos[f[1]], pos[f[2]])
        for v in f:
            Q[v] += q
    # Edges: border (open) edges and material borders get a perpendicular constraint quadric.
    edge_faces: dict[tuple[int, int], list[int]] = {}
    for fi, f in enumerate(faces):
        for k in range(3):
            a, b = sorted((f[k], f[(k + 1) % 3]))
            edge_faces.setdefault((a, b), []).append(fi)
    for (a, b), fl in edge_faces.items():
        if len(fl) == 1 or len({mat[x] for x in fl}) > 1:
            f = faces[fl[0]]
            n = np.cross(pos[f[1]] - pos[f[0]], pos[f[2]] - pos[f[0]])
            e = pos[b] - pos[a]
            m = np.cross(e, n)
            ln = np.linalg.norm(m)
            if ln < 1e-12:
                continue
            m /= ln
            p = np.append(m, -m.dot(pos[a]))
            q = np.outer(p, p) * border_weight * max(np.linalg.norm(e), 1e-6)
            Q[a] += q
            Q[b] += q

    version = [0] * len(pos)

    def cost(a, b):
        q = Q[a] + Q[b]
        best = None
        for cand in (pos[a], pos[b], (pos[a] + pos[b]) / 2):
            v = np.append(cand, 1.0)
            c = v @ q @ v
            if best is None or c < best[0]:
                best = (c, cand.copy())
        return best

    heap = []
    for (a, b) in edge_faces:
        c, p = cost(a, b)
        heapq.heappush(heap, (c, a, b, version[a], version[b], tuple(p)))
    count = len(faces)

    def flips(v, newp, skip):
        for fi in vfaces[v]:
            if not alive[fi] or fi in skip:
                continue
            f = faces[fi]
            old = [pos[x] for x in f]
            new = [newp if x == v else pos[x] for x in f]
            n0 = np.cross(old[1] - old[0], old[2] - old[0])
            n1 = np.cross(new[1] - new[0], new[2] - new[0])
            if n0.dot(n1) <= 1e-12 * (np.linalg.norm(n0) * np.linalg.norm(n1) + 1e-30):
                return True
        return False

    while count > target and heap:
        c, a, b, va, vb, p = heapq.heappop(heap)
        if version[a] != va or version[b] != vb or not vfaces[a] or not vfaces[b]:
            continue
        newp = np.array(p)
        shared = vfaces[a] & vfaces[b]
        if flips(a, newp, shared) or flips(b, newp, shared):
            continue
        # Collapse b into a.
        for fi in shared:
            if alive[fi]:
                alive[fi] = False
                count -= 1
                for v in faces[fi]:
                    vfaces[v].discard(fi)
        for fi in list(vfaces[b]):
            faces[fi] = [a if x == b else x for x in faces[fi]]
            vfaces[a].add(fi)
        vfaces[b] = set()
        pos[a] = newp
        Q[a] = Q[a] + Q[b]
        version[a] += 1
        version[b] += 1
        neighbours = {x for fi in vfaces[a] for x in faces[fi] if x != a}
        for n in neighbours:
            cc, pp = cost(a, n)
            heapq.heappush(heap, (cc, min(a, n), max(a, n), version[min(a, n)], version[max(a, n)], tuple(pp)))
    keep = [fi for fi in range(len(faces)) if alive[fi] and len(set(faces[fi])) == 3]
    used = sorted({v for fi in keep for v in faces[fi]})
    remap = {v: i for i, v in enumerate(used)}
    return pos[used], np.array([[remap[v] for v in faces[fi]] for fi in keep]), np.array([mat[fi] for fi in keep])


# --------------------------------------------------------------------------- axes

AXES = {"x": (0, 1), "-x": (0, -1), "y": (1, 1), "-y": (1, -1), "z": (2, 1), "-z": (2, -1)}


def reaxis(pos, forward, up):
    fi, fs = AXES[forward]
    ui, us = AXES[up]
    ri = 3 - fi - ui
    out = np.zeros_like(pos)
    out[:, 1] = pos[:, fi] * fs
    out[:, 2] = pos[:, ui] * us
    out[:, 0] = pos[:, ri]
    # Keep it right-handed: x = forward × up rotated, i.e. x must equal y × z orientation.
    # Check handedness with the basis determinant; mirror x if needed.
    basis = np.zeros((3, 3))
    basis[1, fi] = fs
    basis[2, ui] = us
    basis[0, ri] = 1
    if np.linalg.det(basis) < 0:
        out[:, 0] = -out[:, 0]
    return out


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("glb")
    ap.add_argument("request", nargs="?")
    ap.add_argument("--out", default=str(ROOT / "assets" / "models"))
    ap.add_argument("--forward", default="-z")
    ap.add_argument("--up", default="y")
    ap.add_argument("--axes", action="store_true")
    a = ap.parse_args(argv)
    pos, uv, faces, tex = load_mesh(Path(a.glb))
    if a.axes:
        lo, hi = pos.min(axis=0), pos.max(axis=0)
        for i, n in enumerate("xyz"):
            print(f"{n}: {lo[i]:+.3f} .. {hi[i]:+.3f}  size {hi[i] - lo[i]:.3f}")
        print(f"{len(pos)} vertices, {len(faces)} triangles, texture {None if tex is None else tex.shape}")
        return 0
    req = json.loads(Path(a.request).read_text(encoding="utf-8"))
    bake = req["bake"]
    materials = bake["materials"]
    colors = face_colors(pos, uv, faces, tex)
    mat = classify(colors, materials)
    counts = np.bincount(mat, minlength=len(materials))
    print("faces per material before decimation:", {m["name"]: int(c) for m, c in zip(materials, counts)})
    pos2, faces2, mat2 = decimate(pos, faces, mat, bake["tris"])
    print(f"decimated {len(faces)} -> {len(faces2)} triangles, {len(pos2)} vertices")
    p = reaxis(pos2, a.forward, a.up)
    p -= (p.min(axis=0) + p.max(axis=0)) / 2
    length = p[:, 1].max() - p[:, 1].min()
    p *= bake["length_px"] / length
    out = Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    path = out / f"{req['id']}.mesh"
    orient = " ".join(f"{k}={v}" for k, v in bake.get("orient", {}).items())
    lines = ["TMESH1", f"name {req['id']}", f"canvas {bake['canvas'][0]} {bake['canvas'][1]}", f"orient {orient}"]
    # Albedo relative to the model's brightest lit paint: the brightest paint lights to the top of its ramp,
    # darker paints (trim, concrete under white) sit lower, never below 0.45.
    def paint_lum(m):
        r, g, b = paint_of(m["source"])
        return 0.299 * r + 0.587 * g + 0.114 * b
    lmax = max((paint_lum(m) for m in materials if not m.get("emissive")), default=1.0)
    for m in materials:
        albedo = 0.45 + 0.55 * paint_lum(m) / lmax
        lines.append(f"material {m['name']} {1 if m.get('emissive') else 0} {m.get('floor', 0)} {albedo:.2f} " + " ".join(m["ramp"]))
    lines += [f"v {x:.4f} {y:.4f} {z:.4f}" for x, y, z in p]
    lines += [f"f {f[0]} {f[1]} {f[2]} {m}" for f, m in zip(faces2, mat2)]
    path.write_text("\n".join(lines) + "\n", encoding="utf-8", newline="\n")
    print(f"wrote {path.relative_to(ROOT).as_posix()}: span {p[:, 0].max() - p[:, 0].min():.1f} x {length * bake['length_px'] / length:.1f} x {p[:, 2].max() - p[:, 2].min():.1f} hires px")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
