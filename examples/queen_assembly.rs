//! The whole GANTRY QUEEN, body plus arms and legs placed exactly as `demo::scene` places them, at full
//! left bank, level and full right bank, on the S1 sea: the review sheet for the part offsets.
//! `cargo run --release --example queen_assembly -- [output/lineup/queen.png]`
use std::path::Path;

use talonhd::encode::SpriteImage;
use talonhd::{demo, hand, palette::PALETTE, s1};

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "output/lineup/queen.png".into());
    let body = hand::gantry_queen().render_all();
    let parts: Vec<Vec<SpriteImage>> = hand::queen_parts().iter().map(|m| m.render_all()).collect();
    let (cw, ch) = (512usize, 400usize);
    let banks = [0u16, 24, 48];
    let mut px = vec![PALETTE[s1::PALETTE.d021 as usize]; cw * banks.len() * ch];
    let width = cw * banks.len();
    for (col, &b) in banks.iter().enumerate() {
        let deg = b as i16 - demo::QUEEN_ROLL_MAX_DEG;
        let r = ((b + 4) / 8) as usize;
        let (cx, cy) = ((col * cw + cw / 2) as i32, (ch / 2) as i32 - 40);
        let mut draw = |img: &SpriteImage, x0: i32, y0: i32| {
            for y in 0..img.h as i32 {
                for x in 0..img.w as i32 {
                    let (sx, sy) = (x0 + x, y0 + y);
                    if sx >= (col * cw) as i32 && sx < ((col + 1) * cw) as i32 && (0..ch as i32).contains(&sy) {
                        if let Some(c) = img.get(x as u16, y as u16) {
                            px[sy as usize * width + sx as usize] = PALETTE[s1::PALETTE.color_of(c) as usize];
                        }
                    }
                }
            }
        };
        for (i, sx) in [-1i16, 1].into_iter().enumerate() {
            let c = demo::QUEEN_ARM_CENTER;
            let (dx, dy) = demo::part_offset([sx * c[0], c[1], c[2]], deg);
            let f = &parts[i * demo::ARM_AIMS as usize + 2][r];
            draw(f, cx + dx as i32 - f.w as i32 / 2, cy + dy as i32 - f.h as i32 / 2);
        }
        for (i, (ax, ay)) in [(-1i16, 1i16), (1, 1), (-1, -1), (1, -1)].into_iter().enumerate() {
            let c = demo::QUEEN_LEG_CENTER;
            let (dx, dy) = demo::part_offset([ax * c[0], ay * c[1], c[2]], deg);
            let f = &parts[2 * demo::ARM_AIMS as usize + i * demo::LEG_STRIDES as usize][r];
            draw(f, cx + dx as i32 - f.w as i32 / 2, cy + dy as i32 - f.h as i32 / 2);
        }
        let f = &body[b as usize];
        draw(f, cx - f.w as i32 / 2, cy - f.h as i32 / 2);
    }
    talonhd::png::save_wide(Path::new(&out), width, &px).unwrap();
    println!("wrote {out}");
}
