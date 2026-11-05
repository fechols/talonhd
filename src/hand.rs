//! Hand-built models (host only): geometry authored here by the art director, for entities whose bible
//! `[model]` block says `ludo.generator = "hand"`. Low-poly by design and shaped for the straight-down
//! camera: what matters is the silhouette from above and facets that catch the one key light.
//!
//! Units: hires pixels. Axes: x right, y forward (up the screen), z toward the viewer.
//! `talonhd hand-models` writes each one to `assets/models/<id>.mesh`; canvas and orientation mirror
//! the bible block (`docs/story/hd/models/*.md`).

use std::collections::BTreeMap;
use std::f64::consts::PI;

use crate::mesh::ModelFile;
use crate::raster3d::{Material, Mesh, V3};

const VOID: u8 = 0;
const DARK: u8 = 1;
const LIGHT: u8 = 2;
const HOT: u8 = 3;

/// The GANTRY QUEEN's on-screen scale (art direction review, 2026-10-31).
pub const QUEEN_SCALE: f64 = 0.6;

/// Every hand model: (file id, model). Pose variants carry a `.<pose>` suffix on the bible id.
pub fn all() -> Vec<(String, ModelFile)> {
    let mut out: Vec<(String, ModelFile)> = vec![
        ("boss.s1.gantry_queen".into(), gantry_queen()),
        ("enemy.common.wasp".into(), wasp()),
        ("enemy.s1.gannet".into(), gannet()),
        ("enemy.s1.skiff".into(), skiff()),
        ("enemy.common.pillbox".into(), pillbox()),
        ("enemy.common.crawler".into(), crawler()),
        ("midboss.s1.keelbreaker".into(), keelbreaker()),
        ("midboss.s1.keelbreaker.turret".into(), keelbreaker_turret()),
        ("hd.ship.talon".into(), talon()),
        ("hd.ship.peregrine".into(), peregrine()),
        ("hd.ship.caracara".into(), caracara()),
    ];
    for k in 0..HATCH_POSES {
        out.push((format!("enemy.common.hatch.open{k}"), hatch(k)));
    }
    out
}

// Materials shared by the roster. Enemies are dark and hot; light is only for highlights.
// A main hull at half albedo on void-dark-light: flat tops solid dark, light-facing slopes catch a
// light checker, only faces turned away fall toward void (a void checker over the sea reads as a hole).
fn hull() -> Material {
    Material::lit(&[VOID, DARK, LIGHT], 0.6, 2)
}
/// Secondary parts (tracks, doors, pods, decks): solid dark on top, darker on the shade side.
fn hull_dark() -> Material {
    Material::lit(&[VOID, DARK], 0.85, 1)
}
fn crystal() -> Material {
    Material::lit(&[DARK, HOT], 1.0, 0)
}
fn weapon() -> Material {
    Material::lit(&[VOID, DARK, HOT], 0.75, 1)
}

/// WASP: a small faceted crystal dart. Pure Lattice: hot facets over dark, a glowing hexagonal core,
/// two swept vanes and a notched tail. 22 px long, about as wide.
pub fn wasp() -> ModelFile {
    let mut m = Mesh::new(vec![Material::lit(&[VOID, DARK, HOT], 0.85, 1), hull_dark(), Material::glow(HOT)]);
    let (n, r, t, l) = ([0.0, 11.0, 1.5], [5.0, -1.0, 1.0], [0.0, -6.0, 1.0], [-5.0, -1.0, 1.0]);
    let (a, b) = ([0.0, 1.0, 5.0], [0.0, 1.0, -2.0]);
    for (p, q) in [(n, r), (r, t), (t, l), (l, n)] {
        m.tri(a, p, q, 0);
        m.tri(b, q, p, 0);
    }
    m.sym(&[[4.0, -1.0, 1.0], [11.0, -11.0, 0.0], [3.0, -7.0, 1.0]], 1);
    m.sym(&[[1.0, -5.0, 1.0], [3.0, -11.0, 0.5], [0.0, -7.0, 1.0]], 1);
    let core: Vec<V3> = Mesh::ring(6, 2.2, 5.2, 0.0).into_iter().map(|p| [p[0], p[1] + 1.0, p[2]]).collect();
    m.poly(&core, 2);
    model("enemy.common.wasp", (16, 32), &[("yaw", 16.0), ("roll", 3.0), ("roll_max_deg", 40.0)], m)
}

/// GANNET: a captured attack drone with gull wings in a flattened M, kinked back at the elbows,
/// a short fat pod body, a fan tail, bomb pods under the elbows, crystal spikes at the wing roots and a
/// crystal eye. 60 px span, 40 long.
pub fn gannet() -> ModelFile {
    let mut m = Mesh::new(vec![hull(), crystal(), hull_dark(), Material::glow(HOT)]);
    m.tube([0.0, -8.0, 3.0], [0.0, 12.0, 3.0], 5.0, 6, 0, 0);
    m.shard([0.0, 12.0, 3.0], [0.0, 19.0, 2.0], 5.0, 0);
    // Inner and outer wing panels: up to the elbow, back down to the tip.
    m.sym(&[[4.0, 8.0, 4.0], [17.0, 0.0, 8.0], [17.0, -8.0, 7.0], [4.0, -4.0, 3.0]], 0);
    m.sym(&[[17.0, 0.0, 8.0], [30.0, 6.0, 2.0], [29.0, 1.0, 1.5], [17.0, -8.0, 7.0]], 0);
    m.sym(&[[0.0, -8.0, 3.0], [6.0, -17.0, 2.0], [0.0, -15.0, 2.5]], 0);
    for sx in [-1.0, 1.0] {
        m.tube([sx * 17.0, -2.0, 3.0], [sx * 17.0, -12.0, 3.0], 2.5, 6, 2, 2);
        m.shard([sx * 5.0, 4.0, 5.0], [sx * 9.0, 10.0, 10.0], 2.0, 1);
    }
    m.poly(&[[0.0, 16.0, 6.5], [1.5, 13.0, 6.8], [0.0, 11.0, 7.2], [-1.5, 13.0, 6.8]], 3);
    model("enemy.s1.gannet", (36, 72), &[("yaw", 16.0), ("pitch", 5.0), ("pitch_max_deg", 50.0)], m)
}

