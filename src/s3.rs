//! The S3 REDLINE level (host only), built as 3D geometry by the art director and rendered offline under
//! the stage's own light (`docs/story/hd/02-ladder.md`: S3's hard low sun from the upper left, almost no
//! fill) with cast shadows, as `s1.rs` and `s2.rs` build theirs: 26 screens bottom to top, laid out as the
//! base bible's stage (`docs/story/04-levels/s3-redline.md`) and the HD direction
//! (`docs/story/hd/models/s3.md`): the OPEN DESERT (dunes, the highway, the single line, telegraph poles),
//! the MESA CANYON (the twin track weaving along the floor, tunnel spurs, the SIDEWINDER's straight run, the
//! signal gantry), the TRESTLE over a dry riverbed, and the FREIGHT YARD (a fan of a dozen tracks, boxcars,
//! water towers, the turntable, the engine shed, ore heaps) up to the TRESTLE TITAN's crane rails.
//!
//! Art direction: ground in `void` (red rock, every shadow) and `dark` (orange sand where the sun reaches
//! it); `light` (yellow) only for paint, lamps and glints; `hot` (cyan) only for crystal ore and the
//! Lattice. Everything that stands up throws a long shadow to the lower right. World units as `s2.rs`:
//! hires px across (x 0..320), stage lines up (`u`), z up. Everything that moves repeats a whole number of
//! times a loop and is keyed to world position or stable hashes, never the random stream.

use crate::bake::Panorama;
use crate::demo;
use crate::raster3d::{self, Light, Material, Mesh, ShadowMap, V3, View};
use crate::render::Palette;
use crate::s1::{hash, noise};

const VOID: u8 = 0;
const DARK: u8 = 1;
const LIGHT: u8 = 2;
const HOT: u8 = 3;

/// `clear::PALETTES[2]`: red rock, orange sand, yellow sun, cyan ore.
pub const PALETTE: Palette = Palette::new(2, 8, 7, 3);
/// The stage's time of day (the ladder's S3 `light`): a hard low sun from the upper left, almost no fill.
pub const SUN: Light = Light { dir: [-0.70, 0.30, 0.45], ambient: 0.08, intensity: 1.10 };

const WIDTH: f64 = 320.0;
const TAU: f64 = std::f64::consts::TAU;

/// Sections (stage lines): the desert up to the canyon mouth, where the mesas rise (screen 8); the canyon
/// to the riverbed; the yard from the far bank to the top.
const CANYON: (f64, f64) = (1640.0, 3196.0);
/// The dry riverbed the TRESTLE crosses (bank to bank) and its floor's depth.
const RIVER: (f64, f64) = (3196.0, 3384.0);
const RIVER_Z: f64 = -40.0;
/// The mesas' height over the canyon floor.
const MESA_Z: f64 = 70.0;
/// The desert's single line.
const DESERT_LINE: f64 = 210.0;

/// The tunnel spurs (rock over the canyon floor, from its lower to its upper mouth): RAIL CARs come out of
/// the first two in turn (screen 10), the SIDEWINDER out of the third (screen 12); the fourth closes its run.
pub const TUNNELS: [(f64, f64); 4] = [(1960.0, 2010.0), (2150.0, 2200.0), (2330.0, 2390.0), (2900.0, 2960.0)];
/// The SIDEWINDER's straight run: the canyon's left track at this x between these lines (the left third).
pub const SIDEWINDER_X: f64 = 82.0 - PAIR / 2.0;
pub const SIDEWINDER_RUN: (f64, f64) = (2390.0, 2860.0);
/// The signal gantry over the canyon's twin track.
pub const GANTRY_U: f64 = 3060.0;
/// The yard's dozen tracks (x where they run straight) and where the fan and the convergence happen.
pub const YARD_X: [f64; 12] = [16.0, 36.0, 56.0, 100.0, 120.0, 140.0, 180.0, 200.0, 220.0, 264.0, 284.0, 304.0];
const FAN: (f64, f64) = (3400.0, 3640.0);
const CONVERGE: (f64, f64) = (4780.0, 4900.0);
/// The tracks under the crane (yard tracks 3..=8) once converged; the outer ones end at buffer stops.
const CONVERGED: [f64; 6] = [110.0, 128.0, 146.0, 174.0, 192.0, 210.0];
const BUFFERS: f64 = 4790.0;
/// Water towers (the WATER TOWER's lid is a sprite over the level's lid here), between the tracks.
pub const WATER_TOWERS: [(f64, f64); 7] = [(78.0, 3820.0), (242.0, 3900.0), (160.0, 4040.0), (78.0, 4330.0), (242.0, 4420.0), (160.0, 4640.0), (78.0, 4700.0)];
/// The turntable pit (the hidden spark) and the engine shed over tracks 5 and 6.
pub const TURNTABLE: (f64, f64) = (160.0, 4200.0);
const TURNTABLE_R: f64 = 22.0;
const SHED: (f64, f64) = (4400.0, 4500.0);
/// Ore heaps beside the tracks (the second-to-last feeds a hopper car by conveyor).
pub const ORE_HEAPS: [(f64, f64); 8] = [(78.0, 3700.0), (242.0, 3760.0), (160.0, 3880.0), (78.0, 4110.0), (242.0, 4200.0), (78.0, 4540.0), (242.0, 4600.0), (160.0, 4760.0)];
/// The TRESTLE TITAN: its body's centre over the converged tracks, and its crane rails' x.
pub const TITAN: (f64, f64) = (160.0, 5000.0);
pub const CRANE_RAILS: [f64; 2] = [36.0, 284.0];
/// Rail gauge (between the two rails) and the canyon's distance between its two tracks.
const GAUGE: f64 = 8.0;
const PAIR: f64 = 22.0;

/// Material ids (`materials()` order).
const SAND: u8 = 0;
const ROCK: u8 = 1;
const BALLAST: u8 = 2;
const STEEL: u8 = 3;
const TIMBER: u8 = 4;
const ASPHALT: u8 = 5;
const FADED: u8 = 6;
const PAINT: u8 = 7;
const CRYSTAL: u8 = 8;
const MUD: u8 = 9;
const CAR: u8 = 10;
const SHADE: u8 = 11;
const BRUSH: u8 = 12;
const ORE: u8 = 13;
const SPILL: u8 = 14;
const FLOOR: u8 = 15;

/// The animation loop: the same length as S1's (the C64 picks the copy by the frame counter).
pub const ANIM_FRAMES: usize = crate::s1::ANIM_FRAMES;
pub const ANIM_SHIFT: u8 = crate::s1::ANIM_SHIFT;

