//! The S1 BREAKWATER level (host only), built as 3D geometry by the art director and rendered offline under
//! the stage's own light (`docs/story/hd/02-ladder.md`: S1 morning, the light its sprites use) with cast
//! shadows: 26 screens bottom to top, as the base bible lays the stage out
//! (`docs/story/04-levels/s1-breakwater.md`): open sea from the carrier's wake, the SEA WALL, the docks with
//! the centre channel, the airfield with its two runways up to the hangar apron.
//!
//! Art direction (`docs/story/hd/01-art-direction.md` §9, the background stays down): ground materials are
//! `void` and `dark` only; `light` only for surf, wake and runway paint (≤ 10 %), `hot` only where the
//! Lattice has grown into the ground. World units: hires px across (x 0..320), stage lines up (`u`, 0 at the
//! stage start, the bottom of the panorama), z up.

use crate::bake::Panorama;
use crate::demo;
use crate::raster3d::{self, Light, Material, Mesh, ShadowMap, V3, View};
use crate::render::Palette;

const VOID: u8 = 0;
const DARK: u8 = 1;
const LIGHT: u8 = 2;
const HOT: u8 = 3;

pub const PALETTE: Palette = Palette::new(6, 12, 1, 3);
/// The stage's time of day (the ladder's S1 `light`): the morning sun every S1 sprite is lit by.
pub const SUN: Light = raster3d::MORNING;

const WIDTH: f64 = 320.0;
/// Section boundaries (stage lines): sea 0-9 screens, docks 9-17, airfield 17-26.
const SEA_END: f64 = 9.0 * 200.0;
const DOCKS_END: f64 = 17.0 * 200.0;
/// The docks' centre channel (hires px), where the KEELBREAKER steams.
const CHANNEL: (f64, f64) = (116.0, 204.0);

/// Material ids (`materials()` order).
const WATER: u8 = 0;
const FOAM: u8 = 1;
const ROCK: u8 = 2;
const CONCRETE: u8 = 3;
const STEEL: u8 = 4;
const PAINT: u8 = 5;
const CRYSTAL: u8 = 6;
const GROUND: u8 = 7;
const SAND: u8 = 8;
const ROOF: u8 = 9;

fn materials() -> Vec<Material> {
    vec![
        // Water: void when flat; the swells' sun-facing slopes catch a dark checker.
        Material::lit(&[VOID, DARK], 0.25, 0),
        // Surf, wake, foam: white.
        Material::glow(LIGHT),
        // Rock and concrete blocks: lit slopes solid dark, shade to void.
        Material::lit(&[VOID, DARK], 0.6, 0),
        // Flat concrete: the void/dark checker; void in shadow.
        Material::lit(&[VOID, DARK], 0.5, 0),
        // Steel (cranes, containers, tanks, hangar roofs): solid dark tops.
        Material::lit(&[VOID, DARK], 0.78, 0),
        // Paint and markings: white.
        Material::glow(LIGHT),
        // The Lattice in the cracks.
        Material::glow(HOT),
        // Open ground (grass and dirt): the dark checker, speckled by the shadows of its bushes and mounds.
        Material::lit(&[VOID, DARK], 0.4, 0),
        // Sand ringing the islets: solid dark.
        Material::lit(&[VOID, DARK], 0.9, 0),
        // Ribbed hangar roofs: the curve bands from solid dark on the sunny side to void in its shade.
        Material::lit(&[VOID, DARK], 0.74, 0),
    ]
}

struct Builder {
    m: Mesh,
    rng: u32,
}

impl Builder {
    fn next(&mut self) -> u32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        self.rng
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (self.next() % 10_000) as f64 / 10_000.0 * (hi - lo)
    }
    /// A fresh local mesh with the level's materials (build at the origin, then `place`).
    fn local() -> Mesh {
        Mesh::new(materials())
    }
    fn place(&mut self, part: &Mesh, at: V3) {
        self.m.add(part, at, 0);
    }
    /// A flat quad decal (paint, foam, crystal) lying at height `z`.
    fn decal(&mut self, x0: f64, u0: f64, x1: f64, u1: f64, z: f64, mat: u8) {
        self.m.poly(&[[x0, u0, z], [x1, u0, z], [x1, u1, z], [x0, u1, z]], mat);
    }
    /// A crystal crack: a jagged polyline of thin hot quads at height `z`, mostly running up the stage.
    fn crack(&mut self, mut x: f64, mut u: f64, segs: usize, z: f64) {
        for _ in 0..segs {
            let (dx, du) = (self.range(-6.0, 6.0), self.range(2.0, 7.0));
            let (nx, nu) = (x + dx, u + du);
            let (w, h) = (1.2, 0.6);
            self.m.poly(&[[x - w, u, z], [x + w, u, z], [nx + w, nu + h, z], [nx - w, nu + h, z]], CRYSTAL);
            x = nx;
            u = nu;
        }
    }
}

