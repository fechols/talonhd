//! The S2 TERRACES roster, hand-built (host only) to the approved `[model]` blocks in
//! `docs/story/hd/models/s2.md` and rendered under S2's afternoon sun (`s2::SUN`, upper right): PIKE,
//! KESTREL, ROOF HATCH, BELL TURRET, DOME MORTAR, the CAMPANILE's crown and bell hub, and the ASTROLABE
//! (shutter halves, three rings, the lens). S1's WASP and PILLBOX come back re-lit by the same sun.
//!
//! Units and axes as `hand.rs`: hires px, x right, y forward (up the screen), z toward the viewer.
//! Light belongs to the player: enemy hulls are void/dark with `light` only on sunny facets; `hot` is the
//! Lattice (light blue in S2).
//!
//! Frame sets (the order the level image stores them, `roster`):
//! - PIKE: 16 yaws. KESTREL: 16 yaws × 3 rolls (yaw-major).
//! - ROOF HATCH: hatch open 0..5, then blown.
//! - BELL TURRET: 16 headings, then the same 16 armed. The square collar stays put; only the bell and its
//!   barrels turn.
//! - DOME MORTAR: the slit opening over 4 poses facing down the screen, then the open dome at 16 headings.
//! - CAMPANILE: crown 8 spins over 45° (8 points), bell hub 4 opening poses.
//! - ASTROLABE: body 8 shutter poses; outer, middle, inner ring 16 spins each over 60° (6 notches); lens
//!   4 pulses.

use std::f64::consts::{PI, TAU};

use crate::encode::SpriteImage;
use crate::mesh::ModelFile;
use crate::raster3d::{self, Material, Mesh, Orient, V3, View};
use crate::s2::SUN;

const VOID: u8 = 0;
const DARK: u8 = 1;
const LIGHT: u8 = 2;
const HOT: u8 = 3;

/// Poses and samples (the bible's `orient`).
pub const YAWS: usize = 16;
pub const KESTREL_ROLLS: usize = 3;
pub const ROOF_HATCH_POSES: usize = 6;
pub const MORTAR_OPEN: usize = 4;
pub const CROWN_SPINS: usize = 8;
pub const HUB_POSES: usize = 4;
pub const SHUTTER_POSES: usize = 8;
pub const RING_SPINS: usize = 16;
pub const LENS_PULSES: usize = 4;
/// The small dome a DOME MORTAR replaces (the level's mortar domes are built at this radius).
pub const MORTAR_R: f64 = 18.0;
/// The ASTROLABE's dome (the level's `GREAT_R`) and how far each shutter half rolls out when open.
pub const ASTRO_R: f64 = 104.0;
pub const SHUTTER_TRAVEL: f64 = 44.0;
/// The rings' radii (inner edge, outer edge), outer to inner.
pub const RINGS: [(f64, f64); 3] = [(96.0, 110.0), (66.0, 78.0), (38.0, 48.0)];
const RING_CANVAS: [(u16, u16); 3] = [(120, 240), (88, 176), (56, 112)];

fn model(name: &str, canvas: (u16, u16), orient: &[(&str, f64)], mesh: Mesh) -> ModelFile {
    ModelFile {
        name: name.to_string(),
        canvas,
        orient: orient.iter().map(|&(k, v)| (k.to_string(), v)).collect(),
        mesh,
    }
}

/// One frame of `mesh` on a `canvas`, under S2's sun.
fn shot(mesh: &Mesh, canvas: (u16, u16), o: Orient) -> SpriteImage {
    raster3d::render_lit(mesh, o, View::centered(canvas.0, canvas.1, 1.0), &SUN)
}

/// Turn every vertex of `m` from index `from` on by `yaw` about the vertical axis (the same sense as
/// `Orient::yaw`), leaving the rest (a fixed base) where it is.
fn turn_from(m: &mut Mesh, from: usize, yaw: f64) {
    let o = Orient { yaw, pitch: 0.0, roll: 0.0 };
    for v in m.verts[from..].iter_mut() {
        *v = o.apply(*v);
    }
}

fn yaw_of(i: usize) -> f64 {
    i as f64 * TAU / YAWS as f64
}

// ------------------------------------------------------------------------------------------- PIKE