fn materials() -> Vec<Material> {
    vec![
        // Sand: solid orange in the sun, red rock in every shadow and on the lee slopes.
        Material::lit(&[VOID, DARK], 0.95, 0),
        // Rock: mesa tops, the sunny canyon wall, outcrops, stones, concrete.
        Material::lit(&[VOID, DARK], 0.95, 0),
        // Ballast: the void/dark checker, so the rails' bed reads against plain sand.
        Material::lit(&[VOID, DARK], 0.62, 0),
        // Steel: rails, girders, signposts; a facet turned hard to the sun catches yellow.
        Material::lit(&[VOID, DARK, LIGHT], 0.62, 0),
        // Timber and the water towers' staves.
        Material::lit(&[VOID, DARK], 0.9, 0),
        // Asphalt.
        Material::glow(VOID),
        // The highway's faded centre dashes: a yellow checker.
        Material::lit(&[VOID, LIGHT], 0.62, 0),
        // Paint, lamps and glints.
        Material::glow(LIGHT),
        // The Lattice.
        Material::glow(HOT),
        // Cracked mud: dark in the sun.
        Material::lit(&[VOID, DARK], 0.95, 0),
        // Boxcars and flatcars.
        Material::lit(&[VOID, DARK], 0.9, 0),
        // Shade: sleepers, tunnel mouths, pits, the mud's cracks.
        Material::glow(VOID),
        // Scrub and tumbleweeds: void with a dark checker where the sun hits.
        Material::lit(&[VOID, DARK], 0.4, 0),
        // Ore heaps: cyan facing the sun, dark away from it.
        Material::lit(&[DARK, HOT], 0.8, 0),
        // Sand in the air: blown off the crests, pouring off the rim.
        Material::glow(DARK),
        // The canyon floor: a checker where the sun reaches it, so the solid mesa tops stand above it.
        Material::lit(&[VOID, DARK], 0.62, 0),
    ]
}

