//! Digital music: DLT5 streams (the format and the 6502 decode kernel of `../sueme64`, vendored; see
//! `THIRD-PARTY.md`), 16 kHz mono, 3-bit codes over per-segment codebooks of 8 signed-16 deltas, decoded
//! to 8-bit PCM into one ring in the REU. The C64 Ultimate's sampler plays that ring on both voices: the
//! left from the write side, the right [`DELAY`] samples (10 ms) behind it, panned hard right. One
//! stream decoded, one PCM stream written: the Haas effect gives the width (user, 2026-11-01).
//!
//! - **Producer** ([`service`], once per frame, the `AUDIO` phase): estimates how far the sampler has
//!   played from the CIA 2 timer (calibrated against the sampler at boot), decodes up to
//!   [`RING_SAMPLES`] - [`GUARD`] ahead of it in batches of at most 128 samples into
//!   [`STAGING`] (the renderer's segment buffer, idle between frames) and stashes them into the ring.
//! - **Setup** ([`tune`], the one-time `TUNE` phase): mapping check, timers chained into 32 bits, the sampler's
//!   retained-low-byte clear, then two muted one-shots (4096 and 16384 samples) whose time difference
//!   is the clock: CIA ticks per [`CLOCK_SAMPLES`] samples.
//! - **State** ([`Player`]) lives at [`PLAYER`] (`$C800-$C9FF`, free RAM between the segment buffer and
//!   the job block); the kernel reads its `Kernel` and `cache` there.
//!
//! DLT5 layout (little-endian): 32-byte header (`DLT5`, rate u32 @4, samples u32 @8, stream bytes u32
//! @12, entries 8 @16, books u16 @18, bits 3 @20, channels @21), then `books` × 16-byte codebooks, the
//! LSB-first 3-bit stream (8 codes per 3 bytes), then a CRC-8 per 1024 stream bytes (checked on the
//! host at bake time). With G = ceil(samples / 8) groups, the first G % books segments are
//! (G / books + 1) × 8 samples long and the rest G / books × 8; predictors carry across segments and
//! reset at the loop. Published sample: `(predictor + 128) >> 8`, saturated to signed 8.

use crate::hw::{self, Hw};

/// Player state in C64 RAM (`$C800-$C9FF`).
pub const PLAYER: u16 = 0xC800;
/// Decoded PCM is staged here before each stash (the renderer's segment buffer).
pub const STAGING: u16 = crate::layout::SEG_BUF;
/// The PCM ring in the REU: below the top bank (`paging::REU_BOOT`, the band sets), clear of any level image.
pub const RING: u32 = 0xFE_8000;
pub const RING_SAMPLES: u32 = 32_768;
/// The producer stays this far behind the left voice's next lap (more than [`DELAY`]: the lagging
/// right voice is never overwritten before it plays).
pub const GUARD: u32 = 2048;
/// The right voice's lag: 10 ms at 16 kHz.
pub const DELAY: u16 = 160;
/// Samples decoded per [`service`] at most: a frame needs ~267. The ring leads by ~1.9 s, so a deficit
/// is made up 133 samples a frame instead of all at once (a catch-up of 1024 made the frame after a slow
/// one slow too: 60 FPS is a hard limit). The C64 also skips the service on heavy frames.
pub const BUDGET: u32 = 400;
/// Samples the boot calibration's two one-shots differ by.
pub const CLOCK_SAMPLES: u32 = 12_288;

/// The sampler's two voices.
pub const LEFT: u16 = 0xDF20;
pub const RIGHT: u16 = 0xDF40;
/// Sampler rate divider: 6.25 MHz / (divider + 2) = 15,985 Hz (the streams are 16 kHz: 1.6 cents flat).
pub const DIVIDER: u16 = 389;
pub const VOLUME: u8 = 32;

pub const HEADER: u32 = 32;
const BOOK_BYTES: u32 = 16;

/// `Player::state`.
pub const OFF: u8 = 0;
pub const RUNNING: u8 = 1;

/// `Player::fault` (why the music is off).
pub const NO_SAMPLER: u8 = 1;
pub const TIMEOUT: u8 = 2;
pub const CLOCK: u8 = 3;
pub const NO_MUSIC: u8 = 4;
pub const BAD_STREAM: u8 = 5;

/// The decode kernel's state; layout shared with `asm/digital.S` (`digital_decode_run`).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kernel {
    /// Codebook deltas: mono uses 0..8.
    pub book: [i16; 16],
    pub samples: [i16; 2],
    pub reservoir: u16,
    pub bits: u8,
    pub channels: u8,
}
const _: () = assert!(core::mem::size_of::<Kernel>() == 40);

