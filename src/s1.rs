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
use crate::raster3d::{self, Light, Material, Mesh, ShadowMap, Spot, V3, View};
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

/// Animation frames per loop: the level is rendered once per frame (`panorama_frames`) and the C64 picks
/// the copy by the frame counter (`render::background_at`), `1 << ANIM_SHIFT` ticks a frame: a 128-tick
/// (2.1 s) loop. Everything that moves repeats a whole number of times per loop.
pub const ANIM_FRAMES: usize = 32;
pub const ANIM_SHIFT: u8 = 2;

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
    /// Position in the animation loop, 0..1.
    t: f64,
    /// The searchlights and the lighthouse lamp at `t`.
    spots: Vec<Spot>,
}

/// A stable hash of a lattice point to 0..1: animated details that must not shimmer frame to frame.
pub(crate) fn hash(a: i64, b: i64, seed: u64) -> f64 {
    let mut h = (a as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (b as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F) ^ seed.wrapping_mul(0x1656_67B1_9E37_79F9);
    h ^= h >> 29;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 32;
    (h >> 11) as f64 / (1u64 << 53) as f64
}

/// Smooth value noise over world (`x`, `u`) with cells `sx` × `su`: 0..1.
pub(crate) fn noise(x: f64, u: f64, sx: f64, su: f64, seed: u64) -> f64 {
    let (fx, fu) = (x / sx, u / su);
    let (ix, iu) = (fx.floor(), fu.floor());
    let s = |t: f64| t * t * (3.0 - 2.0 * t);
    let (tx, tu) = (s(fx - ix), s(fu - iu));
    let (ix, iu) = (ix as i64, iu as i64);
    let a = hash(ix, iu, seed) * (1.0 - tx) + hash(ix + 1, iu, seed) * tx;
    let b = hash(ix, iu + 1, seed) * (1.0 - tx) + hash(ix + 1, iu + 1, seed) * tx;
    a * (1.0 - tu) + b * tu
}

const TAU: f64 = std::f64::consts::TAU;

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
        // The Lattice creeps: each vein reaches out and draws back once a loop, out of step with its
        // neighbours (keyed to where it starts), between 55 % and all of its length.
        let ph = hash(x.round() as i64, u.round() as i64, 41);
        let reach = segs as f64 * (0.55 + 0.45 * (0.5 + 0.5 * (TAU * (self.t + ph)).sin()));
        for k in 0..segs {
            let (dx, du) = (self.range(-6.0, 6.0), self.range(2.0, 7.0));
            let part = (reach - k as f64).clamp(0.0, 1.0);
            if part < 0.05 {
                continue;
            }
            let (nx, nu) = (x + dx * part, u + du * part);
            // The Lattice pulses: a bright band climbs the stage twice a loop, swelling the veins it passes.
            let s = (TAU * 2.0 * self.t - u / 40.0).sin().max(0.0);
            let (w, h) = (1.2 * (0.75 + 0.6 * s * s), 0.6);
            self.m.poly(&[[x - w, u, z], [x + w, u, z], [nx + w, nu + h, z], [nx - w, nu + h, z]], CRYSTAL);
            x = nx;
            u = nu;
        }
    }
}

// --------------------------------------------------------------------------------------------- the sea