/// PIKE: a crystal needle clamped to a guide wire: a long faceted diamond body, a fin ridge down its back,
/// swept tail vanes, two skids. 26 px long; yaws to its rail and never banks.
pub fn pike() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, HOT], 0.8, 1),
        Material::lit(&[DARK, HOT], 0.9, 0),
        Material::lit(&[VOID, DARK], 0.85, 0),
        Material::glow(HOT),
    ]);
    let (nose, tail) = ([0.0, 13.0, 2.5], [0.0, -12.0, 2.5]);
    let (r, t, l, b) = ([3.5, 3.0, 2.5], [0.0, 3.0, 6.0], [-3.5, 3.0, 2.5], [0.0, 3.0, -0.5]);
    for (p, q) in [(r, t), (t, l), (l, b), (b, r)] {
        m.tri(nose, p, q, 0);
        m.tri(tail, q, p, 0);
    }
    m.beam([0.0, 5.0, 5.6], [0.0, -9.0, 4.4], 1.4, 1.4, 1);
    m.sym(&[[1.0, -5.0, 3.2], [5.5, -11.0, 2.0], [1.0, -10.0, 3.0]], 1);
    for sx in [-1.0, 1.0] {
        m.beam([sx * 4.6, 6.0, 0.4], [sx * 4.6, -7.0, 0.4], 1.0, 0.8, 2);
    }
    m.poly(&[[0.0, 10.0, 3.9], [0.9, 8.0, 4.4], [0.0, 6.0, 5.0], [-0.9, 8.0, 4.4]], 3);
    model("enemy.s2.pike", (16, 32), &[("yaw", 16.0)], m)
}

// ---------------------------------------------------------------------------------------- KESTREL

/// KESTREL: a captured Accord jet: a blunt nose, straight wide wings (48 px span) in two facets, a
/// tailplane, twin fins canted outward, twin nozzles, crystal through the canopy. Armour mostly dark: the
/// cream PEREGRINE stays the only light ship.
pub fn kestrel() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 0.55, 1),
        Material::lit(&[VOID, DARK], 0.85, 1),
        Material::lit(&[DARK, HOT], 1.0, 0),
        Material::glow(HOT),
    ]);
    let nose = [0.0, 21.0, 2.5];
    let (c0, c1, c2, tail) = ([0.0, 14.0, 5.0], [0.0, 2.0, 6.0], [0.0, -14.0, 5.5], [0.0, -19.0, 4.0]);
    let (s0, s1, s2, s3) = ([2.5, 15.0, 2.5], [4.5, 4.0, 3.0], [4.5, -12.0, 3.0], [3.5, -19.0, 2.5]);
    m.sym(&[nose, s0, c0], 0);
    m.sym(&[c0, s0, s1, c1], 0);
    m.sym(&[c1, s1, s2, c2], 0);
    m.sym(&[c2, s2, s3, tail], 0);
    m.sym(&[nose, [0.0, 14.0, -1.0], s0], 1);
    m.sym(&[[0.0, 14.0, -1.0], [0.0, -19.0, -1.0], s3, s2, s1, s0], 1);
    // Canopy, with the crystal grown through it.
    m.poly(&[[0.0, 13.0, 5.4], [1.6, 8.0, 6.0], [0.0, 3.0, 6.4], [-1.6, 8.0, 6.0]], 2);
    m.shard([0.0, 0.0, 6.0], [0.0, -5.0, 9.5], 1.6, 2);
    // Straight wings: a forward facet and an aft facet meeting along a spar line, slight dihedral.
    m.sym(&[[4.5, 3.0, 3.0], [23.0, 0.0, 2.0], [23.0, -2.5, 2.5], [4.5, -3.0, 3.8]], 0);
    m.sym(&[[4.5, -3.0, 3.8], [23.0, -2.5, 2.5], [23.0, -5.0, 2.0], [4.5, -9.0, 3.0]], 0);
    // Intakes beside the cockpit.
    for sx in [-1.0, 1.0] {
        let (a, b) = if sx < 0.0 { (-7.0, -4.0) } else { (4.0, 7.0) };
        m.frustum([a, -2.0, 1.0], [b, 7.0, 1.0], 0.6, 4.5, 1);
    }
    // Tailplane and twin fins (canted out, so they show from above).
    m.sym(&[[3.5, -14.0, 3.0], [10.0, -17.0, 2.5], [10.0, -20.0, 2.5], [3.0, -19.0, 3.0]], 1);
    for sx in [-1.0, 1.0] {
        m.poly(&[[sx * 3.5, -11.0, 5.0], [sx * 3.5, -19.0, 4.0], [sx * 6.5, -20.0, 10.0], [sx * 6.0, -15.0, 10.0]], 0);
        m.tube([sx * 1.8, -18.0, 3.0], [sx * 1.8, -21.0, 3.0], 1.6, 6, 1, 3);
    }
    model("enemy.common.kestrel", (32, 52), &[("yaw", 16.0), ("roll", 3.0), ("roll_max_deg", 40.0)], m)
}

