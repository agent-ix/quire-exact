---
id: FR-362
title: "Exact scalar arithmetic and atom charges agree with independent oracles"
type: FR
relationships: []
---
# FR-362: Exact scalar arithmetic and atom charges agree with independent oracles

## Description

When evaluating kernel-owned integer or rational arithmetic, numeric ordering, or a Boolean connective over already-decided operands, the `quire-exact` kernel SHALL return the exact outcome and charge the specified scalar atom points and amounts.

## Behavior

`Integer` is an arbitrary-precision mathematical integer. A finite `i128` reference calculation is a verification oracle for sampled values whose reference operation fits; it does not bound accepted kernel values. The caller owns expression evaluation and any choice to skip a right Boolean operand. The kernel owns retention of the already-decided Boolean value.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-362-AC-1 | Over a fixed set containing `i128::MIN`, -1, 0, 1 and `i128::MAX`, plus reproducibly sampled signed pairs whose reference operation fits, integer add, subtract, multiply and negate agree with independently checked `i128` results. A product exceeding `i128::MAX` still completes to its exact arbitrary-precision `Integer` under sufficient limits. | Test |
| FR-362-AC-2 | For fixed and reproducibly sampled rational pairs whose `i128` reference intermediates fit and whose denominators are nonzero, rational add, subtract, multiply, divide by nonzero and negate agree with an independently reduced-fraction oracle; division by zero returns `Undefined(DivisionByZero)`. Integer and rational ordering agree with independently checked comparisons. | Test |
| FR-362-AC-3 | For integer arithmetic, rational arithmetic and numeric ordering, successful atom calls admit their family’s named operand, arithmetic, normalize where applicable, and result-retain points in order. Independent formulas over input magnitudes and unreduced rational intermediates agree with the observed high-water size counters, value occurrences, cumulative work units and result units. The P11 scalar loop atoms for `n = 2` consume 15 work units and 5 result units; the Q11 two-step integer fold over 1 and 2 consumes 6 work units and 2 result units. | Test |
| FR-362-AC-4 | Inject a one-shot denial at each named point of representative integer arithmetic, rational arithmetic, ordering and already-decided Boolean retention calls. Each returns `Incomplete` at that point with no completed value, consumes no result unit for the denied charge, and admits no later point from that call. A successful already-decided Boolean connective charges `boolean.result-retain` once. | Test |

## Status

Planned.

## Dependencies

- None. This requirement concerns the existing kernel operations in `src/numeric.rs`; callers retain ownership of expression evaluation and short-circuit choice.