/// Offsets inside [`Player`] the DMAs write to (`#[repr(C)]`).
#[cfg_attr(not(target_arch = "mos"), allow(dead_code))]
const BOOK_AT: u16 = 0;
#[cfg_attr(not(target_arch = "mos"), allow(dead_code))]
const CACHE_AT: u16 = 40;
const CACHE: u8 = 32;

/// The DLT5 header's first 22 bytes, as laid out in the asset (`#[repr(C)]` puts every field at its
/// format offset; the C64 DMAs the header straight into [`Player::hdr`]).
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    pub magic: [u8; 4],
    pub rate: u32,
    pub total: u32,
    pub stream_bytes: u32,
    pub entries: u16,
    pub books: u16,
    pub bits: u8,
    pub channels: u8,
}
#[cfg_attr(not(target_arch = "mos"), allow(dead_code))]
const HEADER_BYTES: u16 = 22;
const _: () = assert!(core::mem::size_of::<Header>() == 24 || core::mem::size_of::<Header>() == 22);

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player {
    pub kernel: Kernel,
    pub cache: [u8; 32],
    pub hdr: Header,
    /// REU address of the track's 3-bit stream; bytes fetched.
    pub stream: u32,
    pub fetched: u32,
    /// REU address of codebook 0; the current one.
    pub books_at: u32,
    pub book: u16,
    /// Samples per short segment, the number of long (+8) ones, samples left in this one.
    pub per_segment: u16,
    pub long: u16,
    pub segment_left: u16,
    pub cache_pos: u8,
    pub cache_len: u8,
    /// Samples decoded in this loop.
    pub decoded: u32,
    /// Samples written to the ring, and played by the left voice (estimated), since the start.
    pub produced: u32,
    pub played: u32,
    /// The clock: CIA ticks per [`CLOCK_SAMPLES`] samples, the remainder carried, the last read of
    /// timer A (16 bits: a frame is far shorter than its 64 ms wrap).
    pub ticks: u32,
    pub frac: u32,
    pub last: u16,
    pub state: u8,
    pub fault: u8,
    pub underruns: u16,
    pub loops: u16,
}
const _: () = assert!(core::mem::size_of::<Player>() <= 0x200);

impl Player {
    #[inline(always)]
    pub const fn new() -> Player {
        Player {
            kernel: Kernel { book: [0; 16], samples: [0; 2], reservoir: 0, bits: 0, channels: 1 },
            cache: [0; 32],
            hdr: Header { magic: [0; 4], rate: 0, total: 0, stream_bytes: 0, entries: 0, books: 0, bits: 0, channels: 0 },
            stream: 0,
            fetched: 0,
            books_at: 0,
            book: 0,
            per_segment: 0,
            long: 0,
            segment_left: 0,
            cache_pos: 0,
            cache_len: 0,
            decoded: 0,
            produced: 0,
            played: 0,
            ticks: 0,
            frac: 0,
            last: 0,
            state: OFF,
            fault: 0,
            underruns: 0,
            loops: 0,
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new()
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Kernel { book: [0; 16], samples: [0; 2], reservoir: 0, bits: 0, channels: 1 }
    }
}

/// The player in C64 RAM.
#[cfg(target_arch = "mos")]
#[inline(always)]
pub fn player() -> &'static mut Player {
    // SAFETY: $C800-$C9FF is reserved for it (layout comment above); the main loop is its only user.
    unsafe { &mut *(PLAYER as *mut Player) }
}

/// Published 8-bit sample of a predictor.
pub fn rounded(sample: i16) -> u8 {
    (((sample as i32 + 128) >> 8).clamp(-128, 127)) as i8 as u8
}

/// Little-endian reads from the REU through a bounce in C64 RAM (`at`).
fn reu_bytes<H: Hw + ?Sized, const N: usize>(hw: &mut H, addr: u32, at: u16) -> [u8; N] {
    hw::fetch(hw, addr, at, N as u16);
    let mut b = [0u8; N];
    for (i, v) in b.iter_mut().enumerate() {
        *v = hw.peek(at + i as u16);
    }
    b
}

// ------------------------------------------------------------------------------------------- the producer