// ------------------------------------------------------------------------------------- ROOF HATCH

/// ROOF HATCH pose `k`: a farmhouse's hipped clay roof (42 × 32, the same tiles as the level's) with a
/// hatch in its ridge: a curb, two tiled leaves sliding apart along the ridge (`k` of
/// [`ROOF_HATCH_POSES`]) over a shaft with a glowing crystal nest. `k == ROOF_HATCH_POSES`: blown: the
/// tiles gone, broken rafters over the dark house, the nest bare and dead.
pub fn roof_hatch(k: usize) -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK], 0.8, 1),
        Material::lit(&[VOID, DARK, LIGHT], 0.6, 1),
        Material::lit(&[DARK, HOT], 1.0, 0),
        Material::glow(HOT),
        Material::glow(VOID),
        Material::lit(&[VOID, DARK], 0.5, 0),
    ]);
    let (hw, hl, h) = (21.0, 16.0, 10.0);
    let e = [[-hw, -hl, 0.0], [hw, -hl, 0.0], [hw, hl, 0.0], [-hw, hl, 0.0]];
    let (r0, r1) = ([-hw + hl, 0.0, h], [hw - hl, 0.0, h]);
    let name = if k < ROOF_HATCH_POSES { format!("enemy.s2.roof_hatch.open{k}") } else { "enemy.s2.roof_hatch.blown".into() };
    if k >= ROOF_HATCH_POSES {
        // The house's dark interior, its wall tops, a few tiles left at the corners, broken rafters, and
        // the nest bare (dead crystal).
        m.poly(&[[-hw + 2.0, -hl + 2.0, 0.5], [hw - 2.0, -hl + 2.0, 0.5], [hw - 2.0, hl - 2.0, 0.5], [-hw + 2.0, hl - 2.0, 0.5]], 4);
        for (a, b) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
            m.beam(e[a], e[b], 2.5, 2.0, 5);
        }
        m.tri(e[0], [-hw + 9.0, -hl, 0.0], [-hw + 5.0, -hl + 5.0, 3.0], 0);
        m.tri(e[2], [hw - 10.0, hl, 0.0], [hw - 5.0, hl - 5.0, 3.0], 0);
        m.tri(e[1], [hw, -hl + 8.0, 0.0], [hw - 4.0, -hl + 4.0, 2.5], 0);
        m.beam(e[3], [-8.0, 6.0, 6.0], 1.6, 1.4, 1);
        m.beam(e[1], [9.0, -5.0, 6.5], 1.6, 1.4, 1);
        m.beam([-3.0, 0.0, 7.0], [4.0, 0.0, 6.0], 1.6, 1.4, 1);
        m.beam(e[0], [-12.0, -7.0, 4.0], 1.6, 1.4, 1);
        let dead = 5;
        for (x, y, tx, ty) in [(0.0, 0.0, 1.0, 2.0), (-3.0, 2.0, -6.0, 4.0), (3.0, -2.0, 6.0, -3.0)] {
            m.shard([x, y, 0.5], [tx, ty, 6.0], 2.0, dead);
        }
        return model(&name, (24, 40), &[], m);
    }
    // The hipped roof: ridge along x.
    m.poly(&[e[0], e[1], r1, r0], 0);
    m.poly(&[e[2], e[3], r0, r1], 0);
    m.tri(e[3], e[0], r0, 0);
    m.tri(e[1], e[2], r1, 0);
    // Ridge and hip beams.
    m.beam([r0[0], 0.0, h + 0.3], [r1[0], 0.0, h + 0.3], 1.6, 1.0, 1);
    for (c, r) in [(e[0], r0), (e[3], r0), (e[1], r1), (e[2], r1)] {
        m.beam([c[0], c[1], c[2] + 0.3], [r[0], r[1], r[2] + 0.3], 1.2, 0.8, 1);
    }
    // The hatch: a curb straddling the ridge, the shaft, the nest.
    let (cx, cy, top) = (7.0, 5.0, h + 2.5);
    m.cuboid([-cx - 1.5, -cy - 1.5, 4.0], [cx + 1.5, -cy, top], 1);
    m.cuboid([-cx - 1.5, cy, 4.0], [cx + 1.5, cy + 1.5, top], 1);
    m.cuboid([-cx - 1.5, -cy, 4.0], [-cx, cy, top], 1);
    m.cuboid([cx, -cy, 4.0], [cx + 1.5, cy, top], 1);
    m.poly(&[[-cx, -cy, h - 1.0], [cx, -cy, h - 1.0], [cx, cy, h - 1.0], [-cx, cy, h - 1.0]], 4);
    for (x, y, tx, ty) in [(0.0, 0.0, 0.5, 1.0), (-3.5, 1.5, -5.0, 3.0), (3.5, -1.5, 5.0, -3.0), (3.0, 2.0, 4.5, 3.5), (-3.0, -2.0, -4.5, -3.5)] {
        m.shard([x, y, h - 1.0], [tx, ty, h + 1.8], 2.0, 2);
    }
    m.poly(&Mesh::ring(6, 1.6, h + 1.9, 0.0), 3);
    // The leaves: tiled, a beam along each leading edge; they slide out along the ridge over the tiles.
    let dx = 8.0 * k as f64 / (ROOF_HATCH_POSES - 1) as f64;
    for s in [-1.0, 1.0] {
        let (inner, outer) = (s * dx, s * (cx + dx));
        let (lo, hi) = if s < 0.0 { (outer, inner) } else { (inner, outer) };
        m.cuboid([lo, -cy - 0.5, top], [hi, cy + 0.5, top + 0.6], 0);
        let (bl, bh) = if s < 0.0 { (inner - 1.2, inner) } else { (inner, inner + 1.2) };
        m.cuboid([bl, -cy - 0.5, top + 0.6], [bh, cy + 0.5, top + 1.0], 1);
    }
    model(&name, (24, 40), &[], m)
}

