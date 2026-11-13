---
id: TC-953
title: "The native float carrier matches the owner's float path and verifies under Kani"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-405
    type: verifies
---
# TC-953: The native float carrier matches the owner's float path and verifies under Kani

## Description

Verify FR-405-AC-1 through FR-405-AC-15 by running the native float operations beside the shipped `evaluate_ieee` and `compare_ieee`, by inspecting the proof-path source, and by recording the Kani probe, verdicts and peaks. Each mutant below is a buildable variant of the named site in a scratch copy and names the input that fails it and the step at which it fails; a mutant that no input distinguishes is not admitted. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass.

## Test Procedure

The boundary set is fixed before the implementation, from independent literals and not from the native body. It holds, for `Binary32` and `Binary64`, in both signs: zero, the smallest subnormal, the largest subnormal, the smallest normal, the largest finite, one, the neighbours of one, infinity, a quiet NaN, a signalling NaN and a NaN with a nonzero payload; operand pairs that are exact ties and one unit off a tie for each rounding mode; pairs that overflow, underflow and cancel exactly; and the operations Negate, Add, Subtract, Multiply, Divide, SquareRoot, FusedMultiplyAdd and the comparisons NumericEqual, TotalOrder, BitIdentical in each of the six rounding spellings (Exact and the five modes). It covers these inputs and not every operand pair.

1. For AC-1 and AC-2, build the carrier from every boundary pattern, check `Copy`, structural equality, the `OutsideCarrierDomain` refusal of an out-of-width pattern with no `Meter` in the signature, and the round trip through `IeeeValue`.
2. For AC-3, call every supported operation and a cross-width call (asserting `ill_typed` and an unchanged `Meter`).
3. For AC-4 and AC-5, evaluate every boundary case natively and by the owner and compare bits, flags, provenance and outcome kind, including the `Exact` refusal's would-be flags with no bits, and compare against independently written literals for the tie, overflow-result, exact-zero-sign and NaN cases.
4. For AC-6, run each case at a sufficient, an exact and a one-less limit natively and by the owner on fresh Meters and on one cumulative Meter, and compare every charge point, limit kind, amount, order, the denied charge and the incomplete record.
5. For AC-7 and AC-9, inspect the proof-path source for `num_bigint`, `Integer`, `Rational`, `Vec`, `cfg(kani)`, iterator adapters, loops (each loop has a stated constant bound and no operand-dependent `break`, `return` or `?`) and calls from a native body to an owner function; inspect each harness for the functions its expected side shares with the native body (only the decode may be shared) and for its docstring of covered outputs.
6. For AC-8, run a counting global allocator around the native operations over the boundary set, asserting zero allocations, and call each operation and comparison from a host test.
7. For AC-10, run each mutant through the differential and assert it fails at the step the table names.
8. For AC-11, AC-12 and AC-13, run each Kani harness of each operation, comparison and width under the authorized runner with a 10-minute wall-clock limit and record verdict, time, peak memory with its provenance and tool version. The Binary64 add and multiply harnesses are the probe: they are the AC-11 harnesses, not separate ones, and each combination is judged alone. For a non-VERIFIED outcome, record the row in the Kani tractability table. Confirm each VERIFIED harness's cover is reached and its harness mutant fails, and compare each peak with 16 GiB.
9. For AC-14, search the kernel source for a refusal variant, refusal path or `Meter` behaviour defined for the table-row state; none may exist, and the seam referenced is IR-1526's.
10. For AC-15, inspect the Status for a per-operation, per-width verdict or table row, including SquareRoot and FusedMultiplyAdd.

A Kani harness checks, per operation: it draws the operands (two, one for Negate and SquareRoot, three for FusedMultiplyAdd) as symbolic bit patterns of the width, a symbolic rounding spelling and a symbolic meter limit, evaluates the native operation, and asserts equality of result bits (NaN payload included), flags, outcome kind and the charge sequence with the independent reference. For a comparison it asserts the boolean and the charge sequence.

