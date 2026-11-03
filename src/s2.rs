//! The S2 TERRACES level (host only), built as 3D geometry by the art director and rendered offline under
//! the stage's own light (`docs/story/hd/02-ladder.md`: S2 afternoon, the sun to the upper right) with cast
//! shadows, as `s1.rs` builds BREAKWATER: 26 screens bottom to top, laid out as the base bible's stage
//! (`docs/story/04-levels/s2-terraces.md`) and the HD direction (`docs/story/hd/models/s2.md`): contour
//! terraces of vines with cypress lines and farmhouses, the AQUEDUCT over its ravine, the hill village
//! around the CAMPANILE's square, the observatory campus up to the great dome (the ASTROLABE).
//!
//! Art direction: ground in `void` (brown soil) and `dark` (terracotta: vine rows, tiles, wall caps);
//! `light` (cream) only for water glints, laundry, awnings and the domes' sunny facets (≤ 10 %); `hot`
//! (light blue) only where the Lattice has grown in. World units: hires px across (x 0..320), stage lines
//! up (`u`, 0 at the stage start), z up. Everything that moves repeats a whole number of times a loop and
//! is keyed to world position or stable hashes, never the random stream.

use crate::bake::Panorama;
use crate::demo;
use crate::raster3d::{self, Light, Material, Mesh, ShadowMap, V3, View};
use crate::render::Palette;
use crate::s1::{hash, noise};

const VOID: u8 = 0;
const DARK: u8 = 1;
const LIGHT: u8 = 2;
const HOT: u8 = 3;

pub const PALETTE: Palette = Palette::new(9, 8, 15, 14);
/// The stage's time of day (the ladder's S2 `light`): a warm afternoon sun from the upper right.
pub const SUN: Light = Light { dir: [0.55, 0.35, 0.65], ambient: 0.20, intensity: 1.0 };

const WIDTH: f64 = 320.0;
const TAU: f64 = std::f64::consts::TAU;
/// Sections (stage lines): vineyards 0-9 screens with the AQUEDUCT's ravine at screen 8, the hill village
/// 9-17, the observatory 17-26.
const RAVINE: (f64, f64) = (1590.0, 1770.0);
const OBSERVATORY: f64 = 17.0 * 200.0;
/// The CAMPANILE's tower in the village square (its crown and bell hub are sprites), and the square.
pub const TOWER: (f64, f64) = (160.0, 2500.0);
const SQUARE: f64 = 70.0;
/// The great dome (the ASTROLABE's body) at the top of the stage, and its radius.
pub const GREAT_DOME: (f64, f64) = (160.0, 5000.0);
const GREAT_R: f64 = 104.0;
/// Plateau heights: the village on the hilltop, the observatory a step above it.
const Z_VILLAGE: f64 = 60.0;
const Z_OBS: f64 = 66.0;

/// Material ids (`materials()` order).
const SOIL: u8 = 0;
const STONE: u8 = 1;
const VINE: u8 = 2;
const ROOF: u8 = 3;
const TREE: u8 = 4;
const WATER: u8 = 5;
const PAINT: u8 = 6;
const CRYSTAL: u8 = 7;
const GRAVEL: u8 = 8;
const DOME: u8 = 9;
const SHADE: u8 = 10;

/// The animation loop: the same length as S1's (the C64 picks the copy by the frame counter).
pub const ANIM_FRAMES: usize = crate::s1::ANIM_FRAMES;
pub const ANIM_SHIFT: u8 = crate::s1::ANIM_SHIFT;

fn materials() -> Vec<Material> {
    vec![
        // Soil: plain brown (void); only slopes turned hard to the sun catch a checker.
        Material::lit(&[VOID, DARK], 0.24, 0),
        // Dry stone, plaster, paving: lit tops solid terracotta-dark, shade to void.
        Material::lit(&[VOID, DARK], 0.85, 0),
        // Vine rows: solid dark ridges, their shady flanks void.
        Material::lit(&[VOID, DARK], 0.9, 0),
        // Clay tiles: solid dark on the slopes in the sun, a checker on those turned away.
        Material::lit(&[VOID, DARK], 0.8, 0),
        // Cypress and hedge: dark on the sunny side, void in shade.
        Material::lit(&[VOID, DARK], 0.75, 0),
        // Water glints and the stream: cream.
        Material::glow(LIGHT),
        // Laundry, awning stripes: cream.
        Material::glow(LIGHT),
        // The Lattice.
        Material::glow(HOT),
        // Raked gravel: the void/dark checker (paths read against plain soil and solid stone).
        Material::lit(&[VOID, DARK], 0.5, 0),
        // Observatory domes: dark, the facets turned to the sun catch a cream checker.
        Material::lit(&[VOID, DARK, LIGHT], 0.56, 1),
        // The ravine's depths.
        Material::glow(VOID),
    ]
}

struct Builder {
    m: Mesh,
    rng: u32,
    /// Position in the animation loop, 0..1.
    t: f64,
    /// Where things are, for placing units on them (`spots`).
    spots: Spots,
}

/// A house with a hipped roof: centre, footprint (w across, l along the stage), eaves and ridge heights.
#[derive(Clone, Copy, Debug)]
pub struct House {
    pub x: f64,
    pub u: f64,
    pub w: f64,
    pub l: f64,
    pub eaves: f64,
    pub ridge: f64,
}