/// Swells: long ridges across the stage, faceted so the sun-facing slopes catch a dark checker. They roll
/// in toward the sea wall one `pitch` per loop; crest heights and wobble are noise over world position, so
/// after a loop every ridge stands where its neighbour stood and the loop has no seam. Whitecaps ride the
/// crests where the noise peaks, growing and fading as the ridge moves through it.
fn swells(b: &mut Builder, u0: f64, u1: f64, x0: f64, x1: f64) {
    let (pitch, seg) = (13.0, 32.0);
    // The static version's draws, so the rest of the level keeps its random stream.
    let mut u = u0;
    while u < u1 {
        b.next();
        let mut x = x0;
        while x < x1 {
            b.next();
            b.next();
            x += seg;
        }
        u += pitch;
    }
    for _ in 0..3 * ((u1 - u0) / 7.0) as usize {
        b.next();
    }
    let mut xs = vec![];
    let mut x = x0;
    while x < x1 {
        xs.push(x);
        x += seg;
    }
    xs.push(x1);
    let cu = |u: f64| u.clamp(u0, u1);
    let mut j = -1i32;
    loop {
        let base = u0 + (j as f64 + b.t) * pitch;
        if base >= u1 {
            break;
        }
        // Flatten the ridges that run in under the sea wall.
        let taper = ((u1 - base - pitch) / 26.0).clamp(0.0, 1.0);
        let crest: Vec<(f64, f64)> = xs
            .iter()
            .map(|&x| ((0.6 + 3.0 * noise(x, base, 48.0, 20.0, 1)) * taper, base + pitch * 0.55 + (noise(x, base, 40.0, 26.0, 2) - 0.5) * 4.0))
            .collect();
        for i in 0..xs.len() - 1 {
            let (x, xn) = (xs[i], xs[i + 1]);
            let ((ha, ca), (hb, cb)) = (crest[i], crest[i + 1]);
            // Back slope (rising from the trough below), front slope (falling to the next trough).
            b.m.poly(&[[x, cu(base), 0.0], [xn, cu(base), 0.0], [xn, cu(cb), hb], [x, cu(ca), ha]], WATER);
            b.m.poly(&[[x, cu(ca), ha], [xn, cu(cb), hb], [xn, cu(base + pitch), 0.0], [x, cu(base + pitch), 0.0]], WATER);
        }
        if taper > 0.5 {
            let mut c = 0;
            let mut x = x0;
            while x < x1 - 4.0 {
                let xc = x + noise(c as f64 * 3.7, base, 1.0, 9.0, 5) * 8.0;
                let v = noise(xc, base, 30.0, 52.0, 3);
                if v > 0.72 {
                    let i = (((xc - x0) / seg) as usize).min(xs.len() - 2);
                    let f = (xc - xs[i]) / (xs[i + 1] - xs[i]);
                    let h = crest[i].0 + (crest[i + 1].0 - crest[i].0) * f;
                    let u = crest[i].1 + (crest[i + 1].1 - crest[i].1) * f;
                    let w = 1.0 + (v - 0.72) / 0.28 * 3.0;
                    if u > u0 && u < u1 {
                        b.decal(xc, u - 0.5, xc + w, u + 0.6, h + 0.3, FOAM);
                    }
                }
                x += 10.0;
                c += 1;
            }
        }
        j += 1;
    }
}

