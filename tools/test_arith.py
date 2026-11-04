#!/usr/bin/env python3
"""Check the integer libcalls of `target-c64/asm/arith.S` in the 6502 interpreter.

LLVM-MOS lowers u8/u16/i16 divides and remainders and u8/u16 multiplies to libcalls; `target-c64/build.rs`
links them with `-Wl,--wrap=__X`, so every call reaches `__wrap___X` (arith.S) instead of libcrt's larger
routine. Each one is run from the linked PRG against Python: exhaustively for the 8-bit helpers, every u16
divisor with edge dividends plus random and game-shaped pairs, edge and random signed pairs, and edge plus
random multiplies. A routine may write only its declared scratch registers (arith.S's comments; the
stack page for its return addresses) and must leave the stack balanced. libcrt's `__X` must be gone.

    python tools/test_arith.py [dist/talonhd.prg] [dist/talonhd.elf] [--cases N] [--libcrt]

`--libcrt` runs the same cases against an unwrapped build's libcrt routines (any write allowed), to
compare cycle counts. Prints a JSON report.
"""
import argparse
import json
import random
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from native_cpu import Cpu  # noqa: E402
from test_blit import wsl_path  # noqa: E402

RETURN = 0xFFF0
REM = 0xC000  # where __udivmodqi4's remainder pointer points
RC = lambda n: 2 + n  # __rcN's zero-page address (__rc0 = $02)
STACK = (0x100, 0x200)
# The zero page each routine may write (arith.S's "Writes" lines).
SCRATCH = {
    "__udivqi3": (RC(4), RC(6)), "__umodqi3": (RC(4), RC(6)), "__udivmodqi4": (RC(4), RC(6)),
    "__udivhi3": (RC(4), RC(8)), "__umodhi3": (RC(4), RC(8)),
    "__divhi3": (RC(2), RC(10)), "__modhi3": (RC(2), RC(10)),
    "__mulqi3": (RC(2), RC(7)), "__mulhi3": (RC(2), RC(7)),
}


def symbols(elf):
    nm = Path.home() / '.local/llvm-mos/bin/llvm-nm'
    for cmd in ([str(nm), '-S', elf], ['wsl', '-d', 'Ubuntu-24.04', '--', 'bash', '-lc',
                                       f'~/.local/llvm-mos/bin/llvm-nm -S {wsl_path(elf)}']):
        try:
            out = subprocess.run(cmd, capture_output=True, text=True, check=True).stdout
            break
        except (OSError, subprocess.CalledProcessError):
            continue
    else:
        raise SystemExit(f'cannot run llvm-nm on {elf}')
    syms = {}
    for line in out.splitlines():
        parts = line.split()
        if len(parts) == 4:
            syms[parts[3]] = (int(parts[0], 16), int(parts[1], 16))
    return syms


def call(mem, entry, writable, a=0, x=0, rc=()):
    """Run one libcall to its return: A, X and __rc2.. as given; returns the Cpu."""
    cpu = Cpu(mem, writable)
    cpu.a, cpu.x, cpu.y = a, x, 0x5A
    for i, value in enumerate(rc):
        mem[RC(2) + i] = value
    cpu.pc = entry
    cpu.push((RETURN - 1) >> 8)
    cpu.push((RETURN - 1) & 0xFF)
    steps = 0
    while cpu.pc != RETURN:
        cpu.step()
        steps += 1
        if steps > 100000:
            raise SystemExit(f'runaway at ${cpu.pc:04X}')
    if cpu.s != 0xFD:
        raise SystemExit(f'stack unbalanced: S=${cpu.s:02X}')
    return cpu


def s16(v):
    return v - 65536 if v & 0x8000 else v


