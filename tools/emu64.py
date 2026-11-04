#!/usr/bin/env python3
"""Run the linked TALON HD PRG in a 6502 interpreter with just enough C64 around it to find faults that
only the 6502 build shows: `$01` banking of `$D000-$DFFF` (I/O or RAM), a REU that executes DMAs against
the level image, a raster counter and CIA 2 timers A+B driven by the cycle count, an idle joystick, and
the C64 Ultimate sampler's timing ($DF20/$DF40: mapping probe, one-shot completion, the repeating ring's
play position; audio.rs), and with `--level-dir` the Ultimate Command Interface's DOS target ($DF1C-$DF1F:
the per-level load, `src/uci.rs`, serving that directory's image files). No video, no IRQs (the HUD
multiplexer is not modelled; raster interrupts are delivered, and CIA1 timer A is modelled for the border
painter's line sync, so the IRQ chain's logic runs, not its timing).

At the end it checks the music: the producer stays ahead of the modelled sampler, and the ring holds the
track's samples as the encoder decodes them (output/audio/<cue>.preview.wav, rounded to 8 bits).

    python tools/emu64.py [--prg dist/talonhd.prg] [--reu dist/talonhd.reu] [--elf dist/talonhd.elf]
                          [--frames N] [--trace N] [--level-dir dist] [--device-dir P]

Stops on an illegal opcode, a write into the hardware stack page by DMA, a jump outside known code, or
after N frames (counted by `$DD00` buffer flips); prints the last instructions and DMAs.
"""
import argparse
import os
import re
import collections
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from native_cpu import Cpu, KernelError  # noqa: E402
from test_blit import wsl_path  # noqa: E402


# C64U costs in 64 MHz cycles (docs/validation.md, `c64u.py bench`): an I/O register write 1.26 us; a REU
# fetch ~9 us + 0.25 us a byte (the setup's own register writes are counted as I/O writes too: take ~1 us).
IO_WRITE = 77
DMA_SETUP, DMA_BYTE = 64, 16
PHASE_BANK = 0xFF0000  # paging: REU $FF0000 + 4 KiB x id
PHASE_NAMES = ["boss", "attract", "midboss", "boot", "ground", "audio", "tune", "ace", "motor", "stage", "s2",
               "s2gnd", "s2air", "astro", "bands"]  # paging::NAMES


def functions(elf):
    """Per section: sorted (start, end, name) of the ELF's function symbols."""
    cmd = ['wsl', '-d', 'Ubuntu-24.04', '--', 'bash', '-lc', f'~/.local/llvm-mos/bin/llvm-objdump -t -C {wsl_path(elf)}']
    out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
    secs = collections.defaultdict(list)
    for line in out.splitlines():
        m = re.match(r'^([0-9a-f]{8}) (.{7}) (\S+)\s+([0-9a-f]+) (.*)$', line)
        if m and int(m.group(4), 16) > 0 and 'F' in m.group(2):
            a = int(m.group(1), 16)
            secs[m.group(3)].append((a, a + int(m.group(4), 16), m.group(5)))
    for v in secs.values():
        v.sort()
    return secs


def report_profile(prof, elf, frames, calls=None):
    """cycles by (phase, pc) -> the hottest functions, in ms a frame of C64U time (approximate)."""
    import bisect
    secs = functions(elf)
    resident = sorted(f for n, v in secs.items() if not n.startswith('.phase.') for f in v)
    starts = [f[0] for f in resident]
    total = collections.Counter()
    for (phase, pc), cyc in prof.items():
        if 0x9000 <= pc < 0xA000 and phase is not None:
            fs = secs.get('.phase.' + PHASE_NAMES[phase], [])
            hit = next((f[2] for f in fs if f[0] <= pc < f[1]), None)
            key = f'[{PHASE_NAMES[phase]}] ' + (hit or f'${pc:04x}')
        else:
            i = bisect.bisect_right(starts, pc) - 1
            key = resident[i][2] if i >= 0 and pc < resident[i][1] else f'${pc:04x}'
        total[re.sub(r'::h[0-9a-f]{16}', '', key)] += cyc
    all_ms = sum(total.values()) / 64000 / frames
    print(f'profile over {frames} frames: {all_ms:.2f} ms a frame in all (IRQs and waits included)')
    for k, cyc in total.most_common(45):
        print(f'  {cyc / 64000 / frames:6.3f} ms  {k[:110]}')
    if calls:
        def where(phase, a):
            if 0x9000 <= a < 0xA000 and phase is not None:
                fs = secs.get('.phase.' + PHASE_NAMES[phase], [])
                return f'[{PHASE_NAMES[phase]}] ' + next((f[2] for f in fs if f[0] <= a < f[1]), f'${a:04x}')
            i = bisect.bisect_right(starts, a) - 1
            return resident[i][2] if i >= 0 and a < resident[i][1] else f'${a:04x}'
        by = collections.Counter()
        for (h, phase, ret), cyc in calls.items():
            by[f'{h} <- ' + re.sub(r'::h[0-9a-f]{16}', '', where(phase, ret))] += cyc
        print('runtime helpers by caller:')
        for k, cyc in by.most_common(20):
            print(f'  {cyc / 64000 / frames:6.3f} ms  {k[:110]}')


