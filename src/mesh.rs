//! TALON HD `.mesh` files (host only), written by `tools/model_prep.py` from Ludo GLBs.
//!
//! ```text
//! TMESH1
//! name <id>
//! canvas <w> <h>
//! orient <key>=<value> ...
//! material <name> <emissive 0|1> <floor> <albedo> <role> [<role> ...]
//! v <x> <y> <z>
//! f <a> <b> <c> <material>
//! ```
//! Roles map to the four global codes: void 00, dark 01, light 10, hot 11.

use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::path::Path;

use crate::encode::SpriteImage;
use crate::raster3d::{self, Material, Mesh, Orient, View};

pub struct ModelFile {
    pub name: String,
    pub canvas: (u16, u16),
    pub orient: BTreeMap<String, f64>,
    pub mesh: Mesh,
}

pub fn role_code(role: &str) -> Result<u8, String> {
    match role {
        "void" => Ok(0),
        "dark" => Ok(1),
        "light" => Ok(2),
        "hot" => Ok(3),
        other => Err(format!("unknown role {other}")),
    }
}

pub fn parse(text: &str) -> Result<ModelFile, String> {
    let mut lines = text.lines();
    if lines.next() != Some("TMESH1") {
        return Err("not a TMESH1 file".into());
    }
    let mut out = ModelFile { name: String::new(), canvas: (0, 0), orient: BTreeMap::new(), mesh: Mesh::default() };
    let num = |s: &str| s.parse::<f64>().map_err(|_| format!("bad number {s}"));
    for (n, line) in lines.enumerate() {
        let w: Vec<&str> = line.split_whitespace().collect();
        let bad = || format!("line {}: {line}", n + 2);
        match w.first().copied() {
            Some("name") => out.name = w.get(1).ok_or_else(bad)?.to_string(),
            Some("canvas") if w.len() == 3 => out.canvas = (num(w[1])? as u16, num(w[2])? as u16),
            Some("orient") => {
                for kv in &w[1..] {
                    let (k, v) = kv.split_once('=').ok_or_else(bad)?;
                    out.orient.insert(k.to_string(), num(v)?);
                }
            }
            Some("material") if w.len() >= 6 => {
                let ramp = w[5..].iter().map(|r| role_code(r)).collect::<Result<Vec<u8>, _>>()?;
                let floor = num(w[3])? as u8;
                let albedo = num(w[4])?;
                out.mesh.materials.push(if w[2] == "1" {
                    Material { ramp, albedo: 1.0, floor: 0, emissive: true }
                } else {
                    Material { ramp, albedo, floor, emissive: false }
                });
            }
            Some("v") if w.len() == 4 => out.mesh.verts.push([num(w[1])?, num(w[2])?, num(w[3])?]),
            Some("f") if w.len() == 5 => {
                out.mesh.tris.push([num(w[1])? as u32, num(w[2])? as u32, num(w[3])? as u32]);
                out.mesh.mats.push(num(w[4])? as u8);
            }
            None => {}
            _ => return Err(bad()),
        }
    }
    let nv = out.mesh.verts.len() as u32;
    if out.mesh.tris.iter().flatten().any(|&i| i >= nv) || out.mesh.mats.iter().any(|&m| m as usize >= out.mesh.materials.len()) {
        return Err("face index out of range".into());
    }
    if out.canvas.0 == 0 || out.canvas.1 == 0 {
        return Err("missing canvas".into());
    }
    Ok(out)
}

/// The `.mesh` text of a model (the inverse of [`parse`]).
pub fn to_text(m: &ModelFile) -> String {
    let role = |c: u8| ["void", "dark", "light", "hot"][c as usize & 3];
    let mut out = format!("TMESH1\nname {}\ncanvas {} {}\norient", m.name, m.canvas.0, m.canvas.1);
    for (k, v) in &m.orient {
        out += &format!(" {k}={v}");
    }
    out.push('\n');
    for (i, mat) in m.mesh.materials.iter().enumerate() {
        out += &format!("material m{i} {} {} {:.2}", mat.emissive as u8, mat.floor, mat.albedo);
        for &c in &mat.ramp {
            out += &format!(" {}", role(c));
        }
        out.push('\n');
    }
    for v in &m.mesh.verts {
        out += &format!("v {:.4} {:.4} {:.4}\n", v[0], v[1], v[2]);
    }
    for (t, mat) in m.mesh.tris.iter().zip(&m.mesh.mats) {
        out += &format!("f {} {} {} {}\n", t[0], t[1], t[2], mat);
    }
    out
}

pub fn load(path: &Path) -> Result<ModelFile, String> {
    parse(&std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?)
}

impl ModelFile {
    /// The orientation samples declared by `orient`: `roll`/`roll_max_deg` (banking),
    /// `yaw` (full turn), `pitch`/`pitch_max_deg` (toward the viewer). Sets combine yaw-major.
    pub fn orientations(&self) -> Vec<Orient> {
        let get = |k: &str| self.orient.get(k).copied();
        let span = |n: Option<f64>, max_deg: Option<f64>| -> Vec<f64> {
            match n {
                Some(n) if n > 1.0 => {
                    let n = n as usize;
                    let max = max_deg.unwrap_or(45.0).to_radians();
                    (0..n).map(|i| (i as f64 / (n - 1) as f64 * 2.0 - 1.0) * max).collect()
                }
                _ => vec![0.0],
            }
        };
        let yaws: Vec<f64> = match get("yaw") {
            Some(n) if n > 1.0 => (0..n as usize).map(|i| i as f64 * 2.0 * PI / n).collect(),
            _ => vec![0.0],
        };
        let rolls = span(get("roll"), get("roll_max_deg"));
        let pitches = span(get("pitch"), get("pitch_max_deg"));
        let mut out = Vec::new();
        for &yaw in &yaws {
            for &pitch in &pitches {
                for &roll in &rolls {
                    out.push(Orient { yaw, pitch, roll });
                }
            }
        }
        out
    }

    pub fn render_all(&self) -> Vec<SpriteImage> {
        let view = View::centered(self.canvas.0, self.canvas.1, 1.0);
        self.orientations().into_iter().map(|o| raster3d::render(&self.mesh, o, view)).collect()
    }
}

/// Share of a frame's opaque pixels per code (00, 01, 10, 11).
pub fn code_share(img: &SpriteImage) -> [f64; 4] {
    let mut n = [0usize; 4];
    for p in img.pixels.iter().flatten() {
        n[*p as usize & 3] += 1;
    }
    let total = n.iter().sum::<usize>().max(1) as f64;
    n.map(|c| c as f64 / total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_round_trips() {
        let q = crate::hand::gantry_queen();
        let back = parse(&to_text(&q)).unwrap();
        assert_eq!(back.mesh.tris, q.mesh.tris);
        assert_eq!(back.canvas, q.canvas);
        assert_eq!(back.orientations().len(), 49);
    }

    #[test]
    fn parses_and_orients() {
        let m = parse("TMESH1\nname t\ncanvas 8 8\norient roll=3 roll_max_deg=30 yaw=4\nmaterial hull 0 1 0.9 void dark light\nv 0 0 0\nv 4 0 0\nv 0 4 1\nf 0 1 2 0\n").unwrap();
        assert_eq!(m.mesh.tris.len(), 1);
        let o = m.orientations();
        assert_eq!(o.len(), 12);
        assert!((o[0].roll + 30f64.to_radians()).abs() < 1e-9);
        assert!(parse("TMESH1\ncanvas 8 8\nmaterial a 0 0 1 void\nv 0 0 0\nf 0 1 2 0\n").is_err());
    }
}
