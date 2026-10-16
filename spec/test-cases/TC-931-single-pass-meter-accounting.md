---
id: TC-931
title: "Single-pass meter accounting and bounded integer-arithmetic proof"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-359
    type: verifies
---
# TC-931: Single-pass meter accounting and bounded integer-arithmetic proof

## Description

Verify planned FR-359-AC-8 through AC-10. Kernel accounting and its Kani obligation belong to quire-exact. The five unchanged CG scalar proofs are a separate consumer-owned IR-705/IR-17 acceptance receipt, not kernel test cases.

## Test Procedure

1. For AC-8, inspect production `Meter::charge` and its called helpers to establish that each charge validates and updates its own semantic-size requests with one traversal bounded by the request count, with no `LimitKind::ALL` or other independently sized traversal. Exercise zero, one, two, reordered and repeated same-kind requests, then larger request counts to ensure there is no new count limit. With independent expected values, check same-kind per-admission maxima in the test-support log, every `consumed(kind)`, high-water and cumulative counters, exact first-short `ScalarLimits` field-order refusal including reversed attachment order, and refusal atomicity. Snapshot counters, admissions, log and truncation before an ordinary refusal; check a pending later injected occurrence, an injected denial and cancellation order on separate meters.
2. For AC-9, implement a standalone `cfg(kani)` proof harness that constructs bounded symbolic `i64` operands and runs the production integer-arithmetic path's three charges: operands with at most two size requests, arithmetic with one, and result-retain with none. Vary limits so `IntegerBits`, `ValueOccurrences`, `WorkUnits` and `ResultUnits` can each be first short, including simultaneous shortages. Use charge-level controls for repeated same-kind requests. Compare point order, complete `Incomplete` fields, admitted prefix, all counters and per-admission maxima with a separately written model that checks `ScalarLimits` fields in order and coalesces per-admission maxima. Keep the model outside the production call path. Assert termination via Kani's unwinding checks. Run `cargo kani --harness <integer-arithmetic-meter-harness> --unwind 3 --solver cadical` with a 16 GiB memory limit and 600-second timeout, substituting the actual harness name. Do not disable unwinding checks, assume the conclusion, narrow the original symbolic domain, or route production through `cfg(kani)` logic.
3. For AC-9 mutation controls, separately introduce a temporary double-pass mutant and a temporary `LimitKind::ALL` traversal mutant of charge accounting, then require an unwinding assertion failure for each under the identical command and bounds. Separately swap first-short refusal order and require a model-equivalence failure with a concrete counterexample. Restore production code after each control and record the baseline and mutant results independently; a compiler error or resource failure is not a substantive falsification.
4. For AC-10, the consumer owner re-locks CG to the reviewed kernel and executes the unchanged two TC-027 attested-receipt cases, TC-025 mutated-oracle arithmetic, routed healthy and routed violating-bound. Record the kernel identity, exact selected harnesses, original input/domain bounds, oracle, completion and counterexample checks, unwind 3, Cadical, 16 GiB and 600 seconds, individual verdicts and peak memory. Do not copy or edit CG harnesses into this repository. If any proof fails after the Meter change, report the failure and investigate the separate `Result`/`Outcome::from_stop` carrier before adding a further criterion.

## Expected Results

One request-proportional traversal preserves all previously observable accounting and denial behavior without a new count limit. The standalone Kani proof verifies under its specified ceiling, both traversal mutants fail unwinding, and the refusal-order mutant fails equivalence. `MemoryExhausted`, `UnwindBoundExhausted`, timeout, compiler error, missing mutation execution or an altered harness never counts as acceptance. All five unchanged consumer proofs pass and are recorded as an external receipt. An inconclusive result remains unaccepted even where FR-371-AC-6 classifies it as inconclusive for its broader qualification.

## Status

PLANNED / UNRUN. No Trace binding, standalone proof, mutant result or consumer acceptance receipt is claimed.
