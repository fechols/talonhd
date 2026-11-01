//! Solid-polygon renderer (host only): a flat-shaded triangle mesh, seen
//! straight from above through an orthographic camera, becomes a sprite
//! frame in the four global multicolor codes.
//!
//! The look (TALON HD art direction):
//! - **One key light** for the whole game, upper-left and toward the viewer.
//! - **Faceted:** one intensity per face (Lambert on the face normal).
//! - **Ramps:** each material shades through an ordered list of codes, dark →
//!   light. Intensity is quantized in half steps: a whole step is a solid
//!   code, a half step (the middle 40 % between two codes) a 50 % checkerboard. The checker is
//!   anchored in sprite space, so it moves with the ship and never crawls.
//! - **Emissive** materials (cores, engines, shots) ignore the light.
//!
//! Units: one model unit is one hires pixel (half a multicolor pixel across,
//! one line down). Model axes: x right, y forward (up the screen), z toward
//! the viewer. Each pixel takes 4×4 samples; a pixel is opaque when at least
//! half its samples hit the mesh, and takes the face covering most of them.

use crate::encode::SpriteImage;

pub type V3 = [f64; 3];

#[derive(Clone, Debug)]
pub struct Material {
    /// Multicolor codes, darkest first.
    pub ramp: Vec<u8>,
    /// Scales the light (0..1].
    pub albedo: f64,
    /// Lowest half-step level a lit face may fall to (keeps silhouettes off `void`).
    pub floor: u8,
    /// Ignores the light: always the ramp's last code.
    pub emissive: bool,
}

impl Material {
    pub fn lit(ramp: &[u8], albedo: f64, floor: u8) -> Material {
        Material { ramp: ramp.to_vec(), albedo, floor, emissive: false }
    }
    pub fn glow(code: u8) -> Material {
        Material { ramp: vec![code], albedo: 1.0, floor: 0, emissive: true }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub verts: Vec<V3>,
    pub tris: Vec<[u32; 3]>,
    /// Material index per triangle.
    pub mats: Vec<u8>,
    pub materials: Vec<Material>,
}

impl Mesh {
    pub fn new(materials: Vec<Material>) -> Mesh {
        Mesh { materials, ..Mesh::default() }
    }
    pub fn vert(&mut self, p: V3) -> u32 {
        self.verts.push(p);
        self.verts.len() as u32 - 1
    }
    pub fn tri(&mut self, a: V3, b: V3, c: V3, mat: u8) {
        let (a, b, c) = (self.vert(a), self.vert(b), self.vert(c));
        self.tris.push([a, b, c]);
        self.mats.push(mat);
    }
    /// A planar or near-planar polygon as a triangle fan.
    pub fn poly(&mut self, pts: &[V3], mat: u8) {
        for i in 1..pts.len() - 1 {
            self.tri(pts[0], pts[i], pts[i + 1], mat);
        }
    }
    /// The polygon and its mirror image across x = 0.
    pub fn sym(&mut self, pts: &[V3], mat: u8) {
        self.poly(pts, mat);
        let m: Vec<V3> = pts.iter().map(|p| [-p[0], p[1], p[2]]).collect();
        self.poly(&m, mat);
    }
    /// An axis-aligned box from `lo` to `hi` (all six faces).
    pub fn cuboid(&mut self, lo: V3, hi: V3, mat: u8) {
        let p = |x: usize, y: usize, z: usize| [[lo[0], hi[0]][x], [lo[1], hi[1]][y], [lo[2], hi[2]][z]];
        self.poly(&[p(0, 0, 1), p(1, 0, 1), p(1, 1, 1), p(0, 1, 1)], mat);
        self.poly(&[p(0, 0, 0), p(1, 0, 0), p(1, 1, 0), p(0, 1, 0)], mat);
        self.poly(&[p(0, 0, 0), p(1, 0, 0), p(1, 0, 1), p(0, 0, 1)], mat);
        self.poly(&[p(0, 1, 0), p(1, 1, 0), p(1, 1, 1), p(0, 1, 1)], mat);
        self.poly(&[p(0, 0, 0), p(0, 1, 0), p(0, 1, 1), p(0, 0, 1)], mat);
        self.poly(&[p(1, 0, 0), p(1, 1, 0), p(1, 1, 1), p(1, 0, 1)], mat);
    }
    /// A box with a pyramid-like top: base `lo..hi` rising to an inset top at `top_z`.
    pub fn frustum(&mut self, lo: V3, hi: V3, inset: f64, top_z: f64, mat: u8) {
        let b = [[lo[0], lo[1], lo[2]], [hi[0], lo[1], lo[2]], [hi[0], hi[1], lo[2]], [lo[0], hi[1], lo[2]]];
        let t = [
            [lo[0] + inset, lo[1] + inset, top_z],
            [hi[0] - inset, lo[1] + inset, top_z],
            [hi[0] - inset, hi[1] - inset, top_z],
            [lo[0] + inset, hi[1] - inset, top_z],
        ];
        self.poly(&t, mat);
        for i in 0..4 {
            let j = (i + 1) % 4;
            self.poly(&[b[i], b[j], t[j], t[i]], mat);
        }
    }
    /// Append `other`, offset by `at`, with its materials remapped by `mat_base`.
    pub fn add(&mut self, other: &Mesh, at: V3, mat_base: u8) {
        let base = self.verts.len() as u32;
        self.verts.extend(other.verts.iter().map(|v| [v[0] + at[0], v[1] + at[1], v[2] + at[2]]));
        self.tris.extend(other.tris.iter().map(|t| t.map(|i| i + base)));
        self.mats.extend(other.mats.iter().map(|m| m + mat_base));
    }
}

/// Orientation in radians: `roll` about the forward axis (banking), then
/// `pitch` about x (nose toward the viewer), then `yaw` about the view axis.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Orient {
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
}

