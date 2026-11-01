//! The S1 BREAKWATER panorama (host only), in the S1 palette: 26 screens bottom to top, as the base bible
//! lays the stage out (`docs/story/04-levels/s1-breakwater.md`): open sea from the carrier's wake, the
//! SEA WALL, the docks with the centre channel, the airfield with its two runways up to the hangar
//! apron where the GANTRY QUEEN waits.
//!
//! Art direction (`docs/story/hd/01-art-direction.md` §9, the background stays down): the ground is `void`
//! and `dark` only; flat concrete is the void/dark checker, so solid `dark` and every `light` highlight
//! belong to the play layer. `light` only for surf, wake and runway paint (≤ 10 %), `hot` only where the
//! Lattice has grown into the ground. Structures are rendered from meshes with the sprites' key light.

use crate::bake::Panorama;
use crate::demo;
use crate::raster3d::{self, Material, Mesh, Orient, V3, View};
use crate::render::Palette;

const VOID: u8 = 0;
const DARK: u8 = 1;
const LIGHT: u8 = 2;
const HOT: u8 = 3;

pub const PALETTE: Palette = Palette::new(6, 12, 1, 3);
const W: i32 = 160;
/// Screens per section, from the stage start (bottom) up: sea 0-9, docks 9-17, airfield 17-26.
const SEA_END: i32 = 9 * 200;
const DOCKS_END: i32 = 17 * 200;

struct Canvas {
    period: i32,
    codes: Vec<u8>,
    rng: u32,
}

impl Canvas {
    /// `u` counts lines up from the stage start (the bottom of the panorama).
    fn put(&mut self, x: i32, u: i32, c: u8) {
        if (0..W).contains(&x) && (0..self.period).contains(&u) {
            let y = self.period - 1 - u;
            self.codes[(y * W + x) as usize] = c;
        }
    }
    fn get(&self, x: i32, u: i32) -> u8 {
        if (0..W).contains(&x) && (0..self.period).contains(&u) {
            self.codes[((self.period - 1 - u) * W + x) as usize]
        } else {
            VOID
        }
    }
    fn next(&mut self) -> u32 {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        self.rng
    }
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() % (hi - lo).max(1) as u32) as i32
    }
    /// Flat concrete: the void/dark checker, anchored to the panorama.
    fn concrete(&mut self, x0: i32, x1: i32, u0: i32, u1: i32) {
        for u in u0..u1 {
            for x in x0..x1 {
                self.put(x, u, if (x + u) & 1 == 0 { DARK } else { VOID });
            }
        }
    }
    fn rect(&mut self, x0: i32, x1: i32, u0: i32, u1: i32, c: u8) {
        for u in u0..u1 {
            for x in x0..x1 {
                self.put(x, u, c);
            }
        }
    }
    /// A mesh rendered top-down with the sprites' light, centred on (`x`, `u`); returns its footprint.
    fn stamp(&mut self, mesh: &Mesh, yaw: f64, x: i32, u: i32, w: u16, h: u16) -> Vec<(i32, i32)> {
        let img = raster3d::render(mesh, Orient { yaw, ..Orient::default() }, View::centered(w, h, 1.0));
        let mut out = Vec::new();
        for j in 0..img.h as i32 {
            for i in 0..img.w as i32 {
                if let Some(c) = img.get(i as u16, j as u16) {
                    let (px, pu) = (x + i - img.w as i32 / 2, u + img.h as i32 / 2 - j);
                    self.put(px, pu, c);
                    out.push((px, pu));
                }
            }
        }
        out
    }
    /// A crack the Lattice has grown into: a jagged random walk of `hot`, mostly upward.
    fn crystal_vein(&mut self, mut x: i32, mut u: i32, len: i32) {
        for _ in 0..len {
            self.put(x, u, HOT);
            match self.next() % 6 {
                0 => x -= 1,
                1 => x += 1,
                2 => u -= 1,
                _ => u += 1,
            }
        }
    }
}

