//! The solid-polygon renderer: geometry, lighting, ramps and goldens.

use talonhd::raster3d::{self, Material, Mesh, Orient, View};
use talonhd::{demo, models};

fn slab() -> Mesh {
    let mut m = Mesh::new(vec![Material::lit(&[0, 1, 2], 1.0, 0), Material::glow(3)]);
    m.cuboid([-20.0, -10.0, 0.0], [20.0, 10.0, 4.0], 0);
    m
}

#[test]
fn box_covers_its_footprint() {
    // 40 hires wide = 20 multicolor pixels, 20 lines tall, centred in 32×32.
    let img = raster3d::render(&slab(), Orient::default(), View::centered(32, 32, 1.0));
    let opaque = |x: u16, y: u16| img.get(x, y).is_some();
    assert!(opaque(16, 16));
    assert!(opaque(6, 6) && opaque(25, 25));
    assert!(!opaque(5, 16) && !opaque(26, 16));
    assert!(!opaque(16, 5) && !opaque(16, 26));
    let n = img.pixels.iter().filter(|p| p.is_some()).count();
    assert_eq!(n, 20 * 20);
}

#[test]
fn flat_top_is_one_solid_or_checker_shade() {
    let img = raster3d::render(&slab(), Orient::default(), View::centered(32, 32, 1.0));
    let mut codes: Vec<u8> = (8..24).flat_map(|y| (8..24).map(move |x| (x, y))).map(|(x, y)| img.get(x, y).unwrap()).collect();
    codes.sort();
    codes.dedup();
    assert!(codes.len() <= 2, "a face shades to one code or a two-code checker: {codes:?}");
}

#[test]
fn emissive_ignores_light_and_roll_changes_shading() {
    let mut m = Mesh::new(vec![Material::glow(3)]);
    m.poly(&[[-10.0, -10.0, 0.0], [10.0, -10.0, 0.0], [10.0, 10.0, 0.0], [-10.0, 10.0, 0.0]], 0);
    for roll in [-1.0, 0.0, 1.0] {
        let img = raster3d::render(&m, Orient { roll, ..Orient::default() }, View::centered(16, 24, 1.0));
        assert!(img.pixels.iter().flatten().all(|&c| c == 3));
    }
    let f = models::fighter();
    let view = View::centered(demo::PLAYER_W, demo::PLAYER_H, 1.0);
    let level = raster3d::render(&f, Orient::default(), view);
    let banked = raster3d::render(&f, Orient { roll: 0.8, ..Orient::default() }, view);
    assert_ne!(raster3d::hash(&level), raster3d::hash(&banked));
    // A level ship is mirror-symmetric in coverage.
    for y in 0..level.h {
        for x in 0..level.w {
            assert_eq!(level.get(x, y).is_some(), level.get(level.w - 1 - x, y).is_some(), "({x},{y})");
        }
    }
}

/// Golden hashes of the tech-scene frames. A change here is a change of the
/// look: regenerate deliberately (`cargo test -- --nocapture` prints them).
#[test]
fn demo_frames_golden() {
    let frames = models::demo_frames();
    let (b, d, p) = (demo::BOSS_FRAME as usize, demo::DRONE_FRAME as usize, demo::PLAYER_FRAME as usize);
    let h: Vec<u64> = [b, b + 24, b + 48, d, d + 4, p, p + 4, p + 8].iter().map(|&i| raster3d::hash(&frames[i])).collect();
    println!("{h:#018x?}");
    assert_eq!(h, GOLDEN);
}

const GOLDEN: [u64; 8] = [
    0xa55a6779a3edc59d,
    0x809eb627ae768b58,
    0x246ae3f7c2304eb2,
    0x4ecc311735df122c,
    0x978d09a679c072cc,
    0x72d55191aae5e8f0,
    0x52695fae871540a2,
    0x3da06e5ed13076e8,
];
