//! REU image building (host only): panorama + sprite frames → one level's
//! 16 MiB REU image. Allocator and frame layout vendored from gclass
//! (`a5775bf`); the panorama is vertical and all colors are global.

use crate::encode::{self, SpriteImage};
use crate::format::{self, frame, header, phase};
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

/// Build one level's REU image without music. Fails if it doesn't fit 16 MiB.
pub fn build_image(pano: &Panorama, frames: &[SpriteImage], border: u8) -> Result<Vec<u8>, String> {
    build_level(pano, frames, border, &[], &[])
}

/// Build level 0's REU image with its stage music (a DLT5 asset, `crate::audio`; empty = none) and
/// the C64 build's paged code phases (every level's, `paging::NAMES` order, `dist/phases`; empty = none).
pub fn build_level(pano: &Panorama, frames: &[SpriteImage], border: u8, music: &[u8], phases: &[Vec<u8>]) -> Result<Vec<u8>, String> {
    build_animated_level(std::slice::from_ref(pano), 0, frames, border, music, phases, 0)
}

/// [`build_level`] for an animated level: `panos` is its loop of whole panorama frames (a power of two of
/// them, all the same size and palette), each shown `1 << shift` ticks; the image holds `level`
/// (`format::header::LEVEL`), and of the phases the common ones and that level's (`paging::level_slots`).
pub fn build_animated_level(panos: &[Panorama], shift: u8, frames: &[SpriteImage], border: u8, music: &[u8], phases: &[Vec<u8>], level: u8) -> Result<Vec<u8>, String> {
    assert!(frames.len() <= u16::MAX as usize);
    let pano = &panos[0];
    assert!(pano.rows >= 25, "panorama needs at least 25 cell rows");
    assert!(panos.len().is_power_of_two() && panos.len() <= 256, "panorama frames must be a power of two");
    assert!(panos.iter().all(|p| p.rows == pano.rows && p.palette == pano.palette), "panorama frames differ in size or palette");
    assert!(((panos.len() - 1) << shift) <= 255, "the animation loop must fit the tick's low byte");
    let mut a = Alloc { image: vec![0; format::HEADER_LEN as usize] };
    a.image[..8].copy_from_slice(&format::MAGIC);

    // The frame table first (at `format::BG_TABLE`), then the frames. The window DMA (`hw::fetch`)
    // splits at banks, so a panorama may span them.
    let table = a.reserve(4 * panos.len())?;
    debug_assert_eq!(table, format::BG_TABLE);
    for (k, p) in panos.iter().enumerate() {
        let at = a.place(&p.bitmap_rows())?;
        a.put_u32(table + 4 * k as u32, at);
    }
    let bg_addr = a.image[table as usize..table as usize + 4].try_into().map(u32::from_le_bytes).unwrap();
    a.image[header::BG_MASK] = (panos.len() - 1) as u8;
    a.image[header::BG_SHIFT] = shift;

    let hud: Vec<u8> = crate::hud::static_blocks().iter().flatten().copied().collect();
    let hud_addr = a.place(&hud)?;

    let dir = a.reserve(frames.len() * format::FRAME_RECORD as usize)?;
    for (i, img) in frames.iter().enumerate() {
        assert!(img.phases == 1 || img.phases == 4, "frame {i}: phases must be 1 or 4");
        let rec = dir + i as u32 * format::FRAME_RECORD;
        a.put_u16(rec + frame::W as u32, img.w);
        a.put_u16(rec + frame::H as u32, img.h);
        let (shift, nbands) = encode::bands(img.h);
        a.image[(rec + frame::PHASE_COUNT as u32) as usize] = img.phases;
        a.image[(rec + frame::BAND_SHIFT as u32) as usize] = shift;
        a.image[(rec + frame::BANDS as u32) as usize] = nbands as u8;
        for p in 0..img.phases as u16 {
            let block = encode::encode_block(img, p);
            // The C64 fetches a band's on-screen columns into CHUNK, one or more columns at a time.
            let at = a.place(&block.bytes)?;
            let prec = rec + frame::PHASES as u32 + p as u32 * frame::PHASE_LEN as u32;
            a.image[(prec + phase::WB as u32) as usize] = block.wb as u8;
            a.image[(prec + phase::BLOCK as u32) as usize..(prec + phase::BLOCK as u32 + 3) as usize]
                .copy_from_slice(&at.to_le_bytes()[..3]);
            a.put_u16(prec + phase::LEN as u32, block.bytes.len().min(0xFFFF) as u16);
        }
    }

    let music_addr = if music.is_empty() {
        0
    } else {
        crate::audio::validate(music)?;
        a.place(music)?
    };

    let phase_addr = if phases.is_empty() {
        0
    } else {
        let n = crate::paging::PHASES;
        if phases.len() != n {
            return Err(format!("{} phase images, want {n}", phases.len()));
        }
        let slot = crate::paging::PHASE_SLOT as usize;
        for (id, img) in phases.iter().enumerate() {
            if img.len() > slot {
                return Err(format!("phase {} is {} bytes (> {slot})", crate::paging::NAMES[id], img.len()));
            }
        }
        // The common phases and this level's, each in its slot (`paging`).
        let slots = crate::paging::level_slots(phases, level);
        let mut blob = vec![0u8; slots.len() * slot];
        for (s, img) in slots.iter().enumerate() {
            blob[s * slot..s * slot + img.len()].copy_from_slice(img);
            a.image[header::PHASE_LENS + s] = img.len().div_ceil(256) as u8;
        }
        let refs: Vec<&[u8]> = phases.iter().map(|v| v.as_slice()).collect();
        a.put_u16(header::PHASE_STAMP as u32, crate::paging::stamp(&refs));
        // Page-aligned: the C64 keeps each slot's address as its middle and high bytes.
        let len = a.image.len().next_multiple_of(256);
        a.image.resize(len, 0);
        let at = a.place(&blob)?;
        assert_eq!(at % 256, 0);
        at
    };

    let size = a.image.len() as u32;
    // The attract pilot's scratch, the music ring, BOOT and the band sets sit at the top of the REU
    // (`ace::MIND_REU`, `audio::RING`, `paging`).
    if size > crate::ace::MIND_REU {
        return Err(format!("level image is {size} bytes: it would run into the REU scratch at {:#x}", crate::ace::MIND_REU));
    }
    a.put_u32(header::SIZE as u32, size);
    a.put_u32(header::BACKGROUND as u32, bg_addr);
    a.image[header::LEVEL] = level;
    a.image[header::D021] = pano.palette.d021;
    a.image[header::BORDER] = border;
    a.put_u16(header::FRAMES as u32, frames.len() as u16);
    a.put_u32(header::DIRECTORY as u32, dir);
    a.image[header::COLOR] = pano.palette.color;
    a.image[header::SCREEN] = pano.palette.screen;
    a.put_u16(header::BG_ROWS as u32, pano.rows);
    a.put_u32(header::HUD as u32, hud_addr);
    a.put_u32(header::MUSIC as u32, music_addr);
    a.put_u32(header::PHASES as u32, phase_addr);
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
        n += encode::encode_block(img, p).bytes.len();
    }
    n
}