/// Load codebook `p.book` and its segment length.
#[inline(never)]
fn load_book<H: Hw + ?Sized>(hw: &mut H, p: &mut Player) {
    let from = p.books_at + p.book as u32 * BOOK_BYTES;
    #[cfg(target_arch = "mos")]
    hw::fetch(hw, from, PLAYER + BOOK_AT, BOOK_BYTES as u16);
    #[cfg(not(target_arch = "mos"))]
    {
        let b: [u8; 16] = reu_bytes(hw, from, PLAYER + 0x100);
        for k in 0..8 {
            p.kernel.book[k] = i16::from_le_bytes([b[2 * k], b[2 * k + 1]]);
        }
    }
    p.segment_left = p.per_segment + if p.book < p.long { 8 } else { 0 };
}

/// Back to the first sample (the track loops; predictors reset as the encoder's do).
#[inline(never)]
fn rewind<H: Hw + ?Sized>(hw: &mut H, p: &mut Player) {
    p.decoded = 0;
    p.fetched = 0;
    p.cache_pos = 0;
    p.cache_len = 0;
    p.kernel.samples = [0; 2];
    p.kernel.reservoir = 0;
    p.kernel.bits = 0;
    p.book = 0;
    load_book(hw, p);
}

/// Refill the 32-byte input cache once it is spent.
#[inline(never)]
fn refill<H: Hw + ?Sized>(hw: &mut H, p: &mut Player) {
    let n = (p.hdr.stream_bytes - p.fetched).min(CACHE as u32) as u8;
    #[cfg(target_arch = "mos")]
    hw::fetch(hw, p.stream + p.fetched, PLAYER + CACHE_AT, n as u16);
    #[cfg(not(target_arch = "mos"))]
    {
        let b: [u8; 32] = reu_bytes(hw, p.stream + p.fetched, PLAYER + 0x100);
        p.cache = b;
    }
    p.fetched += n as u32;
    p.cache_pos = 0;
    p.cache_len = n;
}

#[cfg(target_arch = "mos")]
unsafe extern "C" {
    fn digital_decode_run(kernel: *mut Kernel, input: *const u8, output: *mut u8, frames: u8) -> u8;
}

/// Decode `count` samples (enough codes are cached) to C64 RAM at `out`; returns cache bytes used.
#[inline(always)]
fn run<H: Hw + ?Sized>(hw: &mut H, p: &mut Player, out: u16, count: u8) -> u8 {
    #[cfg(target_arch = "mos")]
    {
        let _ = hw;
        // SAFETY: the kernel reads `count` codes from the cache (the caller checked they are there) and
        // writes `count` bytes at `out` (the staging buffer).
        unsafe { digital_decode_run(&mut p.kernel, p.cache.as_ptr().add(p.cache_pos as usize), out as *mut u8, count) }
    }
    #[cfg(not(target_arch = "mos"))]
    {
        let k = &mut p.kernel;
        let mut pos = p.cache_pos as usize;
        for i in 0..count as u16 {
            if k.bits < 3 {
                k.reservoir |= (p.cache[pos] as u16) << k.bits;
                pos += 1;
                k.bits += 8;
            }
            let code = (k.reservoir & 7) as usize;
            k.reservoir >>= 3;
            k.bits -= 3;
            k.samples[0] = k.samples[0].saturating_add(k.book[code]);
            hw.poke(out + i, rounded(k.samples[0]));
        }
        (pos - p.cache_pos as usize) as u8
    }
}

/// Decode `n` (≤ 2048) samples into [`STAGING`].
#[inline(never)]
pub fn decode<H: Hw + ?Sized>(hw: &mut H, p: &mut Player, n: u16) {
    let mut left = n;
    let mut out = STAGING;
    while left != 0 {
        if p.decoded == p.hdr.total {
            rewind(hw, p);
            p.loops = p.loops.wrapping_add(1);
        }
        if p.segment_left == 0 && p.book + 1 < p.hdr.books {
            p.book += 1;
            load_book(hw, p);
        }
        if p.kernel.bits < 3 && p.cache_pos == p.cache_len {
            refill(hw, p);
        }
        let codes = ((p.cache_len - p.cache_pos) as u16 * 8 + p.kernel.bits as u16) / 3;
        let mut count = left.min(codes).min(128);
        let rest = p.hdr.total - p.decoded;
        if rest < count as u32 {
            count = rest as u16;
        }
        if p.book + 1 < p.hdr.books && p.segment_left < count {
            count = p.segment_left;
        }
        if count == 0 {
            // A corrupt stream: stop rather than spin.
            p.state = OFF;
            p.fault = BAD_STREAM;
            return;
        }
        let used = run(hw, p, out, count as u8);
        p.cache_pos += used;
        p.decoded += count as u32;
        p.segment_left = p.segment_left.saturating_sub(count);
        left -= count;
        out += count;
    }
}

