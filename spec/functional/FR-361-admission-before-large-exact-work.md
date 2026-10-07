---
id: FR-361
title: "The kernel admits size charges before large exact work"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-357
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-358
    type: depends_on
---
# FR-361: The kernel admits size charges before large exact work

## Description

When a size charge for integer or rational arithmetic, integer division or modulus, or decimal scale expansion, arithmetic or result retention is denied, the `quire-exact` kernel SHALL return `Incomplete` at that point before requesting an allocation as large as the value being bounded.

## Behavior

The kernel sizes integer and rational arithmetic from operands before forming their intermediate result. Integer division and modulus charge their arithmetic amount before computing the quotient and remainder. Decimal evaluation charges scale expansion and arithmetic from its plan before making the corresponding intermediate, and charges result retention before expanding the retained coefficient by a target-scale power of ten.

For the allocation criteria, construct operands before opening a measurement window around only the denied call. Observe the largest single allocation request on that thread through a test-only external allocator observer with a safe API. The observer's implementation is outside this crate; this crate's `unsafe_code = "forbid"` remains in force and its tests contain no unsafe allocator implementation.

For an integer or rational arithmetic, division or modulus fixture with operand magnitude at least 65,536 bits, the peak request during denial shall be less than one eighth of the largest operand's byte length. For a decimal fixture with a `2^20`-place shift, the peak shall be below 4,096 bytes. These bounds permit small charge-accounting allocations while excluding the large result, quotient, intermediate and power-of-ten allocations the charge is intended to guard.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-361-AC-1 | For an integer multiplication with a large operand, set `integer_bits` to one below the exact `integer-arithmetic.arithmetic` amount while permitting the operand charge. The outcome is `Incomplete` at `integer-arithmetic.arithmetic` with `limit_kind = IntegerBits` and that exact `next_charge`; the observed peak request meets the arithmetic allocation bound in Behavior. Repeat with operands whose subtraction mathematically cancels to 1 but whose operand-derived arithmetic amount exceeds the limit. | Test |
| FR-361-AC-2 | For a rational operation with a large numerator or denominator, set `integer_bits` to one below the exact `rational-arithmetic.arithmetic` amount while permitting the operand charge. The outcome is `Incomplete` at `rational-arithmetic.arithmetic` with `limit_kind = IntegerBits` and that exact `next_charge`; the observed peak request meets the arithmetic allocation bound in Behavior. Include a multiplication whose unreduced large factors cancel mathematically to 1. | Test |
| FR-361-AC-3 | Inject a one-shot denial at each of `integer-division.arithmetic` and `integer-modulus.arithmetic` for large nonzero operands. Each returns `Incomplete` at the selected point with `limit_kind = WorkUnits`, admits no later point from that call, and meets the division allocation bound in Behavior. On a fresh meter, floor division of the same positive large dividend by 3, exposing the quotient in the mathematical domain, completes with `integer_bits` equal to the dividend magnitude bit length to the independently calculated floor quotient and consumes that exact high-water size. With the bit limit one lower it stops at `integer-division.operands`, with `consumed = 0` and `next_charge` equal to the dividend magnitude bit length, admits nothing and meets the same allocation bound. | Test |
| FR-361-AC-4 | For valid decimal operands requiring a `2^20`-place scale expansion, set `scale_expansion` to one below the planned shift and leave other limits sufficient. The outcome is `Incomplete` at `decimal.scale-expansion` with `limit_kind = ScaleExpansion` and `next_charge` equal to the exact shift; the observed peak request is below 4,096 bytes. | Test |
| FR-361-AC-5 | For decimal addition of small coefficients at scales 0 and `2^20`, set `integer_bits` to the admitted scale-expansion bit amount, one below the planned arithmetic bit amount, with other limits sufficient. The outcome is `Incomplete` at `decimal.arithmetic` with `limit_kind = IntegerBits` and `next_charge` equal to that planned bit amount; the observed peak request is below 4,096 bytes. | Test |
| FR-361-AC-6 | For a decimal value retained at a target scale requiring a `2^20`-place power of ten, set `decimal_digits` one below the exact result-retain amount while permitting preceding charges. Evaluation returns `Incomplete` at `decimal.result-retain` with `limit_kind = DecimalDigits` and that exact `next_charge`; the observed peak request is below 4,096 bytes. Include `Multiply` of retained `(1, 0)` by `(1, 0)` into an exact target with coefficient bounds 0 through 1 and scale range 0 through `2^20`: with `decimal_digits = 2^20`, the record has `limit = 2^20`, `consumed = 2`, `next_charge = 2^20 + 1`; only the three preceding decimal points are admitted and no result unit is consumed. | Test |

## Status

Planned.

## Dependencies

- [FR-357](./FR-357-single-member-integer-division.md) defines the existing division outcome and charge order. [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md) defines the one-shot denial used in AC-3. [TC-909](../test-cases/TC-909-admission-before-large-exact-work.md) is the test-case home. The kernel implementation is in `src/numeric.rs`, `src/division.rs`, and `src/decimal.rs`.
