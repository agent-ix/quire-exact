---
id: TC-923
title: "Check the checked equality result and its admitted pair effects"
type: TC
relationships:
  - target: "ix://agent-ix/quire-exact/FR-375"
    type: "verifies"
---
# TC-923: Check the checked equality result and its admitted pair effects

## Description

Verify FR-375 through QSL's actual `CheckedEquality::evaluate` path and the `quire-exact` meter effects. Use the fixed E21 depth-four `List` operands: 16 occurrences per operand and 17 occurrence-path pairs.

## Test Procedure

1. Construct two equal operands with the same checked composite identity. Form `CheckedEquality` through the normal checker, call `plan_equality` for the expected pair-event count, then evaluate through `CheckedEquality::evaluate` with the E21 meter.
2. Construct a same-shape operand pair with the first head unequal and the same checked composite identity. Repeat the normal checked evaluation with a fresh E21 meter.
3. On a fresh meter with `work_units = 50` and all other limits sufficient, evaluate the equal pair. Also exercise the existing test-support denial at the 17th `EqualityPair` and denial at `EqualityResultRetain` to locate both refusal boundaries.
4. Construct the same composite shape under two distinct declaration identities and attempt checked construction. Confirm whether the checked evaluator is reached and inspect the meter before and after.
5. For each successful evaluation, compare the result-bearing seam's pair count, `plan_equality` count and the exact `EqualityPair` entries in the actual checked evaluation's admitted-charge log. Inspect the complete ordered log and consumed counters.
6. If a proposed Kani `proof_for_contract` or `stub_verified` decomposition is used, run a focused control through the same compiled QSL-to-`quire-exact` dependency edge: one case whose dependency contract holds and one deliberately falsified dependency contract. Capture that each case is checked by the installed tool. Do not treat a same-crate or toy control as satisfying this step.

## Expected Results

1. For equal operands, `CheckedEquality::evaluate` returns `Outcome::Completed(true)`. The plan and same-evaluation result report 17 pair events; the real meter log is `EqualityPlanForm`, `EqualityPlan`, 17 `EqualityPair` entries, `EqualityResultRetain`. Consumed `WorkUnits` is 51, `ResultUnits` is 1 and `ValueOccurrences` is 17.
2. For the first-head-unequal operands, the outcome is `Outcome::Completed(false)` and the charge order, pair count, and all three consumed totals are unchanged from the equal case.
3. With 50 work units, `equality.plan-form` admits 32 work units, then `equality.plan` refuses atomically because its `pair_count + 2` reservation is 19 while only 18 remain. The typed `Incomplete` identifies `WorkUnits`, limit 50, consumed 32, next charge 19; no plan, pair, or result-retain charge is admitted. Injected denial at pair 17 occurs after 16 pair charges and reports `EqualityPair`; denial at result retention occurs after all 17 pair charges and reports `EqualityResultRetain`. No completed Boolean is returned in these refused cases.
4. Checked construction returns `IllTypedCause::TypeMismatch` before calling `CheckedEquality::evaluate`; the meter remains unchanged and no pair effect occurs.
5. A successful result is tied to one formed kernel plan and its real admitted meter effects; no copied walker, plan-only proof, or caller-authored charge count can satisfy the assertions.
6. Contract-based composition remains unsupported unless the exact cross-crate control demonstrates that the deliberate dependency-contract violation is detected. Without that evidence, the real callee remains in the outer CBMC graph and this case gives no proof credit.
