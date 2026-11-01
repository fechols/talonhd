//! Procedural stand-in models and the tech-scene level (host only). These
//! prove the pipeline before any bible-approved Ludo models exist; they are
//! not game content.
//!
//! Roles of the four global codes (TALON HD palette law):
//! 00 `void`, 01 `hull_dark`, 10 `hull_light`, 11 `hot`.

use std::f64::consts::PI;

use crate::bake::Panorama;
use crate::demo;
use crate::encode::SpriteImage;
use crate::raster3d::{self, Material, Mesh, Orient, View};
use crate::render::Palette;

pub const VOID: u8 = 0;
pub const DARK: u8 = 1;
pub const LIGHT: u8 = 2;
pub const HOT: u8 = 3;

/// Tech-scene palette: S1 BREAKWATER (docs/story/hd/02-ladder.md): blue sea, mid grey, white, cyan.
pub const DEMO_PALETTE: Palette = Palette::new(6, 12, 1, 3);

/// The approved S1 ship and boss, if their prepared models exist (`tools/model_prep.py`).
pub const TALON_MESH: &str = "assets/models/hd.ship.talon.mesh";
pub const QUEEN_MESH: &str = "assets/models/boss.s1.gantry_queen.mesh";

/// The player's stand-in fighter: nose +y, about 64 long, 60 across.
pub fn fighter() -> Mesh {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, LIGHT], 1.0, 1),
        Material::lit(&[DARK, LIGHT, HOT], 0.9, 1),
        Material::glow(HOT),
    ]);
    let nose = [0.0, 34.0, 2.0];
    let canopy = [0.0, 8.0, 10.0];
    let spine = [0.0, -24.0, 6.0];
    let tail = [0.0, -30.0, 3.0];
    let cheek = [6.0, 14.0, 2.0];
    let hip = [8.0, -8.0, 3.0];
    let rear = [6.0, -30.0, 2.0];
    // Fuselage.
    m.sym(&[nose, cheek, canopy], 0);
    m.sym(&[canopy, cheek, hip, spine], 0);
    m.sym(&[spine, hip, rear, tail], 0);
    // Canopy glass.
    m.sym(&[[0.0, 20.0, 6.0], [3.0, 10.0, 6.5], canopy], 1);
    // Swept wings with a raised mid ridge.
    let root_le = [7.0, 4.0, 2.0];
    let ridge = [16.0, -10.0, 4.0];
    let tip = [30.0, -22.0, 0.0];
    let tip_te = [28.0, -28.0, 0.0];
    let root_te = [8.0, -24.0, 2.0];
    m.sym(&[root_le, tip, ridge], 0);
    m.sym(&[ridge, tip, tip_te], 0);
    m.sym(&[root_le, ridge, root_te, hip], 0);
    m.sym(&[ridge, tip_te, root_te], 0);
    // Twin fins, canted out.
    m.sym(&[[5.0, -16.0, 4.0], [9.0, -30.0, 13.0], [6.0, -30.0, 4.0]], 1);
    // Engines.
    m.sym(&[[2.0, -30.0, 3.0], [5.0, -30.0, 3.0], [5.0, -33.0, 2.0], [2.0, -33.0, 2.0]], 2);
    m
}

/// A drone: an armoured diamond with two forward claws and a glowing eye.
pub fn drone() -> Mesh {
    let mut m = Mesh::new(vec![
        Material::lit(&[VOID, DARK, HOT], 1.0, 1),
        Material::lit(&[VOID, DARK, LIGHT], 0.8, 1),
        Material::glow(LIGHT),
    ]);
    let top = [0.0, 0.0, 11.0];
    let back = [0.0, -14.0, 0.0];
    let side = [13.0, 0.0, 0.0];
    let front = [0.0, 10.0, 2.0];
    m.sym(&[top, side, back], 0);
    m.sym(&[top, front, side], 0);
    // Claws.
    m.sym(&[[4.0, 8.0, 2.0], [11.0, 6.0, 1.0], [7.0, 22.0, 0.0]], 1);
    m.sym(&[[4.0, 8.0, 2.0], [7.0, 22.0, 0.0], [3.0, 14.0, 4.0]], 1);
    // Eye.
    m.poly(&[[0.0, 6.0, 7.0], [3.0, 2.0, 8.5], [0.0, -1.0, 9.5], [-3.0, 2.0, 8.5]], 2);
    m
}

