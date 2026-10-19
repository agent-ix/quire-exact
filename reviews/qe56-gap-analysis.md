---
id: SR-13014
title: "quire-exact PR 56 IR-878 re-homing gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; PR #56 head 159f170d8ab7e339fe636934e26f602667fc0153 vs origin/main b83cab9; spec/functional/FR-369, FR-372, FR-375, FR-376, FR-380; spec/test-cases/TC-923; ticket IR-878"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: reviews
---

## Summary

Planless, diff-scoped audit of the re-homing against the computed strict coverage. Strict coverage was run on origin/main b83cab9 and on PR head 159f170. Plan completion: not assessed.

Ticket: IR-878

## Reviewed units

- FR-375 (examined): When a checked equality caller evaluates two admitted operands, the `quire-exact` kernel SHALL expose the exact equality relation and planned `equality.pair` count from one plan, while its `Meter` records only admitted charges. The runtime's actual checked evaluation and its end-to-end controls are allocated to [RT FR-277](ix://agent-ix/quire-contract-runtime/FR-277).
- FR-375-AC-6 (examined): The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. The real evaluator is owned by quire-contract-runtime; a plan-only equality or duplicated QSL walker does not satisfy this criterion.
- FR-375-AC-7 (examined): No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool control demonstrates both a passing and a deliberately failing dependency-contract case through the exact QSL-to-quire-exact dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real callee remains in the outer CBMC graph.
- FR-375-AC-1..5,8 (examined): Removed from quire-exact; text re-homed verbatim (AC-8 renumbered) as RT FR-277-AC-1..6 in quire-contract-runtime#132.
- FR-369-AC-3,4 (examined): Removed; re-homed verbatim as RT FR-278-AC-1, AC-2 (quire-contract-runtime#132).
- FR-369-AC-7 (examined): Removed; re-homed verbatim as CG FR-042-AC-1 (quire-contract-codegen#449).
- FR-369-AC-9 (examined): Removed; re-homed verbatim as QSV FR-122-AC-1 (quire-semantic-value#47).
- FR-372-AC-5 (examined): Removed; split into QSV FR-122-AC-2 (supplied path; drops only 'evaluator meter change') and QSL FR-391-AC-3 (evaluator meter and charge sequence unchanged).
- FR-380-AC-6 (examined): Removed; split into QSV FR-122-AC-3 (supplied caller) and QSL FR-391-AC-2 (omitted-B 4096 default, evaluator charges unchanged).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | After the move, FR-375's kernel SHALLs have no kernel-executable acceptance criterion in quire-exact. Those SHALLs are: a result-bearing extension of `planned_equality` exposing the relation and the planned pair count from one plan; only successful `Meter::charge` pair charges are admitted; and the seam does not claim all pairs were admitted after an earlier refusal. The remaining AC-6 is a Kani claim over the RT evaluator and AC-7 is an inspection. The kernel API exists (`planned_equality_with_plan`, `EqualityEvaluation` in src/equality.rs) but is exercised only incidentally by tests/composite_declarations.rs under FR-376. The brief says quire-exact keeps criteria for what it owns (equality planning, Meter). Here the owned obligation lost every criterion. Kernel-level ACs should be added for the 17-pair relation/count and the denial-at-pair-17 admitted-prefix cases via `planned_equality_with_plan`. These would be new criteria, not a weakening of the moved ones. | spec/functional/FR-375-composable-equality-effect-seam.md:13-27 |

## Verdict

Coverage delta (`quire coverage --strict`): rows 194 -> 182, backed 167 -> 167, unbacked 27 -> 15. Removed unbacked rows are exactly the 12 moved ids: FR-369-AC-3/4/7/9, FR-372-AC-5, FR-375-AC-1..5/8 and FR-380-AC-6. No row was newly unbacked, `status_lies` is 0, and `unmatched_tags` is unchanged at 26. A grep of src/ and tests/ finds no trace tag on a removed id, so no dangling binding is left. The remaining unbacked rows in the touched FRs are FR-369-AC-6/8 (Inspection), FR-372-AC-6 (Analysis), FR-375-AC-6 (Kani, see SR-13011 FND-001) and FR-375-AC-7 (Inspection). The single gap is that the kernel equality seam now has no criterion of its own.

## Dispositions

Round 1, PR head c58ab92.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-375-AC-9/AC-10 and TC-932 now give the kernel seam its own criteria. Both are PLANNED and untagged, as is the new FR-380-AC-7. Strict coverage at c58ab92 vs origin/main b83cab9: rows 194 -> 184, backed 167 -> 167, unbacked 27 -> 17. 13 moved ids were removed (the round-0 12 plus FR-375-AC-6) and 3 new PLANNED kernel rows were added (FR-375-AC-9, AC-10, FR-380-AC-7). Status lies 0, unmatched tags unchanged at 26, and no trace tag references a removed id. |
