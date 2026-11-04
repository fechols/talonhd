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
ALLOWED = {0x0001, 0xD019, 0xD012, HUD, 0xCADA, 0xCADB, 0xCA98, 0xCA99} | {0xD001 + 2 * i for i in range(8)} | \
    {0x4FF8 + i for i in range(8)} | {0x8FF8 + i for i in range(8)}


def stub_clock(mem, elf):
    """The IRQ clock (`irq_clock_in/out`) lives in regions boot copies into place (.ptext, .uptext), not
    in the raw PRG: return at once (its writes are the status block's IRQ_US and IRQ_T0 only)."""
    for name in ('irq_clock_in', 'irq_clock_out'):
        mem[symbol(elf, name)] = 0x60


def main():
    prg = Path(sys.argv[1] if len(sys.argv) > 1 else 'dist/talonhd.prg').read_bytes()
    elf = sys.argv[2] if len(sys.argv) > 2 else 'dist/talonhd.elf'
    entry = symbol(elf, 'hud_irq')
    base = prg[0] | prg[1] << 8
    rng = random.Random(7)
    for trial in range(20):
        mem = bytearray(65536)
        mem[base:base + len(prg) - 2] = prg[2:]
        stub_clock(mem, elf)
        lines = [rng.randrange(256) for _ in range(ROWS)]
        ys = [rng.randrange(256) for _ in range(ROWS)]
        ptrs = [[rng.randrange(48) for _ in range(8)] for _ in range(ROWS)]
        mem[LINES:LINES + ROWS] = bytes(lines)
        mem[YS:YS + ROWS] = bytes(ys)
        for r in range(ROWS):
            mem[PTRS + 8 * r:PTRS + 8 * r + 8] = bytes(ptrs[r])
        start = rng.randrange(ROWS)
        mem[HUD] = start
        mem[1] = 0x34  # interrupted inside the game step, I/O banked out
        cpu = Cpu(mem, [(0x0001, 0x0002), (0x100, 0x200), (0xD000, 0xD040), (HUD, HUD + 1), (0x4FF8, 0x5000), (0x8FF8, 0x9000)])
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
            if mem[1] != 0x34:
                errors.append('$01 not restored')
            if 0x0001 not in cpu.writes:
                errors.append('I/O not banked in')
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
    painter_chain(prg, elf, base)


PB = 0xCAE0  # layout::PAINTER: P_ON +0, P_GATE +1, P_NEXT +2


def painter_chain(prg, elf, base):
    """With the border painter on (P_ON), one frame of IRQs: row 0 (@20) arms the gate stage (@27), which
    closes the DMA gate and arms block T (@30); block T (border_t, stubbed here) then arms row 1; rows 1..7 as
    before; after row 7 the gate stage (@243) closes the gate and arms block B (@246); block B (border_b,
    stubbed) arms row 0."""
    entry = symbol(elf, 'hud_irq')
    stubs = {symbol(elf, 'border_t'): 'T', symbol(elf, 'border_b'): 'B'}
    rng = random.Random(11)
    for trial in range(10):
        mem = bytearray(65536)
        mem[base:base + len(prg) - 2] = prg[2:]
        stub_clock(mem, elf)
        lines = [20] + [rng.randrange(60, 230) for _ in range(ROWS - 1)]
        mem[LINES:LINES + ROWS] = bytes(lines)
        mem[HUD] = 0
        mem[PB:PB + 3] = bytes([1, 0, 0])
        cpu = Cpu(mem, [(0x0001, 0x0002), (0x100, 0x200), (0xD000, 0xD040), (HUD, HUD + 1), (PB, PB + 32),
                        (0x4FF8, 0x5000), (0x8FF8, 0x9000)])
        want = [(None, 27, None), (None, 30, 0x80), ('T', lines[1], None)]
        want += [(None, lines[r + 1], None) for r in range(1, ROWS - 1)]
        want += [(None, 243, None), (None, 246, 0x80), ('B', lines[0], None)]
        for k, (block, line, gate) in enumerate(want):
            called = []
            cpu.push(SENTINEL >> 8)
            cpu.push(SENTINEL & 255)
            cpu.push(cpu.p)
            cpu.pc = entry
            for _ in range(10_000):
                if cpu.pc in stubs:
                    called.append(stubs[cpu.pc])
                    lo, hi = cpu.pop(), cpu.pop()  # the block returns at once
                    cpu.pc = ((hi << 8 | lo) + 1) & 0xFFFF
                cpu.step()
                if cpu.pc == SENTINEL:
                    break
            else:
                raise SystemExit(f'painter trial {trial} IRQ {k}: no RTI')
            got_block = called[0] if called else None
            if got_block != block or len(called) > 1:
                raise SystemExit(f'painter trial {trial} IRQ {k}: blocks {called}, want {block}')
            if mem[0xD012] != line:
                raise SystemExit(f'painter trial {trial} IRQ {k}: next line {mem[0xD012]}, want {line}')
            if gate is not None and mem[PB + 1] != gate:
                raise SystemExit(f'painter trial {trial} IRQ {k}: gate {mem[PB + 1]:02x}, want {gate:02x}')
        if mem[HUD] != 0 or mem[PB + 2] != 0:
            raise SystemExit(f'painter trial {trial}: the frame ends on row {mem[HUD]}, stage {mem[PB + 2]}')
    print('hud_irq chains the border painter over 10 frames (row 0, gate @27, T @30, rows 1..7, gate @243, B @246)')


if __name__ == '__main__':
    main()
