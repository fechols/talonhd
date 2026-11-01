#!/usr/bin/env python3
"""Run the linked TALON HD PRG in a 6502 interpreter with just enough C64 around it to find faults that
only the 6502 build shows: `$01` banking of `$D000-$DFFF` (I/O or RAM), a REU that executes DMAs against
the level image, a raster counter and CIA 2 timer driven by the cycle count, and an idle joystick. No
video, no IRQs (the HUD multiplexer is not modelled), no sound.

    python tools/emu64.py [--prg dist/talonhd.prg] [--reu dist/talonhd.reu] [--elf dist/talonhd.elf]
                          [--frames N] [--trace N]

Stops on an illegal opcode, a write into the hardware stack page by DMA, a jump outside known code, or
after N frames (counted by `$DD00` buffer flips); prints the last instructions and DMAs.
"""
import argparse
import collections
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from native_cpu import Cpu, KernelError  # noqa: E402
from test_blit import wsl_path  # noqa: E402


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


class C64(Cpu):
    def __init__(self, ram, reu):
        super().__init__(ram, [])
        self.io = bytearray(0x1000)  # $D000-$DFFF as I/O
        self.reu = reu
        self.dmas = collections.deque(maxlen=40)
        self.flips = 0
        self.alarm = None

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
            if a in (0xDD04, 0xDD05):
                t = 0xFFFF - (self.cycles // 64) % 0x10000
                return t & 0xFF if a == 0xDD04 else t >> 8
            if a in (0xDC00, 0xDC01):
                return 0xFF
            if a == 0xDF00:
                return 0x40 | 0x10  # end of block, 256k+ REU
            return self.io[r]
        return self.mem[a]

    def write(self, a, v):
        a &= 0xFFFF
        v &= 0xFF
        if 0xD000 <= a < 0xE000 and self.io_on():
            self.io[a - 0xD000] = v
            if a == 0xDF01 and v & 0x80:
                self.dma(v)
            if a == 0xDD00:
                self.flips += 1
            return
        self.mem[a] = v

    def dma(self, cmd):
        r = self.io
        c64 = r[0xF02] | r[0xF03] << 8
        reu = r[0xF04] | r[0xF05] << 8 | r[0xF06] << 16
        n = (r[0xF07] | r[0xF08] << 8) or 0x10000
        kind = cmd & 3
        self.dmas.append((self.pc, ['stash', 'fetch', 'swap', 'verify'][kind], c64, reu, n))
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


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--prg', default='dist/talonhd.prg')
    ap.add_argument('--reu', default='dist/talonhd.reu')
    ap.add_argument('--elf', default='dist/talonhd.elf')
    ap.add_argument('--frames', type=int, default=60)
    ap.add_argument('--trace', type=int, default=40)
    ap.add_argument('--steps', type=int, default=60_000_000)
    ap.add_argument('--progress', type=int, default=0)
    args = ap.parse_args()
    syms = symbols(args.elf)
    addr2sym = sorted((v, k) for k, v in syms.items())

    def name(a):
        import bisect
        i = bisect.bisect_right([x for x, _ in addr2sym], a) - 1
        return f'{addr2sym[i][1][:60]}+{a - addr2sym[i][0]:#x}' if i >= 0 else '?'

    prg = Path(args.prg).read_bytes()
    ram = bytearray(65536)
    base = prg[0] | prg[1] << 8
    ram[base:base + len(prg) - 2] = prg[2:]
    ram[1] = 0x37
    reu = bytearray(16 << 20)
    image = Path(args.reu).read_bytes()
    cpu = C64(ram, reu)
    # The PRG waits for the image after erasing its magic: install it once the erase has happened.
    erase = next(v for k, v in syms.items() if 'read_image' in k)  # install on the first header read, after the erase
    verify = next(v for k, v in syms.items() if 'verify_image' in k)
    installed = False
    cpu.pc = syms['_start']
    last = collections.deque(maxlen=args.trace)
    steps = 0
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
        last.append(cpu.pc)
        try:
            cpu.step()
        except KernelError as e:
            print('FAULT', e)
            break
        steps += 1
        if cpu.alarm:
            print('FAULT', cpu.alarm)
            break
    else:
        print(f'ran {cpu.flips} flips, {steps} instructions, no fault')
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
