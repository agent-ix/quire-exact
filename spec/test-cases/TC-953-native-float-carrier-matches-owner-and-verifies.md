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
5. For AC-7 and AC-9, code review inspects the proof-path source for `num_bigint`, `Integer`, `Rational`, `Vec`, `cfg(kani)`, iterator adapters, loops (each loop has a stated constant bound and no operand-dependent `break`, `return` or `?`) and calls from a native body to an owner function, and inspects each harness for the functions its expected side shares with the native body (none may be shared) and for its docstring of covered outputs. There is no automated step: a source-text scan is not a test.
6. For AC-8, run a counting global allocator around the native operations over the boundary set, asserting zero allocations, and call each operation and comparison from a host test.
7. For AC-10, run each mutant through the differential and assert it fails at the step the table names.
8. For AC-11, AC-12 and AC-13, run each Kani harness of each operation, comparison and width under the authorized runner with a 10-minute wall-clock limit and record verdict, time, peak memory with its provenance and tool version. The Binary64 add and multiply harnesses are the probe: they are the AC-11 harnesses, not separate ones, and each combination is judged alone. For a non-VERIFIED outcome, record the row in the evidence record below and in the Kani tractability table that IR-1526's seam loads. Confirm each VERIFIED harness's cover is reached and its harness mutant fails, and compare each peak with 16 GiB.
9. For AC-14, code review searches the kernel source for a refusal variant, refusal path or `Meter` behaviour defined for the table-row state; none may exist, and the seam referenced is IR-1526's. There is no automated step.
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
| `ieee.exact-intermediate` is not charged on a finite plan, or is charged on a classification-decided input | a finite add at its exact limit, and Add with a NaN operand, Add of infinity and minus infinity, and Divide by zero, each with a recording `Meter` expecting no `ieee.exact-intermediate` charge | step 4 |
| `ieee.round` is not charged on a finite plan, or is charged on a classification-decided input | the same inputs, expecting `ieee.round` only on the finite plan | step 4 |
| The operand-count amount of the operand charge is wrong for SquareRoot (1) or FusedMultiplyAdd (3) | SquareRoot of four and FusedMultiplyAdd of one, one and one, expecting operand amounts for 1 and 3 operands (CG FR-053-AC-10 b, c) | step 4 |
| The charge of the operand points is dropped, reordered or charged twice | any completed case at its exact limit | step 4 |
| The result charge is dropped or charged at the wrong amount | any completed case at its exact limit and one less | step 4 |
| The first charge is made after the width check, or the denied charge is followed by a later charge | a cross-width add, and a completed case at one less than its first charge | step 4 |
| An ill-typed call charges the Meter | a cross-width add | step 2 |

## Evidence record

The versioned record of every executed harness. The proofs-gen tractability table is the source IR-1526 loads; this section is the copy that can be inspected from this repository, and no criterion depends on the table's path. Each row gives operation, width, outcome (VERIFIED, or timeout, memory exhaustion or unreliable; a FAILED verdict is a defect, not a row), wall time, peak memory with provenance, Kani version and harness id.

