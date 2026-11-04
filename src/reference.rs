//! Naive per-pixel compositor (host only): the specification the DMA
//! renderer must match byte for byte. Vendored from gclass (`a5775bf`),
//! adapted to the vertical panorama, four global colors and the border
//! painter's 240-line canvas.

use crate::bake::Panorama;
use crate::encode::SpriteImage;
use crate::demo::PANO_PERIOD;
use crate::layout::{BAND_LINES, BAND_ROWS, HEIGHT, VISIBLE};
use crate::render::{Scroll, Sprite};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expected {
    /// 160×200 multicolor codes of the bitmap (not just the visible lines).
    pub codes: Vec<u8>,
    /// 160×24 codes of each painted block: above the window (bitmap lines `fine - 24 ..`) and below
    /// (`fine + 192 ..`).
    pub top: Vec<u8>,
    pub bottom: Vec<u8>,
    pub screen: Vec<u8>,
    pub color: Vec<u8>,
}

/// Bitmap lines -`BAND`..`HEIGHT + BAND` (the band rows above and below the bitmap).
const BAND: usize = BAND_ROWS as usize * 8;

pub fn compose(pano: &Panorama, frames: &[SpriteImage], scroll: Scroll, sprites: &[Sprite]) -> Expected {
    let lines = HEIGHT as usize + 2 * BAND;
    // Tall line t is bitmap line t - BAND: panorama line 8 * coarse + t - BAND, wrapped by the loop's
    // period past either end (the panorama loops), void outside the panorama.
    let first = scroll.coarse as i32 * 8 - BAND as i32;
    let end = pano.rows as i32 * 8;
    let period = PANO_PERIOD as i32;
    let mut codes = vec![0u8; lines * 160];
    for t in 0..lines {
        let mut p = first + t as i32;
        if p < 0 {
            p += period;
        } else if p >= end {
            p -= period;
        }
        if (0..pano.rows as i32 * 8).contains(&p) {
            codes[t * 160..(t + 1) * 160].copy_from_slice(&pano.codes[p as usize * 160..(p as usize + 1) * 160]);
        }
    }
    let mut order: Vec<Sprite> = sprites.to_vec();
    order.sort_by_key(|s| s.z);
    for s in &order {
        let Some(img) = frames.get(s.frame as usize) else { continue };
        let sx0 = if img.phases == 1 { s.x as i32 & !3 } else { s.x as i32 };
        let sy0 = s.y as i32 + scroll.fine as i32 + BAND as i32;
        for py in 0..img.h {
            let y = sy0 + py as i32;
            if !(0..lines as i32).contains(&y) {
                continue;
            }
            for px in 0..img.w {
                let x = sx0 + px as i32;
                if let (Some(code), true) = (img.get(px, py), (0..160).contains(&x)) {
                    codes[y as usize * 160 + x as usize] = code;
                }
            }
        }
    }
    let row = |t: usize| t * 160;
    let fine = scroll.fine as usize;
    let n = BAND_LINES as usize;
    // Block T: bitmap lines fine - 24 .. (tall fine ..); block B: fine + 192 .. (tall fine + 216 ..).
    let b0 = fine + BAND + VISIBLE as usize;
    Expected {
        top: codes[row(fine)..row(fine + n)].to_vec(),
        bottom: codes[row(b0)..row(b0 + n)].to_vec(),
        codes: codes[row(BAND)..row(BAND + HEIGHT as usize)].to_vec(),
        screen: vec![pano.palette.screen; 1000],
        color: vec![pano.palette.color; 1000],
    }
}
