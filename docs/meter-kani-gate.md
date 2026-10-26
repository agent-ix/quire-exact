# Meter proof gate

DO NOT RUN: exhausts fixed 16 GiB ceiling, returns Inconclusive MemoryExhausted, proves nothing, blocked work for a week; Peter ordered it never run until all feature work is done.

The FR-359-AC-9 proof remains unqualified. Its source is retained in
`src/accounting/kani_proofs.rs`, but `src/accounting.rs` excludes that module
even during Kani builds. Direct harness selection therefore cannot run it.

`make kani-meter-gate`, `scripts/meter_kani_scope.sh`, and
`scripts/meter_kani_gate.py` print the warning, exit with failure, and skip
proof setup. The ignored native test fails with the warning when selected with
`--ignored`. None of these skip results qualify FR-359-AC-9 or satisfy its healthy proof
and control requirements.