/// The level's landmarks: houses (farmhouses first, then the village), the village lanes' centre lines
/// and the observatory's small domes (x, u, radius).
#[derive(Clone, Debug, Default)]
pub struct Spots {
    pub houses: Vec<House>,
    pub lanes: Vec<f64>,
    pub domes: Vec<(f64, f64, f64)>,
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
    fn decal(&mut self, x0: f64, u0: f64, x1: f64, u1: f64, z: f64, mat: u8) {
        self.m.poly(&[[x0, u0, z], [x1, u0, z], [x1, u1, z], [x0, u1, z]], mat);
    }
    /// A Lattice crack heading `dir` (radians, 0 = up the stage): it pulses and creeps as S1's do.
    fn crack(&mut self, mut x: f64, mut u: f64, dir: f64, segs: usize, z: f64) {
        let ph = hash(x.round() as i64, u.round() as i64, 41);
        let reach = segs as f64 * (0.55 + 0.45 * (0.5 + 0.5 * (TAU * (self.t + ph)).sin()));
        for k in 0..segs {
            let (a, len) = (dir + self.range(-0.7, 0.7), self.range(3.0, 7.0));
            let part = (reach - k as f64).clamp(0.0, 1.0);
            if part < 0.05 {
                continue;
            }
            let (nx, nu) = (x + a.sin() * len * part, u + a.cos() * len * part);
            let s = (TAU * 2.0 * self.t - u / 40.0).sin().max(0.0);
            let w = 1.1 * (0.75 + 0.6 * s * s);
            let (px, pu) = (a.cos() * w, -a.sin() * w);
            self.m.poly(&[[x - px, u - pu, z], [x + px, u + pu, z], [nx + px, nu + pu, z], [nx - px, nu - pu, z]], CRYSTAL);
            x = nx;
            u = nu;
        }
    }
    /// A crystal shard cluster sprouting at (`x`, `u`) on the ground at `z`.
    fn shards(&mut self, x: f64, u: f64, z: f64) {
        for _ in 0..3 {
            let (dx, du, h) = (self.range(-2.5, 2.5), self.range(-2.5, 2.5), self.range(4.0, 8.0));
            let lean = (self.range(-2.0, 2.0), self.range(-2.0, 2.0));
            self.m.shard([x + dx, u + du, z], [x + dx + lean.0, u + du + lean.1, z + h], 1.3, CRYSTAL);
        }
    }
    /// A hipped clay-tile roof over `w` × `l` at (`cx`, `cu`), eaves at `z`, `h` to the ridge; the ridge runs
    /// along the longer side.
    fn roof(&mut self, cx: f64, cu: f64, w: f64, l: f64, z: f64, h: f64) {
        let (hw, hl) = (w / 2.0, l / 2.0);
        let e = [[cx - hw, cu - hl, z], [cx + hw, cu - hl, z], [cx + hw, cu + hl, z], [cx - hw, cu + hl, z]];
        if w >= l {
            let (r0, r1) = ([cx - hw + hl, cu, z + h], [cx + hw - hl, cu, z + h]);
            self.m.poly(&[e[0], e[1], r1, r0], ROOF);
            self.m.poly(&[e[2], e[3], r0, r1], ROOF);
            self.m.tri(e[3], e[0], r0, ROOF);
            self.m.tri(e[1], e[2], r1, ROOF);
        } else {
            let (r0, r1) = ([cx, cu - hl + hw, z + h], [cx, cu + hl - hw, z + h]);
            self.m.poly(&[e[1], e[2], r1, r0], ROOF);
            self.m.poly(&[e[3], e[0], r0, r1], ROOF);
            self.m.tri(e[0], e[1], r0, ROOF);
            self.m.tri(e[2], e[3], r1, ROOF);
        }
    }
    /// A house: plaster walls `walls` high under a hipped roof.
    fn house(&mut self, cx: f64, cu: f64, w: f64, l: f64, z: f64, walls: f64) {
        self.m.cuboid([cx - w / 2.0, cu - l / 2.0, z], [cx + w / 2.0, cu + l / 2.0, z + walls], STONE);
        let h = w.min(l) * 0.35;
        self.roof(cx, cu, w + 2.0, l + 2.0, z + walls, h);
        self.spots.houses.push(House { x: cx, u: cu, w: w + 2.0, l: l + 2.0, eaves: z + walls, ridge: z + walls + h });
    }
    /// A cypress: a tall narrow cone swaying in the breeze (its tip once a loop, out of step with its
    /// neighbours), throwing a long afternoon shadow.
    fn cypress(&mut self, x: f64, u: f64, z: f64, h: f64) {
        let ph = hash(x.round() as i64, u.round() as i64, 77);
        let sway = 0.9 * (TAU * (self.t + ph)).sin();
        let base = Mesh::ring(6, 3.0, z, ph * TAU);
        let mid: Vec<V3> = Mesh::ring(6, 2.4, z + h * 0.55, ph * TAU).iter().map(|p| [p[0] + sway * 0.4, p[1], p[2]]).collect();
        let tip = [x + sway, u + sway * 0.3, z + h];
        let at = |p: &V3| [p[0] + x, p[1] + u, p[2]];
        let (base, mid): (Vec<V3>, Vec<V3>) = (base.iter().map(at).collect(), mid.iter().map(at).collect());
        self.m.loft(&base, &mid, TREE);
        for i in 0..mid.len() {
            self.m.tri(mid[i], mid[(i + 1) % mid.len()], tip, TREE);
        }
    }
    /// A dome of radius `r` at (`cx`, `cu`) on ground `z`: rings of facets, a dark slit down its middle
    /// `slit` wide (the shutter), open along the stage.
    fn dome(&mut self, cx: f64, cu: f64, r: f64, z: f64, slit: f64) {
        let n = 16;
        let rings = 5;
        let ring = |k: usize| -> Vec<V3> {
            let a = k as f64 / rings as f64 * TAU / 4.0;
            Mesh::ring(n, r * a.cos(), z + r * a.sin(), TAU / 32.0).iter().map(|p| [p[0] + cx, p[1] + cu, p[2]]).collect()
        };
        for k in 0..rings {
            let (a, b) = (ring(k), ring(k + 1));
            if k + 1 == rings {
                self.m.poly(&a, DOME);
            } else {
                self.m.loft(&a, &b, DOME);
            }
        }
        // The shutter slit, lying over the dome's crown (a strip of the dome's profile).
        if slit > 0.2 {
            let steps = 8;
            for k in 0..steps {
                let (a0, a1) = (k as f64 / steps as f64 * std::f64::consts::PI, (k + 1) as f64 / steps as f64 * std::f64::consts::PI);
                let (u0, z0) = (cu - r * a0.cos(), z + r * a0.sin() + 0.3);
                let (u1, z1) = (cu - r * a1.cos(), z + r * a1.sin() + 0.3);
                self.m.poly(&[[cx - slit / 2.0, u0, z0], [cx + slit / 2.0, u0, z0], [cx + slit / 2.0, u1, z1], [cx - slit / 2.0, u1, z1]], SHADE);
            }
        }
        // Its drum (the wall under the dome).
        let foot: Vec<V3> = Mesh::ring(n, r + 1.0, z - 4.0, TAU / 32.0).iter().map(|p| [p[0] + cx, p[1] + cu, p[2]]).collect();
        self.m.loft(&foot, &ring(0), STONE);
    }
}