/// SKIFF: a captured harbour patrol boat: pointed bow, darker deck, a wheelhouse overgrown with crystal,
/// one deck gun forward. 52 px long, 16 wide (the wake is a separate VFX).
pub fn skiff() -> ModelFile {
    let mut m = Mesh::new(vec![hull(), hull_dark(), crystal(), weapon(), Material::glow(HOT)]);
    let deck = [
        [0.0, 26.0, 5.0], [6.0, 14.0, 5.0], [8.0, 0.0, 5.0], [7.0, -24.0, 5.0], [6.0, -26.0, 5.0],
        [-6.0, -26.0, 5.0], [-7.0, -24.0, 5.0], [-8.0, 0.0, 5.0], [-6.0, 14.0, 5.0],
    ];
    let water: Vec<V3> = deck.iter().map(|p| [p[0] * 0.75, p[1] * 0.94, -1.0]).collect();
    m.loft(&deck, &water, 0);
    // The deck rises to a centre ridge: each fan facet faces its own way and catches the light.
    let mut top = vec![[0.0, 0.0, 8.0]];
    top.extend_from_slice(&deck);
    top.push(deck[0]);
    m.poly(&top, 0);
    m.frustum([-5.0, -10.0, 7.0], [5.0, 2.0, 7.0], 1.5, 12.0, 1);
    m.shard([0.0, -4.0, 11.0], [2.0, -1.0, 16.0], 3.0, 2);
    m.shard([-2.0, -7.0, 11.0], [-4.0, -9.0, 15.0], 2.5, 2);
    m.tube([0.0, 14.0, 5.0], [0.0, 14.0, 8.0], 3.0, 6, 3, 3);
    m.tube([0.0, 15.0, 7.0], [0.0, 24.0, 7.0], 1.0, 5, 3, 4);
    model("enemy.s1.skiff", (28, 56), &[("yaw", 16.0)], m)
}

/// PILLBOX: a round concrete drum with a chamfered roof, a turret ring with twin barrels, and crystal
/// grown through the back of the roof. 40 px across.
pub fn pillbox() -> ModelFile {
    pillbox_armed(false)
}

/// The PILLBOX about to fire: its barrel tips glow `hot` (a steady telegraph, never a blink).
pub fn pillbox_armed(glow: bool) -> ModelFile {
    let mut m = Mesh::new(vec![hull(), hull_dark(), weapon(), crystal(), Material::glow(HOT)]);
    let rings = [(20.0, 0.0), (20.0, 5.0), (16.0, 9.0)];
    for w in rings.windows(2) {
        m.loft(&Mesh::ring(12, w[0].0, w[0].1, 0.0), &Mesh::ring(12, w[1].0, w[1].1, 0.0), 0);
    }
    m.loft(&Mesh::ring(12, 16.0, 9.0, 0.0), &Mesh::ring(12, 10.0, 9.0, 0.0), 0);
    m.loft(&Mesh::ring(12, 10.0, 9.0, 0.0), &Mesh::ring(12, 10.0, 12.0, 0.0), 1);
    // A shallow faceted dome on the turret.
    let mut dome = vec![[0.0, 0.0, 16.0]];
    let rim = Mesh::ring(12, 10.0, 12.0, 0.0);
    dome.extend_from_slice(&rim);
    dome.push(rim[0]);
    m.poly(&dome, 0);
    for x in [-3.0, 3.0] {
        m.tube([x, 2.0, 11.0], [x, 20.0, 11.0], 1.4, 5, 2, 4);
        if glow {
            m.tube([x, 15.0, 11.5], [x, 21.0, 11.5], 2.0, 5, 4, 4);
        }
    }
    m.shard([-5.0, -9.0, 9.0], [-8.0, -14.0, 15.0], 3.0, 3);
    m.shard([2.0, -11.0, 9.0], [4.0, -16.0, 14.0], 2.5, 3);
    m.shard([6.0, -7.0, 9.0], [10.0, -10.0, 13.0], 2.0, 3);
    model("enemy.common.pillbox", (22, 44), &[("yaw", 16.0)], m)
}

/// Door poses of the HATCH, closed to open.
pub const HATCH_POSES: usize = 6;

/// HATCH: a square armoured frame; two doors slide apart (pose `k` of [`HATCH_POSES`]) over a shaft with
/// a glowing crystal nest. Hazard strips on the doors' leading edges. 40 px square.
pub fn hatch(k: usize) -> ModelFile {
    let mut m = Mesh::new(vec![hull(), hull_dark(), Material::lit(&[DARK, LIGHT], 0.75, 0), crystal(), Material::glow(HOT)]);
    let sq = |h: f64, z: f64| -> Vec<V3> { vec![[-h, -h, z], [h, -h, z], [h, h, z], [-h, h, z]] };
    m.loft(&sq(20.0, 0.0), &sq(18.0, 4.0), 0);
    m.loft(&sq(18.0, 4.0), &sq(14.0, 4.0), 0);
    m.loft(&sq(14.0, 4.0), &sq(14.0, -12.0), 1);
    m.poly(&sq(14.0, -12.0), 3);
    for (x, y, tx, ty) in [(0.0, 0.0, 0.0, 0.0), (-6.0, 5.0, -9.0, 8.0), (6.0, -5.0, 9.0, -8.0), (5.0, 6.0, 8.0, 10.0), (-5.0, -6.0, -8.0, -10.0)] {
        m.shard([x, y, -12.0], [tx, ty, -3.0], 3.5, 3);
    }
    m.poly(&Mesh::ring(6, 3.0, -4.0, 0.0), 4);
    let dx = 14.0 * k as f64 / (HATCH_POSES - 1) as f64;
    for s in [-1.0, 1.0] {
        // The door retracts under the frame: it never shows beyond the frame's outer edge.
        let (inner, outer) = (s * dx, s * (14.0 + dx).min(18.0));
        let (lo, hi) = if s < 0.0 { (outer, inner) } else { (inner, outer) };
        m.cuboid([lo, -14.0, 1.5], [hi, 14.0, 3.0], 1);
        let (sl, sh) = if s < 0.0 { (inner - 3.0, inner) } else { (inner, inner + 3.0) };
        m.cuboid([sl, -14.0, 3.0], [sh, 14.0, 3.4], 2);
    }
    model(&format!("enemy.common.hatch.open{k}"), (22, 44), &[], m)
}

