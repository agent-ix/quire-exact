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

Verify [FR-362](../functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md), AC-1 through AC-12, at kernel scalar entry points without evaluating a caller expression or copying a caller-owned schedule.

## Test Procedure

1. Compare integer operations with checked `i128` results on fixed boundary inputs and reproducibly sampled pairs only where each reference operation fits. Check the two separately specified out-of-range values against their decimal literals.
2. Compare rational operations with independently reduced checked `i128` fractions, including the zero-divisor stop. Compare integer and rational orderings with checked reference comparisons.
3. With `test-support`, inspect the admitted point order and every limit counter for one success of each listed operation. Compute size amounts from the FR's formulas and unreduced rational parts without calling the kernel's amount helpers.
4. Call the direct three-atom sequence in AC-7 on one meter. Inject a denial at each named point in the family rows and inspect each stopped outcome and meter state.
5. Check the truth table, single admitted point and work/result consumption for already-decided Boolean connectives.

6. Execute the fixed five scalar atoms, rational `3/1` divided by `2/1`, and two integer additions in AC-7, without a caller evaluator. Compare every result, ordered prefix and counter total; repeat the named work-short schedules.
7. For AC-10, compute amounts from the normative formulas and independently formed unreduced rational parts, rather than owner amount helpers. Run exact-bit, one-under-bit and the stated work-boundary calls, including power-of-two edges and cancellation. Check all incomplete fields and admitted state. A normalize bit-only denial cannot be isolated when its amount is no larger than an already-admitted arithmetic amount; use the stated work boundary to exercise that stop.
8. Exercise the four public dispositions and extraction behavior in AC-11, checking the full admitted prefix and every counter at each stop.

9. Check `RationalDomain::contains` and division under the denominator-excluding domain in AC-3, then repeat the division without a domain. Check the positive-numerator/negative-denominator and zero constructor parts in AC-12. Existing owner reduction and two-negative sign tests remain separate evidence.

## Expected Results

Values, outcomes, point order, high-water sizes, work/result consumption, direct-call totals and denial stops equal their respective criteria. The five-atom schedule consumes 15/5 work/results and stops at 14/4 under work limit 14; the rational division consumes 4/1 and the two additions 6/2. Exact-limit calls complete and one-under calls retain only their admitted prefixes. Domain refusals preserve arithmetic (and rational normalization), with no retained result. False remains a completed value. A reduced denominator outside its interval refuses despite an admitted numerator; removing the domain permits `1/3`. Positive-over-negative construction moves the sign to the numerator, and zero is `0/1` under either denominator sign. No decimal ordering or caller-level short-circuit schedule is asserted.

## Status

Planned for the IR-653 CODE stage.
