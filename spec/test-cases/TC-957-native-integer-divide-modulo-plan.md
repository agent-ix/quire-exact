---
id: TC-957
title: "The native integer Divide and Modulo plan matches divide and modulo and settles each Kani harness"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-409
    type: verifies
---
# TC-957: The native integer Divide and Modulo plan matches divide and modulo and settles each Kani harness

## Description

Verify FR-409-AC-1 through FR-409-AC-10. The procedure runs the plan path beside `divide` and `modulo` over `Integer` and beside the plan-free evaluators, inspects the harness and plan source, and records the Kani probe, the verdicts and the peaks. Each mutant below is a buildable variant of the named site in a scratch copy. Each names the input that fails it and the step at which it fails, and a mutant that no input distinguishes is not admitted. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass.

## Test Procedure

The grid is fixed before implementation from independent literals, not from the production body. Dividends and divisors are `0`, `1`, `-1`, `2`, `-2`, `7`, `-7`, `i64::MIN`, `i64::MIN + 1`, `i64::MAX`, `i128::MIN`, `i128::MIN + 1` and `i128::MAX`, crossed. Each pair is run under the truncating, floor and Euclidean laws for the quotient and the remainder, and for `Modulo`, with no result interval, a result interval containing the member and one excluding it.

1. For AC-1, construct a `Divide` plan for each profile and member and a `Modulo` plan over in-capacity intervals that contain zero in the divisor. Admit a member pair, a zero divisor, a dividend outside its interval, a divisor outside its interval and unary operands. Assert the typed error, its position, the operation it names, and that a recording `Meter` sees no charge.
2. For AC-2, construct the quotient plan over a dividend containing `i128::MIN` and a divisor containing `-1` under each profile, with no result interval and with `[-1, 1]`, and assert `CapacityExceeded { operation: Divide }`. Construct the remainder and `Modulo` plans over the same intervals, a dividend `[i128::MIN, i128::MIN]` with divisor `[1, 2]`, and a dividend `[i128::MIN + 1, 0]` with divisor `[-1, -1]`. Compare each capacity with independently written literals, including divisor `[i128::MIN, 5]` (remainder capacity `[-i128::MAX, i128::MAX]`) and divisor `[0, 0]` (capacity `[0, 0]`).
3. For AC-3, evaluate every grid case through the plan, through `divide` or `modulo` over `Integer`, and through `evaluate_native_integer_division` or `evaluate_native_integer_modulo`. Each runs on a fresh recording `Meter` at a sufficient limit and at one below each charge. Compare the outcome kind, the value, the refusal payload (`DivisionOutOfDomain { member }`, `ModuloOutOfDomain`, `Undefined(DivisionByZero)`), every charge event and the counters. Check against independent literals: `-7 div 2` is `-3`, `-4`, `-4`; `-7 div -2` is `3`, `3`, `4`; `-7 mod 2` and `-7 mod -2` are `1`; `i64::MIN div -1` is `2^63`; `i64::MIN mod -1` is `0`. Grid pairs whose quotient plan is refused by step 2 are not evaluated as quotients.
4. For AC-4, run `charge_native_integer_schedule` over each admitted `Divide` and `Modulo` operation and over `Division` and `Modulus` with the same operands, including a zero divisor, and compare events and counters. Re-run the existing native arithmetic and FR-397 tests unchanged.
5. For AC-5 and AC-6, list the harnesses of FR-409 Behavior 13 and 14 and check that each is present: three plan harnesses, and for each of the seven member combinations five symbolic-width harnesses and six constant-divisor harnesses (77 evaluation harnesses). Code review inspects each expected side for a shared function with the production body, for `/`, `%`, `checked_div`, `checked_rem`, `wrapping_div`, `wrapping_rem`, `div_euclid` and `rem_euclid`, for quotient-then-divisor operand order and for a docstring of the outputs it determines. It inspects plan construction and admission for division of a symbolic operand and for `Integer` construction. There is no automated step: a source-text scan is not a test.
6. For AC-7, run the probe first: the truncating quotient harness at `w` = 8 and at `w` = 64 (symbolic divisor), each under `timeout 600`. Then run every harness one job at a time, each under a 600-second limit, inside a memory-limited scope that records its peak, with the cgroup `memory.peak` or an equal measured source as provenance. Record outcome, time, peak, Kani, CBMC and solver versions, commit and harness id. Within a member combination, run the symbolic widths in ascending order. After the first width that is a measured row, record the larger widths as covered by it. For each VERIFIED harness, confirm every cover is reached and run its harness mutant to a FAILED verdict.
7. For AC-8, copy each measured row into the evidence record below and into IR-1526's tractability table, naming operation kind, profile, member, divisor shape and smallest failing width. Code review searches the kernel for a refusal variant, refusal path or `Meter` behaviour defined for a measured row; none may exist.
8. For AC-9, run each mutant through steps 2 and 3 and assert it fails at the step named. Then run it under Kani in the harness named, assert FAILED, extract the concrete playback, and replay the decoded operands through `divide` or `modulo` over `Integer`, asserting that the kernel outcome differs from the mutant's.
9. For AC-10, inspect the Status of FR-409 and of this TC for a verdict or table row per member combination, shape and width, and for no claim without a record.

