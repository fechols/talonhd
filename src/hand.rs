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

/// Every hand model: (bible id, model).
pub fn all() -> Vec<(&'static str, ModelFile)> {
    vec![("boss.s1.gantry_queen", gantry_queen())]
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
    const CONCRETE: u8 = 0;
    const GLASS: u8 = 1;
    const CRYSTAL: u8 = 2;
    const HAZARD: u8 = 3;
    const GLOW: u8 = 4;
    const STEEL: u8 = 5;
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 0.58, 1), // concrete: flat tops dark, light-facing slopes catch light
        Material::lit(&[VOID, DARK, HOT], 0.5, 0),    // smoked glass: dark, glints hot toward the light
        Material::lit(&[DARK, HOT], 1.0, 0),          // Lattice crystal
        Material::lit(&[VOID, DARK, HOT], 0.72, 1),   // weapons: hazard-striped, hot
        Material::glow(HOT),                          // muzzles, beacon
        Material::lit(&[VOID, DARK, LIGHT], 0.5, 1),  // legs and struts
    ]);
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

    // Cannon arms: a beam out to each side, an elbow block, twin barrels pointing at the player (-y).
    for sx in [-1.0, 1.0] {
        m.beam([sx * 170.0, 0.0, 160.0], [sx * 400.0, -10.0, 150.0], 46.0, 34.0, HAZARD);
        m.frustum([sx * 400.0 - 40.0, -50.0, 120.0], [sx * 400.0 + 40.0, 30.0, 120.0], 10.0, 176.0, STEEL);
        for dx in [-18.0, 18.0] {
            let x = sx * 400.0 + dx;
            m.tube([x, -40.0, 160.0], [x, -210.0, 160.0], 11.0, 6, HAZARD, GLOW);
        }
        m.shard([sx * 300.0, -6.0, 172.0], [sx * 330.0, 40.0, 215.0], 20.0, CRYSTAL);
    }

    // Four legs splayed sideways: hip on the shaft, knee hump, foot pad; crystal plates on the knees.
    for (ax, ay) in [(1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)] {
        let a: f64 = 0.50; // ~29° off the x axis
        let dir = [ax * a.cos(), ay * a.sin()];
        let at = |r: f64, z: f64| -> V3 { [dir[0] * r, dir[1] * r, z] };
        m.beam(at(60.0, 20.0), at(300.0, 90.0), 44.0, 38.0, STEEL);
        m.beam(at(300.0, 90.0), at(470.0, -70.0), 38.0, 34.0, STEEL);
        let f = at(470.0, -80.0);
        m.frustum([f[0] - 34.0, f[1] - 30.0, -110.0], [f[0] + 34.0, f[1] + 30.0, -110.0], 10.0, -70.0, CONCRETE);
        m.shard(at(300.0, 104.0), at(330.0, 150.0), 26.0, CRYSTAL);
    }

    // Authored at full size, shown at 0.6: the crown about a screen wide, legs and cannons nearly two.
    // Bigger than the screen, still readable as one machine.
    for v in m.verts.iter_mut() {
        *v = v.map(|c| c * QUEEN_SCALE);
    }
    model("boss.s1.gantry_queen", (512, 512), &[("roll", 49.0), ("roll_max_deg", 24.0)], m)
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let n = q.mesh.tris.len();
        assert!((500..=4000).contains(&n), "{n} triangles");
    }
}