// -------------------------------------------------------------------------------------- the vineyards

/// The lower edge of terrace `k` at `x`: the contour wobbles across the hill.
fn contour(base: f64, k: usize, x: f64) -> f64 {
    base + 10.0 * (x / 70.0 + k as f64 * 1.7).sin() + 5.0 * (x / 31.0 + k as f64 * 0.9).sin()
}

/// Terraces are this many lines apart; vine row `r` runs `10 + 9 r` lines into its terrace.
const PITCH: f64 = 64.0;
pub const TERRACES: usize = ((RAVINE.0 - 20.0) / PITCH) as usize;
pub const ROWS: usize = 6;

/// The lower edge of terrace `k` (its dry-stone wall) at `x`.
pub fn terrace_lo(k: usize, x: f64) -> f64 {
    if k == 0 { -20.0 } else { contour(k as f64 * PITCH, k, x) }
}

/// The centre line of vine row `r` on terrace `k` at `x` (a PIKE rail).
pub fn row_u(k: usize, r: usize, x: f64) -> f64 {
    let f = (10.0 + r as f64 * 9.0) / PITCH;
    let (lo, hi) = (terrace_lo(k, x), contour((k + 1) as f64 * PITCH, k + 1, x));
    lo + (hi - lo) * f
}

/// The AQUEDUCT's water channel (a PIKE rail): its line up the stage and its deck's height.
pub const AQUEDUCT_U: f64 = (RAVINE.0 + RAVINE.1) / 2.0 + 14.0;
pub const AQUEDUCT_TOP: f64 = Z_VILLAGE - 2.0;

/// The observatory's gravel path from dome `i` to dome `i + 1` at `f` (0..1) along it.
pub fn path_at(i: usize, f: f64) -> (f64, f64) {
    let (a, c) = (DOMES[i], DOMES[i + 1]);
    path_point(a.0, a.1, c.0, c.1, if a.0 < c.0 { 30.0 } else { -30.0 }, f)
}

fn path_point(x0: f64, u0: f64, x1: f64, u1: f64, bow: f64, f: f64) -> (f64, f64) {
    let (dx, du) = (x1 - x0, u1 - u0);
    let l = dx.hypot(du);
    let s = bow * (f * std::f64::consts::PI).sin();
    (x0 + dx * f - du / l * s, u0 + du * f + dx / l * s)
}

/// The small domes (where DOME MORTARs open): x, u, radius.
pub const DOMES: [(f64, f64, f64); 10] = [
    (70.0, 3560.0, 16.0), (250.0, 3640.0, 20.0), (150.0, 3800.0, 14.0), (60.0, 3980.0, 22.0), (270.0, 4040.0, 16.0),
    (180.0, 4220.0, 18.0), (60.0, 4420.0, 15.0), (255.0, 4440.0, 21.0), (110.0, 4640.0, 16.0), (230.0, 4700.0, 14.0),
];
/// The observatory's ground and the sundial court's centre.
pub const OBS_Z: f64 = Z_OBS;
pub const SUNDIAL: (f64, f64) = (150.0, 4330.0);

/// The level's landmarks (`Spots`), as built at the start of the loop.
pub fn spots() -> Spots {
    let mut b = Builder { m: Mesh::new(materials()), rng: SEED, t: 0.0, spots: Spots::default() };
    vineyards(&mut b);
    aqueduct(&mut b);
    village(&mut b);
    observatory(&mut b);
    b.spots
}

/// Top surface (height, material) under (`x`, `u`), as `s1::surface`; materials as `SURFACE_*`.
pub fn surface(mesh: &Mesh, x: f64, u: f64) -> Option<(f64, u8)> {
    crate::s1::surface(mesh, x, u)
}
pub const SURFACE_SOIL: u8 = SOIL;
pub const SURFACE_STONE: u8 = STONE;
pub const SURFACE_VINE: u8 = VINE;
pub const SURFACE_ROOF: u8 = ROOF;
pub const SURFACE_GRAVEL: u8 = GRAVEL;
pub const SURFACE_DOME: u8 = DOME;
pub const SURFACE_SHADE: u8 = SHADE;
pub const SURFACE_WATER: u8 = WATER;

const SEED: u32 = 0x5EA_0002;

