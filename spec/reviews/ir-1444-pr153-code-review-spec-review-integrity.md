---
id: SR-17940486709564847047717
title: "IR-1444/IR-1508 PR #153 spec-review/integrity"
type: SpecReview
analysis: integrity
date: 2026-11-12
scope: "agent-ix/quire-exact PR #153 at 1ead0352 (base e20118a); IR-1508, IR-1444; FR-402, FR-403, TC-950, TC-951 cross-consistency after the harness removal"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/TC-951
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
---

## Summary

Ticket: IR-1508 (also IR-1444). Integrity check across the four changed spec files. `quire validate` passes on them.

Clean:

- The TC-951 Description range is now AC-1 to AC-15. TC-950's still covers AC-1 to AC-12, which is consistent: AC-13 is PLANNED and TC-950 step 5 says so.
- The Status sections of FR-402, FR-403, TC-950 and TC-951 agree with each other and with the matrix.
- The FR-403 Dependencies bullet that pointed at the CLAUDE.md Kani instruction is replaced by the Verus-track bullet.

## Verdict

Low only. One stale sentence remains (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The TC-951 Description still ends "The host steps have run (see Status); the Kani proofs and the CG measurement have not." No Kani proof remains, and TC-951 Status now says so. Change it to "the CG measurement (step 11) has not run, and AC-15 waits on a Verus track". | spec/test-cases/TC-951-native-decimal-operations-match-owner-path.md:14 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| TC-951 Description | examined | Verify FR-403-AC-1 through FR-403-AC-15 ... the Kani proofs and the CG measurement have not. |
| TC-950 Description | examined | Verify FR-402-AC-1 through FR-402-AC-12 through the public kernel API, the host differential and the recorded measurement. |
| FR-402 Status | examined | Implemented (IR-1444) ... The full-width proof is PLANNED on a separate Verus track (AC-13) and is not claimed. |
| FR-403 Status | examined | Implemented (IR-1508) ... the full-width proof is PLANNED on a separate Verus track (AC-15) and is not claimed. |
| FR-403 Dependencies | examined | A full-width proof of the operations waits on a separate Verus proof track (Behavior 41). |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The new TC-951 mutant row 'The exact-rounding refusal is unreachable (rounding mode dispatch)' says the grid and random tests fail 'on the unreached `exact refusal` class'. A mutant at the dispatch site (exact refusal disabled in `rounding_tail`, with or without the reference) fails both tests earlier, at the owner comparison in `bind` (operations_tests.rs:192). Only an input mutant (MODES without Exact) fails by the class name. Both tests fail either way; correct the expected failure text. | spec/test-cases/TC-951-native-decimal-operations-match-owner-path.md:85 |

## Dispositions

Round 1 verified at fix head 377414e1 (merge 52358d3 of main e82db48 is a clean, conflict-free merge: `git diff e82db48 52358d3` equals `git diff e20118a 1ead035`, so it carries only #152's files beyond this PR). Reviewer runs under flock: fmt --check, clippy -D warnings (default, test-support, thumb lib), cargo doc -D warnings --all-features clean; `cargo test --lib native_decimal` 13 passed, `--test ir1508_native_decimal_ops` 17 passed, `--test ir1444_native_decimal` 11 passed, each also with test-support; quire validate clean; matrix unchanged in status (FR-403-AC-14 and AC-9 gain the new constant test). Mutants: LOG2_10_LOWER L-1 and L+1 each fail `the_log2_ten_constant_is_the_floor_of_the_true_value` (left 4112347796288069798443794287) and the closed-form test; MODES without Exact fails both grid and random tests with 'exact refusal was never reached'; exact refusal disabled in kernel and reference fails both at the owner comparison (operations_tests.rs:192). log2(10) digits checked independently (Python decimal, 80 digits): 3.3219280948873623478703194294893901758648|3139..., and floor(log2(10)*2^90) = L.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
