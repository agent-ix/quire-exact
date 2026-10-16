---
id: FR-359
title: "Cumulative meter boundaries and request-proportional charging"
type: FR
relationships: []
---
# FR-359: Cumulative meter boundaries and request-proportional charging

## Description

When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an `Incomplete` record for the first unavailable counter without wrapping or changing any consumed counter or admission count. The meter SHALL keep semantic-size charge traversal proportional to that charge's request count while preserving refusal order and admitted accounting. A standalone meter proof and an external consumer receipt qualify these claims as specified below.

## Behavior

`work_units` and `result_units` accumulate across admitted charges. An addition that equals its limit is admissible; an addition beyond its limit, including one whose exact amount cannot fit `u64`, is refused. A refused charge preserves the previously admitted state even when its work amount fits but its result amount does not. Semantic-size counters remain high-water values.

For each `Meter::charge`, validation and update of semantic-size requests SHALL use only traversals whose trip counts do not exceed that charge's request count. The charge path SHALL NOT traverse `LimitKind::ALL` or another collection whose length is independent of the charge's request count. A single request traversal is the intended design, but the bounded proof does not establish the number of separate request traversals. The FR-362 integer-arithmetic schedule supplies at most two requests per charge. This proof bound does not add a public request-count limit: FR-358 permits arbitrarily many repeated requests. The meter SHALL preserve field-order refusal, the first-inserted failing amount within a kind, same-kind per-admission maxima, atomicity, injection and cancellation order, `consumed()` for every kind, and the FR-358 log.

The standalone Kani obligation uses production `Meter::charge` for the exact three-charge FR-362 schedule and an independent reference model. Its symbolic operands range over all `i64` values; the charge limits, including `ResultUnits`, vary to exercise admitted and refused prefixes. Its test-only `cfg(kani)` model must not change a production path. The public `evaluate_integer_arithmetic` / `Outcome::from_stop` carrier is outside this charge-level obligation and is checked by the external receipt in AC-10. The implementation lane must first report whether Kani can compile this `#![no_std]` crate; compatibility is unverified and blocks AC-9 until demonstrated.

The implementation lane must add `[lints.rust] unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)'] }` or equivalent check-cfg configuration so `make lint` with `-D warnings` passes, while retaining the production feature set and the no_std build. The explicit non-CI lane command is `cargo kani --features test-support --harness "$HARNESS" --unwind 3 --solver cadical`, where `HARNESS` is the symbol of the `cfg(kani)` harness tagged `Trace: FR-359-AC-9`, with a 16 GiB memory ceiling and 600-second timeout. Neither `make ci` nor current CI runs Kani; a follow-up must wire the harness into a persistent gate.