// ------------------------------------------------------------------------------------ BELL TURRET

/// BELL TURRET: a bronze bell cupola on a square stone collar, three crystal barrels in a fan; `armed`:
/// the barrel tips glow `hot` (the steady telegraph). The collar stays square to the roof; the bell and
/// barrels turn by `yaw`.
pub fn bell_turret_mesh(yaw: f64, armed: bool) -> Mesh {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 0.6, 1),
        Material::lit(&[VOID, DARK], 0.85, 1),
        Material::lit(&[DARK, HOT], 0.55, 0),
        Material::glow(HOT),
    ]);
    m.frustum([-15.0, -15.0, 0.0], [15.0, 15.0, 0.0], 3.0, 5.0, 1);
    let from = m.verts.len();
    let profile = [(11.0, 5.0), (11.0, 8.0), (9.0, 12.0), (6.0, 14.5), (2.5, 16.0)];
    for w in profile.windows(2) {
        m.loft(&Mesh::ring(16, w[0].0, w[0].1, 0.0), &Mesh::ring(16, w[1].0, w[1].1, 0.0), 0);
    }
    m.poly(&Mesh::ring(16, 2.5, 16.0, 0.0), 0);
    m.tube([0.0, 0.0, 16.0], [0.0, 0.0, 18.0], 1.5, 6, 0, 0);
    for a in [-0.45f64, 0.0, 0.45] {
        let (s, c) = a.sin_cos();
        let cap = if armed { 3 } else { 2 };
        m.tube([s * 7.0, c * 7.0, 9.0], [s * 18.0, c * 18.0, 9.5], 1.5, 6, 2, cap);
        if armed {
            m.tube([s * 14.5, c * 14.5, 9.6], [s * 18.6, c * 18.6, 9.8], 2.1, 6, 3, 3);
        }
    }
    turn_from(&mut m, from, yaw);
    m
}

pub fn bell_turret() -> ModelFile {
    model("enemy.s2.bell_turret", (22, 44), &[], bell_turret_mesh(0.0, false))
}

// ------------------------------------------------------------------------------------ DOME MORTAR