/// Decode `n` samples and stash them at the ring's write position.
#[inline(never)]
fn produce<H: Hw + ?Sized>(hw: &mut H, p: &mut Player, n: u16) {
    decode(hw, p, n);
    if p.state != RUNNING {
        return;
    }
    let at = p.produced & (RING_SAMPLES - 1);
    let first = (RING_SAMPLES - at).min(n as u32) as u16;
    hw::stash(hw, STAGING, RING + at, first);
    if first < n {
        hw::stash(hw, STAGING + first, RING, n - first);
    }
    p.produced += n as u32;
}

/// The CIA 2 timers A and B chained: a 32-bit down-counter at the system clock.
#[inline(always)]
pub fn timer<H: Hw + ?Sized>(hw: &mut H) -> u32 {
    loop {
        let h = hw.peek(0xDD07);
        let b = [hw.peek(0xDD04), hw.peek(0xDD05), hw.peek(0xDD06), hw.peek(0xDD07)];
        if b[3] == h && hw.peek(0xDD05) == b[1] {
            return u32::from_le_bytes(b);
        }
    }
}

/// CIA 2 timer A alone (the low half of [`timer`]).
#[inline(always)]
fn timer16<H: Hw + ?Sized>(hw: &mut H) -> u16 {
    loop {
        let h = hw.peek(0xDD05);
        let l = hw.peek(0xDD04);
        if hw.peek(0xDD05) == h {
            return u16::from_le_bytes([l, h]);
        }
    }
}

/// Samples a heavy frame decodes (the caller's choice, `service`'s `light`): a little under the ~267 a
/// frame plays, so a heavy frame pays less; light frames make it up (`BUDGET`).
pub const HEAVY_BUDGET: u32 = 200;
/// Below this lead (samples) every frame decodes the full `BUDGET`, heavy or not.
pub const SAFE_LEAD: u32 = 8192;

/// Once per frame, I/O banked in (`AUDIO` phase): advance the clock and keep the ring full. A `light`
/// frame decodes up to `BUDGET` samples, a heavy one up to `HEAVY_BUDGET` (unless the lead is short):
/// the music is not game state, so its cost follows the frame's (60 FPS is a hard limit).
#[inline(never)]
pub fn service<H: Hw + ?Sized>(hw: &mut H, p: &mut Player, light: bool) {
    if p.state != RUNNING {
        return;
    }
    let now = timer16(hw);
    let elapsed = p.last.wrapping_sub(now) as u32;
    p.last = now;
    // Samples played = ticks × CLOCK_SAMPLES / p.ticks, by subtraction (~267 a frame): a 32-bit
    // divide (and LLVM turns quotient-and-remainder into a 2.4 KB divmod) costs more 6502 code.
    let mut x = elapsed * CLOCK_SAMPLES + p.frac;
    // Samples since the last service: a few hundred, so 16 bits (32-bit counting cost the AUDIO phase room).
    let mut since = 0u16;
    while x >= p.ticks {
        x -= p.ticks;
        since += 1;
    }
    p.played += since as u32;
    p.frac = x;
    if p.played >= p.produced {
        // The sampler caught up (a stall): skip ahead rather than play stale ring data again.
        p.underruns = p.underruns.wrapping_add(1);
        p.produced = p.played + 256;
    }
    let target = p.played + RING_SAMPLES - GUARD;
    let lead = p.produced - p.played;
    // A short lead decodes what was played since the last service and half again, so the ring recovers
    // at any frame rate (BUDGET alone is ~1.5 frames at 60 fps: at 30 fps the ring ran dry).
    let short = lead < SAFE_LEAD;
    let mut budget = if light || short { BUDGET as u16 } else { HEAVY_BUDGET as u16 };
    // Capped at two light budgets (30 fps's need): a long stall must not turn into a long decode inside a
    // frame that is already late; the next frames make up the rest.
    let need = (since + (since >> 1)).min(2 * BUDGET as u16);
    if short && need > budget {
        budget = need;
    }
    let mut want = target.saturating_sub(p.produced).min(budget as u32) as u16;
    while want != 0 && p.state == RUNNING {
        let n = want.min(512);
        produce(hw, p, n);
        want -= n;
    }
}

// ------------------------------------------------------------------------------------- the sampler

#[inline(always)]
fn be24<H: Hw + ?Sized>(hw: &mut H, at: u16, v: u32) {
    hw.poke(at, (v >> 16) as u8);
    hw.poke(at + 1, (v >> 8) as u8);
    hw.poke(at + 2, v as u8);
}

/// Both voices stopped and silent.
#[inline(always)]
pub fn mute<H: Hw + ?Sized>(hw: &mut H) {
    for v in [LEFT, RIGHT] {
        hw.poke(v, 0);
        hw.poke(v + 1, 0);
    }
}

