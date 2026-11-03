//! Procedural stand-in models and the tech-scene level (host only). These
//! prove the pipeline before any bible-approved Ludo models exist; they are
//! not game content.
//!
//! Roles of the four global codes (TALON HD palette law):
//! 00 `void`, 01 `hull_dark`, 10 `hull_light`, 11 `hot`.

use std::f64::consts::PI;

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
pub const WASP_MESH: &str = "assets/models/enemy.common.wasp.mesh";

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

/// The tech scene with the hand-built TALON as the player's ship and the hand-built GANTRY QUEEN as the
/// boss when their meshes are present.
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
    // Her wreck: the body banked left, level and right, the Lattice dead, scorched and holed.
    if let Ok(m) = crate::mesh::load(std::path::Path::new(QUEEN_MESH)) {
        for (k, f) in crate::hand::queen_wreck_frames(&m).into_iter().enumerate() {
            frames[demo::QUEEN_WRECK_FRAME as usize + k] = f;
        }
    }
    // Her arms and legs, when the body is hers: posed per bank sample, rendered from `hand.rs`.
    if std::path::Path::new(QUEEN_MESH).exists() {
        for (k, f) in crate::hand::queen_part_frames().into_iter().enumerate() {
            frames[demo::ARM_FRAME as usize + k] = f;
        }
    }
    if let Ok(m) = crate::mesh::load(std::path::Path::new(WASP_MESH)) {
        // The hand-built WASP flies the drone paths: its 16 yaws, level (on the stand-in's canvas).
        let view = View::centered(demo::DRONE_W, demo::DRONE_H, 1.0);
        for i in 0..demo::DRONE_YAWS {
            let yaw = PI + i as f64 * 2.0 * PI / demo::DRONE_YAWS as f64;
            frames[(demo::DRONE_FRAME + i) as usize] = raster3d::render(&m.mesh, Orient { yaw, pitch: 0.0, roll: 0.0 }, view);
        }
    }
    if let Ok(m) = crate::mesh::load(std::path::Path::new(TALON_MESH)) {
        let view = View::centered(demo::PLAYER_W, demo::PLAYER_H, 1.0);
        let max = m.orient.get("roll_max_deg").copied().unwrap_or(60.0).to_radians();
        for (i, roll) in banks(demo::PLAYER_BANKS, max).enumerate() {
            frames[(demo::PLAYER_FRAME as usize) + i] = raster3d::render(&m.mesh, Orient { yaw: 0.0, pitch: 0.0, roll }, view);
        }
    }
    let ships: Vec<SpriteImage> = frames[demo::PLAYER_FRAME as usize..(demo::PLAYER_FRAME + demo::PLAYER_BANKS) as usize].to_vec();
    for (k, f) in shield_frames(&ships).into_iter().enumerate() {
        frames[demo::SHIELD_FRAME as usize + k] = f;
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
    // The stand-in boss has no separate parts: blank frames hold their slots.
    for _ in demo::ARM_FRAME..demo::DRONE_FRAME {
        out.push(SpriteImage::new(4, 1));
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
    out.extend(game_frames());
    let ships: Vec<SpriteImage> = out[demo::PLAYER_FRAME as usize..(demo::PLAYER_FRAME + demo::PLAYER_BANKS) as usize].to_vec();
    out.extend(shield_frames(&ships));
    // The KEELBREAKER (hand-built): hull rolls, turret headings.
    let hull = crate::hand::keelbreaker().render_all();
    assert_eq!(hull.len(), demo::KEEL_ROLLS as usize);
    out.extend(hull);
    let turret = crate::hand::keelbreaker_turret().render_all();
    assert_eq!(turret.len(), demo::TURRET_YAWS as usize);
    out.extend(turret);
    out.extend(ground_frames());
    // The PEREGRINE (hand-built): its banks, then its energy-wave frames.
    out.extend(peregrine_frames(&raster3d::MORNING));
    // The cards' lettering.
    out.extend(crate::font::card_frames());
    assert_eq!(out.len(), demo::FRAMES as usize);
    out
}

/// The PEREGRINE's 9 banks and its energy waves under `light`: one field around the level bank, without
/// the hull (the bank frame is drawn over it), so 8 frames instead of 72 (the level image is full).
pub fn peregrine_frames(light: &raster3d::Light) -> Vec<SpriteImage> {
    let mut out = crate::hand::peregrine().render_all_lit(light);
    assert_eq!(out.len(), demo::PLAYER_BANKS as usize);
    let level = out[demo::PLAYER_BANKS as usize / 2].clone();
    let mut field = shield_frames(std::slice::from_ref(&level));
    let (ox, oy) = ((demo::SHIELD_W - level.w) / 2, (demo::SHIELD_H - level.h) / 2);
    for f in field.iter_mut() {
        for y in 0..level.h {
            for x in 0..level.w {
                if level.get(x, y).is_some() {
                    f.set(x + ox, y + oy, None);
                }
            }
        }
    }
    out.extend(field);
    out
}

/// Level `level`'s frame directory (`demo`'s stage block): S1's for 0; for S2 TERRACES the common frames
/// with the WASP and the PEREGRINE under S2's sun, S1's stage slots blank, and S2's block from 0
/// (`crate::hand_s2`).
pub fn stage_frames(level: u8) -> Vec<SpriteImage> {
    let mut frames = demo_frames_with_assets();
    if level == 0 {
        return frames;
    }
    assert_eq!(level, 1, "only S1 and S2 are built");
    let sun = crate::s2::SUN;
    for i in (demo::BOSS_FRAME..demo::DRONE_FRAME).chain(demo::KEEL_FRAME..demo::PEREGRINE_FRAME) {
        frames[i as usize] = SpriteImage::new(4, 1);
    }
    let wasp = crate::hand::wasp();
    let view = View::centered(demo::DRONE_W, demo::DRONE_H, 1.0);
    for i in 0..demo::DRONE_YAWS {
        let yaw = PI + i as f64 * 2.0 * PI / demo::DRONE_YAWS as f64;
        frames[(demo::DRONE_FRAME + i) as usize] = raster3d::render_lit(&wasp.mesh, Orient { yaw, pitch: 0.0, roll: 0.0 }, view, &sun);
    }
    for (k, f) in peregrine_frames(&sun).into_iter().enumerate() {
        frames[demo::PEREGRINE_FRAME as usize + k] = f;
    }
    let mut block: Vec<SpriteImage> = Vec::new();
    let roster: std::collections::HashMap<&str, Vec<SpriteImage>> = crate::hand_s2::roster().into_iter().collect();
    for name in ["pike", "kestrel", "roof_hatch", "bell_turret", "dome_mortar", "pillbox"] {
        block.extend(roster[name].iter().cloned());
    }
    // Wrecks: a crater for the PILLBOX and the BELL TURRET, a larger one for the DOME MORTAR (on their
    // canvases).
    block.extend(crate::hand::wreck(20.0).render_all_lit(&sun));
    block.extend(crate::hand::wreck(21.0).render_all_lit(&sun));
    for k in 0..demo::SHELL_SIZES {
        block.push(shell(k));
    }
    for name in ["crown", "hub", "astrolabe", "ring_outer", "ring_middle", "ring_inner", "lens"] {
        block.extend(roster[name].iter().cloned());
    }
    assert_eq!(block.len(), demo::S2_FRAMES as usize);
    for (i, f) in block.into_iter().enumerate() {
        frames[i] = f;
    }
    frames
}

/// The DOME MORTAR's shell at size `k` (0 small, high: it climbs, then falls): a `dark` ball with a
/// `hot` crystal core, two lines a multicolor pixel.
fn shell(k: u16) -> SpriteImage {
    let (w, h) = (demo::SHELL_W, demo::SHELL_H);
    let mut img = SpriteImage::new(w, h);
    let r = [1.2, 1.7, 2.2, 2.8][k as usize];
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f64 + 0.5 - w as f64 / 2.0, (y as f64 + 0.5 - h as f64 / 2.0) / 2.0);
            let d = dx.hypot(dy);
            if d < r {
                img.set(x, y, Some(if d < r - 0.9 { HOT } else { DARK }));
            }
        }
    }
    img
}

