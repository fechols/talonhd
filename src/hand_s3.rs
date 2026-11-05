//! The S3 REDLINE roster, hand-built (host only) to the approved `[model]` blocks in
//! `docs/story/hd/models/s3.md` and rendered under S3's hard low sun (`s3::SUN`, upper left, almost no
//! fill): DUST DEVIL, RAIL CAR and its turret, SANDCRAWLER (and its dust ring), the WATER TOWER's lid,
//! VULTURE, LANCER, the SIDEWINDER's engine and turret car, and the TRESTLE TITAN (body, two claws on
//! their jibs, the boiler door). S1's WASP and CRAWLER come back re-lit by the same sun.
//!
//! Units and axes as `hand.rs`: hires px, x right, y forward (up the screen), z toward the viewer; a
//! canvas is (multicolor pixels across, lines). Light belongs to the player (the yellow CARACARA): enemy
//! hulls are void/dark with `light` only on sunny facets and hazard paint; `hot` is the Lattice (cyan ore).
//!
//! Frame sets (the order `roster` gives them):
//! - DUST DEVIL: 8 spins over 120° (3 blades), its sand ring drawn under the blades.
//! - RAIL CAR: 16 yaws. Turret: 16 headings, then the same 16 armed (the SIDEWINDER's cars use them too).
//! - SANDCRAWLER: 4 rise poses facing down the screen, 16 yaws, 16 yaws armed. Dust ring: 4 sizes.
//! - WATER TOWER: the lid over 4 pulses, then the wreck.
//! - VULTURE: 5 rolls, then the same 5 armed. LANCER: 16 yaws.
//! - SIDEWINDER: engine 3 sways on the rail, then 16 yaws (phase 2); turret car 3 sways, then the wreck.
//! - TRESTLE TITAN: body 4 sways, then the wreck; each claw 16 slews × 4 drops (slew-major), the right
//!   one the left mirrored; boiler door 4 opening poses; the claw's shadow in 4 sizes.

use std::f64::consts::{PI, TAU};

use crate::encode::SpriteImage;
use crate::mesh::ModelFile;
use crate::raster3d::{self, Material, Mesh, Orient, V3, View};
use crate::s3::SUN;

const VOID: u8 = 0;
const DARK: u8 = 1;
const LIGHT: u8 = 2;
const HOT: u8 = 3;

/// Poses and samples (the bible's `orient`).
pub const YAWS: usize = 16;
pub const DEVIL_SPINS: usize = 8;
pub const RISE_POSES: usize = 4;
pub const RING_SIZES: usize = 4;
pub const TOWER_PULSES: usize = 4;
pub const VULTURE_ROLLS: usize = 5;
pub const SWAYS: usize = 3;
pub const TITAN_SWAYS: usize = 4;
pub const SLEWS: usize = 16;
pub const DROPS: usize = 4;
pub const DOOR_POSES: usize = 4;
/// The jibs' sweep either side of straight down the screen, and their length (pivot to cable).
pub const SLEW_DEG: f64 = 60.0;
pub const JIB: f64 = 58.0;
/// Where each jib pivots on the TITAN's body (hires px, lines from the body's centre; the right one is
/// mirrored) and where the boiler door sits.
pub const PIVOT: (f64, f64) = (-96.0, -58.0);
pub const BOILER: (f64, f64) = (0.0, 6.0);
/// The TITAN's bogies sit on the crane rails (`s3::CRANE_RAILS`, 124 px either side of the yard's middle).
pub const BOGIE_X: f64 = 124.0;

fn model(name: &str, canvas: (u16, u16), orient: &[(&str, f64)], mesh: Mesh) -> ModelFile {
    ModelFile {
        name: name.to_string(),
        canvas,
        orient: orient.iter().map(|&(k, v)| (k.to_string(), v)).collect(),
        mesh,
    }
}

/// One frame of `mesh` on a `canvas`, under S3's sun.
fn shot(mesh: &Mesh, canvas: (u16, u16), o: Orient) -> SpriteImage {
    raster3d::render_lit(mesh, o, View::centered(canvas.0, canvas.1, 1.0), &SUN)
}

fn yaw_of(i: usize) -> f64 {
    i as f64 * TAU / YAWS as f64
}

fn rolled(roll_deg: f64) -> Orient {
    Orient { yaw: 0.0, pitch: 0.0, roll: roll_deg.to_radians() }
}

/// The `n` sway samples between -`max` and `max` degrees.
fn sways(n: usize, max: f64) -> Vec<f64> {
    (0..n).map(|i| (i as f64 / (n - 1) as f64 * 2.0 - 1.0) * max).collect()
}

/// Every vertex of `m` from index `from` on moved by `f`.
fn warp(m: &mut Mesh, from: usize, f: impl Fn(V3) -> V3) {
    for v in m.verts[from..].iter_mut() {
        *v = f(*v);
    }
}

/// A procedural frame: `w` × `h` (multicolor pixels × lines) with `code` wherever `inside(x, y)` holds,
/// x in hires px and y in lines from the centre (y up).
fn stamp(w: u16, h: u16, code: u8, inside: impl Fn(f64, f64) -> bool) -> SpriteImage {
    let mut img = SpriteImage::new(w, h);
    for py in 0..h {
        for px in 0..w {
            let (x, y) = ((px as f64 + 0.5) * 2.0 - w as f64, h as f64 / 2.0 - (py as f64 + 0.5));
            if inside(x, y) {
                img.set(px, py, Some(code));
            }
        }
    }
    img
}

// ------------------------------------------------------------------------------------- DUST DEVIL