/// One voice: `start` (REU), `len` bytes, repeating from `start + rep_a` to `start + rep_b`.
#[inline(always)]
fn voice<H: Hw + ?Sized>(hw: &mut H, v: u16, pan: u8, start: u32, len: u32, rep_a: u32, rep_b: u32, divider: u16, volume: u8) {
    hw.poke(v, 0);
    hw.poke(v + 1, volume);
    hw.poke(v + 2, pan);
    hw.poke(v + 4, 1);
    be24(hw, v + 5, start);
    be24(hw, v + 9, len);
    hw.poke(v + 14, (divider >> 8) as u8);
    hw.poke(v + 15, divider as u8);
    be24(hw, v + 17, rep_a);
    be24(hw, v + 21, rep_b);
}

/// Gate both voices (call with interrupts masked: they must start on the same tick).
#[inline(always)]
pub fn gate<H: Hw + ?Sized>(hw: &mut H) {
    hw.poke(LEFT, 3);
    hw.poke(RIGHT, 3);
}

// -------------------------------------------------------------------------------------- start a track

/// Fill the ring up to the guard from the track's start (after [`tune::load`]), before the voices go.
#[inline(never)]
pub fn prime<H: Hw + ?Sized>(hw: &mut H, p: &mut Player) {
    if p.state != RUNNING {
        return;
    }
    rewind(hw, p);
    while p.produced < RING_SAMPLES - GUARD && p.state == RUNNING {
        produce(hw, p, 512);
    }
}

// ------------------------------------------------------------------------------------- boot (calibration)

/// One-time setup (the `TUNE` phase, paged once at boot): check the sampler is mapped, chain the CIA 2
/// timers, clear the sampler's retained low byte, measure the clock ([`calibrate`]); read a track's
/// header ([`load`]); set up and start the voices ([`arm`], [`started`]).
pub mod tune {
    use super::*;

    /// Point the player at the DLT5 asset at REU `asset` (0: no music). Then [`prime`] (AUDIO phase),
    /// [`arm`], gate both voices with interrupts masked, [`started`].
    #[inline(never)]
    pub fn load<H: Hw + ?Sized>(hw: &mut H, p: &mut Player, asset: u32) {
        p.state = OFF;
        if p.ticks == 0 {
            return;
        }
        if asset == 0 {
            p.fault = NO_MUSIC;
            return;
        }
        #[cfg(target_arch = "mos")]
        hw::fetch(hw, asset, PLAYER + core::mem::offset_of!(Player, hdr) as u16, HEADER_BYTES);
        #[cfg(not(target_arch = "mos"))]
        {
            let h: [u8; 22] = reu_bytes(hw, asset, STAGING);
            let u32_at = |i: usize| u32::from_le_bytes([h[i], h[i + 1], h[i + 2], h[i + 3]]);
            p.hdr = Header {
                magic: [h[0], h[1], h[2], h[3]],
                rate: u32_at(4),
                total: u32_at(8),
                stream_bytes: u32_at(12),
                entries: u16::from_le_bytes([h[16], h[17]]),
                books: u16::from_le_bytes([h[18], h[19]]),
                bits: h[20],
                channels: h[21],
            };
        }
        let h = p.hdr;
        if h.magic != *b"DLT5" || h.bits != 3 || h.channels != 1 || h.entries != 8 {
            p.fault = BAD_STREAM;
            return;
        }
        p.books_at = asset + HEADER;
        p.stream = p.books_at + h.books as u32 * BOOK_BYTES;
        // Segment lengths: groups = q × books + r (by subtraction, no 32-bit divide).
        let mut r = h.total.div_ceil(8);
        let mut q = 0u16;
        while r >= h.books as u32 {
            r -= h.books as u32;
            q += 1;
        }
        p.per_segment = q * 8;
        p.long = r as u16;
        p.kernel.channels = 1;
        p.produced = 0;
        p.played = 0;
        p.frac = 0;
        p.state = RUNNING;
        // The right voice's lead-in: DELAY zeros before the ring.
        hw.fill(STAGING, DELAY, 0);
        hw::stash(hw, STAGING, RING - DELAY as u32, DELAY);
    }

    /// The voices were gated: start the clock.
    #[inline(never)]
    pub fn started<H: Hw + ?Sized>(hw: &mut H, p: &mut Player) {
        p.last = timer16(hw);
    }