/// DOME MORTAR: a small observatory dome (radius [`MORTAR_R`], on its drum) whose shutter slit, running
/// from behind the crown down its front, opens over `open` (0..[`MORTAR_OPEN`]); from the third pose a
/// stubby crystal tube shows in the slit. Built facing +y (the level's domes are 16-sided, so a turn by a
/// sixteenth leaves the dome itself unchanged: only the slit and the tube move).
pub fn dome_mortar_mesh(open: usize) -> Mesh {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 0.56, 1),
        Material::lit(&[VOID, DARK], 0.85, 0),
        Material::lit(&[DARK, HOT], 0.8, 0),
        Material::glow(VOID),
        Material::glow(HOT),
        Material::lit(&[VOID, DARK], 0.85, 0),
    ]);
    let (r, z) = (MORTAR_R, 4.0);
    let rings = 5;
    let ring = |k: usize| -> Vec<V3> {
        let a = k as f64 / rings as f64 * PI / 2.0;
        Mesh::ring(16, r * a.cos(), z + r * a.sin(), TAU / 32.0)
    };
    for k in 0..rings {
        if k + 1 == rings {
            m.poly(&ring(k), 0);
        } else {
            m.loft(&ring(k), &ring(k + 1), 0);
        }
    }
    m.loft(&Mesh::ring(16, r + 1.0, 0.0, TAU / 32.0), &ring(0), 5);
    // The slit: the opening (dark) between two shutter-leaf edges, laid over the dome's profile from just
    // behind the crown (100 degrees) to the front (8 degrees).
    let w = [1.2, 3.5, 6.0, 8.0][open.min(MORTAR_OPEN - 1)];
    let steps = 7;
    let at = |a: f64, x: f64, lift: f64| -> V3 { [x, r * a.cos(), z + r * a.sin() + lift] };
    for k in 0..steps {
        let (a0, a1) = (
            (100.0 - k as f64 * 92.0 / steps as f64).to_radians(),
            (100.0 - (k + 1) as f64 * 92.0 / steps as f64).to_radians(),
        );
        m.poly(&[at(a0, -w / 2.0, 0.3), at(a0, w / 2.0, 0.3), at(a1, w / 2.0, 0.3), at(a1, -w / 2.0, 0.3)], 3);
        for s in [-1.0, 1.0] {
            let (i, o) = (s * w / 2.0, s * (w / 2.0 + 1.3));
            let (lo, hi) = if s < 0.0 { (o, i) } else { (i, o) };
            m.poly(&[at(a0, lo, 0.5), at(a0, hi, 0.5), at(a1, hi, 0.5), at(a1, lo, 0.5)], 1);
        }
    }
    if open >= 2 {
        let len = if open == 2 { 6.0 } else { 12.0 };
        let (p0, d) = ([0.0, 3.0, z + r * 0.75], [0.0, 0.92, 0.39]);
        let p1 = [0.0, p0[1] + d[1] * len, p0[2] + d[2] * len];
        m.tube(p0, p1, 2.6, 6, 2, 4);
    }
    m
}

pub fn dome_mortar() -> ModelFile {
    model("enemy.s2.dome_mortar", (24, 48), &[("yaw", 16.0)], dome_mortar_mesh(MORTAR_OPEN - 1))
}

// -------------------------------------------------------------------------------------- CAMPANILE

/// The CAMPANILE's crown: the tower's top gone to crystal: an octagonal stone rim with old tiles, eight
/// long crystal points and eight short ones between them. 8-fold symmetric: 8 spins over 45 degrees turn
/// it smoothly. The middle is open: the bell hub (its own sprite) sits there.
pub fn campanile_crown() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK], 0.85, 1),
        Material::lit(&[VOID, DARK], 0.7, 1),
        Material::lit(&[DARK, HOT], 1.0, 0),
        Material::glow(HOT),
    ]);
    let oct = PI / 8.0;
    m.loft(&Mesh::ring(8, 23.0, 0.0, oct), &Mesh::ring(8, 19.0, 5.0, oct), 0);
    m.loft(&Mesh::ring(8, 19.0, 5.0, oct), &Mesh::ring(8, 12.0, 5.0, oct), 1);
    m.loft(&Mesh::ring(8, 12.0, 5.0, oct), &Mesh::ring(8, 11.0, 2.0, oct), 0);
    for k in 0..8 {
        let a = k as f64 * PI / 4.0;
        let (c, s) = (a.cos(), a.sin());
        m.shard([17.0 * c, 17.0 * s, 5.0], [56.0 * c, 56.0 * s, 13.0], 7.0, 2);
        let b = a + PI / 8.0;
        let (c2, s2) = (b.cos(), b.sin());
        m.shard([19.0 * c2, 19.0 * s2, 5.0], [34.0 * c2, 34.0 * s2, 9.0], 4.0, 2);
        // A glint on each long point.
        let g = [44.0 * c, 44.0 * s, 13.5];
        m.poly(&[[g[0] - 1.2 * s, g[1] + 1.2 * c, g[2]], [g[0] + 1.5 * c, g[1] + 1.5 * s, g[2]], [g[0] + 1.2 * s, g[1] - 1.2 * c, g[2]], [g[0] - 1.5 * c, g[1] - 1.5 * s, g[2]]], 3);
    }
    model("midboss.s2.campanile.crown", (64, 128), &[("yaw", CROWN_SPINS as f64), ("yaw_span_deg", 45.0)], m)
}