/// One breaker at phase `p` (0..1 is one wave): a crest rolling in toward the shore (0..0.6), bursting into
/// foam there (0.6..0.8), the backwash draining and thinning out (0.8..1). `at(d, s)` maps the distance from
/// the shore (0 the shore line, 1 where the wave starts) and the position across the cell (0..1) to the
/// world; `cell` keys the cell's stable fleck pattern; `z` is the foam's height.
fn breaker(b: &mut Builder, p: f64, cell: i64, at: &dyn Fn(f64, f64) -> V3, z: f64) {
    let quad = |b: &mut Builder, d0: f64, d1: f64, s0: f64, s1: f64, zz: f64| {
        let (a, bb, c, d) = (at(d0, s0), at(d0, s1), at(d1, s1), at(d1, s0));
        b.m.poly(&[[a[0], a[1], zz], [bb[0], bb[1], zz], [c[0], c[1], zz], [d[0], d[1], zz]], FOAM);
    };
    if p < 0.6 {
        let s = p / 0.6;
        let d = 1.0 - 0.85 * s;
        // The wave's face: a water slope steepening toward the crest as it nears the shore.
        let (lo, hgt) = (z - 3.4, 0.8 + 2.8 * s);
        let (a, bb, c, e) = (at(d + 0.12, 0.0), at(d + 0.12, 1.0), at(d, 1.0), at(d, 0.0));
        b.m.poly(&[[a[0], a[1], lo], [bb[0], bb[1], lo], [c[0], c[1], lo + hgt], [e[0], e[1], lo + hgt]], WATER);
        // The crest curls white as it nears the shore, broken in places.
        if s > 0.25 && hash(cell, 11, 7) > 0.85 - 0.75 * s {
            let thick = 0.02 + 0.05 * s * hash(cell, 12, 7);
            quad(b, d - thick, d + 0.01, 0.0, 1.0, lo + hgt + 0.1);
        }
    } else {
        let (s, burst) = if p < 0.8 { ((p - 0.6) / 0.2, true) } else { ((p - 0.8) / 0.2, false) };
        // Flecks spread up to the shore, then drain back and thin out.
        for r in 0..5 {
            let keep = if burst { 0.95 - 0.15 * s } else { 0.8 * (1.0 - s) };
            if hash(cell, r, 9) >= keep {
                continue;
            }
            let spread = if burst { 0.12 + 0.18 * s } else { 0.3 + 0.1 * s };
            let d = 0.02 + spread * (r as f64 + hash(cell, r, 13)) / 5.0;
            let (s0, w) = (hash(cell, r, 17) * 0.6, 0.25 + 0.35 * hash(cell, r, 19));
            quad(b, d, d + 0.05, s0, (s0 + w).min(1.0), z);
        }
    }
}

/// A searchlight on a tower: lamp `h` up at (`x`, `u`), aimed `reach` out along the azimuth `az` (world
/// radians, 0 = +x) and down to the water.
fn searchlight(b: &mut Builder, x: f64, u: f64, h: f64, az: f64, reach: f64, cone: f64) {
    let (c, s) = (az.cos(), az.sin());
    let mut t = Builder::local();
    t.tube([0.0, 0.0, 0.0], [0.0, 0.0, h - 2.0], 3.0, 8, STEEL, STEEL);
    // The lamp housing turns with the beam.
    t.beam([-c * 2.0, -s * 2.0, h], [c * 3.0, s * 3.0, h], 4.0, 3.0, STEEL);
    b.place(&t, [x, u, 0.0]);
    let pos = [x + c * 4.5, u + s * 4.5, h];
    let target = [x + c * reach, u + s * reach, 0.0];
    let d = [target[0] - pos[0], target[1] - pos[1], target[2] - pos[2]];
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    b.spots.push(Spot { pos, dir: [d[0] / l, d[1] / l, d[2] / l], inner: cone * 0.55, outer: cone, radius: 1.2, strength: 1.0 });
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
    // The static surf's draws, so the rest of the level keeps its random stream.
    for _ in 0..28 * 3 {
        b.next();
    }
    // Breakers rolling in on the sand twice a loop, the seaward (south) side first.
    let cells = 40;
    let jit_at = |ang: f64, off: usize| -> f64 {
        let f = ang.rem_euclid(TAU) / TAU * n as f64;
        let (k0, w) = (f.floor() as usize % n, f.fract());
        jit[k0 + off] * (1.0 - w) + jit[(k0 + 1) % n + off] * w
    };
    for k in 0..cells {
        let (a0, a1) = (k as f64 / cells as f64 * TAU, (k + 1) as f64 / cells as f64 * TAU);
        let p = (2.0 * b.t + 0.3 * (a0 + 0.5 * TAU / cells as f64).sin() + 0.05 * hash(k, cu as i64, 3)).rem_euclid(1.0);
        let at = |d: f64, s: f64| -> V3 {
            let a = a0 + (a1 - a0) * s;
            let rr = r * (1.28 + 0.6 * d);
            [cx + a.cos() * rr * jit_at(a, 0), cu + a.sin() * rr * jit_at(a, n), 0.0]
        };
        breaker(b, p, k * 7919 + cu as i64, &at, 3.8);
    }
}

