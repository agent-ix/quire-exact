#!/usr/bin/env bash
# DO NOT RUN: exhausts fixed 16 GiB ceiling, returns Inconclusive MemoryExhausted, proves nothing, blocked work for a week; Peter ordered it never run until all feature work is done.
set -euo pipefail
echo "SKIPPED: DO NOT RUN: exhausts fixed 16 GiB ceiling, returns Inconclusive MemoryExhausted, proves nothing, blocked work for a week; Peter ordered it never run until all feature work is done" >&2
exit 2
# Archived gate implementation below is unreachable while the proof is disabled.
cgroup="/sys/fs/cgroup$(awk -F: '$1 == "0" {print $3}' /proc/self/cgroup)"
[[ "$(cat "$cgroup/memory.max")" == 17179869184 ]]
[[ "$(cat "$cgroup/memory.swap.max")" == 0 ]]
python3 scripts/meter_kani_gate.py --preflight
export CARGO_TARGET_DIR="$PWD/target/meter-kani-native"
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1
exec "${1:-cargo}" test --locked --test meter_kani_gate -- --ignored --exact meter_proof_and_production_all_loop_control