fn vineyards(b: &mut Builder) {
    let pitch = PITCH;
    let terraces = TERRACES;
    let xs: Vec<f64> = (0..=20).map(|i| -8.0 + i as f64 * (WIDTH + 16.0) / 20.0).collect();
    // The dirt track zig-zags up the hill, crossing each terrace on a diagonal.
    let track = |k: usize, f: f64| -> f64 { if k % 2 == 0 { 60.0 + 200.0 * f } else { 260.0 - 200.0 * f } };
    for k in 0..terraces {
        let (u0, z) = (k as f64 * pitch, k as f64 * 4.0);
        let lo = |x: f64| if k == 0 { -20.0 } else { contour(u0, k, x) };
        let hi = |x: f64| contour(u0 + pitch, k + 1, x);
        // The terrace's soil, and the dry-stone wall rising to it from the terrace below, its cap catching
        // the sun (the walls face down the stage, away from it: a band of shade under each).
        for i in 0..xs.len() - 1 {
            let (x0, x1) = (xs[i], xs[i + 1]);
            b.m.poly(&[[x0, lo(x0), z], [x1, lo(x1), z], [x1, hi(x1), z], [x0, hi(x0), z]], SOIL);
            if k > 0 {
                b.m.poly(&[[x0, lo(x0) - 0.01, z - 4.0], [x1, lo(x1) - 0.01, z - 4.0], [x1, lo(x1), z], [x0, lo(x0), z]], STONE);
                b.m.poly(&[[x0, lo(x0), z + 0.6], [x1, lo(x1), z + 0.6], [x1, lo(x1) + 2.0, z + 0.6], [x0, lo(x0) + 2.0, z + 0.6]], STONE);
            }
        }
        // The farmhouse yard on some terraces (rows stop short of it).
        let farm = if k % 4 == 2 {
            let x = if k % 8 == 2 { 250.0 } else { 70.0 };
            Some((x, lo(x) + 30.0))
        } else {
            None
        };
        // Vine rows along the contour; a gust crosses the hill once a loop, lifting the leaves it passes
        // (the rows' sunny flanks shimmer). The track breaks every row it crosses.
        let mut r = 0;
        loop {
            let off = 10.0 + r as f64 * 9.0;
            if off > pitch - 8.0 {
                break;
            }
            let f = off / pitch;
            let tx = track(k, f);
            for i in 0..xs.len() - 1 {
                let (x0, x1) = (xs[i].max(0.0), xs[i + 1].min(WIDTH));
                let row = |x: f64| lo(x) + (hi(x) - lo(x)) * f;
                let in_yard = farm.is_some_and(|(fx, fu)| x0 < fx + 30.0 && x1 > fx - 30.0 && (row((x0 + x1) / 2.0) - fu).abs() < 25.0);
                if x1 - x0 < 1.0 || (x0 < tx + 9.0 && x1 > tx - 9.0) || in_yard {
                    continue;
                }
                // A flat-topped hedge of leaves: 6 lines across at the foot, 3 on top.
                let gust = |x: f64| 2.4 + 0.9 * (TAU * (b.t - x / 160.0) + k as f64).sin();
                let (h0, h1) = (gust(x0), gust(x1));
                let (a, c) = ([x0, row(x0), z], [x1, row(x1), z]);
                b.m.poly(&[[a[0], a[1] - 3.0, z], [c[0], c[1] - 3.0, z], [c[0], c[1] - 1.5, z + h1], [a[0], a[1] - 1.5, z + h0]], VINE);
                b.m.poly(&[[a[0], a[1] - 1.5, z + h0], [c[0], c[1] - 1.5, z + h1], [c[0], c[1] + 1.5, z + h1], [a[0], a[1] + 1.5, z + h0]], VINE);
                b.m.poly(&[[a[0], a[1] + 1.5, z + h0], [c[0], c[1] + 1.5, z + h1], [c[0], c[1] + 3.0, z], [a[0], a[1] + 3.0, z]], VINE);
            }
            // Crystal sprouting at the row ends.
            if hash(k as i64, r, 5) > 0.82 {
                let x = if hash(k as i64, r, 6) > 0.5 { 6.0 } else { WIDTH - 6.0 };
                b.shards(x, lo(x) + (hi(x) - lo(x)) * f, z);
            }
            r += 1;
        }
        // The track: a gravel diagonal across the terrace.
        let steps = 6;
        for s in 0..steps {
            let (f0, f1) = (s as f64 / steps as f64, (s + 1) as f64 / steps as f64);
            let (xa, xb) = (track(k, f0), track(k, f1));
            let (ua, ub) = (lo(xa) + (hi(xa) - lo(xa)) * f0, lo(xb) + (hi(xb) - lo(xb)) * f1);
            b.m.poly(&[[xa - 5.0, ua, z + 0.2], [xa + 5.0, ua, z + 0.2], [xb + 5.0, ub, z + 0.2], [xb - 5.0, ub, z + 0.2]], GRAVEL);
        }
        // A line of cypresses along every third terrace's upper edge.
        if k % 3 == 1 {
            let mut x = b.range(10.0, 40.0);
            while x < WIDTH - 10.0 {
                let skip = (x - track(k, 0.95)).abs() < 14.0;
                if !skip {
                    let h = b.range(16.0, 24.0);
                    b.cypress(x, hi(x) - 4.0, z, h);
                }
                x += b.range(14.0, 20.0);
            }
        }
        // A farmhouse on some terraces, beside the track, with a well in its yard.
        if let Some((x, u)) = farm {
            b.decal(x - 26.0, u - 20.0, x + 26.0, u + 20.0, z + 0.15, STONE);
            b.house(x, u, 34.0, 22.0, z, 6.0);
            let (wx, wu) = (x + if x > 160.0 { -22.0 } else { 22.0 }, u - 12.0);
            b.m.tube([wx, wu, z], [wx, wu, z + 3.0], 2.6, 8, STONE, SHADE);
        }
    }
    // The hilltop under the last terraces, down to the ravine's lip.
    let k = terraces;
    let (u0, z) = (k as f64 * pitch, k as f64 * 4.0);
    b.m.poly(&[[-8.0, u0 - 20.0, z], [WIDTH + 8.0, u0 - 20.0, z], [WIDTH + 8.0, RAVINE.0, z], [-8.0, RAVINE.0, z]], SOIL);
}

// ------------------------------------------------------------------------------------- the aqueduct