impl Orient {
    pub fn apply(&self, p: V3) -> V3 {
        let (sr, cr) = self.roll.sin_cos();
        let (sp, cp) = self.pitch.sin_cos();
        let (sy, cy) = self.yaw.sin_cos();
        // Roll about y.
        let (x, y, z) = (p[0] * cr + p[2] * sr, p[1], -p[0] * sr + p[2] * cr);
        // Pitch about x.
        let (x, y, z) = (x, y * cp - z * sp, y * sp + z * cp);
        // Yaw about z (counter-clockwise seen from above).
        [x * cy - y * sy, x * sy + y * cy, z]
    }
}

impl From<(f64, f64, f64)> for Orient {
    fn from((yaw, pitch, roll): (f64, f64, f64)) -> Orient {
        Orient { yaw, pitch, roll }
    }
}

/// The game's one key light: upper-left, toward the viewer.
pub const LIGHT: V3 = [-0.45, 0.55, 0.70];
const AMBIENT: f64 = 0.18;

fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn normalize(a: V3) -> V3 {
    let l = dot(a, a).sqrt();
    [a[0] / l, a[1] / l, a[2] / l]
}

/// Shade of one face: (code a, code b); a == b is solid, else a checkerboard.
fn shade(m: &Material, normal: V3) -> (u8, u8) {
    let n = m.ramp.len();
    if m.emissive || n == 1 {
        let c = *m.ramp.last().unwrap();
        return (c, c);
    }
    // Normalized so a face pointing straight up (at the viewer) is fully lit: flat tops read as
    // solid light and only faces turned away from the key light step down the ramp.
    let l = normalize(LIGHT);
    let lambert = (dot(normal, l) / l[2]).clamp(0.0, 1.0);
    let lit = (AMBIENT + (1.0 - AMBIENT) * lambert) * m.albedo;
    // Position along the ramp; the checker covers only the middle 40 % of each step, so most faces
    // are solid and a checker always means a real mid-tone.
    let t = lit.clamp(0.0, 1.0) * (n - 1) as f64;
    let (i, f) = (t.floor() as usize, t.fract());
    let q = if i >= n - 1 { 2 * (n - 1) } else if f < 0.3 { 2 * i } else if f > 0.7 { 2 * i + 2 } else { 2 * i + 1 };
    let q = q.clamp(m.floor as usize, 2 * (n - 1));
    (m.ramp[q / 2], m.ramp[q.div_ceil(2)])
}

/// Where the model sits on the canvas: the model origin lands at hires
/// (`cx`, `cy`) of a `w`×`h` multicolor-pixel canvas; `scale` multiplies
/// model units.
#[derive(Clone, Copy, Debug)]
pub struct View {
    pub w: u16,
    pub h: u16,
    pub cx: f64,
    pub cy: f64,
    pub scale: f64,
}

impl View {
    /// Origin at the canvas centre.
    pub fn centered(w: u16, h: u16, scale: f64) -> View {
        View { w, h, cx: w as f64, cy: h as f64 / 2.0, scale }
    }
}

