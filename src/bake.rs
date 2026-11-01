//! REU image building (host only): panorama + sprite frames → one level's
//! 16 MiB REU image. Allocator and frame layout vendored from gclass
//! (`a5775bf`); the panorama is vertical and all colors are global.

use crate::encode::{self, SpriteImage};
use crate::format::{self, column, frame, header, phase};
use crate::palette::PALETTE;
use crate::render::Palette;

/// Background panorama: 160 multicolor pixels wide, `rows` cell rows tall.
#[derive(Clone, Debug)]
pub struct Panorama {
    pub rows: u16,
    /// 160 × (rows*8) multicolor codes, top to bottom.
    pub codes: Vec<u8>,
    pub palette: Palette,
}

impl Panorama {
    pub fn new(rows: u16, palette: Palette) -> Panorama {
        Panorama { rows, codes: vec![0; 160 * rows as usize * 8], palette }
    }
    pub fn lines(&self) -> usize {
        self.rows as usize * 8
    }
    /// Bitmap cell rows, top to bottom, 320 bytes each in VIC cell order.
    pub fn bitmap_rows(&self) -> Vec<u8> {
        let mut out = vec![0u8; self.rows as usize * 320];
        for y in 0..self.lines() {
            for x in 0..160 {
                let code = self.codes[y * 160 + x];
                out[(y / 8) * 320 + (x / 4) * 8 + (y & 7)] |= code << encode::pixel_shift(x as u16);
            }
        }
        out
    }
}

/// REU allocator: never lets a block straddle a 512 KiB bank, so no DMA
/// inside a block can wrap.
struct Alloc {
    image: Vec<u8>,
}

