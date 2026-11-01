//! The helmet HUD: 8 hires single-color hardware sprites, 4 down the left
//! edge and 4 down the right, multiplexed into 8 rows that cover the whole
//! visible height. Green line art over the playfield.
//!
//! Rows are 21 lines of sprite plus a 3-line gap (8 × 24 = 192 = the visible
//! lines in 24-row mode). In each gap a raster IRQ (`target-c64/asm/hud.S`)
//! writes the next row's 8 Y positions and 8 pointers from [`Table`]; the gap
//! is the IRQ's timing slack (~190 µs), which is why the REU DMA is capped
//! ([`crate::format::MAX_DMA`]).
//!
//! Hardware sprites have their own color registers, so the HUD stays green on
//! every stage and never touches the four-color bitmap palette.
//!
//! Shared by host and C64: the table, the colors and the dynamic blocks
//! (score, lives, gauge, speed, stage) are computed here; the static art
//! (labels, tapes, brackets, ladder) is baked by the host into the REU.

/// Multiplexed rows.
pub const ROWS: usize = 8;
pub const ROW_PITCH: u8 = 24;
/// Sprite Y of row 0: the first visible raster line (55) is sprite Y 54.
pub const FIRST_Y: u8 = 54;
/// Raster line of the IRQ that sets up row 0 (top border).
pub const TOP_IRQ_LINE: u8 = 20;
/// Sprite X (VIC coordinates; 24 = left screen edge) per channel.
pub const X: [u16; 8] = [24, 48, 72, 96, 248, 272, 296, 320];

/// Colors: frame elements and live readouts.
pub const GREEN: u8 = 5;
pub const LIGHT_GREEN: u8 = 13;
pub const WHITE: u8 = 1;
pub const RED: u8 = 2;
/// Damage flash: frames of white, then frames of red (from the hit).
pub const FLASH_WHITE: u8 = 2;
pub const FLASH_RED: u8 = 6;
pub const FLASH_FRAMES: u8 = FLASH_WHITE + FLASH_RED;

pub const BLOCK: usize = 64;
pub const BLOCKS: usize = crate::format::HUD_BLOCKS as usize;

/// Block ids. Static blocks are baked by the host; readouts with few states
/// (lives, gauge, speed) are pre-drawn variants chosen by pointer; only the
/// score and stage digits are written at runtime.
pub mod block {
    pub const BLANK: u8 = 0;
    pub const BRACKET_TL: u8 = 1;
    pub const BRACKET_BL: u8 = 2;
    pub const BRACKET_TR: u8 = 3;
    pub const BRACKET_BR: u8 = 4;
    /// Four scroll phases each.
    pub const TAPE_L: u8 = 5;
    pub const TAPE_R: u8 = 9;
    /// Five tilts each (bank -2..=2).
    pub const LADDER_L: u8 = 13;
    pub const LADDER_R: u8 = 18;
    pub const LABEL_PTS: u8 = 23;
    pub const LABEL_LIFE: u8 = 24;
    pub const LABEL_ARM: u8 = 25;
    pub const LABEL_SPD: u8 = 26;
    pub const LABEL_STG: u8 = 27;
    /// 0..=3 lives.
    pub const LIVES: u8 = 28;
    /// A pair of gauge slots: neither, upper or lower highlighted.
    pub const GAUGE: u8 = 32;
    /// Speed levels 0..=5.
    pub const SPEED: u8 = 35;
    /// Runtime-written blocks: the score (2 sprites, 3 digits each), the stage digit.
    pub const SCORE: u8 = 41;
    pub const STAGE: u8 = 43;
    pub const DYNAMIC: u8 = 41;
    pub const DYNAMIC_COUNT: usize = 3;
}

/// What the HUD shows this frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct State {
    pub score: u32,
    pub lives: u8,
    /// Arming gauge: highlighted slot 0..=7, or 8 for none.
    pub gauge: u8,
    /// Speed level 0..=5.
    pub speed: u8,
    pub stage: u8,
    /// Bank of the player's ship, -4 (full left) ..= 4 (full right).
    pub bank: i8,
    /// Tape positions (any units; one line per unit).
    pub tape_l: u16,
    pub tape_r: u16,
    /// Frames since the last hit (>= FLASH_FRAMES: no flash).
    pub since_hit: u8,
}

