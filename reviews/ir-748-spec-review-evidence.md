---
id: SR-6413
title: "Evidence review of the equality seam"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-exact@702a13f5089b6ec0971e2ca5f8ac138e15887495; spec/functional/FR-375-composable-equality-effect-seam.md, spec/test-cases/TC-923-composable-equality-effect-seam.md, spec/assurance/AD-002-composable-equality-proof-seam.md"
review_set: subset
---

## Summary

Catalog advice matches the authored Test methods for the behavioral cases. Inspection is appropriate for AC-7 because the selected design explicitly declines contract composition absent a cross-crate control; that control is conditional on proposing the contract. However, the written ResultUnits reservation lacks a test case that can detect its removal or reordering.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | TC-923 omits a zero-ResultUnits control proving EqualityPlan refuses before any EqualityPair charge. | FR-375-AC-5; TC-923 | correct-requirement-no-evidence |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-375-AC-1 | examined | \| FR-375-AC-1 \| On the bounded depth-four `List` input with equal operands, the actual `CheckedEquality::evaluate` path completes `true`; the result-bearing kernel seam reports 17 planned pair events; the meter admits exactly 17 `EqualityPair` charges, in the schedule after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`. \| Test \| |
| FR-375-AC-2 | examined | \| FR-375-AC-2 \| On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit its occurrence-pair traversal or charge schedule. \| Test \| |
| FR-375-AC-3 | examined | \| FR-375-AC-3 \| If a test meter denies the 17th `EqualityPair`, evaluation returns the existing typed `Incomplete` outcome at `EqualityPair`; only the preceding 16 pair charges are admitted, and `EqualityResultRetain` is not admitted. A denial of `EqualityResultRetain` occurs only after all 17 pair charges have been admitted. \| Test \| |
| FR-375-AC-4 | examined | \| FR-375-AC-4 \| With equal-shaped operands whose composite declaration identities differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted. \| Test \| |
| FR-375-AC-5 | examined | \| FR-375-AC-5 \| With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only 18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned. \| Test \| |
| FR-375-AC-6 | examined | \| FR-375-AC-6 \| The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion. \| Test \| |
| FR-375-AC-7 | examined | \| FR-375-AC-7 \| No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool control demonstrates both a passing and a deliberately failing dependency-contract case through the exact QSL-to-quire-exact dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real callee remains in the outer CBMC graph. \| Inspection \| |
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

CONDITIONAL: one medium coverage finding requires a discriminating ResultUnits reservation case at the EqualityPlan boundary.


## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed a188e75c2d54d2eae4a9dc925dc9b43ac6ff13d2 | TC-923 step 4 and expected result 4 add zero-ResultUnits at EqualityPlan, including atomic refusal before pair charges. |
