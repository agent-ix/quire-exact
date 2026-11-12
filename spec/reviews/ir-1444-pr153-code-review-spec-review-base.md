---
id: SR-17940486709564847047715
title: "IR-1444/IR-1508 PR #153 FR-402/FR-403 spec review"
type: SpecReview
analysis: base
date: 2026-11-12
scope: "agent-ix/quire-exact PR #153 at 1ead0352 (base e20118a); IR-1508, IR-1444; spec/functional/FR-402-kani-tractable-native-decimal-carrier.md, spec/functional/FR-403-native-decimal-operations-refine-owner-path.md, spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md, spec/test-cases/TC-951-native-decimal-operations-match-owner-path.md; evidence /home/peter/dev/worktrees/proofs-gen/evidence/decimal/RESULTS.md; PR body"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-950
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-951
    type: reviews
---

## Summary

Ticket: IR-1508 (also IR-1444). This review checks that the rewording weakens the verification method honestly, as the ruling requires: host tests are the evidence, the full-width proof is WAITING on a Verus track and never pending capacity, and no decimal Kani verdict or bounded proof is claimed.

Checked clean:

- FR-402-AC-13 and FR-403-AC-15 say "PLANNED, waiting on a future Verus proof track and not on solver capacity or on a memory budget".
- No row claims a decimal Kani verdict, and no "no unwind bound" wording remains in FR-403-AC-12 or FR-402-AC-10.
- FR-402 and FR-403 Status and TC-950/TC-951 Status say no Kani harness remains.
- TC-950 step 5 and TC-951 steps 6 and 9 name the host tests, and Expected Results no longer require proofs to verify.
- FR-403 Behavior 17 rebinds the gcd bound to the gcd host test.
- The PR body states what was removed (ten harnesses, `kani_proofs.rs`, the `mod` line, `reference` narrowed to `cfg(test)`) and what is claimed (no universal 128-bit refinement, no Kani or bounded proof, no narrow-width instantiation). Its measurement sentence ("the operation proofs at 19 GiB or more") agrees with RESULTS.md.

## Verdict

Honest in substance. One evidence sentence in the spec misstates the measurement the PR relies on (FND-001); fix its wording before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-403-AC-15 says "The ten Kani harnesses ... never returned at 600 s with 19 GiB or more of memory". RESULTS.md shows that only the six operation harnesses reached 19.0-19.5 GB. The constructor, normalized, ordering and log2 harnesses peaked at 0.36-0.51 GB and stalled in their final (or, for log2, first) UNSAT solver query. The runs also used `--unwind 3`. The PR body states this correctly ("the operation proofs at 19 GiB or more"). Reword to "none returned within 600 s (the six operation harnesses at 19 GiB or more, the other four below 1 GB in an unreturned solver query)". | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:118 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-402-AC-10 | examined | ... This is host evidence over the grid and is not a proof over all 128-bit inputs; the Kani harnesses that were written for this criterion never returned and are removed. |
| FR-402-AC-13 | examined | PLANNED, waiting on a future Verus proof track and not on solver capacity or on a memory budget ... |
| FR-403-AC-9 | examined | ... checked by the closed-form test ... and by the exhaustive host test of FR-403-AC-14, not by a proof. |
| FR-403-AC-12 | examined | ... This is host evidence and is not a proof over all 128-bit inputs. |
| FR-403-AC-14 | examined | For every `u32` shift (2^32 shifts, in 16 threads) ... |
| FR-403-AC-15 | examined | PLANNED ... The ten Kani harnesses first written for this requirement ... never returned at 600 s with 19 GiB or more of memory ... |
| FR-403 Behavior 17 | examined | The gcd bound of Behavior 16 SHALL be covered by the gcd host test of Behavior 37 ... |
| TC-950 steps 5, Expected Results, Status | examined | For AC-10, run the host tests `references_agree_with_the_owner_route_over_the_grid` and `least_scale_reference_rejects_wrong_pairs` ... |
| TC-951 steps 6, 9, Expected Results, Status | examined | For AC-12, run the host tests named by that criterion ... AC-15 is not executable: it waits on a separate Verus track. |
| PR #153 body | examined | Removed / Not claimed / Checks run sections |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | FR-403-AC-15 says the four small harnesses 'stalled in a final solver query'; RESULTS.md records the log2 harness stalled in its first UNSAT query (13.6k vars / 70k clauses), the other three in their final one. Say 'stalled in an unreturned solver query'. | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:120 |

## Dispositions

Round 1 verified at fix head 377414e1 (merge 52358d3 of main e82db48 is a clean, conflict-free merge: `git diff e82db48 52358d3` equals `git diff e20118a 1ead035`, so it carries only #152's files beyond this PR). Reviewer runs under flock: fmt --check, clippy -D warnings (default, test-support, thumb lib), cargo doc -D warnings --all-features clean; `cargo test --lib native_decimal` 13 passed, `--test ir1508_native_decimal_ops` 17 passed, `--test ir1444_native_decimal` 11 passed, each also with test-support; quire validate clean; matrix unchanged in status (FR-403-AC-14 and AC-9 gain the new constant test). Mutants: LOG2_10_LOWER L-1 and L+1 each fail `the_log2_ten_constant_is_the_floor_of_the_true_value` (left 4112347796288069798443794287) and the closed-form test; MODES without Exact fails both grid and random tests with 'exact refusal was never reached'; exact refusal disabled in kernel and reference fails both at the owner comparison (operations_tests.rs:192). log2(10) digits checked independently (Python decimal, 80 digits): 3.3219280948873623478703194294893901758648|3139..., and floor(log2(10)*2^90) = L.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