struct Builder {
    m: Mesh,
    rng: u32,
    /// Position in the animation loop, 0..1.
    t: f64,
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
    /// A strip `w` wide from (`x0`, `u0`) to (`x1`, `u1`) at `z`.
    fn strip(&mut self, x0: f64, u0: f64, x1: f64, u1: f64, w: f64, z: f64, mat: u8) {
        let (dx, du) = (x1 - x0, u1 - u0);
        let l = dx.hypot(du).max(1e-6);
        let (px, pu) = (du / l * w / 2.0, -dx / l * w / 2.0);
        self.m.poly(&[[x0 - px, u0 - pu, z], [x0 + px, u0 + pu, z], [x1 + px, u1 + pu, z], [x1 - px, u1 - pu, z]], mat);
    }
    /// A Lattice crack heading `dir` (radians, 0 = up the stage): it pulses and creeps as S1's do.
    fn crack(&mut self, mut x: f64, mut u: f64, dir: f64, segs: usize, z: f64) {
        let ph = hash(x.round() as i64, u.round() as i64, 43);
        let reach = segs as f64 * (0.55 + 0.45 * (0.5 + 0.5 * (TAU * (self.t + ph)).sin()));
        for k in 0..segs {
            let (a, len) = (dir + self.range(-0.7, 0.7), self.range(3.0, 7.0));
            let part = (reach - k as f64).clamp(0.0, 1.0);
            if part < 0.05 {
                continue;
            }
            let (nx, nu) = (x + a.sin() * len * part, u + a.cos() * len * part);
            let s = (TAU * 2.0 * self.t - u / 40.0).sin().max(0.0);
            self.strip(x, u, nx, nu, 2.2 * (0.75 + 0.6 * s * s), z, CRYSTAL);
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
    /// A cone of `mat` (an ore heap, a tumbleweed's cap) of radius `r` and height `h` on ground `z`.
    fn cone(&mut self, x: f64, u: f64, z: f64, r: f64, h: f64, mat: u8) {
        let at = |p: &V3| [p[0] + x, p[1] + u, p[2]];
        let base: Vec<V3> = Mesh::ring(10, r, z, 0.3).iter().map(at).collect();
        let mid: Vec<V3> = Mesh::ring(10, r * 0.55, z + h * 0.6, 0.6).iter().map(at).collect();
        self.m.loft(&base, &mid, mat);
        for i in 0..mid.len() {
            self.m.tri(mid[i], mid[(i + 1) % mid.len()], [x, u, z + h], mat);
        }
    }
    /// A telegraph pole with its crossarm at (`x`, `u`) on ground `z`.
    fn pole(&mut self, x: f64, u: f64, z: f64) {
        self.m.tube([x, u, z], [x, u, z + 22.0], 1.3, 4, TIMBER, TIMBER);
        self.m.beam([x - 6.0, u, z + 20.0], [x + 6.0, u, z + 20.0], 1.2, 1.2, TIMBER);
    }
}

fn smooth(a: f64, b: f64, x: f64) -> f64 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// A curve through `k` (u, value): eased between knots, flat beyond the ends.
fn knots(k: &[(f64, f64)], u: f64) -> f64 {
    if u <= k[0].0 {
        return k[0].1;
    }
    for w in k.windows(2) {
        if u <= w[1].0 {
            return w[0].1 + (w[1].1 - w[0].1) * smooth(w[0].0, w[1].0, u);
        }
    }
    k[k.len() - 1].1
}

// ------------------------------------------------------------------------------------------ the rails

/// The centre of the line before the yard: the desert's single line, the canyon's twin track weaving
/// along the floor (straight up the left third for the SIDEWINDER), the trestle up the middle.
const LINE: [(f64, f64); 10] = [
    (1700.0, DESERT_LINE), (1850.0, 205.0), (2000.0, 170.0), (2150.0, 120.0), (2300.0, 88.0), (2350.0, 82.0),
    (2860.0, 82.0), (2960.0, 115.0), (3100.0, 165.0), (3180.0, 160.0),
];

/// How far apart the line's two tracks run at `u`: one line in the desert and on the trestle, two in
/// the canyon.
fn pair(u: f64) -> f64 {
    PAIR * (smooth(1700.0, 1800.0, u) - smooth(3120.0, 3180.0, u))
}

/// The centre line of track `track` at stage line `u`: before the yard tracks 0 (left) and 1 (right) of
/// the line (one line where they merge; higher tracks follow their parity), in the yard its dozen
/// (`YARD_X`, fanning out from the trestle's line and closing up under the crane). RAIL CARs, the
/// SIDEWINDER and the TRESTLE TITAN ride these.
pub fn rail_x(track: usize, u: f64) -> f64 {
    if u < RIVER.1 {
        let side = if track % 2 == 0 { -0.5 } else { 0.5 };
        return knots(&LINE, u) + side * pair(u);
    }
    let i = track.min(11);
    let x = 160.0 + (YARD_X[i] - 160.0) * smooth(FAN.0, FAN.1, u);
    if (3..=8).contains(&i) { x + (CONVERGED[i - 3] - x) * smooth(CONVERGE.0, CONVERGE.1, u) } else { x }
}

/// One track along `x_of` from `u0` to `u1`: ballast (left out over `open`, the trestle's span), sleepers
/// and two rails, the rails' glints running up them one spacing a loop.
fn track(b: &mut Builder, x_of: &dyn Fn(f64) -> f64, u0: f64, u1: f64, open: (f64, f64)) {
    let step = 4.0;
    let n = ((u1 - u0) / step).ceil() as usize;
    for k in 0..n {
        let (ua, uc) = (u0 + k as f64 * step, (u0 + (k + 1) as f64 * step).min(u1));
        let (xa, xc) = (x_of(ua), x_of(uc));
        let l = (xc - xa).hypot(uc - ua).max(1e-6);
        let (px, pu) = ((uc - ua) / l, -(xc - xa) / l);
        let at = |x: f64, u: f64, o: f64, z: f64| -> V3 { [x + px * o, u + pu * o, z] };
        let over = ua >= open.0 && uc <= open.1;
        if !over {
            // The bed: a flat top on sloping shoulders.
            for (o0, z0, o1, z1) in [(-9.0, 0.0, -6.0, 1.5), (-6.0, 1.5, 6.0, 1.5), (6.0, 1.5, 9.0, 0.0)] {
                b.m.poly(&[at(xa, ua, o0, z0), at(xa, ua, o1, z1), at(xc, uc, o1, z1), at(xc, uc, o0, z0)], BALLAST);
            }
        }
        // A sleeper every 4 lines, across the track.
        let (dx, du) = ((xc - xa) / l, (uc - ua) / l);
        let s = |o: f64, w: f64| -> V3 { [xa + px * o + dx * w, ua + pu * o + du * w, 1.9] };
        b.m.poly(&[s(-7.0, 0.0), s(7.0, 0.0), s(7.0, 1.6), s(-7.0, 1.6)], SHADE);
        for o in [-GAUGE / 2.0, GAUGE / 2.0] {
            b.m.poly(&[at(xa, ua, o - 0.8, 2.4), at(xa, ua, o + 0.8, 2.4), at(xc, uc, o + 0.8, 2.4), at(xc, uc, o - 0.8, 2.4)], STEEL);
        }
    }
    // Glints running up each rail, 48 lines apart, one spacing a loop (keyed to where they are).
    for (r, o) in [(0, -GAUGE / 2.0), (1, GAUGE / 2.0)] {
        let first = (u0 / 48.0).floor() as i64 - 1;
        for k in first..first + ((u1 - u0) / 48.0) as i64 + 3 {
            let gu = k as f64 * 48.0 + r as f64 * 24.0 + 48.0 * b.t;
            if gu < u0 || gu + 3.0 > u1 {
                continue;
            }
            let gx = x_of(gu) + o;
            if !sunlit(gx, gu) {
                continue;
            }
            b.decal(gx - 0.8, gu, gx + 0.8, gu + 3.0, 2.5, PAINT);
        }
    }
}

// ------------------------------------------------------------------------------------------ the desert

/// The highway's centre line: it wanders up the left of the desert and bends off the left edge before the
/// canyon.
fn highway_x(u: f64) -> f64 {
    76.0 + 14.0 * (u / 310.0).sin() - 150.0 * smooth(1350.0, 1620.0, u)
}

/// How much dune there is at (`x`, `u`): none on the highway or the line, fading out toward the canyon.
fn dune_amp(x: f64, u: f64) -> f64 {
    let dh = (x - highway_x(u)).abs() - 16.0;
    let dr = (x - DESERT_LINE).abs() - 13.0;
    smooth(0.0, 24.0, dh.min(dr)) * (1.0 - smooth(1450.0, CANYON.0, u))
}

/// Dune phase at (`x`, `u`): crests where its fraction is 0.2.
fn dune_phase(x: f64, u: f64) -> f64 {
    (u + 20.0 * (x / 60.0).sin() + 9.0 * (x / 23.0 + u / 90.0).sin()) / 72.0
}

/// The desert's height: transverse dunes, a steep lee rising up the stage to a sharp crest (facing away
/// from the sun: red rock), then a long gentle windward slope facing it (orange).
fn dune(x: f64, u: f64) -> f64 {
    let a = dune_amp(x, u);
    if a <= 0.0 {
        return 0.0;
    }
    let f = dune_phase(x, u).rem_euclid(1.0);
    let shape = if f < 0.2 { f / 0.2 } else { (1.0 - f) / 0.8 };
    a * (10.0 + 8.0 * noise(x, u, 90.0, 140.0, 3)) * shape
}

const SEED: u32 = 0x5EA_0003;

fn desert(b: &mut Builder) {
    let (u0, u1) = (-24.0, CANYON.0 + 8.0);
    // The dune field as a height field (8 px × 4 lines), two triangles a cell.
    let xs: Vec<f64> = (0..=42).map(|i| -8.0 + i as f64 * 8.0).collect();
    let rows = ((u1 - u0) / 4.0) as usize;
    let h: Vec<Vec<f64>> = (0..=rows).map(|r| xs.iter().map(|&x| dune(x, u0 + r as f64 * 4.0)).collect()).collect();
    for r in 0..rows {
        let (ua, uc) = (u0 + r as f64 * 4.0, u0 + (r + 1) as f64 * 4.0);
        for i in 0..xs.len() - 1 {
            let (p00, p10) = ([xs[i], ua, h[r][i]], [xs[i + 1], ua, h[r][i + 1]]);
            let (p01, p11) = ([xs[i], uc, h[r + 1][i]], [xs[i + 1], uc, h[r + 1][i + 1]]);
            b.m.tri(p00, p10, p11, SAND);
            b.m.tri(p00, p11, p01, SAND);
        }
    }
    // Sand streaming off the crests into the lee shadow, downwind (down the stage), two wisps a site.
    let mut x = 4.0;
    while x < WIDTH - 4.0 {
        for k in 0..((u1 / 72.0) as i64) {
            if hash(x as i64, k, 13) > 0.45 {
                continue;
            }
            // Solve for the crest's line at this x (phase fraction 0.2).
            let mut u = (k as f64 + 0.2) * 72.0;
            for _ in 0..4 {
                u = (k as f64 + 0.2) * 72.0 - 20.0 * (x / 60.0).sin() - 9.0 * (x / 23.0 + u / 90.0).sin();
            }
            let top = dune(x, u);
            if top < 6.0 {
                continue;
            }
            for w in 0..2 {
                let d = (b.t + hash(x as i64, k, 17 + w)).rem_euclid(1.0);
                let (wx, wu) = (x + 3.0 * d + w as f64 * 2.0, u - 1.0 - 11.0 * d);
                b.decal(wx - 1.0, wu - 1.4, wx + 1.0, wu, top + 0.6 - 2.0 * d, SPILL);
            }
        }
        x += 12.0;
    }
    // Flat rock outcrops among the dunes.
    for _ in 0..12 {
        let (x, u) = (b.range(10.0, 310.0), b.range(80.0, 1450.0));
        if dune_amp(x, u) < 0.9 {
            continue;
        }
        let (w, l, z) = (b.range(14.0, 30.0), b.range(10.0, 22.0), dune(x, u));
        let top = z + b.range(5.0, 9.0);
        b.m.frustum([x - w / 2.0, u - l / 2.0, z - 3.0], [x + w / 2.0, u + l / 2.0, z - 3.0], 2.5, top, ROCK);
        b.m.frustum([x - w / 2.0 - 4.0, u - l / 2.0 + 2.0, z - 3.0], [x - w / 2.0 + 6.0, u + 2.0, z - 3.0], 1.5, z + 3.0, ROCK);
    }
    // Crystal clusters in the dunes, the Lattice creeping out from them.
    for _ in 0..16 {
        let (x, u) = (b.range(10.0, 310.0), b.range(60.0, 1500.0));
        if dune_amp(x, u) < 0.6 {
            continue;
        }
        let z = dune(x, u);
        b.shards(x, u, z);
        for c in 0..3 {
            let dir = c as f64 * 2.1 + b.range(-0.4, 0.4);
            b.crack(x, u, dir, 3, z + 0.4);
        }
    }
    // The highway: two lanes of asphalt, faded centre dashes, sand drifting in over the edges.
    let step = 8.0;
    let mut u = u0;
    while u < u1 {
        let (xa, xc) = (highway_x(u), highway_x(u + step));
        if xa.max(xc) > -20.0 {
            b.m.poly(&[[xa - 12.0, u, 0.2], [xa + 12.0, u, 0.2], [xc + 12.0, u + step, 0.2], [xc - 12.0, u + step, 0.2]], ASPHALT);
            if (u / step).rem_euclid(3.0) < 1.0 {
                b.m.poly(&[[xa - 1.6, u, 0.3], [xa + 1.6, u, 0.3], [xc + 1.6, u + step, 0.3], [xc - 1.6, u + step, 0.3]], FADED);
            }
            for side in [-1.0, 1.0] {
                if hash(u as i64, side as i64, 19) > 0.45 {
                    let (reach, len) = (2.0 + 6.0 * hash(u as i64, side as i64, 20), 8.0 + 14.0 * hash(u as i64, side as i64, 21));
                    let e = xa + side * 12.5;
                    b.m.poly(&[[e + side * 1.5, u, 0.35], [e - side * reach, u + len * 0.4, 0.35], [e + side * 1.5, u + len, 0.35]], SAND);
                }
            }
        }
        u += step;
    }
    // A rusted signpost on the shoulder, twice.
    for su in [640.0, 1180.0] {
        let x = highway_x(su) + 18.0;
        b.m.tube([x, su, 0.0], [x, su, 15.0], 0.9, 4, STEEL, STEEL);
        b.m.cuboid([x - 5.0, su - 0.8, 12.0], [x + 5.0, su + 0.8, 17.0], STEEL);
    }
    // Tumbleweeds rolling across the highway, left to right, 60 px apart, one spacing a loop.
    for (lane, du) in [(300.0, 0.0), (880.0, 9.0), (1260.0, -6.0)] {
        let hx = highway_x(lane);
        for k in -2..3 {
            let wx = hx + k as f64 * 60.0 + 60.0 * b.t;
            if (wx - hx).abs() > 46.0 || !(-4.0..=WIDTH + 4.0).contains(&wx) {
                continue;
            }
            let wu = lane + du + 3.0 * (wx / 17.0).sin();
            let z = dune(wx, wu) + 0.4 + 1.6 * (TAU * wx / 30.0).sin().abs();
            b.cone(wx, wu, z, 3.0, 2.6, BRUSH);
            b.cone(wx, wu, z, 3.0, -1.0, BRUSH);
        }
    }
    // Telegraph poles beside the line, throwing long thin shadows across the sand.
    let mut u = 30.0;
    while u < CANYON.0 {
        b.pole(DESERT_LINE + 22.0, u, 0.0);
        u += 64.0;
    }
}

// ------------------------------------------------------------------------------------------ the canyon

/// The canyon floor's left and right edges (the foot of each wall).
const XL: [(f64, f64); 10] = [
    (1640.0, -8.0), (1700.0, 20.0), (1800.0, 40.0), (2000.0, 62.0), (2200.0, 46.0), (2300.0, 22.0), (2860.0, 22.0), (3000.0, 58.0), (3120.0, 44.0), (3180.0, 30.0),
];
const XR: [(f64, f64); 10] = [
    (1640.0, 328.0), (1700.0, 300.0), (1800.0, 276.0), (2000.0, 248.0), (2200.0, 214.0), (2300.0, 262.0), (2860.0, 290.0), (3000.0, 252.0), (3120.0, 272.0), (3180.0, 290.0),
];

fn xl(u: f64) -> f64 {
    knots(&XL, u) + 8.0 * (noise(0.0, u, 1.0, 37.0, 71) - 0.5)
}
fn xr(u: f64) -> f64 {
    knots(&XR, u) - 8.0 * (noise(0.0, u, 1.0, 41.0, 73) - 0.5)
}
/// The mesas' height at `u`: they rise at the canyon mouth and fall away at the riverbed.
fn mesa_z(u: f64) -> f64 {
    MESA_Z * smooth(CANYON.0, CANYON.0 + 100.0, u) * (1.0 - smooth(3120.0, 3190.0, u))
}
/// The walls' batter (how far the rim stands back from the foot), proportional to the height.
fn batter(u: f64) -> f64 {
    12.0 * mesa_z(u) / MESA_Z
}

/// Whether the sun reaches the canyon floor at (`x`, `u`) past the left mesa's and the tunnel spurs'
/// shadows (a shadow falls 1.56 px right and 0.67 lines down per unit of height): where rails glint.
fn sunlit(x: f64, u: f64) -> bool {
    if !(CANYON.0..CANYON.1).contains(&u) {
        return true;
    }
    let z = mesa_z(u + 0.67 * MESA_Z).max(mesa_z(u));
    let spur = TUNNELS.iter().any(|&(t0, t1)| u > t0 - 0.67 * MESA_Z && u < t1);
    !spur && x > xl(u) - batter(u) + 1.56 * z
}

fn canyon(b: &mut Builder) {
    let (u0, u1) = CANYON;
    // The floor: dust and gravel, which the left mesa's shadow covers but for a strip under the far wall.
    b.m.poly(&[[-8.0, u0, 0.0], [WIDTH + 8.0, u0, 0.0], [WIDTH + 8.0, u1, 0.0], [-8.0, u1, 0.0]], FLOOR);
    // The mesas: flat tops, sheer walls (the left one faces away from the sun, the right one into it).
    let step = 6.0;
    let mut u = u0;
    while u < u1 {
        let uc = (u + step).min(u1);
        let (za, zc) = (mesa_z(u), mesa_z(uc));
        if za.max(zc) > 0.01 {
            let (la, lc) = (xl(u), xl(uc));
            let (ra, rc) = (xr(u), xr(uc));
            let (ba, bc) = (batter(u), batter(uc));
            b.m.poly(&[[-8.0, u, za], [la - ba, u, za], [lc - bc, uc, zc], [-8.0, uc, zc]], ROCK);
            b.m.poly(&[[la - ba, u, za], [la, u, 0.0], [lc, uc, 0.0], [lc - bc, uc, zc]], ROCK);
            b.m.poly(&[[ra, u, 0.0], [ra + ba, u, za], [rc + bc, uc, zc], [rc, uc, 0.0]], ROCK);
            b.m.poly(&[[ra + ba, u, za], [WIDTH + 8.0, u, za], [WIDTH + 8.0, uc, zc], [rc + bc, uc, zc]], ROCK);
        }
        u = uc;
    }
    // Scrub on the mesa tops.
    for _ in 0..220 {
        let u = b.range(u0 + 110.0, 3110.0);
        let (z, left) = (mesa_z(u), b.next() % 2 == 0);
        let x = if left { b.range(-6.0, xl(u) - batter(u) - 3.0) } else { b.range(xr(u) + batter(u) + 3.0, WIDTH + 6.0) };
        if !(-6.0..=WIDTH + 6.0).contains(&x) {
            continue;
        }
        let r = b.range(1.5, 3.0);
        let top = z + b.range(1.5, 3.0);
        b.m.frustum([x - r, u - r, z], [x + r, u + r, z], r * 0.5, top, BRUSH);
    }
    // Crystal veins down the sunny right wall (pulsing and creeping down from the rim), cracks on its rim.
    for i in 0..14 {
        let vu = 1780.0 + i as f64 * 98.0 + 30.0 * hash(i, 0, 81);
        let (z, r, bt) = (mesa_z(vu), xr(vu), batter(vu));
        if z < 20.0 {
            continue;
        }
        let ph = hash(i, 1, 83);
        let reach = 0.6 + 0.4 * (0.5 + 0.5 * (TAU * (b.t + ph)).sin());
        let segs = 6;
        let mut du = 0.0;
        for k in 0..segs {
            let (f0, f1) = (k as f64 / segs as f64, (k + 1) as f64 / segs as f64);
            // Draw only as far as it reaches, but take every step's jitter (the random stream must not
            // depend on `t`).
            let ndu = du + b.range(-2.5, 2.5);
            if f1 > reach + 1e-9 {
                du = ndu;
                continue;
            }
            let (z0, z1) = (z * (1.0 - f0), z * (1.0 - f1));
            let (x0, x1) = (r + bt * z0 / z - 0.6, r + bt * z1 / z - 0.6);
            b.m.poly(&[[x0, vu + du - 0.9, z0 + 0.3], [x0, vu + du + 0.9, z0 + 0.3], [x1, vu + ndu + 0.9, z1 + 0.3], [x1, vu + ndu - 0.9, z1 + 0.3]], CRYSTAL);
            du = ndu;
        }
        let dir = 1.2 + b.range(-0.5, 0.5);
        b.crack(r + bt + 2.0, vu, dir, 4, z + 0.3);
    }
    // Sand pouring off the left rim into the shadow, down the wall in dashes (twice a loop).
    for i in 0..7 {
        let pu = 1820.0 + i as f64 * 190.0 + 40.0 * hash(i, 2, 87);
        let (z, l, bt) = (mesa_z(pu), xl(pu), batter(pu));
        if z < 20.0 {
            continue;
        }
        for k in 0..4 {
            let f = (k as f64 / 4.0 + 2.0 * b.t).rem_euclid(1.0);
            let (fa, fc) = (f, (f + 0.18).min(1.0));
            let p = |g: f64| -> V3 { [l - bt * (1.0 - g) + 0.5, pu, z * (1.0 - g) + 0.5] };
            let (a, c) = (p(fa), p(fc));
            b.m.poly(&[[a[0], pu - 1.2, a[2]], [a[0], pu + 1.2, a[2]], [c[0], pu + 1.4, c[2]], [c[0], pu - 1.4, c[2]]], SPILL);
        }
        b.cone(l + 3.0, pu, 0.0, 4.0, 2.0, SAND);
    }
    // Tunnel spurs: rock bridging the canyon over the tracks, a dark mouth at each end.
    for &(t0, t1) in &TUNNELS {
        let (z, ra) = (mesa_z(t0).min(mesa_z(t1)), 8.0);
        let (l, r) = (xl(t0).min(xl(t1)) - 12.0, xr(t0).max(xr(t1)) + 12.0);
        b.m.poly(&[[l, t0, 0.0], [r, t0, 0.0], [r, t0 + ra, z], [l, t0 + ra, z]], ROCK);
        b.m.poly(&[[l, t0 + ra, z], [r, t0 + ra, z], [r, t1 - ra, z], [l, t1 - ra, z]], ROCK);
        b.m.poly(&[[l, t1 - ra, z], [r, t1 - ra, z], [r, t1, 0.0], [l, t1, 0.0]], ROCK);
        for (mu, up) in [(t0, false), (t1, true)] {
            let c = knots(&LINE, mu);
            let w = pair(mu) / 2.0 + 12.0;
            let rise = 16.0 / z * ra;
            let (inner, h) = if up { (mu - rise, 16.0) } else { (mu + rise, 16.0) };
            b.m.poly(&[[c - w, mu, 0.3], [c + w, mu, 0.3], [c + w, inner, h + 0.3], [c - w, inner, h + 0.3]], SHADE);
            // Timber portal posts either side.
            for s in [-1.0, 1.0] {
                let px = c + s * (w + 1.5);
                b.m.cuboid([px - 1.5, mu - 1.5, 0.0], [px + 1.5, mu + 1.5, 14.0], TIMBER);
            }
            b.m.beam([c - w - 3.0, mu, 15.0], [c + w + 3.0, mu, 15.0], 2.5, 2.0, TIMBER);
        }
    }
    // Rockfall boulders on the floor, clear of the tracks.
    for _ in 0..70 {
        let u = b.range(u0 + 80.0, 3150.0);
        let x = b.range(xl(u) + 3.0, xr(u) - 3.0);
        let c = knots(&LINE, u);
        if (x - c).abs() < pair(u) / 2.0 + 14.0 || TUNNELS.iter().any(|&(a, e)| u > a - 6.0 && u < e + 6.0) {
            continue;
        }
        let (rr, h) = (b.range(2.0, 6.0), b.range(2.0, 6.0));
        b.m.frustum([x - rr, u - rr * 0.8, 0.0], [x + rr, u + rr * 0.8, 0.0], rr * 0.5, h, ROCK);
    }
    // Telegraph poles along the right of the line; some have fallen beside it.
    let mut u = CANYON.0 + 40.0;
    while u < 3120.0 {
        let x = knots(&LINE, u) + pair(u) / 2.0 + 20.0;
        if x < xr(u) - 4.0 && !TUNNELS.iter().any(|&(a, e)| u > a - 10.0 && u < e + 10.0) {
            if hash(u as i64, 3, 89) < 0.3 {
                let a = 0.5 + 1.5 * hash(u as i64, 4, 91);
                b.m.tube([x, u, 1.0], [x + a.cos() * 20.0, u - a.sin() * 20.0, 1.0], 0.9, 4, TIMBER, TIMBER);
            } else {
                b.pole(x, u, 0.0);
            }
        }
        u += 80.0;
    }
    // The rusted signal gantry over both tracks, its lamps changing (six times a loop).
    let (gu, gc) = (GANTRY_U, knots(&LINE, GANTRY_U));
    let half = pair(gu) / 2.0 + 14.0;
    for s in [-1.0, 1.0] {
        let px = gc + s * half;
        b.m.cuboid([px - 1.5, gu - 1.5, 0.0], [px + 1.5, gu + 1.5, 26.0], STEEL);
    }
    b.m.cuboid([gc - half - 2.0, gu - 1.5, 24.0], [gc + half + 2.0, gu + 1.5, 27.0], STEEL);
    let state = (b.t * 6.0) as usize;
    for (h, side) in [-0.5, 0.5].iter().enumerate() {
        let hx = gc + side * pair(gu);
        b.m.cuboid([hx - 2.0, gu - 2.5, 27.0], [hx + 2.0, gu + 2.5, 29.0], STEEL);
        for k in 0..3 {
            let lit = (state + h) % 3 == k;
            let lu = gu - 1.6 + k as f64 * 1.6;
            b.decal(hx - 1.0, lu - 0.6, hx + 1.0, lu + 0.6, 29.1, if lit { PAINT } else { SHADE });
        }
    }
}

// ----------------------------------------------------------------------------------------- the trestle

/// The dry riverbed (cracked mud, round stones) and the TRESTLE over it: timber bents under a steel deck,
/// whose lattice the low sun throws across the mud.
fn trestle(b: &mut Builder) {
    let (u0, u1) = RIVER;
    let z = RIVER_Z;
    let (f0, f1) = (u0 + 26.0, u1 - 26.0);
    // The banks: the near one slopes down facing the sun, the far one climbs out of the shade.
    let xs: Vec<f64> = (0..=16).map(|i| -8.0 + i as f64 * (WIDTH + 16.0) / 16.0).collect();
    for i in 0..xs.len() - 1 {
        let (x0, x1) = (xs[i], xs[i + 1]);
        let w = |x: f64, s: u64| 5.0 * (noise(x, 0.0, 30.0, 1.0, s) - 0.5);
        b.m.poly(&[[x0, u0 + w(x0, 5), 0.0], [x1, u0 + w(x1, 5), 0.0], [x1, f0 + w(x1, 6), z], [x0, f0 + w(x0, 6), z]], SAND);
        b.m.poly(&[[x0, f0 + w(x0, 6), z], [x1, f0 + w(x1, 6), z], [x1, f1 + w(x1, 7), z], [x0, f1 + w(x0, 7), z]], MUD);
        b.m.poly(&[[x0, f1 + w(x0, 7), z], [x1, f1 + w(x1, 7), z], [x1, u1 + w(x1, 8), 0.0], [x0, u1 + w(x0, 8), 0.0]], SAND);
    }
    // The mud's cracks: a jittered net of plates.
    let cell = 15.0;
    let pt = |i: i64, j: i64| -> (f64, f64) { (i as f64 * cell + 9.0 * (hash(i, j, 95) - 0.5), f0 + 4.0 + j as f64 * cell + 9.0 * (hash(i, j, 96) - 0.5)) };
    let nj = ((f1 - f0 - 8.0) / cell) as i64;
    for i in 0..=(WIDTH / cell) as i64 {
        for j in 0..=nj {
            let a = pt(i, j);
            for (di, dj) in [(1, 0), (0, 1)] {
                if j + dj > nj || hash(i * 2 + di, j * 2 + dj, 97) < 0.15 {
                    continue;
                }
                let c = pt(i + di, j + dj);
                b.strip(a.0, a.1, c.0, c.1, 1.3, z + 0.1, SHADE);
            }
        }
    }
    // Round stones.
    for _ in 0..55 {
        let (x, u) = (b.range(0.0, WIDTH), b.range(f0 + 2.0, f1 - 2.0));
        let r = b.range(1.5, 3.5);
        b.m.frustum([x - r, u - r, z], [x + r, u + r, z], r * 0.45, z + r * 0.8, ROCK);
    }
    // The trestle: a bent every 14 lines (battered posts, cap, waling, X bracing), longitudinal walings
    // and braces between them, two steel stringers under the track.
    let c = knots(&LINE, u0);
    let post = |s: f64, zz: f64| -> f64 { c + s * (8.0 + 12.0 * (-zz) / (-z)) };
    let mut prev: Option<f64> = None;
    let mut u = u0 + 10.0;
    while u < u1 - 6.0 {
        for s in [-1.0, 1.0] {
            b.m.beam([post(s, -1.0), u, -1.0], [post(s, z), u, z], 1.8, 1.8, TIMBER);
        }
        b.m.beam([post(-1.0, -2.0), u, -2.0], [post(1.0, -2.0), u, -2.0], 1.6, 1.6, TIMBER);
        b.m.beam([post(-1.0, -20.0), u, -20.0], [post(1.0, -20.0), u, -20.0], 1.4, 1.4, TIMBER);
        b.m.beam([post(-1.0, -4.0), u, -4.0], [post(1.0, -18.0), u, -18.0], 1.2, 1.2, TIMBER);
        b.m.beam([post(1.0, -4.0), u, -4.0], [post(-1.0, -18.0), u, -18.0], 1.2, 1.2, TIMBER);
        b.m.beam([post(-1.0, -22.0), u, -22.0], [post(1.0, -36.0), u, -36.0], 1.2, 1.2, TIMBER);
        b.m.beam([post(1.0, -22.0), u, -22.0], [post(-1.0, -36.0), u, -36.0], 1.2, 1.2, TIMBER);
        if let Some(p) = prev {
            for s in [-1.0, 1.0] {
                b.m.beam([post(s, -20.0), p, -20.0], [post(s, -20.0), u, -20.0], 1.2, 1.2, TIMBER);
                b.m.beam([post(s, -4.0), p, -4.0], [post(s, -18.0), u, -18.0], 1.0, 1.0, TIMBER);
            }
        }
        prev = Some(u);
        u += 14.0;
    }
    for s in [-1.0, 1.0] {
        b.m.beam([c + s * 5.0, u0 - 2.0, 0.0], [c + s * 5.0, u1 + 2.0, 0.0], 2.0, 2.0, STEEL);
    }
}

// -------------------------------------------------------------------------------------------- the yard

/// A boxcar (ridged roof along it) or a flatcar (with crates) centred on `x`, from `u` for `l` lines.
fn boxcar(b: &mut Builder, x: f64, u: f64, l: f64) {
    b.m.cuboid([x - 7.0, u, 2.5], [x + 7.0, u + l, 12.0], CAR);
    b.m.poly(&[[x - 7.5, u, 12.0], [x, u, 14.0], [x, u + l, 14.0], [x - 7.5, u + l, 12.0]], CAR);
    b.m.poly(&[[x + 7.5, u, 12.0], [x, u, 14.0], [x, u + l, 14.0], [x + 7.5, u + l, 12.0]], CAR);
}
fn flatcar(b: &mut Builder, x: f64, u: f64, l: f64) {
    b.m.cuboid([x - 7.0, u, 2.5], [x + 7.0, u + l, 4.2], CAR);
    let crates = b.next() % 3;
    for k in 0..crates {
        let cu = u + 4.0 + k as f64 * 14.0;
        b.m.cuboid([x - 5.0, cu, 4.2], [x + 5.0, cu + 10.0, 9.0], STEEL);
    }
}

/// A water tower on splayed legs at (`x`, `u`): braced legs, a staved tank with rust bands, its lid.
fn water_tower(b: &mut Builder, x: f64, u: f64) {
    let (zb, zt) = (32.0, 46.0);
    let legs: Vec<(f64, f64)> = (0..4).map(|k| (TAU / 8.0 + k as f64 * TAU / 4.0).sin_cos()).collect();
    for (k, &(s, c)) in legs.iter().enumerate() {
        b.m.beam([x + s * 15.0, u + c * 15.0, 0.0], [x + s * 9.0, u + c * 9.0, zb], 1.6, 1.6, STEEL);
        let (s2, c2) = legs[(k + 1) % 4];
        b.m.beam([x + s * 13.5, u + c * 13.5, 8.0], [x + s2 * 11.0, u + c2 * 11.0, 24.0], 0.9, 0.9, STEEL);
    }
    let at = |r: f64, z: f64| -> Vec<V3> { Mesh::ring(16, r, z, 0.2).iter().map(|p| [p[0] + x, p[1] + u, p[2]]).collect() };
    b.m.poly(&at(13.0, zb), TIMBER);
    b.m.loft(&at(13.0, zb), &at(13.0, zt), TIMBER);
    for bz in [zb + 3.0, zt - 4.0] {
        b.m.loft(&at(13.4, bz), &at(13.4, bz + 1.2), STEEL);
    }
    b.m.loft(&at(13.0, zt), &at(4.0, zt + 4.0), TIMBER);
    b.m.poly(&at(4.0, zt + 4.0), TIMBER);
}

fn yard(b: &mut Builder) {
    let (u0, u1) = (RIVER.1, demo::PANO_PERIOD as f64 + 30.0);
    b.m.poly(&[[-8.0, u0, 0.0], [WIDTH + 8.0, u0, 0.0], [WIDTH + 8.0, u1, 0.0], [-8.0, u1, 0.0]], SAND);
    // Oil stains between the tracks.
    for _ in 0..40 {
        let (x, u) = (b.range(0.0, WIDTH), b.range(3700.0, 4900.0));
        let r = b.range(2.0, 5.0);
        let ring: Vec<V3> = Mesh::ring(7, r, 0.1, b.range(0.0, 1.0)).iter().map(|p| [p[0] + x, p[1] * 1.4 + u, p[2]]).collect();
        b.m.poly(&ring, SHADE);
    }
    // The dozen tracks: tracks 5 and 6 break round the turntable; the outer ones end at buffer stops.
    let (tx, tu) = TURNTABLE;
    for i in 0..12 {
        let x_of = move |u: f64| rail_x(i, u);
        let end = if (3..=8).contains(&i) { u1 } else { BUFFERS };
        if i == 5 || i == 6 {
            track(b, &x_of, u0, tu - TURNTABLE_R - 2.0, (0.0, 0.0));
            track(b, &x_of, tu + TURNTABLE_R + 2.0, end, (0.0, 0.0));
        } else {
            track(b, &x_of, u0, end, (0.0, 0.0));
        }
        if end == BUFFERS {
            let x = rail_x(i, BUFFERS);
            b.m.frustum([x - 7.0, BUFFERS, 0.0], [x + 7.0, BUFFERS + 4.0, 0.0], 0.5, 5.0, STEEL);
            for k in 0..3 {
                let sx = x - 6.0 + k as f64 * 5.0;
                b.decal(sx, BUFFERS + 0.6, sx + 2.4, BUFFERS + 3.4, 5.1, PAINT);
            }
        }
    }
    // Boxcars and flatcars on the sidings; tracks 2, 4, 7 and 9 stay clear for the RAIL CARs, 5 and 6 run
    // into the turntable and the shed.
    for i in [0usize, 1, 3, 8, 10, 11] {
        let x = rail_x(i, 4000.0);
        let mut u = 3680.0 + b.range(0.0, 40.0);
        while u < BUFFERS - 44.0 {
            let r = b.next() % 10;
            let hopper = i == 8 && u > 4540.0 && u < 4660.0;
            if r < 5 && !hopper {
                let l = 38.0;
                boxcar(b, x, u, l);
                u += l + 3.0;
            } else if r < 7 && !hopper {
                let l = 34.0;
                flatcar(b, x, u, l);
                u += l + 3.0;
            } else {
                u += b.range(20.0, 60.0);
            }
        }
    }
    // Water towers in the gaps between the tracks.
    for &(x, u) in &WATER_TOWERS {
        water_tower(b, x, u);
    }
    // The turntable pit: a ring wall down to a shaded floor, the circular rail, the bridge across it.
    let ring = |r: f64, z: f64| -> Vec<V3> { Mesh::ring(24, r, z, 0.0).iter().map(|p| [p[0] + tx, p[1] + tu, p[2]]).collect() };
    b.m.loft(&ring(TURNTABLE_R + 2.0, 0.4), &ring(TURNTABLE_R, 0.4), ROCK);
    b.m.loft(&ring(TURNTABLE_R, 0.0), &ring(TURNTABLE_R, -4.0), ROCK);
    b.m.poly(&ring(TURNTABLE_R, -4.0), SHADE);
    b.m.loft(&ring(TURNTABLE_R - 2.0, -3.8), &ring(TURNTABLE_R - 3.0, -3.8), STEEL);
    let a: f64 = 0.45;
    let (dx, du) = (a.sin() * (TURNTABLE_R - 1.0), a.cos() * (TURNTABLE_R - 1.0));
    b.m.beam([tx - dx, tu - du, -1.0], [tx + dx, tu + du, -1.0], 8.0, 2.0, STEEL);
    // The engine shed over tracks 5 and 6: a corrugated gable roof, the ridge along the tracks.
    let (s0, s1) = SHED;
    let (w, eaves, ridge) = (34.0, 14.0, 20.0);
    b.m.cuboid([tx - w, s0, 0.0], [tx + w, s1, eaves], CAR);
    let mut u = s0 - 1.0;
    while u < s1 + 1.0 {
        let (ua, um, uc) = (u, u + 1.0, u + 2.0);
        let lift = 0.7;
        for s in [-1.0, 1.0] {
            let (e, r) = (tx + s * (w + 1.5), tx);
            b.m.poly(&[[e, ua, eaves], [r, ua, ridge], [r, um, ridge + lift], [e, um, eaves + lift]], CAR);
            b.m.poly(&[[e, um, eaves + lift], [r, um, ridge + lift], [r, uc, ridge], [e, uc, eaves]], CAR);
        }
        u += 2.0;
    }
    // Ore heaps, the Lattice creeping out from their feet.
    for (i, &(x, u)) in ORE_HEAPS.iter().enumerate() {
        b.cone(x, u, 0.0, 10.0, 9.0, ORE);
        for c in 0..4 {
            let dir = c as f64 * 1.57 + 0.4 + 0.5 * (hash(i as i64, c, 99) - 0.5);
            b.crack(x + dir.sin() * 9.0, u + dir.cos() * 9.0, dir, 3, 0.4);
        }
    }
    // The conveyor feeding a hopper car on track 8 from the heap beside it: ore lumps riding up it.
    let (hx, hu) = (rail_x(8, 4600.0), 4592.0);
    b.m.cuboid([hx - 7.0, hu, 2.5], [hx + 7.0, hu + 32.0, 4.0], CAR);
    for (lo, hi) in [([hx - 7.0, hu, 4.0], [hx + 7.0, hu + 1.5, 12.0]), ([hx - 7.0, hu + 30.5, 4.0], [hx + 7.0, hu + 32.0, 12.0]), ([hx - 7.0, hu, 4.0], [hx - 5.5, hu + 32.0, 12.0]), ([hx + 5.5, hu, 4.0], [hx + 7.0, hu + 32.0, 12.0])] {
        b.m.cuboid(lo, hi, CAR);
    }
    b.decal(hx - 5.5, hu + 1.5, hx + 5.5, hu + 30.5, 9.0, SHADE);
    b.shards(hx, hu + 10.0, 9.0);
    let (p0, p1) = ([ORE_HEAPS[6].0 + 12.0, ORE_HEAPS[6].1 - 6.0, 1.0], [hx + 3.0, hu + 16.0, 14.0]);
    b.m.beam(p0, p1, 4.0, 1.0, STEEL);
    let mid = [(p0[0] + p1[0]) / 2.0, (p0[1] + p1[1]) / 2.0, 0.0];
    b.m.tube(mid, [mid[0], mid[1], (p0[2] + p1[2]) / 2.0 - 0.5], 0.8, 4, STEEL, STEEL);
    for k in 0..6 {
        let f = (k as f64 / 6.0 + 2.0 * b.t).rem_euclid(1.0);
        let p = [p0[0] + (p1[0] - p0[0]) * f, p0[1] + (p1[1] - p0[1]) * f, p0[2] + (p1[2] - p0[2]) * f + 0.5];
        b.m.cuboid([p[0] - 1.0, p[1] - 1.0, p[2]], [p[0] + 1.0, p[1] + 1.0, p[2] + 1.5], CRYSTAL);
    }
    // The crane rails (the TRESTLE TITAN's) on concrete beams at the yard's end, ore spilled across the
    // sleepers between them and the Lattice running through it.
    for x in CRANE_RAILS {
        b.m.cuboid([x - 4.0, CONVERGE.0 + 60.0, 0.0], [x + 4.0, u1, 3.0], ROCK);
        b.m.cuboid([x - 1.0, CONVERGE.0 + 60.0, 3.0], [x + 1.0, u1, 4.0], STEEL);
    }
    for _ in 0..26 {
        let (x, u) = (b.range(100.0, 220.0), b.range(4880.0, 5180.0));
        b.shards(x, u, 2.0);
        let dir = b.range(0.0, TAU);
        b.crack(x, u, dir, 4, 2.6);
    }
}

/// The whole stage at `t` (0..1) through its animation loop.
pub fn level_at(t: f64) -> Mesh {
    let mut b = Builder { m: Mesh::new(materials()), rng: SEED, t };
    desert(&mut b);
    canyon(&mut b);
    trestle(&mut b);
    // The line from the stage's start to the yard: one track in the desert and over the trestle, two in
    // the canyon (they merge where `pair` is 0).
    for side in 0..2 {
        let x_of = move |u: f64| rail_x(side, u);
        let (u0, u1) = if side == 0 { (-24.0, RIVER.1) } else { (1700.0, 3180.0) };
        track(&mut b, &x_of, u0, u1, (RIVER.0 - 2.0, RIVER.1 + 2.0));
    }
    yard(&mut b);
    b.m
}

/// Top surface (height, material) under (`x`, `u`), as `s1::surface`; materials as `SURFACE_*`.
pub fn surface(mesh: &Mesh, x: f64, u: f64) -> Option<(f64, u8)> {
    crate::s1::surface(mesh, x, u)
}
pub const SURFACE_SAND: u8 = SAND;
pub const SURFACE_ROCK: u8 = ROCK;
pub const SURFACE_BALLAST: u8 = BALLAST;
pub const SURFACE_STEEL: u8 = STEEL;
pub const SURFACE_MUD: u8 = MUD;
pub const SURFACE_CAR: u8 = CAR;
pub const SURFACE_TIMBER: u8 = TIMBER;

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

/// The S3 panorama at `t` through the loop (one stage period plus its first 200 lines again).
pub fn panorama_at(t: f64) -> Panorama {
    let period = demo::PANO_PERIOD as usize;
    let mesh = level_at(t);
    let shadows = ShadowMap::new(&mesh, &SUN);
    let mut codes = render_lines(&mesh, &shadows, 0, period);
    let wrap = codes[..160 * 200].to_vec();
    codes.extend_from_slice(&wrap);
    Panorama { rows: ((period + 200) / 8) as u16, codes, palette: PALETTE }
}

/// Every frame of the S3 animation loop, rendered in parallel.
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
        assert!(s[3] <= 0.03, "hot share {:.3}: crystal only in the ore and the cracks", s[3]);
        assert!(s[1] >= 0.25, "dark share {:.3}: the sand reads orange in the sun", s[1]);
        assert_eq!(p.codes[..160 * 200], p.codes[p.codes.len() - 160 * 200..]);
    }