/// CRAWLER: a squat captured tank: two wide tracks, a boxy hull with a sloped glacis, one fixed forward
/// gun, a crystal cluster on the engine deck. 44 px long, 38 wide.
pub fn crawler() -> ModelFile {
    crawler_armed(false)
}

/// The CRAWLER about to fire: its muzzle glows `hot`.
pub fn crawler_armed(glow: bool) -> ModelFile {
    let mut m = Mesh::new(vec![hull(), hull_dark(), crystal(), weapon(), Material::glow(HOT)]);
    for s in [-1.0, 1.0] {
        let (a, b) = if s < 0.0 { (-19.0, -11.0) } else { (11.0, 19.0) };
        // Bevelled track guards: the outer slopes catch (or lose) the light.
        m.frustum([a, -22.0, 0.0], [b, 20.0, 0.0], 1.5, 8.0, 1);
    }
    m.frustum([-11.0, -20.0, 4.0], [11.0, 16.0, 4.0], 3.0, 13.0, 0);
    // A hipped armour roof: two long slopes and two end facets instead of a flat top.
    let (r0, r1) = ([0.0, -11.0, 17.0], [0.0, 7.0, 17.0]);
    let (bl, br, fr, fl) = ([-8.0, -17.0, 13.0], [8.0, -17.0, 13.0], [8.0, 13.0, 13.0], [-8.0, 13.0, 13.0]);
    m.poly(&[br, fr, r1, r0], 0);
    m.poly(&[fl, bl, r0, r1], 0);
    m.poly(&[bl, br, r0], 0);
    m.poly(&[fr, fl, r1], 0);
    m.poly(&[[-8.0, 13.0, 13.0], [8.0, 13.0, 13.0], [11.0, 22.0, 5.0], [-11.0, 22.0, 5.0]], 0);
    m.sym(&[[8.0, 13.0, 13.0], [11.0, 16.0, 4.0], [11.0, 22.0, 5.0]], 0);
    m.tube([0.0, 14.0, 11.0], [0.0, 27.0, 11.0], 2.0, 6, 3, 4);
    if glow {
        m.tube([0.0, 22.0, 11.5], [0.0, 29.0, 11.5], 2.8, 6, 4, 4);
    }
    m.shard([-4.0, -12.0, 13.0], [-6.0, -18.0, 20.0], 3.0, 2);
    m.shard([3.0, -13.0, 13.0], [5.0, -19.0, 19.0], 3.0, 2);
    m.shard([0.0, -8.0, 13.0], [0.0, -10.0, 22.0], 2.5, 2);
    model("enemy.common.crawler", (32, 64), &[("yaw", 16.0)], m)
}

/// A ground unit's wreck: a scorched crater, a ring of broken slabs tipped outward around a dark pit
/// with a few dying crystal shards. `r` is the outer radius (20: PILLBOX, HATCH; 24: CRAWLER).
pub fn wreck(r: f64) -> ModelFile {
    let mut m = Mesh::new(vec![hull_dark(), Material::glow(VOID), crystal()]);
    m.poly(&Mesh::ring(10, r * 0.55, 0.5, 0.0), 1);
    let n = 9;
    for k in 0..n {
        let a0 = k as f64 / n as f64 * 2.0 * PI + 0.2 * (k % 2) as f64;
        let a1 = a0 + 2.0 * PI / n as f64 * 0.8;
        let (ri, ro) = (r * 0.55, r * (0.85 + 0.15 * ((k * 7 % 5) as f64 / 4.0)));
        let h = 3.0 + (k * 5 % 4) as f64;
        let p = |a: f64, rr: f64, z: f64| -> V3 { [rr * a.cos(), rr * a.sin(), z] };
        // A slab tipped up at its inner edge.
        m.poly(&[p(a0, ri, h), p(a1, ri, h), p(a1, ro, 0.0), p(a0, ro, 0.0)], 0);
    }
    m.shard([2.0, 1.0, 0.5], [4.0, 4.0, 6.0], 2.0, 2);
    m.shard([-3.0, -2.0, 0.5], [-5.0, -1.0, 5.0], 1.5, 2);
    let canvas = if r > 21.0 { (32, 64) } else { (22, 44) };
    model(&format!("wreck{}", r as u32), canvas, &[], m)
}