After the consumer re-locks to the reviewed kernel, the unchanged CG scalar tests in AC-10 provide the external IR-705/IR-17 receipt. A test passes when its own assertions hold: healthy proofs must be Kani `Verified`, while mutation controls must be Kani `Falsified` with their specified counterexamples. FR-371-AC-6 and TC-919 step 6 require the shared routed healthy, routed violating-bound, and TC-025 arithmetic-mutant controls; AC-10 additionally requires both attested receipt cases. A `MemoryExhausted`, timeout or compiler error is inconclusive for each test and does not satisfy either acceptance criterion. If the receipt fails after the Meter change, investigate the separate `Result`/`Outcome::from_stop` carrier before claiming acceptance.

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
| FR-359-AC-8 | Reordered and repeated requests preserve the per-admission maximum in the FR-358 log. With `IntegerBits` limit 5 and requests 7 then 9 for that kind in one charge, refusal reports `next_charge = 7`, the first-inserted failing amount; the existing 9 then 7 case alone cannot distinguish maximum coalescing before refusal. Simultaneous shortages refuse in `ScalarLimits` field order; ordinary refusal preserves counters, high-water, admissions, log and a pending injected occurrence. Cancellation and injection precedence, cumulative totals and `consumed()` for every kind remain unchanged. Zero, one, two and larger request counts gain no request-count denial. | Test |
| FR-359-AC-9 | The `cfg(kani)` harness tagged `Trace: FR-359-AC-9` uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand charge with at most two semantic-size requests, arithmetic charge with one, result-retain charge with none) over the full `i64` operand range, plus separate repeated-kind charge controls with at most two requests each. Symbolic limits including `ResultUnits` cover first-short and simultaneous shortages and absence of the third admission. Under `--features test-support --unwind 3 --solver cadical`, 16 GiB and 600 seconds, the baseline verifies unwinding and agrees with an independent model of complete refusal fields, field order, first-inserted failing amount, per-admission maxima, admitted prefix and counters; harness and model loops also stay within the bound, with ten-field comparisons unrolled. A `LimitKind::ALL`-traversal mutant reports an unwinding-assertion failure at its mutated production loop; a refusal-order mutant yields a model-equivalence counterexample. For the baseline, any `UnwindBoundExhausted`, `MemoryExhausted`, timeout, compile error or missing assertion fails acceptance. For either mutation control, an unwinding assertion is expected only for the ALL-traversal mutant; resource exhaustion, timeout or compile error is inconclusive and does not satisfy the control. Test-only code changes no production path. | Test |
| FR-359-AC-10 | After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their own assertions under their original domains, oracle, completion and counterexample checks, unwind 3, Cadical, 16 GiB and 600 seconds. Record their individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test or altered consumer harness substitutes for it. | Test |
| FR-359-AC-11 | Inspection of semantic-size processing in production `Meter::charge` and called helpers finds no traversal with a trip count greater than that charge's semantic-size request count, including no traversal of `LimitKind::ALL` or another independently sized collection. The FR-362 proof schedule has at most two requests per charge, while the public `Charge` API retains no request-count ceiling. | Inspection |

AC-10 expected CG verdicts (a passing CG test means its own assertions hold):

| Consumer test in `tests/it/kani_obligations.rs` | Expected Kani verdict |
| --- | --- |
| `tc_027_attested_scalar_receipt_binds_real_proof_and_native_result` | `Verified` for both ordinary and zero-result-unit constrained receipts; native outcomes are completed 5 and `Incomplete` at result-retain respectively. |
| `tc_027_attested_scalar_subtraction_receipts_bind_both_operand_orders` | `Verified` for both operand orders; native outcomes are completed 3 and -3. |
| `tc_025_scalar_harness_falsifies_a_mutated_oracle_arithmetic` | Healthy sub-run `Verified`; mutated Add-to-Subtract oracle sub-run `Falsified` at its sound assertion with a counterexample. |
| `tc_027_a_routed_scalar_harness_verifies` | `Verified`, including its completion cover. |
| `tc_027_a_routed_scalar_harness_violating_its_bound_is_falsified` | `Falsified` with a counterexample for the narrowed asserted result bound. |

## Status

AC-1 through AC-5 retain earlier Trace bindings. PR #9 adds executable Trace bindings for AC-6 and AC-7. Run `quire matrix` for the current criterion-to-test mapping.

AC-8, AC-9 and AC-11 are PLANNED / UNRUN. No implementation, Kani result or mutation control is claimed by this proposal. AC-10 is permanently external consumer evidence and deliberately remains untagged in quire-exact: the CG tests belong to CG and must not be represented by an unrelated local Trace tag. Its receipt is UNRUN; even after the consumer evaluates it, Quire's local strict reports will continue to show AC-10 unbacked unless the receipt moves to the consumer's spec.

## Dependencies

- [TC-907](../test-cases/TC-907-cumulative-meter-boundary.md) is the test-case home for AC-1 through AC-7. This requirement concerns the existing `Meter::charge` and `Charge` accounting contract in `src/accounting.rs`.
- [TC-931](../test-cases/TC-931-single-pass-meter-accounting.md) owns AC-8 through AC-11 procedures and the external receipt record. FR-362 defines the three-charge schedule; FR-358 defines the diagnostic log; FR-368 defines the limit vocabulary.
