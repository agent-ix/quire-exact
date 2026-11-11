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

Verify FR-403-AC-1 through FR-403-AC-10 by running the native operations beside the owner's `evaluate_decimal` and `order_numbers`, each under a `Meter` with the same limits. Every mutant below is a real buildable variant in a scratch copy and names the input, or the inspection, that fails it. This case is planned; no part of it has run.

## Test Procedure

The operand and type set is fixed before the implementation is written, from independent literals. A carried Decimal is `(coefficient, scale)` built by the total constructor of FR-402 Behavior 3 (or by a binding where the coefficient is an `i64`).

- The cases of FR-403-AC-1 to AC-4 and AC-6, with their types.
- Capacity edges: the sums, products, shifts and aligned magnitudes of FR-403-AC-1 and AC-6; `Divide` of `(1, 0)` by `(1, 38)` in `Decimal[0, 10^38; 0, 38]` (numerator `10^38 x 10^38` does not fit; capacity refusal naming the numerator); `Divide` of `(1, 0)` by `(1, 0)` in `Decimal[0, 100; 0, 2]` (admitted); in `Decimal[0, 1; 0, 0]` under nearest-even, `Divide` of `(2^126, 0)` by `(2^126 + 1, 0)` (quotient 0, remainder `2^126`, whose double `2^127` does not fit: capacity refusal naming the doubled remainder) and `Divide` of `(2^126 - 1, 0)` by `(2^126, 0)` (doubled remainder `2^127 - 2`, admitted, completes with `(1, 0)`).
- Meter limits: for each case that completes, the limits that are sufficient, exactly the charge of each point reached, and one less than it, for `integer_bits`, `decimal_digits`, `scale_expansion`, `value_occurrences`, `work_units` and `result_units`.

1. For AC-2 to AC-4 and AC-6, evaluate every case natively and compare the typed outcome, the retained pair and the loss record with independently written literals.
2. For AC-1 and AC-8, evaluate every case a second time by the owner route and assert that the native read-out `DecimalResult` equals the owner's under `Eq`, that the typed outcomes are equal, and that the complete list of disagreements is the set of native capacity refusals, each checked against its named quantity. For each capacity refusal, run the owner route and record what it returns without asserting equality. Assert that a `Meter` presented to a refused admission is unchanged.
3. For AC-5, run each case at each limit of the Meter list natively and by the owner route on fresh Meters and on one cumulative Meter, and compare every charge point reached, its limit kind and amount, the order, the denied next charge, every counter and the incomplete record.
4. For AC-7, inspect the source of the admission, the evaluation and the ordering for a `cfg(kani)` token, for loops, iterator adapters and recursion, and for `Integer`, `Decimal`, `Rational`, `Vec`, `Box` and `alloc`; check `Copy` of the result and the loss record; and run the capacity and completed cases inside a counting global allocator window around admission, asserting zero allocations.
5. For AC-9, run the seven proofs `native_decimal_add_refines_owner`, `native_decimal_subtract_refines_owner`, `native_decimal_multiply_refines_owner`, `native_decimal_divide_refines_owner`, `native_decimal_negate_refines_owner`, `native_decimal_round_refines_owner` and `native_decimal_ordering_refines_owner` with no unwind attribute and their covers under the authorized runner, and run the host test that binds each independent reference to the owner route over the boundary set. A proof is credited only on `VERIFIED` with the covers satisfied. The repository's standing instruction forbids running the Kani runners until the feature work is done; the step stays unexecuted until the implementation ticket's lead authorizes the runner.
6. Run each mutant: the input mutants through the checks of steps 1 to 3, the inspection mutants through step 4.
7. For AC-10, once CG's decimal row (IR-259) consumes the delivered operations, run it under the authorized runner and record the peak memory and its provenance, time, tool version and terminal outcome under IR-1444. Until then this step is not executable and AC-10 stays unexecuted; no fit is claimed.

### Mutants