/// The ground and sea units, in [`demo`] order from `SKIFF_FRAME` (hand-built).
pub fn ground_frames() -> Vec<SpriteImage> {
    use crate::hand;
    let mut out = Vec::new();
    for m in [hand::skiff(), hand::pillbox_armed(false), hand::pillbox_armed(true)] {
        let f = m.render_all();
        assert_eq!(f.len(), demo::UNIT_YAWS as usize);
        out.extend(f);
    }
    assert_eq!(hand::HATCH_POSES, demo::HATCH_POSES as usize);
    for k in 0..hand::HATCH_POSES {
        out.extend(hand::hatch(k).render_all());
    }
    for m in [hand::crawler_armed(false), hand::crawler_armed(true)] {
        out.extend(m.render_all());
    }
    out.extend(hand::wreck(20.0).render_all());
    out.extend(hand::wreck(24.0).render_all());
    // The GANTRY QUEEN's wreck frames hold their slots (rendered from her mesh when it is present).
    for _ in 0..demo::QUEEN_WRECKS {
        out.push(SpriteImage::new(4, 1));
    }
    out
}

/// The game's frames after the ships: the GANNET (hand-built), TWIN pairs, enemy bullets, explosions and
/// SPARKs (drawn pixel by pixel: they are a few pixels across).
pub fn game_frames() -> Vec<SpriteImage> {
    let mut out = Vec::new();
    let g = crate::hand::gannet();
    let view = View::centered(demo::GANNET_W, demo::GANNET_H, 1.0);
    let max = g.orient.get("pitch_max_deg").copied().unwrap_or(50.0).to_radians();
    for yaw in [PI, 0.0] {
        for k in 0..demo::GANNET_PITCHES {
            // Index 0 is the full dive (nose toward the viewer), the last pitched away.
            let pitch = max * (1.0 - 2.0 * k as f64 / (demo::GANNET_PITCHES - 1) as f64);
            out.push(raster3d::render(&g.mesh, Orient { yaw, pitch, roll: 0.0 }, view));
        }
    }
    // TWIN: two 4 x 10 bolts 8 px apart (multicolor columns 1-2 and 5-6), light with a hot head and a
    // dark tail. Variants: left only, right only, both.
    for alive in 1..=3u8 {
        let mut img = SpriteImage::new(demo::PAIR_W, demo::PAIR_H);
        for (bit, x0) in [(1u8, 1u16), (2, 5)] {
            if alive & bit != 0 {
                for y in 0..demo::PAIR_H {
                    let c = if y < 2 { HOT } else if y < 8 { LIGHT } else { DARK };
                    img.set(x0, y, Some(c));
                    img.set(x0 + 1, y, Some(c));
                }
            }
        }
        out.push(img);
    }
    out.push(pattern(&[".h.", "hlh", "hlh", "hlh", ".h."]));
    out.push(pattern(&[".d.", "ddd", "ddd", "ddd", "ddd", "dhd", ".h.", ".h."]));
    for k in 0..demo::BURST_FRAMES {
        out.push(burst(k));
    }
    for k in 0..demo::SPARK_FRAMES {
        // A spinning diamond: its width breathes 3, 2, 1, 2 multicolor px.
        let half = [3i32, 2, 1, 2][k as usize];
        let mut img = SpriteImage::new(demo::SPARK_W, demo::SPARK_H);
        for y in 0..demo::SPARK_H as i32 {
            let w = (half * (5 - (y * 2 - 9).abs() / 2) + 4) / 5;
            for x in 0..demo::SPARK_W as i32 {
                let d = (x * 2 - 5).abs() / 2;
                if d < w.max(1) {
                    img.set(x as u16, y as u16, Some(if d + 1 >= w && w > 1 { LIGHT } else { HOT }));
                }
            }
        }
        out.push(img);
    }
    out
}

