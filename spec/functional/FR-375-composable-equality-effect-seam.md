---
id: FR-375
title: "Expose the exact equality relation and admitted pair effects from one kernel evaluation"
type: FR
relationships:
  - target: "ix://agent-ix/quire-exact/AD-002"
    type: "references"
---
# FR-375: Expose the exact equality relation and admitted pair effects from one kernel evaluation

## Description

When a checked equality caller evaluates two admitted operands, the `quire-exact` kernel SHALL expose the exact equality result and the count of `equality.pair` charges admitted by that same evaluation, without a second caller-side equality implementation or a caller-authored charge oracle.

## Behavior

The seam is owned by `quire-exact::equality`, where `plan_pairs` forms both the relation and the count, and `planned_equality` admits the charge schedule. The API SHALL be a result-bearing extension of `planned_equality` or an equivalent kernel API. `CheckedEquality::evaluate` SHALL use that real kernel path and expose its result through its existing `Outcome<bool>` behavior.

The successful evaluation record SHALL carry the kernel's Boolean relation and exact planned `equality.pair` event count from the same formed plan. The meter SHALL remain authoritative for effects: only pair charges whose `Meter::charge` call succeeds are admitted. The seam SHALL NOT claim all planned pairs were admitted when an earlier charge refuses. Existing callers of `planned_equality` SHALL retain its current result and refusal semantics.

For the bounded IR-241 qualification input, operands are the two separately constructed depth-four `List` values in `qsl-eval/tests/it/equality_matrix.rs::e21_operands`: each has `occ() == 16` and the pair plan has 17 events. The input is fixed and finite; the seam does not promise a bound for arbitrary recursive or caller-supplied values. Admission of the input and its composite declaration identity precedes evaluation.

The kernel evaluation preserves this ordered effect schedule: `equality.plan-form`, `equality.plan`, one `equality.pair` for each planned occurrence-path pair, then `equality.result-retain`. The plan-form charge uses the maximum operand occurrence count as `ValueOccurrences` and the sum of operand occurrence counts as `WorkUnits`. The plan charge sets the `ValueOccurrences` high-water size to the pair count, requires `pair_count + 2` remaining `WorkUnits` and one remaining `ResultUnit`, then consumes one plan work unit. Each pair charge consumes one work unit; result retention consumes one work unit and one result unit. For ordinary charges, `Meter::charge` checks semantic-size limits in `LimitKind` field order before cumulative `WorkUnits` and then `ResultUnits`; it admits no part of a refused charge. `charge_plan` checks a named denial first, then pair-count `ValueOccurrences`, remaining work reservation, and remaining result capacity before mutation. A `ForeignReference` or checked-invariant refusal during plan formation occurs after plan-form admission and before plan and pair charges. A meter refusal stops at its first unavailable charge and does not admit later charges.

The kernel `EqualityPlan` and `planned_equality` remain the source of count and relation semantics. `plan_pairs` and `PlannedPairs` stay private; QSL does not copy the occurrence-pair walker, construct a parallel charge schema, or substitute a plan-only assertion for `CheckedEquality::evaluate`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-375-AC-1 | On the bounded depth-four `List` input with equal operands, the actual `CheckedEquality::evaluate` path completes `true`; the result-bearing kernel seam reports 17 planned pair events; the meter admits exactly 17 `EqualityPair` charges, in the schedule after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`. | Test |
| FR-375-AC-2 | On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit its occurrence-pair traversal or charge schedule. | Test |
| FR-375-AC-3 | If a test meter denies the 17th `EqualityPair`, evaluation returns the existing typed `Incomplete` outcome at `EqualityPair`; only the preceding 16 pair charges are admitted, and `EqualityResultRetain` is not admitted. A denial of `EqualityResultRetain` occurs only after all 17 pair charges have been admitted. | Test |
| FR-375-AC-4 | With equal-shaped operands whose declared (static) composite types differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted. | Test |
| FR-375-AC-5 | With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only 18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned. | Test |
| FR-375-AC-6 | The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion. | Test |
| FR-375-AC-7 | No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool control demonstrates both a passing and a deliberately failing dependency-contract case through the exact QSL-to-quire-exact dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real callee remains in the outer CBMC graph. | Inspection |
| FR-375-AC-8 | With `result_units = 0` and sufficient work and occurrence limits, the actual checked evaluation admits `equality.plan-form`, then refuses `equality.plan` atomically with typed `Incomplete { limit_kind: ResultUnits, limit: 0, consumed: 0, next_charge: 1 }`. No plan, pair, or result-retain charge is admitted and no equality result is returned. A version that checks result capacity only after pair charges fails this control. | Test |

## Dependencies

- The architecture decision is [AD-002](../assurance/AD-002-composable-equality-proof-seam.md).
- The falsifying controls are [TC-923](../test-cases/TC-923-composable-equality-effect-seam.md).
- Source grounding: `src/equality.rs` (`EqualityPlan`, `plan_equality`, `planned_equality`, `planned_equality_stop`, private `plan_pairs`); `src/accounting.rs` (`Meter`, `ChargePoint`, `LimitKind`); QSL's `qsl-eval/tests/it/equality_matrix.rs` (`e21_operands`, `checked_equality_records_the_full_plan_for_equal_and_unequal_lists`, `list_equality_refuses_distinct_declaration_identities`).
- The cross-crate tool boundary remains open until an exact dependency-contract control proves it; a same-crate proof or toy contract is insufficient.