/// KEELBREAKER: a captured destroyer, 256 lines long and about 52 wide: pointed bow, square stern, a
/// stepped bridge crowned with a faceted crystal, a funnel, three turret bases (the turrets are a
/// separate part), crystal grown through the deck seams. Rolls gently in the swell.
pub fn keelbreaker() -> ModelFile {
    let mut m = Mesh::new(vec![
        // A ground-level target the TALON flies over: the deck is one step down (void and dark only), so
        // the light TALON always reads on top of it; light only on the bridge.
        Material::lit(&[VOID, DARK], 0.75, 1),
        hull_dark(),
        crystal(),
        Material::lit(&[VOID, DARK, LIGHT], 0.55, 1),
        Material::glow(HOT),
    ]);
    let deck = [
        [0.0, 128.0, 10.0], [14.0, 100.0, 10.0], [25.0, 60.0, 10.0], [26.0, 0.0, 10.0], [25.0, -100.0, 10.0], [22.0, -128.0, 10.0],
        [-22.0, -128.0, 10.0], [-25.0, -100.0, 10.0], [-26.0, 0.0, 10.0], [-25.0, 60.0, 10.0], [-14.0, 100.0, 10.0],
    ];
    let water: Vec<V3> = deck.iter().map(|p| [p[0] * 0.86, p[1] * 0.96, -6.0]).collect();
    m.loft(&deck, &water, 0);
    // A low pyramid deck: long facets from the edges up to the centre catch the light differently.
    let mut top = vec![[0.0, 0.0, 18.0]];
    top.extend_from_slice(&deck);
    top.push(deck[0]);
    m.poly(&top, 0);
    m.frustum([-14.0, 0.0, 10.0], [14.0, 40.0, 10.0], 3.0, 24.0, 3);
    m.frustum([-9.0, 6.0, 24.0], [9.0, 32.0, 24.0], 2.0, 32.0, 3);
    m.shard([0.0, 20.0, 32.0], [0.0, 26.0, 44.0], 6.0, 2);
    m.shard([-5.0, 14.0, 32.0], [-9.0, 10.0, 40.0], 4.0, 2);
    m.shard([5.0, 14.0, 32.0], [9.0, 10.0, 40.0], 4.0, 2);
    m.tube([0.0, -14.0, 14.0], [0.0, -14.0, 28.0], 6.0, 6, 3, 1);
    for y in [86.0, -40.0, -92.0] {
        m.tube([0.0, y, 12.0], [0.0, y, 18.0], 12.0, 8, 1, 1);
    }
    for y in [-112.0, -66.0, 52.0, 108.0] {
        for sx in [-1.0, 1.0] {
            m.shard([sx * 15.0, y, 12.0], [sx * 18.0, y + 4.0, 21.0], 3.0, 2);
        }
    }
    // Authored on a 256-line hull; shown at 94 % so the bow and stern clear the canvas.
    for v in m.verts.iter_mut() {
        v[1] *= 0.94;
    }
    model("midboss.s1.keelbreaker", (32, 256), &[("roll", 5.0), ("roll_max_deg", 8.0)], m)
}

/// A KEELBREAKER turret (the part sprite): a domed ring with twin barrels and glowing muzzles.
pub fn keelbreaker_turret() -> ModelFile {
    let mut m = Mesh::new(vec![hull(), weapon(), Material::glow(HOT)]);
    m.loft(&Mesh::ring(8, 9.0, 0.0, PI / 8.0), &Mesh::ring(8, 7.0, 5.0, PI / 8.0), 0);
    m.poly(&Mesh::ring(8, 7.0, 5.0, PI / 8.0), 0);
    for x in [-2.5, 2.5] {
        m.tube([x, 4.0, 3.5], [x, 13.0, 3.5], 1.2, 5, 1, 2);
    }
    model("midboss.s1.keelbreaker.turret", (16, 32), &[("yaw", 16.0)], m)
}

/// The TALON (the S1 ship; the user picked it over the Ludo model), hand-built to its approved silhouette: a knife-edge
/// arrowhead nose, a narrow chiselled fuselage, wings swept steeply back that hook forward into claw
/// tips, one raked tail blade, twin nozzles. 64 px long.
pub fn talon() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 1.0, 2),
        Material::lit(&[VOID, DARK, LIGHT], 0.6, 2),
        Material::lit(&[DARK, LIGHT, HOT], 0.8, 1),
        Material::glow(HOT),
    ]);
    let nose = [0.0, 31.0, 2.0];
    let (c0, c1, c2, tail) = ([0.0, 18.0, 6.0], [0.0, 4.0, 7.0], [0.0, -20.0, 6.0], [0.0, -28.0, 4.0]);
    let (s0, s1, s2, s3) = ([3.0, 22.0, 2.0], [5.0, 8.0, 3.0], [5.0, -14.0, 3.0], [5.0, -28.0, 2.0]);
    m.sym(&[nose, s0, c0], 0);
    m.sym(&[c0, s0, s1, c1], 0);
    m.sym(&[c1, s1, s2, c2], 0);
    m.sym(&[c2, s2, s3, tail], 0);
    m.sym(&[nose, [0.0, 18.0, -1.0], s0], 1);
    m.sym(&[[0.0, 18.0, -1.0], [0.0, -24.0, -1.0], s2, s1, s0], 1);
    m.poly(&[[0.0, 16.0, 6.4], [1.6, 9.0, 6.8], [0.0, 2.0, 7.3], [-1.6, 9.0, 6.8]], 2);
    // Wings swept back, the tips hooking forward into claws; dark leading edges.
    m.sym(&[s1, [22.0, -12.0, 1.5], [12.0, -18.0, 2.5], s2], 0);
    m.sym(&[[22.0, -12.0, 1.5], [29.0, -3.0, 0.5], [26.0, -15.0, 0.8], [12.0, -18.0, 2.5]], 0);
    m.sym(&[s1, [6.0, 10.0, 3.6], [23.0, -10.0, 2.0], [22.0, -12.0, 1.5]], 1);
    m.beam([0.0, -14.0, 6.0], [0.0, -30.0, 14.0], 2.5, 3.0, 1);
    for x in [-2.5, 2.5] {
        m.tube([x, -26.0, 3.0], [x, -31.0, 3.0], 2.0, 6, 1, 3);
    }
    model("hd.ship.talon", (36, 66), &[("roll", 9.0), ("roll_max_deg", 60.0)], m)
}