/// Ground structures: one step down (void/dark only), the same key light as the sprites.
fn ground() -> Material {
    Material::lit(&[VOID, DARK], 0.6, 0)
}
fn ground_dim() -> Material {
    Material::lit(&[VOID, DARK], 0.45, 0)
}

/// A rocky islet: irregular stepped rings to a flat cap.
fn islet(c: &mut Canvas, r: f64) -> Mesh {
    let mut m = Mesh::new(vec![ground(), ground_dim()]);
    let n = 9;
    let jit: Vec<f64> = (0..n).map(|_| 0.75 + (c.next() % 50) as f64 / 100.0).collect();
    let ring = |s: f64, z: f64| -> Vec<V3> {
        (0..n)
            .map(|k| {
                let a = k as f64 * 2.0 * std::f64::consts::PI / n as f64;
                [a.cos() * r * s * jit[k], a.sin() * r * s * jit[(k + 3) % n], z]
            })
            .collect()
    };
    let (a, b, t) = (ring(1.0, 0.0), ring(0.7, 7.0), ring(0.35, 11.0));
    m.loft(&a, &b, 1);
    m.loft(&b, &t, 0);
    m.poly(&t, 0);
    m
}

fn sea(c: &mut Canvas) {
    // Long swells: faint dashed dark streaks running across; whitecaps: specks of light.
    for _ in 0..900 {
        let (x, u, len) = (c.range(-10, W), c.range(0, SEA_END), c.range(4, 18));
        for k in (0..len).step_by(2) {
            c.put(x + k, u, DARK);
        }
    }
    for _ in 0..260 {
        let (x, u) = (c.range(0, W), c.range(0, SEA_END));
        c.put(x, u, LIGHT);
        if c.next() % 3 == 0 {
            c.put(x + 1, u, LIGHT);
        }
    }
    // The MERIDIAN's wake: a broad white V fading behind the start position.
    for u in 0..260 {
        let spread = (260 - u) * 60 / 260;
        let fade = 2 + u / 40;
        for side in [-1, 1] {
            for d in -3..=3 {
                if c.next() % fade as u32 == 0 {
                    c.put(80 + side * spread + d, u, if d.abs() < 2 { LIGHT } else { DARK });
                }
            }
        }
    }
    // Islets ringed by sand (dark) and surf (light specks).
    for &(x, u, r) in &[(22, 520, 26.0), (138, 900, 34.0), (30, 1300, 20.0), (128, 1560, 24.0), (60, 1180, 12.0)] {
        let mesh = islet(c, r);
        let foot = c.stamp(&mesh, 0.0, x, u, 48, 80);
        let set: std::collections::HashSet<(i32, i32)> = foot.iter().copied().collect();
        for &(px, pu) in &foot {
            for (dx, du) in [(-1, 0), (1, 0), (0, -1), (0, 1), (-2, 0), (2, 0), (0, -2), (0, 2), (-3, 1), (3, -1)] {
                let p = (px + dx, pu + du);
                if !set.contains(&p) && c.get(p.0, p.1) != DARK {
                    let near = dx.abs() + du.abs() <= 1;
                    let v = if near { DARK } else if c.next() % 3 == 0 { LIGHT } else { VOID };
                    c.put(p.0, p.1, v);
                }
            }
        }
    }
    // A marker buoy: a light dot with a dark ring.
    for &(x, u) in &[(104, 700), (52, 1650)] {
        c.rect(x - 1, x + 2, u - 1, u + 2, DARK);
        c.put(x, u, LIGHT);
    }
}