def symbols(elf):
    nm = Path.home() / '.local/llvm-mos/bin/llvm-nm'
    for cmd in ([str(nm), '-n', elf], ['wsl', '-d', 'Ubuntu-24.04', '--', 'bash', '-lc',
                                         f'~/.local/llvm-mos/bin/llvm-nm -n {wsl_path(elf)}']):
        try:
            out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
            break
        except (OSError, subprocess.CalledProcessError):
            continue
    syms = {}
    for line in out.splitlines():
        p = line.split()
        if len(p) == 3:
            syms[p[2]] = int(p[0], 16)
    return syms


class Uci:
    """The Ultimate Command Interface's DOS target as `src/uci.rs` drives it (`model::Uci` on the host):
    CHANGE_DIR (only `device_dir`, the PRG's `uci::LEVEL_DIR`; the files come from `folder`), OPEN_FILE,
    LOAD_REU, CLOSE_FILE, ABORT. Commands finish at once; replies come in 16-byte packets (DATA_MORE until
    the last), and replies and status strings are worded as the firmware's (1541ultimate dos.cc)."""
    CONTROL, COMMAND, RESPONSE, STATUS = 0xDF1C, 0xDF1D, 0xDF1E, 0xDF1F

    PACKET = 16

    def __init__(self, folder, reu, device_dir):
        self.folder, self.reu, self.device_dir = Path(folder), reu, device_dir.rstrip('/')
        self.cmd, self.reply, self.status = bytearray(), bytearray(), bytearray()
        self.more = []
        self.state, self.error, self.open, self.in_dir = 0, False, None, False
        self.log = []

    def read(self, a):
        if a == self.CONTROL:
            return (self.state | (8 if self.error else 0) | (0x80 if self.reply else 0)
                    | (0x40 if self.status else 0))
        if a == self.COMMAND:
            return 0xC9
        q = self.reply if a == self.RESPONSE else self.status
        return q.pop(0) if q else 0

    def write(self, a, v):
        if a == self.COMMAND:
            self.cmd.append(v)
        elif a == self.CONTROL:
            if v & 1:
                if self.state:
                    self.error = True
                    self.log.append('PUSH refused: not idle')
                else:
                    c, self.cmd = bytes(self.cmd), bytearray()
                    self.status += self.execute(c)
                    self.more = [self.reply[i:i + self.PACKET] for i in range(0, len(self.reply), self.PACKET)]
                    self.reply = bytearray(self.more.pop(0) if self.more else b'')
                    self.state = 0x30 if self.more else 0x20
            if v & 2:
                self.reply.clear()
                self.status.clear()
                if self.more:
                    self.reply = bytearray(self.more.pop(0))
                    self.state = 0x30 if self.more else 0x20
                else:
                    self.state = 0
            if v & 4:
                self.log.append('ABORT')
                self.reply.clear()
                self.status.clear()
                self.cmd.clear()
                self.more = []
                self.state = 0
            if v & 8:
                self.error = False

    def execute(self, c):
        text = lambda b: b.split(b'\0')[0].decode()
        if c[:2] == b'\x01\x11':
            path = text(c[2:]).rstrip('/')
            self.in_dir = path == self.device_dir
            self.log.append(f'CHANGE_DIR {path}: {"ok" if self.in_dir else "no such directory"}')
            return b'00,OK' if self.in_dir else b'83,NO SUCH DIRECTORY'
        if c[:2] == b'\x01\x02':
            name = text(c[3:])
            path = self.folder / name
            self.open = path if self.in_dir and path.is_file() else None
            self.log.append(f'OPEN_FILE {name} mode {c[2]}: {"ok" if self.open else "not found"}')
            return b'00,OK' if self.open else b'62,FILE NOT FOUND'
        if c[:2] == b'\x01\x21':
            if self.open is None:
                return b'85,NO FILE OPEN'
            addr = int.from_bytes(c[2:6], 'little') & 0xFFFFFF
            length = min(int.from_bytes(c[6:10], 'little') & 0x1FFFFFF, (1 << 24) - addr)
            data = self.open.read_bytes()[:length]
            self.reu[addr:addr + len(data)] = data
            self.log.append(f'LOAD_REU ${addr:06x} cap ${length:06x}: {len(data)} bytes')
            self.reply += f'${len(data):6x} BYTES LOADED TO REU ${addr:6x}'.encode()
            return b'00,OK'
        if c[:2] == b'\x01\x03':
            self.open = None
            self.log.append('CLOSE_FILE')
            return b'00,OK'
        raise KernelError(f'UCI command {c.hex()} not modelled')