/// The ravine (deep shade with a thin stream far below) and the AQUEDUCT crossing it on tall arches, its
/// channel glinting: the glints run along it one spacing a loop.
fn aqueduct(b: &mut Builder) {
    let (u0, u1) = RAVINE;
    let rim = |x: f64, top: bool| -> f64 { if top { u1 - 6.0 * (x / 40.0).sin().abs() } else { u0 + 6.0 * (x / 37.0).cos().abs() } };
    let zf = -40.0;
    let xs: Vec<f64> = (0..=16).map(|i| -8.0 + i as f64 * (WIDTH + 16.0) / 16.0).collect();
    let zl = (((u0 - 20.0) / 64.0) as usize) as f64 * 4.0;
    for i in 0..xs.len() - 1 {
        let (x0, x1) = (xs[i], xs[i + 1]);
        // The near wall falls from the vineyards' lip, the far one climbs to the village; the floor is shade.
        b.m.poly(&[[x0, rim(x0, false), zl], [x1, rim(x1, false), zl], [x1, rim(x1, false) + 30.0, zf], [x0, rim(x0, false) + 30.0, zf]], STONE);
        b.m.poly(&[[x0, rim(x0, true) - 30.0, zf], [x1, rim(x1, true) - 30.0, zf], [x1, rim(x1, true), Z_VILLAGE], [x0, rim(x0, true), Z_VILLAGE]], STONE);
        b.m.poly(&[[x0, rim(x0, false) + 30.0, zf], [x1, rim(x1, false) + 30.0, zf], [x1, rim(x1, true) - 30.0, zf], [x0, rim(x0, true) - 30.0, zf]], SHADE);
    }
    // Boulders on the ravine floor: dark where a sunbeam reaches down past the far wall.
    for _ in 0..90 {
        let (x, u) = (b.range(0.0, WIDTH), b.range(u0 + 32.0, u1 - 32.0));
        let (rr, h) = (b.range(2.0, 5.0), b.range(2.0, 5.0));
        b.m.frustum([x - rr, u - rr * 0.8, zf], [x + rr, u + rr * 0.8, zf], rr * 0.5, zf + h, STONE);
    }
    // The stream: a thin meandering cream thread, its sparkle drifting downstream (to the left).
    let mid = (u0 + u1) / 2.0;
    let mut x = 0.0;
    while x < WIDTH {
        let u = mid + 8.0 * (x / 45.0).sin();
        let ph = ((x / 8.0) + b.t * 2.0).rem_euclid(1.0);
        if ph < 0.7 {
            b.decal(x, u - 0.8, x + 5.0, u + 0.8, zf + 0.2, WATER);
        }
        x += 8.0;
    }
    // The aqueduct: a stone channel on piers, lengthwise across the stage.
    let (au, w, top) = (mid + 14.0, 14.0, Z_VILLAGE - 2.0);
    b.m.cuboid([-8.0, au - w / 2.0, top - 6.0], [WIDTH + 8.0, au + w / 2.0, top], STONE);
    // Parapets along both edges, the water channel between them.
    for e in [-1.0, 1.0] {
        let eu = au + e * (w / 2.0 - 1.0);
        b.m.cuboid([-8.0, eu - 1.0, top], [WIDTH + 8.0, eu + 1.0, top + 2.5], STONE);
    }
    let mut px = 10.0;
    while px < WIDTH {
        b.m.cuboid([px - 4.0, au - w / 2.0 + 1.0, zf], [px + 4.0, au + w / 2.0 - 1.0, top - 6.0], STONE);
        px += 40.0;
    }
    // The water channel along its top: shade, with glints running along it, 20 px apart, one spacing a loop.
    b.decal(-8.0, au - 2.5, WIDTH + 8.0, au + 2.5, top + 0.1, SHADE);
    for k in -1..17 {
        let gx = k as f64 * 20.0 + 20.0 * b.t;
        // Only over the channel: geometry beyond the level's edges would move its shadow map's bounds.
        if !(-8.0..=WIDTH).contains(&gx) {
            continue;
        }
        // Keyed to where it is, not which it is: a loop on, each glint stands where the next one stood.
        let len = 4.0 + 3.0 * noise(gx, 0.0, 26.0, 1.0, 11);
        b.decal(gx, au - 1.0, gx + len, au + 1.0, top + 0.2, WATER);
    }
    // Crystal grows along one span.
    for k in 0..8 {
        let x = 210.0 + k as f64 * 9.0;
        b.crack(x, au + w / 2.0 - 1.0, if k % 2 == 0 { 0.4 } else { -0.4 }, 3, top + 0.3);
    }
    b.shards(236.0, au, top);
    b.shards(262.0, au + 3.0, top);
}

// --------------------------------------------------------------------------------------- the village

/// A fountain: a round basin of pale stone with ripples spreading from the jet, two a loop.
fn fountain(b: &mut Builder, cx: f64, cu: f64, z: f64) {
    let ring = |r: f64, zz: f64| -> Vec<V3> { Mesh::ring(16, r, zz, 0.0).iter().map(|p| [p[0] + cx, p[1] + cu, p[2]]).collect() };
    for (k, r) in [44.0, 36.0, 28.0].iter().enumerate() {
        let (a, c) = (ring(*r, z + 0.1 + k as f64 * 0.1), ring(r - 3.0, z + 0.1 + k as f64 * 0.1));
        b.m.loft(&a, &c, if k == 1 { GRAVEL } else { STONE });
    }
    b.m.loft(&ring(16.0, z), &ring(16.0, z + 2.5), STONE);
    b.m.loft(&ring(16.0, z + 2.5), &ring(14.0, z + 2.5), STONE);
    b.m.poly(&ring(14.0, z + 1.0), SHADE);
    for k in 0..3 {
        let r = 2.0 + 12.0 * ((b.t * 2.0 + k as f64 / 3.0).rem_euclid(1.0));
        if r > 3.0 && r < 13.0 {
            b.m.loft(&ring(r, z + 1.1), &ring(r - 0.8, z + 1.1), WATER);
        }
    }
    b.m.tube([cx, cu, z + 1.0], [cx, cu, z + 4.0], 1.2, 6, STONE, WATER);
}

