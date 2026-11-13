---
id: SR-17940486709564847047769
title: "IR-1532 PR #157 KERNEL_VERSION constant gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-exact; PR #157 @ c50883c7 on origin/main a4184c4; IR-1532; FR-406 AC-1..AC-5 and TC-954 against tests/ir1532_kernel_version.rs and src/lib.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-406
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-954
    type: reviews
---

## Summary

Ticket: IR-1532. Planless gap analysis of PR #157 against FR-406 (5 ACs) and TC-954, using the computed matrix (`quire matrix --scope . --format json`, quire 0.44.0 / engine 0.59.1) and a reading of every tagged test. Plan completion: not assessed.

Matrix state at c50883c7: FR-406-AC-1 tagged by `kernel_version_is_a_public_u32_const`; AC-2 by `golden_grid_matches_the_record`; AC-3 by `altered_outcome_or_charge_is_reported`; AC-4 and AC-5 are `method-without-symbol`, correct for Inspection criteria (both checked by inspection in the code-review SR and found satisfied). No untagged criterion, no criterion tagged only by an ignored test. The `#[ignore]`d `print_observed_grid` carries no tag and backs nothing, so it inflates nothing.

Reverse gap: the diff adds one public item, `KERNEL_VERSION`, owned by FR-406. No code without an owning requirement. No stubs: every test asserts values that can fail (two TC-954 mutants re-run by the reviewer were killed).

## Verdict

**CONDITIONAL (one medium).** Traceability is complete; the AC-2 evidence is partly hollow on charge amounts.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-406-AC-2 is tagged by a test that compares only the eight semantic-size requests of each charge, not its `work_units` or `result_units`, so the evidence behind AC-2 does not reach Behavior 5's "any ... charge in the grid". A reviewer mutant changing a result-retain charge amount (src/numeric.rs:289, `.results(1)` to `.results(2)`) survives. Same root cause as code-review SR-17940486709564847047768 FND-001; it closes when that is fixed. | tests/ir1532_kernel_version.rs:87-124 |

## Coverage

- Criteria: 5. Tagged: 3 (AC-1, AC-2, AC-3). Inspection, method-without-symbol: 2 (AC-4, AC-5), both satisfied by inspection.
- Untagged: 0. Tagged only by ignored tests: 0.
- Reverse gaps: 0. Stubs or placeholder tests: 0.
- Semantic review: skipped as a separate pass (the reviewer read every tagged test against its AC; findings above).
- Plan completion: not assessed

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-406-AC-1 | examined | `quire_exact::KERNEL_VERSION` is a public `u32` const reachable from the crate root in default, `test-support` and `no_std` builds, defined as an integer literal, usable in a const context, with a doc comment stating the bump rule. |
| FR-406-AC-2 | examined | The golden test evaluates the grid of Behavior 5, with the `test-support` feature enabled, and passes on the unchanged source. |
| FR-406-AC-3 | examined | A fixture in which one expected outcome, and separately one expected charge, is altered makes the golden comparison report a difference. |
| FR-406-AC-4 | examined | The doc comment names the change classes of Behavior 3 that do and do not require a bump, including the verification-cost class. |
| FR-406-AC-5 | examined | In the history of a change that alters an operation outcome or charge, a bump of `KERNEL_VERSION` accompanies it, and no change lowers the value. |
| TC-954 | context_only | Test procedure steps 1-5 and the mutant table |

## Dispositions

Round 1, head 2dc8da75. Verified against `git diff c50883c7..2dc8da75`; see code-review SR-17940486709564847047768 round 1 for the runs and mutants.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The AC-2 test's charge strings now include consumed work and result units for every case; the result-retain mutant at src/numeric.rs:289 is killed. Matrix tagging unchanged (AC-1..3 tagged, AC-4..5 inspection). |