def tdiv(a, b):
    """C's truncating division and the remainder with the dividend's sign."""
    q = abs(a) // abs(b)
    q = q if (a < 0) == (b < 0) else -q
    return q, a - q * b


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('prg', nargs='?', default='dist/talonhd.prg')
    ap.add_argument('elf', nargs='?', default='dist/talonhd.elf')
    ap.add_argument('--cases', type=int, default=20000, help='random cases per wide routine')
    ap.add_argument('--libcrt', action='store_true', help="test libcrt's unwrapped routines (baseline cycles)")
    args = ap.parse_args()
    syms = symbols(args.elf)
    prg = Path(args.prg).read_bytes()
    mem = bytearray(65536)
    base = prg[0] | prg[1] << 8
    mem[base:base + len(prg) - 2] = prg[2:]
    rng = random.Random(64)
    report = {}

    def entry(name):
        label = name if args.libcrt else '__wrap_' + name
        if label not in syms:
            return None
        if not args.libcrt and name in syms:
            raise SystemExit(f'libcrt {name} is still linked beside {label}')
        report[name] = {'symbol': label, 'address': f'${syms[label][0]:04X}', 'bytes': syms[label][1]}
        return syms[label][0]

    def regions(name):
        if args.libcrt:
            return [(0, 0x10000)]
        return [SCRATCH[name], STACK] + ([(REM, REM + 1)] if name == '__udivmodqi4' else [])

    def check(name, cases, run, want, key):
        total = 0
        for case in cases:
            got, cycles = run(*case)
            if got != want(*case):
                raise SystemExit(f'{name}: {key(*case)}: got {got}, want {want(*case)}')
            total += cycles
        report[name]['cases'] = report[name].get('cases', 0) + len(cases)
        return round(total / max(len(cases), 1))

    # u8 / u8, u8 % u8 and both at once: every pair.
    pairs8 = [(n, d) for n in range(256) for d in range(1, 256)]
    game8 = [(rng.randrange(256), rng.choice((3, 5, 6, 8, 10, 12, 20, 24))) for _ in range(2000)]
    for name, want in (('__udivqi3', lambda n, d: n // d), ('__umodqi3', lambda n, d: n % d),
                       ('__udivmodqi4', lambda n, d: (n // d, n % d))):
        at = entry(name)
        if at is None:
            continue
        w = regions(name)
        if name == '__udivmodqi4':
            def run(n, d, at=at, w=w):
                mem[REM] = 0xA5
                c = call(mem, at, w, n, d, (REM & 255, REM >> 8))
                return (c.a, mem[REM]), c.cycles
        else:
            def run(n, d, at=at, w=w):
                c = call(mem, at, w, n, d)
                return c.a, c.cycles
        key = lambda n, d: f'{n} op {d}'
        report[name]['mean_cycles'] = check(name, pairs8, run, want, key)
        report[name]['game_cycles'] = check(name, game8, run, want, key)

    # u8 * u8 (low 8 bits): every pair.
    at = entry('__mulqi3')
    if at is not None:
        w = regions('__mulqi3')
        run = lambda a, b: (lambda c: (c.a, c.cycles))(call(mem, at, w, a, b))
        report['__mulqi3']['mean_cycles'] = check('__mulqi3', [(a, b) for a in range(256) for b in range(256)],
                                                  run, lambda a, b: a * b & 255, lambda a, b: f'{a} * {b}')

    # u16 / u16 and u16 % u16: every divisor with edge dividends, then random and game-shaped pairs.
    edge16 = lambda d: {0, 1, d - 1, d, d + 1 & 0xFFFF, min(2 * d - 1, 65535), (d << 8) - 1 & 0xFFFF, 65535}
    every = [(n, d) for d in range(1, 65536) for n in (edge16(d) if d < 1024 or d % 61 == 0 else
                                                        {d - 1, rng.randrange(65536)})]
    rand16 = [(rng.randrange(65536), rng.randrange(1, 65536) >> rng.randrange(16) or 1) for _ in range(args.cases)]
    # The game's: constant byte divisors (127, 10, 9, 5, 3) and the panorama period (5200).
    game16 = [(rng.randrange(65536) >> rng.randrange(8), rng.choice((3, 5, 9, 10, 127, 5200))) for _ in range(4000)]
    for name, want in (('__udivhi3', lambda n, d: n // d), ('__umodhi3', lambda n, d: n % d)):
        at = entry(name)
        if at is None:
            continue
        w = regions(name)
        run = lambda n, d, at=at, w=w: (lambda c: (c.a | c.x << 8, c.cycles))(
            call(mem, at, w, n & 255, n >> 8, (d & 255, d >> 8)))
        key = lambda n, d: f'{n} op {d}'
        check(name, every, run, want, key)
        report[name]['mean_cycles'] = check(name, rand16, run, want, key)
        report[name]['game_cycles'] = check(name, game16, run, want, key)

    # i16 / i16 and i16 % i16 (truncating; never by 0, never MIN / -1: Rust checks both first).
    edges = (0, 1, 2, 3, 5, 127, 128, 255, 256, 5200, 0x7FFF, -1, -2, -3, -5, -127, -128, -255, -256,
             -5200, -0x7FFF, -0x8000)
    signed = [(a, b) for a in edges for b in edges if b and not (a == -0x8000 and b == -1)]
    signed += [(a, b) for a, b in ((rng.randrange(-32768, 32768), rng.randrange(-32768, 32768) >> rng.randrange(15))
                                   for _ in range(args.cases)) if b and not (a == -0x8000 and b == -1)]
    game_s = [(rng.randrange(-16384, 16384) >> rng.randrange(8), rng.choice((127, 5, 5200, 3))) for _ in range(4000)]
    for name, pick in (('__divhi3', 0), ('__modhi3', 1)):
        at = entry(name)
        if at is None:
            continue
        w = regions(name)
        run = lambda a, b, at=at, w=w: (lambda c: (s16(c.a | c.x << 8), c.cycles))(
            call(mem, at, w, a & 255, a >> 8 & 255, (b & 255, b >> 8 & 255)))
        want = lambda a, b, pick=pick: tdiv(a, b)[pick]
        key = lambda a, b: f'{a} op {b}'
        report[name]['mean_cycles'] = check(name, signed, run, want, key)
        report[name]['game_cycles'] = check(name, game_s, run, want, key)

    # u16 * u16 (low 16 bits, the same signed or unsigned).
    at = entry('__mulhi3')
    if at is not None:
        w = regions('__mulhi3')
        run = lambda a, b: (lambda c: (c.a | c.x << 8, c.cycles))(call(mem, at, w, a & 255, a >> 8, (b & 255, b >> 8)))
        want = lambda a, b: a * b & 0xFFFF
        key = lambda a, b: f'{a} * {b}'
        edge = (0, 1, 2, 3, 40, 127, 128, 150, 255, 256, 257, 0x7FFF, 0x8000, 0xFFFF, 0xFF00, 0x00FF, 0x1234)
        cases = [(a, b) for a in edge for b in edge] + [(a, b) for a in range(0, 65536, 257) for b in (1, 40, 65535)]
        check('__mulhi3', cases, run, want, key)
        report['__mulhi3']['mean_cycles'] = check('__mulhi3', [(rng.randrange(65536), rng.randrange(65536))
                                                               for _ in range(args.cases)], run, want, key)
        report['__mulhi3']['game_cycles'] = check('__mulhi3', [(rng.randrange(512), rng.randrange(512)) if rng.randrange(2)
                                                               else (rng.randrange(-200, 200) & 0xFFFF, rng.choice((150, 3, 5)))
                                                               for _ in range(4000)], run, want, key)

    if not report:
        raise SystemExit('no arithmetic helpers found')
    print(json.dumps({'elf': str(args.elf), 'libcrt': args.libcrt, 'helpers': report}, indent=2))


if __name__ == '__main__':
    main()
