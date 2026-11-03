//! The per-level load: the C64 reads the next level's image file from the Ultimate's storage into the REU
//! through the Ultimate Command Interface (UCI, `$DF1C-$DF1F`; the C64U config "Command Interface" on).
//!
//! The DOS target's commands, as the firmware parses them (GideonZ/1541ultimate
//! `software/filemanager/dos.cc`): `CHANGE_DIR` (path), `OPEN_FILE` (mode, name), `LOAD_REU` (u32 LE REU
//! address, u32 LE length: the open file is read in one go straight into REU memory, clamped at 16 MiB,
//! and the reply is `"$%6x BYTES LOADED TO REU $%6x"`), `CLOSE_FILE`. Each answers a status string,
//! `"00,OK"` on success. The Ultimate does the transfer, so the C64 bus is not stalled (the HUD IRQ runs).
//!
//! Every wait is bounded ([`SHORT`], [`LONG`]): an interface that stops answering is aborted, never waited
//! on forever. A reply may come in several packets (`DATA_MORE` until the last). An interface found in the
//! middle of a command (a reset during one) is aborted first; a push it refuses (`ERROR`) is aborted and
//! sent once more.
//!
//! Level images are written at their used size and never reach `ace::MIND_REU` (`bake` refuses them), so a
//! load leaves the Mind, the music ring and the phase bank alone; every image carries the same phases.
//! Shared code (`Hw`), so the host model ([`crate::model::Machine`]'s UCI) checks the protocol; on the
//! C64 it is in the TUNE phase (`paging::TUNE`, with `load_level`).

use crate::hw::Hw;

/// Write: control (`PUSH`, `ACCEPT`, `ABORT`, `CLEAR_ERROR`); read: status (`BUSY`, `ERROR`, `STATE`,
/// `STATUS_AV`, `DATA_AV`).
pub const CONTROL: u16 = 0xDF1C;
/// Write: the command's bytes; read: the interface's id ([`ID`]).
pub const COMMAND: u16 = 0xDF1D;
/// Read: the reply's bytes, then the status string's.
pub const RESPONSE: u16 = 0xDF1E;
pub const STATUS: u16 = 0xDF1F;
pub const ID: u8 = 0xC9;

pub const PUSH: u8 = 0x01;
pub const ACCEPT: u8 = 0x02;
pub const ABORT: u8 = 0x04;
pub const CLEAR_ERROR: u8 = 0x08;

pub const BUSY: u8 = 0x01;
pub const ERROR: u8 = 0x08;
pub const STATE: u8 = 0x30;
pub const IDLE: u8 = 0x00;
pub const PROCESSING: u8 = 0x10;
pub const DATA_LAST: u8 = 0x20;
pub const DATA_MORE: u8 = 0x30;
pub const STATUS_AV: u8 = 0x40;
pub const DATA_AV: u8 = 0x80;

/// The DOS target and its commands.
pub const DOS: u8 = 0x01;
pub const OPEN_FILE: u8 = 0x02;
pub const CLOSE_FILE: u8 = 0x03;
pub const CHANGE_DIR: u8 = 0x11;
pub const LOAD_REU: u8 = 0x21;
/// FatFs `FA_READ`.
pub const READ: u8 = 0x01;

/// A wait's bound, in 65,536 status polls (a poll is an I/O read, ~2 us on the C64U with its loop): ~2 s
/// for the quick commands.
pub const SHORT: u8 = 16;
/// ~33 s for `LOAD_REU` (16 MB; the real time is still to be measured on the C64U).
pub const LONG: u8 = u8::MAX;

/// [`load_image`]'s outcomes: the whole file arrived.
pub const LOADED: u8 = 0;
/// Nothing was written (the directory or the file could not be opened): the REU holds the old image.
pub const UNCHANGED: u8 = 1;
/// `LOAD_REU` failed or timed out: the REU may hold part of the file.
pub const DAMAGED: u8 = 2;

/// Each level's image file (`bake` writes them under these names; `tools/c64u.py upload-levels` puts them
/// in [`LEVEL_DIR`] on the device).
pub const LEVEL_FILES: [&[u8]; 2] = [b"talonhd.reu", b"talonhd_s2.reu"];
const _: () = assert!(LEVEL_FILES.len() == crate::clear::LAST_LEVEL as usize + 1);
/// The device directory holding the level images: `TALONHD_DIR` at build time, else `/Usb0/talonhd`.
pub const LEVEL_DIR: &[u8] = match option_env!("TALONHD_DIR") {
    Some(d) => d.as_bytes(),
    None => b"/Usb0/talonhd",
};

