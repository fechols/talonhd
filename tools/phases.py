#!/usr/bin/env python3
"""Extract the paged code phases (src/paging.rs) from the linked C64 program and stamp the PRG.

Every phase but BOOT is linked with a load address outside the PRG (target-c64/link.ld `phimg`), so the
PRG stays small; `talonhd bake` puts dist/phases/<name>.bin into the level REU images (the common ones and
each level's own) and the C64 pages them from there. The stamp (paging::stamp over every phase, same
algorithm here) ties the two: it is patched into the PRG's PHASE_STAMP and written by bake into every
image header; boot refuses a mismatch.

    python3 tools/phases.py dist/talonhd.elf dist/talonhd.prg dist/phases   (scripts/build-target.sh)
"""
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
LLVM = Path(os.environ.get('LLVM_MOS', Path.home() / '.local/llvm-mos')) / 'bin'
PRG_BASE = 0x0801


def names():
    src = (ROOT / 'src/paging.rs').read_text(encoding='utf-8')
    m = re.search(r'pub const NAMES: \[&str; PHASES\] = \[([^\]]*)\]', src)
    found = re.findall(r'"(\w+)"', m.group(1))
    return found, found.index('boot')


def slots():
    """{(owner, slot): name} from src/paging.rs: owner 0 for every level's phases, level + 1 for a
    level's own (`paging::of_level`), as the id's high and low nibbles."""
    src = (ROOT / 'src/paging.rs').read_text(encoding='utf-8')
    ids = {}
    for name, n, level, slot in re.findall(r'pub const (\w+): u8 = (?:(\d+)|of_level\((\d+), (\d+)\));', src):
        ids[name] = int(n) if n else (int(level) + 1) << 4 | int(slot)
    order = re.findall(r'\w+', re.search(r'pub const IDS: \[u8; PHASES\] = \[([^\]]*)\]', src).group(1))
    found, _ = names()
    assert len(order) == len(found), 'paging::IDS and NAMES differ in length'
    return {(ids[i] >> 4, ids[i] & 15): n for i, n in zip(order, found)}


def stamp(images, boot):
    s = 0x5A5A
    for i, img in enumerate(images):
        if i == boot:
            continue
        for b in img:
            s = (((s << 1) | (s >> 15)) & 0xFFFF)
            s = (s + b) & 0xFFFF
        s = (s + len(img)) & 0xFFFF
    return s


def main():
    elf, prg, out = sys.argv[1], sys.argv[2], Path(sys.argv[3])
    out.mkdir(parents=True, exist_ok=True)
    phase_names, boot = names()
    images = []
    for n in phase_names:
        path = out / f'{n}.bin'
        subprocess.run([str(LLVM / 'llvm-objcopy'), '-O', 'binary', f'--only-section=.phase.{n}', elf, str(path)],
                       check=True)
        images.append(path.read_bytes())
        if len(images[-1]) > 0x1000:
            raise SystemExit(f'phase {n}: {len(images[-1])} bytes > 4096')
    s = stamp(images, boot)
    nm = subprocess.run([str(LLVM / 'llvm-nm'), elf], capture_output=True, text=True, check=True).stdout
    addr = next(int(p[0], 16) for p in (l.split() for l in nm.splitlines()) if p and p[-1] == 'PHASE_STAMP')
    data = bytearray(Path(prg).read_bytes())
    off = addr - PRG_BASE + 2
    if data[off:off + 2] != b'\xff\xff':
        raise SystemExit(f'PHASE_STAMP at ${addr:04x}: expected the $FFFF placeholder in the PRG')
    data[off:off + 2] = s.to_bytes(2, 'little')
    Path(prg).write_bytes(data)
    sizes = ', '.join(f'{n} {len(i)}' for n, i in zip(phase_names, images))
    print(f'phases: {sizes}; stamp {s:04x}')


if __name__ == '__main__':
    main()
