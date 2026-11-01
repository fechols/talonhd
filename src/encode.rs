//! Sprite encoder (host only): indexed sprite → four (or one) phase
//! encodings of column-major pixel bytes and segment lists. Vendored from
//! gclass (`a5775bf`).

use crate::format::{MAX_SPRITE_SIZE, MIN_SKIP, SEG_MIX, SEG_SKIP};

/// A sprite frame in multicolor codes. `None` is transparent; `Some(code)`
/// is an opaque pixel in any of the four global codes.
#[derive(Clone, Debug)]
pub struct SpriteImage {
    pub w: u16,
    pub h: u16,
    pub pixels: Vec<Option<u8>>,
    /// Stored phases: 4 (drawn at any x) or 1 (drawn at `x & !3`).
    pub phases: u8,
}

impl SpriteImage {
    pub fn new(w: u16, h: u16) -> Self {
        assert!((1..=MAX_SPRITE_SIZE).contains(&w) && (1..=MAX_SPRITE_SIZE).contains(&h));
        SpriteImage { w, h, pixels: vec![None; w as usize * h as usize], phases: 4 }
    }
    pub fn get(&self, x: u16, y: u16) -> Option<u8> {
        self.pixels[y as usize * self.w as usize + x as usize]
    }
    pub fn set(&mut self, x: u16, y: u16, v: Option<u8>) {
        let w = self.w as usize;
        self.pixels[y as usize * w + x as usize] = v;
    }
}

#[derive(Clone, Debug, Default)]
pub struct Column {
    /// The run bytes the REU stores, packed: from the first run's line, with
    /// every Skip-ped gap left out.
    pub data: Vec<u8>,
    /// Line of `data[0]` (the column's REU address is biased by it).
    pub first: u16,
    pub segments: Vec<[u8; 4]>,
}

#[derive(Clone, Debug)]
pub struct Phase {
    pub wb: u16,
    pub columns: Vec<Column>,
}

/// Bit position of pixel `k` (0 = leftmost) within a multicolor byte.
#[inline]
pub fn pixel_shift(k: u16) -> u16 {
    6 - 2 * (k & 3)
}

pub fn encode_phase(img: &SpriteImage, p: u16) -> Phase {
    assert!(p < 4);
    let wb = (img.w + p).div_ceil(4);
    let h = img.h as usize;
    let mut bits = vec![0u8; wb as usize * h];
    let mut mask = vec![0u8; wb as usize * h];
    for y in 0..img.h {
        for x in 0..img.w {
            if let Some(code) = img.get(x, y) {
                let px = x + p;
                let i = (px / 4) as usize * h + y as usize;
                let sh = pixel_shift(px);
                bits[i] |= (code & 3) << sh;
                mask[i] |= 3 << sh;
            }
        }
    }
    let mut columns = Vec::with_capacity(wb as usize);
    for j in 0..wb as usize {
        let col_bits = &bits[j * h..(j + 1) * h];
        let col_mask = &mask[j * h..(j + 1) * h];
        let mut segments = Vec::new();
        let mut y = 0;
        while y < h {
            let m = col_mask[y];
            if m == 0xFF {
                let start = y;
                while y < h && col_mask[y] == 0xFF {
                    y += 1;
                }
                let len = (y - start) as u16;
                segments.push([start as u8, (start >> 8) as u8, len as u8, (len >> 8) as u8]);
                continue;
            }
            if m != 0 {
                segments.push([y as u8, (y >> 8) as u8 | SEG_MIX, col_bits[y], !m]);
            }
            y += 1;
        }
        let (data, first, segments) = pack(col_bits, segments);
        columns.push(Column { data, first, segments });
    }
    Phase { wb, columns }
}

/// Pack a column: keep only run bytes, inserting a Skip before each run that
/// follows a gap of at least `MIN_SKIP` lines (shorter gaps stay as data).
fn pack(col_bits: &[u8], segments: Vec<[u8; 4]>) -> (Vec<u8>, u16, Vec<[u8; 4]>) {
    let line = |s: &[u8; 4]| s[0] as usize | ((s[1] & 1) as usize) << 8;
    let mut data = Vec::new();
    let mut out = Vec::with_capacity(segments.len());
    let mut first = None;
    let mut end = 0usize; // line after the last stored byte
    for s in segments {
        if s[1] & SEG_MIX != 0 {
            out.push(s);
            continue;
        }
        let (start, len) = (line(&s), s[2] as usize | (s[3] as usize) << 8);
        match first {
            None => first = Some(start),
            Some(_) => {
                let gap = start - end;
                if gap >= MIN_SKIP {
                    out.push([start as u8, (start >> 8) as u8 | SEG_SKIP, gap as u8, (gap >> 8) as u8]);
                } else {
                    data.extend_from_slice(&col_bits[end..start]);
                }
            }
        }
        data.extend_from_slice(&col_bits[start..start + len]);
        end = start + len;
        out.push(s);
    }
    (data, first.unwrap_or(0) as u16, out)
}