/// The CAMPANILE's bell hub pose `k` of [`HUB_POSES`]: an iron bell seen from above in four quarter
/// petals; opening, the petals part and lean out over a `hot` core.
pub fn campanile_hub(k: usize) -> ModelFile {
    let mut m = Mesh::new(vec![Material::lit(&[VOID, DARK, HOT], 0.6, 1), Material::glow(HOT), Material::glow(VOID)]);
    m.poly(&Mesh::ring(12, 10.0, 0.5, 0.0), 2);
    m.poly(&Mesh::ring(8, 2.5 + 1.2 * k as f64, 2.0, 0.0), 1);
    let d = 2.6 * k as f64;
    for q in 0..4 {
        let mid = (q as f64 + 0.5) * PI / 2.0;
        let (ox, oy) = (d * mid.cos(), d * mid.sin());
        let apex = [ox, oy, 14.0 - 1.5 * k as f64];
        let n = 4;
        let pt = |i: usize, r: f64, z: f64| -> V3 {
            let a = q as f64 * PI / 2.0 + i as f64 / n as f64 * PI / 2.0;
            [ox + r * a.cos(), oy + r * a.sin(), z]
        };
        for i in 0..n {
            m.poly(&[pt(i, 12.0, 2.0), pt(i + 1, 12.0, 2.0), pt(i + 1, 9.0, 9.0), pt(i, 9.0, 9.0)], 0);
            m.tri(pt(i, 9.0, 9.0), pt(i + 1, 9.0, 9.0), apex, 0);
        }
    }
    model(&format!("midboss.s2.campanile.bell_hub.open{k}"), (24, 48), &[], m)
}

// -------------------------------------------------------------------------------------- ASTROLABE

/// The ASTROLABE's body at shutter pose `k` of [`SHUTTER_POSES`]: the great dome split down the middle
/// into two shutter halves that roll apart over a dark pit with the rings' bearing in it.
pub fn astrolabe_body(k: usize) -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 0.56, 1),
        Material::lit(&[VOID, DARK], 0.85, 0),
        Material::lit(&[VOID, DARK, HOT], 0.7, 1),
        Material::glow(VOID),
        Material::glow(HOT),
    ]);
    let r = ASTRO_R;
    m.poly(&Mesh::ring(32, r - 2.0, 0.5, 0.0), 3);
    m.loft(&Mesh::ring(32, 58.0, 2.0, 0.0), &Mesh::ring(32, 50.0, 3.0, 0.0), 2);
    m.loft(&Mesh::ring(32, r + 1.0, -2.0, 0.0), &Mesh::ring(32, r, 4.0, 0.0), 1);
    for i in 0..6 {
        let a = i as f64 * PI / 3.0;
        m.poly(&[[54.0 * a.cos(), 54.0 * a.sin(), 3.5], [56.0 * (a + 0.04).cos(), 56.0 * (a + 0.04).sin(), 3.5], [52.0 * (a + 0.04).cos(), 52.0 * (a + 0.04).sin(), 3.5]], 4);
    }
    let dx = SHUTTER_TRAVEL * k as f64 / (SHUTTER_POSES - 1) as f64;
    let (rings, segs) = (6usize, 16usize);
    for s in [-1.0, 1.0] {
        // A half: longitudes from -90 to 90 degrees about the +x (or -x) axis.
        let pt = |lat: usize, lon: usize| -> V3 {
            let e = lat as f64 / rings as f64 * PI / 2.0;
            let a = -PI / 2.0 + lon as f64 / segs as f64 * PI;
            let rr = r * e.cos();
            [s * (rr * a.cos()) + s * dx, rr * a.sin(), 4.0 + r * e.sin() * 0.62]
        };
        for lat in 0..rings {
            for lon in 0..segs {
                if lat + 1 == rings {
                    m.tri(pt(lat, lon), pt(lat, lon + 1), pt(lat + 1, 0), 0);
                } else {
                    m.poly(&[pt(lat, lon), pt(lat, lon + 1), pt(lat + 1, lon + 1), pt(lat + 1, lon)], 0);
                }
            }
        }
        // The leaf's cut edge: a rib down the seam.
        let seam: Vec<V3> = (0..=rings * 2)
            .map(|i| {
                let e = (i as f64 / (rings * 2) as f64) * PI - PI / 2.0;
                [s * dx + s * 1.5, r * e.sin(), 4.0 + r * e.cos() * 0.62 + 0.6]
            })
            .collect();
        for w in seam.windows(2) {
            m.beam(w[0], w[1], 3.0, 1.2, 1);
        }
    }
    model(&format!("boss.s2.astrolabe.open{k}"), (160, 256), &[], m)
}

