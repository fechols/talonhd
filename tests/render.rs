//! The DMA renderer (running against the REU model) must match the naive
//! per-pixel compositor byte for byte: bitmap codes, screen RAM, color RAM.
//! Vendored from gclass (`a5775bf`), adapted to the vertical panorama, four
//! global colors and 1-phase frames.

use talonhd::bake::{self, Panorama};
use talonhd::encode::SpriteImage;
use talonhd::layout::{BUFFERS, Buffer};
use talonhd::model::Machine;
use talonhd::reference::{self, Expected};
use talonhd::render::{self, Palette, Scroll, Sprite};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 16) as u32
    }
    fn range(&mut self, lo: i32, hi: i32) -> i32 {
        lo + (self.next() % (hi - lo + 1) as u32) as i32
    }
}

fn panorama(rng: &mut Rng, rows: u16) -> Panorama {
    let palette = Palette { d021: 6, screen: rng.next() as u8, color: (rng.next() & 15) as u8 };
    Panorama { rows, codes: (0..160 * rows as usize * 8).map(|_| (rng.next() & 3) as u8).collect(), palette }
}

/// Blobby sprite: a few ellipses (long opaque runs, ragged edges, holes)
/// plus speckle so every segment kind shows up.
fn sprite(rng: &mut Rng, w: u16, h: u16) -> SpriteImage {
    let mut img = SpriteImage::new(w, h);
    let blobs: Vec<(f32, f32, f32, f32, u8)> = (0..1 + rng.next() % 4)
        .map(|_| {
            (
                rng.range(0, w as i32) as f32,
                rng.range(0, h as i32) as f32,
                1.0 + rng.range(0, w as i32) as f32 / 2.0,
                1.0 + rng.range(0, h as i32) as f32 / 2.0,
                (rng.next() & 3) as u8,
            )
        })
        .collect();
    for y in 0..h {
        for x in 0..w {
            let mut v = None;
            for &(cx, cy, rx, ry, code) in &blobs {
                let dx = (x as f32 - cx) / rx;
                let dy = (y as f32 - cy) / ry;
                if dx * dx + dy * dy <= 1.0 {
                    v = Some(code ^ ((x / 7 + y / 5) & 1) as u8);
                }
            }
            if rng.next() % 50 == 0 {
                v = if v.is_some() { None } else { Some((rng.next() & 3) as u8) };
            }
            img.set(x, y, v);
        }
    }
    img
}

fn compare(m: &Machine, buf: Buffer, want: &Expected, what: &str) {
    let codes = m.codes(buf);
    if let Some(i) = (0..codes.len()).find(|&i| codes[i] != want.codes[i]) {
        panic!("{what}: pixel ({}, {}) code {} want {}", i % 160, i / 160, codes[i], want.codes[i]);
    }
    for i in 0..1000 {
        assert_eq!(m.ram[buf.screen as usize + i], want.screen[i], "{what}: screen cell {i}");
        assert_eq!(m.color_ram[i] & 15, want.color[i], "{what}: color cell {i}");
    }
    assert_eq!(m.stats.bank_wraps, 0, "{what}: a DMA wrapped inside a 512 KiB bank");
    assert_eq!(m.stats.stray_writes, 0, "{what}: a DMA touched I/O");
}

fn run(m: &mut Machine, pano: &Panorama, frames: &[SpriteImage], buf: Buffer, scroll_px: u16, sprites: &[Sprite], what: &str) {
    let img = render::read_image(m).expect("header");
    let scroll = Scroll::new(&img, scroll_px);
    let mut list = sprites.to_vec();
    render::set_palette(m, img.palette);
    assert_eq!(m.d021, img.palette.d021);
    render::render_frame(m, &img, buf, scroll, &mut list);
    render::present(m, buf, scroll);
    assert_eq!(m.dd00 & 3, buf.dd00);
    assert_eq!(m.d011 & 7, 7 - scroll.fine);
    let want = reference::compose(pano, frames, scroll, sprites);
    compare(m, buf, &want, what);
}