/// The SEA WALL: a breakwater of interlocking blocks across the whole screen, surf bursting over the sea
/// side, a lighthouse stub at the right end with a crystal lamp, calm harbour water beyond.
fn sea_wall(c: &mut Canvas) {
    let u0 = SEA_END;
    let mut m = Mesh::new(vec![ground(), ground_dim()]);
    for k in 0..22 {
        let x = -150.0 + k as f64 * 14.0;
        let dy = if k % 2 == 0 { 2.0 } else { -3.0 };
        m.frustum([x - 8.0, -10.0 + dy, 0.0], [x + 8.0, 10.0 + dy, 0.0], 3.0, 10.0 + (k % 3) as f64 * 2.0, (k % 2) as u8);
    }
    m.frustum([-160.0, -6.0, 0.0], [160.0, 6.0, 0.0], 1.0, 8.0, 1);
    c.stamp(&m, 0.0, 75, u0 + 14, 160, 40);
    // Lighthouse stub and its crystal lamp.
    let mut l = Mesh::new(vec![ground()]);
    l.tube([0.0, 0.0, 0.0], [0.0, 0.0, 22.0], 9.0, 10, 0, 0);
    c.stamp(&l, 0.0, 148, u0 + 14, 16, 24);
    for (dx, du) in [(0, 0), (-1, 0), (1, 0), (0, 1), (0, -1)] {
        c.put(148 + dx, u0 + 14 + du, HOT);
    }
    // Surf bursting white on the sea side, in patches.
    for x in 0..W {
        let burst = (x / 12) % 3 != 1;
        for du in 0..6 {
            let p = if burst { 2 } else { 5 };
            if c.next() % (p + du as u32) == 0 {
                c.put(x, u0 + 2 - du, LIGHT);
            }
        }
    }
}

/// Container stacks, fuel tanks and cranes along a quay.
fn container_block(c: &mut Canvas, x: i32, u: i32) {
    let mut m = Mesh::new(vec![ground(), ground_dim()]);
    for row in 0..3 {
        for col in 0..2 {
            let (cx, cy) = (col as f64 * 18.0 - 9.0, row as f64 * 8.0 - 8.0);
            let h = 5.0 + ((row + col) % 2) as f64 * 5.0;
            m.frustum([cx - 8.0, cy - 3.0, 0.0], [cx + 8.0, cy + 3.0, 0.0], 0.8, h, ((row + col) % 2) as u8);
        }
    }
    c.stamp(&m, 0.0, x, u, 24, 32);
}

fn fuel_tank(c: &mut Canvas, x: i32, u: i32, r: f64) {
    let mut m = Mesh::new(vec![ground(), ground_dim()]);
    m.tube([0.0, 0.0, 0.0], [0.0, 0.0, 10.0], r, 12, 1, 0);
    m.frustum([-3.0, -3.0, 10.0], [3.0, 3.0, 10.0], 1.0, 12.0, 1);
    c.stamp(&m, 0.0, x, u, (r as u16 + 4).max(8), (r as u16 * 2 + 6).max(16));
}

/// A gantry crane on the quay edge, its boom swung over the water; crystal veins climb its legs.
fn crane(c: &mut Canvas, x: i32, u: i32, dir: i32) {
    let mut m = Mesh::new(vec![ground(), ground_dim()]);
    for (lx, ly) in [(-8.0, -8.0), (8.0, -8.0), (-8.0, 8.0), (8.0, 8.0)] {
        m.cuboid([lx - 2.0, ly - 2.0, 0.0], [lx + 2.0, ly + 2.0, 26.0], 1);
    }
    m.beam([0.0, -10.0, 28.0], [0.0, 10.0, 28.0], 18.0, 4.0, 0);
    m.beam([0.0, 0.0, 32.0], [dir as f64 * 60.0, 0.0, 30.0], 5.0, 4.0, 0);
    c.stamp(&m, 0.0, x + dir * 15, u, 48, 32);
    for (lx, lu) in [(-4, -8), (4, 8)] {
        let len = c.range(6, 14);
        c.crystal_vein(x + lx, u + lu, len);
    }
}

