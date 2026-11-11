---
id: SR-17940486709564847047684
title: "IR-1508 PR #149 native Decimal operations code review"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #149 @ 0cc3bb82 on origin/main 62de2aa; IR-1508; src/native_decimal/{operations,readout,reference,kani_proofs,operations_tests}.rs, src/native_decimal.rs, src/decimal.rs, src/lib.rs, tests/ir1508_native_decimal_ops.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-951
    type: reviews
---

## Summary

Ticket: IR-1508. PR #149 (head 0cc3bb82, branch code/ir-1508-native-decimal-ops), scoped to `git diff origin/main...HEAD` (11 files, +3732/-7). Methods: code-review with the rust-review lane folded in. Reviewed read-only in /home/peter/dev/worktrees/IR-1508-review against FR-403 and TC-951 (QE #147) and the carrier of FR-402 (QE #148).

Reviewer gate run, done once. Each job ran under `flock /tmp/cargo-slot.lock` with its own CARGO_TARGET_DIR. `cargo fmt --all -- --check` passed (rc 0). `cargo clippy --all-targets -- -D warnings` passed (rc 0), and so did the same command with `--features test-support` (rc 0). `cargo clippy --lib --target thumbv7em-none-eabi -- -D warnings` passed (rc 0), and `cargo build --lib --target thumbv7em-none-eabi` succeeded. The coder's pending fmt/clippy re-check of the last commit is therefore closed, and it is green. Focused baseline: `--test ir1508_native_decimal_ops` passed 16 of 16, and `--lib native_decimal` passed 11 of 11 (9 new and 2 carrier). No Kani, no `make ci` and no full suite were run.

Mutation controls. The coder built none, so the reviewer ran eight mutants from the TC-951 table and the brief's list. Each was applied to `operations.rs` in the review worktree, run as one focused test, and reverted. All eight were KILLED by an assertion failure (exit 101, not a compile error):

- **M1**, `gcd256` uses `gcd_steps_128`. Killed by `loss_records_are_the_owners_factored`, line 579.
- **M2**, the ordering forms the aligned operand with an i128 wrapping multiply and no digit-count guard. Killed by `ordering_is_by_value_and_charges_as_the_owner`, line 747.
- **M3**, the doubled remainder uses `wrapping_add` (the capacity ground is skipped). Killed by `admission_edges_sums_products_and_shifts`.
- **M4**, the numerator exponent ground is applied before the zero-divisor exit. Killed by `capacity_edges_of_the_boundary_set`, line 357.
- **M5**, `decimal.rounding` is charged on an exact result (wrong charge point). Killed by `charge_amounts_are_the_owners`.
- **M6**, the log2(10) approximation uses 64 fractional bits, outside 67-94. Killed by `the_log2_ten_approximations_share_a_floor_over_every_u32`.
- **M7**, a Divide overflow is named `ShiftedOperand` instead of `Numerator`. Killed by `native_and_owner_agree_over_the_boundary_set`, through the independent `model()`.
- **M8**, the unary lift-exponent ground is moved after the negation ground (wrong ground order). Killed by the same differential.

No mutant survived. Independent checks: `LOG2_10_LOWER` equals floor(log2(10) x 2^90) exactly. A Stern-Brocot search finds that the simplest fraction in (L/2^90, (L+1)/2^90] has denominator 23,062,019,849,360 > 2^32, so the lower and upper floors agree for every u32 shift, and the AC-9 property holds. (2^32-1)(L+1) is 124 bits. The `digits_of` estimate `(bits x 1233) >> 12` is exact at every bit-length and power-of-ten boundary up to 128 bits.

## Verdict

**CHANGES REQUIRED (one medium, one low).** No defect was found in the production code on the proof path. Checked and found correct:

