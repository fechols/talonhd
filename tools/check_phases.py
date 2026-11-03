#!/usr/bin/env python3
"""Audit REU code paging in the linked C64 program (src/paging.rs rules):

- resident code (every section that is not a `.phase.*`) JSRs/JMPs into the window $9000-$9FFF only at a
  declared phase entry point (`ENTRIES`: called right after `paging::page`);
- a phase's code only jumps into its own bytes or into resident code (never another phase, which would
  share the same addresses), and never through an indirect JMP;
- the bootstrap call to `talonhd_page` is the only way in (not checked: entry is by Rust call after page).

    python tools/check_phases.py [dist/talonhd.elf]
"""
import re
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from test_blit import wsl_path  # noqa: E402

WINDOW = (0x9000, 0xA000)
# Phase entry points (mangled-name fragments), each called only after paging its phase in.
ENTRIES = ('7Midboss4step', '6Ground4step', '5audio7service', '5audio5prime', '5audio4tune3arm',
           '5audio4tune7started', '5audio4tune9calibrate', '5audio4tune4load', '4game4Game5reset',
           '14install_phases', '4boss4Boss4step', '6replay9autopilot', '3ace3see', '3ace5motor', '3ace5think', '5clear5Clear4step', '5clear5Clear7sprites', '5clear5Clear7restart', '6stage25waves', '9Campanile4step', '9Campanile7sprites', '9Astrolabe4step', '9Astrolabe7sprites', '4gnd24step', '4air24step', '4game4Game8reset_at', '6render8hud_init', '6render10read_image',
           '6render12verify_image', '6render16invalidate_image', 'bench_all')


def objdump(elf, *args):
    tool = '~/.local/llvm-mos/bin/llvm-objdump'
    cmd = ['wsl', '-d', 'Ubuntu-24.04', '--', 'bash', '-lc', f'{tool} {" ".join(args)} {wsl_path(elf)}']
    return subprocess.run(cmd, capture_output=True, text=True, check=True).stdout


def main():
    elf = sys.argv[1] if len(sys.argv) > 1 else 'dist/talonhd.elf'
    sections = {}
    for line in objdump(elf, '-h').splitlines():
        p = line.split()
        if len(p) >= 4 and p[0].isdigit():
            sections[p[1]] = (int(p[3], 16), int(p[2], 16))
    phases = {n: s for n, s in sections.items() if n.startswith('.phase.')}
    code = [n for n in sections if n.startswith('.text') or n in ('.hitext', '.lowtext', '.uptext', '.iotext')]
    entries = set()
    # Every phase's own code ranges (its symbols): all phases share the window's addresses, so a jump to a
    # window address is only safe if that address is code of the same phase.
    owned = {n: [] for n in phases}
    for line in objdump(elf, '-t').splitlines():
        p = line.split()
        if len(p) >= 2 and any(e in p[-1] for e in ENTRIES):
            entries.add(int(p[0], 16))
        if len(p) >= 5 and p[-3] in owned:
            addr, size = int(p[0], 16), int(p[-2], 16)
            owned[p[-3]].append((addr, max(size, 1)))
    errors = []
    call = re.compile(r'^\s*([0-9a-f]+):.*\t(jsr|jmp)\t(\(?)\$([0-9a-f]+)')
    for name in code + list(phases):
        start, size = sections[name]
        for line in objdump(elf, '-d', f'--section={name}').splitlines():
            m = call.match(line)
            if not m:
                continue
            at, op, indirect, target = int(m.group(1), 16), m.group(2), m.group(3), int(m.group(4), 16)
            in_window = WINDOW[0] <= target < WINDOW[1]
            if name in phases:
                if indirect:
                    errors.append(f'{name} ${at:04x}: indirect {op} (${target:04x})')
                elif in_window and not start <= target < start + size:
                    errors.append(f'{name} ${at:04x}: {op} ${target:04x} outside the phase')
                elif in_window:
                    # A call or jump must land on a function of this phase, or stay inside its own function
                    # (an address inside another of this phase's functions is where another phase's
                    # function may sit: phases share the window).
                    starts = {a for a, n in owned[name]}
                    home = [(a, n) for a, n in owned[name] if a <= at < a + n]
                    ok = target in starts or any(a <= target < a + n for a, n in home)
                    if not ok:
                        errors.append(f'{name} ${at:04x}: {op} ${target:04x} is not a function of this phase')
            elif in_window and (indirect or target not in entries):
                errors.append(f'{name} ${at:04x}: resident {op} into the window ${target:04x} (not a phase entry)')
    for e in errors[:40]:
        print(e)
    if errors:
        raise SystemExit(f'{len(errors)} paging violation(s)')
    print(f'paging ok: {len(code)} resident code sections, {len(phases)} phases ({", ".join(phases)})')


if __name__ == '__main__':
    main()
