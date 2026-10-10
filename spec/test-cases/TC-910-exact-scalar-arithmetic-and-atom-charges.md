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

Verify [FR-362](../functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md), AC-1 through AC-20, at kernel scalar entry points without evaluating a caller expression or copying a caller-owned schedule.

## Test Procedure

1. Compare integer operations with checked `i128` results on fixed boundary inputs and reproducibly sampled pairs only where each reference operation fits. Check the two separately specified out-of-range values against their decimal literals.
2. Compare rational operations with independently reduced checked `i128` fractions, including the zero-divisor stop. Compare integer and rational orderings with checked reference comparisons.
3. With `test-support`, inspect the admitted point order and every limit counter for one success of each listed operation. Compute size amounts from the FR's formulas and unreduced rational parts without calling the kernel's amount helpers.
4. Call the direct three-atom sequence in AC-7 on one meter. Inject a denial at each named point in the family rows and inspect each stopped outcome and meter state.
5. Check the truth table, single admitted point and work/result consumption for already-decided Boolean connectives.

6. For AC-13, check `RationalDomain::contains(1/3)` against the denominator-excluding domain and divide under that domain, then repeat without a domain.
7. Independently exercise AC-14, AC-15 and AC-16 as three direct atom schedules on separate meters, including their work-short cases.
8. For AC-10, independently calculate exact-bit and one-under-bit requests from the normative formulas and unreduced rational parts. Check completed values, incomplete fields and admitted prefixes.
9. For AC-17, use integer `5 + 7` at work limits 2 and 3, then rational `(2/3) * (3/2)` at work limits 2, 3 and 4; inspect the first unavailable point and unchanged result count. A normalize bit-only denial is not claimed when arithmetic already admitted a larger amount.
10. For AC-18, use independently constructed powers of two and compare exact result bit lengths with the specified charged amounts at each exponent and cancellation fixture.
11. For AC-11, exercise the four public dispositions and extraction behavior, checking every counter and admitted prefix.
12. For AC-12, check exposed numerator and denominator of `4/8`, `-4/-8`, `1/-2`, `0/5` and `0/-5`. A constructor that leaves an unreduced `4/8` fails this criterion.
13. For AC-19, construct fresh public meters for both bit limits of each integer call `8 + 3`, `3 + 8`, `8 - 7` and `8 * 3`, with no result bound and sufficient other limits. Independently calculate both operand bit lengths and the arithmetic request from the FR formula. Call the public integer arithmetic entry point; inspect the completed value or all `Incomplete` fields, `Meter::consumed` for integer bits, value occurrences, work and results, and the admitted point sequence under `test-support`. The reversed add detects a left-only maximum, subtraction detects charging from the one-bit result, and multiplication detects using the add/subtract formula or the operand maximum.

14. For AC-20, call public `evaluate_rational_arithmetic` with fresh sufficient meters and no domain for each of the six fixed calls in the criterion's table. Independently form the unreduced numerator and denominator and count their magnitude bits with `B(0) = 1`, without calling kernel amount helpers. Inspect the actual typed admission records: require the four-point sequence, the arithmetic bound, exactly one normalize size request and its amount, then compare the completed canonical value and work/result consumption. The multiply, subtract and zero-cancellation fixtures discriminate unreduced parts from the reduced value; add, subtract, multiply, divide and zero cancellation discriminate the actual normalize amount from the earlier arithmetic bound. A counter delta or final maximum alone is not evidence for this criterion.

## Expected Results

Values, outcomes, point order, high-water sizes, work/result consumption, direct-call totals and denial stops equal their respective criteria. The five-atom schedule consumes 15/5 work/results and stops at 14/4 under work limit 14; the rational division consumes 4/1 and the two additions 6/2. Bit limits, work limits and power-of-two edge amounts each match their separate ACs. AC-19 exact-limit calls complete with respective results 11, 11, 1 and 24 and bit consumption 5, 5, 5 and 6; one-under calls stop at the arithmetic point with requests 5, 5, 5 and 6 after admitting only operands. Domain refusals preserve arithmetic (and rational normalization), with no retained result. False remains a completed value. A reduced denominator outside its interval refuses despite an admitted numerator; removing the domain permits `1/3`. Construction reduces `4/8` and `-4/-8` to `1/2`, moves the single negative denominator sign to the numerator, and makes zero `0/1` under either denominator sign. For AC-20, actual normalize sizes are respectively 3, 6, 3, 5, 4 and 3, separately from arithmetic sizes 5, 7, 4, 6, 4 and 4. Replacing normalize requests with arithmetic bounds, reduced-result sizes or a constant one bit fails the corresponding controls even where outcomes and final maxima remain unchanged. No decimal ordering or caller-level short-circuit schedule is asserted.

## Status

FR-362-AC-1 through AC-9 retain earlier Trace bindings. PR #9 adds executable Trace bindings for FR-362-AC-11 through AC-18. FR-362-AC-19 is planned for IR-678 as an independently bindable integer boundary test. FR-362-AC-10 remains planned and untagged for IR-667; AC-19 does not claim its rational, negate or ordering coverage. FR-362-AC-20 separately allocates direct normalize-size observation and remains planned until executable bindings land. Its proposed observation does not discharge AC-10's complete boundary scope or claim runtime conformance. Run `quire matrix` for the current criterion-to-test mapping.