impl Alloc {
    fn place(&mut self, data: &[u8]) -> Result<u32, String> {
        let mut at = self.image.len() as u32;
        let bank = format::REU_BANK;
        if data.len() as u32 <= bank && (at % bank) + data.len() as u32 > bank {
            at = (at / bank + 1) * bank;
            self.image.resize(at as usize, 0);
        }
        if at as usize + data.len() > format::REU_SIZE as usize {
            return Err(format!("level REU image exceeds 16 MiB (at {at:#x} + {} bytes)", data.len()));
        }
        self.image.extend_from_slice(data);
        Ok(at)
    }
    fn reserve(&mut self, len: usize) -> Result<u32, String> {
        self.place(&vec![0; len])
    }
    fn put_u32(&mut self, at: u32, v: u32) {
        self.image[at as usize..at as usize + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn put_u16(&mut self, at: u32, v: u16) {
        self.image[at as usize..at as usize + 2].copy_from_slice(&v.to_le_bytes());
    }
}

/// Build one level's REU image. Fails if it doesn't fit 16 MiB.
pub fn build_image(pano: &Panorama, frames: &[SpriteImage], border: u8) -> Result<Vec<u8>, String> {
    assert!(frames.len() <= u16::MAX as usize);
    assert!(pano.rows >= 25, "panorama needs at least 25 cell rows");
    let mut a = Alloc { image: vec![0; format::HEADER_LEN as usize] };
    a.image[..8].copy_from_slice(&format::MAGIC);

    // The window DMA (`hw::fetch`) splits at banks, so the panorama may span them.
    let bg_addr = a.place(&pano.bitmap_rows())?;

    let dir = a.reserve(frames.len() * format::FRAME_RECORD as usize)?;
    for (i, img) in frames.iter().enumerate() {
        assert!(img.phases == 1 || img.phases == 4, "frame {i}: phases must be 1 or 4");
        let rec = dir + i as u32 * format::FRAME_RECORD;
        a.put_u16(rec + frame::W as u32, img.w);
        a.put_u16(rec + frame::H as u32, img.h);
        a.image[(rec + frame::PHASE_COUNT as u32) as usize] = img.phases;
        for p in 0..img.phases as u16 {
            let enc = encode::encode_phase(img, p);
            let prec = rec + frame::PHASES as u32 + p as u32 * frame::PHASE_LEN as u32;
            a.image[(prec + phase::WB as u32) as usize] = enc.wb as u8;
            let table = a.reserve(enc.wb as usize * format::COLUMN_ENTRY as usize)?;
            a.put_u32(prec + phase::COLUMNS as u32, table);
            for (j, col) in enc.columns.iter().enumerate() {
                let e = table + j as u32 * format::COLUMN_ENTRY as u32;
                if col.segments.is_empty() {
                    continue;
                }
                let segs: Vec<u8> = col.segments.iter().flatten().copied().collect();
                let seg_addr = a.place(&segs)?;
                // Only the lines covered by runs are ever fetched; store
                // those and bias the address so `data + line` still works.
                let line = |s: &[u8; 4]| s[0] as usize | ((s[1] & 1) as usize) << 8;
                let runs: Vec<&[u8; 4]> = col.segments.iter().filter(|s| s[1] & format::SEG_MIX == 0).collect();
                let data_addr = match runs.iter().map(|s| line(s)).min() {
                    Some(first) => {
                        let end = runs.iter().map(|s| line(s) + (s[2] as usize | (s[3] as usize) << 8)).max().unwrap();
                        a.place(&col.data[first..end])?.wrapping_sub(first as u32)
                    }
                    None => 0,
                };
                let entry = &mut a.image[e as usize..e as usize + 8];
                entry[column::DATA..column::DATA + 3].copy_from_slice(&data_addr.to_le_bytes()[..3]);
                entry[column::SEGS..column::SEGS + 3].copy_from_slice(&seg_addr.to_le_bytes()[..3]);
                entry[column::COUNT..column::COUNT + 2].copy_from_slice(&(col.segments.len() as u16).to_le_bytes());
            }
        }
    }

    let size = a.image.len() as u32;
    a.put_u32(header::SIZE as u32, size);
    a.put_u32(header::BACKGROUND as u32, bg_addr);
    a.image[header::D021] = pano.palette.d021;
    a.image[header::BORDER] = border;
    a.put_u16(header::FRAMES as u32, frames.len() as u16);
    a.put_u32(header::DIRECTORY as u32, dir);
    a.image[header::COLOR] = pano.palette.color;
    a.image[header::SCREEN] = pano.palette.screen;
    a.put_u16(header::BG_ROWS as u32, pano.rows);
    let sum = checksum(&a.image);
    a.put_u32(header::CHECKSUM as u32, sum);
    Ok(a.image)
}

/// Additive checksum of everything after the header.
pub fn checksum(image: &[u8]) -> u32 {
    image[format::HEADER_LEN as usize..].iter().fold(0u32, |s, &b| s.wrapping_add(b as u32))
}

/// Preview a sprite frame on a checkerboard (RGB, one entry per multicolor pixel).
pub fn sprite_preview(img: &SpriteImage, pal: Palette) -> Vec<[u8; 3]> {
    (0..img.h)
        .flat_map(|y| (0..img.w).map(move |x| (x, y)))
        .map(|(x, y)| match img.get(x, y) {
            Some(code) => PALETTE[pal.color_of(code) as usize],
            None if (x / 4 + y / 8) % 2 == 0 => [40, 40, 48],
            None => [64, 64, 72],
        })
        .collect()
}

/// REU bytes one frame takes (column tables, segments, run pixel data),
/// before bank padding.
pub fn frame_bytes(img: &SpriteImage) -> usize {
    let mut n = format::FRAME_RECORD as usize;
    for p in 0..img.phases as u16 {
        let enc = encode::encode_phase(img, p);
        n += enc.wb as usize * format::COLUMN_ENTRY as usize;
        for col in &enc.columns {
            n += col.segments.len() * 4;
            let runs = col.segments.iter().filter(|s| s[1] & format::SEG_MIX == 0);
            let line = |s: &[u8; 4]| s[0] as usize | ((s[1] & 1) as usize) << 8;
            let first = runs.clone().map(line).min();
            if let Some(first) = first {
                let end = runs.map(|s| line(s) + (s[2] as usize | (s[3] as usize) << 8)).max().unwrap();
                n += end - first;
            }
        }
    }
    n
}
