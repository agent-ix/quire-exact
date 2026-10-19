---
id: SR-13012
title: "quire-exact PR 56 IR-878 integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact; PR #56 head 159f170d8ab7e339fe636934e26f602667fc0153 vs origin/main b83cab9; spec/functional/FR-369, FR-372, FR-375, FR-376, FR-380; spec/test-cases/TC-923; ticket IR-878"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: reviews
---

## Summary

Structural and internal consistency of the six changed files after the criteria were removed.

Ticket: IR-878

## Reviewed units

- TC-923 (examined): This procedure is retained as the IR-241 Kani-lane context for FR-375-AC-6 and the FR-375-AC-7 cross-crate tool inspection. The actual `CheckedEquality::evaluate` path belongs to quire-contract-runtime `src/exact/equality.rs`; its end-to-end test obligations are RT FR-277-AC-1 through AC-6. Use the fixed E21 depth-four `List` operands: 16 occurrences per operand and 17 occurrence-path pairs. No test tag or proof credit is assigned by this procedure.
- FR-375-AC-7 (examined): No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool control demonstrates both a passing and a deliberately failing dependency-contract case through the exact QSL-to-quire-exact dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real callee remains in the outer CBMC graph.
- FR-369 (examined): QSV's QSpec FR-142/FR-143/FR-149 contracts must test their corresponding internal-fault routes without adding a QSV-local duplicate requirement.
- FR-375 (examined): When a checked equality caller evaluates two admitted operands, the `quire-exact` kernel SHALL expose the exact equality relation and planned `equality.pair` count from one plan, while its `Meter` records only admitted charges. The runtime's actual checked evaluation and its end-to-end controls are allocated to [RT FR-277](ix://agent-ix/quire-contract-runtime/FR-277).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-923 still `verifies` FR-375, and its Test Procedure steps 1-6 and Expected Results 1-6 are the end-to-end RT `CheckedEquality::evaluate` controls now owned by RT FR-277. They include oracle values that are not in the AC text: WorkUnits 51, ResultUnits 1, ValueOccurrences 17, and `charge_point: EqualityPlan`. RT FR-277 has no TC (its matrix row is '—'). The only written procedure for the re-homed criteria therefore lives in a leaf that cannot run it, and the owner has none. The new description says no credit is assigned, but steps 1-6 serve neither AC-6 (Kani) nor AC-7 (inspection). The procedure should move to an RT TC: re-home it, do not copy it. | spec/test-cases/TC-923-composable-equality-effect-seam.md:13-36 |
| FND-002 | low | The PR now says the evaluator lives in quire-contract-runtime, and TC-923 is retained 'for the FR-375-AC-7 cross-crate tool inspection'. Yet AC-7 and TC-923 step 7 still name the 'QSL-to-quire-exact' dependency boundary, and FR-375 Behavior/Dependencies still ground the fixture in `qsl-eval/tests/it/equality_matrix.rs::e21_operands`. Two readers would disagree about which crate edge AC-7 inspects. | spec/functional/FR-375-composable-equality-effect-seam.md:32 |
| FND-003 | low | FR-369 Dependencies still says QSV must test its internal-fault routes 'without adding a QSV-local duplicate requirement'. The PR's own new paragraph allocates FR-369-AC-9 to a new QSV-local requirement (QSV FR-122-AC-1). That is a move, not a duplicate, but the unchanged prose now contradicts the allocation. It should name QSV FR-122 (and RT FR-278 / CG FR-042) as the owners. | spec/functional/FR-369-typed-checked-invariant-causes.md:174-186 |

## Verdict

The ids stay unique and sequential where they remain, `quire validate` is clean, and no test tag in src/ or tests/ references a removed id. The three consistency defects above are left behind by the move.