/// The raster IRQ's table at [`crate::layout::HUD_TABLE`]:
/// `+0` the next row index (IRQ state), `+1` the IRQ raster line that sets up
/// each row, `+9` each row's sprite Y, `+17` each row's 8 pointers (block ids
/// = bank-2 pointers; the IRQ ORs in [`crate::layout::HUD_BANK1_POINTER`] for
/// bank 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Table {
    pub lines: [u8; ROWS],
    pub ys: [u8; ROWS],
    pub ptrs: [[u8; 8]; ROWS],
}

pub mod table {
    pub const ROW: u16 = 0;
    pub const LINES: u16 = 1;
    pub const YS: u16 = 9;
    pub const PTRS: u16 = 17;
    pub const LEN: u16 = 17 + 64;
}

pub const fn row_y(r: usize) -> u8 {
    FIRST_Y + r as u8 * ROW_PITCH
}

/// Which block each channel shows on each row.
pub fn build(s: &State) -> Table {
    use block::*;
    let mut ptrs = [[BLANK; 8]; ROWS];
    let tl = TAPE_L + ((s.tape_l / 3) % 4) as u8;
    let tr = TAPE_R + ((s.tape_r / 3) % 4) as u8;
    let tilt = (s.bank.clamp(-4, 4) + 4) as u8 / 2; // 0..=4
    for (r, row) in ptrs.iter_mut().enumerate() {
        row[0] = tl;
        row[7] = tr;
        if r == 3 || r == 4 {
            row[3] = LADDER_L + tilt;
            row[4] = LADDER_R + tilt;
        }
    }
    ptrs[0][3] = BRACKET_TL;
    ptrs[7][3] = BRACKET_BL;
    ptrs[0][4] = BRACKET_TR;
    ptrs[7][4] = BRACKET_BR;
    ptrs[0][1] = LABEL_PTS;
    ptrs[1][1] = SCORE;
    ptrs[1][2] = SCORE + 1;
    ptrs[6][1] = LABEL_SPD;
    ptrs[7][1] = SPEED + s.speed.min(5);
    ptrs[0][5] = LABEL_LIFE;
    ptrs[1][5] = LIVES + s.lives.min(3);
    ptrs[2][5] = LABEL_ARM;
    for k in 0..4u8 {
        // Slots 2k (upper) and 2k+1 (lower) of the 8-slot gauge.
        ptrs[3 + k as usize][5] = GAUGE + if s.gauge == 2 * k { 1 } else if s.gauge == 2 * k + 1 { 2 } else { 0 };
    }
    ptrs[7][5] = LABEL_STG;
    ptrs[7][6] = STAGE;
    let mut lines = [0u8; ROWS];
    let mut ys = [0u8; ROWS];
    for r in 0..ROWS {
        ys[r] = row_y(r);
        lines[r] = if r == 0 { TOP_IRQ_LINE } else { row_y(r) - 3 };
    }
    Table { lines, ys, ptrs }
}

/// Per-channel colors, with the damage flash (white, then red, then green).
pub fn colors(s: &State) -> [u8; 8] {
    if s.since_hit < FLASH_WHITE {
        return [WHITE; 8];
    }
    if s.since_hit < FLASH_FRAMES {
        return [RED; 8];
    }
    [GREEN, LIGHT_GREEN, LIGHT_GREEN, GREEN, GREEN, LIGHT_GREEN, LIGHT_GREEN, GREEN]
}

// --------------------------------------------------------------------- drawing

/// 5×7 glyphs, one byte per row (bit 4 = leftmost).
pub const fn glyph(c: u8) -> [u8; 7] {
    match c {
        b'0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        b'1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
        b'2' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
        b'3' => [0x1F, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0E],
        b'4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        b'5' => [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
        b'6' => [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        b'7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        b'8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        b'9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
        b'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        b'D' => [0x1E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1E],
        b'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        b'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10],
        b'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0F],
        b'I' => [0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E],
        b'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        b'M' => [0x11, 0x1B, 0x15, 0x15, 0x11, 0x11, 0x11],
        b'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10],
        b'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11],
        b'S' => [0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E],
        b'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        _ => [0; 7],
    }
}

#[inline]
pub fn set(b: &mut [u8; BLOCK], x: i32, y: i32) {
    if (0..24).contains(&x) && (0..21).contains(&y) {
        b[(y * 3 + x / 8) as usize] |= 0x80 >> (x & 7);
    }
}

pub fn get(b: &[u8; BLOCK], x: i32, y: i32) -> bool {
    (0..24).contains(&x) && (0..21).contains(&y) && b[(y * 3 + x / 8) as usize] & (0x80 >> (x & 7)) != 0
}