/// One of the ASTROLABE's rings (`i`: 0 outer, 1 middle, 2 inner): a bevelled band broken by six notches
/// 60 degrees apart, a glowing crystal channel along its top, a crystal stud mid-way between each pair. 6-fold symmetric: 16 spins over 60
/// degrees.
pub fn astrolabe_ring(i: usize) -> ModelFile {
    let mut m = Mesh::new(vec![Material::lit(&[VOID, DARK, HOT], 0.5, 1), Material::lit(&[DARK, HOT], 1.0, 0), Material::glow(HOT)]);
    let (ri, ro) = RINGS[i];
    let notch = (10.0f64).to_radians();
    let at = |a: f64, r: f64, z: f64| -> V3 { [r * a.cos(), r * a.sin(), z] };
    for s in 0..6 {
        let (a0, a1) = (s as f64 * PI / 3.0 + notch / 2.0, (s + 1) as f64 * PI / 3.0 - notch / 2.0);
        let n = ((a1 - a0) * ro / 9.0).ceil().max(3.0) as usize;
        for k in 0..n {
            let (b0, b1) = (a0 + (a1 - a0) * k as f64 / n as f64, a0 + (a1 - a0) * (k + 1) as f64 / n as f64);
            m.poly(&[at(b0, ri + 2.0, 6.0), at(b1, ri + 2.0, 6.0), at(b1, ro - 2.0, 6.0), at(b0, ro - 2.0, 6.0)], 0);
            m.poly(&[at(b0, ro - 2.0, 6.0), at(b1, ro - 2.0, 6.0), at(b1, ro, 2.0), at(b0, ro, 2.0)], 0);
            m.poly(&[at(b0, ri, 2.0), at(b1, ri, 2.0), at(b1, ri + 2.0, 6.0), at(b0, ri + 2.0, 6.0)], 0);
            // The crystal channel inlaid along the band: it outlines the ring over the dome and breaks
            // at every notch (the gaps lining up are the telegraph).
            let rm = (ri + ro) / 2.0;
            m.poly(&[at(b0, rm - 1.2, 6.2), at(b1, rm - 1.2, 6.2), at(b1, rm + 1.2, 6.2), at(b0, rm + 1.2, 6.2)], 2);
        }
        // The notch ends.
        for a in [a0, a1] {
            m.poly(&[at(a, ri, 2.0), at(a, ro, 2.0), at(a, ro - 2.0, 6.0), at(a, ri + 2.0, 6.0)], 0);
        }
        let mid = (a0 + a1) / 2.0;
        let rm = (ri + ro) / 2.0;
        m.shard(at(mid, rm, 6.0), at(mid + 0.02, rm + 1.0, 10.0 + (ro - ri) * 0.2), (ro - ri) * 0.32, 1);
    }
    let names = ["ring_outer", "ring_middle", "ring_inner"];
    model(&format!("boss.s2.astrolabe.{}", names[i]), RING_CANVAS[i], &[("yaw", RING_SPINS as f64), ("yaw_span_deg", 60.0)], m)
}

/// The ASTROLABE's lens at pulse `k` of [`LENS_PULSES`]: a bevelled rim round a glass boss, its white-hot
/// core growing.
pub fn astrolabe_lens(k: usize) -> ModelFile {
    let mut m = Mesh::new(vec![Material::lit(&[VOID, DARK, HOT], 0.7, 1), Material::glow(HOT), Material::lit(&[DARK, HOT], 0.9, 0)]);
    m.loft(&Mesh::ring(12, 20.0, 0.0, 0.0), &Mesh::ring(12, 17.0, 5.0, 0.0), 0);
    m.loft(&Mesh::ring(12, 17.0, 5.0, 0.0), &Mesh::ring(12, 13.0, 5.0, 0.0), 0);
    let rim = Mesh::ring(12, 13.0, 5.0, 0.0);
    for i in 0..12 {
        m.tri(rim[i], rim[(i + 1) % 12], [0.0, 0.0, 9.0], 2);
    }
    m.poly(&Mesh::ring(8, 3.0 + 2.0 * k as f64, 9.5, 0.0), 1);
    model(&format!("boss.s2.astrolabe.lens.pulse{k}"), (24, 48), &[], m)
}

// ------------------------------------------------------------------------------------------ sets

/// Every S2 hand model's base pose (for `hand-models`: `assets/models/<id>.mesh`).
pub fn all() -> Vec<(String, ModelFile)> {
    let mut out: Vec<ModelFile> = vec![pike(), kestrel(), bell_turret(), dome_mortar(), campanile_crown()];
    out.extend((0..=ROOF_HATCH_POSES).map(roof_hatch));
    out.extend((0..HUB_POSES).map(campanile_hub));
    out.extend((0..SHUTTER_POSES).map(astrolabe_body));
    out.extend((0..3).map(astrolabe_ring));
    out.extend((0..LENS_PULSES).map(astrolabe_lens));
    out.into_iter().map(|m| (m.name.clone(), m)).collect()
}

