//! REU code paging (C64; a no-op on the host): code that is only active some of the time lives in the
//! REU as a *phase* and is copied into one shared window just before it runs, as TALON's runtime does.
//!
//! - The C64 link script (`target-c64/link.ld`) links every phase at [`WINDOW`]; the PRG carries the
//!   images and `main` stashes them at boot to [`REU_PHASES`] + [`PHASE_SLOT`] × phase, the REU's top bank,
//!   far above the level image (which is uploaded from address 0).
//! - [`page`] makes a phase current: one REU fetch when it is not already in the window. It may be called
//!   from the game step, which runs with I/O banked out; the C64 side banks I/O in around the DMA.
//! - Code in a phase may call resident code only (never another phase), and nothing resident may call
//!   into a phase without paging it first. Render-time helpers of a paged module stay resident (the link
//!   script names them before the phase's catch-all pattern).

/// Where every phase runs: `$9000-$9FFF`, which VIC bank 2 sees as character ROM and the CPU as RAM.
pub const WINDOW: u16 = 0x9000;
pub const WINDOW_LEN: u16 = 0x1000;
/// REU address of phase images: the top 64 KiB bank of the 16 MiB REU, one 4 KiB slot per phase.
pub const REU_PHASES: u32 = 0xFF_0000;
pub const PHASE_SLOT: u32 = 0x1000;

/// Phase ids.
/// The GANTRY QUEEN's fight logic (`boss::Boss::step` and its helpers).
pub const BOSS: u8 = 0;
/// The attract mode's autopilot (`sim::replay::autopilot`), paged only while nobody is playing.
pub const ATTRACT: u8 = 1;
pub const PHASES: u8 = 2;

#[cfg(target_arch = "mos")]
unsafe extern "C" {
    fn talonhd_page(phase: u8);
}

/// Make `phase` the one in the window.
#[inline(always)]
pub fn page(phase: u8) {
    #[cfg(target_arch = "mos")]
    unsafe {
        talonhd_page(phase)
    };
    let _ = phase;
}