/// The tech-scene boss: a flying-wing fortress about 1000 wide and 400 long,
/// nose +y. Hull ridge, stepped swept wings, nacelles, turrets and cores.
pub fn fortress() -> Mesh {
    let mut m = Mesh::new(vec![
        // Enemies are dark and hot; light belongs to the player (palette law).
        Material::lit(&[VOID, DARK], 1.0, 1),       // plating
        Material::lit(&[VOID, DARK], 0.7, 1),       // dark armour
        Material::lit(&[VOID, DARK, HOT], 0.9, 1),  // turrets
        Material::glow(HOT),                          // cores and exhausts
        Material::glow(LIGHT),                        // running lights
    ]);
    // Hull: five cross-sections, each (y, half width, ridge height).
    let st = [(165.0, 6.0, 14.0), (110.0, 42.0, 44.0), (40.0, 72.0, 66.0), (-110.0, 62.0, 56.0), (-235.0, 40.0, 30.0)];
    let section = |(y, w, h): (f64, f64, f64)| -> [[f64; 3]; 3] { [[0.0, y, h], [w * 0.55, y, h * 0.8], [w, y, 0.0]] };
    for k in 0..st.len() - 1 {
        let (a, b) = (section(st[k]), section(st[k + 1]));
        m.sym(&[a[0], a[1], b[1], b[0]], 0);
        m.sym(&[a[1], a[2], b[2], b[1]], 1);
    }
    let r = section(st[4]);
    m.sym(&[r[0], r[1], r[2], [0.0, -235.0, 0.0]], 1);
    // Armour plates stepped along the spine.
    for (y, len) in [(95.0, 30.0), (60.0, 46.0), (-10.0, 50.0), (-80.0, 46.0), (-150.0, 40.0)] {
        m.frustum([-18.0, y - len, 58.0], [18.0, y, 58.0], 6.0, 70.0, 1);
    }
    // Wings: span sections at x with leading edge y, trailing edge y, ridge height.
    let span = [(60.0, 70.0, -190.0, 26.0), (210.0, 0.0, -205.0, 20.0), (360.0, -70.0, -215.0, 12.0), (500.0, -140.0, -200.0, 4.0)];
    for k in 0..span.len() - 1 {
        let (x0, le0, te0, h0) = span[k];
        let (x1, le1, te1, h1) = span[k + 1];
        let mid0 = (le0 + te0) / 2.0 + 15.0;
        let mid1 = (le1 + te1) / 2.0 + 15.0;
        let plate = if k == 1 { 1 } else { 0 };
        m.sym(&[[x0, le0, 6.0], [x1, le1, 2.0], [x1, mid1, h1], [x0, mid0, h0]], plate);
        m.sym(&[[x0, mid0, h0], [x1, mid1, h1], [x1, te1, 2.0], [x0, te0, 6.0]], 1 - plate);
    }
    // Wingtip fins.
    m.sym(&[[490.0, -120.0, 4.0], [500.0, -205.0, 4.0], [470.0, -215.0, 40.0]], 0);
    // Nacelles with glowing exhausts.
    for x in [150.0, 300.0] {
        m.frustum([x - 26.0, -200.0, 8.0], [x + 26.0, 30.0 - x / 5.0, 8.0], 9.0, 38.0, 0);
        m.frustum([-x - 26.0, -200.0, 8.0], [-x + 26.0, 30.0 - x / 5.0, 8.0], 9.0, 38.0, 0);
        m.sym(&[[x - 14.0, -200.0, 30.0], [x + 14.0, -200.0, 30.0], [x + 10.0, -212.0, 26.0], [x - 10.0, -212.0, 26.0]], 3);
    }
    // Turrets: armoured domes with hot muzzles.
    for (x, y) in [(225.0, -60.0), (390.0, -120.0), (0.0, 118.0)] {
        for sx in if x == 0.0 { vec![1.0] } else { vec![1.0, -1.0] } {
            let cx = x * sx;
            m.frustum([cx - 18.0, y - 18.0, 14.0], [cx + 18.0, y + 18.0, 14.0], 7.0, 36.0, 2);
            m.poly(&[[cx - 3.0, y + 11.0, 37.0], [cx + 3.0, y + 11.0, 37.0], [cx + 3.0, y + 34.0, 34.0], [cx - 3.0, y + 34.0, 34.0]], 3);
        }
    }
    // Reactor cores along the hull.
    for (y, s) in [(70.0, 14.0), (-45.0, 18.0)] {
        m.poly(&[[0.0, y + s, 74.0], [s * 0.8, y, 74.0], [0.0, y - s, 74.0], [-s * 0.8, y, 74.0]], 3);
    }
    // Running lights at the wing roots.
    m.sym(&[[64.0, 72.0, 7.0], [76.0, 60.0, 7.0], [70.0, 52.0, 7.0]], 4);
    m
}

/// Bank angles from full left to full right.
fn banks(n: u16, max: f64) -> impl Iterator<Item = f64> {
    (0..n).map(move |i| (i as f64 / (n - 1) as f64 * 2.0 - 1.0) * max)
}

