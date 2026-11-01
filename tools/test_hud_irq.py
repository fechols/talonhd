#!/usr/bin/env python3
"""Run the linked `hud_irq` (from dist/talonhd.prg) in a 6502 interpreter as a raster IRQ, eight times
(one frame), and check it against talonhd::hud::Table semantics: every row's Y in all 8 sprites, its
pointers in both buffers' pointer slots, the row counter and the next raster line, registers preserved,
and nothing else written.

    python tools/test_hud_irq.py [dist/talonhd.prg] [dist/talonhd.elf]
"""
import random
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from native_cpu import Cpu  # noqa: E402
from test_blit import symbol  # noqa: E402

HUD, ROWS = 0xCB00, 8
LINES, YS, PTRS = HUD + 1, HUD + 9, HUD + 17
SENTINEL = 0xFFF0
ALLOWED = {0xD019, 0xD012, HUD} | {0xD001 + 2 * i for i in range(8)} | \
    {0x4FF8 + i for i in range(8)} | {0x8FF8 + i for i in range(8)}


def main():
    prg = Path(sys.argv[1] if len(sys.argv) > 1 else 'dist/talonhd.prg').read_bytes()
    elf = sys.argv[2] if len(sys.argv) > 2 else 'dist/talonhd.elf'
    entry = symbol(elf, 'hud_irq')
    base = prg[0] | prg[1] << 8
    rng = random.Random(7)
    for trial in range(20):
        mem = bytearray(65536)
        mem[base:base + len(prg) - 2] = prg[2:]
        lines = [rng.randrange(256) for _ in range(ROWS)]
        ys = [rng.randrange(256) for _ in range(ROWS)]
        ptrs = [[rng.randrange(48) for _ in range(8)] for _ in range(ROWS)]
        mem[LINES:LINES + ROWS] = bytes(lines)
        mem[YS:YS + ROWS] = bytes(ys)
        for r in range(ROWS):
            mem[PTRS + 8 * r:PTRS + 8 * r + 8] = bytes(ptrs[r])
        start = rng.randrange(ROWS)
        mem[HUD] = start
        cpu = Cpu(mem, [(0x100, 0x200), (0xD000, 0xD040), (HUD, HUD + 1), (0x4FF8, 0x5000), (0x8FF8, 0x9000)])
        for k in range(ROWS):
            row = (start + k) % ROWS
            cpu.a, cpu.x, cpu.y = 0x11, 0x22, 0x33
            cpu.writes.clear()
            # Enter as the hardware does: push PC and P, jump to the vector.
            cpu.push(SENTINEL >> 8)
            cpu.push(SENTINEL & 255)
            cpu.push(cpu.p)
            cpu.pc = entry
            for _ in range(10_000):
                cpu.step()
                if cpu.pc == SENTINEL:
                    break
            else:
                raise SystemExit(f'trial {trial} row {row}: no RTI')
            nxt = (row + 1) % ROWS
            errors = []
            if (cpu.a, cpu.x, cpu.y) != (0x11, 0x22, 0x33):
                errors.append('registers not restored')
            if any(mem[0xD001 + 2 * i] != ys[row] for i in range(8)):
                errors.append('sprite Y')
            for i in range(8):
                if mem[0x4FF8 + i] != ptrs[row][i] | 0x40 or mem[0x8FF8 + i] != ptrs[row][i]:
                    errors.append(f'pointer {i}')
            if mem[HUD] != nxt or mem[0xD012] != lines[nxt] or mem[0xD019] != 1:
                errors.append('row counter / next line / ack')
            stray = sorted(set(cpu.writes) - ALLOWED - set(range(0x100, 0x200)))
            if stray:
                errors.append('stray writes ' + ', '.join(f'${a:04x}' for a in stray))
            if errors:
                raise SystemExit(f'trial {trial} row {row}: ' + '; '.join(errors))
    print('hud_irq matches the table contract over 20 random frames (160 IRQs)')


if __name__ == '__main__':
    main()