- **Admission order.** In each plan the grounds run (a) to (g) in order. Ground (a) covers the plan shifts and the lift `T - min(W, T)`, so a zero coefficient is included. A zero divisor returns before any ground, and sizes the `decimal.operands` charge only.
- **Divide.** `kn` and `kd` are computed in u64 with saturating_sub and checked at 38 or less before the `as u32`. `reduce_quotient` moves the sign and refuses 2^127 in either reduced part, `CanonicalQuotient`, matching the four AC-1 literals.
- **The 256-step binary gcd.** It is exactly Behavior 16's step. The zero-operand case leaves the state unchanged, so gcd(0, m) = m and 0/D reduces to 0/1. `g << 1` and `g * (u + v)` cannot overflow.
- **Rounding.** `round_units` takes the floor and the Euclidean remainder over a reduced denominator of at least 2, so `ceiling = floor + 1` cannot overflow. Ground (f) is `checked_add` on the reduced remainder, and the tie rules are right for both signs.
- **Placement.** The proxy branch, where the excess exceeds the digit count, forms no power and records the exact intermediate. Ground (e) applies only when the excess is at most the digit count. Ground (g) applies on the lift only.
- **Loss record.** It is factored with the working-scale power kept separate. The five-strip is a descending binary search over a downward-closed predicate, so it is exact, and its 63-step capacity is at least the 55 fives a u128 can hold.
- **Charges.** `evaluate` charges in the owner's order: operands, then the DivisionByZero exit, then expansion, arithmetic, Inexact, rounding before the out-of-domain refusal, and retain. The ordering uses `power_of_ten_bits` over the whole u32 range and never forms the alignment power. Digit counts decide, equal counts bound the shift at 38, and the 256-bit `widening_mul` is exact.
- **Arithmetic and safety surface.** Proof-path arithmetic has no `unsafe`, no `unwrap` and no `expect`. Every unchecked operation is bounded: table indices are 38 or less, and the u64 sums of u32 scales cannot overflow. `readout.rs` has one `expect` on `DecimalType::new`, outside the proof path, and that invariant holds because `NativeDecimalType` is built only by the owner's own checks.
- **Public API and portability.** The additions are coherent: `admit`, the read-outs, two free functions and seven types. They are no_std-clean, since `operations.rs` uses `core` only, and the thumbv7em clippy pass and build both succeed.
- **Trace tags.** All tags are `#[trace]`. None of the new code uses the `/// Trace:` form.
- **Kani proofs, written and not run.** They are plausibly well-typed. `any_type` calls `NativeDecimalType::new(i128, i128, u64, u64, RoundingMode)` after `assume(lower <= upper, min <= max)`, so the `unwrap` cannot fail. `order_values`, `power_of_ten_bits`, the `LOG2_10_*` constants and the reference items are all `pub(super)`, which is visible from the sibling module `kani_proofs`. The reference uses Euclid unrolled to 256 steps and macro-unrolled multiplicities, so no proof needs an unwind bound. The reference shares no arithmetic with the native path; `native_decision` is only the adapter.
- **Spec gap.** See the spec-review SR for the ruling: (ii), an acceptable implementer choice.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Every proof calls the shared `refines` helper, which emits all six covers, including `zero divisor` (Decision::DivisionByZero). That outcome is unreachable for Add, Subtract, Multiply, Negate and Round, so five of the seven Behavior 37 proofs carry an unsatisfiable cover. TC-951 step 9 credits a proof "only on VERIFIED with the covers satisfied", and AC-12 rejects a proof that "lacks a cover". As written, those five proofs can never be credited, even when their assertions verify. Pass the expected cover set per operation: emit DivisionByZero only from the Divide proof, and keep the completed, rounding, exact-refusal, domain-refusal and capacity covers, all of which are reachable for every operation. | src/native_decimal/kani_proofs.rs:127-142 |
| FND-002 | low | `the_log2_ten_approximations_share_a_floor_over_every_u32` justifies its all-u32 claim with an informal argument: convergent denominators of log2(10) times multiples up to 8, plus 2,000,000 random u32 samples. As stated the argument is incomplete, since it skips semiconvergents and multiples above 8, and the test is not exhaustive. The property does hold: the reviewer found that the simplest fraction in (L/2^90, (L+1)/2^90] has denominator 23,062,019,849,360 > 2^32. Because the FR-403 Status rests the all-u32 half of AC-9 on these host checks, replace the comment and the sampling with that exact bound, computed deterministically on the host: find the least-denominator fraction in the interval and assert that its denominator exceeds u32::MAX. | src/native_decimal/operations_tests.rs:322-376 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| src/native_decimal/operations.rs | examined | admission (plan_combine, plan_multiply, plan_unary, plan_divide, reduce_quotient, place_scaled, place_quotient, exact_tail, rounding_tail), gcd_step..gcd256, strip_fives/strip_twos/loss_record, round_units/tie, settle, evaluate and its charges, compare_shifted_magnitude, widening_mul, order_values, order, bits_of, digits_of, power_of_ten_bits |
| src/native_decimal/readout.rs | examined | to_decimal_type, to_decimal_loss, to_decimal_result, evaluate_native_decimal, order_native_decimals |
| src/native_decimal/reference.rs | examined | FIVE_POWERS, RefType/RefOp/RefLoss/Decision, euclid_256, normalize, loss_of, round_ref, place_scaled, decide_pair, decide_divide, decide, order_ref, native_decision |
| src/native_decimal/kani_proofs.rs | examined | any_type, any_decimal, refines, seven *_refines_owner proofs, native_decimal_log2_ten_approximations_agree |
| src/native_decimal/operations_tests.rs | examined | 9 unit tests |
| tests/ir1508_native_decimal_ops.rs | examined | 16 integration tests |
| src/decimal.rs | examined | DecimalLoss::from_factored, DecimalResult::from_parts, inexact/out_of_domain widened to pub(crate) |
| src/lib.rs | examined | re-exports evaluate_native_decimal, order_native_decimals, AdmittedNativeDecimalOperation, NativeDecimalCapacityQuantity, NativeDecimalCapacityRefusal, NativeDecimalLoss, NativeDecimalOperation, NativeDecimalResult |
| FR-403 Behaviors 1-38 | context_only | Admission, evaluation, charges, shape and evidence of FR-403 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | The fix narrowed FR-403 Behavior 3 (b) to "a shifted operand of `Add` or `Subtract`". The doc comments of `NativeDecimalCapacityQuantity::Numerator` and `::Denominator` still say "(grounds b and c)". They should now read "(ground c)", or the code documentation contradicts the spec it cites. | src/native_decimal/operations.rs:71, src/native_decimal/operations.rs:73 |

