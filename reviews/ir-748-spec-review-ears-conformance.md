---
id: SR-6416
title: "EARS review of FR-375"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-exact@702a13f5089b6ec0971e2ca5f8ac138e15887495; spec/functional/FR-375-composable-equality-effect-seam.md, spec/test-cases/TC-923-composable-equality-effect-seam.md, spec/assurance/AD-002-composable-equality-proof-seam.md"
review_set: subset
---

## Summary

The engine reports 55/55 repository documents grammar-clean with no EARS warnings. The new FR’s requirement statements use named subjects and singular SHALL obligations. No EARS defect was found.

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

## Verdict

PASS: no findings for this analysis.