    /// The ring's register image for both voices: (register offset, value). The left voice plays the ring
    /// from its start, repeating it; the right voice starts [`DELAY`] samples earlier (zeros), so it plays
    /// every sample 10 ms after the left. Volume full, panned hard left and right.
    const ARM: [(u8, u8); 34] = {
        let mut t = [(0u8, 0u8); 34];
        let mut v = 0;
        while v < 2 {
            let d = if v == 0 { 0 } else { DELAY as u32 };
            let (start, len, rep_a, rep_b) = (RING - d, RING_SAMPLES + d + 1, d, RING_SAMPLES + d);
            let base = (v * 0x20) as u8;
            let regs: [(u8, u8); 17] = [
                (0, 0), (1, VOLUME), (2, if v == 0 { 0 } else { 15 }), (4, 1),
                (5, (start >> 16) as u8), (6, (start >> 8) as u8), (7, start as u8),
                (9, (len >> 16) as u8), (10, (len >> 8) as u8), (11, len as u8),
                (14, (DIVIDER >> 8) as u8), (15, DIVIDER as u8),
                (17, (rep_a >> 16) as u8), (18, (rep_a >> 8) as u8), (19, rep_a as u8),
                (21, (rep_b >> 16) as u8), (22, (rep_b >> 8) as u8),
            ];
            let mut i = 0;
            while i < 17 {
                t[v * 17 + i] = (base + regs[i].0, regs[i].1);
                i += 1;
            }
            v += 1;
        }
        t
    };

    /// Set up the repeating ring on both voices (not yet gated; see [`ARM`]).
    #[inline(never)]
    pub fn arm<H: Hw + ?Sized>(hw: &mut H) {
        for &(r, v) in ARM.iter() {
            hw.poke(LEFT + r as u16, v);
        }
        // rep_b's low bytes (the table holds 17 registers a voice).
        hw.poke(LEFT + 23, RING_SAMPLES as u8);
        hw.poke(RIGHT + 23, (RING_SAMPLES + DELAY as u32) as u8);
        hw.poke(LEFT + 0x1F, 255);
    }

    #[inline(never)]
    pub fn calibrate<H: Hw + ?Sized>(hw: &mut H, p: &mut Player) {
        // SAFETY: Player is plain integers; all-zero is a stopped player (cheaper than a struct literal).
        unsafe { core::ptr::write_bytes(p as *mut Player, 0, 1) };
        mute(hw);
        if hw.peek(LEFT + 1) != 0x10 {
            p.fault = NO_SAMPLER;
            return;
        }
        // Timer B counts timer A's underflows: 32 bits. Timer A stays the renderer's 16-bit clock.
        hw.poke(0xDD0D, 0x7F);
        let _ = hw.peek(0xDD0D);
        for r in [0xDD04, 0xDD05, 0xDD06, 0xDD07] {
            hw.poke(r, 0xFF);
        }
        hw.poke(0xDD0E, 0x11);
        hw.poke(0xDD0F, 0x51);
        // Silence in the ring, then a muted 4-frame 16-bit stereo play clears the retained low byte.
        hw.fill(STAGING, 1024, 0);
        let mut at = 0;
        while at < RING_SAMPLES + 1024 {
            hw::stash(hw, STAGING, RING - 1024 + at, 1024);
            at += 1024;
        }
        if measure(hw, 312, 4, true).is_none() {
            p.fault = TIMEOUT;
            return;
        }
        let (Some(a), Some(b)) = (measure(hw, DIVIDER, 4096, false), measure(hw, DIVIDER, 16384, false)) else {
            p.fault = TIMEOUT;
            return;
        };
        let ticks = b.wrapping_sub(a);
        if !(250_000..=1_100_000).contains(&ticks) {
            p.fault = CLOCK;
            return;
        }
        p.ticks = ticks;
    }

    /// CIA ticks for a muted one-shot of `frames` (mono, or 16-bit stereo when `wide`); `None` on timeout.
    #[inline(never)]
    fn measure<H: Hw + ?Sized>(hw: &mut H, divider: u16, frames: u16, wide: bool) -> Option<u32> {
        let len = if wide { (frames as u32) << 2 } else { frames as u32 };
        for (v, off) in [(LEFT, 0u32), (RIGHT, if wide { 2 } else { 0 })] {
            voice(hw, v, 0, RING + off, len, 0, 0, divider, 0);
        }
        hw.poke(LEFT + 0x1F, 255);
        let control = if wide { 0x50 } else { 0 };
        let begin = timer(hw);
        hw.poke(LEFT, control | 5);
        hw.poke(RIGHT, control | 1);
        let mut spins = 4_000_000u32;
        while hw.peek(LEFT) == 0 {
            spins -= 1;
            if spins == 0 || begin.wrapping_sub(timer(hw)) > 4_000_000 {
                mute(hw);
                return None;
            }
        }
        let t = begin.wrapping_sub(timer(hw));
        mute(hw);
        hw.poke(LEFT + 0x1F, 255);
        Some(t)
    }
}