/// A laundry line between two points at height `z`: pegged cloths that stir in the breeze.
fn laundry(b: &mut Builder, x0: f64, u0: f64, x1: f64, u1: f64, z: f64) {
    let n = ((x1 - x0).hypot(u1 - u0) / 5.0) as usize;
    for k in 1..n {
        let f = k as f64 / n as f64;
        if hash(x0 as i64 + k as i64, u0 as i64, 21) < 0.35 {
            continue;
        }
        let (x, u) = (x0 + (x1 - x0) * f, u0 + (u1 - u0) * f);
        let stir = 0.7 * (TAU * (b.t + hash(k as i64, u0 as i64, 23))).sin();
        b.decal(x - 1.5 + stir, u - 1.2, x + 1.5 + stir, u + 1.2, z, PAINT);
    }
}

fn village(b: &mut Builder) {
    let (u0, u1) = (RAVINE.1 - 8.0, OBSERVATORY);
    let z = Z_VILLAGE;
    // The plateau: packed earth lanes under everything.
    b.m.poly(&[[-8.0, u0, z], [WIDTH + 8.0, u0, z], [WIDTH + 8.0, u1, z], [-8.0, u1, z]], SOIL);
    let (tx, tu) = TOWER;
    let fount = (88.0, 2960.0);
    let open = |x: f64, u: f64, r: f64| -> bool {
        ((x - tx).abs() < SQUARE + r && (u - tu).abs() < SQUARE + r) || (x - fount.0).hypot(u - fount.1) < 48.0 + r
    };
    // Blocks of houses between lanes: rows of lots, each a house of its own size and ridge direction, a
    // courtyard here and there.
    let mut u = u0 + 24.0;
    while u < u1 - 30.0 {
        let depth = b.range(34.0, 52.0);
        let mut x = b.range(-6.0, 10.0);
        while x < WIDTH {
            let w = b.range(18.0, 40.0);
            // Each house its own depth, set back or forward in its lot.
            let d = depth * b.range(0.6, 1.0);
            let (cx, cu) = (x + w / 2.0, u + depth / 2.0 + b.range(-1.0, 1.0) * (depth - d) / 2.0);
            let kind = b.next() % 7;
            if kind == 6 {
                // An empty lot: a garden of low hedges.
                if !open(cx, cu, w / 2.0) {
                    b.m.frustum([cx - w / 2.0 + 3.0, cu - 2.0, z], [cx + w / 2.0 - 3.0, cu + 2.0, z], 1.0, z + 3.0, TREE);
                }
            } else if !open(cx, cu, w.max(depth) / 2.0) {
                if kind == 0 {
                    // A courtyard: paving, a potted tree, laundry across it.
                    b.decal(x + 1.0, u + 1.0, x + w - 1.0, u + depth - 1.0, z + 0.1, STONE);
                    b.m.tube([cx, cu, z], [cx, cu, z + 2.0], 2.5, 6, STONE, TREE);
                    b.cypress(cx, cu, z + 2.0, 6.0);
                    laundry(b, x + 2.0, u + 4.0, x + w - 2.0, u + depth - 4.0, z + 7.0);
                } else {
                    let walls = 5.0 + (b.next() % 3) as f64 * 3.0;
                    b.house(cx, cu, w - 3.0, d - 3.0, z, walls);
                    // Crystal crusting the gutters of houses nearest the tower.
                    if (cx - tx).hypot(cu - tu) < 260.0 && hash(cx as i64, cu as i64, 9) > 0.7 {
                        b.crack(x + 2.0, u + 1.0, 0.0, 3, z + walls + 0.3);
                    }
                }
            }
            x += w + b.range(4.0, 9.0);
        }
        // A lane, and now and then a line of washing across it.
        let lane = b.range(8.0, 12.0);
        b.spots.lanes.push(u + depth + lane / 2.0);
        if hash(u as i64, 1, 31) > 0.5 {
            let lx = b.range(30.0, 280.0);
            laundry(b, lx, u + depth - 1.0, lx + 6.0, u + depth + lane + 1.0, z + 9.0);
        }
        u += depth + lane;
    }
    // The square: pale paving in rings round the tower, the tower itself (the crown is a sprite), and
    // crystal cracking through the paving toward it.
    b.decal(tx - SQUARE, tu - SQUARE, tx + SQUARE, tu + SQUARE, z + 0.1, STONE);
    for k in 0..6 {
        let r = 22.0 + k as f64 * 8.0;
        let a: Vec<V3> = Mesh::ring(24, r, z + 0.2, 0.0).iter().map(|p| [p[0] + tx, p[1] + tu, p[2]]).collect();
        let c: Vec<V3> = Mesh::ring(24, r - 1.0, z + 0.2, 0.0).iter().map(|p| [p[0] + tx, p[1] + tu, p[2]]).collect();
        b.m.loft(&a, &c, GRAVEL);
    }
    let s = 18.0;
    b.m.cuboid([tx - s, tu - s, z], [tx + s, tu + s, z + 46.0], STONE);
    for (dx, du) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
        let (bx, bu) = (tx + dx * s, tu + du * s);
        b.m.frustum([bx - 4.0, bu - 4.0, z], [bx + 4.0, bu + 4.0, z], 1.5, z + 40.0, STONE);
    }
    // The roof's old tiles at its edges, and the dark hub where the bell hangs.
    for (dx, du, w, l) in [(0.0, -s + 3.0, 2.0 * s, 6.0), (0.0, s - 3.0, 2.0 * s, 6.0), (-s + 3.0, 0.0, 6.0, 2.0 * s), (s - 3.0, 0.0, 6.0, 2.0 * s)] {
        b.m.cuboid([tx + dx - w / 2.0, tu + du - l / 2.0, z + 46.0], [tx + dx + w / 2.0, tu + du + l / 2.0, z + 47.5], ROOF);
    }
    b.decal(tx - 6.0, tu - 6.0, tx + 6.0, tu + 6.0, z + 46.2, SHADE);
    for k in 0..14 {
        let a = k as f64 / 14.0 * TAU;
        let r = SQUARE - 4.0 + 30.0 * hash(k, 2, 51);
        let (x, u) = (tx + a.sin() * r, tu + a.cos() * r);
        b.crack(x, u, a + std::f64::consts::PI, 6, z + 0.4);
    }
    // The fountain plaza, with striped market awnings round it.
    fountain(b, fount.0, fount.1, z);
    for k in 0..5 {
        let a = 0.6 + k as f64 * 0.55;
        let (ax, au) = (fount.0 + a.cos() * 58.0, fount.1 + a.sin() * 58.0);
        for s in 0..4 {
            let mat = if s % 2 == 0 { PAINT } else { ROOF };
            let x0 = ax - 8.0 + s as f64 * 4.0;
            b.m.poly(&[[x0, au - 5.0, z + 5.0], [x0 + 4.0, au - 5.0, z + 5.0], [x0 + 4.0, au + 5.0, z + 7.0], [x0, au + 5.0, z + 7.0]], mat);
        }
    }
}