| Operation | Width | Outcome | Time | Peak memory | Kani version | Harness id |
| --- | --- | --- | --- | --- | --- | --- |
| Negate | Binary32 | VERIFIED | 1.8 s | 597381120 bytes | 0.68.0 | native_float_kani_proofs::float32_negate |
| Add | Binary32 | timeout | 600 s (limit; killed, no verdict) | 22771355648 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_add |
| Subtract | Binary32 | timeout | 600 s (limit; killed, no verdict) | 23011209216 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_subtract |
| Multiply | Binary32 | timeout | 600 s (limit; killed, no verdict) | 22814351360 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_multiply |
| Divide | Binary32 | timeout | 600 s (limit; killed, no verdict) | 22905405440 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_divide |
| SquareRoot | Binary32 | timeout | 600 s (limit; killed, no verdict) | 23219585024 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_square_root |
| FusedMultiplyAdd | Binary32 | timeout | 600 s (limit; killed, no verdict) | 22764011520 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_fused_multiply_add |
| NumericEqual | Binary32 | VERIFIED | 1.8 s | 653529088 bytes | 0.68.0 | native_float_kani_proofs::float32_numeric_equal |
| TotalOrder | Binary32 | VERIFIED | 1.6 s | 566456320 bytes | 0.68.0 | native_float_kani_proofs::float32_total_order |
| BitIdentical | Binary32 | VERIFIED | 1.5 s | 567275520 bytes | 0.68.0 | native_float_kani_proofs::float32_bit_identical |
| Negate | Binary64 | VERIFIED | 1.8 s | 597979136 bytes | 0.68.0 | native_float_kani_proofs::float64_negate |
| Add | Binary64 | timeout | 600 s (limit; killed, no verdict) | 23179018240 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_add |
| Subtract | Binary64 | timeout | 600 s (limit; killed, no verdict) | 23192272896 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_subtract |
| Multiply | Binary64 | timeout | 600 s (limit; killed, no verdict) | 22704226304 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_multiply |
| Divide | Binary64 | timeout | 600 s (limit; killed, no verdict) | 23269302272 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_divide |
| SquareRoot | Binary64 | timeout | 600 s (limit; killed, no verdict) | 22983905280 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_square_root |
| FusedMultiplyAdd | Binary64 | timeout | 600 s (limit; killed, no verdict) | 22954770432 bytes (above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_fused_multiply_add |
| NumericEqual | Binary64 | VERIFIED | 1.7 s | 566562816 bytes | 0.68.0 | native_float_kani_proofs::float64_numeric_equal |
| TotalOrder | Binary64 | VERIFIED | 1.6 s | 566165504 bytes | 0.68.0 | native_float_kani_proofs::float64_total_order |
| BitIdentical | Binary64 | VERIFIED | 1.5 s | 565882880 bytes | 0.68.0 | native_float_kani_proofs::float64_bit_identical |

Runner and provenance. The harnesses and native bodies measured are those of commit 6f0d666 (`src/native_float.rs` and `src/native_float/kani_proofs.rs` are unchanged after it). Each row is an executed run of `cargo kani --harness native_float_kani_proofs::<id> --exact --unwind 57 --jobs 1 --solver cadical` (Kani 0.68.0, CBMC 6.11.0), one job at a time under `flock /tmp/cargo-slot.lock` and `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, with a 600 s `timeout`. The unwind bound is 57 because the square-root digit loop has the constant bound 56 (`SQRT_STEPS`); no other loop exceeds 3 iterations. Peak memory is the scope's cgroup `memory.peak`, read inside the scope after the run; the scope held the whole `cargo kani` invocation, so it includes the build and is an upper bound on the solver's peak. A VERIFIED time is Kani's reported verification time. A timeout row's time is the 600 s limit, at which `timeout` killed the run; its harness reached no verdict, and its peak (22.7 to 23.3 GB, just under the 24 GiB scope limit) is above the 16 GiB ceiling of CG FR-028, so by FR-405-AC-13 it is a non-VERIFIED row and no proof is claimed for it. Every VERIFIED row satisfied all of its cover properties (2 of 2 for each comparison, 1 of 1 for each Negate) and its peak is at most 0.66 GB.

Expected side of the harnesses. Each arithmetic harness checks the native operation against the host language's own `f32`/`f64` operation (`+`, `-`, `*`, `/`, `sqrt`, `mul_add`), which shares no function with the native body and not even the bit-pattern decode. The docstring of `src/native_float/kani_proofs.rs` lists the outputs a harness covers (result bits under `NearestEven` and `Exact`, NaN results under every spelling, the `invalid` flag of NaN-producing operations, that only `Exact` refuses, provenance, the result-unit charge) and the outputs left to the host differential (directed and nearest-away bits, the `overflow`, `underflow`, `inexact` and `divide_by_zero` flags, the `Exact` refusal's would-be flags, charge amounts and meter limits). The comparison harnesses assert the boolean and the result-unit charge against the host `==`, bit equality and a signed-integer fold of the sign-magnitude order. Negate asserts bitwise sign negation with no flag.

Harness mutants (FR-405-AC-12). Each VERIFIED harness fails a buildable mutant of the native body, run under the same runner: NumericEqual with zeros of opposite sign unequal fails `float32_numeric_equal` and `float64_numeric_equal`; BitIdentical comparing numeric value fails both `bit_identical` harnesses; TotalOrder ignoring the sign fails both `total_order` harnesses; Negate quieting its operand fails both `negate` harnesses. All eight reported `VERIFICATION:- FAILED`.

Host mutants (FR-405-AC-10). `scripts/ir1446_native_float_mutants.py` builds 49 mutants in scratch copies (one per row of the table above that has a buildable form, one per charge point of `ieee.rs` including `ieee.exact-intermediate` and `ieee.round` each made on a classification-decided input, and the inspection mutants) and requires the named step's test to fail; all 49 were killed and none survived.

### Vacuity and harness mutants

| Harness id | Covers reached | Mutant | Verdict |
| --- | --- | --- | --- |
| `native_float_kani_proofs::float32_negate` | 1 of 1 | negate-quiets | FAILED |
| `native_float_kani_proofs::float64_negate` | 1 of 1 | negate-quiets | FAILED |
| `native_float_kani_proofs::float32_numeric_equal` | 2 of 2 | numeric-equal-zeros | FAILED |
| `native_float_kani_proofs::float64_numeric_equal` | 2 of 2 | numeric-equal-zeros | FAILED |
| `native_float_kani_proofs::float32_total_order` | 2 of 2 | total-order-sign | FAILED |
| `native_float_kani_proofs::float64_total_order` | 2 of 2 | total-order-sign | FAILED |
| `native_float_kani_proofs::float32_bit_identical` | 2 of 2 | bit-identical-numeric | FAILED |
| `native_float_kani_proofs::float64_bit_identical` | 2 of 2 | bit-identical-numeric | FAILED |
| `native_float_kani_proofs::float32_add_nearest_even` | 3 of 3 | tie-even-is-away | FAILED |
| `native_float_kani_proofs::float64_add_nearest_even` | 3 of 3 | tie-even-is-away | FAILED |
| `native_float_kani_proofs::float32_subtract_nearest_even` | 3 of 3 | tie-even-is-away | FAILED |
| `native_float_kani_proofs::float64_subtract_nearest_even` | 3 of 3 | tie-even-is-away | FAILED |
| `native_float_kani_proofs::float32_multiply_nearest_even` | 3 of 3 | tie-even-is-away | FAILED |

## Expected Results

Every boundary case yields the owner's bits, flags, provenance and charges, with independent literals for the cases named. Every mutant fails as named. Each combination of operation and width ends as an executed VERIFIED verdict or a measured table row, judged alone, with its peak recorded and at most 16 GiB; a combination with neither is inconclusive and no proof is claimed.

## Status

Run under IR-1446. The host tests (`tests/ir1446_native_float.rs`, steps 1 to 7 and 9) and the 49 mutants pass; the Kani harnesses of step 8 ran once for every combination of operation and width. Negate, NumericEqual, TotalOrder and BitIdentical are VERIFIED at both widths, with the vacuity covers reached and the harness mutants failing. Add, Subtract, Multiply, Divide, SquareRoot and FusedMultiplyAdd are measured table rows (timeout, peak above 16 GiB) at both widths, including the Binary64 add and multiply probe; no proof is claimed for them. The table rows are not yet loaded into IR-1526's tractability table, which this repository does not own; the evidence record above is the versioned copy.