/// The docks: a centre channel (the KEELBREAKER's berth and route), quays with basins cut in either
/// side, piers on piles, containers, fuel tanks and cranes; crystal veins splitting the slabs.
fn docks(c: &mut Canvas) {
    let (u0, u1) = (SEA_END + 30, DOCKS_END);
    // Quays left and right of the channel (x 58..102), basins cut into them every ~170 lines.
    c.concrete(0, 58, u0, u1);
    c.concrete(102, W, u0, u1);
    let mut u = u0 + 60;
    let mut k = 0;
    while u + 90 < u1 {
        let left = k % 2 == 0;
        let (x0, x1) = if left { (0, 46) } else { (114, W) };
        let len = c.range(70, 100);
        c.rect(x0, x1, u, u + len, VOID);
        // A timber pier on piles down the basin.
        let px = if left { 20 } else { 138 };
        c.rect(px, px + 3, u, u + len, DARK);
        for pu in (u..u + len).step_by(6) {
            c.put(px - 1, pu, DARK);
            c.put(px + 3, pu, DARK);
        }
        // White bollards along the basin edges.
        for bu in (u + 4..u + len).step_by(14) {
            c.put(if left { x1 } else { x0 - 1 }, bu, LIGHT);
        }
        // A crane at the basin mouth, boom over the water.
        crane(c, if left { 52 } else { 108 }, u + len / 2, if left { -1 } else { 1 });
        u += len + c.range(70, 110);
        k += 1;
    }
    // Quay furniture on the slabs that are left.
    let mut u = u0 + 20;
    while u < u1 - 20 {
        for x in [10, 32, 128, 148] {
            if c.get(x, u) == VOID && c.get(x + 1, u) == VOID {
                continue; // a basin
            }
            match c.next() % 5 {
                0 | 1 => container_block(c, x, u),
                2 => fuel_tank(c, x, u, 9.0),
                _ => {}
            }
        }
        u += 34;
    }
    // Light quay edges along the channel (hazard paint) and crystal veins in the slabs.
    for u in (u0..u1).step_by(3) {
        c.put(57, u, LIGHT);
        c.put(102, u, LIGHT);
    }
    for _ in 0..26 {
        let (x, u) = if c.next() % 2 == 0 { (c.range(0, 56), c.range(u0, u1)) } else { (c.range(104, W), c.range(u0, u1)) };
        let len = c.range(10, 30);
        c.crystal_vein(x, u, len);
    }
}

/// A 3×5 digit, painted `scale` times larger; `flip` paints it upside down (read from the far end).
fn paint_digit(c: &mut Canvas, d: u8, x: i32, u: i32, scale: i32, flip: bool) {
    const FONT: [[u8; 5]; 10] = [
        [7, 5, 5, 5, 7], [2, 6, 2, 2, 7], [7, 1, 7, 4, 7], [7, 1, 3, 1, 7], [5, 5, 7, 1, 1],
        [7, 4, 7, 1, 7], [7, 4, 7, 5, 7], [7, 1, 1, 2, 2], [7, 5, 7, 5, 7], [7, 5, 7, 1, 7],
    ];
    for row in 0..5 {
        for col in 0..3 {
            if FONT[d as usize][row] >> (2 - col) & 1 == 1 {
                let (r, cc) = if flip { (4 - row as i32, 2 - col) } else { (row as i32, col) };
                c.rect(x + cc * scale, x + (cc + 1) * scale, u - (r + 1) * scale * 2, u - r * scale * 2, LIGHT);
            }
        }
    }
}

/// A curved hangar roof of ribbed metal, lengthwise along y.
fn hangar(c: &mut Canvas, x: i32, u: i32) {
    let mut m = Mesh::new(vec![ground(), ground_dim()]);
    let (w, l, n) = (34.0, 30.0, 8);
    let arc = |k: usize| -> (f64, f64) {
        let a = std::f64::consts::PI * k as f64 / n as f64;
        (-w * a.cos(), 26.0 * a.sin())
    };
    for k in 0..n {
        let (x0, z0) = arc(k);
        let (x1, z1) = arc(k + 1);
        m.poly(&[[x0, -l, z0], [x1, -l, z1], [x1, l, z1], [x0, l, z0]], (k % 2) as u8);
    }
    c.stamp(&m, 0.0, x, u, 40, 66);
}