// --------------------------------------------------------------------------------------------- the sea

/// Swells: long ridges across the stage, faceted so the sun-facing slopes catch a dark checker.
fn swells(b: &mut Builder, u0: f64, u1: f64, x0: f64, x1: f64) {
    let pitch = 13.0;
    let seg = 32.0;
    let mut u = u0;
    while u < u1 {
        let mut x = x0;
        let crest = b.range(0.6, 3.6);
        while x < x1 {
            let xn = (x + seg).min(x1);
            let (ca, cb) = (u + pitch * 0.55 + b.range(-2.0, 2.0), u + pitch * 0.55 + b.range(-2.0, 2.0));
            // Back slope (rising from the trough below), front slope (falling to the next trough).
            b.m.poly(&[[x, u, 0.0], [xn, u, 0.0], [xn, cb, crest], [x, ca, crest]], WATER);
            b.m.poly(&[[x, ca, crest], [xn, cb, crest], [xn, u + pitch, 0.0], [x, u + pitch, 0.0]], WATER);
            x = xn;
        }
        u += pitch;
    }
    // Whitecaps: specks of foam on a few crests.
    let n = ((u1 - u0) / 7.0) as usize;
    for _ in 0..n {
        let (x, u) = (b.range(x0, x1), b.range(u0, u1));
        let w = b.range(1.0, 3.0);
        b.decal(x, u, x + w, u + 1.0, 3.0, FOAM);
    }
}

/// A rocky islet: irregular stepped rings to a flat cap, ringed by sand and broken surf.
fn islet(b: &mut Builder, cx: f64, cu: f64, r: f64) {
    let n = 9;
    let jit: Vec<f64> = (0..2 * n).map(|_| b.range(0.75, 1.25)).collect();
    let ring = |s: f64, z: f64| -> Vec<V3> {
        (0..n)
            .map(|k| {
                let a = k as f64 * std::f64::consts::TAU / n as f64;
                [cx + a.cos() * r * s * jit[k], cu + a.sin() * r * s * jit[k + n], z]
            })
            .collect()
    };
    let (sand, a, m, t) = (ring(1.25, 0.6), ring(1.0, 2.0), ring(0.7, 9.0), ring(0.35, 13.0));
    b.m.loft(&sand, &a, SAND);
    b.m.loft(&a, &m, ROCK);
    b.m.loft(&m, &t, ROCK);
    b.m.poly(&t, ROCK);
    // Surf: foam flecks just outside the sand.
    for k in 0..28 {
        let ang = k as f64 / 28.0 * std::f64::consts::TAU + b.range(-0.1, 0.1);
        let rr = r * b.range(1.35, 1.6);
        let (x, u) = (cx + ang.cos() * rr, cu + ang.sin() * rr);
        let w = b.range(1.5, 3.5);
        b.decal(x, u, x + w, u + 1.2, 3.2, FOAM);
    }
}

fn sea(b: &mut Builder) {
    swells(b, -20.0, SEA_END, 0.0, WIDTH);
    // The MERIDIAN's wake: a white V fading behind the start position, broken into flecks.
    for i in 0..260 {
        let u = i as f64;
        let spread = (260.0 - u) * 120.0 / 260.0;
        let keep = 1.0 - u / 300.0;
        for side in [-1.0, 1.0] {
            if b.range(0.0, 1.0) < keep * 0.8 {
                let x = 160.0 + side * spread + b.range(-4.0, 4.0);
                let w = b.range(1.5, 4.0);
                b.decal(x, u, x + w, u + 1.0, 3.4, FOAM);
            }
        }
    }
    for &(x, u, r) in &[(44.0, 520.0, 26.0), (276.0, 900.0, 34.0), (60.0, 1300.0, 20.0), (256.0, 1560.0, 24.0), (120.0, 1180.0, 12.0)] {
        islet(b, x, u, r);
    }
    // Marker buoys.
    for &(x, u) in &[(208.0, 700.0), (104.0, 1650.0)] {
        let mut p = Builder::local();
        p.tube([0.0, 0.0, 0.0], [0.0, 0.0, 6.0], 2.5, 8, STEEL, FOAM);
        b.place(&p, [x, u, 0.0]);
    }
}