/// Host checks of a DLT5 asset (bake time): header, extents and the CRC-8 table.
#[cfg(feature = "host")]
pub fn validate(asset: &[u8]) -> Result<(u32, u16), String> {
    if asset.len() < HEADER as usize || &asset[..4] != b"DLT5" {
        return Err("not a DLT5 asset".into());
    }
    let u32_at = |i: usize| u32::from_le_bytes(asset[i..i + 4].try_into().unwrap());
    let u16_at = |i: usize| u16::from_le_bytes(asset[i..i + 2].try_into().unwrap());
    let (rate, total, stream) = (u32_at(4), u32_at(8), u32_at(12));
    let books = u16_at(18);
    if rate != 16_000 || u16_at(16) != 8 || asset[20] != 3 || asset[21] != 1 {
        return Err(format!("profile: rate {rate}, entries {}, bits {}, channels {} (want 16 kHz mono 3-bit)", u16_at(16), asset[20], asset[21]));
    }
    let s0 = HEADER as usize + books as usize * BOOK_BYTES as usize;
    let crc_at = s0 + stream as usize;
    let crcs = (stream as usize).div_ceil(1024);
    if asset.len() != crc_at + crcs || stream as u64 * 8 < total as u64 * 3 {
        return Err(format!("extents: {} bytes, stream {stream} at {s0}, {crcs} CRCs", asset.len()));
    }
    let mut crc = 0u8;
    for (i, &b) in asset[s0..crc_at].iter().enumerate() {
        crc ^= b;
        for _ in 0..8 {
            crc = (crc << 1) ^ if crc & 0x80 != 0 { 7 } else { 0 };
        }
        if (i + 1) % 1024 == 0 || i + 1 == stream as usize {
            if asset[crc_at + i / 1024] != crc {
                return Err(format!("CRC at stream byte {i}"));
            }
        }
    }
    Ok((total, books))
}

#[cfg(all(test, feature = "host"))]
mod tests {
    use super::*;
    use crate::model::Machine;

    /// The REU model plus a sampler that plays at an exact rate against a CIA that counts cycles.
    struct Rig {
        m: Machine,
        voices: [u8; 64],
        /// CIA ticks elapsed; the sampler plays 16 kHz at 1,022,727 ticks per second (NTSC).
        t: u64,
        one_shot_until: u64,
    }

    impl Hw for Rig {
        fn peek(&mut self, a: u16) -> u8 {
            self.t += 2;
            match a {
                0xDF20..=0xDF5F => {
                    if a == LEFT {
                        return (self.one_shot_until != 0 && self.t >= self.one_shot_until) as u8;
                    }
                    if a == LEFT + 1 {
                        return 0x10;
                    }
                    self.voices[(a - 0xDF20) as usize]
                }
                0xDD04..=0xDD07 => {
                    let v = (u32::MAX as u64 - self.t) as u32;
                    v.to_le_bytes()[(a - 0xDD04) as usize]
                }
                _ => self.m.peek(a),
            }
        }
        fn poke(&mut self, a: u16, v: u8) {
            self.t += 2;
            match a {
                0xDF20..=0xDF5F => {
                    self.voices[(a - 0xDF20) as usize] = v;
                    if a == LEFT && v & 1 != 0 {
                        let len = u32::from_be_bytes([0, self.voices[9], self.voices[10], self.voices[11]]) as u64;
                        let frames = if v & 0x10 != 0 { len / 4 } else { len };
                        self.one_shot_until = self.t + 30 + frames * 1_022_727 / 16_000;
                    }
                    if a == LEFT && v == 0 {
                        self.one_shot_until = 0;
                    }
                }
                0xDD00..=0xDD0F => {}
                _ => self.m.poke(a, v),
            }
        }
        fn reu_exec(&mut self, c: u8) {
            self.m.reu_exec(c)
        }
    }

    fn rig(asset: &[u8], at: usize) -> Rig {
        let mut m = Machine::new(&[]);
        m.reu[at..at + asset.len()].copy_from_slice(asset);
        Rig { m, voices: [0; 64], t: 0, one_shot_until: 0 }
    }

    fn asset() -> Option<Vec<u8>> {
        std::fs::read("assets/music/cue.stage_s1.dlt").ok()
    }