#[test]
fn randomized_scenes() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let pano = panorama(&mut rng, 61);
    let sizes = [(1, 1), (3, 5), (4, 8), (5, 9), (17, 33), (64, 64), (192, 192), (160, 200), (300, 100), (511, 7), (512, 512)];
    let mut frames: Vec<SpriteImage> = sizes.iter().map(|&(w, h)| sprite(&mut rng, w, h)).collect();
    // Some 1-phase frames (drawn at x & !3).
    for i in [2usize, 6, 10] {
        let mut f = frames[i].clone();
        f.phases = 1;
        frames.push(f);
    }
    // Two dense 512×512 frames (~1 MiB) push later frames across REU banks.
    for _ in 0..2 {
        let mut dense = SpriteImage::new(512, 512);
        for y in 0..512 {
            for x in 0..512 {
                let hole = (x / 9 + y / 13) % 7 == 0;
                dense.set(x, y, if hole { None } else { Some(((x ^ y) & 3) as u8) });
            }
        }
        frames.push(dense);
    }
    let image = bake::build_image(&pano, &frames, 0).unwrap();
    assert!(image.len() > 0x8_0000, "image should span REU banks to exercise the allocator");
    let mut m = Machine::new(&image);
    for case in 0..150 {
        let n = rng.range(1, 7) as usize;
        let sprites: Vec<Sprite> = (0..n)
            .map(|_| {
                let frame = rng.range(0, frames.len() as i32 - 1) as u16;
                let (w, h) = (frames[frame as usize].w as i32, frames[frame as usize].h as i32);
                Sprite {
                    frame,
                    // Mostly near or overlapping the screen; sometimes far off.
                    x: rng.range(-w - 8, 168) as i16,
                    y: rng.range(-h - 8, 208) as i16,
                    z: rng.range(0, 3) as i16,
                }
            })
            .collect();
        let scroll = rng.range(0, (61 - 25) * 8 + 4) as u16;
        let buf = BUFFERS[case & 1];
        run(&mut m, &pano, &frames, buf, scroll, &sprites, &format!("case {case} {sprites:?}"));
    }
}

#[test]
fn clipping_edges_every_phase() {
    let mut rng = Rng(12345);
    let pano = panorama(&mut rng, 27);
    let mut frames = vec![sprite(&mut rng, 5, 9), sprite(&mut rng, 13, 17), sprite(&mut rng, 13, 17)];
    frames[2].phases = 1;
    let image = bake::build_image(&pano, &frames, 0).unwrap();
    let mut m = Machine::new(&image);
    for frame in 0..3u16 {
        let (w, h) = (frames[frame as usize].w as i16, frames[frame as usize].h as i16);
        for x in (-w - 1..=4).chain(150..=161) {
            for y in [-h - 8, -h - 1, -h, -h + 1, -7, -1, 0, 1, 7, 8, 96, 191, 192, 195, 199, 200] {
                for scroll in [0u16, 3, 7, 13] {
                    let s = [Sprite { frame, x, y, z: 0 }];
                    run(&mut m, &pano, &frames, BUFFERS[0], scroll, &s, &format!("frame {frame} x {x} y {y} scroll {scroll}"));
                }
            }
        }
    }
}

#[test]
fn fully_opaque_sprite_covering_screen() {
    let mut rng = Rng(777);
    let pano = panorama(&mut rng, 25);
    let mut big = SpriteImage::new(512, 512);
    for y in 0..512 {
        for x in 0..512 {
            big.set(x, y, Some(((x / 4 + y / 8) & 3) as u8));
        }
    }
    let frames = vec![big];
    let image = bake::build_image(&pano, &frames, 0).unwrap();
    let mut m = Machine::new(&image);
    for (x, y) in [(-100, -100), (-3, -5), (-351, -311), (0, 0)] {
        run(&mut m, &pano, &frames, BUFFERS[1], 0, &[Sprite { frame: 0, x, y, z: 0 }], &format!("{x},{y}"));
    }
}

#[test]
fn scrolling_fine_and_coarse() {
    let mut rng = Rng(4242);
    let pano = panorama(&mut rng, 40);
    let frames = vec![sprite(&mut rng, 192, 192)];
    let image = bake::build_image(&pano, &frames, 0).unwrap();
    let mut m = Machine::new(&image);
    for scroll in 0..=(40 - 25) * 8 + 5 {
        let s = [Sprite { frame: 0, x: 3 - scroll as i16 / 3, y: -40 + scroll as i16 / 2, z: 0 }];
        run(&mut m, &pano, &frames, BUFFERS[scroll as usize & 1], scroll, &s, &format!("scroll {scroll}"));
    }
}

#[test]
fn palette_is_global() {
    let mut rng = Rng(99);
    let pano = panorama(&mut rng, 25);
    let image = bake::build_image(&pano, &[], 0).unwrap();
    let mut m = Machine::new(&image);
    let pal = Palette::new(2, 9, 8, 10);
    render::set_palette(&mut m, pal);
    assert_eq!(m.d021, 2);
    for b in BUFFERS {
        assert!(m.ram[b.screen as usize..b.screen as usize + 1000].iter().all(|&v| v == 0x98));
    }
    assert!(m.color_ram[..1000].iter().all(|&c| c == 10));
    assert_eq!((0..4).map(|c| pal.color_of(c)).collect::<Vec<_>>(), [2, 9, 8, 10]);
}

#[test]
fn z_order_is_stable_painters() {
    let mut list: Vec<Sprite> = [3, 1, 3, 0, 1, 2]
        .iter()
        .enumerate()
        .map(|(i, &z)| Sprite { frame: i as u16, x: 0, y: 0, z })
        .collect();
    render::sort_by_z(&mut list);
    let order: Vec<u16> = list.iter().map(|s| s.frame).collect();
    assert_eq!(order, [3, 1, 4, 5, 0, 2]);
}