/// Every S2 frame set in the level image's order, rendered under S2's sun: (name, frames).
pub fn roster() -> Vec<(&'static str, Vec<SpriteImage>)> {
    let mut out: Vec<(&'static str, Vec<SpriteImage>)> = Vec::new();
    out.push(("pike", pike().render_all_lit(&SUN)));
    out.push(("kestrel", kestrel().render_all_lit(&SUN)));
    out.push(("roof_hatch", (0..=ROOF_HATCH_POSES).map(|k| roof_hatch(k).render_all_lit(&SUN).remove(0)).collect()));
    let bell: Vec<SpriteImage> =
        [false, true].iter().flat_map(|&armed| (0..YAWS).map(move |i| shot(&bell_turret_mesh(yaw_of(i), armed), (22, 44), Orient::default()))).collect();
    out.push(("bell_turret", bell));
    let down = Orient { yaw: PI, pitch: 0.0, roll: 0.0 };
    let mut mortar: Vec<SpriteImage> = (0..MORTAR_OPEN).map(|k| shot(&dome_mortar_mesh(k), (24, 48), down)).collect();
    mortar.extend(dome_mortar().render_all_lit(&SUN));
    out.push(("dome_mortar", mortar));
    out.push(("wasp", crate::hand::wasp().render_all_lit(&SUN)));
    let mut pill = crate::hand::pillbox().render_all_lit(&SUN);
    pill.extend(crate::hand::pillbox_armed(true).render_all_lit(&SUN));
    out.push(("pillbox", pill));
    out.push(("crown", campanile_crown().render_all_lit(&SUN)));
    out.push(("hub", (0..HUB_POSES).map(|k| campanile_hub(k).render_all_lit(&SUN).remove(0)).collect()));
    out.push(("astrolabe", (0..SHUTTER_POSES).map(|k| astrolabe_body(k).render_all_lit(&SUN).remove(0)).collect()));
    for (i, name) in ["ring_outer", "ring_middle", "ring_inner"].into_iter().enumerate() {
        out.push((name, astrolabe_ring(i).render_all_lit(&SUN)));
    }
    out.push(("lens", (0..LENS_PULSES).map(|k| astrolabe_lens(k).render_all_lit(&SUN).remove(0)).collect()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every S2 frame keeps clear of its canvas edge and draws something; the frame counts are the
    /// bible's `orient`.
    #[test]
    fn the_s2_roster_fits_its_canvases() {
        let mut bad = Vec::new();
        for (name, frames) in roster() {
            for (i, img) in frames.iter().enumerate() {
                let (w, h) = (img.w, img.h);
                let edge = (0..w).any(|x| img.get(x, 0).is_some() || img.get(x, h - 1).is_some())
                    || (0..h).any(|y| img.get(0, y).is_some() || img.get(w - 1, y).is_some());
                // S1's armed PILLBOX (approved, unchanged) grazes its edge with the muzzle glow when
                // it points sideways, in S1 as here.
                let s1_glow = name == "pillbox" && (i == 20 || i == 28);
                if (edge && !s1_glow) || img.pixels.iter().all(|p| p.is_none()) {
                    bad.push(format!("{name}#{i}"));
                }
            }
        }
        assert!(bad.is_empty(), "clipped or empty: {bad:?}");
        let n: Vec<(&str, usize)> = roster().iter().map(|(k, f)| (*k, f.len())).collect();
        assert_eq!(
            n,
            [
                ("pike", 16), ("kestrel", 48), ("roof_hatch", 7), ("bell_turret", 32), ("dome_mortar", 20), ("wasp", 48),
                ("pillbox", 32), ("crown", 8), ("hub", 4), ("astrolabe", 8), ("ring_outer", 16), ("ring_middle", 16),
                ("ring_inner", 16), ("lens", 4)
            ]
        );
    }

    /// Light belongs to the player: no S2 enemy is more than 15 % `light` (the bible's highlight budget).
    #[test]
    fn s2_enemies_keep_light_for_highlights() {
        for (name, frames) in roster() {
            let (mut l, mut all) = (0usize, 0usize);
            for img in &frames {
                for p in img.pixels.iter().flatten() {
                    all += 1;
                    l += (*p == LIGHT) as usize;
                }
            }
            let share = l as f64 / all.max(1) as f64;
            assert!(share <= 0.15, "{name}: {:.1} % light", share * 100.0);
        }
    }
}
