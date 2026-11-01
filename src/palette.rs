//! C64 palette and color matching (host only).

/// The palette impact's tooling uses (C64U-captured).
pub const PALETTE: [[u8; 3]; 16] = [
    [0, 0, 0],
    [247, 247, 247],
    [141, 47, 52],
    [106, 212, 205],
    [152, 53, 164],
    [76, 180, 66],
    [44, 41, 177],
    [239, 239, 93],
    [152, 78, 32],
    [91, 56, 0],
    [209, 103, 109],
    [74, 74, 74],
    [123, 123, 123],
    [159, 239, 147],
    [109, 106, 239],
    [178, 178, 178],
];

/// Perceptually weighted squared distance.
pub fn distance(a: [u8; 3], b: [u8; 3]) -> u32 {
    let dr = a[0] as i32 - b[0] as i32;
    let dg = a[1] as i32 - b[1] as i32;
    let db = a[2] as i32 - b[2] as i32;
    (3 * dr * dr + 4 * dg * dg + 2 * db * db) as u32
}

/// Nearest of the given C64 colors; returns the index into `choices`.
pub fn nearest_of(rgb: [u8; 3], choices: &[u8]) -> usize {
    let mut best = 0;
    let mut best_d = u32::MAX;
    for (i, &c) in choices.iter().enumerate() {
        let d = distance(rgb, PALETTE[c as usize]);
        if d < best_d {
            best_d = d;
            best = i;
        }
    }
    best
}

pub fn nearest(rgb: [u8; 3]) -> u8 {
    const ALL: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    nearest_of(rgb, &ALL) as u8
}

/// The 3 C64 colors minimizing the total error over a weighted RGB
/// histogram. Exhaustive over all C(16,3) sets.
pub fn best_set3(hist: &[([u8; 3], u64)]) -> [u8; 3] {
    let dist: Vec<[u32; 16]> = hist
        .iter()
        .map(|(rgb, _)| core::array::from_fn(|c| distance(*rgb, PALETTE[c])))
        .collect();
    let mut best = ([0, 1, 2], u128::MAX);
    for a in 0..16 {
        for b in a + 1..16 {
            for c in b + 1..16 {
                let mut err = 0u128;
                for (i, (_, w)) in hist.iter().enumerate() {
                    let d = &dist[i];
                    err += d[a].min(d[b]).min(d[c]) as u128 * *w as u128;
                }
                if err < best.1 {
                    best = ([a as u8, b as u8, c as u8], err);
                }
            }
        }
    }
    best.0
}