/// The SEA WALL: interlocking blocks across the stage, surf bursting on the sea side, a lighthouse stub
/// with a crystal lamp at the right end, calm water beyond.
fn sea_wall(b: &mut Builder) {
    let u = SEA_END + 14.0;
    let mut k = 0;
    let mut x = -6.0;
    while x < WIDTH + 6.0 {
        let du = if k % 2 == 0 { 2.0 } else { -3.0 };
        let top = 10.0 + (k % 3) as f64 * 3.0;
        b.m.frustum([x - 8.0, u - 10.0 + du, 0.0], [x + 8.0, u + 10.0 + du, 0.0], 3.0, top, ROCK);
        x += 14.0;
        k += 1;
    }
    b.m.frustum([-8.0, u - 6.0, 0.0], [WIDTH + 8.0, u + 6.0, 0.0], 1.0, 8.0, CONCRETE);
    let mut l = Builder::local();
    l.tube([0.0, 0.0, 0.0], [0.0, 0.0, 30.0], 9.0, 10, ROCK, ROCK);
    l.tube([0.0, 0.0, 30.0], [0.0, 0.0, 34.0], 4.0, 8, CRYSTAL, CRYSTAL);
    b.place(&l, [296.0, u, 0.0]);
    // Surf: patches of foam on the sea side.
    let mut x = 0.0;
    while x < WIDTH {
        if ((x / 24.0) as i32) % 3 != 1 {
            for _ in 0..5 {
                let (fx, fu) = (x + b.range(0.0, 12.0), u - 12.0 - b.range(0.0, 6.0));
                let w = b.range(1.5, 4.0);
                b.decal(fx, fu, fx + w, fu + 1.0, 4.0, FOAM);
            }
        }
        x += 6.0;
    }
}

// --------------------------------------------------------------------------------------------- the docks

fn container_block(b: &mut Builder, x: f64, u: f64, base: f64) {
    let mut m = Builder::local();
    for row in 0..3 {
        for col in 0..2 {
            let (cx, cy) = (col as f64 * 18.0 - 9.0, row as f64 * 8.0 - 8.0);
            let h = 5.0 + ((row + col) % 2) as f64 * 5.0;
            m.frustum([cx - 8.0, cy - 3.0, 0.0], [cx + 8.0, cy + 3.0, 0.0], 0.8, h, STEEL);
        }
    }
    b.place(&m, [x, u, base]);
}

fn fuel_tank(b: &mut Builder, x: f64, u: f64, base: f64) {
    let mut m = Builder::local();
    m.tube([0.0, 0.0, 0.0], [0.0, 0.0, 12.0], 10.0, 12, STEEL, STEEL);
    m.frustum([-3.0, -3.0, 12.0], [3.0, 3.0, 12.0], 1.0, 14.0, STEEL);
    b.place(&m, [x, u, base]);
}

/// A gantry crane on the quay edge, its boom swung over the water (`dir` -1 left, 1 right); long morning
/// shadows; crystal veins climbing its legs.
fn crane(b: &mut Builder, x: f64, u: f64, base: f64, dir: f64) {
    let mut m = Builder::local();
    for (lx, ly) in [(-8.0, -8.0), (8.0, -8.0), (-8.0, 8.0), (8.0, 8.0)] {
        m.cuboid([lx - 2.0, ly - 2.0, 0.0], [lx + 2.0, ly + 2.0, 34.0], STEEL);
    }
    m.beam([0.0, -10.0, 36.0], [0.0, 10.0, 36.0], 18.0, 4.0, STEEL);
    m.beam([0.0, 0.0, 40.0], [dir * 64.0, 0.0, 38.0], 5.0, 4.0, STEEL);
    b.place(&m, [x, u, base]);
    for (lx, lu) in [(-8.0, -8.0), (8.0, 8.0)] {
        let segs = (b.next() % 3 + 2) as usize;
        b.crack(x + lx, u + lu, segs, base + 0.3);
    }
}

