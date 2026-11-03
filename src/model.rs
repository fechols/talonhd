//! Host model (vendored from gclass `a5775bf`) of the parts of a C64 + REU
//! the renderer touches: 64 KiB RAM
//! (`$01=$35`: RAM everywhere except I/O), color RAM, `$DD00`, and the REU
//! registers with per-byte 19-bit address wrap, auto-increment, fixed-address
//! control and the "registers keep their final values" behavior that chained
//! DMAs rely on.

use crate::format::REU_SIZE;
use crate::hw::{Hw, reu};
use crate::layout::{self, Buffer};
use crate::uci;
use crate::palette::PALETTE;

pub struct Machine {
    pub ram: Vec<u8>,
    pub color_ram: [u8; 1024],
    pub reu: Vec<u8>,
    regs: [u8; 16],
    pub dd00: u8,
    /// Last values written to `$D011` and `$D021`.
    pub d011: u8,
    pub d021: u8,
    /// Sprite colors `$D027-$D02E` as last written.
    pub sprite_color: [u8; 8],
    pub stats: Stats,
    /// The Ultimate Command Interface, when modelled (`crate::uci`).
    pub uci: Option<Uci>,
}

/// The Ultimate Command Interface's DOS target as `crate::uci` drives it: one directory of files, the
/// commands the level load uses, replies and status strings as the firmware words them. Commands finish
/// at once (no busy phase) unless [`Uci::stalled_at`] holds one in PROCESSING; [`Uci::chunked`] splits
/// replies into packets (`DATA_MORE` until the last); `ABORT` returns to idle.
#[derive(Clone, Debug)]
pub struct Uci {
    dir: Vec<u8>,
    files: Vec<(Vec<u8>, Vec<u8>)>,
    in_dir: bool,
    open: Option<usize>,
    command: Vec<u8>,
    reply: std::collections::VecDeque<u8>,
    /// The reply's packets after the current one.
    more: std::collections::VecDeque<Vec<u8>>,
    status: std::collections::VecDeque<u8>,
    state: u8,
    error: bool,
    chunk: Option<usize>,
    stall: Option<u8>,
    /// Bytes `LOAD_REU` wrote, all loads.
    pub loaded: usize,
    /// `ABORT`s received.
    pub aborts: usize,
}

impl Uci {
    /// `dir` holding `files` (name, contents).
    pub fn new(dir: &[u8], files: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        Uci {
            dir: dir.to_vec(),
            files,
            in_dir: false,
            open: None,
            command: Vec::new(),
            reply: Default::default(),
            more: Default::default(),
            status: Default::default(),
            state: uci::IDLE,
            error: false,
            chunk: None,
            stall: None,
            loaded: 0,
            aborts: 0,
        }
    }

    /// Replies in packets of `n` bytes.
    pub fn chunked(mut self, n: usize) -> Self {
        self.chunk = Some(n);
        self
    }

    /// The DOS command `code` never finishes (PROCESSING until aborted).
    pub fn stalled_at(mut self, code: u8) -> Self {
        self.stall = Some(code);
        self
    }

    /// As a reset left it in the middle of a command: a reply waiting to be accepted.
    pub fn mid_command(mut self) -> Self {
        self.state = uci::DATA_LAST;
        self.status.extend(b"00,OK".iter().copied());
        self
    }

    /// No command pending and no file open.
    pub fn idle(&self) -> bool {
        self.state == uci::IDLE && self.open.is_none() && self.command.is_empty() && !self.error
    }

    fn peek(&mut self, addr: u16) -> u8 {
        match addr {
            uci::CONTROL => {
                let mut s = self.state;
                if self.error {
                    s |= uci::ERROR;
                }
                if !self.reply.is_empty() {
                    s |= uci::DATA_AV;
                }
                if !self.status.is_empty() {
                    s |= uci::STATUS_AV;
                }
                s
            }
            uci::COMMAND => uci::ID,
            uci::RESPONSE => self.reply.pop_front().unwrap_or(0),
            _ => self.status.pop_front().unwrap_or(0),
        }
    }

