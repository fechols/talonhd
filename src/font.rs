//! The cards' lettering (host): a condensed 3×7 upper-case font (art director's own), baked into
//! software sprite frames: the stage-clear and stage cards' lines (`clear::CARD_TEXT`) and the tally's
//! digits. A font pixel is one multicolor pixel (two hires wide).

use crate::demo::{CARD_W, DIGIT_H, DIGIT_W, LINE_H};
use crate::encode::SpriteImage;

const VOID: u8 = 0;
const DARK: u8 = 1;
const LIGHT: u8 = 2;

/// A 3×7 glyph (one row per byte, bit 2 = left pixel), `None` if the font has none; space is blank.
/// Three multicolor pixels are six hires ones: on screen this is a 6×7 font, the proportions of an
/// ordinary character set (TALON's 5×7 glyphs at two hires pixels each were twice too fat).
pub fn glyph(c: char) -> Option<[u8; 7]> {
    Some(match c {
        ' ' => [0; 7],
        'A' => [0b010, 0b101, 0b101, 0b111, 0b101, 0b101, 0b101],
        'B' => [0b110, 0b101, 0b101, 0b110, 0b101, 0b101, 0b110],
        'C' => [0b011, 0b100, 0b100, 0b100, 0b100, 0b100, 0b011],
        'D' => [0b110, 0b101, 0b101, 0b101, 0b101, 0b101, 0b110],
        'E' => [0b111, 0b100, 0b100, 0b110, 0b100, 0b100, 0b111],
        'F' => [0b111, 0b100, 0b100, 0b110, 0b100, 0b100, 0b100],
        'G' => [0b011, 0b100, 0b100, 0b101, 0b101, 0b101, 0b011],
        'H' => [0b101, 0b101, 0b101, 0b111, 0b101, 0b101, 0b101],
        'I' => [0b111, 0b010, 0b010, 0b010, 0b010, 0b010, 0b111],
        'J' => [0b001, 0b001, 0b001, 0b001, 0b001, 0b101, 0b010],
        'K' => [0b101, 0b101, 0b110, 0b100, 0b110, 0b101, 0b101],
        'L' => [0b100, 0b100, 0b100, 0b100, 0b100, 0b100, 0b111],
        'M' => [0b101, 0b111, 0b111, 0b101, 0b101, 0b101, 0b101],
        'N' => [0b101, 0b101, 0b111, 0b111, 0b111, 0b101, 0b101],
        'O' => [0b010, 0b101, 0b101, 0b101, 0b101, 0b101, 0b010],
        'P' => [0b110, 0b101, 0b101, 0b110, 0b100, 0b100, 0b100],
        'Q' => [0b010, 0b101, 0b101, 0b101, 0b101, 0b110, 0b011],
        'R' => [0b110, 0b101, 0b101, 0b110, 0b101, 0b101, 0b101],
        'S' => [0b011, 0b100, 0b100, 0b010, 0b001, 0b001, 0b110],
        'T' => [0b111, 0b010, 0b010, 0b010, 0b010, 0b010, 0b010],
        'U' => [0b101, 0b101, 0b101, 0b101, 0b101, 0b101, 0b111],
        'V' => [0b101, 0b101, 0b101, 0b101, 0b101, 0b010, 0b010],
        'W' => [0b101, 0b101, 0b101, 0b101, 0b111, 0b111, 0b101],
        'X' => [0b101, 0b101, 0b101, 0b010, 0b101, 0b101, 0b101],
        'Y' => [0b101, 0b101, 0b101, 0b010, 0b010, 0b010, 0b010],
        'Z' => [0b111, 0b001, 0b001, 0b010, 0b100, 0b100, 0b111],
        '0' => [0b111, 0b101, 0b101, 0b101, 0b101, 0b101, 0b111],
        '1' => [0b010, 0b110, 0b010, 0b010, 0b010, 0b010, 0b111],
        '2' => [0b110, 0b001, 0b001, 0b010, 0b100, 0b100, 0b111],
        '3' => [0b110, 0b001, 0b001, 0b010, 0b001, 0b001, 0b110],
        '4' => [0b101, 0b101, 0b101, 0b111, 0b001, 0b001, 0b001],
        '5' => [0b111, 0b100, 0b100, 0b110, 0b001, 0b001, 0b110],
        '6' => [0b011, 0b100, 0b100, 0b110, 0b101, 0b101, 0b010],
        '7' => [0b111, 0b001, 0b001, 0b010, 0b010, 0b010, 0b010],
        '8' => [0b010, 0b101, 0b101, 0b010, 0b101, 0b101, 0b010],
        '9' => [0b010, 0b101, 0b101, 0b011, 0b001, 0b001, 0b110],
        '.' => [0b000, 0b000, 0b000, 0b000, 0b000, 0b000, 0b010],
        ',' => [0b000, 0b000, 0b000, 0b000, 0b000, 0b010, 0b100],
        ':' => [0b000, 0b010, 0b000, 0b000, 0b000, 0b010, 0b000],
        '\'' => [0b010, 0b010, 0b000, 0b000, 0b000, 0b000, 0b000],
        '!' => [0b010, 0b010, 0b010, 0b010, 0b010, 0b000, 0b010],
        '?' => [0b110, 0b001, 0b001, 0b010, 0b010, 0b000, 0b010],
        '-' => [0b000, 0b000, 0b000, 0b111, 0b000, 0b000, 0b000],
        _ => return None,
    })
}

