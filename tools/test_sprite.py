#!/usr/bin/env python3
"""Run the linked `draw_sprite_asm` (dist/talonhd.prg) in a 6502 interpreter with a modeled REU holding
the baked level image (dist/talonhd.reu), and compare every bitmap and band byte with a direct port of the Rust
reference `talonhd::render::draw_sprite` (which reads the REU, no cache) for random frames, positions and
targets (the bitmap, 200 lines; a border band's rows, 24), clipped on every edge, and its SP_H. One memory image persists across cases, so the small-sprite cache is exercised
(hits, misses, evictions).

    python tools/test_sprite.py [cases]
"""
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from native_cpu import Cpu  # noqa: E402
from test_blit import line_offset, symbol  # noqa: E402

SP, SENTINEL, BITMAP = 0xCAA0, 0xFFFF, 0x6000
SP_HEIGHT, SP_H = 0xCAD7, 0xCAAF
# The border painter's bands as composed (layout::BANDS): the top band's 3 rows, then the bottom band's.
BANDS, BAND_BYTES = 0x9800, 960
# Targets: (address, lines) - the bitmap, or a band's rows.
TARGETS = [(BITMAP, 200), (BANDS, 24), (BANDS + BAND_BYTES, 24)]
FRAME_RECORD = 32


class ReuCpu(Cpu):
    def __init__(self, memory, reu):
        super().__init__(memory, [(0xF0, 0x100), (0x100, 0x200), (BITMAP, BITMAP + 8000), (BANDS, BANDS + 2 * BAND_BYTES), (0xC000, 0xC800),
                                  (0xCA00, 0xCB00), (0xE000, 0xE410), (0xDF00, 0xDF10)])
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
        assert n <= 256, f'a {n}-byte DMA (MAX_DMA is 256)'
        assert any(a <= c64 and c64 + n <= b for a, b in [(0xC000, 0xC800), (0xCA20, 0xCA60), (0xE000, 0xE410)]), hex(c64)
        assert (addr >> 19) == ((addr + n - 1) >> 19), 'a DMA across a 512 KiB bank'
        m[c64:c64 + n] = self.reu[addr:addr + n]
        addr += n
        m[0xDF02:0xDF04] = (c64 + n).to_bytes(2, 'little')
        m[0xDF04], m[0xDF05], m[0xDF06] = addr & 255, addr >> 8 & 255, addr >> 16
        m[0xDF07], m[0xDF08] = 1, 0
        self.dmas += 1


def le(b, at, n):
    return int.from_bytes(b[at:at + n], 'little')


def reference(ram, reu, directory, frame, x, y, target=BITMAP, height=200):
    """Returns the frame's height (what draw_sprite_asm leaves in SP_H)."""
    rec = reu[directory + frame * FRAME_RECORD:][:FRAME_RECORD]
    h, pc, shift, nb = le(rec, 2, 2), rec[4], rec[5], rec[6]
    p = 0 if pc == 1 else x & 3
    ph = 8 + 6 * p
    wb, block = rec[ph], le(rec, ph + 1, 3)
    base = x >> 2
    j0 = -base if base < 0 else 0
    j1 = min(wb, 40 - base)
    if j0 >= j1:
        return h
    top, bot = (-y if y < 0 else 0), min(h, height - y)
    if top >= bot:
        return h
    for b in range(top >> shift, ((bot - 1) >> shift) + 1):
        band = block + le(reu, block + 3 * b, 3)
        for j in range(j0, j1):
            src = band + le(reu, band + 2 * j, 2)
            dst = target + (base + j) * 8
            items = reu[src]
            src += 1
            for _ in range(items):
                b0, b1 = reu[src], reu[src + 1]
                line = b0 | (b1 & 1) << 8
                if b1 & 0x80:
                    if top <= line < bot:
                        a = dst + line_offset(y + line)
                        ram[a] = ram[a] & reu[src + 3] | reu[src + 2]
                    src += 4
                else:
                    n = reu[src + 2]
                    for l in range(max(line, top), min(line + n, bot)):
                        ram[dst + line_offset(y + l)] = reu[src + 3 + l - line]
                    src += 3 + n
    return h


def main():
    cases = int(sys.argv[1]) if len(sys.argv) > 1 else 300
    prg = Path('dist/talonhd.prg').read_bytes()
    # Any level's image: python tools/test_sprite.py [cases] [image]
    image = Path(sys.argv[2] if len(sys.argv) > 2 else 'dist/talonhd.reu').read_bytes()
    entry = symbol('dist/talonhd.elf', 'draw_sprite_asm')
    reu = bytearray(16 << 20)
    reu[:len(image)] = image
    directory, frames = le(image, 20, 3), le(image, 18, 2)
    base_addr = prg[0] | prg[1] << 8
    mem = bytearray(65536)
    mem[base_addr:base_addr + len(prg) - 2] = prg[2:]
    mem[0xE000:0xE410] = bytes(0x410)  # the cache, as reset_sprite_cache leaves it (tags 0)
    rng = random.Random(7)
    small = list(range(378, 390))  # shots, bullets, bombs, bursts, SPARKs (demo.rs PAIR_FRAME..SHIELD_FRAME)
    cycles = dmas = 0
    for case in range(cases):
        frame = rng.choice(small) if rng.random() < 0.5 else rng.randrange(frames)
        target, height = TARGETS[0] if rng.random() < 0.6 else rng.choice(TARGETS[1:])
        x, y = rng.randint(-640, 330), rng.randint(-560, height + 15)
        for a in list(range(BITMAP, BITMAP + 8000)) + list(range(BANDS, BANDS + 2 * BAND_BYTES)):
            mem[a] = rng.randrange(256)
        job = [frame & 255, frame >> 8, x & 255, (x >> 8) & 255, y & 255, (y >> 8) & 255,
               target & 255, target >> 8, directory & 255, directory >> 8 & 255, directory >> 16]
        mem[SP:SP + len(job)] = bytes(job)
        mem[SP_HEIGHT] = height
        want = bytearray(mem)
        h = reference(want, reu, directory, frame, x, y, target, height)
        cpu = ReuCpu(mem, reu)
        cpu.pc = entry
        cpu.push((SENTINEL - 1) >> 8)
        cpu.push((SENTINEL - 1) & 255)
        for _ in range(20_000_000):
            cpu.step()
            if cpu.pc == SENTINEL:
                break
        else:
            raise SystemExit(f'case {case}: no return')
        cycles += cpu.cycles
        dmas += cpu.dmas
        diff = [a for a in list(range(BITMAP, BITMAP + 8000)) + list(range(BANDS, BANDS + 2 * BAND_BYTES)) if mem[a] != want[a]]
        if diff:
            a = diff[0]
            raise SystemExit(f'case {case} frame {frame} x {x} y {y} target ${target:04x}: {len(diff)} bytes differ, '
                             f'first ${a:04x} got {mem[a]:02x} want {want[a]:02x}')
        if mem[SP_H] | mem[SP_H + 1] << 8 != h:
            raise SystemExit(f'case {case} frame {frame}: SP_H {mem[SP_H] | mem[SP_H + 1] << 8}, the frame is {h} lines')
    print(f'draw_sprite_asm matches the reference on {cases} random sprites '
          f'({cycles // cases} cycles, {dmas / cases:.1f} DMAs per sprite on average)')


if __name__ == '__main__':
    main()