/// The tech scene with the Ludo TALON as the player's ship and the Ludo GANTRY QUEEN as the boss when
/// their meshes are present.
pub fn demo_frames_with_assets() -> Vec<SpriteImage> {
    let mut frames = demo_frames();
    if let Ok(m) = crate::mesh::load(std::path::Path::new(QUEEN_MESH)) {
        let rendered = m.render_all();
        assert_eq!(rendered.len(), demo::BOSS_BANKS as usize, "boss roll samples must match BOSS_BANKS");
        for (i, f) in rendered.into_iter().enumerate() {
            assert_eq!((f.w, f.h), (demo::BOSS_SIZE, demo::BOSS_SIZE));
            frames[demo::BOSS_FRAME as usize + i] = f;
        }
    }
    if let Ok(m) = crate::mesh::load(std::path::Path::new(TALON_MESH)) {
        let view = View::centered(demo::PLAYER_W, demo::PLAYER_H, 1.0);
        let max = m.orient.get("roll_max_deg").copied().unwrap_or(60.0).to_radians();
        for (i, roll) in banks(demo::PLAYER_BANKS, max).enumerate() {
            frames[(demo::PLAYER_FRAME as usize) + i] = raster3d::render(&m.mesh, Orient { yaw: 0.0, pitch: 0.0, roll }, view);
        }
    }
    frames
}

/// Every sprite frame of the tech scene (procedural stand-ins only), in [`demo`] directory order.
pub fn demo_frames() -> Vec<SpriteImage> {
    let mut out = Vec::new();
    let boss = fortress();
    let view = View::centered(demo::BOSS_SIZE, demo::BOSS_SIZE, 1.0);
    for roll in banks(demo::BOSS_BANKS, 0.42) {
        // Nose down, toward the player; banking right dips the right wing.
        out.push(raster3d::render(&boss, Orient { yaw: PI, pitch: 0.0, roll }, view));
    }
    let d = drone();
    let view = View::centered(demo::DRONE_W, demo::DRONE_H, 1.0);
    for i in 0..demo::DRONE_YAWS {
        let yaw = PI + i as f64 * 2.0 * PI / demo::DRONE_YAWS as f64;
        out.push(raster3d::render(&d, Orient { yaw, pitch: 0.35, roll: 0.0 }, view));
    }
    let f = fighter();
    let view = View::centered(demo::PLAYER_W, demo::PLAYER_H, 1.0);
    for roll in banks(demo::PLAYER_BANKS, 1.0) {
        out.push(raster3d::render(&f, Orient { yaw: 0.0, pitch: 0.0, roll }, view));
    }
    assert_eq!(out.len(), demo::FRAMES as usize);
    out
}

/// A ground structure for the panorama: a platform of stepped blocks.
fn platform(seed: u32) -> Mesh {
    // The background sits one step down: platforms are only ever the void/dark checker (or void), so
    // solid dark always means the play layer.
    let mut m = Mesh::new(vec![Material::lit(&[VOID, DARK], 0.42, 0), Material::lit(&[VOID, DARK], 0.6, 0)]);
    let mut rng = seed;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };
    m.frustum([-120.0, -60.0, 0.0], [120.0, 60.0, 0.0], 8.0, 10.0, 0);
    for _ in 0..7 {
        let x = (next() % 200) as f64 - 100.0;
        let y = (next() % 90) as f64 - 45.0;
        let w = 8.0 + (next() % 22) as f64;
        let h = 8.0 + (next() % 16) as f64;
        let z = 14.0 + (next() % 20) as f64;
        m.frustum([x - w, y - h, 10.0], [x + w, y + h, 10.0], 3.0, z, (next() % 2) as u8);
    }
    m
}

/// The tech-scene panorama: a starfield with platforms drifting far below,
/// darker than the sprites (value-first: the play layer owns the contrast).
pub fn demo_panorama() -> Panorama {
    let period = demo::PANO_PERIOD as usize;
    let mut codes = vec![VOID; 160 * period];
    let mut rng = 0x2545_F491u32;
    let mut next = move || {
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        rng
    };
    for _ in 0..420 {
        let (x, y) = ((next() % 160) as usize, (next() % period as u32) as usize);
        codes[y * 160 + x] = if next() % 9 == 0 { LIGHT } else { DARK };
    }
    for (k, &(cx, cy)) in [(40i32, 120i32), (120, 380), (30, 640), (110, 880)].iter().enumerate() {
        let view = View::centered(140, 140, 1.0);
        let img = raster3d::render(&platform(k as u32 * 7919 + 1), Orient { yaw: k as f64 * 0.4, ..Orient::default() }, view);
        for y in 0..img.h as i32 {
            for x in 0..img.w as i32 {
                let (px, py) = (cx + x - img.w as i32 / 2, cy + y - img.h as i32 / 2);
                if let (Some(c), true) = (img.get(x as u16, y as u16), (0..160).contains(&px)) {
                    codes[py.rem_euclid(period as i32) as usize * 160 + px as usize] = c;
                }
            }
        }
    }
    // Append the first 200 lines so every 200-line window is contiguous.
    let wrap = codes[..160 * 200].to_vec();
    codes.extend_from_slice(&wrap);
    let rows = ((period + 200) / 8) as u16;
    Panorama { rows, codes, palette: DEMO_PALETTE }
}