    #[test]
    fn the_animation_loops_without_a_seam() {
        // Dunes, the highway's tumbleweeds; the canyon's pours and veins; the gantry; the trestle; the
        // conveyor in the yard.
        for (lo, hi) in [(200, 400), (1880, 2080), (2960, 3160), (3200, 3400), (4500, 4700)] {
            assert!(lines(0.0, lo, hi) == lines(1.0, lo, hi), "lines {lo}..{hi} jump at the loop");
            assert!(lines(0.0, lo, hi) != lines(0.5, lo, hi), "lines {lo}..{hi} do not move");
        }
    }

    #[test]
    fn the_low_sun_casts_long_shadows_lower_right() {
        let mut m = Mesh::new(materials());
        m.frustum([0.0, 0.0, 0.0], [200.0, 200.0, 0.0], 0.0, 1.0, SAND);
        m.cuboid([50.0, 100.0, 1.0], [52.0, 102.0, 21.0], TIMBER);
        let sm = ShadowMap::new(&m, &SUN);
        // A 20-line pole's shadow runs ~31 px right and ~13 lines down: long, to the lower right.
        assert!(sm.shadowed([51.0 + 15.0, 101.0 - 6.5, 1.0]), "halfway along the shadow");
        assert!(sm.shadowed([51.0 + 27.0, 101.0 - 11.5, 1.0]), "near its tip: a long shadow");
        assert!(!sm.shadowed([51.0 - 15.0, 101.0 + 6.5, 1.0]), "toward the sun");
    }

    #[test]
    fn the_rails_run_where_the_units_will_ride() {
        // One line in the desert, two in the canyon, the left one straight for the SIDEWINDER; a dozen in
        // the yard, the middle six closing up under the crane.
        assert_eq!(rail_x(0, 500.0), rail_x(1, 500.0));
        assert!((rail_x(1, 2600.0) - rail_x(0, 2600.0) - PAIR).abs() < 1e-9);
        for u in [SIDEWINDER_RUN.0, 2600.0, SIDEWINDER_RUN.1] {
            assert!((rail_x(0, u) - SIDEWINDER_X).abs() < 0.5, "line {u}: {}", rail_x(0, u));
        }
        for u in (1700..3180).step_by(10).map(|u| u as f64) {
            assert!(rail_x(0, u) - 9.0 > xl(u) && rail_x(1, u) + 9.0 < xr(u), "line {u}: the track leaves the floor");
        }
        for i in 0..12 {
            assert_eq!(rail_x(i, 4200.0), YARD_X[i]);
        }
        assert!((rail_x(3, 5100.0) - CONVERGED[0]).abs() < 1e-9);
    }
}
