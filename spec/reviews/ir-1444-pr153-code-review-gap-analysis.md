---
id: SR-17940486709564847047714
title: "IR-1444/IR-1508 PR #153 decimal host-evidence gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-12
scope: "agent-ix/quire-exact PR #153 at 1ead0352 (base e20118a); IR-1508, IR-1444; FR-402 Behavior 9 / AC-10, FR-403 Behavior 17, 37, 40 / AC-9, AC-12, AC-14 against src/native_decimal/tests.rs, src/native_decimal/operations_tests.rs, src/native_decimal/reference.rs, tests/ir1508_native_decimal_ops.rs, tests/ir1444_native_decimal.rs; quire matrix FR-402/FR-403 rows vs origin/main"
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

Ticket: IR-1508 (also IR-1444). Plan completion: not assessed. This analysis checks each host-evidence size and property the reworded spec cites by reading the loops and generators.

- `references_agree_with_the_owner_route_over_the_grid`: 7 BOUNDS x 8 SCALE_RANGES x 17 COEFFICIENTS x 11 SCALES = 10,472 iterations (correct). Each iteration asserts that the native normalized view equals the owner's `normalized()`, that `is_least_scale_pair` accepts the owner pair, and that `is_member` equals `DecimalType::contains`. For the 13 of 17 coefficients that fit `i64` (8,008 cases), it also asserts `from_bindings(..).is_ok() == contains`.
- `the_reference_is_bound_to_the_owner_route_over_the_grid`: 7 x 7 x 6 modes x 24 x 24 x 6 ops = 1,016,064 iterations (correct). `bind` asserts that `native_decision` equals `decide` (outcome kind, retained pair, scale, factored loss) on every case. On non-capacity cases it also compares the owner outcome: exact `Completed` result including the loss under `Eq`, `DivisionByZero`, `inexact()`, or `DecimalOutOfDomain` by `matches!`.
- `..._over_random_operands`: 60,000 rounds (correct), each one operation plus the ordering against both `order_values` and `order_ref` vs the owner `compare`.
- `the_gcd_takes_exactly_256_steps_and_handles_a_zero_operand`: 20,000 random pairs against `euclid_gcd` plus the `(2^126 - 1, 2^126)` pair (correct).
- `native_and_owner_agree_over_the_boundary_set`: 19 types (14 + 5 modes) x 38 operands x 38 x 10 ops = 274,360 iterations (correct). `agree` compares the native and owner `Res` and the recorded `Seen` (ordered charges) on every non-capacity case, and checks capacity refusals against the independent `model`.

`quire matrix`, before and after on FR-402/FR-403: no row lost a binding. FR-402-AC-10 keeps its two bindings (tests.rs:51, :91); main already bound them to a Kani-worded AC. FR-403-AC-12 goes from untagged to tagged (4 tests). FR-403-AC-9 gains the exhaustive test. FR-403-AC-14 is new and tagged. FR-402-AC-13 and FR-403-AC-15 are new PLANNED rows shown as `method-without-symbol`, the same as the existing Inspection rows FR-402-AC-11 and FR-403-AC-13.

## Verdict