/// DUST DEVIL: a three-bladed crystal rotor over a hexagonal hub, a dotted dark sand ring under its
/// blades. 3-fold symmetric: 8 spins over 120 degrees.
pub fn dust_devil() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, HOT], 0.8, 1),
        Material::lit(&[VOID, DARK], 0.85, 0),
        Material::glow(HOT),
        Material::glow(DARK),
    ]);
    for k in 0..15 {
        let a = k as f64 * TAU / 15.0;
        let (c, s) = (a.cos(), a.sin());
        m.poly(&[[9.0 * c - s, 9.0 * s + c, 0.0], [10.3 * c - s, 10.3 * s + c, 0.0], [10.3 * c, 10.3 * s, 0.0], [9.0 * c, 9.0 * s, 0.0]], 3);
    }
    for b in 0..3 {
        let a = b as f64 * TAU / 3.0;
        let at = |r: f64, da: f64, z: f64| -> V3 { [r * (a + da).cos(), r * (a + da).sin(), z] };
        // A curved blade: the leading facet lit, the trailing facet in shade, a dark edge behind.
        let (root0, root1, mid, tip) = (at(3.0, -0.45, 3.0), at(3.0, 0.45, 3.0), at(6.5, 0.25, 5.0), at(10.0, 0.6, 3.5));
        let spine = at(6.5, 0.0, 6.0);
        m.poly(&[root0, spine, tip, mid], 0);
        m.poly(&[root1, mid, tip], 0);
        m.poly(&[root0, at(6.5, -0.25, 3.0), tip, spine], 1);
    }
    m.loft(&Mesh::ring(6, 3.6, 2.0, 0.0), &Mesh::ring(6, 3.0, 6.5, 0.0), 1);
    m.poly(&Mesh::ring(6, 3.0, 6.5, 0.0), 2);
    model("enemy.s3.dust_devil", (24, 24), &[("yaw", DEVIL_SPINS as f64), ("yaw_span_deg", 120.0)], m)
}

// --------------------------------------------------------------------------------------- RAIL CAR

/// RAIL CAR: a flatcar, its deck heaped with crystal ore fore and aft, stakes along its sides and an
/// empty turret ring amidships (the turret is its own sprite). 60 px long; yaws to its rail.
pub fn rail_car() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK], 0.85, 1),
        Material::lit(&[VOID, DARK], 0.6, 0),
        Material::lit(&[DARK, HOT], 0.8, 0),
        Material::glow(VOID),
    ]);
    m.cuboid([-7.0, -29.0, 0.0], [7.0, 29.0, 4.0], 0);
    for y in [-29.0, 29.0] {
        m.cuboid([-2.0, y - 1.0, 1.0], [2.0, y + 1.0, 3.0], 1);
    }
    let mut y = -26.0;
    while y <= 26.0 {
        for x in [-6.5, 6.5] {
            m.cuboid([x - 0.6, y - 0.6, 4.0], [x + 0.6, y + 0.6, 7.0], 1);
        }
        y += 6.5;
    }
    // Ore heaps: rough faceted mounds.
    for cy in [-17.0, 17.0] {
        let base: Vec<V3> = Mesh::ring(7, 5.5, 4.0, 0.3).iter().map(|p| [p[0], p[1] * 1.6 + cy, p[2]]).collect();
        let top = [0.0, cy + 1.0, 9.5];
        m.loft(&base, &base.iter().map(|p| [p[0] * 0.5, (p[1] - cy) * 0.5 + cy, 7.5]).collect::<Vec<_>>(), 2);
        let ring: Vec<V3> = base.iter().map(|p| [p[0] * 0.5, (p[1] - cy) * 0.5 + cy, 7.5]).collect();
        for i in 0..ring.len() {
            m.tri(ring[i], ring[(i + 1) % ring.len()], top, 2);
        }
    }
    m.loft(&Mesh::ring(10, 6.0, 4.2, 0.0), &Mesh::ring(10, 4.5, 4.2, 0.0), 1);
    m.poly(&Mesh::ring(10, 4.5, 4.1, 0.0), 3);
    model("enemy.s3.rail_car", (32, 64), &[("yaw", YAWS as f64)], m)
}

/// The RAIL CAR's turret at `yaw`: a squat housing on its ring, twin barrels; `armed`: the muzzles glow
/// `hot` (the steady telegraph, as S1's PILLBOX).
pub fn turret_mesh(yaw: f64, armed: bool) -> Mesh {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 0.55, 1),
        Material::lit(&[VOID, DARK], 0.85, 1),
        Material::glow(HOT),
    ]);
    m.loft(&Mesh::ring(10, 6.0, 0.0, 0.0), &Mesh::ring(10, 5.5, 3.0, 0.0), 1);
    m.frustum([-4.5, -5.0, 3.0], [4.5, 4.0, 3.0], 1.5, 7.0, 0);
    for x in [-1.8, 1.8] {
        m.tube([x, 3.0, 5.0], [x, 10.0, 5.0], 1.0, 5, 1, if armed { 2 } else { 1 });
        if armed {
            m.tube([x, 8.5, 5.0], [x, 10.5, 5.0], 1.5, 5, 2, 2);
        }
    }
    let o = Orient { yaw, pitch: 0.0, roll: 0.0 };
    warp(&mut m, 0, |v| o.apply(v));
    m
}

pub fn turret() -> ModelFile {
    model("enemy.s3.rail_car.turret", (24, 24), &[], turret_mesh(0.0, false))
}

// ------------------------------------------------------------------------------------ SANDCRAWLER