/// A file name as sent: NUL-padded (the firmware reads names as C strings).
const NAME: usize = 16;

/// Every byte the load sends, in one static (on the C64 in the TUNE phase's window, `.rodata.uci`, not
/// main RAM). Paths and names end in a NUL.
#[repr(C)]
struct Commands {
    cd: [u8; 2],
    dir: [u8; LEVEL_DIR.len() + 1],
    open: [u8; 3],
    names: [[u8; NAME]; 2],
    load: [u8; 10],
    close: [u8; 2],
}

const fn padded<const N: usize>(s: &[u8]) -> [u8; N] {
    assert!(s.len() < N, "a NUL must follow");
    let mut out = [0u8; N];
    let mut i = 0;
    while i < s.len() {
        out[i] = s[i];
        i += 1;
    }
    out
}

/// Up to the REU scratch (`ace::MIND_REU`): the Ultimate stops at the file's end.
const CAP: u32 = crate::ace::MIND_REU;
#[cfg_attr(target_arch = "mos", unsafe(link_section = ".rodata.uci"))]
static COMMANDS: Commands = Commands {
    cd: [DOS, CHANGE_DIR],
    dir: padded(LEVEL_DIR),
    open: [DOS, OPEN_FILE, READ],
    names: [padded(LEVEL_FILES[0]), padded(LEVEL_FILES[1])],
    load: [DOS, LOAD_REU, 0, 0, 0, 0, CAP as u8, (CAP >> 8) as u8, (CAP >> 16) as u8, 0],
    close: [DOS, CLOSE_FILE],
};

/// Whether the Ultimate Command Interface answers.
#[inline(always)]
pub fn present<H: Hw + ?Sized>(hw: &mut H) -> bool {
    hw.peek(COMMAND) == ID
}

/// Poll the status until the interface is idle (`idle`), or else has an answer (a reply, `ERROR` or both:
/// not busy and in `DATA_LAST`/`DATA_MORE`); `None` after `budget + 1` × 65,536 polls.
#[inline(never)]
fn wait<H: Hw + ?Sized>(hw: &mut H, budget: u8, idle: bool) -> Option<u8> {
    let mut outer = budget;
    loop {
        let mut n = 0u16;
        loop {
            let s = hw.peek(CONTROL);
            let done = if idle { s & STATE == IDLE } else { s & ERROR != 0 || (s & BUSY == 0 && s & DATA_LAST != 0) };
            if done {
                return Some(s);
            }
            n = n.wrapping_add(1);
            if n == 0 {
                break;
            }
        }
        if outer == 0 {
            return None;
        }
        outer -= 1;
    }
}

/// Abandon whatever the interface is doing, back to idle (bounded).
#[inline(never)]
fn abort<H: Hw + ?Sized>(hw: &mut H) {
    hw.poke(CONTROL, ABORT);
    hw.poke(CONTROL, CLEAR_ERROR);
    wait(hw, SHORT, true);
}

/// One DOS command: `head` then `tail` as its bytes, its answer awaited for up to `budget`; true when its
/// status string starts `00` (its reply is drained unread, every packet).
#[inline(never)]
fn command<H: Hw + ?Sized>(hw: &mut H, head: &[u8], tail: &[u8], budget: u8) -> bool {
    // Left in the middle of a command (a reset during one): a push would only be refused.
    if hw.peek(CONTROL) & (STATE | ERROR) != 0 {
        abort(hw);
    }
    let mut tries = 2u8;
    let mut s = loop {
        for &b in head {
            hw.poke(COMMAND, b);
        }
        for &b in tail {
            hw.poke(COMMAND, b);
        }
        hw.poke(CONTROL, PUSH);
        match wait(hw, budget, false) {
            Some(s) if s & ERROR == 0 => break s,
            got => {
                abort(hw);
                tries -= 1;
                if got.is_none() || tries == 0 {
                    return false;
                }
            }
        }
    };
    let mut ok = 0u8;
    let mut k = 0u8;
    loop {
        while hw.peek(CONTROL) & DATA_AV != 0 {
            hw.peek(RESPONSE);
        }
        while hw.peek(CONTROL) & STATUS_AV != 0 {
            let b = hw.peek(STATUS);
            if k < 2 {
                if b == b'0' {
                    ok += 1;
                }
                k += 1;
            }
        }
        let more = s & STATE == DATA_MORE;
        hw.poke(CONTROL, ACCEPT);
        if !more {
            break;
        }
        match wait(hw, budget, false) {
            Some(n) if n & ERROR == 0 => s = n,
            _ => {
                abort(hw);
                return false;
            }
        }
    }
    if wait(hw, SHORT, true).is_none() {
        abort(hw);
        return false;
    }
    ok == 2
}