/// Character pitch (3 columns and a gap) and the widest line the cards take (19 characters: inside the
/// HUD's ladder columns).
pub const PITCH: u16 = 4;
pub const MAX_TEXT: u16 = 19 * PITCH - 1;

/// Columns a line of text takes, less the last gap.
pub fn width(text: &str) -> u16 {
    (text.chars().count() as u16 * PITCH).saturating_sub(1)
}

/// Draw `text` with its top-left at (`x`, `y`) in `color`, a `dark` drop shadow down and right where it
/// falls on the plate.
fn draw(img: &mut SpriteImage, text: &str, x: u16, y: u16, color: u8, shadow: bool) {
    for (k, c) in text.chars().enumerate() {
        let g = glyph(c).unwrap_or_else(|| panic!("no glyph for {c:?} in {text:?}"));
        for (r, bits) in g.iter().enumerate() {
            for col in 0..3u16 {
                if bits & (4 >> col) != 0 {
                    let (px, py) = (x + k as u16 * PITCH + col, y + r as u16);
                    img.set(px, py, Some(color));
                    if shadow && px + 1 < img.w && py + 1 < img.h && img.get(px + 1, py + 1) == Some(VOID) {
                        img.set(px + 1, py + 1, Some(DARK));
                    }
                }
            }
        }
    }
    // A shadow pixel may have landed where a later glyph pixel belongs: glyphs win.
    if shadow {
        for (k, c) in text.chars().enumerate() {
            for (r, bits) in glyph(c).unwrap().iter().enumerate() {
                for col in 0..3u16 {
                    if bits & (4 >> col) != 0 {
                        img.set(x + k as u16 * PITCH + col, y + r as u16, Some(color));
                    }
                }
            }
        }
    }
}

/// One card line: `LINE_H` lines of `void` plate (its sides run under the HUD's ladder columns), the
/// text centred (a leading `<`: left-aligned, the
/// tally's labels) in `light` with a `dark` shadow (a leading `~`: in `dark`, no shadow). Stored
/// unshifted (drawn at multicolor x multiple of 4).
pub fn card_line(text: &str) -> SpriteImage {
    let mut img = SpriteImage::new(CARD_W, LINE_H);
    img.phases = 1;
    for p in img.pixels.iter_mut() {
        *p = Some(VOID);
    }
    let (text, color, left) = match text.chars().next() {
        Some('~') => (&text[1..], DARK, false),
        Some('<') => (&text[1..], LIGHT, true),
        _ => (text, LIGHT, false),
    };
    let w = width(text);
    assert!(w <= MAX_TEXT, "card line too long: {text:?}");
    let x = if left { crate::clear::TALLY_LEFT } else { (CARD_W - w) / 2 };
    draw(&mut img, text, x, 1, color, color == LIGHT);
    img
}

/// The rule above and below a card.
pub fn edge() -> SpriteImage {
    let mut img = SpriteImage::new(CARD_W, 1);
    img.phases = 1;
    for p in img.pixels.iter_mut() {
        *p = Some(DARK);
    }
    img
}

/// The tally's digits 0-9, then a comma: `light`, transparent around the glyph (drawn over a plate).
pub fn digits() -> Vec<SpriteImage> {
    "0123456789,"
        .chars()
        .map(|c| {
            let mut img = SpriteImage::new(DIGIT_W, DIGIT_H);
            draw(&mut img, &c.to_string(), 0, 0, LIGHT, false);
            img
        })
        .collect()
}

/// Every card frame of level `level`'s image, in `demo` order from `CARD_FRAME`: the lines, the edge,
/// the digits.
pub fn card_frames(level: u8) -> Vec<SpriteImage> {
    let mut out: Vec<SpriteImage> = crate::clear::CARD_TEXT[level as usize].iter().map(|t| card_line(t)).collect();
    out.push(edge());
    out.extend(digits());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every card line fits its plate and every character has a glyph; the cards fit the playfield.
    #[test]
    fn the_card_fits() {
        for t in crate::clear::CARD_TEXT.iter().flatten() {
            let s = t.trim_start_matches(['~', '<']);
            assert!(s.chars().all(|c| glyph(c).is_some()), "{t:?}");
            assert!(width(s) <= MAX_TEXT, "{t:?} is {} px", width(s));
        }
        for lines in [crate::clear::CLEAR_LINES, crate::clear::STAGE_LINES] {
            assert!(lines * LINE_H + 2 <= 176, "{lines} lines");
        }
        // The plate sits inside the screen (160 multicolor px) at a 4-px boundary, centred on the
        // playfield (hires 156), its text inside the HUD's ladders (hires 80..240).
        assert_eq!(crate::clear::CARD_X % 4, 0);
        assert!(crate::clear::CARD_X as u16 + CARD_W <= 160);
        assert_eq!(2 * crate::clear::CARD_X as u16 + CARD_W, 156);
        let x0 = 2 * (crate::clear::CARD_X as u16 + (CARD_W - MAX_TEXT) / 2);
        assert!(x0 >= 80 && x0 + 2 * MAX_TEXT <= 240, "{x0}");
        // The longest tally label and a five-digit value with its comma do not touch.
        let label = crate::clear::TALLY_LEFT + width("CLEAN AIRFRAME");
        assert!(label + 2 <= crate::clear::TALLY_RIGHT - width("10,000"));
        let f = card_frames(1);
        assert_eq!(f.len() as u16, crate::demo::CARD_LINES + 1 + crate::demo::DIGITS);
    }
}
