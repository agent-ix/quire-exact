---
id: SR-17940486709564847047716
title: "IR-1444/IR-1508 PR #153 spec-review/ears"
type: SpecReview
analysis: ears-conformance
date: 2026-11-12
scope: "agent-ix/quire-exact PR #153 at 1ead0352 (base e20118a); IR-1508, IR-1444; new or edited Behaviors FR-402 9, 11; FR-403 17, 37, 40, 41"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
---

## Summary

Ticket: IR-1508 (also IR-1444). EARS check of the new and edited Behavior statements. `quire validate` reports no `ears:` diagnostics on FR-402 or FR-403, on either the branch or main.

Clean:

- FR-402 Behavior 9 and FR-403 Behaviors 17, 37 and 40 are ubiquitous statements with one `SHALL` each and one obligation each: carry a host test, or be covered by a named test.
- Behavior 40 lists three checks of one test, which is one obligation.

## Verdict

Low only. The two conditional Behaviors are not single-obligation (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-402 Behavior 11 and FR-403 Behavior 41 are each a `WHERE` statement with two `SHALL` clauses ("SHALL come from a separate Verus proof track and SHALL be recorded as its own acceptance row") plus an "until then" claim clause. FR-403 Behavior 41 adds a third assertion ("the six operations rest on the host evidence of Behavior 37"). Split each into one optional-feature requirement (WHERE delivered, it SHALL come from a separate Verus track) and move the recording and non-claim text to Notes or Status. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:58, spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:98 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-402 Behavior 9 | examined | The kernel SHALL carry a host test that binds the constructor's outcome and retained pair, and the normalized view ... |
| FR-402 Behavior 11 | examined | WHERE a full-width refinement proof of the constructor and the normalized view ... is delivered, it SHALL come from a separate Verus proof track and SHALL be recorded as its own acceptance row; until then no full-width proof is claimed. |
| FR-403 Behavior 17 | examined | The gcd bound of Behavior 16 SHALL be covered by the gcd host test of Behavior 37 ... |
| FR-403 Behavior 37 | examined | The kernel SHALL carry host tests that bind the admission decision, the outcome, the completed value and the loss record ... |
| FR-403 Behavior 40 | examined | The kernel SHALL carry a host test that checks every one of the 2^32 shifts ... |
| FR-403 Behavior 41 | examined | WHERE a full-width refinement proof of the six operations and the ordering is delivered, it SHALL come from a separate Verus proof track and SHALL be recorded as its own acceptance row; until then ... |

## Dispositions

Round 1 verified at fix head 377414e1 (merge 52358d3 of main e82db48 is a clean, conflict-free merge: `git diff e82db48 52358d3` equals `git diff e20118a 1ead035`, so it carries only #152's files beyond this PR). Reviewer runs under flock: fmt --check, clippy -D warnings (default, test-support, thumb lib), cargo doc -D warnings --all-features clean; `cargo test --lib native_decimal` 13 passed, `--test ir1508_native_decimal_ops` 17 passed, `--test ir1444_native_decimal` 11 passed, each also with test-support; quire validate clean; matrix unchanged in status (FR-403-AC-14 and AC-9 gain the new constant test). Mutants: LOG2_10_LOWER L-1 and L+1 each fail `the_log2_ten_constant_is_the_floor_of_the_true_value` (left 4112347796288069798443794287) and the closed-form test; MODES without Exact fails both grid and random tests with 'exact refusal was never reached'; exact refusal disabled in kernel and reference fails both at the owner comparison (operations_tests.rs:192). log2(10) digits checked independently (Python decimal, 80 digits): 3.3219280948873623478703194294893901758648|3139..., and floor(log2(10)*2^90) = L.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
