---
id: FR-359
title: "Cumulative meter counters refuse at the u64 boundary"
type: FR
relationships: []
---
# FR-359: Cumulative meter counters refuse at the u64 boundary

## Description

When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an `Incomplete` record for the first unavailable counter without wrapping or changing any consumed counter or admission count.

## Behavior

`work_units` and `result_units` accumulate across admitted charges. An addition that equals its limit is admissible; an addition beyond its limit, including one whose exact amount cannot fit `u64`, is refused. A refused charge preserves the previously admitted state even when its work amount fits but its result amount does not. Semantic-size counters remain high-water values.

For each `Meter::charge` admission, validation and update of semantic-size requests SHALL use one traversal of that charge's own requests, bounded by its request count. It SHALL NOT traverse `LimitKind::ALL` or another collection whose length is independent of that count. The FR-362 integer-arithmetic schedule supplies at most two requests per charge. This proof bound does not add a public request-count limit: FR-358 permits arbitrarily many repeated requests. Field-order refusal, same-kind per-admission maxima, atomicity, injection and cancellation order, `consumed()` for every kind, and the FR-358 log SHALL remain unchanged.

The kernel SHALL own a standalone Kani obligation for the three integer-arithmetic charges over symbolic bounded `i64` operands.

The obligation SHALL exercise production `Meter::charge` with unwinding assertions at `--unwind 3` and compare field-order refusal, coalescing, admitted prefix and counters to an independent reference model.

Test-only `cfg(kani)` harness code may construct inputs and the model.

Such harness code SHALL NOT change any production path. The lane command is `cargo kani --harness <integer-arithmetic-meter-harness> --unwind 3 --solver cadical`, with a 16 GiB memory ceiling and 600-second timeout; the implementation lane replaces the placeholder with the actual harness name.

A double-pass or `LimitKind::ALL`-traversal mutant SHALL fail an unwinding assertion.

A refusal-order mutant SHALL fail model equivalence. `MemoryExhausted`, `UnwindBoundExhausted`, timeout, compiler error or missing assertion is not accepted evidence.

The unchanged consumer-owned CG scalar proofs are the external IR-705/IR-17 acceptance receipt after re-locking to the reviewed kernel: TC-027 attested receipt twice, TC-025 mutated-oracle arithmetic, routed healthy, and routed violating-bound. Each SHALL pass with its original domain, oracle, completion and counterexample checks at unwind 3, Cadical, 16 GiB and 600 seconds. This is not a quire-exact test. The integer-arithmetic `Result` to `Outcome::from_stop` carrier is a separate open risk outside this Meter-only slice. If the five unchanged proofs still fail after the Meter change, investigate that carrier and add a separate criterion before claiming acceptance. FR-371-AC-6 calls `MemoryExhausted` inconclusive for its broader qualification; an inconclusive run does not satisfy this obligation or receipt.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-359-AC-1 | With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX - 1` followed by one work unit reaches `u64::MAX` exactly. The next one-unit charge returns `Incomplete` at its named point with `limit_kind = WorkUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; all consumed counters and admission count stay at their pre-refusal values. | Test |
| FR-359-AC-2 | With `work_units = result_units = u64::MAX`, admit one charge requesting `u64::MAX` result units. Its work consumption is 1. A second charge requesting one result unit returns `Incomplete` at its named point with `limit_kind = ResultUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; work remains 1 and admission count remains 1. | Test |
| FR-359-AC-3 | With both cumulative limits at `u64::MAX`, a work charge whose exact amount is `u64::MAX + 1` returns `Incomplete` with `limit_kind = WorkUnits` and `next_charge = u64::MAX + 1`; no counter or admission count changes. | Test |
| FR-359-AC-4 | With both cumulative limits at `u64::MAX`, a charge with one work unit and `u64::MAX + 1` result units returns `Incomplete` with `limit_kind = ResultUnits` and `next_charge = u64::MAX + 1`; no counter or admission count changes. | Test |
| FR-359-AC-5 | First admit an `integer_bits` high-water size of 8 and one result unit under limits of at least 16 bits, `u64::MAX` work units and one result unit. Then attempt a charge carrying `integer_bits = 16` and one result unit. The result refusal leaves `integer_bits = 8`, work units, result units and admission count at their pre-refusal values; under `test-support`, the admitted-charge log is unchanged too. | Test |
| FR-359-AC-6 | With no injection installed, independently make each of the ten counters the first unavailable counter of a charge, including several simultaneously short counters and semantic sizes attached in either order. The reported `limit_kind` follows `ScalarLimits` field order, with the exact limit, previously consumed amount, requested amount and point. A refusal after an earlier size check succeeds changes no counter, admission count or test-support log. A separate meter with a pending later named denial proves that this ordinary refusal does not advance its occurrence. | Test |
| FR-359-AC-7 | On fresh, charged and stopped meters, `Meter::consumed(kind)` answers without panic for every member of `LimitKind::ALL`: untouched counters are zero, semantic sizes are admitted high-water maxima and work/results are admitted cumulative totals. | Test |
| FR-359-AC-8 | Inspection and tests show `Meter::charge` validates and updates one charge's semantic-size requests in a single traversal bounded by that charge's request count, without traversing `LimitKind::ALL` or an independently sized collection. For FR-362 integer arithmetic the bound is two requests per charge. Reordered and repeated requests preserve the per-admission maximum in the FR-358 log; simultaneous shortages refuse in `ScalarLimits` field order; ordinary refusal preserves counters, high-water, admissions, log and a pending injected occurrence. Cancellation and injection precedence, cumulative totals and `consumed()` for every kind remain unchanged. No request-count denial is added. | Test |
| FR-359-AC-9 | A standalone quire-exact Kani harness over symbolic bounded `i64` operands executes the real three-charge FR-362 schedule and bounded charge-level repeated-kind controls, proves accounting terminates under `--unwind 3`, and agrees with an independent model of field-order refusal, coalescing, admitted prefix and counters. At Cadical, 16 GiB and 600 seconds, a double-pass or `LimitKind::ALL` mutant fails unwinding and a refusal-order mutant fails equivalence. Test-only `cfg(kani)` harness code changes no production path; `MemoryExhausted`, `UnwindBoundExhausted` or another inconclusive outcome fails acceptance. | Test |
| FR-359-AC-10 | After the consumer re-locks to the reviewed kernel, the five unchanged CG scalar proofs—TC-027 attested receipt twice, TC-025 mutated-oracle arithmetic, routed healthy and routed violating-bound—pass with their original input bounds, oracle and resource ceilings. Record each result as an external IR-705/IR-17 receipt; no quire-exact test or altered consumer harness substitutes for it. | Test |

## Status

AC-1 through AC-5 retain earlier Trace bindings. PR #9 adds executable Trace bindings for AC-6 and AC-7. Run `quire matrix` for the current criterion-to-test mapping.

AC-8 through AC-10 are PLANNED / UNRUN. No implementation, Kani result, mutation control or consumer receipt is claimed by this proposal.

## Dependencies

- [TC-907](../test-cases/TC-907-cumulative-meter-boundary.md) is the test-case home for these criteria. This requirement concerns the existing `Meter::charge` and `Charge` accounting contract in `src/accounting.rs`.
- [TC-931](../test-cases/TC-931-single-pass-meter-accounting.md) owns the planned single-pass, standalone proof and external-receipt procedures. FR-362 defines the three-charge schedule; FR-358 defines the diagnostic log; FR-368 defines the limit vocabulary.
