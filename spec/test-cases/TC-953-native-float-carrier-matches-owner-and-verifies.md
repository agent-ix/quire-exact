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

Verify FR-405-AC-1 through FR-405-AC-13 by running the native float operations beside the shipped `evaluate_ieee` and `compare_ieee`, by inspecting the proof-path source, and by recording the Kani probe, verdicts and peaks. Each mutant below is a buildable variant of the named site in a scratch copy and names the input that fails it and the step at which it fails; a mutant that no input distinguishes is not admitted. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass.

## Test Procedure

The boundary set is fixed before the implementation, from independent literals and not from the native body. It holds, for `Binary32` (and for `Binary64` if in scope), in both signs: zero, the smallest subnormal, the largest subnormal, the smallest normal, the largest finite, one, the neighbours of one, infinity, a quiet NaN, a signalling NaN and a NaN with a nonzero payload; operand pairs that are exact ties and one unit off a tie for each rounding mode; pairs that overflow, underflow and cancel exactly; and the operations Negate, Add, Subtract, Multiply, Divide and the comparisons NumericEqual, TotalOrder, BitIdentical in each of the six rounding spellings (Exact and the five modes).

1. For AC-1, build the carrier from every boundary pattern, check `Copy`, structural equality, the refusal of an out-of-width pattern and the round trip through `IeeeValue`.
2. For AC-2, call every supported operation, a cross-width call (asserting `ill_typed` and an unchanged `Meter`), and inspect the native surface for any square-root, fused-multiply-add or `requires_bound` entry.
3. For AC-3 and AC-4, evaluate every boundary case natively and by the owner and compare bits, flags, provenance and outcome kind, including the `Exact` refusal's would-be flags with no bits, and compare against independently written literals for the tie, overflow-result, exact-zero-sign and NaN cases.
4. For AC-5, run each case at a sufficient, an exact and a one-less limit natively and by the owner on fresh Meters and on one cumulative Meter, and compare every charge point, limit kind, amount, order, the denied charge and the incomplete record.
5. For AC-6 and AC-7, inspect the proof-path source for `num_bigint`, `Integer`, `Rational`, `Vec`, `cfg(kani)`, loops and iterator adapters, and for a call from a native body to an owner function; run a counting global allocator around the native operations over the boundary set, asserting zero allocations; check that the Kani harness expected side shares no function with the native body.
6. For AC-8, run each mutant through the differential and assert it fails at the named step.
7. For AC-9, run the probe: a Kani harness for `Binary64` addition and for `Binary64` multiplication with a 10-minute wall-clock limit each, and record verdict, time, peak memory and tool version. Decide the width set from the recorded verdicts only.
8. For AC-10 and AC-11, run each in-scope Kani harness under the authorized runner and record verdict, time, peak memory with its provenance and tool version; confirm each vacuity cover is reached and each harness mutant fails; compare each peak with the K-DR memory budget.
9. For AC-12, once CG's float row consumes the carrier, run its composed harness and its host differential and record the same fields.
10. For AC-13, inspect the status for a per-operation, per-width verdict or an explicit absence.

### Mutants

| Mutant (site) | Distinguishing input | Fails at |
| --- | --- | --- |
| The tie rule of nearest-even is nearest-away | a tie operand pair whose even neighbour is the lower value | step 3 |
| Nearest-away rounds ties to even | a tie whose larger-magnitude neighbour is odd | step 3 |
| Toward-positive and toward-negative are swapped | an inexact positive result and an inexact negative result | step 3 |
| The sticky bit is dropped in alignment | an add of one and the smallest subnormal-scale operand, one unit off a tie | step 3 |
| The subnormal boundary is off by one | the largest subnormal plus the smallest subnormal | step 3 |
| The overflow result ignores the mode | the largest finite plus itself under toward-zero | step 3 |
| An exact zero sum takes the wrong sign | positive one plus negative one in toward-negative, and in the other modes | step 3 |
| The NaN payload is cleared or not quieted | a signalling NaN and a payload NaN as operands | step 3 |
| `invalid` is not raised for infinity minus infinity | infinity minus infinity | step 3 |
| `divide_by_zero` is raised for zero over zero | zero over zero, and one over zero | step 3 |
| `underflow` is raised on an exact subnormal result | an exact subnormal sum | step 3 |
| `inexact` is not raised when only the sticky bit is set | a divide whose remainder is nonzero | step 3 |
| `Exact` returns the rounded value | one divided by three under `Exact` | step 3 |
| `Exact` refusal carries bits | the same input | step 3 |
| A charge point is dropped, reordered or charged twice | any completed case at its exact limit | step 4 |
| An ill-typed call charges the Meter | a cross-width add | step 2 |
| A data-dependent loop or `cfg(kani)` body | the source inspection | step 5 |
| A native body calls the owner | the source inspection | step 5 |
| A Kani harness checks the native body against itself | the expected-side inspection and a harness mutant that the harness must fail | step 8 |

## Expected Results

Every boundary case yields the owner's bits, flags, provenance and charges, with independent literals for the cases named. Every mutant fails as named. The width set follows the recorded probe. Each in-scope harness has an executed VERIFIED verdict with a recorded peak within the K-DR budget, or is recorded as inconclusive with no proof claimed.

## Status

Not run. No implementation, probe, harness or measurement exists; this case is specified under IR-1446 and is executable once the native operations are delivered.
