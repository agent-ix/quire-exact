---
id: SR-13060
title: "IR-878 quire-exact PR 57 code review: FR-375 AC-9/AC-10 kernel trace tags"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@5da4d401063c6e7298f2070db8d522831254fd99 vs origin/main; src/equality.rs, spec/functional/FR-375-composable-equality-effect-seam.md, spec/test-cases/TC-932-kernel-equality-plan-and-admitted-pairs.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: reviews
---

## Summary

Ticket: IR-878. The diff adds `#[trace("TC-932", "FR-375-AC-9")]` to `kernel_result_bearing_evaluation_returns_relation_plan_and_charge_order` and `#[trace("TC-932", "FR-375-AC-10")]` to `pair_and_result_refusals_stop_at_the_first_denied_charge` (behind `cfg(feature = "test-support")`). It replaces the AC-10 test's loose prefix checks (`[0]`, `[1]`, `all(EqualityPair)`) with exact `assert_eq!` on the whole admitted charge-point vector: plan-form, plan and 16 pairs for the pair denial; the same plus one pair (17) for the result-retain denial. It also flips the FR-375 and TC-932 Status lines from PLANNED to IMPLEMENTED. The rust-review lane ran over the test hunk. It uses `#[trace]` only and adds no `/// Trace:` lines. It adds no panic surface beyond the existing test `unwrap`s, and its idioms are fine. `make fmt-check`, `make lint` (clippy `-D warnings` for default, test-support and thumbv7em no_std) and `make test` (default, then `--features test-support`) all pass at the PR head.

Mutation checks were run in a throwaway worktree, one mutant at a time over `planned_equality_stop`, `plan_equality` and `plan_pairs`. In each run the target tests were executed in both feature configs.

- M1 `equal: !plan.equal`: killed by AC-9 in both configs.
- M2 result `pair_events + 1`: killed by AC-9 in both configs.
- M3 one fewer pair charge: killed by AC-9 in both configs and by AC-10.
- M4 pair refusal swallowed (`let _ =`): killed by AC-10 (test-support only).
- M5 result-retain admitted before the pair-stop returns: killed by AC-10.
- M6 result-retain charge removed: killed by AC-9 in both configs and by AC-10.
- M7 result-retain refusal swallowed: killed by AC-10.
- M8 `plan_equality` count `+ 1`: killed by AC-9 in both configs.
- M9 relation fold `&&` changed to `||`: killed by AC-9 through the first-head-unequal case.
- M10 result-retain charged before the pairs: **survives the default-feature run** of the AC-9 test. It is killed only under `--features test-support`, by AC-9's charge-order assertion and by AC-10. See FND-001.

## Examined criteria

- FR-375-AC-9 (examined): On equal and first-head-unequal bounded kernel operands, `planned_equality_with_plan` returns the Boolean relation and 17 planned pair events from the same formed plan; `plan_equality` agrees on that count, and the admitted meter log contains 17 pair charges on success.
- FR-375-AC-10 (examined): When `Meter::charge` refuses the 17th pair, the result-bearing kernel operation returns the typed stop with exactly the prior 16 pair charges admitted and no result-retain charge; it does not report all 17 planned pairs as admitted. A result-retain refusal after all 17 pair charges retains that admitted prefix.

Every clause of both criteria is asserted. AC-9 runs both operand pairs, asserts the relation (`true`/`false`), `plan_equality` = 17 and result plan = 17. Under test-support it asserts the exact admitted log `[plan-form, plan, pair x17, result-retain]`. AC-10 asserts `Incomplete` at `EqualityPair` with exactly plan-form, plan and 16 pairs admitted and `ResultUnits` 0. It asserts `Incomplete` at `EqualityResultRetain` with plan-form, plan and 17 pairs admitted and no result-retain.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The AC-9 clause "the admitted meter log contains 17 pair charges" is asserted only inside `#[cfg(feature = "test-support")]` in a test tagged FR-375-AC-9 that also runs without the feature. In the default run the test checks only aggregates (`admission_count == 20`, work 51, results 1, occurrences 17). A mutant that charges result-retain before the 17 pairs passes the default run. `make test` runs the test-support pass, so the clause is enforced at the repo gate. The default-feature binding is weaker than the tag claims. Pinning the `EqualityPair` count through a default-feature observable, or gating the whole test on the feature, would close it. | src/equality.rs:452-468 |

## Verdict

Both tags are justified. Every clause of AC-9 and AC-10 has an assertion that a real mutant kills under `make test`. The AC-10 assertion tightening is a strict improvement, from prefix-shape checks to exact admitted vectors. The one low finding is that AC-9's meter-log clause is only checked under the test-support feature. Mergeable as-is.

## Dispositions

Round 1, commit `56f7eca`.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-9's tagged test now denies the first result-retain charge through the public meter seam and asserts 19 admissions, 50 work units, and zero result units after the stop. This exposes the two setup charges followed by all 17 pair charges without `test-support`; the feature-gated exact-log assertion remains. The early result-retain mutant fails the focused default test (2 admissions versus 19) and the `test-support` test (wrong charge order). The restored source passes both focused tests; `make lint` passes. |