/// The PEREGRINE (S2's airframe, QUILL's after the first promotion), hand-built to its approved
/// silhouette (`docs/story/hd/ships/ship-peregrine.md`): a needle nose, a long narrow fuselage, a cranked
/// delta with a sharp kink at half span, a dorsal spine ridge, small wingtip fins and one big round
/// nozzle. Long and narrow where the TALON is broad: 70 px long, 50 across. Cream (`light`) armour,
/// terracotta (`dark`) wing panels, in S2's colors.
pub fn peregrine() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 1.0, 2),
        Material::lit(&[VOID, DARK, LIGHT], 0.6, 2),
        Material::lit(&[DARK, LIGHT, HOT], 0.8, 1),
        Material::glow(HOT),
    ]);
    // Fuselage: a needle nose, then a long body that thickens to the wing root and narrows to the nozzle.
    let nose = [0.0, 34.0, 2.0];
    let (c0, c1, c2, c3) = ([0.0, 22.0, 4.5], [0.0, 8.0, 6.0], [0.0, -14.0, 6.0], [0.0, -28.0, 5.0]);
    let (s0, s1, s2, s3) = ([1.6, 24.0, 2.0], [3.6, 10.0, 3.0], [4.2, -14.0, 3.0], [4.4, -28.0, 2.5]);
    m.sym(&[nose, s0, c0], 0);
    m.sym(&[c0, s0, s1, c1], 0);
    m.sym(&[c1, s1, s2, c2], 0);
    m.sym(&[c2, s2, s3, c3], 0);
    m.sym(&[nose, [0.0, 24.0, -1.0], s0], 1);
    m.sym(&[[0.0, 24.0, -1.0], [0.0, -28.0, -1.0], s3, s2, s1, s0], 1);
    // Canopy: a slim light-blue slit ahead of the wing.
    m.poly(&[[0.0, 21.0, 4.9], [1.3, 15.0, 5.5], [0.0, 9.0, 6.3], [-1.3, 15.0, 5.5]], 2);
    // Dorsal spine ridge from behind the canopy to the tail.
    m.beam([0.0, 8.0, 6.0], [0.0, -27.0, 6.5], 1.6, 1.4, 1);
    // Cranked delta: the inner panel swept steeply from far forward to the kink at half span, the
    // outer panel swept less to the tip; one straight trailing edge. Terracotta panels in the middle.
    let (root_le, kink, tip, tip_te, root_te) =
        ([3.4, 12.0, 3.0], [13.0, -8.0, 2.4], [25.0, -18.0, 1.6], [25.0, -23.0, 1.6], [4.3, -23.0, 2.8]);
    m.sym(&[root_le, kink, [12.0, -12.0, 2.4], [4.2, -10.0, 3.0]], 0);
    m.sym(&[[4.2, -10.0, 3.0], [12.0, -12.0, 2.4], [13.0, -23.0, 2.2], root_te], 1);
    m.sym(&[kink, tip, tip_te, [13.0, -23.0, 2.2], [12.0, -12.0, 2.4]], 0);
    // Wingtip fins: small, raked back.
    for sx in [-1.0, 1.0] {
        m.poly(&[[sx * 25.0, -17.0, 1.6], [sx * 25.0, -23.5, 1.6], [sx * 25.0, -25.0, 6.5]], 1);
    }
    // One big round nozzle.
    m.tube([0.0, -27.0, 4.0], [0.0, -34.0, 4.0], 3.6, 8, 1, 3);
    model("hd.ship.peregrine", (32, 72), &[("roll", 9.0), ("roll_max_deg", 60.0)], m)
}

/// The CARACARA (S3's airframe, QUILL's after the second promotion), hand-built to its approved silhouette
/// (`docs/story/hd/ships/ship-caracara.md` rev 4): the first twin-boom ship. A short blunt armoured pod, a
/// broad straight wing, two parallel tail booms with engine pods, a crossbar tailplane, and the open
/// middle between the booms that no enemy has. Wide where the PEREGRINE is narrow: 56 px long, 88
/// across. Yellow (`light`) armour, orange (`dark`) booms and wing panels, in S3's colors.
pub fn caracara() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 1.2, 2),
        Material::lit(&[VOID, DARK, LIGHT], 0.62, 2),
        Material::lit(&[DARK, LIGHT, HOT], 0.8, 1),
        Material::glow(HOT),
    ]);
    // The pod: a blunt armoured nose, a high back, cut off square behind the wing.
    let (n0, n1) = ([0.0, 27.0, 3.0], [4.0, 25.0, 3.0]);
    let (c0, c1, c2) = ([0.0, 20.0, 8.0], [0.0, 4.0, 9.0], [0.0, -12.0, 7.0]);
    let (s0, s1, s2) = ([6.0, 20.0, 3.5], [7.0, 4.0, 4.0], [5.5, -12.0, 3.5]);
    m.sym(&[n0, n1, s0, c0], 0);
    m.sym(&[c0, s0, s1, c1], 0);
    m.sym(&[c1, s1, s2, c2], 0);
    m.poly(&[[-5.5, -12.0, 3.5], [5.5, -12.0, 3.5], [0.0, -12.0, 7.0]], 1);
    m.sym(&[n0, [0.0, 24.0, -1.0], [6.0, 20.0, -1.0], s0, n1], 1);
    // The canopy: a wide light-blue band across the pod's brow.
    m.poly(&[[-3.5, 19.0, 7.9], [3.5, 19.0, 7.9], [4.5, 13.0, 8.5], [0.0, 11.0, 8.8], [-4.5, 13.0, 8.5]], 2);
    // The broad straight wing: an armoured leading facet and an orange trailing facet, slight dihedral.
    m.sym(&[[6.5, 9.0, 4.0], [44.0, 6.0, 3.0], [44.0, 1.0, 3.6], [7.0, 0.0, 4.8]], 0);
    m.sym(&[[7.0, 0.0, 4.8], [44.0, 1.0, 3.6], [44.0, -4.0, 3.0], [6.0, -8.0, 4.0]], 1);
    // The booms, each with an engine pod over the wing and a nozzle glowing at its tail.
    for sx in [-1.0, 1.0] {
        let x = sx * 17.0;
        m.tube([x, 14.0, 5.0], [x, -6.0, 5.0], 4.0, 8, 0, 0);
        m.tube([x, -6.0, 5.0], [x, -9.0, 5.0], 3.0, 8, 1, 3);
        m.beam([x, -6.0, 4.5], [x, -27.0, 4.0], 3.0, 2.6, 1);
        // Twin fins, canted outward.
        m.poly(&[[x, -18.0, 5.5], [x, -27.0, 5.0], [x + sx * 3.5, -27.5, 11.0], [x + sx * 3.0, -22.0, 11.0]], 0);
    }
    // The crossbar tailplane joining the booms.
    m.poly(&[[-20.0, -22.0, 4.6], [20.0, -22.0, 4.6], [20.0, -27.0, 4.4], [-20.0, -27.0, 4.4]], 0);
    model("hd.ship.caracara", (52, 58), &[("roll", 9.0), ("roll_max_deg", 60.0)], m)
}