/// The invincible TALON: each bank frame centred in a larger canvas inside an energy field that follows
/// its silhouette. Distance from the hull (in hires px: a multicolor pixel is 2 wide) drives it: a faint
/// dotted glow against the hull, then two wave fronts half a loop apart rolling outward one pixel a
/// step, bright (`light`) as they leave and thinning to sparse `dark` dots as they fade at the edge.
/// Light belongs to the player (art direction): no `hot`.
pub fn shield_frames(ships: &[SpriteImage]) -> Vec<SpriteImage> {
    let (w, h) = (demo::SHIELD_W, demo::SHIELD_H);
    let steps = demo::SHIELD_STEPS as f64;
    let mut out = Vec::new();
    for ship in ships {
        let (ox, oy) = ((w - ship.w) / 2, (h - ship.h) / 2);
        // Distance (hires px) from every canvas pixel to the nearest hull pixel.
        let hull: Vec<(f64, f64)> = (0..ship.h)
            .flat_map(|y| (0..ship.w).map(move |x| (x, y)))
            .filter(|&(x, y)| ship.get(x, y).is_some())
            .map(|(x, y)| (((x + ox) * 2) as f64, (y + oy) as f64))
            .collect();
        let dist: Vec<f64> = (0..h)
            .flat_map(|y| (0..w).map(move |x| (x, y)))
            .map(|(x, y)| {
                let (px, py) = ((x * 2) as f64, y as f64);
                hull.iter().map(|&(hx, hy)| (hx - px).powi(2) + (hy - py).powi(2)).fold(f64::MAX, f64::min).sqrt()
            })
            .collect();
        // The field's reach: the canvas margin, minus a pixel.
        let reach = (((w - ship.w) as f64).min((h - ship.h) as f64 / 2.0) - 1.0).max(4.0);
        for step in 0..demo::SHIELD_STEPS {
            let mut img = SpriteImage::new(w, h);
            for y in 0..h {
                for x in 0..w {
                    let d = dist[(y * w + x) as usize];
                    if d < 0.5 || d > reach {
                        continue;
                    }
                    let checker = (x + y) % 2 == 0;
                    let mut c = None;
                    if d < 2.2 && checker {
                        c = Some(LIGHT);
                    }
                    for front in 0..2 {
                        let f = ((step as f64 / steps) + front as f64 * 0.5) % 1.0;
                        let at = 2.0 + (reach - 2.0) * f;
                        if (d - at).abs() < 1.1 {
                            c = if f < 0.35 {
                                Some(LIGHT)
                            } else if f < 0.7 {
                                if checker { Some(LIGHT) } else { c }
                            } else if checker && y % 2 == 0 {
                                Some(DARK)
                            } else {
                                c
                            };
                        }
                    }
                    if c.is_some() {
                        img.set(x, y, c);
                    }
                }
            }
            for y in 0..ship.h {
                for x in 0..ship.w {
                    if let Some(c) = ship.get(x, y) {
                        img.set(x + ox, y + oy, Some(c));
                    }
                }
            }
            out.push(img);
        }
    }
    out
}

