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
8. For AC-11, AC-12 and AC-13, run each Kani harness of each operation, comparison and width under the authorized runner with a 10-minute wall-clock limit and record verdict, time, peak memory with its provenance and tool version. An arithmetic combination has three harnesses that together cover the six rounding spellings (`NearestEven`; `Exact`; the four directed and nearest-away spellings drawn symbolically), and each harness is its own row; a combination is VERIFIED only when all three are. The Binary64 add and multiply harnesses are the probe: they are the AC-11 harnesses, not separate ones, and each combination is judged alone. For a non-VERIFIED outcome, record the row in the evidence record below and in the Kani tractability table that IR-1526's seam loads. Confirm each VERIFIED harness's cover is reached and its harness mutant fails, and compare each peak with 16 GiB.
9. For AC-14, code review searches the kernel source for a refusal variant, refusal path or `Meter` behaviour defined for the table-row state; none may exist, and the seam referenced is IR-1526's. There is no automated step.
10. For AC-15, inspect the Status for a per-operation, per-width verdict or table row, including SquareRoot and FusedMultiplyAdd.

A Kani harness is a property harness (FR-405 Behavior 19, second sentence): per operation it draws the operands (two, one for Negate and SquareRoot, three for FusedMultiplyAdd) as fully symbolic bit patterns of the width, runs the native operation with a meter that never denies, and asserts the outputs listed in the harness docstring of `src/native_float/kani_proofs.rs` against an expected side that shares no function with the native body: result bits under `NearestEven` and a completed `Exact`, the NaN result (sign, quietness, payload) and the `invalid` flag of NaN-producing operations under every spelling, that only `Exact` refuses, provenance and the result-unit charge. The outputs it does not cover (result bits under the directed and nearest-away spellings, the `overflow`, `underflow`, `inexact` and `divide_by_zero` flags, the `Exact` refusal's would-be flags, charge amounts and meter limits) are left to the host differential. For a comparison it asserts the boolean and the result-unit charge.

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

The versioned record of every executed harness. The proofs-gen tractability table is the source IR-1526 loads; this section is the copy that can be inspected from this repository, and no criterion depends on the table's path. Each row gives operation, width, rounding spelling group, outcome (VERIFIED, or timeout, memory exhaustion or unreliable; a FAILED verdict is a defect, not a row), time, peak memory with provenance, Kani version and harness id. A VERIFIED time is Kani's reported verification time followed by the wall time of the whole `cargo kani` invocation; a timeout row's time is the 600 s limit.

| Operation | Width | Spelling | Outcome | Time | Peak memory | Kani version | Harness id |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Negate | Binary32 | n/a | VERIFIED | 1.7 s (wall 6.4 s) | 891949056 bytes (0.83 GiB) | 0.68.0 | native_float_kani_proofs::float32_negate |
| Add | Binary32 | NearestEven | VERIFIED | 49.4 s (wall 52.7 s) | 956686336 bytes (0.89 GiB) | 0.68.0 | native_float_kani_proofs::float32_add_nearest_even |
| Add | Binary32 | Exact | timeout | 600 s (limit; killed, no verdict) | 14349295616 bytes (13.36 GiB) | 0.68.0 | native_float_kani_proofs::float32_add_exact |
| Add | Binary32 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 15285170176 bytes (14.24 GiB) | 0.68.0 | native_float_kani_proofs::float32_add_directed |
| Subtract | Binary32 | NearestEven | VERIFIED | 51.6 s (wall 54.9 s) | 951373824 bytes (0.89 GiB) | 0.68.0 | native_float_kani_proofs::float32_subtract_nearest_even |
| Subtract | Binary32 | Exact | timeout | 600 s (limit; killed, no verdict) | 15525650432 bytes (14.46 GiB) | 0.68.0 | native_float_kani_proofs::float32_subtract_exact |
| Subtract | Binary32 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 15553826816 bytes (14.49 GiB) | 0.68.0 | native_float_kani_proofs::float32_subtract_directed |
| Multiply | Binary32 | NearestEven | VERIFIED | 18.5 s (wall 21.8 s) | 883322880 bytes (0.82 GiB) | 0.68.0 | native_float_kani_proofs::float32_multiply_nearest_even |
| Multiply | Binary32 | Exact | timeout | 600 s (limit; killed, no verdict) | 15546863616 bytes (14.48 GiB) | 0.68.0 | native_float_kani_proofs::float32_multiply_exact |
| Multiply | Binary32 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 15528415232 bytes (14.46 GiB) | 0.68.0 | native_float_kani_proofs::float32_multiply_directed |
| Divide | Binary32 | NearestEven | timeout | 600 s (limit; killed, no verdict) | 671404032 bytes (0.63 GiB) | 0.68.0 | native_float_kani_proofs::float32_divide_nearest_even |
| Divide | Binary32 | Exact | timeout | 600 s (limit; killed, no verdict) | 15211565056 bytes (14.17 GiB) | 0.68.0 | native_float_kani_proofs::float32_divide_exact |
| Divide | Binary32 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 15283220480 bytes (14.23 GiB) | 0.68.0 | native_float_kani_proofs::float32_divide_directed |
| SquareRoot | Binary32 | NearestEven | timeout | 600 s (limit; killed, no verdict) | 716824576 bytes (0.67 GiB) | 0.68.0 | native_float_kani_proofs::float32_square_root_nearest_even |
| SquareRoot | Binary32 | Exact | timeout | 600 s (limit; killed, no verdict) | 21902503936 bytes (20.40 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_square_root_exact |
| SquareRoot | Binary32 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 21777010688 bytes (20.28 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_square_root_directed |
| FusedMultiplyAdd | Binary32 | NearestEven | timeout | 600 s (limit; killed, no verdict) | 22744002560 bytes (21.18 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_fused_multiply_add_nearest_even |
| FusedMultiplyAdd | Binary32 | Exact | timeout | 600 s (limit; killed, no verdict) | 22803144704 bytes (21.24 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_fused_multiply_add_exact |
| FusedMultiplyAdd | Binary32 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 22386270208 bytes (20.85 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float32_fused_multiply_add_directed |
| NumericEqual | Binary32 | n/a | VERIFIED | 1.7 s (wall 4.6 s) | 591974400 bytes (0.55 GiB) | 0.68.0 | native_float_kani_proofs::float32_numeric_equal |
| TotalOrder | Binary32 | n/a | VERIFIED | 1.6 s (wall 4.5 s) | 591728640 bytes (0.55 GiB) | 0.68.0 | native_float_kani_proofs::float32_total_order |
| BitIdentical | Binary32 | n/a | VERIFIED | 1.4 s (wall 4.4 s) | 590188544 bytes (0.55 GiB) | 0.68.0 | native_float_kani_proofs::float32_bit_identical |
| Negate | Binary64 | n/a | VERIFIED | 1.7 s (wall 4.8 s) | 620261376 bytes (0.58 GiB) | 0.68.0 | native_float_kani_proofs::float64_negate |
| Add | Binary64 | NearestEven | VERIFIED | 299.6 s (wall 302.8 s) | 929153024 bytes (0.87 GiB) | 0.68.0 | native_float_kani_proofs::float64_add_nearest_even |
| Add | Binary64 | Exact | timeout | 600 s (limit; killed, no verdict) | 22906060800 bytes (21.33 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_add_exact |
| Add | Binary64 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 22761852928 bytes (21.20 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_add_directed |
| Subtract | Binary64 | NearestEven | VERIFIED | 365.6 s (wall 368.8 s) | 897171456 bytes (0.84 GiB) | 0.68.0 | native_float_kani_proofs::float64_subtract_nearest_even |
| Subtract | Binary64 | Exact | timeout | 600 s (limit; killed, no verdict) | 22906421248 bytes (21.33 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_subtract_exact |
| Subtract | Binary64 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 22380310528 bytes (20.84 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_subtract_directed |
| Multiply | Binary64 | NearestEven | timeout | 600 s (limit; killed, no verdict) | 628924416 bytes (0.59 GiB) | 0.68.0 | native_float_kani_proofs::float64_multiply_nearest_even |
| Multiply | Binary64 | Exact | timeout | 600 s (limit; killed, no verdict) | 23159287808 bytes (21.57 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_multiply_exact |
| Multiply | Binary64 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 22962089984 bytes (21.39 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_multiply_directed |
| Divide | Binary64 | NearestEven | timeout | 600 s (limit; killed, no verdict) | 714199040 bytes (0.67 GiB) | 0.68.0 | native_float_kani_proofs::float64_divide_nearest_even |
| Divide | Binary64 | Exact | timeout | 600 s (limit; killed, no verdict) | 22179082240 bytes (20.66 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_divide_exact |
| Divide | Binary64 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 21637455872 bytes (20.15 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_divide_directed |
| SquareRoot | Binary64 | NearestEven | timeout | 600 s (limit; killed, no verdict) | 944357376 bytes (0.88 GiB) | 0.68.0 | native_float_kani_proofs::float64_square_root_nearest_even |
| SquareRoot | Binary64 | Exact | timeout | 600 s (limit; killed, no verdict) | 21968105472 bytes (20.46 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_square_root_exact |
| SquareRoot | Binary64 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 21884227584 bytes (20.38 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_square_root_directed |
| FusedMultiplyAdd | Binary64 | NearestEven | timeout | 600 s (limit; killed, no verdict) | 22985097216 bytes (21.41 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_fused_multiply_add_nearest_even |
| FusedMultiplyAdd | Binary64 | Exact | timeout | 600 s (limit; killed, no verdict) | 22956912640 bytes (21.38 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_fused_multiply_add_exact |
| FusedMultiplyAdd | Binary64 | directed and nearest-away | timeout | 600 s (limit; killed, no verdict) | 22565724160 bytes (21.02 GiB, above the 16 GiB ceiling) | 0.68.0 | native_float_kani_proofs::float64_fused_multiply_add_directed |
| NumericEqual | Binary64 | n/a | VERIFIED | 1.8 s (wall 4.7 s) | 589656064 bytes (0.55 GiB) | 0.68.0 | native_float_kani_proofs::float64_numeric_equal |
| TotalOrder | Binary64 | n/a | VERIFIED | 1.6 s (wall 4.6 s) | 591192064 bytes (0.55 GiB) | 0.68.0 | native_float_kani_proofs::float64_total_order |
| BitIdentical | Binary64 | n/a | VERIFIED | 1.5 s (wall 4.3 s) | 589537280 bytes (0.55 GiB) | 0.68.0 | native_float_kani_proofs::float64_bit_identical |

Runner and provenance. The harnesses and native bodies measured are those of commit c7ea531 of branch `code/ir-1446-native-float-carrier` (`src/native_float.rs` and `src/native_float/kani_proofs.rs` as committed there). Each row is one executed run of `cargo kani --harness native_float_kani_proofs::<id> --exact --jobs 1 --solver cadical` (Kani 0.68.0, CBMC 6.11.0), one job at a time under `flock /tmp/cargo-slot.lock` and `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`, wrapped in `timeout 600`. The first 19 runs of the table (the eight comparison and Negate harnesses and the Binary32 add, subtract, multiply and divide harnesses) also passed `--unwind 3`; the rest ran without a command-line bound, so each harness's own `#[kani::unwind]` applied (57, for the square-root digit loop whose constant bound is `SQRT_STEPS` = 56). A command-line `--unwind 3` overrides the attribute and makes the square-root harness fail its unwinding assertion, so it was dropped; every other body has loops of at most 3 iterations, and Kani's unwinding assertions are on, so a bound of 3 could not have hidden a longer loop in the runs that carry it. Peak memory is the scope's cgroup `memory.peak`, read inside the scope after the run; the scope held the whole `cargo kani` invocation, so it includes the build and is an upper bound on the solver's peak. The 600 s `timeout` also wraps the whole invocation, so it includes about 5 s of compilation and translation before the solver starts. The 31 non-VERIFIED rows all ended by that `timeout` (exit status 124); none was killed by the scope's memory limit, and none returned a verdict. Their peaks fall in two groups. The NearestEven rows of Divide and SquareRoot (Binary32 and Binary64) and of Multiply Binary64 sit at 0.6 to 0.9 GiB: the solver, not memory, is the limit, and these are timeout rows. The Exact and directed rows of Add, Subtract, Multiply and Divide Binary32 sit at 13.4 to 14.5 GiB, below the 16 GiB ceiling of CG FR-028 and still timeout rows. Every other non-VERIFIED row (20.1 to 21.6 GiB, between 2.4 and 3.9 GiB under the 24 GiB scope limit, which is a configured limit and not the harness's demand) is above the 16 GiB ceiling, so by FR-405-AC-13 it is a non-VERIFIED row and no proof is claimed for it. Every VERIFIED row has a peak below 1 GiB.

Expected side of the harnesses. The harnesses compare the native operation with the host language's own `f32`/`f64` operation for Add, Subtract, Multiply and FusedMultiplyAdd (`+`, `-`, `*`, `mul_add`), computed only for operands whose result is not a NaN (a predicate over the bits decides the NaN cases first, so no NaN-producing host operation runs and Kani's default NaN check cannot fail). For SquareRoot and Divide the expected side is an exact-integer bracket of the correctly rounded result: Kani's model of the host `/` is not bit-exact (under symbolic operands it reports `0x800000A9 / 0x9C873333` as other than `0x1BA00000`, which the hardware, the native body and the same expression over concrete operands all give; the same defect was found as a FAILED verdict of the first Binary32 divide harness and measured on a scratch harness before the bracket replaced it), and its model of `sqrt` is not exact either. The comparison harnesses assert the boolean and the result-unit charge against the host `==`, bit equality and a signed-integer fold of the sign-magnitude order. Negate asserts bitwise sign negation with no flag. None of these shares a function with the native body, not even the bit-pattern decode.

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

Vacuity and harness mutants (FR-405-AC-12). Each of the 13 VERIFIED harnesses has the cover properties listed above (every `kani::cover!` of the harness is reached: 1 for Negate, 2 for each comparison, 3 for each arithmetic harness) and fails a buildable exact-text mutant of the native body, run by `scripts/ir1446_native_float_kani_mutants.py <harness id>` under the same runner and reporting `VERIFICATION:- FAILED`. The mutants are the host mutants of the table of the same names (`negate-quiets`, `numeric-equal-zeros`, `total-order-sign`, `bit-identical-numeric`, `tie-even-is-away`), so each harness mutant is also an AC-10 mutant. All 13 failed their harness. A harness without a VERIFIED verdict has no vacuity record and claims no proof.

Host mutants (FR-405-AC-10). `scripts/ir1446_native_float_mutants.py` builds 49 mutants in scratch copies (one per row of the table above that has a buildable form, one per charge point of `ieee.rs` including `ieee.exact-intermediate` and `ieee.round` each made on a classification-decided input) and requires the named step's test to fail; all 49 were killed at the step they name and none survived. The table rows that name a source inspection (a loop with an operand-dependent exit, a `cfg(kani)` body, a native body calling the owner, a harness checking the native body against itself, a refusal variant for the table-row state) have no mutant: FR-405-AC-7, AC-9 and AC-14 are inspections by code review and have no automated step.

## Expected Results

Every boundary case yields the owner's bits, flags, provenance and charges, with independent literals for the cases named. Every mutant fails as named. Each combination of operation and width ends as an executed VERIFIED verdict or a measured table row, judged alone, with its peak recorded and at most 16 GiB; a combination with neither is inconclusive and no proof is claimed.

## Status

Run under IR-1446. The host tests (`tests/ir1446_native_float.rs`, steps 1 to 4, 6, 7 and the evidence checks of step 8) pass. All 44 Kani harnesses of step 8 ran once to a terminal state; none returned a FAILED verdict. 13 are VERIFIED, each with its covers reached and its harness mutant failing: Negate, NumericEqual, TotalOrder and BitIdentical at both widths; Add and Subtract at both widths and Multiply at Binary32, each under `NearestEven` only. The other 31 are measured table rows (timeout; see the evidence record for each peak, 13 of them at or below the 16 GiB ceiling and 18 above it). Per combination: Negate, NumericEqual, TotalOrder and BitIdentical are VERIFIED at both widths; Add, Subtract, Multiply, Divide, SquareRoot and FusedMultiplyAdd are table rows at both widths, because each has an unverified spelling group (Divide, SquareRoot and FusedMultiplyAdd have no verified group at all). Add and Subtract (both widths) and Multiply (Binary32) hold a proof of their `NearestEven` harness only. No proof is claimed for any other harness or for a table-row combination as a whole. The table rows are for IR-1526 (the owner of the refusal seam and of the tractability table) to load; this repository does not own that table, and the evidence record above is the versioned copy.