The cited sizes are real and the tests compare what FR-403-AC-12 and FR-402-AC-10 say they compare. The newly added FR-403-AC-12 tags sit on tests that assert the property. Two Behavior sentences still claim more than the tests do, and the covers the Kani harnesses carried were dropped without a host replacement (FND-001, FND-002). Those two need fixing in this PR; FND-003 is wording.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-402 Behavior 9 says the host test binds "the constructor's outcome and retained pair ... to an independent bounded mathematical reference", and that "the reference for the constructor takes the lift of Behavior 5 including values at which it wraps or exceeds the native width". Over the 10,472 grid, the constructor is only compared as `from_bindings(..).is_ok()` vs `owner.contains`, for 8,008 cases. Its retained pair and refusal variant are never compared there. `reference.rs` has no constructor or lift reference: `is_member` is evaluated on the owner's normalized pair. The retained pair and `OutsideCarrierDomain` are bound to the owner, not to a reference, and only over the TC-950 boundary set in `tests/ir1444_native_decimal.rs::read_out_and_owner_differential`. AC-10 describes the test accurately; Behavior 9 overclaims. Reword Behavior 9 to match AC-10, or assert the retained pair and refusal in the grid. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:56, src/native_decimal/tests.rs:76-82 |
| FND-002 | medium | FR-403 Behavior 37 now says the host tests run "with the completed path, a rounding step, each refusal and a capacity refusal reached". The Kani covers that carried this obligation were deleted, and no host test asserts it. The grid asserts only `bound > 20_000 && capacity > 5_000`, the random test asserts `bound > 5_000`, and the differential asserts `compared > 10_000 && refused > 1_000`. Nothing counts `DivisionByZero`, `Inexact`, `OutOfDomain` or a `Completed` with `loss: Some`, so a generator change could silently stop reaching a refusal or rounding path. Count each `Decision` variant in `bind` and assert every one is non-zero, or drop the clause from Behavior 37. | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:94, src/native_decimal/operations_tests.rs:183-239 |
| FND-003 | low | The FR-403-AC-12 sizes are loop iterations, not distinct cases. `Negate` and `Round` ignore the second operand, so the grid's 1,016,064 iterations hold 691,488 distinct cases, and the differential's 274,360 hold 220,932. State them as "iterations", or count distinct cases. | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:115 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-402-AC-10 | examined | The constructor and the normalized view agree with their independent references and with the owner route on the boundary grid of Behavior 9 ... 10,472 cases |
| FR-402 Behavior 9 | examined | The kernel SHALL carry a host test that binds the constructor's outcome and retained pair, and the normalized view, each to an independent bounded mathematical reference |
| FR-403-AC-12 | examined | The six operations and the ordering agree with their independent references and with the owner route on host evidence of these sizes ... |
| FR-403 Behavior 37 | examined | The kernel SHALL carry host tests that bind the admission decision, the outcome, the completed value and the loss record ... with the completed path, a rounding step, each refusal and a capacity refusal reached. |
| FR-403 Behavior 17 | examined | The gcd bound of Behavior 16 SHALL be covered by the gcd host test of Behavior 37 ... includes the pair that needs 252 steps. |
| FR-403-AC-9 | examined | ... checked by the closed-form test ... and by the exhaustive host test of FR-403-AC-14, not by a proof. |
| FR-403-AC-14 | examined | For every `u32` shift (2^32 shifts, in 16 threads) ... test `bits_of_a_power_of_ten_matches_the_reference_for_every_u32_shift`. |
| FR-402-AC-13 | examined | PLANNED, waiting on a future Verus proof track and not on solver capacity or on a memory budget |
| FR-403-AC-15 | examined | PLANNED, waiting on a future Verus proof track and not on solver capacity or on a memory budget |
| tests/ir1444_native_decimal.rs::read_out_and_owner_differential | context_only | built.is_ok() == owner.contains; representation and normalized pair vs owner |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | FR-402-AC-10 says 'for each of the 8,008 coefficients that fit `i64`'; 8,008 is the number of grid cases (13 fitting coefficients x 616 type/scale combinations), not coefficients. Say 'for each of the 8,008 cases whose coefficient fits `i64`'. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:75 |

## Dispositions

Round 1 verified at fix head 377414e1 (merge 52358d3 of main e82db48 is a clean, conflict-free merge: `git diff e82db48 52358d3` equals `git diff e20118a 1ead035`, so it carries only #152's files beyond this PR). Reviewer runs under flock: fmt --check, clippy -D warnings (default, test-support, thumb lib), cargo doc -D warnings --all-features clean; `cargo test --lib native_decimal` 13 passed, `--test ir1508_native_decimal_ops` 17 passed, `--test ir1444_native_decimal` 11 passed, each also with test-support; quire validate clean; matrix unchanged in status (FR-403-AC-14 and AC-9 gain the new constant test). Mutants: LOG2_10_LOWER L-1 and L+1 each fail `the_log2_ten_constant_is_the_floor_of_the_true_value` (left 4112347796288069798443794287) and the closed-form test; MODES without Exact fails both grid and random tests with 'exact refusal was never reached'; exact refusal disabled in kernel and reference fails both at the owner comparison (operations_tests.rs:192). log2(10) digits checked independently (Python decimal, 80 digits): 3.3219280948873623478703194294893901758648|3139..., and floor(log2(10)*2^90) = L.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
