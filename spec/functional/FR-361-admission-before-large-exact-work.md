---
id: FR-361
title: "The kernel admits size charges before large exact work"
type: FR
relationships: []
---
# FR-361: The kernel admits size charges before large exact work

## Description

When an operand-derived or result-derived size charge is denied, the `quire-exact` kernel SHALL return `Incomplete` at that charge point before materializing the large exact value whose size the charge bounds.

## Behavior

The kernel sizes integer and rational arithmetic from operands before forming their intermediate result. Integer division and modulus charge their arithmetic amount before computing the quotient and remainder. Decimal evaluation charges scale expansion and arithmetic from its plan before making the corresponding large intermediate, and charges result retention before expanding the retained coefficient by a target-scale power of ten. Small bookkeeping allocations used to decide and report a charge are permitted.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-361-AC-1 | For a large integer multiplication and a large rational operation, deny the operand-derived `integer-arithmetic.arithmetic` or `rational-arithmetic.arithmetic` size charge at one below its exact amount. Each returns `Incomplete` with the correct point, counter and exact amount; allocator observation during the call shows no result-sized intermediate was allocated. Include a cancellation case whose mathematical result is small but whose operand-derived charge is large. | Test |
| FR-361-AC-2 | Inject denial at `integer-division.arithmetic` and `integer-modulus.arithmetic` for large nonzero operands. Each returns `Incomplete` at the named point, admits no later charge, and allocator observation shows no quotient- or remainder-sized allocation before denial. | Test |
| FR-361-AC-3 | For decimal operands that require a large scale expansion, a denied `decimal.scale-expansion` or `decimal.arithmetic` size charge returns `Incomplete` before allocating the corresponding large power of ten or intermediate coefficient. The incomplete record names the denied point, counter and exact planned amount. | Test |
| FR-361-AC-4 | For a decimal value retained at a target scale requiring a large power of ten, set `decimal_digits` one below the exact result-retain amount while allowing preceding charges. Evaluation returns `Incomplete` at `decimal.result-retain` with the exact denied amount, and allocator observation shows no target-scale coefficient allocation before denial. | Test |

## Status

Planned.

## Dependencies

- [FR-357](./FR-357-single-member-integer-division.md) defines the existing division outcome and charge order; this requirement adds allocation-order evidence for its arithmetic charge. The kernel implementation is in `src/numeric.rs`, `src/division.rs`, and `src/decimal.rs`.
