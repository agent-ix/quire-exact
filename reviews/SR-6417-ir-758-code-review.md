---
id: SR-6417
title: code-review of IR-758 result-bearing equality seam
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact@1647a4a7a893034831a7f0cb173a916a3d0c0ada; src/equality.rs, src/lib.rs; Ticket IR-758
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: references
  - target: ix://agent-ix/quire-exact/TC-923
    type: references
---

## Summary

Ticket: IR-758. Independent review of frozen PR #34 at `1647a4a7a893034831a7f0cb173a916a3d0c0ada`, against base `78b357f2aba09179ee70964154086abbb8bc4b8b`. The production change keeps `plan_pairs` private, constructs the result only after plan, all pair, and result-retain charges succeed, and maps each existing `Outcome` variant through the compatibility API. I found one test-to-criterion mismatch: the tests tagged for FR-375-AC-1/2 use a flat raw kernel composite and call `planned_equality_with_plan` directly, while those criteria specify QSL's actual checked path and fixed depth-four `List` operands.

## Verdict

CONDITIONAL. No production-code or Rust-idiom defect found. The current tagged test does not establish the end-to-end criterion it claims to bind; IR-241 must establish that path in its own qualification. No source edit or merge was made.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Tagged test uses a flat raw kernel value and bypasses the specified checked caller path | src/equality.rs:303-365 |
| FND-002 | medium | Tagged test uses a flat raw kernel value and bypasses the specified checked caller path | src/equality.rs:303-365 |

## Coverage

Reviewed the complete PR diff (only `src/equality.rs` and `src/lib.rs`), repo conventions in `AGENTS.md` and `CLAUDE.md`, and the merged FR-375/TC-923/AD-002 contract. No applicable AssuranceProfile was found. Rust review ran as the code-review language lane. No cargo or Kani gate was run by this reviewer; `git diff --check 78b357f2aba09179ee70964154086abbb8bc4b8b...1647a4a7a893034831a7f0cb173a916a3d0c0ada` passed. Gap analysis is recorded separately. Python and React lanes skipped (no applicable files); spec-review sub-analyses skipped (the PR diff changes no spec or plan documents). Model backend id was not exposed by this session; marker records `unavailable`.

Full reviewed scope and verified bindings:

```yaml
clean: false
scope:
  - {id: FR-375-AC-1, path: spec/functional/FR-375-composable-equality-effect-seam.md, role: examined, excerpt: "On the bounded depth-four `List` input with equal operands, the actual `CheckedEquality::evaluate` path completes `true`; the result-bearing kernel seam reports 17 planned pair events; the meter admits exactly 17 `EqualityPair` charges, in the schedule after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`."}
  - {id: FR-375-AC-2, path: spec/functional/FR-375-composable-equality-effect-seam.md, role: examined, excerpt: "On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit its occurrence-pair traversal or charge schedule."}
  - {id: FR-375-AC-3, path: spec/functional/FR-375-composable-equality-effect-seam.md, role: examined, excerpt: "If a test meter denies the 17th `EqualityPair`, evaluation returns the existing typed `Incomplete` outcome at `EqualityPair`; only the preceding 16 pair charges are admitted, and `EqualityResultRetain` is not admitted. A denial of `EqualityResultRetain` occurs only after all 17 pair charges have been admitted."}
  - {id: FR-375-AC-4, path: spec/functional/FR-375-composable-equality-effect-seam.md, role: context_only, excerpt: "With equal-shaped operands whose composite declaration identities differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted."}
  - {id: FR-375-AC-5, path: spec/functional/FR-375-composable-equality-effect-seam.md, role: examined, excerpt: "With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only 18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned."}
  - {id: FR-375-AC-6, path: spec/functional/FR-375-composable-equality-effect-seam.md, role: context_only, excerpt: "The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. A plan-only equality or duplicated QSL walker does not satisfy this criterion."}
  - {id: FR-375-AC-7, path: spec/functional/FR-375-composable-equality-effect-seam.md, role: context_only, excerpt: "No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool control demonstrates both a passing and a deliberately failing dependency-contract case through the exact QSL-to-quire-exact dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real callee remains in the outer CBMC graph."}
  - {id: FR-375-AC-8, path: spec/functional/FR-375-composable-equality-effect-seam.md, role: examined, excerpt: "With `result_units = 0` and sufficient work and occurrence limits, the actual checked evaluation admits `equality.plan-form`, then refuses `equality.plan` atomically with typed `Incomplete { limit_kind: ResultUnits, limit: 0, consumed: 0, next_charge: 1 }`. No plan, pair, or result-retain charge is admitted and no equality result is returned. A version that checks result capacity only after pair charges fails this control."}
  - {id: src/equality.rs, path: src/equality.rs, role: examined, excerpt: "The new public `planned_equality_with_plan` returns a private-field `EqualityEvaluation` only after `planned_equality_stop` successfully forms the plan and admits all pair and result-retain charges. `planned_equality` maps the completed record back to the prior bool outcome."}
  - {id: src/lib.rs, path: src/lib.rs, role: examined, excerpt: "The new result-bearing API and record are exported from the crate root."}