/// SANDCRAWLER rise pose `rise` (0..[`RISE_POSES`]: a dune mound, the blade through, the sand sliding
/// off, clear): a wide desert tank, a dozer blade, a three-tube crystal launcher; `armed`: the tubes'
/// mouths glow `hot`. Built facing +y.
pub fn sandcrawler_mesh(rise: usize, armed: bool) -> Mesh {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 0.55, 1),
        Material::lit(&[VOID, DARK], 0.85, 0),
        Material::lit(&[VOID, DARK], 0.95, 0),
        Material::lit(&[DARK, HOT], 0.8, 0),
        Material::glow(HOT),
    ]);
    let sunk = [-11.0, -6.0, -1.5, 0.0][rise.min(RISE_POSES - 1)];
    let from = m.verts.len();
    // Treads, the hull between them, a raised engine deck.
    for x in [-12.0, 12.0] {
        m.frustum([x - 4.0, -20.0, 0.0], [x + 4.0, 19.0, 0.0], 1.0, 6.0, 1);
    }
    m.frustum([-9.0, -21.0, 2.0], [9.0, 17.0, 2.0], 1.5, 9.0, 0);
    m.frustum([-6.0, -18.0, 9.0], [6.0, -4.0, 9.0], 1.0, 11.0, 0);
    // The dozer blade: a curved plate across the front, wider than the treads.
    m.poly(&[[-14.0, 20.0, 0.0], [14.0, 20.0, 0.0], [13.5, 22.0, 6.0], [-13.5, 22.0, 6.0]], 1);
    m.poly(&[[-13.5, 22.0, 6.0], [13.5, 22.0, 6.0], [13.0, 20.5, 9.0], [-13.0, 20.5, 9.0]], 1);
    for x in [-9.0, 9.0] {
        m.beam([x, 21.0, 4.0], [x, 15.0, 5.0], 1.5, 1.5, 1);
    }
    // The launcher: three crystal tubes on the hull, pointing forward and up.
    for x in [-4.0, 0.0, 4.0] {
        m.tube([x, 0.0, 10.0], [x, 12.0, 13.0], 1.5, 6, 3, if armed { 4 } else { 3 });
    }
    warp(&mut m, from, |v| [v[0], v[1], v[2] + sunk]);
    // The sand: a dune mound over it, then heaps sliding off the deck and the treads.
    let s = [1.0, 0.75, 0.4, 0.0][rise.min(RISE_POSES - 1)];
    if s > 0.0 {
        if rise < 2 {
            let h = 13.0 * s;
            let ring = |f: f64, z: f64| -> Vec<V3> { Mesh::ring(12, 1.0, z, 0.2).iter().map(|p| [p[0] * 21.0 * f, p[1] * 25.0 * f - 2.0, p[2]]).collect() };
            m.loft(&ring(1.0, 0.0), &ring(0.75, h * 0.6), 2);
            m.loft(&ring(0.75, h * 0.6), &ring(0.4, h), 2);
            m.poly(&ring(0.4, h), 2);
        } else {
            for (x, y, r) in [(0.0, -10.0, 6.0), (-12.0, 8.0, 4.5), (12.0, -6.0, 4.5), (4.0, 10.0, 3.5)] {
                let ring: Vec<V3> = Mesh::ring(8, r, 0.0, 0.0).iter().map(|p| [p[0] + x, p[1] + y, p[2] + 6.0]).collect();
                let top = [x, y, 6.0 + r * 0.9];
                for i in 0..ring.len() {
                    m.tri(ring[i], ring[(i + 1) % ring.len()], top, 2);
                }
            }
        }
    }
    m
}

pub fn sandcrawler() -> ModelFile {
    model("enemy.s3.sandcrawler", (40, 56), &[("yaw", YAWS as f64)], sandcrawler_mesh(RISE_POSES - 1, false))
}

/// The SANDCRAWLER's dust ring at size `k` of [`RING_SIZES`]: a dotted dark circle (not a model).
pub fn dust_ring(k: usize) -> SpriteImage {
    let r = 11.0 + 5.0 * k as f64;
    let dots = (r * 0.9) as usize;
    stamp(56, 56, DARK, |x, y| {
        let d = (x * x + y * y).sqrt();
        if (d - r).abs() > 1.3 {
            return false;
        }
        let a = y.atan2(x).rem_euclid(TAU) / TAU * dots as f64;
        a.fract() < 0.5
    })
}

// ------------------------------------------------------------------------------------ WATER TOWER

/// The WATER TOWER's lid at pulse `k` of [`TOWER_PULSES`] (`k == TOWER_PULSES`: the wreck), drawn over
/// the level's tank top (`s3::water_tower`: tank radius 13, the lid a cone to radius 4): the lid split in
/// two by the crystal growing through it, more of it `hot` each pulse. The wreck: the lid blown off, the
/// tank's dark inside, splintered staves.
pub fn water_tower(k: usize) -> ModelFile {
    let glow = [0.75, 0.95, 1.15, 1.4][k.min(TOWER_PULSES - 1)];
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK], 0.9, 1),
        Material::lit(&[VOID, DARK, LIGHT], 0.5, 1),
        Material::lit(&[DARK, HOT], glow, 0),
        Material::glow(VOID),
        Material::glow(HOT),
    ]);
    let rim = |r: f64, z: f64| Mesh::ring(16, r, z, 0.2);
    m.loft(&rim(13.5, 0.0), &rim(13.5, 1.5), 1);
    if k >= TOWER_PULSES {
        m.poly(&rim(12.0, 0.5), 3);
        m.loft(&rim(13.0, 1.5), &rim(11.5, 1.5), 0);
        for i in 0..7 {
            let a = i as f64 * TAU / 7.0 + 0.3;
            let (c, s) = (a.cos(), a.sin());
            m.poly(&[[12.0 * c, 12.0 * s, 1.5], [13.0 * c, 13.0 * s, 1.5], [13.5 * c - 1.5 * s, 13.5 * s + 1.5 * c, 5.0 + (i % 3) as f64]], 0);
        }
        return model("enemy.s3.water_tower.wreck", (48, 48), &[], m);
    }
    // The lid: two halves of the cone parted along x by the split.
    let gap = 1.5 + 1.0 * k as f64;
    for sx in [-1.0, 1.0] {
        let n = 8;
        for i in 0..n {
            let (a0, a1) = (-PI / 2.0 + i as f64 * PI / n as f64, -PI / 2.0 + (i + 1) as f64 * PI / n as f64);
            let p = |a: f64, r: f64, z: f64| -> V3 { [sx * (r * a.cos() + gap), r * a.sin(), z] };
            m.poly(&[p(a0, 13.0, 1.5), p(a1, 13.0, 1.5), p(a1, 4.0, 5.5), p(a0, 4.0, 5.5)], 0);
        }
        m.poly(&[[sx * gap, -4.0, 5.5], [sx * gap, 4.0, 5.5], [sx * (gap + 4.0), 0.0, 5.5]], 0);
    }
    m.poly(&[[-gap, -13.0, 1.0], [gap, -13.0, 1.0], [gap, 13.0, 1.0], [-gap, 13.0, 1.0]], 3);
    // The crystal: shards through the split, taller each pulse, a hot core at the last.
    let h = 5.0 + 2.5 * k as f64;
    // Leaning out of the split either way, so from above they read longer each pulse.
    let out = 1.5 + 2.0 * k as f64;
    for (i, (y, lean)) in [(-7.0, -2.0), (-2.0, 0.5), (3.0, -0.5), (8.0, 2.0)].into_iter().enumerate() {
        let sx = if i % 2 == 0 { -1.0 } else { 1.0 };
        m.shard([0.0, y, 1.0], [sx * out, y + lean, 1.0 + h], 2.4 + 0.5 * k as f64, 2);
    }
    if k + 1 == TOWER_PULSES {
        m.poly(&Mesh::ring(6, 1.6, 1.0 + h * 0.6, 0.0), 4);
    }
    model(&format!("enemy.s3.water_tower.pulse{k}"), (48, 48), &[], m)
}