/// Draw `text` with 6-pixel pitch from (`x`, `y`).
pub fn text(b: &mut [u8; BLOCK], x: i32, y: i32, text: &[u8]) {
    for (i, &c) in text.iter().enumerate() {
        let g = glyph(c);
        for (row, bits) in g.iter().enumerate() {
            for col in 0..5 {
                if bits & (0x10 >> col) != 0 {
                    set(b, x + i as i32 * 6 + col, y + row as i32);
                }
            }
        }
    }
}

fn hline(b: &mut [u8; BLOCK], x0: i32, x1: i32, y: i32) {
    for x in x0..=x1 {
        set(b, x, y);
    }
}

fn vline(b: &mut [u8; BLOCK], x: i32, y0: i32, y1: i32) {
    for y in y0..=y1 {
        set(b, x, y);
    }
}

fn rect(b: &mut [u8; BLOCK], x0: i32, y0: i32, x1: i32, y1: i32, fill: bool) {
    for y in y0..=y1 {
        for x in x0..=x1 {
            if fill || x == x0 || x == x1 || y == y0 || y == y1 {
                set(b, x, y);
            }
        }
    }
}

/// Digit glyphs, built at compile time, pre-shifted into a byte (5 pixels
/// in bits 7..3): digits sit at an 8-pixel pitch, so drawing one is 7 byte
/// stores.
const DIGITS: [[u8; 7]; 10] = {
    let mut t = [[0u8; 7]; 10];
    let mut i = 0;
    while i < 10 {
        let g = glyph(b'0' + i as u8);
        let mut r = 0;
        while r < 7 {
            t[i][r] = g[r] << 3;
            r += 1;
        }
        i += 1;
    }
    t
};
const POW10: [u32; 6] = [100_000, 10_000, 1_000, 100, 10, 1];
/// Score digits shown (the score is capped at 999,999).
pub const SCORE_DIGITS: usize = 6;

#[inline(always)]
fn put_digit(b: &mut [u8; BLOCK], column: usize, y: usize, d: usize) {
    for (row, &bits) in DIGITS[d].iter().enumerate() {
        b[(y + row) * 3 + column] = bits;
    }
}

/// The runtime-written blocks, in order from [`block::DYNAMIC`]: the score
/// (6 digits, 3 per sprite, one byte column each) and the stage digit.
/// Written for the 6502: byte stores from a compile-time table, digits by
/// subtraction (no division).
#[inline(never)]
pub fn dynamic_blocks(s: &State, out: &mut [[u8; BLOCK]; block::DYNAMIC_COUNT]) {
    for b in out.iter_mut() {
        *b = [0; BLOCK];
    }
    let mut v = if s.score > 999_999 { 999_999 } else { s.score };
    for (i, &p) in POW10.iter().enumerate() {
        let mut d = 0;
        while v >= p {
            v -= p;
            d += 1;
        }
        put_digit(&mut out[i / 3], i % 3, 7, d);
    }
    put_digit(&mut out[2], 0, 7, (s.stage % 10) as usize);
}

