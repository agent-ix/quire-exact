---
id: FR-363
title: "Decimal ordering compares values and charges retained representations"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-362
    type: references
  - target: ix://agent-ix/quire-exact/FR-358
    type: depends_on
---
# FR-363: Decimal ordering compares values and charges retained representations

## Description

When `order_numbers` receives two decimal operands, the `quire-exact` kernel SHALL compare their exact mathematical values and charge the retained operand representations before exposing the Boolean result.

## Behavior

For a retained decimal pair `(c₁, s₁)` and `(c₂, s₂)`, let `t = max(s₁, s₂)` and `kᵢ = t - sᵢ`. Let `B(x)` be the magnitude bit length with `B(0) = 1`, and `D(x)` the magnitude decimal digit count with `D(0) = 1`. Define `SB(c,0) = B(c)`, `SB(c,k) = B(c) + B(10^k)` for positive `k`, and `SD(c,k) = D(c) + k`. These amounts are exact mathematical integers; calculating them for a charge does not require materializing `10^k`.

The ordered charges are:

| Point | Size amounts | Cumulative amounts |
| --- | --- | --- |
| `ordering.operands` | `integer_bits = max(B(c₁), B(c₂))`, `decimal_digits = max(D(c₁), D(c₂))`, `value_occurrences = 2` | One work unit |
| `ordering.arithmetic` | `scale_expansion = max(k₁, k₂)`, `integer_bits = max(SB(c₁,k₁), SB(c₂,k₂))`, `decimal_digits = max(SD(c₁,k₁), SD(c₂,k₂))` | One work unit |
| `ordering.result-retain` | None | One work unit and one result unit |

Sizes are high-water maxima; work and result units are cumulative. The comparison uses the decimals' normalized mathematical values. The charge amounts use their retained `(coefficient, scale)` representations even when the normalized values are equal. A denied charge returns `Incomplete` before any later point is admitted.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-363-AC-1 | For decimal pairs with equal values but different retained scales, and for positive, negative and zero values at differing scales, `order_numbers` agrees with an independent exact rational comparison for each ordering operator. The retained representations remain unchanged. | Test |
| FR-363-AC-2 | For a successful decimal comparison, the meter admits `ordering.operands`, `ordering.arithmetic`, `ordering.result-retain` in that order. On a fresh meter, its size high-water counters equal the maxima in Behavior, with three work units and one result unit. | Test |
| FR-363-AC-3 | Comparing retained `(0, 5) >= (0, 0)` completes `true` and consumes `integer_bits = 18`, `decimal_digits = 6`, `scale_expansion = 5`, `value_occurrences = 2`, `work_units = 3`, `result_units = 1`. Comparing retained `(100, 2) <= (2, 0)` completes `true` and consumes `integer_bits = 9`, `decimal_digits = 3`, `scale_expansion = 2`, `value_occurrences = 2`, `work_units = 3`, `result_units = 1`. | Test |
| FR-363-AC-4 | With all limits at `u64::MAX` except `decimal_digits = 64`, testing retained `(1, 0) < (1, u32::MAX)` returns `Incomplete` at `ordering.arithmetic` with `limit_kind = DecimalDigits`, `limit = 64`, `consumed = 1`, and `next_charge = 4,294,967,296`. Only `ordering.operands` is admitted; no result unit is consumed and the scale-alignment power of ten is not materialized. | Test |
| FR-363-AC-5 | On separate fresh meters with generous limits, inject a one-shot denial at occurrence 1 of each decimal-ordering point: `ordering.operands`, `ordering.arithmetic`, and `ordering.result-retain`. For each point, ordering retained `(0, 5) >= (0, 0)` returns `Incomplete` with `limit_kind = WorkUnits`, `charge_point` equal to the selected point, `limit = consumed` equal to the number of preceding admitted points (0, 1, or 2), and `next_charge = 1`. No Boolean result or result unit is exposed; only the preceding point prefix is admitted and logged, with no later point admitted. | Test |
| FR-363-AC-6 | For retained decimals `(110, 2)` and `(11, 1)`, public unmetered `Decimal::compare` returns `Equal`, `numerically_equal` returns true and both normalized representations are `(11, 1)`; their retained representations remain `(110, 2)` and `(11, 1)`. | Test |
| FR-363-AC-7 | On separate fresh meters, `(110, 2) < (11, 1)` and `(11, 1) < (11, 1)` both complete false. The mixed pair consumes integer bits, decimal digits and scale expansion 8/3/1; the narrow pair consumes 4/2/0. Both consume value occurrences/work/results 2/3/1 with all other counters zero, despite equal normalized values. | Test |

## Status

PR #9 adds executable Trace bindings for AC-6 and AC-7. Run `quire matrix` for the current criterion-to-test mapping.

## Dependencies

- [FR-362](./FR-362-exact-scalar-arithmetic-and-atom-charges.md) owns the integer and rational ordering families. [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md) defines the one-shot denial used in AC-5. [TC-911](../test-cases/TC-911-metered-decimal-ordering.md) is the test-case home. The decimal ordering branch is in `src/numeric.rs` and its exact comparison primitive is in `src/decimal.rs`.