/// The airfield: two broad runways up the screen with threshold bars, centre dashes and painted
/// numbers, yellow-edged taxiways, hangars, AA pits, a radar blockhouse, and crystal threading the
/// cracks, thicker toward the apron at the top.
fn airfield(c: &mut Canvas) {
    let (u0, u1) = (DOCKS_END, c.period);
    // The airfield seawall: concrete across, the field beyond.
    c.concrete(0, W, u0, u0 + 16);
    for x in (0..W).step_by(2) {
        c.put(x, u0 + 16, LIGHT);
    }
    // Infield: a 25 % dark dither (grass and dirt), so open ground never reads as water and the 50 %
    // concrete checker of runways and taxiways stands out from it.
    for u in u0 + 17..u1 {
        for x in 0..W {
            c.put(x, u, if x % 2 == 0 && u % 2 == 0 { DARK } else { VOID });
        }
    }
    // Runways (x 22..54 and 106..138) with paint.
    for &(x0, x1) in &[(22, 54), (106, 138)] {
        c.concrete(x0, x1, u0 + 40, u1 - 120);
        let mid = (x0 + x1) / 2;
        for u in (u0 + 110..u1 - 190).step_by(24) {
            c.rect(mid - 1, mid + 1, u, u + 10, LIGHT);
        }
        for (end, flip) in [(u0 + 46, false), (u1 - 134, true)] {
            for bx in (x0 + 2..x1 - 2).step_by(4) {
                c.rect(bx, bx + 2, if flip { end - 14 } else { end }, if flip { end } else { end + 14 }, LIGHT);
            }
            // 36 at the near end, 18 painted upside down at the far end.
            let (a, b) = if flip { (8, 1) } else { (3, 6) };
            let nu = if flip { end - 22 } else { end + 38 };
            paint_digit(c, a, mid - 7, nu, 2, flip);
            paint_digit(c, b, mid + 1, nu, 2, flip);
        }
    }
    // Taxiways across, edged in light dashes.
    for &u in &[u0 + 420, u0 + 1100] {
        c.concrete(0, W, u, u + 12);
        for x in (0..W).step_by(4) {
            c.put(x, u, LIGHT);
            c.put(x, u + 11, LIGHT);
        }
    }
    // Hangars between the runways and at the edges; AA pits; the radar blockhouse.
    for &(x, u) in &[(80, u0 + 300), (80, u0 + 700), (80, u0 + 1300), (148, u0 + 900), (11, u0 + 1500)] {
        hangar(c, x, u);
    }
    for &(x, u) in &[(70, u0 + 520), (92, u0 + 980), (66, u0 + 1500), (148, u0 + 600), (10, u0 + 240)] {
        for a in 0..14 {
            let t = a as f64 * std::f64::consts::PI / 7.0;
            c.put(x + (5.0 * t.cos()).round() as i32, u + (9.0 * t.sin()).round() as i32, DARK);
        }
    }
    let mut r = Mesh::new(vec![ground(), ground_dim()]);
    r.frustum([-10.0, -8.0, 0.0], [10.0, 8.0, 0.0], 2.0, 10.0, 1);
    r.tube([0.0, 0.0, 10.0], [0.0, 3.0, 14.0], 7.0, 8, 0, 0);
    c.stamp(&r, 0.0, 12, u0 + 820, 20, 28);
    // The apron: open concrete at the top, crystal everywhere.
    c.concrete(0, W, u1 - 120, u1);
    for i in 0..160 {
        let u = u1 - 10 - (i * i * (u1 - u0 - 50) / (160 * 160));
        let x = c.range(0, W);
        let len = c.range(8, 26);
        c.crystal_vein(x, u, len);
    }
}

/// The whole S1 panorama, one loop of the stage (`demo::PANO_PERIOD` lines) plus the first 200 lines
/// again at the end so every window is contiguous.
pub fn panorama() -> Panorama {
    let period = demo::PANO_PERIOD as i32;
    let mut c = Canvas { period, codes: vec![VOID; (W * period) as usize], rng: 0x5EA_0001 };
    sea(&mut c);
    sea_wall(&mut c);
    docks(&mut c);
    airfield(&mut c);
    let mut codes = c.codes;
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
        // The wrap copy keeps the loop seamless.
        assert_eq!(p.codes[..160 * 200], p.codes[p.codes.len() - 160 * 200..]);
    }
}
