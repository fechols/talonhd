#!/usr/bin/env python3
"""Run the linked `draw_column_asm` (from dist/talonhd.prg) in a 6502
interpreter with a modeled REU, and compare it with a direct port of the Rust
reference kernel `talonhd::render::draw_column` on randomized columns.

    python tools/test_blit.py [dist/talonhd.prg] [dist/talonhd.elf]
"""
import random
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from native_cpu import Cpu  # noqa: E402

JOB, SEG_BUF = 0xCA00, 0xC000
SENTINEL = 0xFFFF


def symbol(elf, name):
    nm = Path.home() / '.local/llvm-mos/bin/llvm-nm'
    for cmd in ([str(nm), elf], ['wsl', '-d', 'Ubuntu-24.04', '--', 'bash', '-lc',
                                  f'~/.local/llvm-mos/bin/llvm-nm {wsl_path(elf)}']):
        try:
            out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
            break
        except (OSError, subprocess.CalledProcessError):
            continue
    for line in out.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[2] == name:
            return int(parts[0], 16)
    raise SystemExit(f'{name} not in {elf}')


def wsl_path(p):
    p = str(Path(p).resolve())
    return '/mnt/' + p[0].lower() + p[2:].replace('\\', '/')


def line_offset(y):
    return (y >> 3) * 320 + (y & 7)


class ReuCpu(Cpu):
    def __init__(self, memory, reu):
        super().__init__(memory, [(0xF0, 0x100), (0x100, 0x200), (0x6000, 0x8000), (0xDF00, 0xDF10)])
        self.reu = reu
        self.dmas = 0

    def write(self, address, value):
        super().write(address, value)
        if address != 0xDF01:
            return
        m = self.mem
        assert value == 0x91, hex(value)
        c64 = m[0xDF02] | m[0xDF03] << 8
        addr = m[0xDF04] | m[0xDF05] << 8 | m[0xDF06] << 16
        n = (m[0xDF07] | m[0xDF08] << 8) or 65536
        assert m[0xDF0A] == 0
        assert 0x6000 <= c64 and c64 + n <= 0x8000, hex(c64)
        for i in range(n):
            m[c64 + i] = self.reu[addr + i]
        addr += n
        m[0xDF02:0xDF04] = (c64 + n).to_bytes(2, 'little')
        m[0xDF04], m[0xDF05], m[0xDF06] = addr & 255, addr >> 8 & 255, addr >> 16
        m[0xDF07], m[0xDF08] = 1, 0
        self.dmas += 1


def reference(ram, reu, dst, y, top, bot, data, segs):
    for b0, b1, b2, b3 in segs:
        line = b0 | (b1 & 1) << 8
        if line >= bot:
            break
        if b1 & 0x80:
            if line >= top:
                a = dst + line_offset((y + line) & 0xFFFF)
                ram[a] = ram[a] & b3 | b2
        else:
            s, e = max(line, top), min(line + (b2 | b3 << 8), bot)
            src = data + s
            for l in range(s, e):
                ram[dst + line_offset((y + l) & 0xFFFF)] = reu[src]
                src += 1


def random_case(rng):
    h = rng.choice([1, 7, 8, 9, 64, 192, 300, 512])
    y = rng.randint(-h - 2, 202)
    top, bot = max(0, -y), min(h, 200 - y)
    segs, line = [], 0
    while line < h:
        line += rng.choice([0, 0, 1, 3, 9])
        if line >= h:
            break
        if rng.random() < 0.4:
            segs.append([line & 255, line >> 8 | 0x80, rng.randrange(256), rng.randrange(256)])
            line += 1
        else:
            n = min(rng.randint(1, 70), h - line)
            segs.append([line & 255, line >> 8, n & 255, n >> 8])
            line += n
    col = rng.randrange(40)
    return dict(dst=0x6000 + col * 8, y=y, top=top, bot=bot, h=h, segs=segs[:512])


def main():
    prg = Path(sys.argv[1] if len(sys.argv) > 1 else 'dist/talonhd.prg').read_bytes()
    elf = sys.argv[2] if len(sys.argv) > 2 else 'dist/talonhd.elf'
    entry = symbol(elf, 'draw_column_asm')
    base = prg[0] | prg[1] << 8
    rng = random.Random(1)
    reu = bytes(rng.randrange(256) for _ in range(1 << 20))
    for case_no in range(400):
        c = random_case(rng)
        if c['top'] >= c['bot']:
            continue
        data = rng.randrange(0x1000, 0xF0000)
        mem = bytearray(65536)
        mem[base:base + len(prg) - 2] = prg[2:]
        for a in range(0x6000, 0x8000):
            mem[a] = rng.randrange(256)
        want = bytearray(mem)
        job = bytearray(16)
        job[0:2] = c['dst'].to_bytes(2, 'little')
        job[2:4] = (c['y'] & 0xFFFF).to_bytes(2, 'little')
        job[4:6] = c['top'].to_bytes(2, 'little')
        job[6:8] = c['bot'].to_bytes(2, 'little')
        job[8:11] = data.to_bytes(3, 'little')
        job[11:13] = len(c['segs']).to_bytes(2, 'little')
        mem[JOB:JOB + 16] = job
        for i, s in enumerate(c['segs']):
            mem[SEG_BUF + 4 * i:SEG_BUF + 4 * i + 4] = bytes(s)
        reference(want, reu, c['dst'], c['y'], c['top'], c['bot'], data, c['segs'])
        cpu = ReuCpu(mem, reu)
        cpu.pc = entry
        cpu.push((SENTINEL - 1) >> 8)
        cpu.push((SENTINEL - 1) & 255)
        for _ in range(2_000_000):
            cpu.step()
            if cpu.pc == SENTINEL:
                break
        else:
            raise SystemExit(f'case {case_no}: no return')
        diff = [a for a in range(0x6000, 0x8000) if mem[a] != want[a]]
        if diff:
            a = diff[0]
            raise SystemExit(f'case {case_no} {c["h"]=} {c["y"]=} {c["top"]=} {c["bot"]=} '
                             f'{len(c["segs"])} segs: {len(diff)} bytes differ, first ${a:04x} '
                             f'got {mem[a]:02x} want {want[a]:02x}; segs {c["segs"][:6]}')
    print('draw_column_asm matches the reference on 400 random columns')


if __name__ == '__main__':
    main()
