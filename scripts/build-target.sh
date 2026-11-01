#!/usr/bin/env bash
# Build the C64 program (WSL/Linux with rust-mos + llvm-mos SDK).
# Extra arguments go to cargo, e.g. --features rust-columns,raster-bars
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root"
source scripts/env.sh
native_target="$root/target-c64/target"
(cd target-c64 && CARGO_TARGET_DIR="$native_target" cargo build --release "$@")
mkdir -p dist
cp "$native_target/mos-c64-none/release/talonhd-c64" dist/talonhd.prg
for elf in "$native_target/mos-c64-none/release/deps"/talonhd_c64-*.elf; do
    if [[ "${elf%.elf}" -ef "$native_target/mos-c64-none/release/talonhd-c64" ]]; then
        cp "$elf" dist/talonhd.elf
    fi
done
# Paged phases (all but BOOT live outside the PRG): extract them for bake and stamp the PRG.
python3 tools/phases.py dist/talonhd.elf dist/talonhd.prg dist/phases
"$LLVM_MOS/bin/llvm-size" -A dist/talonhd.elf | grep -v '^\.debug' || true
ls -l dist/talonhd.prg