fn model(name: &str, canvas: (u16, u16), orient: &[(&str, f64)], mesh: Mesh) -> ModelFile {
    ModelFile {
        name: name.to_string(),
        canvas,
        orient: orient.iter().map(|&(k, v)| (k.to_string(), v)).collect::<BTreeMap<_, _>>(),
        mesh,
    }
}

/// The GANTRY QUEEN, hovering: Breakwater's control tower torn off the apron and recrystallised.
///
/// From above: a stepped octagonal crown (the cab roof in tiers, every slope 8 facets catching the light),
/// a ring of cyan crystal shards around the cab, two cannon arms with forward barrels, and four splayed
/// legs with knee humps, wider than tall (the canvas is 1024 × 512 hires). The tower is short so a 24°
/// bank tilts the crown without throwing it across the screen.
pub fn gantry_queen() -> ModelFile {
    let mut m = queen_mesh();
    let oct = PI / 8.0; // octagons with flat sides facing the axes

    // Shaft, ribbed, mostly seen when banking.
    let mut prev = Mesh::ring(8, 62.0, -70.0, oct);
    for (k, z) in [-40.0, -10.0, 20.0, 50.0, 80.0, 110.0].iter().enumerate() {
        let r = if k % 2 == 0 { 72.0 } else { 62.0 };
        let next = Mesh::ring(8, r, *z, oct);
        m.loft(&prev, &next, CONCRETE);
        prev = next;
    }
    // Cab underside flaring out, then the slanted window band.
    let skirt = Mesh::ring(8, 190.0, 150.0, oct);
    m.loft(&prev, &skirt, CONCRETE);
    let window_top = Mesh::ring(8, 214.0, 190.0, oct);
    m.loft(&skirt, &window_top, GLASS);
    // The crystal crown band (the purple facet ring of the base bible), then the stepped roof.
    let band_out = Mesh::ring(16, 226.0, 200.0, 0.0);
    let band_top = Mesh::ring(16, 206.0, 212.0, oct / 2.0);
    let window_top16 = Mesh::ring(16, 214.0, 190.0, 0.0);
    m.loft(&window_top16, &band_out, CRYSTAL);
    m.loft(&band_out, &band_top, CRYSTAL);
    // A faceted dome, cut like a gem: broad ~30° rings, so each shows 8 facets lit differently by the
    // upper-left key light, with a short vertical lip between rings for a crisp edge.
    let tiers = [
        (206.0, 212.0),
        (200.0, 220.0),
        (158.0, 244.0),
        (152.0, 250.0),
        (108.0, 272.0),
        (102.0, 277.0),
        (58.0, 292.0),
        (30.0, 296.0),
    ];
    let mut prev = Mesh::ring(8, tiers[0].0, tiers[0].1, oct);
    for &(r, z) in &tiers[1..] {
        let next = Mesh::ring(8, r, z, oct);
        m.loft(&prev, &next, CONCRETE);
        prev = next;
    }
    m.poly(&prev, CONCRETE);
    // Roof equipment: four vent blocks on the second tier, the radar mast and its beacon.
    for k in 0..4 {
        let a = PI / 4.0 + k as f64 * PI / 2.0;
        let (c, s) = (176.0 * a.cos(), 176.0 * a.sin());
        m.frustum([c - 16.0, s - 16.0, 226.0], [c + 16.0, s + 16.0, 226.0], 5.0, 250.0, STEEL);
    }
    m.tube([0.0, 0.0, 294.0], [0.0, 0.0, 360.0], 9.0, 6, HAZARD, GLOW);
    m.beam([-34.0, 0.0, 336.0], [34.0, 0.0, 336.0], 6.0, 6.0, HAZARD);

    // Eight crystal shards around the cab skirt, pointing out and slightly up.
    for k in 0..8 {
        let a = oct + k as f64 * PI / 4.0;
        let (c, s) = (a.cos(), a.sin());
        let len: f64 = if k % 2 == 0 { 120.0 } else { 90.0 };
        // Keep the tips inside the canvas height (±256 lines); the sideways ones reach furthest.
        let reach = (200.0 + len).min(244.0 / s.abs().max(1e-6));
        m.shard([200.0 * c, 200.0 * s, 170.0], [reach * c, reach * s, 205.0], 28.0, CRYSTAL);
    }

    // Authored at full size, shown at 0.6: the crown about a screen wide; with legs and cannons (separate
    // sprites, below) nearly three. Bigger than the screen, still readable as one machine.
    for v in m.verts.iter_mut() {
        *v = v.map(|c| c * QUEEN_SCALE);
    }
    model("boss.s1.gantry_queen", (512, 512), &[("roll", 49.0), ("roll_max_deg", 24.0)], m)
}

// The GANTRY QUEEN's materials (shared by the body and its parts).
const CONCRETE: u8 = 0;
const GLASS: u8 = 1;
const CRYSTAL: u8 = 2;
const HAZARD: u8 = 3;
const GLOW: u8 = 4;
const STEEL: u8 = 5;

