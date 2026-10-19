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

When a checked equality caller evaluates two admitted operands, the `quire-exact` kernel SHALL expose the exact equality relation and planned `equality.pair` count from one plan, while its `Meter` records only admitted charges. The runtime's actual checked evaluation and its end-to-end controls are allocated to [RT FR-277](ix://agent-ix/quire-contract-runtime/FR-277).

## Behavior

The kernel seam is owned by `quire-exact::equality`, where `plan_pairs` forms both the relation and the count, and `planned_equality` admits the charge schedule. The API SHALL be a result-bearing extension of `planned_equality` or an equivalent kernel API. `CheckedEquality::evaluate` is implemented in `quire-contract-runtime/src/exact/equality.rs`; RT FR-277 requires that consumer to use the real kernel path and expose its result through its existing `Outcome<bool>` behavior.

The successful evaluation record SHALL carry the kernel's Boolean relation and exact planned `equality.pair` event count from the same formed plan. The meter SHALL remain authoritative for effects: only pair charges whose `Meter::charge` call succeeds are admitted. The seam SHALL NOT claim all planned pairs were admitted when an earlier charge refuses. Existing callers of `planned_equality` SHALL retain its current result and refusal semantics.

For the bounded kernel qualification input, two separately constructed depth-four `CollectionKind::Sequence` values each have 16 visited occurrences and one plan has 17 pair events. This fixed input provides a finite test control without asserting a bound for arbitrary recursive values.

The kernel evaluation preserves this ordered effect schedule: `equality.plan-form`, `equality.plan`, one `equality.pair` for each planned occurrence-path pair, then `equality.result-retain`. The plan-form charge uses the maximum operand occurrence count as `ValueOccurrences` and the sum of operand occurrence counts as `WorkUnits`. The plan charge sets the `ValueOccurrences` high-water size to the pair count, requires `pair_count + 2` remaining `WorkUnits` and one remaining `ResultUnit`, then consumes one plan work unit. Each pair charge consumes one work unit; result retention consumes one work unit and one result unit. For ordinary charges, `Meter::charge` checks semantic-size limits in `LimitKind` field order before cumulative `WorkUnits` and then `ResultUnits`; it admits no part of a refused charge. `charge_plan` checks a named denial first, then pair-count `ValueOccurrences`, remaining work reservation, and remaining result capacity before mutation. A `ForeignReference` or checked-invariant refusal during plan formation occurs after plan-form admission and before plan and pair charges. A meter refusal stops at its first unavailable charge and does not admit later charges.

The kernel `EqualityPlan` and `planned_equality` remain the source of count and relation semantics. `plan_pairs` and `PlannedPairs` stay private; QSL does not copy the occurrence-pair walker, construct a parallel charge schema, or substitute a plan-only assertion for `CheckedEquality::evaluate`.

## Acceptance Criteria

| ID | Criteria | Verification |
|----|----------|--------------|
| FR-375-AC-9 | On equal and first-head-unequal bounded kernel operands, `planned_equality_with_plan` returns the Boolean relation and 17 planned pair events from the same formed plan; `plan_equality` agrees on that count, and the admitted meter log contains 17 pair charges on success. | Test |
| FR-375-AC-10 | When `Meter::charge` refuses the 17th pair, the result-bearing kernel operation returns the typed stop with exactly the prior 16 pair charges admitted and no result-retain charge; it does not report all 17 planned pairs as admitted. A result-retain refusal after all 17 pair charges retains that admitted prefix. | Test |
| FR-375-AC-7 | No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool control demonstrates both a passing and a deliberately failing dependency-contract case through the actual RT-to-quire-exact dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real callee remains in the outer CBMC graph. | Inspection |

## Dependencies

- The architecture decision is [AD-002](../assurance/AD-002-composable-equality-proof-seam.md).
- [RT FR-277](ix://agent-ix/quire-contract-runtime/FR-277) owns the real checked-evaluator controls formerly FR-375-AC-1 through AC-5 and AC-8. FR-277-AC-7 carries FR-375-AC-6 verbatim in the RT Kani lane (owner titan:kani:plan, PLANNED). This kernel leaf claims no proof.
- [TC-923](../test-cases/TC-923-composable-equality-effect-seam.md) points to RT TC-203 for end-to-end controls; kernel controls for AC-9/10 are PLANNED under TC-932.
- Source grounding: `src/equality.rs` (`EqualityPlan`, `plan_equality`, `planned_equality`, `planned_equality_stop`, private `plan_pairs`); `src/accounting.rs` (`Meter`, `ChargePoint`, `LimitKind`).
- The RT-to-kernel tool boundary remains open until a focused dependency-contract control proves it; a same-crate proof or toy contract is insufficient.

## Status

PLANNED: kernel AC-9 and AC-10 need direct tagged tests. FR-277-AC-7 is PLANNED at RT owner titan:kani:plan.