/// The docks: quays either side of the centre channel with basins cut into them, piers on piles,
/// containers, fuel tanks and cranes; hazard paint on the channel edges; crystal splitting the slabs.
fn docks(b: &mut Builder) {
    let (u0, u1) = (SEA_END + 30.0, DOCKS_END);
    let quay_z = 6.0;
    // Calm water in the channel and basins: low swells.
    swells(b, u0, u1, CHANNEL.0, CHANNEL.1);
    // Basins alternate left and right; quays are the slabs between them.
    let mut basins: Vec<(bool, f64, f64)> = Vec::new();
    let mut u = u0 + 60.0;
    let mut left = true;
    while u + 180.0 < u1 {
        let len = b.range(140.0, 200.0);
        basins.push((left, u, u + len));
        u += len + b.range(140.0, 220.0);
        left = !left;
    }
    for side in [true, false] {
        let (x0, x1) = if side { (-6.0, CHANNEL.0) } else { (CHANNEL.1, WIDTH + 6.0) };
        // The quay's free stretches between this side's basins.
        let mut at = u0;
        let mut cuts: Vec<(f64, f64)> = basins.iter().filter(|bs| bs.0 == side).map(|bs| (bs.1, bs.2)).collect();
        cuts.push((u1, u1));
        for (c0, c1) in cuts {
            if c0 > at + 4.0 {
                b.m.frustum([x0, at, 0.0], [x1, c0, 0.0], 0.0, quay_z, CONCRETE);
            }
            at = c1;
        }
    }
    for &(side, c0, c1) in &basins {
        // The basin: water, a narrow quay strip on its outer edge, a pier on piles down its middle.
        let (bx0, bx1) = if side { (-6.0, CHANNEL.0 - 24.0) } else { (CHANNEL.1 + 24.0, WIDTH + 6.0) };
        swells(b, c0, c1, bx0, bx1);
        let (sx0, sx1) = if side { (CHANNEL.0 - 24.0, CHANNEL.0) } else { (CHANNEL.1, CHANNEL.1 + 24.0) };
        b.m.frustum([sx0, c0, 0.0], [sx1, c1, 0.0], 0.0, quay_z, CONCRETE);
        let px = if side { 40.0 } else { 280.0 };
        b.m.cuboid([px - 3.0, c0, 3.0], [px + 3.0, c1, 4.5], STEEL);
        let mut pu = c0 + 4.0;
        while pu < c1 {
            b.m.cuboid([px - 4.5, pu, 0.0], [px - 3.0, pu + 1.5, 4.0], ROCK);
            b.m.cuboid([px + 3.0, pu, 0.0], [px + 4.5, pu + 1.5, 4.0], ROCK);
            pu += 10.0;
        }
        // White bollards on the strip and a crane at the basin mouth.
        let mut bu = c0 + 6.0;
        while bu < c1 {
            let bx = if side { sx0 + 3.0 } else { sx1 - 3.0 };
            b.decal(bx - 1.0, bu, bx + 1.0, bu + 1.5, quay_z + 0.2, PAINT);
            bu += 22.0;
        }
        crane(b, if side { sx0 + 12.0 } else { sx1 - 12.0 }, (c0 + c1) / 2.0, quay_z, if side { -1.0 } else { 1.0 });
    }
    // Quay furniture where the slabs are free of basins.
    let mut u = u0 + 24.0;
    while u < u1 - 24.0 {
        for x in [22.0, 70.0, 250.0, 298.0] {
            let side = x < 160.0;
            let in_basin = basins.iter().any(|&(s, c0, c1)| s == side && u > c0 - 16.0 && u < c1 + 16.0);
            if in_basin {
                continue;
            }
            match b.next() % 5 {
                0 | 1 => container_block(b, x, u, quay_z),
                2 => fuel_tank(b, x, u, quay_z),
                _ => {}
            }
        }
        u += 40.0;
    }
    // Hazard paint dashes on the channel edges, crystal in the slabs.
    let mut u = u0;
    while u < u1 {
        for x in [CHANNEL.0 - 2.0, CHANNEL.1] {
            b.decal(x, u, x + 2.0, u + 3.0, quay_z + 0.2, PAINT);
        }
        u += 8.0;
    }
    for _ in 0..40 {
        let left = b.next() % 2 == 0;
        let (x, u) = (if left { b.range(4.0, CHANNEL.0 - 6.0) } else { b.range(CHANNEL.1 + 6.0, WIDTH - 4.0) }, b.range(u0, u1));
        let in_basin = basins.iter().any(|&(s, c0, c1)| s == left && u > c0 - 4.0 && u < c1 + 4.0);
        if !in_basin {
            let segs = (b.next() % 4 + 2) as usize;
            b.crack(x, u, segs, quay_z + 0.3);
        }
    }
}