fn sea(b: &mut Builder) {
    // Flat open water under everything (void in the sun), so the searchlights have a surface to light.
    b.decal(-8.0, -20.0, WIDTH + 8.0, demo::PANO_PERIOD as f64 + 8.0, -0.05, WATER);
    swells(b, -20.0, SEA_END, 0.0, WIDTH);
    // The static wake's draws, so the rest of the level keeps its random stream.
    for i in 0..260 {
        let keep = 1.0 - i as f64 / 300.0;
        for _ in 0..2 {
            if b.range(0.0, 1.0) < keep * 0.8 {
                b.next();
                b.next();
            }
        }
    }
    // The MERIDIAN's wake: a white V fading behind the start position, broken into flecks that stream
    // back from the carrier, 65 lines a loop (the fleck pattern repeats every 65 lines, so it loops).
    const FLOW: i64 = 65;
    for j in 0..260 + FLOW {
        let u = j as f64 - FLOW as f64 * b.t;
        if !(0.0..260.0).contains(&u) {
            continue;
        }
        let key = j.rem_euclid(FLOW);
        let spread = (260.0 - u) * 120.0 / 260.0;
        let keep = 1.0 - u / 300.0;
        for (si, side) in [(0, -1.0), (1, 1.0)] {
            if hash(key, si, 31) < keep * 0.8 {
                let x = 160.0 + side * spread + hash(key, si, 32) * 8.0 - 4.0;
                let w = 1.5 + hash(key, si, 33) * 2.5;
                b.decal(x, u, x + w, u + 1.0, 3.4, FOAM);
            }
        }
    }
    meridian(b);
    for &(x, u, r) in &[(44.0, 520.0, 26.0), (276.0, 900.0, 34.0), (60.0, 1300.0, 20.0), (256.0, 1560.0, 24.0), (120.0, 1180.0, 12.0)] {
        islet(b, x, u, r);
    }
    // Marker buoys.
    for (i, &(x, u)) in [(208.0, 700.0), (104.0, 1650.0)].iter().enumerate() {
        // Each bobs twice a loop, the foam ring spreading as it dips.
        let bob = (TAU * (2.0 * b.t + 0.3 * i as f64)).sin();
        let mut p = Builder::local();
        p.tube([0.0, 0.0, 0.0], [0.0, 0.0, 6.0 + 0.8 * bob], 2.5, 8, STEEL, FOAM);
        b.place(&p, [x, u, 0.0]);
        let (r, n) = (4.5 + 3.0 * (0.5 - 0.5 * bob), 14);
        for k in 0..n {
            if k % 3 == 2 {
                continue;
            }
            let (a0, a1) = (k as f64 / n as f64 * TAU, (k + 1) as f64 / n as f64 * TAU);
            let pt = |a: f64, rr: f64| -> V3 { [x + a.cos() * rr, u + a.sin() * rr, 3.9] };
            b.m.poly(&[pt(a0, r), pt(a1, r), pt(a1, r + 1.2), pt(a0, r + 1.2)], FOAM);
        }
    }
}

/// Where the MERIDIAN lies: her stern at the point of her wake, her bow up the stage.
const MERIDIAN_STERN: f64 = 262.0;
const MERIDIAN_BOW: f64 = 542.0;

