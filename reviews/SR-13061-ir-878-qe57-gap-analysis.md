---
id: SR-13061
title: "IR-878 quire-exact PR 57 gap analysis: FR-375 AC-9/AC-10 backing"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@5da4d401063c6e7298f2070db8d522831254fd99 vs origin/main; src/equality.rs, spec/functional/FR-375-composable-equality-effect-seam.md, spec/test-cases/TC-932-kernel-equality-plan-and-admitted-pairs.md, Makefile, .github/workflows/ci.yml"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-932
    type: reviews
---

## Summary

Ticket: IR-878. This is a planless, diff-scoped audit. Plan completion: not assessed. `quire coverage --scope . --strict` (quire 0.36.2, engine 0.50.2) was run on origin/main and on PR head 5da4d40. The spec-review sub-analyses were skipped. The spec edits are two Status lines and one Dependencies sentence, from PLANNED to IMPLEMENTED with a TC-932 link. No requirement or acceptance-criterion text changed.

## Reviewed units

- FR-375-AC-9 (examined): On equal and first-head-unequal bounded kernel operands, `planned_equality_with_plan` returns the Boolean relation and 17 planned pair events from the same formed plan; `plan_equality` agrees on that count, and the admitted meter log contains 17 pair charges on success.
- FR-375-AC-10 (examined): When `Meter::charge` refuses the 17th pair, the result-bearing kernel operation returns the typed stop with exactly the prior 16 pair charges admitted and no result-retain charge; it does not report all 17 planned pairs as admitted. A result-retain refusal after all 17 pair charges retains that admitted prefix.
- FR-375-AC-7 (context_only): No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool control demonstrates both a passing and a deliberately failing dependency-contract case through the actual RT-to-quire-exact dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real callee remains in the outer CBMC graph.
- TC-932 (examined): Direct kernel controls for FR-375-AC-9 and AC-10; Status now "IMPLEMENTED: direct kernel tests in `src/equality.rs` trace FR-375-AC-9 and FR-375-AC-10, including the admitted pair prefixes on refusal."

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | This gap pre-dates the PR, but the PR makes FR-375-AC-10's backing depend on it. The only backing test for AC-10 is `#[cfg(feature = "test-support")]`, as is AC-9's meter-log assertion. `make test` runs `cargo test --features test-support`, so the repo gate executes both. The GitHub workflow `.github/workflows/ci.yml` (manual `workflow_dispatch`) runs plain `cargo test` and clippy without the feature. It never compiles or runs the AC-10 test, yet `quire coverage` counts AC-10 as backed. Either the workflow should call `make test`/`make lint`, or the gap should be ticketed as pre-existing CI debt. | .github/workflows/ci.yml:25-29; Makefile:43-45; src/equality.rs:510-513 |

## Verdict

Coverage delta (strict), origin/main to PR head:

- Rows backed: 167/184 to 169/184.
- Unbacked rows: 17 to 15. The only rows removed from the unbacked list are FR-375-AC-9 and FR-375-AC-10.
- FR-375: 0/3 to 2/3. FR-375-AC-7 (Inspection) stays unbacked as before.
- Contradicted statuses: 0 to 0.
- Rust bound/tagged: 222/226 to 224/228.

The two new tags add a "traces to `TC-932`, which matches no declared row" advisory each. This is the repo-wide pattern for TC ids, the same as TC-992, TC-918 and TC-924, and is not a dangling AC tag. Both FR ids resolve. No new unbacked or contradicted row appears. The FR-375 and TC-932 Status wording is honest. AC-9/AC-10 have direct tagged tests and the strict run reports no status contradiction. FR-277-AC-7 stays PLANNED at RT. The tags are justified clause by clause; see SR-13060 for the clause audit and mutation results. Mergeable as-is. The one low finding is pre-existing CI scope.

## Dispositions

Round 1, commit `56f7eca`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred: pre-existing CI debt | The lead will file a ticket for the workflow's missing `test-support` lane. The workflow was not changed in this fix round. |