/// A small sprite from text rows: `.` transparent, `v` void, `d` dark, `l` light, `h` hot.
fn pattern(rows: &[&str]) -> SpriteImage {
    let mut img = SpriteImage::new(rows[0].len() as u16, rows.len() as u16);
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            let c = match ch {
                'v' => Some(VOID),
                'd' => Some(DARK),
                'l' => Some(LIGHT),
                'h' => Some(HOT),
                _ => None,
            };
            img.set(x as u16, y as u16, c);
        }
    }
    img
}

/// Explosion frame `k`: a light flash with a hot ring, then a broken hot ring, then dark smoke. A
/// multicolor pixel is two lines wide, so radii in lines are halved across.
fn burst(k: u16) -> SpriteImage {
    let (w, h) = (demo::BURST_W, demo::BURST_H);
    let mut img = SpriteImage::new(w, h);
    let (r0, r1) = [(0.0, 9.0), (8.0, 16.0), (13.0, 21.0)][k as usize];
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = ((x as f64 + 0.5 - w as f64 / 2.0) * 2.0, y as f64 + 0.5 - h as f64 / 2.0);
            let r = (dx * dx + dy * dy).sqrt();
            let noise = ((x as u32 * 7919 + y as u32 * 104_729 + k as u32 * 31) * 2_654_435_761) >> 28;
            if r < r1 && r >= r0 {
                let c = match k {
                    0 => Some(if r < 5.0 { LIGHT } else { HOT }),
                    1 => match noise {
                        0..=3 => None,
                        4..=6 => Some(LIGHT),
                        _ => Some(HOT),
                    },
                    _ => match noise {
                        0..=8 => None,
                        9..=10 => Some(HOT),
                        _ => Some(DARK),
                    },
                };
                img.set(x, y, c);
            }
        }
    }
    img
}

