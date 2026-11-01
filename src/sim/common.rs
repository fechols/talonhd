//! Helpers that both halves of a tick (and the scene builder) call. On the C64 this module stays resident
//! (target-c64/link.ld matches no phase pattern for it), so either phase may reach it.

/// Reject-first collision bands (`docs/design.md` 13): bit `b` covers playfield lines `16 b - 32 ..
/// 16 b - 17`, so the 16 bands span y -32..223 (objects entering from above included). An object's mask
/// has the bits of every band its box `[top, top + h)` touches; two objects can only overlap when their
/// masks intersect, which is tested before any coordinate.
#[inline(never)]
pub fn band_mask(top: i16, h: i16) -> u16 {
    let first = band(top);
    let last = band(top + h - 1);
    // Bits first..=last (tables: variable shifts are slow and large on the 6502).
    FROM[first as usize] & UPTO[last as usize]
}

const fn masks(from: bool) -> [u16; 16] {
    let mut out = [0u16; 16];
    let mut i = 0;
    while i < 16 {
        out[i] = if from { 0xffff << i } else { 0xffff >> (15 - i) };
        i += 1;
    }
    out
}
static FROM: [u16; 16] = masks(true);
static UPTO: [u16; 16] = masks(false);

#[inline(always)]
fn band(y: i16) -> u8 {
    let b = (y + 32) >> 4;
    if b < 0 { 0 } else if b > 15 { 15 } else { b as u8 }
}

/// `|a - b| < limit` for half-extents (centre distance test of two boxes).
#[inline(always)]
pub fn near(a: i16, b: i16, limit: i16) -> bool {
    let d = a - b;
    d < limit && d > -limit
}

/// The SPARK's horizontal sway at phase `t`: a triangle wave of +-8 px over 64 ticks.
#[inline(never)]
pub fn sway(t: u8) -> i16 {
    let p = (t & 63) as i16;
    if p < 32 { p / 2 - 8 } else { 24 - p / 2 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bands_cover_the_box() {
        assert_eq!(band_mask(-32, 16), 1);
        assert_eq!(band_mask(-32, 17), 3);
        assert_eq!(band_mask(0, 16), 1 << 2);
        assert_eq!(band_mask(10, 10), 0b1100);
        assert_eq!(band_mask(300, 10), 1 << 15);
        assert_eq!(band_mask(-100, 10), 1);
        // Disjoint boxes far apart never share a band.
        assert_eq!(band_mask(0, 10) & band_mask(40, 10), 0);
    }
}