// ------------------------------------------------------------------------------------------ the airfield

/// A 3×5 digit painted on the ground, `s` hires px per cell (`flip`: upside down, read from the far end).
fn paint_digit(b: &mut Builder, d: u8, x: f64, u: f64, s: f64, z: f64, flip: bool) {
    const FONT: [[u8; 5]; 10] = [
        [7, 5, 5, 5, 7], [2, 6, 2, 2, 7], [7, 1, 7, 4, 7], [7, 1, 3, 1, 7], [5, 5, 7, 1, 1],
        [7, 4, 7, 1, 7], [7, 4, 7, 5, 7], [7, 1, 1, 2, 2], [7, 5, 7, 5, 7], [7, 5, 7, 1, 7],
    ];
    for row in 0..5 {
        for col in 0..3 {
            if FONT[d as usize][row] >> (2 - col) & 1 == 1 {
                let (r, c) = if flip { (4 - row, 2 - col) } else { (row, col) };
                let (x0, u0) = (x + c as f64 * s, u - (r + 1) as f64 * s * 1.6);
                b.decal(x0, u0, x0 + s, u0 + s * 1.6, z, PAINT);
            }
        }
    }
}

/// A curved hangar roof of ribbed metal, lengthwise along the stage.
fn hangar(b: &mut Builder, x: f64, u: f64) {
    let mut m = Builder::local();
    let (w, l, n) = (34.0, 30.0, 8);
    let arc = |k: usize| -> (f64, f64) {
        let a = std::f64::consts::PI * k as f64 / n as f64;
        (-w * a.cos(), 26.0 * a.sin())
    };
    for k in 0..n {
        let (x0, z0) = arc(k);
        let (x1, z1) = arc(k + 1);
        m.poly(&[[x0, -l, z0], [x1, -l, z1], [x1, l, z1], [x0, l, z0]], ROOF);
    }
    // Gable ends.
    let mut front: Vec<V3> = (0..=n).map(|k| { let (ax, az) = arc(k); [ax, l, az] }).collect();
    front.reverse();
    m.poly(&front, CONCRETE);
    b.place(&m, [x, u, 0.0]);
}

