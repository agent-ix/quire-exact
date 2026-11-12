---
id: TC-951
title: "Native Decimal operations match the owner's metered path"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-403
    type: verifies
---
# TC-951: Native Decimal operations match the owner's metered path

## Description

Verify FR-403-AC-1 through FR-403-AC-15 by running the native operations beside the owner's `evaluate_decimal` and `order_numbers`, each under a `Meter` with the same limits. Each mutant below is a buildable variant of the named site in a scratch copy and names the input, or the inspection, that fails it and the step at which it fails; a mutant that no input and no inspection distinguishes from the unmutated body is not admitted. The host steps have run (see Status); the CG measurement has not, and no Kani harness remains.

## Test Procedure

The operand and type set is fixed before the implementation is written, from independent literals. A carried Decimal is `(coefficient, scale)` built by the total constructor of FR-402 Behavior 3 (or by a binding where the coefficient is an `i64`). Types are written `Decimal[lo, hi; smin, smax]` with the rounding spelling named where it matters.

- The cases of FR-403-AC-1 to AC-6, AC-8 and AC-9, with their types, including `Round` of `(1004, 1)` into `Decimal[0, 100; 0, 0; nearest-even]`, `Divide` of `(3, 0)` by `(9, 0)` in `Decimal[-100, 100; 0, 2; nearest-even]` `Multiply` of `(0, 2)` and `(7, 2)` and `Divide` of `(0, 0)` by `(12, 0)`, each in `Decimal[0, 100; 0, 2; Exact]`.
- Capacity edges: the sums, products, shifts and aligned magnitudes of FR-403-AC-1; `Divide` of `(1, 0)` by `(1, 38)` in `Decimal[0, 10^38; 0, 38]` (numerator shift 76: capacity refusal naming the exponent) and `Divide` of `(1, 0)` by `(0, 1)` in the same type (zero divisor: admitted, `Undefined(DivisionByZero)` after `decimal.operands`); in `Decimal[i128::MIN, i128::MAX; 0, 0; toward-zero]`, `Divide` of `(i128::MIN, 0)` by `(-3, 0)` and of `(1, 0)` by `(i128::MIN, 0)` (capacity refusals naming the canonical quotient), `Divide` of `(i128::MIN, 0)` by `(-2, 0)` (admitted, completes with `(2^126, 0)`) and by `(1, 0)` (admitted, completes with `(i128::MIN, 0)`); the orderings `lt`, `le`, `gt`, `ge` of `(2, 0)` against `(1, 38)`, `gt` of `(1, 0)` against `(1, 39)` and against `(1, 4294967295)` (all admitted); `bits(10^k)` at `k` from 0 to 200 (the owner's value) and at 2^31 - 1, 2^31, 2^32 - 2 and 2^32 - 1 (7133786261, 7133786264, 14267572521, 14267572524); `Divide` of `(1, 0)` by `(1, 0)` in `Decimal[0, 100; 0, 2]` (admitted, completes with `(100, 2)`); `Add` of `(0, 0)` and `(0, 39)` (exponent 39) and `Negate` of `(0, 0)` in `Decimal[0, 1; 0, 100]` (lift exponent 100): each a capacity refusal naming the exponent (a zero placed at `T` is admitted: `Multiply` of `(0, 2)` and `(7, 2)` in `Decimal[0, 100; 0, 2]`); in `Decimal[0, 1; 0, 0; nearest-even]`, `Divide` of `(2^126, 0)` by `(2^126 + 1, 0)` (quotient 0, remainder `2^126`, whose double `2^127` does not fit: capacity refusal naming the doubled remainder), `Divide` of `(2^126 - 1, 0)` by `(2^126, 0)` (252 gcd steps; loss numerator `2^126 - 1`, cofactor 1, twos 126, fives 0; doubled remainder `2^127 - 2`, admitted, completes with `(1, 0)`) and `Divide` of `(2^126, 0)` by `(2^127 - 2, 0)` (the pair reduces by 2 to `2^125 / (2^126 - 1)`, whose doubled remainder `2^126` fits: admitted, completes with `(1, 0)`; the unreduced doubled remainder `2^127` would not fit); in `Decimal[i128::MIN, i128::MAX; 0, 0; toward-zero]`, `Round` of `(10^38, 1)` (admitted), `Round` of `(i128::MAX, 38)` (excess 38, power `10^38` fits: admitted, completes with `(1, 0)`) and `Round` of `(i128::MAX, 39)` (excess 39, digits 39, power `10^39` does not fit: capacity refusal naming the power of ten); in `Decimal[0, 100; 0, 2; toward-positive]`, `Multiply` of `(1, 4294967295)` and `(1, 4294967295)` (working scale `8589934590`, a proxy rounding step with no power of ten: admitted, completes with `(1, 2)` and the loss `1/10^8589934590`, twos and fives both `8589934590`); in `Decimal[i128::MIN, i128::MAX; 38, 38]`, `Negate` of `(-1, 0)` (retained `10^38` fits: admitted) and `Negate` of `(-2, 0)` (retained `2 x 10^38` does not fit: capacity refusal naming the retained coefficient).
- Meter limits: for each case that completes, the limits that are sufficient, exactly the charge of each point reached, and one less than it, for `integer_bits`, `decimal_digits`, `scale_expansion`, `value_occurrences`, `work_units` and `result_units`.

1. For AC-2, AC-4, AC-5, AC-6 and AC-8, evaluate every case natively and compare the typed outcome, the retained pair, the loss record (numerator, cofactor, twos, fives, rounded pair, mode) and, for AC-6 and AC-8, each charge point with its limit kinds and amounts with independently written literals.
2. For AC-1, evaluate every capacity-edge case natively. For each capacity refusal, assert the refusal and the quantity it names and assert that a `Meter` presented to it is unchanged, and record what the owner route returns without asserting equality. For each admitted edge, assert the admission and the literal outcome (the completed pair, or the typed outcome).
3. For AC-3, check each typed outcome against its charge point: the zero-divisor cases against a recording `Meter` (charges: `decimal.operands` only, and admission not refused) and against a zero budget; each `exact` refusal against a recording `Meter` (no `decimal.rounding`); the `DecimalOutOfDomain` cases against a recording `Meter` (no `decimal.result-retain`); and the completed `Round` of `(1004, 1)`.
4. For AC-4 and AC-5, evaluate the listed rounding cases in all five modes and compare the retained pairs and loss records with the literals of the criteria, including `Divide` of `(3, 0)` by `(9, 0)`.
5. For AC-7, run each case at each limit of the Meter list natively and by the owner route on fresh Meters and on one cumulative Meter, and compare every charge point reached, its limit kind and amount, the order, the denied next charge, every counter and the incomplete record.
6. For AC-9, compare the native `bits(10^k)` with the owner's `power_of_ten_bits(k)` at the listed `k`, and run the exhaustive host test over all `u32` values that the lower and upper approximations agree.
7. For AC-10, inspect the source of the admission, the evaluation, the gcd and the ordering for a `cfg(kani)` token, for loops, iterator adapters and recursion, and for owner `Integer`, `Decimal` and `Rational` types and `Vec`, `Box` and `alloc` uses (the native types' own names are not scanned); check `Copy` of the result and the loss record; and run the capacity and completed cases inside a counting global allocator window around admission, asserting zero allocations.
8. For AC-11, evaluate every case a second time by the owner route and assert that the native read-out `DecimalResult` equals the owner's under `Eq`, that the typed outcomes are equal, and that the complete list of disagreements is the set of native capacity refusals, each checked against its named quantity.
9. For AC-12, run the host tests named by that criterion (the grid, the random operands, the gcd pairs and the differential of `tests/ir1508_native_decimal_ops.rs`). For AC-14, run `bits_of_a_power_of_ten_matches_the_reference_for_every_u32_shift` over all 2^32 shifts. AC-15 is not executable: it waits on a separate Verus track.
10. Run each mutant: the input mutants through the check of the step named in the table, the inspection mutants through step 7.
11. For AC-13, once CG's decimal row (IR-259) consumes the delivered operations, run it under the authorized runner and record the peak memory and its provenance, time, tool version and terminal outcome under IR-1508. Until then this step is not executable and AC-13 stays unexecuted; no fit is claimed.

### Mutants

| Mutant (site) | Distinguishing input | Fails at |
| --- | --- | --- |
| `Add` is computed as `Subtract` (operation dispatch) | `Add` of `(5, 1)` and `(5, 1)`, expected `(100, 2)` | step 1 |
| `Subtract` has its operands swapped (operation dispatch) | `Subtract` of `(5, 1)` and `(1, 1)`, expected `(40, 2)` (the mutant gives -40, outside the domain) | step 1 |
| The result is retained normalized (result retention) | `Add` of `(5, 1)` and `(5, 1)`, expected the retained pair `(100, 2)` | step 1 |
| The result is retained at the larger operand scale instead of `T` (result retention) | `Negate` of `(0, 0)`, expected `(0, 2)` | step 1 |
| The lift to `T` is skipped (result retention) | `Round` of `(5, 1)`, expected `(50, 2)` | step 1 |
| Nearest-away uses the nearest-even tie rule (rounding) | `Round` of `(125, 3)` in `Decimal[-100, 100; 0, 2; nearest-away]`, expected 13 (the mutant gives 12) | step 4 |
| Toward-zero is implemented as floor (rounding) | `Round` of `(-125, 3)` under toward-zero, expected -12 (the mutant gives -13) | step 4 |
| A tie is always rounded up (rounding) | `Divide` of `(1, 0)` by `(8, 0)` under nearest-even, expected 12, and `Divide` of `(-1, 0)` by `(8, 0)` under nearest-away, expected -13 | step 4 |
| The proxy step (working scale above digits) is treated as an exact zero (placement) | `Multiply` of `(1, 2)` and `(1, 2)` under toward-positive in `Decimal[0, 100; 0, 2]`, expected `(1, 2)` (the mutant gives `(0, 2)`) | step 4 |
| A zero result at a working scale above `T` becomes a rounding step (placement) | `Multiply` of `(0, 2)` and `(7, 2)` under `exact`, expected completed `(0, 2)` | step 3 |
| The loss record keeps the proxy instead of the exact intermediate (loss record) | `Multiply` of `(1, 2)` and `(1, 2)` under toward-positive, expected the loss `1/10^4` (twos 4, fives 4) | step 4 |
| The loss record keeps the denominator materialized or unfactored (loss record) | `Divide` of `(1, 0)` by `(8, 0)`, expected twos 3, fives 0, and `Divide` of `(1, 0)` by `(3, 0)`, expected cofactor 3, twos 0, fives 0 | step 4 |
| The loss record does not cancel the common twos and fives (loss record) | `Round` of `(125, 3)`, expected numerator 1, twos 3, fives 0 (the mutant keeps numerator 125, twos 3, fives 3) | step 4 |
| The doubled remainder wraps (nearest comparison) | `Divide` of `(2^126, 0)` by `(2^126 + 1, 0)` under nearest-even in `Decimal[0, 1; 0, 0]`, expected a capacity refusal | step 2 |
| `DivisionByZero` is reported before the `decimal.operands` charge (divisor test) | `Divide` of `(1, 0)` by `(0, 3)` under a zero budget, expected `Incomplete` at `decimal.operands` | step 3 |
| Admission applies its grounds to a zero divisor (admission order) | `Divide` of `(1, 0)` by `(0, 1)` in `Decimal[0, 10^38; 0, 38]`, expected admitted and `Undefined(DivisionByZero)` (the mutant refuses by the numerator exponent) | step 3 |
| `Exact` rounding returns the rounded value (rounding mode dispatch) | `Divide` of `(1, 0)` by `(3, 0)` in `Decimal[0, 100; 0, 2]`, expected `Refused(InexactDecimal)` | step 3 |
| `InexactDecimal` is reported after a `decimal.rounding` charge or with a loss record (rounding charge) | the same input, with a recording `Meter`: expected no `decimal.rounding` charge | step 3 |
| The membership check precedes rounding (membership) | `Round` of `(1004, 1)` into `Decimal[0, 100; 0, 0; nearest-even]`, expected completed `(100, 0)` (the mutant checks 100.4 against the bound first and refuses) | step 3 |
| The out-of-domain check is skipped after rounding (membership) | `Round` of `(1005, 1)` into `Decimal[0, 100; 0, 0; nearest-away]`, expected `DecimalOutOfDomain` (101) | step 3 |
| `decimal.result-retain` is charged before membership (charge order) | `Add` of `(100, 0)` and `(1, 0)` in `Decimal[0, 100; 0, 2]` with a recording `Meter`, expected no `decimal.result-retain` charge | step 3 |
| `decimal.scale-expansion` charges the retained shift only (expansion amount) | `Add` of `(1, 0)` and `(1, 3)`, expected `scale_expansion` 3 | step 1 |
| `sbits` is taken as the bit length of the shifted coefficient (size amount) | `Add` of `(1, 0)` and `(1, 3)` in `Decimal[0, 10000; 0, 3]`, expected `integer_bits` 11 at `decimal.scale-expansion` (the mutant gives 10) | step 1 |
| `sdigits` omits the shift (size amount) | the same input, expected `decimal_digits` 4 (the mutant gives 1) | step 1 |
| The `decimal.arithmetic` maximum omits the `+ 1` for `Add` (arithmetic amount) | `Add` of `(5, 1)` and `(5, 1)`, expected `integer_bits` 4 and `decimal_digits` 2 | step 1 |
| `Multiply` charges the maximum instead of the sum (arithmetic amount) | `Multiply` of `(5, 1)` and `(4, 1)`, expected `integer_bits` 6 (the mutant gives 3) | step 1 |
| `decimal.rounding` carries the `decimal.result-retain` amounts (rounding amount) | `Divide` of `(1, 0)` by `(8, 0)`, expected `decimal.rounding` of 8 and 3 | step 1 |
| A charge point is dropped, reordered or charged twice (charge sequence) | any completed case of step 5, at its exact limit | step 5 |
| The `decimal.rounding` charge is made on an exact result (rounding charge) | `Add` of `(5, 1)` and `(5, 1)` with a recording `Meter` | step 3 |
| A denied charge is followed by a later charge (charge sequence) | `Add` of `(5, 1)` and `(5, 1)` at one less than the `decimal.arithmetic` amount | step 5 |
| The ordering compares raw coefficients (ordering) | `lt` of `(10, 1)` and `(2, 0)`, expected true | step 1 |
| The ordering `le` is implemented as `lt` (ordering) | `le` of `(10, 1)` and `(1, 0)`, expected true | step 1 |
| The ordering `ge` is implemented as `gt` (ordering) | `ge` of `(-1, 0)` and `(-10, 1)`, expected true | step 1 |
| `ordering.arithmetic` charges the left operand's shift only (ordering amount) | `lt` of `(10, 1)` and `(2, 0)`, expected `scale_expansion` 1 (the left shift is 0) and `integer_bits` 6 | step 1 |
| The ordering forms the aligned operand with a wrapping multiply or refuses by capacity (ordering) | `gt` of `(2, 0)` and `(1, 38)`, expected completed true with `scale_expansion` 38 and `integer_bits` 129 | step 1 |
| `bits(10^k)` is clamped or tabled at 38 (ordering amount) | `gt` of `(1, 0)` and `(1, 39)`, expected `integer_bits` 131, and `bits(10^k)` at 2^32 - 1, expected 14267572524 | step 6 |
| A capacity-exceeding sum wraps or saturates (sum admission) | `Add` of `(2^126, 0)` and `(2^126, 0)`, expected a capacity refusal | step 2 |
| A capacity-exceeding product wraps (product admission) | `Multiply` of `(2^64, 0)` and `(2^64, 0)` (`2^128` is 0 modulo `2^128`), expected a capacity refusal | step 2 |
| A shifted operand wraps (shift admission) | `Add` of `(2, 0)` and `(1, 38)`, expected a capacity refusal naming the shifted operand | step 2 |
| The exponent limit is applied only to nonzero coefficients (exponent admission) | `Add` of `(0, 0)` and `(0, 39)`, expected a capacity refusal naming the exponent | step 2 |
| The lift exponent is not checked for a zero result (lift admission) | `Negate` of `(0, 0)` in `Decimal[0, 1; 0, 100]`, expected a capacity refusal naming the exponent | step 2 |
| The negation of `i128::MIN` wraps (negation admission) | `Negate` of `(i128::MIN, 0)`, expected a capacity refusal naming the negation | step 2 |
| The power of ten of the placement is formed when the proxy applies (placement) | `Multiply` of `(1, 4294967295)` and `(1, 4294967295)` under toward-positive in `Decimal[0, 100; 0, 2]`, expected admitted with `(1, 2)` (the mutant forms `10^8589934590`, which does not fit, and refuses) | step 2 |
| The doubled remainder is taken before reducing the pair (nearest comparison) | `Divide` of `(2^126, 0)` by `(2^127 - 2, 0)` under nearest-even in `Decimal[0, 1; 0, 0]`, expected admitted with `(1, 0)` (the mutant refuses by capacity) | step 2 |
| The `log2(10)` constant is one unit low or one unit high (bits of a power of ten) | `the_log2_ten_constant_is_the_floor_of_the_true_value` and `the_log2_ten_approximations_share_a_floor_over_every_u32` fail; the shift-by-shift test alone does not | step 6 |
| The exact-rounding refusal is skipped (rounding mode dispatch) | `the_reference_is_bound_to_the_owner_route_over_the_grid` and `..._over_random_operands` fail at the native-versus-owner comparison of `bind`, on the first `Exact` case that rounds | step 9 |
| No `Exact` rounding mode is drawn (input) | the same two tests fail at `assert_every_class_reached`, naming `exact refusal` as never reached | step 9 |
| The gcd sequence is shorter than 256 steps, for example 128 (gcd step sequence) | `Divide` of `(2^126 - 1, 0)` by `(2^126, 0)` in `Decimal[0, 1; 0, 0; nearest-even]`, expected loss numerator `2^126 - 1`, cofactor 1, twos 126, fives 0 (this pair needs 252 steps of Behavior 16; after 128 steps the state is `(2^124 - 1, 1)` with `g` 1 and the mutant reads the gcd `2^124`, so its reduced pair and loss record differ) | step 4 |
| The gcd step halves an even operand while the other is zero (gcd step) | `Divide` of `(0, 0)` by `(12, 0)` in `Decimal[0, 100; 0, 2; Exact]`, expected completed `(0, 2)` with no `decimal.rounding` charge (the mutant ends with gcd 3, reduces to `0/4`, sees a non-integer quotient and returns `Refused(InexactDecimal)`) | step 1 |
| The quotient is not reduced by the gcd (gcd) | `Divide` of `(3, 0)` by `(9, 0)` in `Decimal[-100, 100; 0, 2; nearest-even]`, expected loss numerator 1, cofactor 3 (the mutant gives numerator 3, cofactor 9) | step 4 |
| The canonical sign of `i128::MIN` wraps in the quotient (quotient admission) | `Divide` of `(i128::MIN, 0)` by `(-3, 0)` and of `(1, 0)` by `(i128::MIN, 0)`, expected capacity refusals naming the canonical quotient | step 2 |
| A zero placed at `T` is refused by the exponent ground (lift admission) | `Multiply` of `(0, 2)` and `(7, 2)` in `Decimal[0, 100; 0, 2]`, expected admitted and completed `(0, 2)` | step 2 |
| The power of ten of the placement is not checked for fit (placement admission) | `Round` of `(i128::MAX, 39)` under toward-zero, expected a capacity refusal naming the power of ten | step 2 |
| The retained coefficient after the lift wraps (lift admission) | `Negate` of `(-2, 0)` in `Decimal[i128::MIN, i128::MAX; 38, 38]`, expected a capacity refusal naming the retained coefficient | step 2 |
| Admission refuses an operand pair that fits (admission bounds) | `Add` of `(2^125, 0)` and `(2^125, 0)`, `Multiply` of `(2^63, 0)` and `(2^63, 0)`, `Add` of `(1, 0)` and `(1, 38)` and `Round` of `(i128::MAX, 38)`, each expected admitted | step 2 |
| Admission charges the Meter (admission) | any refused case of step 2: the `Meter` afterwards is unchanged | step 2 |
| A loop, fixed or data-bounded, forms a power of ten (power of ten) | the source inspection (a fixed-bound loop verifies under Kani, so only inspection catches it) | step 7 |
| A `cfg(kani)`-selected body (any proof-path body) | the source inspection | step 7 |
| The result holds an `Integer` or a `Vec` (result type) | the `Copy` check does not compile | step 7 |

## Expected Results

Every case yields the independently written outcome, retained pair, loss record and charge. The native path and the owner route agree on the whole set except the listed capacity refusals, and on every charge, counter and incomplete record at sufficient, exact and one-less limits. The proof-path bodies contain no loop, iterator adapter, `cfg(kani)` token or heap type. Every mutant fails as named. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass. The measurement of step 11 is a recorded number with its provenance once CG's decimal row consumes the operations, and is never a prediction.

## Status

Run on the host: steps 1 to 8, through `tests/ir1508_native_decimal_ops.rs` (literal cases, the differential against `evaluate_decimal` and `order_numbers` over 19 types and 38 operands with an independently modelled capacity set, the limit sweeps and the source inspection) and `src/native_decimal/operations_tests.rs` (the reference binding of Behavior 38, `bits(10^k)`, the gcd and the wide product). The boundary set holds each mutant's distinguishing input, asserted against its literal in the step the table names, but the TC-951 mutants were not built and run as a set (step 10). The PR #149 reviewer ran eight controls on the unmutated branch, all killed by an assertion failure: M1 the gcd takes 128 steps, M2 the ordering forms the aligned operand with a wrapping multiply, M3 the doubled remainder wraps, M4 the numerator exponent ground runs before the zero-divisor exit, M5 `decimal.rounding` is charged on an exact result, M6 the `log2(10)` approximation has 64 fractional bits, M7 a Divide overflow is named `ShiftedOperand`, M8 the unary lift-exponent ground runs after the negation ground. The exhaustive all-`u32` host test of step 6 and the host tests of step 9 have run; no Kani harness remains, and the full-width proof of AC-15 waits on a separate Verus track. Step 11 waits for IR-259. No proof result or measurement exists.
