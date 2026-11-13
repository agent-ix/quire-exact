---
id: TC-956
title: "The native Rational carrier matches the owner route and verifies"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-408
    type: verifies
---
# TC-956: The native Rational carrier matches the owner route and verifies

## Description

Verify FR-408-AC-1 through FR-408-AC-11 through the public kernel API, a literal-charge test, the owner differential and the recorded Kani measurement. The differential lives in quire-exact's `tests/`, because the owner route (`Rational`, `evaluate_rational_arithmetic`, `order_numbers`, `planned_equality`) is in the same crate. Each mutant names the input or inspection that fails it.

## Test Procedure

1. For AC-1, check `Copy` by a generic bound, no drop glue by `needs_drop`, zero allocations of construction, accessors and every operation in a counting allocator window, and that no field is public (a struct-literal program fails with `E0451`).
2. For AC-2, call `from_i64_parts` over the admitted, `ZeroDenominator` and `NonCanonical` pairs of the criterion with literals written independently of the carrier, with a `Meter`-free signature (coerce to a function pointer), and check the precedence with `(0, 0)`.
3. For AC-3, round-trip the CG domain (111 canonical values) and the boundary set through `from_rational` and `to_rational` and compare with `Rational::new`; present `2^127/1` and `1/2^127`.
4. For AC-4 and AC-5, evaluate the literal cases of the criteria and the four orderings and equality over every ordered pair of `{-1/2, 0/1, 1/3, 1/2, 2/3}` against a table written independently.
5. For AC-6, present the capacity operands of the criterion to each operation, with a recording `Meter` at a zero budget, and assert the named quantity and the unchanged `Meter`; assert that no pair of `i64` parts is a capacity refusal for any operation over a boundary sample and a random sample.
6. For AC-7 and AC-8, run the literal charge cases (including `9/1 + 9/1`, operands 4, arithmetic 6) under a recording `Meter` at a sufficient, an exact and a one-less limit for each limit kind used and compare the charge points, limit kinds, amounts, denied charge, counters and incomplete record with literals written independently of the carrier; run the `a + b`, `b + a`, `=` sequence on one `Meter`, with the denial placed in the second addition.
7. For AC-9, run the differential of FR-408 Behavior 19 beside the owner route over the 12,321 pairs of the CG domain for each of the seven operations and over the boundary set, at the three limits, comparing outcome, value, Boolean, every charge, the denied charge, counters and the incomplete record; list the capacity inputs and check each.
8. AC-10 is a code-review inspection item (FR-408 Behavior 15 and the one-shared-module rule); it has no automated step. The one-source property is evidenced functionally by step 9 running the shipped function.
9. For AC-11, once the code lead authorizes the run, execute the harnesses of FR-408 Behavior 18 under the authorized Kani runner and record time, peak memory with provenance, tool version and outcome in the evidence record below; confirm every cover is reached and each mutant fails. Until then this step is not executable.
10. Run each mutant through the step it names.

### Mutants

| Mutant | Fails at |
| --- | --- |
| Add forms `a q - c p` | `1/2 + 1/3` of step 4 |
| Subtract has its operands swapped | `-1/2 - 1/3` of step 4 |
| The result is not reduced | `1/2 + 1/2` of step 4 and the differential |
| Reduction runs fewer than 256 gcd steps (for example 128) | the 252-step pair `(2^126 - 1)/2^126` times `1/1` in the differential boundary set of step 7 |
| The sign is put on the denominator instead of the numerator | `-1/2 - 1/3` of step 4 |
| Zero is retained as `0/d` | `1/2 - 1/2` of step 4 |
| `from_i64_parts` normalizes instead of refusing | `(2, 4)` of step 2 |
| `NonCanonical` is judged after `ZeroDenominator` is skipped | `(0, 0)` of step 2 |
| `(0, 2)` is admitted | step 2 |
| Equality compares the cross product only for the numerators | `1/2` against `-1/2` of step 4 |
| `LessOrEqual` is evaluated as `Less` | the equal pair of step 4 and the differential |
| Ordering compares `a p` with `c q` | `1/3 < 1/2` of step 4 |
| The sum wraps instead of refusing | the `Combination` case of step 5 |
| Subtract negates `c` and refuses on `i128::MIN` | `-1/1 - i128::MIN/1` of step 5 |
| A capacity refusal charges the `Meter` | step 5 |
| The normalize charge uses the reduced parts | `1/2 + 1/2` of step 6 and the differential |
| The arithmetic charge omits the `+ 1` of add | `9/1 + 9/1` of step 6 (arithmetic `integer_bits` 6, not 5) and the differential |
| `result-retain` is charged before `normalize` | step 6 and the differential |
| A denied charge consumes a result unit | the one-less limits of step 6 |
| Equality charges the ordering schedule | step 6 |
| The carrier owns a `Rational` | step 1 |

## Evidence record

No measurement exists. Columns: operation, outcome, wall time, peak memory, Kani version, harness id.

| Operation | Outcome | Wall time | Peak memory | Kani version | Harness id |
| --- | --- | --- | --- | --- | --- |

## Expected Results

Every step passes, each mutant fails at the step it names, and the evidence record holds an executed row for FR-408-AC-11 or the criterion stays planned.

## Status

Specified (IR-258). No step has run.