## Dispositions

Round 1, head cf296b07. Verified against `git diff 0cc3bb82..cf296b07`. Reviewer job: `--test ir1508_native_decimal_ops` passed 17 of 17, and `--lib native_decimal::operations_tests` passed 8 of 8. A production mutant P1 (the Subtract `decimal.arithmetic` digits drop the left shift) was KILLED by three tests, including both new AC-7 tests.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `refines()` now computes `let divides = matches!(op, NativeDecimalOperation::Divide(..));` and emits `if divides { kani::cover!(matches!(native, Decision::DivisionByZero), "zero divisor"); }`. The other five covers remain, and each is reachable for every operation. |
| FND-002 | fixed | The convergent/random argument is replaced by a deterministic `simplest_between(L, 2^90, L+1, 2^90)`, a continued-fraction recursion that is correct for the open interval. The test checks that `p x 2^90 - L x q` lies in (0, q), so the fraction really is in the interval, and asserts `denominator == 23_062_019_849_360` and `denominator > u32::MAX`. That matches the reviewer's independent value. The closed upper end has a power-of-two reduced denominator far above 2^32, so the open interval decides. |
| FND-003 | still-open | New in disposition pass 1 and routed to the coder. The Numerator/Denominator doc comments still cite "grounds b and c"; the one-line fix is "(ground c)". |