class C64(Cpu):
    def __init__(self, ram, reu):
        super().__init__(ram, [])
        self.io = bytearray(0x1000)  # $D000-$DFFF as I/O
        self.reu = reu
        self.dmas = collections.deque(maxlen=40)
        self.flips = 0
        self.stack_low = 0xD000
        self.alarm = None
        self.one_shot_end = None
        self.play_start = None
        self.rate = 16000.0
        self.uci = None
        # CIA1 timer A (the border painter's line clock): latch, counter at the anchor, anchor (PHI2),
        # running; underflows before the anchor, and the count at the last $DC0D read.
        self.ta_latch, self.ta_c0, self.ta_t0, self.ta_run = 0xFFFF, 0xFFFF, 0, False
        self.ta_uf_acc, self.ta_uf_read = 0, 0
        self.last_line = -1
        self.irqs = 0
        # The border painter's $D020 stores, per block (set by main: border_lines' address).
        self.border_lines = None
        self.capture = None
        self.painted = {}
        # --profile: the paged phase (by its window fetch) and C64U time not modelled per instruction
        # (REU DMAs, I/O writes), in 64 MHz cycles, charged to the instruction that caused it.
        self.phase = None
        self.extra = 0

    def phi2(self):
        return self.cycles // 64

    def ta_value(self):
        k = self.phi2() - self.ta_t0
        if not self.ta_run or k <= self.ta_c0:
            return self.ta_c0 - (k if self.ta_run else 0)
        return self.ta_latch - (k - self.ta_c0 - 1) % (self.ta_latch + 1)

    def ta_underflows(self):
        k = self.phi2() - self.ta_t0
        if not self.ta_run or k <= self.ta_c0:
            return self.ta_uf_acc
        return self.ta_uf_acc + 1 + (k - self.ta_c0 - 1) // (self.ta_latch + 1)

    def ta_rebase(self):
        self.ta_uf_acc = self.ta_underflows()
        self.ta_c0, self.ta_t0 = self.ta_value(), self.phi2()

    def cia1_write(self, a, v):
        if a in (0xDC04, 0xDC05):
            self.ta_rebase()
            if a == 0xDC04:
                self.ta_latch = (self.ta_latch & 0xFF00) | v
            else:
                self.ta_latch = (self.ta_latch & 0xFF) | v << 8
                if not self.ta_run:
                    self.ta_c0 = self.ta_latch  # a high byte write while stopped loads the counter
        elif a == 0xDC0E:
            self.ta_rebase()
            if v & 0x10:
                self.ta_c0 = self.ta_latch
            self.ta_run = bool(v & 1)

    def poll_irq(self):
        """The VIC's raster interrupt: latched when the beam reaches $D012's line, delivered while the I flag
        is clear and $D01A enables it."""
        line = self.raster()
        if line != self.last_line:
            self.last_line = line
            if line == self.io[0x12] | (self.io[0x11] & 0x80) << 1:
                self.io[0x19] |= 0x81
        if self.io[0x19] & self.io[0x1A] & 1 and not self.p & 0x04:
            self.push(self.pc >> 8)
            self.push(self.pc & 0xFF)
            self.push((self.p | 0x20) & ~0x10)
            self.p |= 0x04
            self.pc = self.mem[0xFFFE] | self.mem[0xFFFF] << 8
            self.cycles += 7
            self.irqs += 1

    def tick(self):
        self.poll_irq()
        if self.pc == self.border_lines:
            self.capture = []
            self.capture_flips = self.flips
        self.step()

    def stored_border(self, v):
        """A $D020 store inside border_lines: 160 pixels and the corner per line, 20 lines a block (T or B by
        the block's first raster line, P_FIRST)."""
        self.capture.append(v & 15)
        if len(self.capture) == PAINT_LINES * 161:
            # Keyed by the flips before the block: the frame it shows (block B paints, then flips).
            kind = 'T' if self.mem[0xCAE0 + 20] == 31 else 'B'
            self.painted[(kind, self.capture_flips)] = bytes(c for i, c in enumerate(self.capture) if i % 161 != 160)
            self.capture = None

    def io_on(self):
        return self.mem[1] & 3 != 0 and self.mem[1] & 4 != 0

    def raster(self):
        return (self.cycles // 4160) % 263  # 65 cycles a line at 64x speed

    def read(self, a):
        a &= 0xFFFF
        if 0xD000 <= a < 0xE000 and self.io_on():
            r = a - 0xD000
            if a == 0xD012:
                return self.raster() & 0xFF
            if a == 0xD011:
                return (self.io[0x11] & 0x7F) | (0x80 if self.raster() > 255 else 0)
            if a == 0xDC04:
                return self.ta_value() & 0xFF
            if a == 0xDC05:
                return self.ta_value() >> 8 & 0xFF
            if a == 0xDC0D:
                n = self.ta_underflows()
                flag = n > self.ta_uf_read
                self.ta_uf_read = n
                return 0x01 if flag else 0x00
            if 0xDD04 <= a <= 0xDD07:
                # Timers A and B chained: one 32-bit down-counter at 1 MHz.
                t = 0xFFFFFFFF - (self.cycles // 64) % 0x100000000
                return (t >> (8 * (a - 0xDD04))) & 0xFF
            if a == 0xDF21:
                return 0x10  # the sampler's mapping probe
            if a == 0xDF20:
                return int(self.one_shot_end is not None and self.cycles >= self.one_shot_end)
            if a in (0xDC00, 0xDC01):
                return 0xFF
            if a == 0xDF00:
                return 0x40 | 0x10  # end of block, 256k+ REU
            if 0xDF1C <= a <= 0xDF1F and self.uci:
                return self.uci.read(a)
            return self.io[r]
        return self.mem[a]

    def write(self, a, v):
        a &= 0xFFFF
        v &= 0xFF
        if 0xD000 <= a < 0xE000 and self.io_on():
            self.extra += IO_WRITE
            if a == 0xD019:
                self.io[0x19] &= ~v & 0xFF  # acknowledge
                return
            if 0xDC04 <= a <= 0xDC0F:
                self.cia1_write(a, v)
            if a == 0xD020 and self.capture is not None:
                self.stored_border(v)
            self.io[a - 0xD000] = v
            if 0xDF1C <= a <= 0xDF1F and self.uci:
                self.uci.write(a, v)
            if a == 0xDF01 and v & 0x80:
                self.dma(v)
            if a == 0xDD00:
                self.flips += 1
            if a == 0xDF20:
                self.sampler_gate(v)
            return
        if 0xCE30 <= a < 0xD000 and a < self.stack_low:
            # The soft stack's high-water mark (it grows down from $D000 toward the HUD table).
            self.stack_low = a
        self.mem[a] = v

    def sampler_gate(self, v):
        r = self.io
        if not v & 1:
            self.one_shot_end = None
            return
        divider = r[0xF2E] << 8 | r[0xF2F]
        self.rate = 6_250_000 / (divider + 2)
        length = r[0xF29] << 16 | r[0xF2A] << 8 | r[0xF2B]
        frames = length // 4 if v & 0x10 else length
        if v & 2:
            self.play_start = self.cycles
        else:
            self.one_shot_end = self.cycles + int(frames / self.rate * 64_000_000)

    def played(self):
        return int((self.cycles - self.play_start) / 64_000_000 * self.rate) if self.play_start is not None else 0

    def dma(self, cmd):
        r = self.io
        c64 = r[0xF02] | r[0xF03] << 8
        reu = r[0xF04] | r[0xF05] << 8 | r[0xF06] << 16
        n = (r[0xF07] | r[0xF08] << 8) or 0x10000
        kind = cmd & 3
        self.dmas.append((self.pc, ['stash', 'fetch', 'swap', 'verify'][kind], c64, reu, n))
        self.extra += DMA_SETUP + n * DMA_BYTE
        if kind == 1 and c64 == 0x9000 and PHASE_BANK <= reu < PHASE_BANK + 15 * 0x1000:
            self.phase = (reu - PHASE_BANK) >> 12
        for i in range(n):
            ca, ra = (c64 + i) & 0xFFFF, (reu + i) & 0xFFFFFF
            if kind == 0:
                self.reu[ra] = self.read(ca)
            elif kind == 1:
                if 0x100 <= ca < 0x200 and self.alarm is None:
                    self.alarm = f'DMA wrote the stack page: {self.dmas[-1]}'
                self.write(ca, self.reu[ra])
        # The registers advance like the real REU's.
        c64e, reue = (c64 + n) & 0xFFFF, (reu + n) & 0xFFFFFF
        r[0xF02], r[0xF03] = c64e & 0xFF, c64e >> 8
        r[0xF04], r[0xF05], r[0xF06] = reue & 0xFF, (reue >> 8) & 0xFF, reue >> 16
        r[0xF07], r[0xF08] = 1, 0


# audio::Player at $C800 (#[repr(C)], 6502 alignment 1): offsets of the fields checked here.
PLAYER = 0xC800
P_PRODUCED, P_PLAYED, P_TICKS, P_STATE, P_FAULT, P_UNDERRUNS = 120, 124, 128, 138, 139, 140
RING, RING_SAMPLES, GUARD = 0xFE8000, 32768, 2048


def check_music(cpu, reu, wav='output/audio/cue.stage_s1.preview.wav'):
    m = cpu.mem
    u32 = lambda o: int.from_bytes(m[PLAYER + o:PLAYER + o + 4], 'little')
    state, fault, under = m[PLAYER + P_STATE], m[PLAYER + P_FAULT], m[PLAYER + P_UNDERRUNS]
    produced, played, ticks = u32(P_PRODUCED), u32(P_PLAYED), u32(P_TICKS)
    pos = cpu.played()
    print(f'music: state {state} fault {fault} underruns {under}, clock {ticks} ticks/12288, produced {produced}, '
          f'player estimate {played}, sampler at {pos}')
    if state != 1:
        return
    if not pos < produced <= pos + RING_SAMPLES - GUARD + 512:
        print(f'MUSIC FAULT: the ring is not ahead of the sampler (lead {produced - pos})')
    try:
        pcm = Path(wav).read_bytes()[44:]
    except OSError:
        return
    total = len(pcm) // 2
    bad = 0
    for k in range(max(0, produced - (RING_SAMPLES - GUARD)), produced):
        s = int.from_bytes(pcm[2 * (k % total):2 * (k % total) + 2], 'little', signed=True)
        want = max(-128, min(127, (s + 128) >> 8)) & 0xFF
        bad += reu[RING + (k & (RING_SAMPLES - 1))] != want
    print(f'music: {bad} of {RING_SAMPLES - GUARD} ring samples differ from the encoder decode')


STATUS, STATUS_LEN, MAGIC = 0xCA80, 32, 0xC1
PAINT_LINES = 24
# layout::BUFFERS (bitmap, screen) by status FRONT; layout::band_set: the REU set each buffer's bands use.
BUFFERS = [(0x6000, 0x4C00), (0xA000, 0x8C00)]
BAND_REU, BAND_SET, BAND_BYTES = 0xFFF000, 0x800, 2 * 24 * 40  # layout::BAND_SET_BYTES (line-major)


def write_dump(cpu, reu, out):
    """Run on until the flip's status block is written (FLIPPING 0), then dump the shown frame as
    `c64u.py dump` does (status, bitmap, screen, color RAM) and the shown buffer's bands from the REU."""
    for _ in range(30_000_000):
        st = cpu.mem[STATUS:STATUS + STATUS_LEN]
        if st[0] == MAGIC and st[1] == 0:
            break
        cpu.tick()
    else:
        raise SystemExit('dump: the status block never settled')
    front = st[2] & 1
    bitmap, screen = BUFFERS[front]
    out.mkdir(parents=True, exist_ok=True)
    (out / 'status.bin').write_bytes(bytes(st))
    (out / 'bitmap.bin').write_bytes(bytes(cpu.mem[bitmap:bitmap + 8000]))
    (out / 'screen.bin').write_bytes(bytes(cpu.mem[screen:screen + 1000]))
    (out / 'color.bin').write_bytes(bytes(cpu.io[0x800:0x800 + 1000]))
    for stale in ('bands.bin', 'painted.bin'):
        (out / stale).unlink(missing_ok=True)
    if not cpu.mem[0xCAE0]:
        # The canvas is off (status::CANVAS 0, the default; `--painter` turns it on): no bands.
        print(f'dump: tick {st[4] | st[5] << 8}, front {front}, painter off -> {out}')
        return
    set_ = BAND_REU + front * BAND_SET
    (out / 'bands.bin').write_bytes(bytes(reu[set_:set_ + BAND_BYTES]))
    # What the painter shows of this frame: the next block T and block B (B paints before any flip).
    if cpu.border_lines is not None and cpu.mem[0xCAE0]:
        f = cpu.flips
        want = [('T', f), ('B', f)]
        for _ in range(30_000_000):
            if all(k in cpu.painted for k in want):
                break
            cpu.tick()
        if all(k in cpu.painted for k in want):
            (out / 'painted.bin').write_bytes(cpu.painted[want[0]] + cpu.painted[want[1]])
        else:
            print('dump: the painter did not paint both blocks of the frame')
    print(f'dump: tick {st[4] | st[5] << 8}, front {front} -> {out}')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--prg', default='dist/talonhd.prg')
    ap.add_argument('--reu', default='dist/talonhd.reu')
    ap.add_argument('--elf', default='dist/talonhd.elf')
    ap.add_argument('--frames', type=int, default=60)
    ap.add_argument('--trace', type=int, default=40)
    ap.add_argument('--steps', type=int, default=60_000_000)
    ap.add_argument('--progress', type=int, default=0)
    ap.add_argument('--level-dir', help='serve the per-level load (UCI) from this directory')
    ap.add_argument('--device-dir', default=os.environ.get('TALONHD_DIR', '/Usb0/talonhd'),
                    help="the device directory the UCI answers to (the PRG's TALONHD_DIR; default /Usb0/talonhd)")
    ap.add_argument('--garbage', action='store_true',
                    help='start with RAM full of a pseudo-random pattern, as a real C64 after a reset (state the '
                         'program forgets to initialise shows up)')
    ap.add_argument('--painter', action='store_true',
                    help='turn the border painter on (status::CANVAS, as F1 does) once the main loop runs')
    ap.add_argument('--profile', type=int, metavar='FLIP',
                    help='from this flip on, charge every instruction (and the modelled C64U REU-DMA and I/O-write '
                         'costs) to its function: the hottest per frame at the end')
    ap.add_argument('--dump', help="after the last flip, write a dump like `c64u.py dump` (plus the shown "
                                   "buffer's border bands, bands.bin) to this directory, for `talonhd compare`")
    args = ap.parse_args()
    syms = symbols(args.elf)
    addr2sym = sorted((v, k) for k, v in syms.items())

    def name(a):
        import bisect
        i = bisect.bisect_right([x for x, _ in addr2sym], a) - 1
        return f'{addr2sym[i][1][:60]}+{a - addr2sym[i][0]:#x}' if i >= 0 else '?'

    prg = Path(args.prg).read_bytes()
    ram = bytearray(65536)
    if args.garbage:
        import random
        ram[:] = random.Random(64).randbytes(65536)
    base = prg[0] | prg[1] << 8
    ram[base:base + len(prg) - 2] = prg[2:]
    ram[1] = 0x37
    reu = bytearray(16 << 20)
    image = Path(args.reu).read_bytes()
    cpu = C64(ram, reu)
    if args.level_dir:
        cpu.uci = Uci(args.level_dir, reu, args.device_dir)
    # The PRG waits for the image after erasing its magic: install it once the erase has happened.
    # Install on the first header read, after the erase (the `bench` build has none).
    erase = next((v for k, v in syms.items() if 'read_image' in k), -1)
    verify = next((v for k, v in syms.items() if 'verify_image' in k), -1)
    installed = False
    cpu.border_lines = syms.get('border_lines')
    cpu.pc = syms['_start']
    last = collections.deque(maxlen=args.trace)
    steps = 0
    prof, prof_from = None, 0
    want_painter = args.painter
    helpers = {v: k for k, v in syms.items() if k in ('__mulhi3', '__udivhi3', '__divhi3', '__memset', '__mulsi3',
                                                     '__udivsi3', '__divsi3', '__udivmodhi4', '__umodhi3', 'memcpy')}
    helper, calls = None, {}
    while cpu.flips < args.frames and steps < args.steps:
        if args.progress and steps % args.progress == 0:
            print(f'  step {steps}: ${cpu.pc:04x} {name(cpu.pc)} flips {cpu.flips} installed {installed}')
        if cpu.pc == verify:
            # Skip the whole-image checksum (minutes in Python): pretend it passed. Once only: the
            # address belongs to the BOOT phase, and other phases reuse the window afterwards.
            if not installed:
                reu[:len(image)] = image
                installed = True
            verify = -1
            cpu.a = 1
            lo, hi = cpu.pop(), cpu.pop()
            cpu.pc = ((hi << 8 | lo) + 1) & 0xFFFF
            continue
        if cpu.pc == erase and not installed and verify != -1:
            reu[:len(image)] = image
            installed = True
        if want_painter and cpu.flips >= 2:
            cpu.mem[0xCA80 + 31] = 1  # layout::status::CANVAS
            want_painter = False
        last.append(cpu.pc)
        pc, c0 = cpu.pc, cpu.cycles
        cpu.extra = 0
        try:
            cpu.tick()
        except KernelError as e:
            print('FAULT', e)
            break
        steps += 1
        if prof is not None:
            key = (cpu.phase, pc)
            spent = cpu.cycles - c0 + cpu.extra
            prof[key] = prof.get(key, 0) + spent
            # Runtime helpers (multiply, divide, memset) charged to their callers too.
            if helper is None and pc in helpers:
                ret = cpu.mem[0x100 + ((cpu.s + 1) & 0xFF)] | cpu.mem[0x100 + ((cpu.s + 2) & 0xFF)] << 8
                helper = (helpers[pc], cpu.phase, ret, (ret + 1) & 0xFFFF)
            if helper is not None:
                k = (helper[0], helper[1], helper[2])
                calls[k] = calls.get(k, 0) + spent
                if cpu.pc == helper[3]:
                    helper = None
        elif args.profile is not None and cpu.flips >= args.profile:
            prof, prof_from = {}, cpu.flips
        if cpu.alarm:
            print('FAULT', cpu.alarm)
            break
    else:
        print(f'ran {cpu.flips} flips, {steps} instructions, {cpu.irqs} interrupts, no fault')
        pb = 0xCAE0  # layout::PAINTER
        m = cpu.mem
        print(f'border painter: on {m[pb]}, delta {m[pb + 12]}, blocks skipped {m[pb + 15] | m[pb + 16] << 8}, '
              f'gate {m[pb + 1]:02x}, next stage {m[pb + 2]}')
        print(f'soft stack: deepest write ${cpu.stack_low:04x} ({0xD000 - cpu.stack_low} bytes below $D000, '
              f'limit 464: the static frames end at $CE30)')
        if cpu.uci:
            print('level load:', '; '.join(cpu.uci.log) or 'none', f'(the REU holds level {reu[63]})')
        # The stage's cue: the image's level is header byte 63 (`format::header::LEVEL`).
        check_music(cpu, reu, f'output/audio/cue.stage_s{reu[63] + 1}.preview.wav')
        if prof:
            report_profile(prof, args.elf, cpu.flips - prof_from, calls)
        if args.dump:
            write_dump(cpu, reu, Path(args.dump))
        return
    print('last instructions:')
    for pc in list(last)[-args.trace:]:
        print(f'  ${pc:04x} {name(pc)}')
    print('last DMAs (pc, kind, c64, reu, len):')
    for d in list(cpu.dmas)[-12:]:
        print(f'  ${d[0]:04x} {name(d[0])[:40]} {d[1]} c64=${d[2]:04x} reu=${d[3]:06x} n={d[4]}')
    print(f'flips {cpu.flips}, $01={cpu.mem[1]:02x}, SP={cpu.s:02x}')


if __name__ == '__main__':
    main()
