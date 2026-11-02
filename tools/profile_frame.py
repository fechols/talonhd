#!/usr/bin/env python3
"""Where a frame's sprite time goes: replay attract-mode frames' sprite lists (from
`cargo run --release --example scene_dump -- <ticks>`) through the linked `draw_sprite_asm` in the 6502
interpreter with the REU model (`test_sprite.py`), in draw order with the cache carried across sprites, and
price each sprite with the C64U's measured unit costs (`docs/validation.md`): a CPU cycle at 64 MHz, a REU
register write 1.26 us, a DMA byte 0.25 us.

    cargo run --release --example scene_dump -- 10300 10765 > scenes.txt
    python tools/profile_frame.py scenes.txt
"""
import collections
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from test_blit import symbol  # noqa: E402
from test_sprite import BITMAP, SENTINEL, SP, ReuCpu, le  # noqa: E402

CYCLE_US = 1 / 64.0
IO_US = 1.26
BYTE_US = 0.25


class CountingCpu(ReuCpu):
    def __init__(self, memory, reu):
        super().__init__(memory, reu)
        self.io = 0
        self.bytes = 0

    def write(self, address, value):
        if 0xDF00 <= address < 0xDF10:
            self.io += 1
            if address == 0xDF01:
                self.bytes += (self.mem[0xDF07] | self.mem[0xDF08] << 8) or 65536
        super().write(address, value)


def main():
    scenes = collections.defaultdict(list)
    for line in Path(sys.argv[1]).read_text().split('\n'):
        if line.strip():
            t, f, x, y = map(int, line.split())
            scenes[t].append((f, x, y))
    prg = Path('dist/talonhd.prg').read_bytes()
    image = Path('dist/talonhd.reu').read_bytes()
    entry = symbol('dist/talonhd.elf', 'draw_sprite_asm')
    reu = bytearray(16 << 20)
    reu[:len(image)] = image
    directory, frames = le(image, 20, 3), le(image, 18, 2)
    base = prg[0] | prg[1] << 8
    mem = bytearray(65536)
    mem[base:base + len(prg) - 2] = prg[2:]
    mem[0xE000:0xE410] = bytes(0x410)
    names = [(0, 'boss body'), (49, 'arm'), (147, 'leg'), (343, 'drone'), (359, 'talon'), (368, 'gannet'),
             (378, 'pair'), (381, 'round'), (382, 'bomb'), (383, 'burst'), (386, 'spark'), (390, 'shield'),
             (462, 'keel'), (467, 'turret'), (483, 'skiff'), (499, 'pillbox'), (531, 'hatch'), (537, 'crawler'),
             (569, 'wreck')]

    def kind(f):
        return [n for s, n in names if f >= s][-1]

    for t in sorted(scenes):
        rows, total = [], 0.0
        by_kind = collections.defaultdict(float)
        for f, x, y in scenes[t]:
            if f >= frames:
                continue
            job = [f & 255, f >> 8, x & 255, (x >> 8) & 255, y & 255, (y >> 8) & 255, BITMAP & 255, BITMAP >> 8,
                   directory & 255, directory >> 8 & 255, directory >> 16]
            mem[SP:SP + len(job)] = bytes(job)
            cpu = CountingCpu(mem, reu)
            cpu.pc = entry
            cpu.push((SENTINEL - 1) >> 8)
            cpu.push((SENTINEL - 1) & 255)
            while cpu.pc != SENTINEL:
                cpu.step()
            us = cpu.cycles * CYCLE_US + cpu.io * IO_US + cpu.bytes * BYTE_US
            total += us
            by_kind[kind(f)] += us
            rows.append((us, kind(f), f, x, y, cpu.cycles, cpu.io, cpu.bytes, cpu.dmas))
        print(f'tick {t}: {len(rows)} sprites, {total / 1000:.2f} ms estimated')
        print('  by kind: ' + ', '.join(f'{k} {v / 1000:.2f}' for k, v in sorted(by_kind.items(), key=lambda kv: -kv[1])))
        for us, k, f, x, y, cyc, io, nb, dm in sorted(rows, reverse=True)[:10]:
            print(f'    {us:7.0f} us  {k:9} frame {f:3} at ({x:4},{y:4})  cpu {cyc * CYCLE_US:6.0f} us  '
                  f'io {io:3} ({io * IO_US:5.0f} us)  dma {nb:5} B in {dm:2} ({nb * BYTE_US:5.0f} us)')


if __name__ == '__main__':
    main()