### Mutants

| Mutant (site) | Distinguishing input | Fails at | Kani harness |
| --- | --- | --- | --- |
| Truncating plan evaluated as floor | `-7 div 2` (quotient `-3` versus `-4`) | step 3 | truncating quotient, `w` = 8 |
| Truncating plan evaluated as Euclidean | `-7 div 2` | step 3 | truncating quotient, `w` = 8 |
| Floor plan evaluated as truncating | `-7 div 2` | step 3 | floor quotient, `w` = 8 |
| Floor plan evaluated as Euclidean | `-7 div -2` (quotient `3` versus `4`) | step 3 | floor quotient, `w` = 8 |
| Euclidean plan evaluated as truncating | `-7 div 2` | step 3 | Euclidean quotient, `w` = 8 |
| Euclidean plan evaluated as floor | `-7 div -2` | step 3 | Euclidean quotient, `w` = 8 |
| `Modulo` evaluated as the truncating remainder | `-7 mod 2` (`1` versus `-1`) | step 3 | `Modulo`, `w` = 8 |
| `Modulo` evaluated as the floor remainder | `-7 mod -2` (`1` versus `-1`) | step 3 | `Modulo`, `w` = 8 |
| Zero-divisor check removed | any dividend over divisor `0` (`Undefined(DivisionByZero)` versus a panic or completion) | step 3 | each profile's quotient, `w` = 8 |
| Zero-divisor check moved after the arithmetic charge | `5 div 0` at a sufficient limit (an extra charge event) | step 3 | none (charges are host-only) |
| MIN / -1 capacity refusal removed | quotient plan over dividend `[i128::MIN, 0]` and divisor `[-1, -1]` | step 2 | quotient plan harness |
| Remainder member returns the quotient | `7 rem 2` under truncating | step 3 | truncating remainder, `w` = 8 |
| Result-interval check skipped | `7 div 2` with result interval `[0, 2]` | step 3 | truncating quotient, `w` = 8 |

## Evidence record

The versioned record of every executed harness, filled by the coder from executed runs. The tractability table that IR-1526 loads is the source of each measured row, and this section is the copy that can be inspected from this repository. Each row gives the harness id, operation kind, profile, member, divisor shape, width, outcome (VERIFIED, or timeout, memory exhaustion or unreliable; a FAILED verdict is a defect, not a row), time, peak memory with provenance, tool versions and commit.

No harness has run. The record is empty and no outcome is claimed.

| Harness id | Kind | Profile | Member | Shape | Width | Outcome | Time | Peak | Tools | Commit |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| (none yet) | | | | | | | | | | |

## Expected Results

Every grid case through the plan has the outcome, value, refusal payload and charge events of `divide` or `modulo` over `Integer` and of the plan-free evaluator. The quotient plan at `i128::MIN` with `-1` is a capacity refusal. Every mutant fails at the step named, and each mutant with a Kani harness yields a FAILED verdict whose playback differs from the kernel on the host. Each harness ends as an executed VERIFIED verdict with its covers and mutant, or as a measured row with its time and peak. No harness is left without a record.

## Status

PLANNED (IR-1541). Not run.