// ----------------------------------------------------------------------------------- the observatory

/// A curving raked-gravel path from (`x0`, `u0`) to (`x1`, `u1`), bowed by `bow`.
fn path(b: &mut Builder, x0: f64, u0: f64, x1: f64, u1: f64, bow: f64, z: f64) {
    let n = 12;
    let p = |f: f64| -> (f64, f64) {
        let (dx, du) = (x1 - x0, u1 - u0);
        let l = dx.hypot(du);
        let s = bow * (f * std::f64::consts::PI).sin();
        (x0 + dx * f - du / l * s, u0 + du * f + dx / l * s)
    };
    for k in 0..n {
        let (a, c) = (p(k as f64 / n as f64), p((k + 1) as f64 / n as f64));
        let (dx, du) = (c.0 - a.0, c.1 - a.1);
        let l = dx.hypot(du).max(1e-6);
        let (nx, nu) = (-du / l * 4.0, dx / l * 4.0);
        b.m.poly(&[[a.0 - nx, a.1 - nu, z], [a.0 + nx, a.1 + nu, z], [c.0 + nx, c.1 + nu, z], [c.0 - nx, c.1 - nu, z]], GRAVEL);
    }
}

fn observatory(b: &mut Builder) {
    let (u0, u1) = (OBSERVATORY, demo::PANO_PERIOD as f64);
    let z = Z_OBS;
    // A retaining wall up from the village, then the campus lawn.
    b.m.frustum([-8.0, u0, Z_VILLAGE], [WIDTH + 8.0, u0 + 10.0, Z_VILLAGE], 0.0, z, STONE);
    b.m.poly(&[[-8.0, u0 + 10.0, z], [WIDTH + 8.0, u0 + 10.0, z], [WIDTH + 8.0, u1 + 30.0, z], [-8.0, u1 + 30.0, z]], SOIL);
    // The small domes (where DOME MORTARs open), each with its shutter slit creeping open and shut.
    let domes = DOMES;
    b.spots.domes = domes.to_vec();
    // Gravel paths joining them, curving.
    for w in domes.windows(2) {
        let (a, c) = (w[0], w[1]);
        path(b, a.0, a.1, c.0, c.1, if a.0 < c.0 { 30.0 } else { -30.0 }, z + 0.1);
    }
    path(b, 160.0, u0 + 10.0, domes[0].0, domes[0].1, 20.0, z + 0.1);
    path(b, domes[9].0, domes[9].1, GREAT_DOME.0, GREAT_DOME.1 - GREAT_R, -24.0, z + 0.1);
    for (i, &(x, u, r)) in domes.iter().enumerate() {
        let slit = r * 0.22 * (1.0 + 0.6 * (TAU * (b.t + i as f64 * 0.13)).sin());
        b.dome(x, u, r, z + 4.0, slit);
        // A gravel ring round each, and hedges in an arc on its shady side.
        let a: Vec<V3> = Mesh::ring(20, r + 7.0, z + 0.1, 0.0).iter().map(|p| [p[0] + x, p[1] + u, p[2]]).collect();
        let c: Vec<V3> = Mesh::ring(20, r + 1.0, z + 0.1, 0.0).iter().map(|p| [p[0] + x, p[1] + u, p[2]]).collect();
        b.m.loft(&a, &c, GRAVEL);
        for k in 0..5 {
            let ang = 3.6 + k as f64 * 0.3;
            let (hx, hu) = (x + ang.cos() * (r + 12.0), u + ang.sin() * (r + 12.0));
            b.m.frustum([hx - 3.0, hu - 3.0, z], [hx + 3.0, hu + 3.0, z], 1.0, z + 4.0, TREE);
        }
        // The Lattice climbs its walls.
        let segs = 3 + (i % 3);
        b.crack(x + r * 0.7, u - r * 0.7, -0.6, segs, z + 0.3);
    }
    // The instrument hall: long and low, a copper roof; and the sundial court with its gnomon.
    b.house(160.0, 3920.0, 120.0, 26.0, z, 8.0);
    let (sx, su) = (150.0, 4330.0);
    let a: Vec<V3> = Mesh::ring(24, 30.0, z + 0.2, 0.0).iter().map(|p| [p[0] + sx, p[1] + su, p[2]]).collect();
    b.m.poly(&a, STONE);
    for k in 0..12 {
        let ang = k as f64 / 12.0 * TAU;
        b.decal(sx + ang.cos() * 26.0 - 1.0, su + ang.sin() * 26.0 - 1.0, sx + ang.cos() * 26.0 + 1.0, su + ang.sin() * 26.0 + 1.0, z + 0.3, PAINT);
    }
    b.m.poly(&[[sx - 1.0, su - 14.0, z + 0.3], [sx + 1.0, su - 14.0, z + 0.3], [sx, su + 2.0, z + 16.0]], STONE);
    // Crystal thick on the paths, swallowing them toward the great dome.
    for i in 0..120 {
        let u = u1 - 160.0 - (i * i) as f64 * (u1 - u0 - 220.0) / (120.0 * 120.0);
        let x = b.range(0.0, WIDTH);
        let segs = (b.next() % 4 + 2) as usize;
        let dir = b.range(-1.0, 1.0);
        b.crack(x, u, dir, segs, z + 0.4);
    }
    // The great dome at the top: shutters shut (the ASTROLABE opens them), a gravel collar, crystal over
    // its rim.
    let (gx, gu) = GREAT_DOME;
    let a: Vec<V3> = Mesh::ring(32, GREAT_R + 14.0, z + 0.1, 0.0).iter().map(|p| [p[0] + gx, p[1] + gu, p[2]]).collect();
    let c: Vec<V3> = Mesh::ring(32, GREAT_R + 2.0, z + 0.1, 0.0).iter().map(|p| [p[0] + gx, p[1] + gu, p[2]]).collect();
    b.m.loft(&a, &c, GRAVEL);
    b.dome(gx, gu, GREAT_R, z + 6.0, 6.0);
    for k in 0..20 {
        let ang = k as f64 / 20.0 * TAU;
        let (x, u) = (gx + ang.cos() * (GREAT_R + 4.0), gu + ang.sin() * (GREAT_R + 4.0));
        if hash(k, 1, 61) > 0.4 {
            b.shards(x, u, z);
        }
    }
}