// ---------------------------------------------------------------------------------------- VULTURE

/// VULTURE: a heavy gunship flying nose down the screen: a thick fuselage around a crystal core, broad
/// straight wings (104 px) with two engine pods each, a tailplane. Armour mostly dark, the light only on
/// the sun-facing wing edges; `armed`: the pods' muzzles glow `hot`.
pub fn vulture_mesh(armed: bool) -> Mesh {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 0.5, 1),
        Material::lit(&[VOID, DARK], 0.8, 1),
        Material::lit(&[DARK, HOT], 0.9, 0),
        Material::glow(HOT),
    ]);
    // Fuselage, nose at -y.
    let nose = [0.0, -38.0, 4.0];
    let (c0, c1, c2, tail) = ([0.0, -28.0, 10.0], [0.0, -6.0, 12.0], [0.0, 22.0, 10.0], [0.0, 34.0, 7.0]);
    let (s0, s1, s2, s3) = ([6.0, -30.0, 4.0], [9.0, -6.0, 5.0], [8.0, 22.0, 5.0], [4.0, 34.0, 4.0]);
    m.sym(&[nose, s0, c0], 0);
    m.sym(&[c0, s0, s1, c1], 0);
    m.sym(&[c1, s1, s2, c2], 0);
    m.sym(&[c2, s2, s3, tail], 0);
    // Straight wings: a leading facet (catches the sun) and a trailing facet.
    m.sym(&[[8.5, -14.0, 6.0], [52.0, -12.0, 4.0], [52.0, -6.0, 5.0], [9.0, -4.0, 7.0]], 0);
    m.sym(&[[9.0, -4.0, 7.0], [52.0, -6.0, 5.0], [52.0, 0.0, 4.0], [8.5, 6.0, 5.5]], 1);
    // Engine pods, two a wing.
    for x in [-38.0, -20.0, 20.0, 38.0] {
        m.tube([x, -20.0, 5.5], [x, 4.0, 5.5], 3.4, 8, 1, 1);
        m.tube([x, 4.0, 5.5], [x, 7.0, 5.5], 2.4, 8, 1, 3);
        if armed {
            m.tube([x, -22.0, 5.5], [x, -20.0, 5.5], 2.6, 8, 3, 3);
        }
    }
    // Tailplane and a dorsal fin.
    m.sym(&[[4.0, 26.0, 6.0], [20.0, 30.0, 5.0], [20.0, 35.0, 5.0], [4.0, 34.0, 5.0]], 1);
    m.poly(&[[0.0, 18.0, 10.5], [0.0, 34.0, 7.0], [0.0, 36.0, 15.0], [0.0, 28.0, 15.0]], 1);
    // The crystal core through the fuselage's back.
    for (y, h) in [(-12.0, 6.0), (-6.0, 8.0), (0.0, 5.5)] {
        m.shard([0.0, y, 11.0], [0.0, y + 2.0, 11.0 + h], 3.2, 2);
    }
    m.poly(&Mesh::ring(6, 2.0, 12.6, 0.0).iter().map(|p| [p[0], p[1] - 6.0, p[2]]).collect::<Vec<_>>(), 3);
    m
}

pub fn vulture(armed: bool) -> ModelFile {
    let name = if armed { "enemy.s3.vulture.armed" } else { "enemy.s3.vulture" };
    model(name, (112, 88), &[("roll", VULTURE_ROLLS as f64), ("roll_max_deg", 24.0)], vulture_mesh(armed))
}

// ----------------------------------------------------------------------------------------- LANCER

/// LANCER: a crystal spear: a long faceted spearhead, a ridge down its spine (its only `hot`), a short
/// shaft, four rear fins. 44 px long, nose +y.
pub fn lancer() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK], 0.85, 1),
        Material::lit(&[DARK, HOT], 1.0, 0),
        Material::glow(VOID),
    ]);
    let (tip, waist) = ([0.0, 22.0, 3.0], [0.0, -8.0, 3.0]);
    let (r, t, l, b) = ([3.5, 0.0, 3.0], [0.0, 0.0, 6.5], [-3.5, 0.0, 3.0], [0.0, 0.0, 0.0]);
    for (p, q) in [(r, t), (t, l), (l, b), (b, r)] {
        m.tri(tip, p, q, 0);
        m.tri(waist, q, p, 0);
    }
    m.tube([0.0, -8.0, 3.0], [0.0, -19.0, 3.0], 1.4, 4, 0, 0);
    m.sym(&[[1.0, -12.0, 3.0], [6.0, -20.0, 2.5], [5.0, -21.0, 2.5], [1.0, -19.0, 3.0]], 0);
    m.poly(&[[0.0, -12.0, 4.0], [0.0, -19.0, 4.0], [0.0, -21.0, 8.5], [0.0, -19.5, 8.5]], 0);
    m.beam([0.0, 19.0, 3.6], [0.0, 1.0, 6.6], 1.2, 0.8, 1);
    m.beam([0.0, -1.0, 6.4], [0.0, -7.0, 3.8], 1.0, 0.6, 1);
    for (p, q) in [(r, t), (l, t)] {
        m.beam([p[0] * 0.6, 8.0, p[2] * 0.6 + q[2] * 0.4], [p[0], 0.0, p[2]], 0.4, 0.3, 2);
    }
    model("enemy.common.lancer", (24, 48), &[("yaw", YAWS as f64)], m)
}

// ------------------------------------------------------------------------------------- SIDEWINDER