    fn poke(&mut self, addr: u16, value: u8, reu: &mut [u8]) {
        match addr {
            uci::CONTROL => {
                if value & uci::PUSH != 0 {
                    if self.state != uci::IDLE {
                        self.error = true;
                    } else {
                        let c = std::mem::take(&mut self.command);
                        if c.get(1).is_some() && c.get(1).copied() == self.stall {
                            self.state = uci::PROCESSING;
                        } else {
                            self.execute(&c, reu);
                        }
                    }
                }
                if value & uci::ACCEPT != 0 {
                    self.reply.clear();
                    self.status.clear();
                    match self.more.pop_front() {
                        Some(p) => {
                            self.reply.extend(p);
                            self.state = if self.more.is_empty() { uci::DATA_LAST } else { uci::DATA_MORE };
                        }
                        None => self.state = uci::IDLE,
                    }
                }
                if value & uci::ABORT != 0 {
                    self.aborts += 1;
                    self.reply.clear();
                    self.more.clear();
                    self.status.clear();
                    self.command.clear();
                    self.state = uci::IDLE;
                }
                if value & uci::CLEAR_ERROR != 0 {
                    self.error = false;
                }
            }
            uci::COMMAND => self.command.push(value),
            _ => {}
        }
    }

    fn execute(&mut self, c: &[u8], reu: &mut [u8]) {
        let trim = |p: &[u8]| p.strip_suffix(b"/").unwrap_or(p).to_vec();
        let mut reply = Vec::new();
        let status: &[u8] = match (c.first(), c.get(1)) {
            (Some(&uci::DOS), Some(&uci::CHANGE_DIR)) => {
                self.in_dir = trim(c[2..].split(|&b| b == 0).next().unwrap()) == trim(&self.dir);
                if self.in_dir { b"00,OK" } else { b"83,NO SUCH DIRECTORY" }
            }
            (Some(&uci::DOS), Some(&uci::OPEN_FILE)) => {
                assert_eq!(c[2], uci::READ, "the level load only reads");
                // A C string: the name ends at the first NUL.
                let name = c[3..].split(|&b| b == 0).next().unwrap();
                self.open = if self.in_dir { self.files.iter().position(|f| f.0 == name) } else { None };
                if self.open.is_some() { b"00,OK" } else { b"62,FILE NOT FOUND" }
            }
            (Some(&uci::DOS), Some(&uci::LOAD_REU)) => match self.open {
                None => b"85,NO FILE OPEN",
                Some(f) => {
                    let le = |i: usize| u32::from_le_bytes([c[i], c[i + 1], c[i + 2], c[i + 3]]) as usize;
                    let addr = le(2) & 0xFF_FFFF;
                    let len = (le(6) & 0x1FF_FFFF).min(REU_SIZE as usize - addr);
                    let data = &self.files[f].1;
                    let n = len.min(data.len());
                    reu[addr..addr + n].copy_from_slice(&data[..n]);
                    self.loaded += n;
                    reply.extend(format!("${n:6x} BYTES LOADED TO REU ${addr:6x}").bytes());
                    b"00,OK"
                }
            },
            (Some(&uci::DOS), Some(&uci::CLOSE_FILE)) => {
                self.open = None;
                b"00,OK"
            }
            _ => panic!("UCI command {c:02x?} not modelled"),
        };
        self.status.extend(status.iter().copied());
        let n = self.chunk.unwrap_or(usize::MAX).max(1);
        let mut packets = reply.chunks(n).map(|p| p.to_vec());
        self.reply.extend(packets.next().unwrap_or_default());
        self.more.extend(packets);
        self.state = if self.more.is_empty() { uci::DATA_LAST } else { uci::DATA_MORE };
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Stats {
    pub commands: u64,
    pub bytes: u64,
    /// Commands whose REU address wrapped inside a 512 KiB bank.
    pub bank_wraps: u64,
    /// DMA commands that wrote somewhere other than RAM/color RAM.
    pub stray_writes: u64,
    pub peeks: u64,
    pub pokes: u64,
}

impl Machine {
    pub fn new(image: &[u8]) -> Self {
        let mut reu = vec![0u8; REU_SIZE as usize];
        reu[..image.len()].copy_from_slice(image);
        Machine { ram: vec![0; 65536], color_ram: [0; 1024], reu, regs: [0; 16], dd00: 0x03, d011: 0, d021: 0, sprite_color: [0; 8], stats: Stats::default(), uci: None }
    }

    fn is_io(addr: u16) -> bool {
        (0xD000..0xE000).contains(&addr)
    }

    fn reg(&self, r: u16) -> u8 {
        self.regs[(r - 0xDF00) as usize]
    }

    /// The visible lines of `buf` (24-row mode, `fine` = scroll fine) as RGB.
    pub fn visible(&self, buf: Buffer, d021: u8, fine: u8) -> Vec<[u8; 3]> {
        let all = self.decode(buf, d021);
        let top = fine as usize * 160;
        all[top..top + layout::VISIBLE as usize * 160].to_vec()
    }

    /// The visible screen at full hires resolution (320×192) with the HUD's
    /// multiplexed sprites drawn from the table and blocks in RAM, as the
    /// raster IRQ shows them.
    pub fn visible_with_hud(&self, buf: Buffer, d021: u8, fine: u8) -> Vec<[u8; 3]> {
        use crate::hud;
        let mc = self.visible(buf, d021, fine);
        let mut out: Vec<[u8; 3]> = mc.iter().flat_map(|&p| [p, p]).collect();
        let bank = if buf == layout::BUFFERS[0] { 0 } else { 1 };
        let table = layout::HUD_TABLE as usize;
        for r in 0..hud::ROWS {
            let top = (r * hud::ROW_PITCH as usize) as i32; // screen line of the row
            for (c, &x0) in hud::X.iter().enumerate() {
                let ptr = self.ram[table + hud::table::PTRS as usize + r * 8 + c] as usize;
                let ptr = if bank == 0 { ptr | layout::HUD_BANK1_POINTER as usize } else { ptr };
                let at = layout::VIC_BANK[bank] as usize + ptr * hud::BLOCK;
                let mut blk = [0u8; hud::BLOCK];
                blk.copy_from_slice(&self.ram[at..at + hud::BLOCK]);
                let color = PALETTE[self.sprite_color[c] as usize & 15];
                for y in 0..21 {
                    for x in 0..24 {
                        let (sx, sy) = (x0 as i32 - 24 + x, top + y);
                        if hud::get(&blk, x, y) && (0..320).contains(&sx) && (0..layout::VISIBLE as i32).contains(&sy) {
                            out[sy as usize * 320 + sx as usize] = color;
                        }
                    }
                }
            }
        }
        out
    }

    /// The bytes the VIC shows for `buf`, decoded to (code, cell) RGB pixels.
    pub fn decode(&self, buf: Buffer, d021: u8) -> Vec<[u8; 3]> {
        let mut out = vec![[0u8; 3]; 160 * 200];
        for y in 0..200u16 {
            for x in 0..160u16 {
                let cell = (y / 8) * 40 + x / 4;
                let byte = self.ram[(buf.bitmap + layout::line_offset(y) + (x / 4) * 8) as usize];
                let code = (byte >> (6 - 2 * (x & 3))) & 3;
                let scr = self.ram[(buf.screen + cell) as usize];
                let color = match code {
                    0 => d021,
                    1 => scr >> 4,
                    2 => scr & 15,
                    _ => self.color_ram[cell as usize] & 15,
                };
                out[y as usize * 160 + x as usize] = PALETTE[color as usize & 15];
            }
        }
        out
    }

    /// Multicolor code of every pixel of `buf`'s bitmap.
    pub fn codes(&self, buf: Buffer) -> Vec<u8> {
        let mut out = vec![0u8; 160 * 200];
        for y in 0..200u16 {
            for x in 0..160u16 {
                let byte = self.ram[(buf.bitmap + layout::line_offset(y) + (x / 4) * 8) as usize];
                out[y as usize * 160 + x as usize] = (byte >> (6 - 2 * (x & 3))) & 3;
            }
        }
        out
    }

    pub fn front(&self) -> Option<Buffer> {
        layout::BUFFERS.iter().copied().find(|b| b.dd00 == self.dd00 & 3)
    }
}

impl Hw for Machine {
    fn peek(&mut self, addr: u16) -> u8 {
        self.stats.peeks += 1;
        match addr {
            0xD800..=0xDBFF => self.color_ram[(addr - 0xD800) as usize] | 0xF0,
            0xDD00 => self.dd00,
            0xDF00..=0xDF0F => self.reg(addr),
            0xDF1C..=0xDF1F if self.uci.is_some() => self.uci.as_mut().unwrap().peek(addr),
            a if Self::is_io(a) => 0xFF,
            a => self.ram[a as usize],
        }
    }

    fn poke(&mut self, addr: u16, value: u8) {
        self.stats.pokes += 1;
        match addr {
            0xD800..=0xDBFF => self.color_ram[(addr - 0xD800) as usize] = value & 15,
            0xDD00 => self.dd00 = value,
            0xD011 => self.d011 = value,
            0xD021 => self.d021 = value & 15,
            0xD027..=0xD02E => self.sprite_color[(addr - 0xD027) as usize] = value & 15,
            0xDF01 => self.reu_exec(value),
            0xDF00..=0xDF0F => self.regs[(addr - 0xDF00) as usize] = value,
            0xDF1C..=0xDF1F if self.uci.is_some() => self.uci.as_mut().unwrap().poke(addr, value, &mut self.reu),
            a if Self::is_io(a) => {}
            a => self.ram[a as usize] = value,
        }
    }

    fn reu_exec(&mut self, command: u8) {
        assert!(command & 0x80 != 0, "REU command without execute bit");
        let mut c64 = self.reg(reu::C64_LO) as u16 | (self.reg(reu::C64_HI) as u16) << 8;
        let mut addr = self.reg(reu::REU_LO) as u32
            | (self.reg(reu::REU_MID) as u32) << 8
            | (self.reg(reu::REU_BANK) as u32) << 16;
        let raw = self.reg(reu::LEN_LO) as u32 | (self.reg(reu::LEN_HI) as u32) << 8;
        let len = if raw == 0 { 65536 } else { raw };
        let ctrl = self.reg(reu::CONTROL);
        self.stats.commands += 1;
        self.stats.bytes += len as u64;
        let mut wrapped = false;
        let mut stray = false;
        for _ in 0..len {
            let phys = (addr as usize) % self.reu.len();
            match command & 3 {
                1 => {
                    let v = self.reu[phys];
                    match c64 {
                        0xD800..=0xDBFF => self.color_ram[(c64 - 0xD800) as usize] = v & 15,
                        a if Self::is_io(a) => stray = true,
                        a => self.ram[a as usize] = v,
                    }
                }
                0 => {
                    let v = match c64 {
                        0xD800..=0xDBFF => self.color_ram[(c64 - 0xD800) as usize],
                        a if Self::is_io(a) => {
                            stray = true;
                            0xFF
                        }
                        a => self.ram[a as usize],
                    };
                    self.reu[phys] = v;
                }
                _ => panic!("swap/verify not modeled"),
            }
            if ctrl & 0x80 == 0 {
                c64 = c64.wrapping_add(1);
            }
            if ctrl & 0x40 == 0 {
                let next = (addr & !0x7FFFF) | ((addr + 1) & 0x7FFFF);
                wrapped |= next & 0x7FFFF == 0;
                addr = next;
            }
        }
        // A transfer that ends exactly on a bank boundary is fine; only a
        // byte transferred *after* the wrap is wrong.
        let ended_on_boundary = wrapped && addr & 0x7FFFF == 0;
        if wrapped && !ended_on_boundary {
            self.stats.bank_wraps += 1;
        }
        if stray {
            self.stats.stray_writes += 1;
        }
        let r = |reg: u16| (reg - 0xDF00) as usize;
        self.regs[r(reu::C64_LO)] = c64 as u8;
        self.regs[r(reu::C64_HI)] = (c64 >> 8) as u8;
        self.regs[r(reu::REU_LO)] = addr as u8;
        self.regs[r(reu::REU_MID)] = (addr >> 8) as u8;
        self.regs[r(reu::REU_BANK)] = (addr >> 16) as u8;
        self.regs[r(reu::LEN_LO)] = 1;
        self.regs[r(reu::LEN_HI)] = 0;
    }
}

/// Render the game state `g` into `buf` and present it, as the C64 does (the palette is set up here too;
/// the C64 does that once at boot).
pub fn render_game(m: &mut Machine, img: &crate::render::Image, g: &crate::game::Game, buf: Buffer) -> crate::render::Scroll {
    use crate::{game, render};
    let mut sprites = [render::Sprite::default(); game::INSTANCES];
    let n = game::scene(g, &mut sprites);
    let scroll = game::view(g, img);
    let mut img = *img;
    img.background = render::background_at(m, &img, g.ticks as u8);
    let img = &img;
    render::set_palette(m, game::palette(g, img.palette));
    render::render_frame(m, img, buf, scroll, &mut sprites[..n]);
    render::present(m, buf, scroll);
    render::hud_init(m, img);
    render::hud_update(m, &game::hud_state(g));
    scroll
}