fn airfield(b: &mut Builder) {
    let (u0, u1) = (DOCKS_END, demo::PANO_PERIOD as f64);
    // The airfield's own seawall.
    b.m.frustum([-6.0, u0, 0.0], [WIDTH + 6.0, u0 + 16.0, 0.0], 1.0, 8.0, CONCRETE);
    let mut x = 0.0;
    while x < WIDTH {
        b.decal(x, u0 + 16.0, x + 2.0, u0 + 17.0, 8.2, PAINT);
        x += 4.0;
    }
    // Rolling ground: a height field of 24-px cells, two facets each.
    let cell = 12.0;
    let (nx, nu) = ((WIDTH / cell) as usize + 2, ((u1 - u0 - 16.0) / cell) as usize + 2);
    let mut hgt = vec![0.0; nx * nu];
    for v in hgt.iter_mut() {
        *v = b.range(0.0, 0.5);
    }
    for j in 0..nu - 1 {
        for i in 0..nx - 1 {
            let p = |i: usize, j: usize| -> V3 { [i as f64 * cell - 6.0, u0 + 16.0 + j as f64 * cell, hgt[j * nx + i]] };
            b.m.tri(p(i, j), p(i + 1, j), p(i + 1, j + 1), GROUND);
            b.m.tri(p(i, j), p(i + 1, j + 1), p(i, j + 1), GROUND);
        }
    }
    // Bushes and mounds scattered over the open ground: each throws a small morning shadow.
    for _ in 0..1600 {
        let (x, u) = (b.range(0.0, WIDTH), b.range(u0 + 20.0, u1 - 125.0));
        let on_slab = (44.0..108.0).contains(&x) || (212.0..276.0).contains(&x);
        if on_slab {
            continue;
        }
        let (r, h) = (b.range(1.5, 3.5), b.range(2.0, 5.0));
        b.m.frustum([x - r, u - r * 0.8, 0.0], [x + r, u + r * 0.8, 0.0], r * 0.6, h, ROCK);
    }
    let slab = 5.0;
    // Runways with paint: centre dashes, threshold bars, numbers (36 at the near end, 18 at the far end).
    for &(x0, x1) in &[(44.0, 108.0), (212.0, 276.0)] {
        b.m.frustum([x0, u0 + 40.0, 0.0], [x1, u1 - 120.0, 0.0], 1.0, slab, CONCRETE);
        let mid = (x0 + x1) / 2.0;
        let mut u = u0 + 110.0;
        while u < u1 - 190.0 {
            b.decal(mid - 2.0, u, mid + 2.0, u + 10.0, slab + 0.2, PAINT);
            u += 24.0;
        }
        for (end, flip) in [(u0 + 46.0, false), (u1 - 134.0, true)] {
            let mut bx = x0 + 4.0;
            while bx < x1 - 4.0 {
                let (a, c) = if flip { (end - 14.0, end) } else { (end, end + 14.0) };
                b.decal(bx, a, bx + 4.0, c, slab + 0.2, PAINT);
                bx += 8.0;
            }
            let (d0, d1) = if flip { (8, 1) } else { (3, 6) };
            let nu = if flip { end - 22.0 } else { end + 38.0 };
            paint_digit(b, d0, mid - 14.0, nu, 4.0, slab + 0.2, flip);
            paint_digit(b, d1, mid + 2.0, nu, 4.0, slab + 0.2, flip);
        }
    }
    // Taxiways across, edged with dashes.
    for &u in &[u0 + 420.0, u0 + 1100.0] {
        b.m.frustum([-6.0, u, 0.0], [WIDTH + 6.0, u + 12.0, 0.0], 0.5, slab - 0.5, CONCRETE);
        let mut x = 0.0;
        while x < WIDTH {
            b.decal(x, u, x + 2.0, u + 1.0, slab, PAINT);
            b.decal(x, u + 11.0, x + 2.0, u + 12.0, slab, PAINT);
            x += 8.0;
        }
    }
    for &(x, u) in &[(160.0, u0 + 300.0), (160.0, u0 + 700.0), (160.0, u0 + 1300.0), (296.0, u0 + 900.0), (22.0, u0 + 1500.0)] {
        hangar(b, x, u);
    }
    // AA pits: rings of sandbags.
    for &(x, u) in &[(140.0, u0 + 520.0), (184.0, u0 + 980.0), (132.0, u0 + 1500.0), (296.0, u0 + 600.0), (20.0, u0 + 240.0)] {
        for k in 0..12 {
            let a = k as f64 / 12.0 * std::f64::consts::TAU;
            let (sx, su) = (x + 10.0 * a.cos(), u + 9.0 * a.sin());
            b.m.cuboid([sx - 2.0, su - 1.5, 0.0], [sx + 2.0, su + 1.5, 6.0], SAND);
        }
    }
    // The radar blockhouse.
    let mut r = Builder::local();
    r.frustum([-10.0, -8.0, 0.0], [10.0, 8.0, 0.0], 2.0, 12.0, ROCK);
    r.tube([0.0, 0.0, 12.0], [0.0, 3.0, 18.0], 7.0, 8, STEEL, STEEL);
    b.place(&r, [24.0, u0 + 820.0, 0.0]);
    // The apron at the top, and crystal threading the cracks, thicker toward it.
    b.m.frustum([-6.0, u1 - 120.0, 0.0], [WIDTH + 6.0, u1 + 30.0, 0.0], 0.5, slab, CONCRETE);
    for i in 0..170 {
        let u = u1 - 10.0 - (i * i) as f64 * (u1 - u0 - 50.0) / (170.0 * 170.0);
        let x = b.range(0.0, WIDTH);
        let segs = (b.next() % 4 + 2) as usize;
        b.crack(x, u, segs, slab + 0.4);
    }
}

/// The whole stage as one mesh (world coordinates).
pub fn level() -> Mesh {
    let mut b = Builder { m: Mesh::new(materials()), rng: 0x5EA_0001 };
    sea(&mut b);
    sea_wall(&mut b);
    docks(&mut b);
    airfield(&mut b);
    b.m
}