### Mutants

| Mutant (site) | Distinguishing input | Fails at |
| --- | --- | --- |
| The tie rule of nearest-even is nearest-away | a tie operand pair whose even neighbour is the lower value | step 3 |
| Nearest-away rounds ties to even | a tie whose larger-magnitude neighbour is odd | step 3 |
| Toward-zero increments the magnitude | an inexact positive result and an inexact negative result under toward-zero | step 3 |
| Toward-positive and toward-negative are swapped | an inexact positive result and an inexact negative result | step 3 |
| The sticky bit is dropped in alignment | an add of one and a tiny operand, one unit off a tie | step 3 |
| The subnormal boundary is off by one | the largest subnormal plus the smallest subnormal | step 3 |
| The overflow result ignores the mode | the largest finite plus itself under toward-zero, toward-positive, toward-negative and each nearest mode, each in both signs (infinity or largest finite) | step 3 |
| The `overflow` flag is not raised, or raised on the largest finite | the largest finite plus itself, and the largest finite plus zero | step 3 |
| An exact zero sum takes the wrong sign | positive one plus negative one in toward-negative, and in the other modes | step 3 |
| The NaN payload is cleared or not quieted | a signalling NaN and a payload NaN as operands, for each operation | step 3 |
| `invalid` is not raised for infinity minus infinity | infinity minus infinity | step 3 |
| `divide_by_zero` is raised for zero over zero | zero over zero, and one over zero | step 3 |
| `underflow` is raised on an exact subnormal result | an exact subnormal sum | step 3 |
| `inexact` is not raised when only the sticky bit is set | a divide whose remainder is nonzero | step 3 |
| `Exact` returns the rounded value | one divided by three under `Exact` | step 3 |
| `Exact` refusal carries bits | the same input | step 3 |
| Square root of a negative nonzero operand does not raise `invalid`, or negative zero loses its sign | the square root of minus one, and of minus zero | step 3 |
| Fused multiply-add rounds the product before the addition | operands whose product is inexact and cancels in the sum | step 3 |
| NumericEqual treats zeros of opposite sign as unequal, or a NaN as equal | minus zero against plus zero, and a NaN against itself | step 3 |
| TotalOrder ignores the sign or the payload of a NaN | a negative NaN against a positive NaN, and two NaN payloads | step 3 |
| BitIdentical compares numeric value | two NaNs with different payloads, and the two zeros | step 3 |
| Negate raises a flag or quiets a NaN | the negation of a signalling NaN | step 3 |
| The provenance of a result is not the owner's | any completed case, the provenance compared | step 3 |
| The charge of the operand points is dropped, reordered or charged twice | any completed case at its exact limit | step 4 |
| The result charge is dropped or charged at the wrong amount | any completed case at its exact limit and one less | step 4 |
| The first charge is made after the width check, or the denied charge is followed by a later charge | a cross-width add, and a completed case at one less than its first charge | step 4 |
| An ill-typed call charges the Meter | a cross-width add | step 2 |
| A loop exits on an operand-dependent condition | the source inspection | step 5 |
| `cfg(kani)` selects a body | the source inspection | step 5 |
| A native body calls the owner | the source inspection | step 5 |
| A Kani harness checks the native body against itself | the expected-side inspection and a harness mutant that the harness must fail | step 8 |
| A refusal variant for the table-row state is defined in the kernel | the source search | step 9 |

## Expected Results

Every boundary case yields the owner's bits, flags, provenance and charges, with independent literals for the cases named. Every mutant fails as named. Each combination of operation and width ends as an executed VERIFIED verdict or a measured table row, judged alone, with its peak recorded and at most 16 GiB; a combination with neither is inconclusive and no proof is claimed.

## Status

Not run. No implementation, harness, table row or measurement exists; this case is specified under IR-1446 and is executable once the native operations are delivered.