/// The SIDEWINDER's engine: an armoured crystal locomotive with a wedge nose striped yellow and black
/// (its one light area) and a crystal rib down its roof. 72 px long, nose +y.
pub fn sidewinder_engine() -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK], 0.85, 1),
        Material::glow(VOID),
        Material::lit(&[VOID, DARK, LIGHT], 0.9, 1),
        Material::lit(&[DARK, HOT], 1.0, 0),
    ]);
    // Body: armoured box with a chamfered roof.
    m.frustum([-12.0, -34.0, 0.0], [12.0, 18.0, 0.0], 3.0, 13.0, 0);
    // The wedge nose: hazard stripes across it, alternating yellow and black.
    let n = 6;
    for i in 0..n {
        let (f0, f1) = (i as f64 / n as f64, (i + 1) as f64 / n as f64);
        let at = |f: f64, x: f64| -> V3 { [x * (1.0 - 0.45 * f), 18.0 + 17.0 * f, 13.0 - 10.0 * f] };
        m.poly(&[at(f0, -12.0), at(f0, 12.0), at(f1, 12.0), at(f1, -12.0)], if i % 2 == 0 { 2 } else { 1 });
    }
    m.poly(&[[-6.6, 35.0, 3.0], [6.6, 35.0, 3.0], [6.0, 36.0, 0.0], [-6.0, 36.0, 0.0]], 1);
    // Plates, the roof rib, couplers.
    for y in [-24.0, -12.0, 0.0] {
        m.beam([-9.0, y, 13.2], [9.0, y, 13.2], 1.2, 0.6, 1);
    }
    m.beam([0.0, -30.0, 13.4], [0.0, 14.0, 13.4], 2.4, 1.6, 3);
    m.cuboid([-3.0, -38.0, 2.0], [3.0, -34.0, 6.0], 1);
    model("midboss.s3.sidewinder", (48, 80), &[("yaw", YAWS as f64)], m)
}

/// One of the SIDEWINDER's turret cars (`wreck`: plates burst, the crystal dead, derailed at an angle):
/// an armoured car with a turret well for the RAIL CAR's turret, short crystal ribs, couplers.
pub fn sidewinder_car(wreck: bool) -> ModelFile {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK], 0.85, 1),
        Material::glow(VOID),
        Material::lit(&[DARK, HOT], if wreck { 0.4 } else { 1.0 }, 0),
        Material::lit(&[VOID, DARK], 0.5, 0),
    ]);
    m.frustum([-11.0, -26.0, 0.0], [11.0, 26.0, 0.0], 2.5, 10.0, 0);
    m.loft(&Mesh::ring(10, 7.5, 10.2, 0.0), &Mesh::ring(10, 6.0, 10.2, 0.0), 1);
    m.poly(&Mesh::ring(10, 6.0, 10.1, 0.0), 1);
    for y in [-18.0, 18.0] {
        m.beam([-6.0, y, 10.4], [6.0, y, 10.4], 1.8, 1.2, 2);
    }
    for y in [-28.0, 28.0] {
        m.cuboid([-2.5, y - 2.0, 2.0], [2.5, y + 2.0, 5.0], 1);
    }
    if wreck {
        // Plates blown up and out, holes into the dark hull.
        m.poly(&[[-8.0, -14.0, 10.3], [-2.0, -14.0, 10.3], [-2.0, -8.0, 10.3], [-8.0, -8.0, 10.3]], 1);
        m.poly(&[[2.0, 8.0, 10.3], [8.0, 9.0, 10.3], [7.0, 14.0, 10.3]], 1);
        m.poly(&[[-8.0, -8.0, 10.5], [-2.0, -8.0, 10.5], [-1.0, -5.0, 16.0], [-9.0, -6.0, 15.0]], 3);
        m.poly(&[[8.0, 9.0, 10.5], [7.0, 14.0, 10.5], [12.0, 15.0, 14.0]], 3);
        let o = Orient { yaw: 14f64.to_radians(), pitch: 0.0, roll: 10f64.to_radians() };
        warp(&mut m, 0, |v| o.apply(v));
        return model("midboss.s3.sidewinder.car.wreck", (40, 64), &[], m);
    }
    model("midboss.s3.sidewinder.car", (40, 64), &[], m)
}

// ---------------------------------------------------------------------------------- TRESTLE TITAN

fn titan_materials(dead: bool) -> Vec<Material> {
    vec![
        // Girders: rust orange.
        Material::lit(&[VOID, DARK], 0.85, 1),
        // The boiler and the bogies: dark.
        Material::lit(&[VOID, DARK], 0.5, 0),
        // Hazard stripes on the girders.
        Material::lit(&[DARK, LIGHT], 0.9, 0),
        // Ore in the seams and the claws (dark once it is dead).
        Material::lit(&[DARK, HOT], if dead { 0.35 } else { 1.0 }, 0),
        Material::glow(VOID),
    ]
}

/// A box girder from `a` to `b` with a lattice of braces on its top (rust orange).
fn girder(m: &mut Mesh, a: V3, b: V3, w: f64, h: f64) {
    m.beam(a, b, w, h, 0);
    let (dx, dy, dz) = (b[0] - a[0], b[1] - a[1], b[2] - a[2]);
    let l = (dx * dx + dy * dy).sqrt();
    let n = (l / (w * 1.4)).max(1.0) as usize;
    let (px, py) = (-dy / l * w * 0.35, dx / l * w * 0.35);
    for i in 0..n {
        let (f0, f1) = (i as f64 / n as f64, (i + 1) as f64 / n as f64);
        let p = |f: f64, s: f64| -> V3 { [a[0] + dx * f + s * px, a[1] + dy * f + s * py, a[2] + dz * f + h / 2.0 + 0.3] };
        let s = if i % 2 == 0 { 1.0 } else { -1.0 };
        m.beam(p(f0, -s), p(f1, s), 0.9, 0.6, 4);
    }
}