/// The whole S1 panorama: the level rendered top-down under the stage light with cast shadows, in strips,
/// one loop of the stage (`demo::PANO_PERIOD` lines) plus the first 200 lines again so every window is
/// contiguous. Pixels no geometry covers are `void` (open water).
pub fn panorama() -> Panorama {
    let period = demo::PANO_PERIOD as usize;
    let mesh = level();
    let shadows = ShadowMap::new(&mesh, &SUN);
    let mut codes = vec![VOID; 160 * period];
    let strip = 400usize;
    let mut y0 = 0;
    while y0 < period {
        let h = strip.min(period - y0);
        // Canvas row r is stage line period - 1 - (y0 + r); model x 0 is the canvas's left edge.
        let view = View { w: 160, h: h as u16, cx: 0.0, cy: (period - y0) as f64, scale: 1.0 };
        let img = raster3d::render_scene(&mesh, view, &SUN, &shadows);
        for r in 0..h {
            for x in 0..160 {
                if let Some(c) = img.get(x as u16, r as u16) {
                    codes[(y0 + r) * 160 + x] = c;
                }
            }
        }
        y0 += h;
    }
    let wrap = codes[..160 * 200].to_vec();
    codes.extend_from_slice(&wrap);
    Panorama { rows: ((period + 200) / 8) as u16, codes, palette: PALETTE }
}

/// Share of the panorama's pixels per code (00, 01, 10, 11).
pub fn code_share(p: &Panorama) -> [f64; 4] {
    let mut n = [0usize; 4];
    for &c in &p.codes {
        n[c as usize & 3] += 1;
    }
    n.map(|k| k as f64 / p.codes.len() as f64)
}

/// The top surface under world point (`x`, `u`): the highest triangle's height there and its material
/// (`None` over nothing). Placement checks for the ground units (`crate::ground`).
pub fn surface(mesh: &Mesh, x: f64, u: f64) -> Option<(f64, u8)> {
    let mut best: Option<(f64, u8)> = None;
    for (t, &m) in mesh.tris.iter().zip(&mesh.mats) {
        let (a, b, c) = (mesh.verts[t[0] as usize], mesh.verts[t[1] as usize], mesh.verts[t[2] as usize]);
        if x < a[0].min(b[0]).min(c[0]) || x > a[0].max(b[0]).max(c[0]) || u < a[1].min(b[1]).min(c[1]) || u > a[1].max(b[1]).max(c[1]) {
            continue;
        }
        let area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
        if area.abs() < 1e-9 {
            continue;
        }
        let w0 = ((b[0] - x) * (c[1] - u) - (b[1] - u) * (c[0] - x)) / area;
        let w1 = ((c[0] - x) * (a[1] - u) - (c[1] - u) * (a[0] - x)) / area;
        let w2 = 1.0 - w0 - w1;
        if w0 < -1e-9 || w1 < -1e-9 || w2 < -1e-9 {
            continue;
        }
        let z = w0 * a[2] + w1 * b[2] + w2 * c[2];
        if best.is_none_or(|(bz, _)| z > bz) {
            best = Some((z, m));
        }
    }
    best
}

/// Surface material names for placement checks.
pub const SURFACE_WATER: u8 = WATER;
pub const SURFACE_CONCRETE: u8 = CONCRETE;
pub const SURFACE_FOAM: u8 = FOAM;
pub const SURFACE_PAINT: u8 = PAINT;
pub const SURFACE_CRYSTAL: u8 = CRYSTAL;
pub const SURFACE_ROOF: u8 = ROOF;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_background_stays_down() {
        let p = panorama();
        assert_eq!(p.codes.len(), 160 * p.rows as usize * 8);
        let s = code_share(&p);
        assert!(s[2] <= 0.10, "light share {:.3} over the 10 % background budget", s[2]);
        assert!(s[3] <= 0.03, "hot share {:.3}: the Lattice only in the cracks", s[3]);
        assert_eq!(p.codes[..160 * 200], p.codes[p.codes.len() - 160 * 200..]);
    }

    #[test]
    fn structures_cast_shadows() {
        // A tall block on flat concrete: the concrete beside it, away from the sun, is in shadow.
        let mut m = Mesh::new(materials());
        m.frustum([0.0, 0.0, 0.0], [100.0, 100.0, 0.0], 0.0, 1.0, CONCRETE);
        m.cuboid([40.0, 40.0, 1.0], [50.0, 50.0, 30.0], STEEL);
        let sm = ShadowMap::new(&m, &SUN);
        // The sun is upper-left: shadows fall to the lower right (+x, -u).
        assert!(sm.shadowed([58.0, 38.0, 1.0]));
        assert!(!sm.shadowed([30.0, 60.0, 1.0]));
        assert!(!sm.shadowed([45.0, 45.0, 30.0]));
    }
}