/// The whole stage at `t` (0..1) through its animation loop.
pub fn level_at(t: f64) -> Mesh {
    let mut b = Builder { m: Mesh::new(materials()), rng: SEED, t, spots: Spots::default() };
    vineyards(&mut b);
    aqueduct(&mut b);
    village(&mut b);
    observatory(&mut b);
    b.m
}

/// Stage lines `y0..y0 + h` from the top of the loop, as codes, 160 a line.
fn render_lines(mesh: &Mesh, shadows: &ShadowMap, y0: usize, h: usize) -> Vec<u8> {
    let period = demo::PANO_PERIOD as usize;
    let mut codes = vec![VOID; 160 * h];
    let strip = 400usize;
    let mut r0 = 0;
    while r0 < h {
        let n = strip.min(h - r0);
        let view = View { w: 160, h: n as u16, cx: 0.0, cy: (period - y0 - r0) as f64, scale: 1.0 };
        let img = raster3d::render_scene(mesh, view, &SUN, shadows);
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

/// The S2 panorama at `t` through the loop (one stage period plus its first 200 lines again).
pub fn panorama_at(t: f64) -> Panorama {
    let period = demo::PANO_PERIOD as usize;
    let mesh = level_at(t);
    let shadows = ShadowMap::new(&mesh, &SUN);
    let mut codes = render_lines(&mesh, &shadows, 0, period);
    let wrap = codes[..160 * 200].to_vec();
    codes.extend_from_slice(&wrap);
    Panorama { rows: ((period + 200) / 8) as u16, codes, palette: PALETTE }
}

/// Every frame of the S2 animation loop, rendered in parallel.
pub fn panorama_frames() -> Vec<Panorama> {
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get()).min(ANIM_FRAMES);
    let per = ANIM_FRAMES.div_ceil(threads);
    let mut out: Vec<Option<Panorama>> = (0..ANIM_FRAMES).map(|_| None).collect();
    std::thread::scope(|sc| {
        for (c, chunk) in out.chunks_mut(per).enumerate() {
            sc.spawn(move || {
                for (i, slot) in chunk.iter_mut().enumerate() {
                    *slot = Some(panorama_at((c * per + i) as f64 / ANIM_FRAMES as f64));
                }
            });
        }
    });
    out.into_iter().map(Option::unwrap).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(t: f64, lo: usize, hi: usize) -> Vec<u8> {
        let period = demo::PANO_PERIOD as usize;
        let mesh = level_at(t);
        let shadows = ShadowMap::new(&mesh, &SUN);
        render_lines(&mesh, &shadows, period - hi, hi - lo)
    }

    #[test]
    fn the_background_stays_down() {
        let p = panorama_at(0.0);
        let s = crate::s1::code_share(&p);
        assert!(s[2] <= 0.10, "light share {:.3} over the 10 % background budget", s[2]);
        assert!(s[3] <= 0.03, "hot share {:.3}: the Lattice only in the cracks", s[3]);
        assert!(s[1] >= 0.2, "dark share {:.3}: the vines, roofs and walls read", s[1]);
        assert_eq!(p.codes[..160 * 200], p.codes[p.codes.len() - 160 * 200..]);
    }

    #[test]
    fn the_animation_loops_without_a_seam() {
        // Vines, cypresses and a farmhouse; the aqueduct; the village lanes and the square; the fountain;
        // the observatory domes.
        for (lo, hi) in [(100, 300), (1580, 1780), (2380, 2580), (2860, 3060), (3500, 3700)] {
            assert!(lines(0.0, lo, hi) == lines(1.0, lo, hi), "lines {lo}..{hi} jump at the loop");
            assert!(lines(0.0, lo, hi) != lines(0.5, lo, hi), "lines {lo}..{hi} do not move");
        }
    }

    #[test]
    fn the_afternoon_shadows_fall_lower_left() {
        let mut m = Mesh::new(materials());
        m.frustum([0.0, 0.0, 0.0], [100.0, 100.0, 0.0], 0.0, 1.0, STONE);
        m.cuboid([40.0, 40.0, 1.0], [50.0, 50.0, 30.0], STONE);
        let sm = ShadowMap::new(&m, &SUN);
        assert!(sm.shadowed([34.0, 38.0, 1.0]), "down and to the left of the block");
        assert!(!sm.shadowed([58.0, 56.0, 1.0]), "toward the sun");
    }
}
