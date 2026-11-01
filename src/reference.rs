//! Naive per-pixel compositor (host only): the specification the DMA
//! renderer must match byte for byte. Vendored from gclass (`a5775bf`),
//! adapted to the vertical panorama and four global colors.

use crate::bake::Panorama;
use crate::encode::SpriteImage;
use crate::render::{Scroll, Sprite};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expected {
    /// 160×200 multicolor codes of the bitmap (not just the visible lines).
    pub codes: Vec<u8>,
    pub screen: Vec<u8>,
    pub color: Vec<u8>,
}

pub fn compose(pano: &Panorama, frames: &[SpriteImage], scroll: Scroll, sprites: &[Sprite]) -> Expected {
    let top = scroll.coarse as usize * 8;
    let mut codes = pano.codes[top * 160..(top + 200) * 160].to_vec();
    let mut order: Vec<Sprite> = sprites.to_vec();
    order.sort_by_key(|s| s.z);
    for s in &order {
        let Some(img) = frames.get(s.frame as usize) else { continue };
        let sx0 = if img.phases == 1 { s.x as i32 & !3 } else { s.x as i32 };
        let sy0 = s.y as i32 + scroll.fine as i32;
        for py in 0..img.h {
            let y = sy0 + py as i32;
            if !(0..200).contains(&y) {
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
    Expected { codes, screen: vec![pano.palette.screen; 1000], color: vec![pano.palette.color; 1000] }
}
