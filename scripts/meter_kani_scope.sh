#!/usr/bin/env bash
# Enforce the native wrapper's ceiling before it can start Cargo.
set -euo pipefail
cgroup="/sys/fs/cgroup$(awk -F: '$1 == "0" {print $3}' /proc/self/cgroup)"
[[ "$(cat "$cgroup/memory.max")" == 17179869184 ]]
[[ "$(cat "$cgroup/memory.swap.max")" == 0 ]]
python3 scripts/meter_kani_gate.py --preflight
export CARGO_TARGET_DIR="$PWD/target/meter-kani-native"
export CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 RAYON_NUM_THREADS=1
exec "${1:-cargo}" test --locked --test meter_kani_gate -- --ignored --exact meter_proof_and_production_all_loop_control
