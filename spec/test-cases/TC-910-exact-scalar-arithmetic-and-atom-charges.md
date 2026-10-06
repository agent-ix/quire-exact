---
id: TC-910
title: "Independent scalar value and atom-charge oracles"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-362
    type: verifies
---
# TC-910: Independent scalar value and atom-charge oracles

## Description

Verify [FR-362](../functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md), AC-1 through AC-9, at kernel scalar entry points without evaluating a caller expression or copying a caller-owned schedule.

## Test Procedure

1. Compare integer operations with checked `i128` results on fixed boundary inputs and reproducibly sampled pairs only where each reference operation fits. Check the two separately specified out-of-range values against their decimal literals.
2. Compare rational operations with independently reduced checked `i128` fractions, including the zero-divisor stop. Compare integer and rational orderings with checked reference comparisons.
3. With `test-support`, inspect the admitted point order and every limit counter for one success of each listed operation. Compute size amounts from the FR's formulas and unreduced rational parts without calling the kernel's amount helpers.
4. Call the direct three-atom sequence in AC-7 on one meter. Inject a denial at each named point in the family rows and inspect each stopped outcome and meter state.
5. Check the truth table, single admitted point and work/result consumption for already-decided Boolean connectives.

## Expected Results

Values, outcomes, point order, high-water sizes, work/result consumption, direct-call totals and denial stops equal their respective criteria. No decimal ordering or caller-level short-circuit schedule is asserted.

## Status

Planned for the IR-653 CODE stage.