| Mutant | Fails at |
| --- | --- |
| `Add` is computed as `Subtract` | `Add` of `(5, 1)` and `(5, 1)`, expected `(100, 2)` |
| `Subtract` has its operands swapped | `Subtract` of `(5, 1)` and `(1, 1)`, expected `(40, 2)` (the mutant gives `-40`, outside the domain) |
| The result is retained normalized | `Add` of `(5, 1)` and `(5, 1)`, expected the retained pair `(100, 2)` |
| The result is retained at the larger operand scale instead of `T` | `Negate` of `(0, 0)`, expected `(0, 2)` |
| The lift to `T` is skipped | `Round` of `(5, 1)`, expected `(50, 2)` |
| The rounding mode `nearest-even` is used for `nearest-away` | `Round` of `(125, 3)` under nearest-away in `Decimal[-100, 100; 0, 2]`, expected `13` (the mutant gives `12`) |
| Toward-zero is implemented as floor | `Round` of `(-125, 3)` under toward-zero, expected `-12` (the mutant gives `-13`) |
| A tie is always rounded up | `Divide` of `(1, 0)` by `(8, 0)` under nearest-even, expected `12`, and `Divide` of `(-1, 0)` by `(8, 0)` under nearest-away, expected `-13` |
| The doubled remainder wraps | `Divide` of `(2^126, 0)` by `(2^126 + 1, 0)` under nearest-even in `Decimal[0, 1; 0, 0]`, expected a capacity refusal |
| `DivisionByZero` is reported before the `decimal.operands` charge | `Divide` by `(0, 3)` under a zero budget, expected `Incomplete` at `decimal.operands` |
| `Exact` rounding returns the rounded value | `Divide` of `(1, 0)` by `(3, 0)` in `Decimal[0, 100; 0, 2]`, expected `Refused(InexactDecimal)` |
| `InexactDecimal` is reported after a `decimal.rounding` charge or with a loss record | the same input, with a Meter that records charges: expected no `decimal.rounding` charge |
| The membership check precedes rounding | `Round` of `(1004, 1)` into `Decimal[0, 100; 0, 0]` under nearest-even, expected completed `(100, 0)` (the mutant checks 100.4 against the bound before rounding and refuses) |
| The out-of-domain check is skipped after rounding | `Round` of `(1005, 1)` into `Decimal[0, 100; 0, 0]` under nearest-away, expected `DecimalOutOfDomain` (101) |
| `decimal.scale-expansion` is charged with the retained shift only | `Add` of `(1, 0)` and `(1, 3)`, expected a shift of 3 |
| A charge point is dropped, reordered or charged twice | any completed case of step 3, at its exact limit |
| The `decimal.rounding` charge is made on an exact result | `Add` of `(5, 1)` and `(5, 1)` with a Meter that records charges |
| A denied charge is followed by a later charge | `Add` of `(5, 1)` and `(5, 1)` at one less than the `decimal.arithmetic` amount |
| The ordering compares raw coefficients | `lt` of `(10, 1)` and `(2, 0)`, expected true |
| The ordering `le` is implemented as `lt` | `le` of `(10, 1)` and `(1, 0)`, expected true |
| The ordering `ge` is implemented as `gt` | `ge` of `(-1, 0)` and `(-10, 1)`, expected true |
| An ordering aligns by multiplying with a wrapping multiply | `lt` of `(1, 0)` and `(1, 39)`, expected a capacity refusal |
| A capacity-exceeding sum wraps or saturates | `Add` of `(2^126, 0)` and `(2^126, 0)`, expected a capacity refusal |
| A capacity-exceeding product wraps | `Multiply` of `(2^64, 0)` and `(2^64, 0)` (`2^128` is 0 modulo `2^128`), expected a capacity refusal |
| A shifted operand wraps | `Add` of `(1, 0)` and `(1, 39)`, expected a capacity refusal |
| Admission refuses an operand pair that fits | `Add` of `(2^125, 0)` and `(2^125, 0)`, `Multiply` of `(2^63, 0)` and `(2^63, 0)` and `Add` of `(1, 0)` and `(1, 38)`, each expected admitted |
| Admission charges the Meter | any case of step 2: the Meter after a refused admission is unchanged |
| A loop, fixed or data-bounded, forms a power of ten | the source inspection of step 4 (a fixed-bound loop verifies under Kani, so only inspection catches it) |
| A `cfg(kani)`-selected body | the source inspection of step 4 |
| The result holds an `Integer` or a `Vec` | the `Copy` check does not compile |

## Expected Results

Every case yields the independently written outcome. The native path and the owner route agree on the whole set except the listed capacity refusals, and on every charge, counter and incomplete record at sufficient, exact and one-less limits. The proof-path bodies contain no loop, iterator adapter, `cfg(kani)` token or heap type. The seven proofs verify with no unwind bound and their covers. Every mutant fails as named. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass. The measurement of step 7 is a recorded number with its provenance once CG's decimal row consumes the operations, and is never a prediction.

## Status

Planned. No implementation, test, proof or measurement has run.