/// The TRESTLE TITAN's body: a rust gantry straddling the yard's middle tracks on four bogies (on the
/// crane rails), two box girders across with hazard stripes and ore in their seams, A-frame legs, the
/// dark upright boiler between the girders (its door is its own sprite), the jibs' pivots at the lower
/// corners. `wreck`: the girders sag across the tracks, the crystal dark. 240 px across.
pub fn titan_body(wreck: bool) -> Mesh {
    let mut m = Mesh::new(titan_materials(wreck));
    let (top, gy) = (34.0, [22.0, -42.0]);
    let sag = |x: f64| -> f64 { if wreck { -18.0 * (1.0 - (x / BOGIE_X).powi(2)) } else { 0.0 } };
    // Bogies on the rails, and the legs up to the girders.
    for sx in [-1.0, 1.0] {
        let x = sx * BOGIE_X;
        for y in [gy[0] + 6.0, gy[1] - 6.0] {
            m.cuboid([x - 5.0, y - 14.0, 0.0], [x + 5.0, y + 14.0, 6.0], 1);
            m.beam([x, y, 5.0], [x - sx * 6.0, (gy[0] + gy[1]) / 2.0 + (y - (gy[0] + gy[1]) / 2.0) * 0.8, top + sag(x - sx * 6.0)], 4.0, 4.0, 0);
        }
        m.beam([x - sx * 3.0, gy[0] + 3.0, 16.0], [x - sx * 3.0, gy[1] - 3.0, 16.0], 2.0, 2.0, 0);
    }
    // The girders, each in segments (so a wreck can sag), striped at the ends.
    for &y in &gy {
        let n = 8;
        for i in 0..n {
            let (x0, x1) = (-BOGIE_X - 4.0 + i as f64 * (2.0 * BOGIE_X + 8.0) / n as f64, -BOGIE_X - 4.0 + (i + 1) as f64 * (2.0 * BOGIE_X + 8.0) / n as f64);
            girder(&mut m, [x0, y, top + sag(x0)], [x1, y, top + sag(x1)], 13.0, 8.0);
        }
        for sx in [-1.0, 1.0] {
            for k in 0..4 {
                let x = sx * (BOGIE_X - 6.0 - k as f64 * 5.0);
                m.poly(&[[x - 1.5, y - 6.2, top + 4.6 + sag(x)], [x + 1.5, y - 6.2, top + 4.6 + sag(x)], [x + 3.5, y + 6.2, top + 4.6 + sag(x)], [x + 0.5, y + 6.2, top + 4.6 + sag(x)]], 2);
            }
        }
        // Ore in the girder seams.
        for x in [-70.0, -32.0, 44.0, 86.0] {
            m.shard([x, y + 3.0, top + 3.0 + sag(x)], [x + 2.0, y + 6.0, top + 10.0 + sag(x)], 3.0, 3);
        }
    }
    // A walkway grating between the girders, either side of the boiler: plates with gaps.
    let mut x = -BOGIE_X + 10.0;
    while x < BOGIE_X - 10.0 {
        if x.abs() > 34.0 {
            let (a, b) = (x, x + 5.0);
            m.poly(&[[a, gy[1] + 6.0, top + 1.5 + sag(a)], [b, gy[1] + 6.0, top + 1.5 + sag(b)], [b, gy[0] - 6.0, top + 1.5 + sag(b)], [a, gy[0] - 6.0, top + 1.5 + sag(a)]], 0);
        }
        x += 7.0;
    }
    // Cross ties between the girders.
    for x in [-110.0, -60.0, 60.0, 110.0] {
        girder(&mut m, [x, gy[0], top + 1.0 + sag(x)], [x, gy[1], top + 1.0 + sag(x)], 5.0, 4.0);
    }
    // The jibs' pivot housings at the lower corners.
    for sx in [-1.0, 1.0] {
        let (px, py) = (sx * -PIVOT.0, PIVOT.1);
        m.loft(&Mesh::ring(8, 8.0, top - 2.0 + sag(px), 0.0).iter().map(|p| [p[0] + px, p[1] + py, p[2]]).collect::<Vec<_>>(), &Mesh::ring(8, 6.0, top + 6.0 + sag(px), 0.0).iter().map(|p| [p[0] + px, p[1] + py, p[2]]).collect::<Vec<_>>(), 1);
        m.poly(&Mesh::ring(8, 6.0, top + 6.0 + sag(px), 0.0).iter().map(|p| [p[0] + px, p[1] + py, p[2]]).collect::<Vec<_>>(), 1);
    }
    // The boiler: an upright riveted drum between the girders, a firebox hatch on top.
    let (bx, by) = BOILER;
    let drum = |r: f64, z: f64| -> Vec<V3> { Mesh::ring(16, r, z, 0.1).iter().map(|p| [p[0] + bx, p[1] + by, p[2]]).collect() };
    let bt = top + 8.0 + if wreck { -10.0 } else { 0.0 };
    m.loft(&drum(30.0, 4.0), &drum(30.0, bt), 1);
    m.loft(&drum(30.0, bt), &drum(26.0, bt + 4.0), 1);
    m.poly(&drum(26.0, bt + 4.0), 1);
    for z in [12.0, 24.0] {
        m.loft(&drum(30.6, z), &drum(30.6, z + 1.5), 1);
    }
    for k in 0..6 {
        let a = k as f64 * TAU / 6.0 + 0.4;
        m.shard([bx + 27.0 * a.cos(), by + 27.0 * a.sin(), bt + 3.0], [bx + 31.0 * a.cos(), by + 31.0 * a.sin(), bt + 9.0], 2.0, 3);
    }
    m
}