fn queen_mesh() -> Mesh {
    Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 0.58, 1), // concrete: flat tops dark, light-facing slopes catch light
        Material::lit(&[VOID, DARK, HOT], 0.5, 0),    // smoked glass: dark, glints hot toward the light
        Material::lit(&[DARK, HOT], 1.0, 0),          // Lattice crystal
        Material::lit(&[VOID, DARK, HOT], 0.72, 1),   // weapons: hazard-striped, hot
        Material::glow(HOT),                          // muzzles, beacon
        Material::lit(&[VOID, DARK, LIGHT], 0.5, 1),  // legs and struts
    ])
}

/// How the parts follow the body's bank `f` (-1 full left .. 1 full right, i.e. moving left .. right):
/// the barrels lean into the turn; the hanging legs trail behind it like pendulums.
pub const ARM_LEAN_DEG: f64 = 12.0;
pub const LEG_SWING_DEG: f64 = 12.0;

/// The GANTRY QUEEN's wreck (`demo::QUEEN_WRECK_FRAME`): her body at the `QUEEN_WRECK_BANKS` samples. The
/// Lattice is dead (every material that could glow `hot` falls back to void/dark), the
/// hull is scorched (lower albedo), and plates are blown out of the upper hull (a stable ~12% of its
/// upward faces, chosen by hash, so the sea shows through).
pub fn queen_wreck_frames(m: &crate::mesh::ModelFile) -> Vec<crate::encode::SpriteImage> {
    let mut mesh = m.mesh.clone();
    for mat in mesh.materials.iter_mut() {
        if mat.ramp.contains(&HOT) {
            *mat = Material::lit(&[VOID, DARK], 0.45, 0);
        } else {
            mat.albedo *= 0.7;
        }
    }
    let mut kept = Mesh::new(mesh.materials.clone());
    for (i, t) in mesh.tris.iter().enumerate() {
        let (a, b, c) = (mesh.verts[t[0] as usize], mesh.verts[t[1] as usize], mesh.verts[t[2] as usize]);
        let n = [(b[1] - a[1]) * (c[2] - a[2]) - (b[2] - a[2]) * (c[1] - a[1]), (b[2] - a[2]) * (c[0] - a[0]) - (b[0] - a[0]) * (c[2] - a[2]), (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])];
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(1e-9);
        let up = (n[2] / len).abs() > 0.5;
        let h = (i as u32).wrapping_mul(2_654_435_761) >> 24;
        if up && h < 31 {
            continue;
        }
        kept.tri(a, b, c, mesh.mats[i]);
    }
    let view = crate::raster3d::View::centered(m.canvas.0, m.canvas.1, 1.0);
    let orients = m.orientations();
    let mut out = Vec::new();
    for &b in crate::demo::QUEEN_WRECK_BANKS.iter() {
        out.push(crate::raster3d::render(&kept, orients[b as usize], view));
    }
    out
}

/// The body's bank sample `b` (0..49) as a fraction (-1..1) and as a roll in radians.
pub fn queen_bank(b: usize) -> (f64, f64) {
    let n = crate::demo::BOSS_BANKS as f64 - 1.0;
    let f = b as f64 / n * 2.0 - 1.0;
    (f, (f * crate::demo::QUEEN_ROLL_MAX_DEG as f64).to_radians())
}

/// A cannon arm (`sx` = -1 left, 1 right) posed for bank sample `b`: a beam out from the cab, an elbow
/// block, twin barrels pointing at the player (-y), a crystal plate; swivelled at the shoulder.
pub fn queen_arm(sx: f64, b: usize) -> Mesh {
    let mut m = queen_mesh();
    m.beam([sx * 170.0, 0.0, 160.0], [sx * 400.0, -10.0, 150.0], 46.0, 34.0, HAZARD);
    m.frustum([sx * 400.0 - 40.0, -50.0, 120.0], [sx * 400.0 + 40.0, 30.0, 120.0], 10.0, 176.0, STEEL);
    for dx in [-18.0, 18.0] {
        let x = sx * 400.0 + dx;
        m.tube([x, -40.0, 160.0], [x, -210.0, 160.0], 11.0, 6, HAZARD, GLOW);
    }
    m.shard([sx * 300.0, -6.0, 172.0], [sx * 330.0, 40.0, 215.0], 20.0, CRYSTAL);
    // Counter-clockwise from above swings the barrels toward +x: into a turn to the right.
    swivel(&mut m, [sx * 170.0, 0.0], (ARM_LEAN_DEG * queen_bank(b).0).to_radians());
    let c = crate::demo::QUEEN_ARM_CENTER;
    part(m, [sx * c[0] as f64, c[1] as f64, c[2] as f64])
}

/// A leg (`ax`, `ay` = ±1: the quadrant) posed for bank sample `b`: hip on the shaft, knee hump, foot
/// pad, crystal plate on the knee; swung about the hip so the foot trails the direction of travel.
pub fn queen_leg(ax: f64, ay: f64, b: usize) -> Mesh {
    let mut m = queen_mesh();
    let a: f64 = 0.50; // ~29 deg off the x axis
    let dir = [ax * a.cos(), ay * a.sin()];
    let at = |r: f64, z: f64| -> V3 { [dir[0] * r, dir[1] * r, z] };
    m.beam(at(60.0, 20.0), at(300.0, 90.0), 44.0, 38.0, STEEL);
    m.beam(at(300.0, 90.0), at(470.0, -70.0), 38.0, 34.0, STEEL);
    let f = at(470.0, -80.0);
    m.frustum([f[0] - 34.0, f[1] - 30.0, -110.0], [f[0] + 34.0, f[1] + 30.0, -110.0], 10.0, -70.0, CONCRETE);
    m.shard(at(300.0, 104.0), at(330.0, 150.0), 26.0, CRYSTAL);
    // Rotating by θ about the hip moves the foot by θ·R·(-ay·sin a) in x: sign ay makes every foot trail.
    let hip = at(60.0, 0.0);
    swivel(&mut m, [hip[0], hip[1]], ay * (LEG_SWING_DEG * queen_bank(b).0).to_radians());
    let c = crate::demo::QUEEN_LEG_CENTER;
    part(m, [ax * c[0] as f64, ay * c[1] as f64, c[2] as f64])
}