findings:
  - fnd: FND-001
    severity: medium
    confidence: high
    method: code-review
    check: trace
    artifact_id: FR-375-AC-1
    related_ids: [FR-375-AC-2]
    path: src/equality.rs
    lines: "303-365"
    excerpt: "On the bounded depth-four `List` input with equal operands, the actual `CheckedEquality::evaluate` path completes `true`; the result-bearing kernel seam reports 17 planned pair events; the meter admits exactly 17 `EqualityPair` charges, in the schedule after `EqualityPlanForm` and `EqualityPlan` and before `EqualityResultRetain`."
    finding: "The test tagged for AC-1/2 constructs one flat composite with 15 Boolean slots and calls the raw kernel API directly. It does not use the specified depth-four `List` values from QSL's `e21_operands`, call `CheckedEquality::evaluate`, or assert the exact successful charge-point sequence. A change in the checked caller path or nested E21 traversal can therefore leave this test green while AC-1/2 are broken; keep the end-to-end claim for IR-241's QSL qualification and avoid treating this binding as that evidence."
    test_id: result_bearing_evaluation_returns_relation_and_same_plan_count
    claimed_ac: FR-375-AC-1
    correct_ac: none
  - fnd: FND-002
    severity: medium
    confidence: high
    method: code-review
    check: trace
    artifact_id: FR-375-AC-2
    related_ids: [FR-375-AC-1]
    path: src/equality.rs
    lines: "303-365"
    excerpt: "On the same bounded shape with the first head unequal, the actual `CheckedEquality::evaluate` path completes `false` and still admits the complete 17-pair schedule and result-retain charge; equality does not short-circuit its occurrence-pair traversal or charge schedule."
    finding: "The test tagged for AC-1/2 constructs one flat composite with 15 Boolean slots and calls the raw kernel API directly. It does not use the specified depth-four `List` values from QSL's `e21_operands`, call `CheckedEquality::evaluate`, or assert the exact successful charge-point sequence. A change in the checked caller path or nested E21 traversal can therefore leave this test green while AC-1/2 are broken; keep the end-to-end claim for IR-241's QSL qualification and avoid treating this binding as that evidence."
    test_id: result_bearing_evaluation_returns_relation_and_same_plan_count
    claimed_ac: FR-375-AC-2
    correct_ac: none
bindings:
  - {test_id: result_bearing_evaluation_returns_relation_and_same_plan_count, ac_id: FR-375-AC-1, trace: wrong}
  - {test_id: result_bearing_evaluation_returns_relation_and_same_plan_count, ac_id: FR-375-AC-2, trace: wrong}
```

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed 7509d09a48598affb7ce48928df99520e892c191 | Replaced checked-caller AC claim with an untagged kernel-scoped nested schedule test; exact charge order is asserted. IR-241 retains the real CheckedEquality integration obligation. |
| FND-002 | fixed 7509d09a48598affb7ce48928df99520e892c191 | Replaced checked-caller AC claim with an untagged kernel-scoped first-head-unequal nested schedule test; full pair/result-retain schedule is asserted. IR-241 retains the real CheckedEquality integration obligation. |

After excerpt (src/equality.rs, lines 361-392):

```rust
#[test]
fn kernel_result_bearing_evaluation_returns_relation_plan_and_charge_order() {
    for (first_head_unequal, expected) in [(false, true), (true, false)] {
        let (left, right) = e21_list_shape_operands(first_head_unequal);
        assert_eq!(left.occ(), Integer::from(16_u64));
        assert_eq!(right.occ(), Integer::from(16_u64));
        let plan = plan_equality(&left, &right).unwrap();
        assert_eq!(plan.pair_events(), &Integer::from(17_u64));
```