/// Load level `level`'s image file ([`LEVEL_FILES`], in [`LEVEL_DIR`]) into the REU from address 0:
/// [`LOADED`], [`UNCHANGED`] (nothing written) or [`DAMAGED`]. A read error or a truncated load replaces
/// the `LOAD_REU` status, so `00` means the whole file arrived.
#[inline(never)]
pub fn load_image<H: Hw + ?Sized>(hw: &mut H, level: u8) -> u8 {
    let k = &COMMANDS;
    if !command(hw, &k.cd, &k.dir, SHORT) || !command(hw, &k.open, &k.names[(level & 1) as usize], SHORT) {
        return UNCHANGED;
    }
    let ok = command(hw, &k.load, &[], LONG);
    command(hw, &k.close, &[], SHORT);
    if ok { LOADED } else { DAMAGED }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Machine, Uci};

    fn files() -> Vec<(Vec<u8>, Vec<u8>)> {
        let a: Vec<u8> = (0..70_000u32).map(|i| (i * 7 + 3) as u8).collect();
        vec![(LEVEL_FILES[1].to_vec(), a)]
    }

    fn machine(uci: Uci) -> Machine {
        let mut m = Machine::new(&[0xAA; 16]);
        m.uci = Some(uci);
        m
    }

    fn arrived(m: &Machine) {
        let want = files()[0].1.clone();
        assert_eq!(m.uci.as_ref().unwrap().loaded, want.len());
        assert_eq!(&m.reu[..want.len()], &want[..]);
        assert_eq!(m.reu[want.len()], 0);
        assert!(m.uci.as_ref().unwrap().idle(), "every command accepted, the file closed");
    }

    #[test]
    fn the_loader_reads_a_level() {
        let mut m = machine(Uci::new(LEVEL_DIR, files()));
        assert!(present(&mut m));
        assert_eq!(load_image(&mut m, 1), LOADED);
        arrived(&m);
        assert_eq!(m.uci.as_ref().unwrap().aborts, 0);
    }

    #[test]
    fn a_missing_level_writes_nothing() {
        let mut m = machine(Uci::new(LEVEL_DIR, files()));
        assert_eq!(load_image(&mut m, 0), UNCHANGED);
        assert_eq!(&m.reu[..16], &[0xAA; 16]);
        // A wrong directory fails at the change.
        m.uci = Some(Uci::new(b"/Usb1/elsewhere", files()));
        assert_eq!(load_image(&mut m, 1), UNCHANGED);
        assert_eq!(&m.reu[..16], &[0xAA; 16]);
        assert!(m.uci.as_ref().unwrap().idle());
    }

    #[test]
    fn a_split_reply_is_read_to_the_end() {
        let mut m = machine(Uci::new(LEVEL_DIR, files()).chunked(4));
        assert_eq!(load_image(&mut m, 1), LOADED);
        arrived(&m);
    }

    #[test]
    fn a_stalled_interface_times_out() {
        let mut m = machine(Uci::new(LEVEL_DIR, files()).stalled_at(CHANGE_DIR));
        assert_eq!(load_image(&mut m, 1), UNCHANGED);
        assert_eq!(&m.reu[..16], &[0xAA; 16]);
        assert!(m.uci.as_ref().unwrap().idle() && m.uci.as_ref().unwrap().aborts > 0);
        let mut m = machine(Uci::new(LEVEL_DIR, files()).stalled_at(LOAD_REU));
        assert_eq!(load_image(&mut m, 1), DAMAGED);
        assert!(m.uci.as_ref().unwrap().idle(), "aborted, then the file closed");
    }

    #[test]
    fn a_stuck_interface_is_aborted_first() {
        let mut m = machine(Uci::new(LEVEL_DIR, files()).mid_command());
        assert_eq!(load_image(&mut m, 1), LOADED);
        arrived(&m);
        assert_eq!(m.uci.as_ref().unwrap().aborts, 1);
    }

    #[test]
    fn paths_and_names_end_in_a_nul() {
        assert_eq!(COMMANDS.dir.last(), Some(&0));
        assert_eq!(&COMMANDS.dir[..LEVEL_DIR.len()], LEVEL_DIR);
        assert!(COMMANDS.names.iter().all(|n| n.contains(&0)));
    }

    #[test]
    fn no_interface_is_not_present() {
        let mut m = Machine::new(&[]);
        assert!(!present(&mut m));
    }
}