/// Part `i` (arms left, right; legs nw, ne, sw, se: `demo::PART_CENTERS` order) posed for bank `b`.
pub fn queen_part(i: usize, b: usize) -> Mesh {
    match i {
        0 => queen_arm(-1.0, b),
        1 => queen_arm(1.0, b),
        2 => queen_leg(-1.0, 1.0, b),
        3 => queen_leg(1.0, 1.0, b),
        4 => queen_leg(-1.0, -1.0, b),
        _ => queen_leg(1.0, -1.0, b),
    }
}

/// Every part frame in the level image's order: per part, one frame per body bank sample, posed for
/// that bank and rendered at the body's own roll (1° apart, so parts and body never disagree).
pub fn queen_part_frames() -> Vec<crate::encode::SpriteImage> {
    let mut out = Vec::new();
    for i in 0..6 {
        let (w, h) = crate::demo::PART_CANVAS[i];
        let view = crate::raster3d::View::centered(w, h, 1.0);
        for b in 0..crate::demo::BOSS_BANKS as usize {
            let roll = queen_bank(b).1;
            out.push(crate::raster3d::render(&queen_part(i, b), crate::raster3d::Orient { yaw: 0.0, pitch: 0.0, roll }, view));
        }
    }
    out
}

/// Rotate the mesh about the vertical axis through `pivot` (x, y), counter-clockwise from above.
fn swivel(m: &mut Mesh, pivot: [f64; 2], a: f64) {
    let (s, c) = a.sin_cos();
    for v in m.verts.iter_mut() {
        let (x, y) = (v[0] - pivot[0], v[1] - pivot[1]);
        *v = [pivot[0] + x * c - y * s, pivot[1] + x * s + y * c, v[2]];
    }
}

/// A part: scaled like the body, then moved so `center` (in scaled body coordinates, hires) is its
/// origin. The scene puts the part's sprite at the body's rolled image of `center`.
fn part(mut m: Mesh, center: V3) -> Mesh {
    for v in m.verts.iter_mut() {
        *v = [v[0] * QUEEN_SCALE - center[0], v[1] * QUEEN_SCALE - center[1], v[2] * QUEEN_SCALE - center[2]];
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    /// No hand model (every stage's) may touch its canvas edge at any orientation (that would be clipping).
    #[test]
    fn every_model_fits_at_every_orientation() {
        let mut bad = Vec::new();
        // S2's and S3's too: `hand-models` writes their `.mesh` files at these canvases and orientations.
        for (id, m) in all().into_iter().chain(crate::hand_s2::all()).chain(crate::hand_s3::all()) {
            for (i, img) in m.render_all().iter().enumerate() {
                let (w, h) = (img.w, img.h);
                let edge = (0..w).any(|x| img.get(x, 0).is_some() || img.get(x, h - 1).is_some())
                    || (0..h).any(|y| img.get(0, y).is_some() || img.get(w - 1, y).is_some());
                if edge || img.pixels.iter().all(|p| p.is_none()) {
                    bad.push(format!("{id}#{i}"));
                }
            }
        }
        assert!(bad.is_empty(), "clipped or empty: {bad:?}");
    }

    /// No part frame touches its canvas edge, and between neighbouring bank samples no part's pixels move
    /// far: the sway is smooth (the user saw the old 6-pose, 7-roll legs jump).
    #[test]
    fn queen_parts_fit_and_move_smoothly() {
        let frames = queen_part_frames();
        let n = crate::demo::BOSS_BANKS as usize;
        for (k, img) in frames.iter().enumerate() {
            let (w, h) = (img.w, img.h);
            let edge = (0..w).any(|x| img.get(x, 0).is_some() || img.get(x, h - 1).is_some())
                || (0..h).any(|y| img.get(0, y).is_some() || img.get(w - 1, y).is_some());
            assert!(!edge, "part {} bank {} touches its canvas edge", k / n, k % n);
        }
        // Centroid shift between neighbouring banks (part-relative; the bank slide is exact and added by
        // the scene), in multicolor px and lines.
        let centroid = |img: &crate::encode::SpriteImage| {
            let (mut sx, mut sy, mut c) = (0.0, 0.0, 0.0);
            for y in 0..img.h {
                for x in 0..img.w {
                    if img.get(x, y).is_some() {
                        sx += x as f64;
                        sy += y as f64;
                        c += 1.0;
                    }
                }
            }
            (sx / c, sy / c)
        };
        for i in 0..6 {
            for b in 1..n {
                let (a, c) = (centroid(&frames[i * n + b - 1]), centroid(&frames[i * n + b]));
                let d = ((a.0 - c.0).powi(2) + (a.1 - c.1).powi(2)).sqrt();
                assert!(d < 1.5, "part {i} bank {b}: centroid moves {d:.2} px");
            }
        }
    }

    #[test]
    fn queen_fits_its_canvas_and_budget() {
        let q = gantry_queen();
        let (mut lo, mut hi) = ([f64::MAX; 3], [f64::MIN; 3]);
        for v in &q.mesh.verts {
            for i in 0..3 {
                lo[i] = lo[i].min(v[i]);
                hi[i] = hi[i].max(v[i]);
            }
        }
        // 512 multicolor px = 1024 hires across, 512 lines tall, centred.
        assert!(lo[0] >= -512.0 && hi[0] <= 512.0, "x {lo:?} {hi:?}");
        assert!(lo[1] >= -256.0 && hi[1] <= 256.0, "y {lo:?} {hi:?}");
        // The assembly (body, two arms, four legs) is the BOSS budget.
        let n = q.mesh.tris.len() + (0..6).map(|i| queen_part(i, 24).tris.len()).sum::<usize>();
        assert!((500..=4000).contains(&n), "{n} triangles");
    }
}


