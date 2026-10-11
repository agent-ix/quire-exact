---
id: SR-6408
title: "Code review of IR-748 spec-only changes"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@702a13f5089b6ec0971e2ca5f8ac138e15887495; spec/functional/FR-375-composable-equality-effect-seam.md, spec/test-cases/TC-923-composable-equality-effect-seam.md, spec/assurance/AD-002-composable-equality-proof-seam.md"
review_set: subset
---

## Summary

The diff adds three specification artifacts and no source or test code. I checked the added seam against the current equality kernel and consumer references for duplication, unsupported implementation claims, and scope fidelity; no code-review defect was found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder). | - |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-375 | examined | When a checked equality caller evaluates two admitted operands, the `quire-exact` kernel SHALL expose the exact equality result and the count of `equality.pair` charges admitted by that same evaluation, without a second caller-side equality implementation or a caller-authored charge oracle. |
| FR-375 | examined | The seam is owned by `quire-exact::equality`, where `plan_pairs` forms both the relation and the count, and `planned_equality` admits the charge schedule. The API SHALL be a result-bearing extension of `planned_equality` or an equivalent kernel API. `CheckedEquality::evaluate` SHALL use that real kernel path and expose its result through its existing `Outcome<bool>` behavior. |
| FR-375 | examined | The successful evaluation record SHALL carry the kernel's Boolean relation and exact planned `equality.pair` event count from the same formed plan. The meter SHALL remain authoritative for effects: only pair charges whose `Meter::charge` call succeeds are admitted. The seam SHALL NOT claim all planned pairs were admitted when an earlier charge refuses. Existing callers of `planned_equality` SHALL retain its current result and refusal semantics. |
| FR-375 | examined | For the bounded IR-241 qualification input, operands are the two separately constructed depth-four `List` values in `qsl-eval/tests/it/equality_matrix.rs::e21_operands`: each has `occ() == 16` and the pair plan has 17 events. The input is fixed and finite; the seam does not promise a bound for arbitrary recursive or caller-supplied values. Admission of the input and its composite declaration identity precedes evaluation. |
| FR-375 | examined | The kernel evaluation preserves this ordered effect schedule: `equality.plan-form`, `equality.plan`, one `equality.pair` for each planned occurrence-path pair, then `equality.result-retain`. The plan-form charge uses the maximum operand occurrence count as `ValueOccurrences` and the sum of operand occurrence counts as `WorkUnits`. The plan charge sets the `ValueOccurrences` high-water size to the pair count, requires `pair_count + 2` remaining `WorkUnits` and one remaining `ResultUnit`, then consumes one plan work unit. Each pair charge consumes one work unit; result retention consumes one wor |
| FR-375 | examined | The kernel `EqualityPlan` and `planned_equality` remain the source of count and relation semantics. `plan_pairs` and `PlannedPairs` stay private; QSL does not copy the occurrence-pair walker, construct a parallel charge schema, or substitute a plan-only assertion for `CheckedEquality::evaluate`. |
| FR-375-AC-1 | examined | \| FR-375-AC-1 \| On the bounded depth-four `List` input with equal operands, the actual `CheckedEquality::evaluate` path completes `true`; the result-bearing kernel seam reports 17 planned pair events; the meter admits exactly 17 `EqualityPair` charges, in the schedule after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`. \| Test \| |
| FR-375-AC-2 | examined | \| FR-375-AC-2 \| On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit its occurrence-pair traversal or charge schedule. \| Test \| |
| FR-375-AC-3 | examined | \| FR-375-AC-3 \| If a test meter denies the 17th `EqualityPair`, evaluation returns the existing typed `Incomplete` outcome at `EqualityPair`; only the preceding 16 pair charges are admitted, and `EqualityResultRetain` is not admitted. A denial of `EqualityResultRetain` occurs only after all 17 pair charges have been admitted. \| Test \| |
| FR-375-AC-4 | examined | \| FR-375-AC-4 \| With equal-shaped operands whose composite declaration identities differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted. \| Test \| |
| FR-375-AC-5 | examined | \| FR-375-AC-5 \| With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only 18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned. \| Test \| |
| FR-375-AC-6 | examined | \| FR-375-AC-6 \| The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion. \| Test \| |
| FR-375-AC-7 | examined | \| FR-375-AC-7 \| No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool control demonstrates both a passing and a deliberately failing dependency-contract case through the exact QSL-to-quire-exact dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real callee remains in the outer CBMC graph. \| Inspection \| |
| AD-002 | examined | The `quire-exact` owner SHALL expose a production API that returns the exact equality relation and the pair-event count from the same formed `PlannedPairs`, while the actual `Meter::charge` operations remain responsible for admitting effects. The API SHALL preserve the current `planned_equality` behavior for existing callers; the exact signature is implementation-owned. The QSL `CheckedEquality::evaluate` path SHALL invoke this API and preserve its existing `Outcome<bool>` result and operator handling. |
| TC-923-step-1 | examined | 1. Construct two equal operands with the same checked composite identity. Form `CheckedEquality` through the normal checker, call `plan_equality` for the expected pair-event count, then evaluate through `CheckedEquality::evaluate` with the E21 meter. |
| TC-923-step-2 | examined | 2. Construct a same-shape operand pair with the first head unequal and the same checked composite identity. Repeat the normal checked evaluation with a fresh E21 meter. |
| TC-923-step-3 | examined | 3. On a fresh meter with `work_units = 50` and all other limits sufficient, evaluate the equal pair. Also exercise the existing test-support denial at the 17th `EqualityPair` and denial at `EqualityResultRetain` to locate both refusal boundaries. |
| TC-923-step-4 | examined | 4. Construct the same composite shape under two distinct declaration identities and attempt checked construction. Confirm whether the checked evaluator is reached and inspect the meter before and after. |
| TC-923-step-5 | examined | 5. For each successful evaluation, compare the result-bearing seam's pair count, `plan_equality` count and the exact `EqualityPair` entries in the actual checked evaluation's admitted-charge log. Inspect the complete ordered log and consumed counters. |
| TC-923-step-6 | examined | 6. If a proposed Kani `proof_for_contract` or `stub_verified` decomposition is used, run a focused control through the same compiled QSL-to-`quire-exact` dependency edge: one case whose dependency contract holds and one deliberately falsified dependency contract. Capture that each case is checked by the installed tool. Do not treat a same-crate or toy control as satisfying this step. |
| TC-923-expected-1 | examined | 1. Construct two equal operands with the same checked composite identity. Form `CheckedEquality` through the normal checker, call `plan_equality` for the expected pair-event count, then evaluate through `CheckedEquality::evaluate` with the E21 meter. |
| TC-923-expected-2 | examined | 2. Construct a same-shape operand pair with the first head unequal and the same checked composite identity. Repeat the normal checked evaluation with a fresh E21 meter. |
| TC-923-expected-3 | examined | 3. On a fresh meter with `work_units = 50` and all other limits sufficient, evaluate the equal pair. Also exercise the existing test-support denial at the 17th `EqualityPair` and denial at `EqualityResultRetain` to locate both refusal boundaries. |
| TC-923-expected-4 | examined | 4. Construct the same composite shape under two distinct declaration identities and attempt checked construction. Confirm whether the checked evaluator is reached and inspect the meter before and after. |
| TC-923-expected-5 | examined | 5. For each successful evaluation, compare the result-bearing seam's pair count, `plan_equality` count and the exact `EqualityPair` entries in the actual checked evaluation's admitted-charge log. Inspect the complete ordered log and consumed counters. |
| TC-923-expected-6 | examined | 6. If a proposed Kani `proof_for_contract` or `stub_verified` decomposition is used, run a focused control through the same compiled QSL-to-`quire-exact` dependency edge: one case whose dependency contract holds and one deliberately falsified dependency contract. Capture that each case is checked by the installed tool. Do not treat a same-crate or toy control as satisfying this step. |

## Verdict

PASS: no findings for this analysis.