/// The carrier MERIDIAN steaming up the stage at dawn, the TALON's home (`docs/story/00-bible.md`): a
/// flared hull, the flight deck with the Accord's emblem (a white chevron over a bar in a circle), its
/// centreline and the bow catapults; the island to starboard with its funnel, mast and turning radar; a bow
/// wave and the churn at her stern feeding the wake, both streaming aft.
fn meridian(b: &mut Builder) {
    let (cx, u0, u1) = (160.0, MERIDIAN_STERN, MERIDIAN_BOW);
    let (deck, half) = (14.0, 40.0);
    let shoulder = u1 - 80.0;
    // Hull: the waterline inset under the flared deck edge.
    let outline = |inset: f64, z: f64| -> Vec<V3> {
        vec![
            [cx - half + inset, u0 + inset, z],
            [cx + half - inset, u0 + inset, z],
            [cx + half - inset, shoulder, z],
            [cx + half * 0.45 - inset * 0.5, u1 - 30.0 - inset, z],
            [cx, u1 - inset, z],
            [cx - half * 0.45 + inset * 0.5, u1 - 30.0 - inset, z],
            [cx - half + inset, shoulder, z],
        ]
    };
    let (water, top) = (outline(6.0, 0.0), outline(0.0, deck));
    b.m.loft(&water, &top, STEEL);
    b.m.poly(&top, STEEL);
    let z = deck + 0.2;
    // Centreline dashes, and the two bow catapults.
    let mut u = u0 + 20.0;
    while u < u1 - 80.0 {
        b.decal(cx - 2.0, u, cx + 2.0, u + 8.0, z, PAINT);
        u += 18.0;
    }
    for x in [cx - 14.0, cx + 10.0] {
        b.decal(x, u1 - 120.0, x + 2.0, u1 - 34.0, z, PAINT);
        b.decal(x - 2.0, u1 - 123.0, x + 4.0, u1 - 120.0, z, PAINT);
    }
    // The emblem near the stern: a ring, a bar, a chevron over it.
    let (ex, eu, r) = (cx - 14.0, u0 + 40.0, 11.0);
    let n = 16;
    for k in 0..n {
        let (a0, a1) = (k as f64 / n as f64 * TAU, (k + 1) as f64 / n as f64 * TAU);
        let pt = |a: f64, rr: f64| -> V3 { [ex + a.cos() * rr, eu + a.sin() * rr, z] };
        b.m.poly(&[pt(a0, r), pt(a1, r), pt(a1, r - 1.6), pt(a0, r - 1.6)], PAINT);
    }
    b.decal(ex - 6.0, eu - 4.0, ex + 6.0, eu - 2.0, z, PAINT);
    for side in [-1.0, 1.0] {
        b.m.poly(&[[ex, eu + 5.0, z], [ex + side * 6.0, eu - 0.5, z], [ex + side * 6.0, eu + 1.5, z], [ex, eu + 7.0, z]], PAINT);
    }
    // The island to starboard: two decks, the funnel aft, the mast with a radar turning twice a loop.
    let ix = cx + half - 8.0;
    b.m.cuboid([ix - 7.0, u0 + 110.0, deck], [ix + 6.0, u0 + 170.0, deck + 10.0], CONCRETE);
    b.m.cuboid([ix - 5.0, u0 + 122.0, deck + 10.0], [ix + 4.0, u0 + 156.0, deck + 18.0], CONCRETE);
    b.m.cuboid([ix - 4.0, u0 + 158.0, deck + 10.0], [ix + 3.0, u0 + 166.0, deck + 22.0], ROCK);
    b.m.tube([ix, u0 + 140.0, deck + 18.0], [ix, u0 + 140.0, deck + 30.0], 1.0, 6, STEEL, STEEL);
    let (c, s) = ((TAU * 2.0 * b.t).cos(), (TAU * 2.0 * b.t).sin());
    b.m.beam([ix - c * 7.0, u0 + 140.0 - s * 7.0, deck + 31.0], [ix + c * 7.0, u0 + 140.0 + s * 7.0, deck + 31.0], 2.0, 1.2, STEEL);
    // Bow wave and stern churn: flecks streaming aft at the wake's pace (keyed modulo its flow, so they loop).
    const FLOW: i64 = 65;
    for j in 0..120 + FLOW {
        let d = j as f64 - FLOW as f64 * b.t; // lines aft of the bow
        if !(0.0..120.0).contains(&d) {
            continue;
        }
        let key = j.rem_euclid(FLOW);
        let u = u1 - 6.0 - d;
        // Along the hull, peeling away from it the further aft.
        let edge = if u > shoulder { half * 0.45 * (u1 - u) / 30.0 } else { half - 6.0 };
        let edge = edge.min(half - 6.0) + 1.0 + d * 0.12;
        for (si, side) in [(0, -1.0), (1, 1.0)] {
            if hash(key, si, 51) < 0.75 - d / 200.0 {
                let x = cx + side * (edge + hash(key, si, 52) * 3.0);
                b.decal(x - 1.0, u, x + 1.0 + hash(key, si, 53) * 2.0, u + 1.0, 4.0, FOAM);
            }
        }
    }
    for j in 0..60 + FLOW {
        let d = j as f64 - FLOW as f64 * b.t; // lines aft of the stern
        if !(0.0..60.0).contains(&d) {
            continue;
        }
        let key = j.rem_euclid(FLOW);
        for f in 0..4 {
            if hash(key, f, 61) < 0.85 - d / 80.0 {
                let x = cx + (hash(key, f, 62) - 0.5) * (40.0 - d * 0.5);
                b.decal(x, u0 - d, x + 2.0 + hash(key, f, 63) * 3.0, u0 - d + 1.0, 4.0, FOAM);
            }
        }
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
    // The static surf's draws, so the rest of the level keeps its random stream.
    let mut x = 0.0;
    while x < WIDTH {
        if ((x / 24.0) as i32) % 3 != 1 {
            for _ in 0..15 {
                b.next();
            }
        }
        x += 6.0;
    }
    // Breakers: two waves a loop peel along the wall, their crests rolling in from 36 lines out and
    // bursting white on the blocks.
    let face = u - 13.0;
    let mut c = 0i64;
    let mut x = -4.0;
    while x < WIDTH + 4.0 {
        let p = (2.0 * b.t - x / 1280.0 + 0.1 * (noise(x, 0.0, 56.0, 1.0, 21) - 0.5)).rem_euclid(1.0);
        let at = |d: f64, s: f64| -> V3 { [x + 4.0 * s, face - 36.0 * d, 0.0] };
        breaker(b, p, c, &at, 4.2);
        x += 4.0;
        c += 1;
    }
    // Searchlights on the wall sweep the sea approach, out of step with each other.
    for (sx, phase) in [(64.0, 0.0), (200.0, 0.4)] {
        let az = -TAU / 4.0 + 0.55 * (TAU * (b.t + phase)).sin();
        searchlight(b, sx, u - 4.0, 30.0, az, 70.0, 0.2);
    }
    // The lighthouse lamp turns once a loop, its beam dipping onto the water and the quays.
    let az = TAU * b.t;
    let (c, s) = (az.cos(), az.sin());
    let pos = [296.0 + c * 5.0, u + s * 5.0, 32.0];
    let d = [c * 64.0, s * 64.0, -32.0];
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    b.spots.push(Spot { pos, dir: [d[0] / l, d[1] / l, d[2] / l], inner: 0.07, outer: 0.13, radius: 1.0, strength: 0.9 });
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
    // The boom swings ±7° once a loop (the quays in mirror), its shadow sweeping the water.
    let a = 0.12 * (TAU * b.t).sin() * dir;
    m.beam([0.0, 0.0, 40.0], [dir * 64.0 * a.cos(), dir * 64.0 * a.sin(), 38.0], 5.0, 4.0, STEEL);
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
    // Searchlight pairs on the channel quays, their beams crossing over the channel in mirror (where the
    // KEELBREAKER steams), each pair a quarter loop after the last; none beside a crane.
    let mut u = u0 + 100.0;
    let mut k = 0;
    while u < u1 - 60.0 {
        if !basins.iter().any(|&(_, c0, c1)| (u - (c0 + c1) / 2.0).abs() < 30.0) {
            let sweep = 0.6 * (TAU * (b.t + 0.25 * k as f64)).sin();
            searchlight(b, CHANNEL.0 - 7.0, u, 26.0, sweep, 64.0, 0.2);
            searchlight(b, CHANNEL.1 + 7.0, u, 26.0, TAU / 2.0 - sweep, 64.0, 0.2);
            k += 1;
        }
        u += 360.0;
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
    // Edge lights: a chase running up each runway toward the hangars, one step a frame.
    let k = (b.t * ANIM_FRAMES as f64).round() as i64;
    for &(x0, x1) in &[(44.0, 108.0), (212.0, 276.0)] {
        let mut i = 0i64;
        let mut u = u0 + 64.0;
        while u < u1 - 140.0 {
            let lit = (i - k).rem_euclid(8) < 2;
            for x in [x0 + 3.0, x1 - 7.0] {
                b.decal(x, u, x + 4.0, u + 3.0, slab + 0.3, if lit { PAINT } else { STEEL });
            }
            u += 12.0;
            i += 1;
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
    // AA pits: rings of sandbags, each with a searchlight beside it sweeping the nearest runway.
    for (i, &(x, u)) in [(140.0, u0 + 520.0), (184.0, u0 + 980.0), (132.0, u0 + 1500.0), (296.0, u0 + 600.0), (20.0, u0 + 240.0)].iter().enumerate() {
        let toward = if x < 160.0 && x > 60.0 || x > 260.0 { TAU / 2.0 } else { 0.0 };
        let az = toward + 0.7 * (TAU * (b.t + 0.2 * i as f64)).sin();
        searchlight(b, x, u + 18.0, 20.0, az, 50.0, 0.22);
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
    // The antenna turns once a loop; its shadow sweeps with it.
    let (c, s) = ((TAU * b.t).cos(), (TAU * b.t).sin());
    r.beam([-c * 10.0, -s * 10.0, 20.0], [c * 10.0, s * 10.0, 20.0], 2.5, 1.5, STEEL);
    r.cuboid([-1.0, -1.0, 17.0], [1.0, 1.0, 20.0], STEEL);
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

/// The whole stage as one mesh (world coordinates), at the start of its animation loop.
pub fn level() -> Mesh {
    level_at(0.0).0
}

/// The stage at `t` (0..1) through its animation loop, and its spotlights then.
pub fn level_at(t: f64) -> (Mesh, Vec<Spot>) {
    let mut b = Builder { m: Mesh::new(materials()), rng: 0x5EA_0001, t, spots: vec![] };
    sea(&mut b);
    sea_wall(&mut b);
    docks(&mut b);
    airfield(&mut b);
    (b.m, b.spots)
}

/// Stage lines `y0..y0 + h` from the top of the loop (canvas row r is stage line `period - 1 - (y0 + r)`)
/// of the stage at `t`, as codes, 160 a line.
fn render_lines(mesh: &Mesh, spots: &[Spot], shadows: &ShadowMap, y0: usize, h: usize) -> Vec<u8> {
    let period = demo::PANO_PERIOD as usize;
    let mut codes = vec![VOID; 160 * h];
    let strip = 400usize;
    let mut r0 = 0;
    while r0 < h {
        let n = strip.min(h - r0);
        // Model x 0 is the canvas's left edge.
        let view = View { w: 160, h: n as u16, cx: 0.0, cy: (period - y0 - r0) as f64, scale: 1.0 };
        let img = raster3d::render_scene_lit(mesh, view, &SUN, shadows, spots, [VOID, DARK, LIGHT]);
        for r in 0..n {
            for x in 0..160 {
                if let Some(c) = img.get(x as u16, r as u16) {
                    codes[(r0 + r) * 160 + x] = c;
                }
            }
        }
        r0 += n;
    }
    codes
}

/// The S1 panorama at `t` (0..1) through the animation loop: the level rendered top-down under the stage
/// light with cast shadows and its spotlights, one loop of the stage (`demo::PANO_PERIOD` lines) plus the
/// first 200 lines again so every window is contiguous. Pixels no geometry covers are `void` (open water).
pub fn panorama_at(t: f64) -> Panorama {
    let period = demo::PANO_PERIOD as usize;
    let (mesh, spots) = level_at(t);
    let shadows = ShadowMap::new(&mesh, &SUN);
    let mut codes = render_lines(&mesh, &spots, &shadows, 0, period);
    let wrap = codes[..160 * 200].to_vec();
    codes.extend_from_slice(&wrap);
    Panorama { rows: ((period + 200) / 8) as u16, codes, palette: PALETTE }
}

/// The S1 panorama, the first frame of its loop.
pub fn panorama() -> Panorama {
    panorama_at(0.0)
}

/// Every frame of the S1 animation loop (`ANIM_FRAMES`), rendered in parallel.
pub fn panorama_frames() -> Vec<Panorama> {
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get()).min(ANIM_FRAMES);
    let mut out: Vec<Option<Panorama>> = (0..ANIM_FRAMES).map(|_| None).collect();
    std::thread::scope(|sc| {
        let chunks: Vec<_> = out.chunks_mut(ANIM_FRAMES.div_ceil(threads)).enumerate().collect();
        let per = ANIM_FRAMES.div_ceil(threads);
        for (c, chunk) in chunks {
            sc.spawn(move || {
                for (i, slot) in chunk.iter_mut().enumerate() {
                    *slot = Some(panorama_at((c * per + i) as f64 / ANIM_FRAMES as f64));
                }
            });
        }
    });
    out.into_iter().map(Option::unwrap).collect()
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

    /// Stage lines `lo..hi` of the panorama at `t` (rows from the top of the loop).
    fn lines(t: f64, lo: usize, hi: usize) -> Vec<u8> {
        let period = demo::PANO_PERIOD as usize;
        let (mesh, spots) = level_at(t);
        let shadows = ShadowMap::new(&mesh, &SUN);
        render_lines(&mesh, &spots, &shadows, period - hi, hi - lo)
    }

    #[test]
    fn the_animation_loops_without_a_seam() {
        // A loop on is the start again: the wake, the open sea, a buoy, the sea wall's breakers,
        // searchlights and lighthouse, the docks' cranes, and the airfield's radar, chase lights and Lattice.
        for (lo, hi) in [(40, 240), (300, 500), (600, 800), (1700, 1900), (2200, 2400), (4100, 4300)] {
            assert!(lines(0.0, lo, hi) == lines(1.0, lo, hi), "lines {lo}..{hi} jump at the loop");
        }
        // And it does move: half a loop later the sea and the wall look different.
        assert!(lines(0.0, 1700, 1900) != lines(0.5, 1700, 1900));
    }

    #[test]
    fn spotlights_light_and_shadow() {
        // Flat water under a lamp, a post in its pool: the pool lifts void, the post's shadow does not.
        let mut m = Mesh::new(materials());
        m.poly(&[[0.0, 0.0, 0.0], [160.0, 0.0, 0.0], [160.0, 100.0, 0.0], [0.0, 100.0, 0.0]], WATER);
        m.cuboid([78.0, 48.0, 0.0], [82.0, 52.0, 12.0], STEEL);
        let pos = [80.0, 90.0, 40.0];
        let d: [f64; 3] = [0.0, -50.0, -40.0];
        let l = (d[1] * d[1] + d[2] * d[2]).sqrt();
        let spot = Spot { pos, dir: [0.0, d[1] / l, d[2] / l], inner: 0.3, outer: 0.4, radius: 0.5, strength: 1.0 };
        let sm = ShadowMap::new(&m, &SUN);
        let view = View { w: 80, h: 100, cx: 0.0, cy: 100.0, scale: 1.0 };
        let img = raster3d::render_scene_lit(&m, view, &SUN, &sm, &[spot], [VOID, DARK, LIGHT]);
        let at = |x: f64, u: f64| img.get((x / 2.0) as u16, (100.0 - u) as u16);
        assert_ne!(at(70.0, 44.0), Some(VOID), "the pool is lit");
        assert_eq!(at(80.0, 40.0), Some(VOID), "the post shadows the pool away from the lamp");
        assert_eq!(at(20.0, 80.0), Some(VOID), "outside the cone the water stays dark");
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