/// A claw on its jib at slew `i` of [`SLEWS`] and drop `d` of [`DROPS`]: the left one (the right is its
/// mirror image), built about its pivot (the frame's centre). The jib swings through `SLEW_DEG` either
/// side of straight down the screen; the claw hangs from the jib's tip on its cable, swung back toward
/// the pivot and tilted while raised, straight under the tip and level at the bottom of its drop (from
/// above the cable shortens in view and the claw's shading turns).
pub fn claw_mesh(i: usize, d: usize, dead: bool) -> Mesh {
    let mut m = Mesh::new(titan_materials(dead));
    let a = (-SLEW_DEG + 2.0 * SLEW_DEG * i as f64 / (SLEWS - 1) as f64).to_radians();
    let (dx, dy) = (a.sin(), -a.cos());
    let (zp, zt) = (40.0, 38.0);
    let tip = [dx * JIB, dy * JIB, zt];
    girder(&mut m, [0.0, 0.0, zp], tip, 10.0, 6.0);
    // Hazard stripes near the tip.
    for k in 0..3 {
        let f = 0.82 + k as f64 * 0.05;
        let (cx, cy) = (dx * JIB * f, dy * JIB * f);
        m.poly(&[[cx - dy * 3.6, cy + dx * 3.6, zt + 3.2], [cx + dy * 3.6, cy - dx * 3.6, zt + 3.2], [cx + dy * 3.6 + dx * 1.5, cy - dx * 3.6 + dy * 1.5, zt + 3.2], [cx - dy * 3.6 + dx * 1.5, cy + dx * 3.6 + dy * 1.5, zt + 3.2]], 2);
    }
    // Where the claw hangs: swung back toward the pivot while raised.
    let f = d as f64 / (DROPS - 1) as f64;
    let back = 10.0 * (1.0 - f);
    let (cx, cy, cz) = (tip[0] - dx * back, tip[1] - dy * back, 30.0 - 26.0 * f);
    m.beam([tip[0], tip[1], zt - 1.0], [cx, cy, cz + 9.0], 1.0, 1.0, 1);
    // The claw: a hub, three fingers curling in, ore in its palm; tilted toward the pivot while raised.
    let from = m.verts.len();
    m.loft(&Mesh::ring(8, 9.0, 6.0, 0.0), &Mesh::ring(8, 6.0, 14.0, 0.0), 1);
    m.poly(&Mesh::ring(8, 6.0, 14.0, 0.0), 1);
    for k in 0..3 {
        let b = k as f64 * TAU / 3.0 + 0.5;
        let (c, s) = (b.cos(), b.sin());
        m.beam([7.0 * c, 7.0 * s, 10.0], [19.0 * c, 19.0 * s, 7.0], 5.0, 4.0, 0);
        m.beam([19.0 * c, 19.0 * s, 7.0], [15.0 * c - 4.0 * s, 15.0 * s + 4.0 * c, 0.0], 4.0, 3.5, 0);
        m.poly(&[[18.0 * c - 2.0 * s, 18.0 * s + 2.0 * c, 9.3], [18.0 * c + 2.0 * s, 18.0 * s - 2.0 * c, 9.3], [15.0 * c + 2.0 * s, 15.0 * s - 2.0 * c, 9.8], [15.0 * c - 2.0 * s, 15.0 * s + 2.0 * c, 9.8]], 2);
    }
    for k in 0..3 {
        let b = k as f64 * TAU / 3.0 + 1.55;
        m.shard([5.0 * b.cos(), 5.0 * b.sin(), 4.0], [9.0 * b.cos(), 9.0 * b.sin(), 11.0], 3.2, 3);
    }
    let lean = (1.0 - f) * 0.5;
    warp(&mut m, from, |v| {
        // Lean the claw toward the pivot (about the axis across the jib).
        let (u, w) = (v[0] * dx + v[1] * dy, v[2]);
        let (u2, w2) = (u * lean.cos() - w * lean.sin(), u * lean.sin() + w * lean.cos());
        let side = -v[0] * dy + v[1] * dx;
        [cx + u2 * dx - side * dy, cy + u2 * dy + side * dx, cz + w2]
    });
    m
}

/// The mirror image of `m` (x to -x): the right claw from the left.
fn mirrored(mut m: Mesh) -> Mesh {
    warp(&mut m, 0, |v| [-v[0], v[1], v[2]]);
    m
}

/// The boiler door at pose `k` of [`DOOR_POSES`]: the drum's top rim and a round hatch in two leaves that
/// swing up and out on hinges over a `hot` firebox.
pub fn boiler_door(k: usize) -> Mesh {
    let mut m = Mesh::new(titan_materials(false));
    m.loft(&Mesh::ring(16, 26.0, 0.0, 0.1), &Mesh::ring(16, 19.0, 0.0, 0.1), 1);
    m.poly(&Mesh::ring(16, 19.0, -0.5, 0.1), 3);
    let ang = [0.0f64, 35.0, 65.0, 88.0][k.min(DOOR_POSES - 1)].to_radians();
    for sx in [-1.0, 1.0] {
        let hinge = sx * 19.0;
        let n = 6;
        // A half disc (its chord on the middle line), turned up about the hinge at its outer edge.
        let pts: Vec<V3> = (0..=n)
            .map(|j| {
                let b = -PI / 2.0 + j as f64 * PI / n as f64;
                let (x, y) = (sx * 18.5 * b.cos(), 18.5 * b.sin());
                let r = hinge - x;
                [hinge - r * ang.cos(), y, 0.6 + r.abs() * ang.sin()]
            })
            .collect();
        m.poly(&pts, 1);
        m.beam([hinge, -15.0, 0.8], [hinge, 15.0, 0.8], 2.0, 1.5, 0);
    }
    m
}

/// The claw's shadow under its drop, size `k` of [`DROPS`]: a void blob closing in as the claw comes down.
pub fn claw_shadow(k: usize) -> SpriteImage {
    let r = 11.0 - 2.0 * k as f64;
    stamp(16, 24, VOID, |x, y| x * x + y * y * 1.2 <= r * r)
}

// ------------------------------------------------------------------------------------------ sets

/// Every S3 hand model's base pose (for `hand-models`: `assets/models/<id>.mesh`).
pub fn all() -> Vec<(String, ModelFile)> {
    let mut out: Vec<ModelFile> = vec![
        dust_devil(),
        rail_car(),
        turret(),
        sandcrawler(),
        vulture(false),
        lancer(),
        sidewinder_engine(),
        sidewinder_car(false),
        sidewinder_car(true),
        model("boss.s3.trestle_titan", (240, 176), &[], titan_body(false)),
        model("boss.s3.trestle_titan.wreck", (240, 176), &[], titan_body(true)),
        model("boss.s3.trestle_titan.claw", (176, 176), &[], claw_mesh(SLEWS / 2, 0, false)),
    ];
    out.extend((0..=TOWER_PULSES).map(water_tower));
    out.extend((0..DOOR_POSES).map(|k| model(&format!("boss.s3.trestle_titan.boiler.open{k}"), (80, 80), &[], boiler_door(k))));
    out.into_iter().map(|m| (m.name.clone(), m)).collect()
}