const SS: usize = 4;

/// Render `mesh` at orientation `o`.
pub fn render(mesh: &Mesh, o: Orient, view: View) -> SpriteImage {
    let (w, h) = (view.w as usize, view.h as usize);
    let (sw, sh) = (w * SS, h * SS);
    let mut depth = vec![f64::NEG_INFINITY; sw * sh];
    let mut face = vec![u32::MAX; sw * sh];
    // Sample (i, j) sits at hires x = (i + 0.5) / 2, line y = (j + 0.5) / 4.
    let pts: Vec<V3> = mesh
        .verts
        .iter()
        .map(|&v| {
            let r = o.apply([v[0] * view.scale, v[1] * view.scale, v[2] * view.scale]);
            // Screen: x right in samples, y down in samples, z toward the viewer.
            [(view.cx + r[0]) * 2.0, (view.cy - r[1]) * 4.0, r[2]]
        })
        .collect();
    let mut shades = Vec::with_capacity(mesh.tris.len());
    for (ti, t) in mesh.tris.iter().enumerate() {
        let (a, b, c) = (pts[t[0] as usize], pts[t[1] as usize], pts[t[2] as usize]);
        // Normal in model-view space (undo the sample scaling).
        let ua = [a[0] / 2.0, -a[1] / 4.0, a[2]];
        let ub = [b[0] / 2.0, -b[1] / 4.0, b[2]];
        let uc = [c[0] / 2.0, -c[1] / 4.0, c[2]];
        let n = cross(sub(ub, ua), sub(uc, ua));
        let len = dot(n, n).sqrt();
        let normal = if len > 1e-9 {
            let n = [n[0] / len, n[1] / len, n[2] / len];
            // Winding-agnostic: the visible side faces the viewer.
            if n[2] < 0.0 { [-n[0], -n[1], -n[2]] } else { n }
        } else {
            [0.0, 0.0, 1.0]
        };
        shades.push(shade(&mesh.materials[mesh.mats[ti] as usize], normal));

        let area = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
        if area.abs() < 1e-9 {
            continue;
        }
        let x0 = (a[0].min(b[0]).min(c[0]).floor().max(0.0)) as usize;
        let x1 = (a[0].max(b[0]).max(c[0]).ceil().min(sw as f64)) as usize;
        let y0 = (a[1].min(b[1]).min(c[1]).floor().max(0.0)) as usize;
        let y1 = (a[1].max(b[1]).max(c[1]).ceil().min(sh as f64)) as usize;
        for sy in y0..y1 {
            let py = sy as f64 + 0.5;
            for sx in x0..x1 {
                let px = sx as f64 + 0.5;
                let w0 = ((b[0] - px) * (c[1] - py) - (b[1] - py) * (c[0] - px)) / area;
                let w1 = ((c[0] - px) * (a[1] - py) - (c[1] - py) * (a[0] - px)) / area;
                let w2 = 1.0 - w0 - w1;
                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }
                let z = w0 * a[2] + w1 * b[2] + w2 * c[2];
                let i = sy * sw + sx;
                if z > depth[i] {
                    depth[i] = z;
                    face[i] = ti as u32;
                }
            }
        }
    }

    let mut img = SpriteImage::new(view.w, view.h);
    let mut counts: Vec<(u32, u8)> = Vec::with_capacity(SS * SS);
    for y in 0..h {
        for x in 0..w {
            counts.clear();
            let mut covered = 0;
            for j in 0..SS {
                for i in 0..SS {
                    let f = face[(y * SS + j) * sw + x * SS + i];
                    if f == u32::MAX {
                        continue;
                    }
                    covered += 1;
                    match counts.iter_mut().find(|c| c.0 == f) {
                        Some(c) => c.1 += 1,
                        None => counts.push((f, 1)),
                    }
                }
            }
            if covered * 2 < SS * SS {
                continue;
            }
            let best = counts.iter().max_by_key(|c| (c.1, std::cmp::Reverse(c.0))).unwrap().0;
            let (ca, cb) = shades[best as usize];
            img.set(x as u16, y as u16, Some(if (x + y) & 1 == 0 { ca } else { cb }));
        }
    }
    img
}

/// FNV-1a over a frame's pixels (golden tests).
pub fn hash(img: &SpriteImage) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for p in &img.pixels {
        h ^= p.map_or(0xFF, |c| c as u64);
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h
}