/// The static blocks (host bakes them into the REU; the C64 copies them into
/// both VIC banks at boot). Dynamic slots are left blank.
pub fn static_blocks() -> [[u8; BLOCK]; BLOCKS] {
    use block::*;
    let mut b = [[0u8; BLOCK]; BLOCKS];
    // View-frame brackets on the inner columns (the playfield side).
    {
        let k = &mut b[BRACKET_TL as usize];
        hline(k, 8, 23, 4);
        vline(k, 8, 4, 18);
    }
    {
        let k = &mut b[BRACKET_BL as usize];
        hline(k, 8, 23, 16);
        vline(k, 8, 2, 16);
    }
    {
        let k = &mut b[BRACKET_TR as usize];
        hline(k, 0, 15, 4);
        vline(k, 15, 4, 18);
    }
    {
        let k = &mut b[BRACKET_BR as usize];
        hline(k, 0, 15, 16);
        vline(k, 15, 2, 16);
    }
    // Tapes: a rail with minor ticks every 6 lines and major ticks every 12,
    // in 4 phases of 3 lines. Rows are 24 lines apart, so the pattern runs on
    // across the gaps.
    for p in 0..4 {
        let (mut l, mut r) = ([0u8; BLOCK], [0u8; BLOCK]);
        vline(&mut l, 3, 0, 20);
        vline(&mut r, 20, 0, 20);
        for y in 0..21 {
            let pos = (y + p as i32 * 3) % 12;
            if pos == 0 {
                hline(&mut l, 3, 13, y);
                hline(&mut r, 10, 20, y);
            } else if pos == 6 {
                hline(&mut l, 3, 8, y);
                hline(&mut r, 15, 20, y);
            }
        }
        b[(TAPE_L + p) as usize] = l;
        b[(TAPE_R + p) as usize] = r;
    }
    // Pitch ladder: a dashed horizon bar with an end tick, tilted with the bank.
    for t in 0..5i32 {
        let slope = t - 2; // -2..=2 pixels per 12 across
        let l = &mut b[(LADDER_L + t as u8) as usize];
        for x in 4..22 {
            if (x / 3) % 2 == 0 {
                set(l, x, 10 + slope * (x - 13) / 6);
            }
        }
        vline(l, 4, 10 - slope * 9 / 6, 13 - slope * 9 / 6);
        let r = &mut b[(LADDER_R + t as u8) as usize];
        for x in 2..20 {
            if (x / 3) % 2 == 0 {
                set(r, x, 10 - slope * (x - 11) / 6);
            }
        }
        vline(r, 19, 10 - slope * 8 / 6, 13 - slope * 8 / 6);
    }
    text(&mut b[LABEL_PTS as usize], 0, 12, b"PTS");
    text(&mut b[LABEL_LIFE as usize], 0, 12, b"LIFE");
    text(&mut b[LABEL_ARM as usize], 0, 12, b"ARM");
    text(&mut b[LABEL_SPD as usize], 0, 12, b"SPD");
    text(&mut b[LABEL_STG as usize], 0, 7, b"STG");
    // Lives 0..=3: small arrowheads 8 pixels apart.
    for n in 0..4u8 {
        let k = &mut b[(LIVES + n) as usize];
        for i in 0..n as i32 {
            let c = 5 + i * 8;
            for r in 0..6 {
                hline(k, c - r / 2, c + r / 2, 7 + r);
            }
        }
    }
    // Gauge slot pairs: neither, upper or lower highlighted.
    for v in 0..3u8 {
        let k = &mut b[(GAUGE + v) as usize];
        rect(k, 4, 1, 19, 8, v == 1);
        rect(k, 4, 11, 19, 18, v == 2);
    }
    // Speed 0..=5: six segments, lit up to the level.
    for lvl in 0..6i32 {
        let k = &mut b[(SPEED + lvl as u8) as usize];
        for seg in 0..6 {
            rect(k, seg * 4, 8, seg * 4 + 2, 14, seg <= lvl);
        }
    }
    // Underline the labels (a helmet-display touch).
    for id in [LABEL_PTS, LABEL_LIFE, LABEL_ARM, LABEL_SPD] {
        hline(&mut b[id as usize], 0, 22, 20);
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_fill_the_visible_lines_with_gaps() {
        let t = build(&State::default());
        assert_eq!(t.ys[0], 54);
        assert_eq!(t.ys[7] as u16 + 21, 54 + 189);
        for r in 1..ROWS {
            // The IRQ for row r lands after row r-1's last line and before row r.
            assert!(t.lines[r] > t.ys[r - 1] + 20 && t.lines[r] < t.ys[r]);
        }
        assert!(t.ptrs.iter().flatten().all(|&p| (p as usize) < BLOCKS));
    }

    #[test]
    fn flash_goes_white_red_green() {
        let at = |f| colors(&State { since_hit: f, ..State::default() })[0];
        assert_eq!(at(0), WHITE);
        assert_eq!(at(1), WHITE);
        assert_eq!(at(2), RED);
        assert_eq!(at(7), RED);
        assert_eq!(at(8), GREEN);
    }

    #[test]
    fn score_digits_render() {
        let mut out = [[0u8; BLOCK]; block::DYNAMIC_COUNT];
        dynamic_blocks(&State { score: 123_456, lives: 3, gauge: 2, speed: 5, stage: 1, ..State::default() }, &mut out);
        assert!(out.iter().all(|b| b.iter().any(|&v| v != 0)));
        // "1" in the first digit cell: its stem at x = 2.
        assert!(get(&out[0], 2, 8));
        // "4" starts the second sprite: its diagonal top at x = 3.
        assert!(get(&out[1], 3, 7));
        let t = build(&State { lives: 2, gauge: 5, speed: 3, ..State::default() });
        assert_eq!(t.ptrs[1][5], block::LIVES + 2);
        assert_eq!(t.ptrs[5][5], block::GAUGE + 2); // slot 5 = row 3+2, lower
        assert_eq!(t.ptrs[3][5], block::GAUGE);
        assert_eq!(t.ptrs[7][1], block::SPEED + 3);
    }
}