/// Every S3 frame set in the level image's order, rendered under S3's sun: (name, frames).
pub fn roster() -> Vec<(&'static str, Vec<SpriteImage>)> {
    let mut out: Vec<(&'static str, Vec<SpriteImage>)> = Vec::new();
    out.push(("dust_devil", dust_devil().render_all_lit(&SUN)));
    out.push(("rail_car", rail_car().render_all_lit(&SUN)));
    let turret: Vec<SpriteImage> = [false, true].iter().flat_map(|&armed| (0..YAWS).map(move |i| shot(&turret_mesh(yaw_of(i), armed), (24, 24), Orient::default()))).collect();
    out.push(("turret", turret));
    let down = Orient { yaw: PI, pitch: 0.0, roll: 0.0 };
    let mut crawler: Vec<SpriteImage> = (0..RISE_POSES).map(|k| shot(&sandcrawler_mesh(k, false), (40, 56), down)).collect();
    for armed in [false, true] {
        let m = sandcrawler_mesh(RISE_POSES - 1, armed);
        crawler.extend((0..YAWS).map(|i| shot(&m, (40, 56), Orient { yaw: yaw_of(i), pitch: 0.0, roll: 0.0 })));
    }
    out.push(("sandcrawler", crawler));
    out.push(("dust_ring", (0..RING_SIZES).map(dust_ring).collect()));
    out.push(("water_tower", (0..=TOWER_PULSES).map(|k| water_tower(k).render_all_lit(&SUN).remove(0)).collect()));
    let mut vul = vulture(false).render_all_lit(&SUN);
    vul.extend(vulture(true).render_all_lit(&SUN));
    out.push(("vulture", vul));
    out.push(("lancer", lancer().render_all_lit(&SUN)));
    out.push(("wasp", crate::hand::wasp().render_all_lit(&SUN)));
    let mut crawl = crate::hand::crawler().render_all_lit(&SUN);
    crawl.extend(crate::hand::crawler_armed(true).render_all_lit(&SUN));
    out.push(("crawler", crawl));
    let engine = sidewinder_engine();
    let mut eng: Vec<SpriteImage> = sways(SWAYS, 4.0).iter().map(|&r| shot(&engine.mesh, (48, 80), rolled(r))).collect();
    eng.extend(engine.render_all_lit(&SUN));
    out.push(("sidewinder", eng));
    let car = sidewinder_car(false);
    let mut cars: Vec<SpriteImage> = sways(SWAYS, 4.0).iter().map(|&r| shot(&car.mesh, (40, 64), rolled(r))).collect();
    cars.push(sidewinder_car(true).render_all_lit(&SUN).remove(0));
    out.push(("sidewinder_car", cars));
    let body = titan_body(false);
    let mut titan: Vec<SpriteImage> = sways(TITAN_SWAYS, 3.0).iter().map(|&r| shot(&body, (240, 176), rolled(r))).collect();
    titan.push(shot(&titan_body(true), (240, 176), Orient::default()));
    out.push(("titan", titan));
    let claws = |right: bool| -> Vec<SpriteImage> {
        (0..SLEWS)
            .flat_map(|i| (0..DROPS).map(move |d| (i, d)))
            .map(|(i, d)| {
                let m = claw_mesh(i, d, false);
                shot(&if right { mirrored(m) } else { m }, (176, 176), Orient::default())
            })
            .collect()
    };
    out.push(("claw_left", claws(false)));
    out.push(("claw_right", claws(true)));
    out.push(("boiler", (0..DOOR_POSES).map(|k| shot(&boiler_door(k), (80, 80), Orient::default())).collect()));
    out.push(("claw_shadow", (0..DROPS).map(claw_shadow).collect()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every S3 frame keeps clear of its canvas edge and draws something; the frame counts are the
    /// bible's `orient`.
    #[test]
    fn the_s3_roster_fits_its_canvases() {
        let roster = roster();
        let mut bad = Vec::new();
        for (name, frames) in &roster {
            for (i, img) in frames.iter().enumerate() {
                let (w, h) = (img.w, img.h);
                let edge = (0..w).any(|x| img.get(x, 0).is_some() || img.get(x, h - 1).is_some())
                    || (0..h).any(|y| img.get(0, y).is_some() || img.get(w - 1, y).is_some());
                if edge || img.pixels.iter().all(|p| p.is_none()) {
                    bad.push(format!("{name}#{i}"));
                }
            }
        }
        assert!(bad.is_empty(), "clipped or empty: {bad:?}");
        let n: Vec<(&str, usize)> = roster.iter().map(|(k, f)| (*k, f.len())).collect();
        assert_eq!(
            n,
            [
                ("dust_devil", 8), ("rail_car", 16), ("turret", 32), ("sandcrawler", 36), ("dust_ring", 4), ("water_tower", 5),
                ("vulture", 10), ("lancer", 16), ("wasp", 48), ("crawler", 32), ("sidewinder", 19), ("sidewinder_car", 4),
                ("titan", 5), ("claw_left", 64), ("claw_right", 64), ("boiler", 4), ("claw_shadow", 4)
            ]
        );
    }

    /// Light belongs to the player (the yellow CARACARA): no S3 enemy is more than 15 % `light`.
    #[test]
    fn s3_enemies_keep_light_for_highlights() {
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

    /// The right claw is the left one's mirror: at mirrored slews their silhouettes match.
    #[test]
    fn the_claws_sweep_in_mirror() {
        for i in [0, 5, SLEWS - 1] {
            let (l, r) = (shot(&claw_mesh(i, 1, false), (176, 176), Orient::default()), shot(&mirrored(claw_mesh(i, 1, false)), (176, 176), Orient::default()));
            let (w, h) = (l.w, l.h);
            let mut diff = 0;
            for y in 0..h {
                for x in 0..w {
                    diff += (l.get(x, y).is_some() != r.get(w - 1 - x, y).is_some()) as usize;
                }
            }
            let area = l.pixels.iter().flatten().count();
            assert!(diff * 10 < area, "slew {i}: {diff} of {area} pixels off the mirror");
        }
    }
}