    /// The player's decode equals the encoder's own decode (its preview WAV, rounded to 8 bits) for the
    /// whole track and on into the loop.
    #[test]
    fn decode_matches_the_encoder() {
        let Some(a) = asset() else { return };
        let (total, _) = validate(&a).unwrap();
        let wav = std::fs::read("output/audio/cue.stage_s1.preview.wav").ok();
        let mut r = rig(&a, 0x10_0000);
        let mut p = Player::new();
        p.ticks = 785_000;
        tune::load(&mut r, &mut p, 0x10_0000);
        prime(&mut r, &mut p);
        assert_eq!(p.state, RUNNING);
        // Read the ring back: the first lap is the start of the track.
        let ring = &r.m.reu[RING as usize..(RING + RING_SAMPLES - GUARD) as usize];
        if let Some(w) = wav {
            let pcm: Vec<i16> = w[44..].chunks_exact(2).map(|c| i16::from_le_bytes([c[0], c[1]])).collect();
            let mut off = 0usize;
            for (i, &s) in ring.iter().enumerate() {
                let want = rounded(pcm[i]);
                off += (s != want) as usize;
            }
            println!("audio: {off} of {} ring samples differ from the encoder's decode", ring.len());
            assert!(off * 1000 < ring.len(), "{off} of {} samples differ from the encoder's decode", ring.len());
        }
        // Decode the rest and over the loop point without faults.
        let mut left = total as i64 - p.decoded as i64 + 5000;
        while left > 0 {
            decode(&mut r, &mut p, 2000);
            assert_eq!(p.fault, 0);
            left -= 2000;
        }
        assert_eq!(p.loops, 1);
    }

    /// Boot calibration measures the rig's clock, and the producer keeps the ring ahead of the
    /// sampler (never caught, never overwriting the lagging right voice) over many frames.
    #[test]
    fn the_ring_stays_ahead() {
        let Some(a) = asset() else { return };
        let mut r = rig(&a, 0x10_0000);
        let mut p = Player::new();
        tune::calibrate(&mut r, &mut p);
        assert_eq!(p.fault, 0);
        let want = 12_288u64 * 1_022_727 / 16_000;
        assert!((p.ticks as i64 - want as i64).abs() < 200, "ticks {} want {want}", p.ticks);
        tune::load(&mut r, &mut p, 0x10_0000);
        prime(&mut r, &mut p);
        tune::arm(&mut r);
        gate(&mut r);
        tune::started(&mut r, &mut p);
        assert_eq!(r.voices[0x20], 3, "right voice gated");
        let mut lead_min = u32::MAX;
        for frame in 0..6000u32 {
            // A frame of 60 Hz, with an occasional long one; frames 1000..3500 a boss fight of mostly
            // heavy frames (decoding less: the lead runs down to SAFE_LEAD and holds there).
            r.t += if frame % 97 == 0 { 40_000 } else { 17_045 };
            let light = !(1000..3500).contains(&frame) || frame % 4 == 0;
            service(&mut r, &mut p, light);
            let played = ((r.t as f64) * 16_000.0 / 1_022_727.0) as u32;
            let _ = played;
            lead_min = lead_min.min(p.produced - p.played);
            assert!(p.produced - p.played <= RING_SAMPLES - GUARD + 1, "overran the right voice");
        }
        assert_eq!(p.underruns, 0);
        assert!(lead_min > SAFE_LEAD / 2, "lead {lead_min}");
        println!("audio: ticks {} lead min {lead_min} produced {}", p.ticks, p.produced);
    }

    /// A game that drops to 30 fps, every frame heavy (the border painter's first C64U run): the
    /// short-lead budget follows what was played, so the ring never runs dry.
    #[test]
    fn the_ring_survives_30_fps() {
        let Some(a) = asset() else { return };
        let mut r = rig(&a, 0x10_0000);
        let mut p = Player::new();
        tune::calibrate(&mut r, &mut p);
        tune::load(&mut r, &mut p, 0x10_0000);
        prime(&mut r, &mut p);
        tune::arm(&mut r);
        gate(&mut r);
        tune::started(&mut r, &mut p);
        let mut lead_min = u32::MAX;
        for _ in 0..4000u32 {
            r.t += 2 * 17_045;
            service(&mut r, &mut p, false);
            lead_min = lead_min.min(p.produced - p.played);
            assert!(p.produced - p.played <= RING_SAMPLES - GUARD + 1, "overran the right voice");
        }
        assert_eq!(p.underruns, 0, "lead min {lead_min}");
        assert!(lead_min > SAFE_LEAD / 2, "lead {lead_min}");
    }
}
