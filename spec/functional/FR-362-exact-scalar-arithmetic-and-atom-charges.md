---
id: FR-362
title: "Exact scalar arithmetic and atom charges agree with independent oracles"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-358
    type: depends_on
---
# FR-362: Exact scalar arithmetic and atom charges agree with independent oracles

## Description

When evaluating kernel-owned integer or rational arithmetic, integer or rational ordering, or a Boolean connective over already-decided operands, the `quire-exact` kernel SHALL return the exact outcome and charge the point order and amounts defined below.

The kernel SHALL construct rational values with a positive denominator and a unique zero representation `0/1`.

## Behavior

`Integer` is an arbitrary-precision mathematical integer. A checked `i128` calculation is an independent oracle only where that reference operation fits. For a result outside `i128`, an independently authored decimal literal supplies the expected value. The caller owns expression evaluation and any choice to skip a right Boolean operand. [FR-363](./FR-363-metered-decimal-ordering.md) owns decimal ordering values and charges separately from this finite `i128` oracle and charge table.

For the following amounts, `B(x)` is the bit length of the magnitude of `x`, with `B(0) = 1`. Rational operands are `a/p` and `c/q` in reduced canonical form. A size amount updates its counter to the maximum of the old value and the amount. Every admitted named point consumes one work unit. Only a result-retain point consumes one result unit.

| Family | Ordered points | Size amounts |
| --- | --- | --- |
| Integer arithmetic | `integer-arithmetic.operands`, `integer-arithmetic.arithmetic`, `integer-arithmetic.result-retain` | For binary `a op c`, operands: `integer_bits = max(B(a), B(c))`, `value_occurrences = 2`; for negate, `integer_bits = B(a)`, `value_occurrences = 1`. Arithmetic: add/subtract `max(B(a), B(c)) + 1`; multiply `B(a) + B(c)`; negate `B(a)`, all as `integer_bits`. |
| Rational arithmetic | `rational-arithmetic.operands`, `rational-arithmetic.arithmetic`, `rational-arithmetic.normalize`, `rational-arithmetic.result-retain` | Operands: `integer_bits = max(B(a), B(p), B(c), B(q))`, `value_occurrences = 2` (for negate, `max(B(a), B(p))` and one occurrence). Arithmetic `integer_bits` is `max(N,D)`: add/subtract `N = max(B(a)+B(q), B(c)+B(p)) + 1`, `D = B(p)+B(q)`; multiply `N = B(a)+B(c)`, `D = B(p)+B(q)`; divide `N = B(a)+B(q)`, `D = B(p)+B(c)`; negate `N = B(a)`, `D = B(p)`. Normalize `integer_bits` is the maximum actual bit length of the unreduced numerator and denominator. |
| Integer ordering | `ordering.operands`, `ordering.arithmetic`, `ordering.result-retain` | Operands: `integer_bits = max(B(a), B(c))`, `value_occurrences = 2`. Arithmetic: `integer_bits = max(B(a), B(c))`. |
| Rational ordering | `ordering.operands`, `ordering.arithmetic`, `ordering.result-retain` | Operands: `integer_bits = max(B(a), B(p), B(c), B(q))`, `value_occurrences = 2`. Arithmetic: `integer_bits = max(B(a)+B(q), B(c)+B(p))`. |
| Already-decided Boolean retention | `boolean.result-retain` | No size amount; one work unit and one result unit. |

An integer result outside an optional bound refuses after arithmetic and before result retention. A zero rational divisor is `Undefined(DivisionByZero)` after operands, with no arithmetic, normalize or result retention charge. A rational result outside its optional domain refuses after normalization and before result retention.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-362-AC-1 | For binary operations, use pairs from a fixed set including `i128::MIN`, -1, 0, 1 and `i128::MAX`; for negate, use each member as a unary operand. Compare integer add, subtract, multiply and negate only when the corresponding checked `i128` operation returns a value. Repeat over reproducibly sampled signed inputs. Every compared kernel result equals the independent checked result. | Test |
| FR-362-AC-2 | Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes to the independently authored decimal value `340282366920938463463374607431768211454`; negating `Integer::from(i128::MIN)` completes to `170141183460469231731687303715884105728`. Neither result is narrowed to `i128`. | Test |
| FR-362-AC-3 | For fixed and reproducibly sampled rational pairs whose checked `i128` reference intermediates fit and whose denominators are nonzero, rational add, subtract, multiply, divide by nonzero and negate equal an independently reduced-fraction oracle. A zero rational divisor returns `Undefined(DivisionByZero)` after only the operand charge. With numerator interval `[0, 1]` and denominator interval `[1, 2]`, `RationalDomain::contains(1/3)` is false; dividing `1/1` by `3/1` under that domain returns `Refused(RationalOutOfDomain { target })` after normalization with no result unit, while the same call without a domain completes to `1/3`. | Test |
| FR-362-AC-4 | Integer and rational comparisons over fixed and reproducibly sampled pairs equal independently checked numeric comparisons for equality and order. Decimal comparison is specified by [FR-363](./FR-363-metered-decimal-ordering.md). | Test |
| FR-362-AC-5 | For a successful representative of each integer arithmetic operation, each rational arithmetic operation, integer ordering and rational ordering, the admitted point sequence equals its family row in Behavior. A successful already-decided Boolean retention admits only `boolean.result-retain`. | Test |
| FR-362-AC-6 | For those successful operations on fresh meters, independently calculate the maximum size amount across their points from the formulas in Behavior, using the unreduced numerator and denominator for rational normalization. The observed final `integer_bits` and `value_occurrences` equal those maxima; work consumption equals the number of admitted points and result consumption is one. | Test |
| FR-362-AC-7 | On one meter, directly add integers 5 and 7, compare rationals 1/2 and 2/3 for less-than, then retain an already-decided Boolean `true`. The admitted atom sequence consumes 7 work units and 3 result units, derived from the three family rows in Behavior. Also call the fixed five-atom sequence `2 > 0`, `2 - 1`, `1 > 0`, `1 - 1`, `0 > 0` on one meter: completed values are `true`, 1, `true`, 0, `false`, with 15 work units and 5 result units. At work limit 14, the last atom stops at `ordering.result-retain`, with `limit = consumed = 14`, `next_charge = 1` and four result units. Separately divide `3/1` by `2/1`: the result is `3/2`, high-water bits 3, occurrences 2, work 4 and results 1; work limit 3 stops at rational result retention. Two integer additions `0 + 1`, then `1 + 2` produce 1 then 3, high-water bits 3, occurrences 2, work 6 and results 2, with the integer point sequence repeated twice. These are direct kernel calls; caller recursion, function-call charges and fold evaluation are outside this criterion. | Test |
| FR-362-AC-8 | Inject a one-shot denial at each named point in representative integer arithmetic, rational arithmetic, integer ordering, rational ordering and already-decided Boolean retention calls. Each returns `Incomplete` naming the denied point, with no completed value, no result unit for the denied charge and no later point admitted by that call. | Test |
| FR-362-AC-9 | For every already-decided Boolean connective, the completed value matches its truth table and the kernel admits `boolean.result-retain` once, consuming one work and one result unit. | Test |
| FR-362-AC-10 | For integer add, subtract, multiply and negate, rational add, subtract, multiply, divide and negate, and integer/rational ordering, independently calculate the required bit high-water amount from Behavior. On fresh meters with other limits sufficient, that exact bit limit completes with the expected value, while one below stops at the first point whose amount exceeds it, with exact prior consumption, request and admitted prefix and no result unit. Include cancellation `1000 - 999`, `255 * 255`, rational `(2/3) * (3/2)`, `(3/4) / (5/7)`, `(5/7) - (4/7)`, negate `-5/8`, and `7/3 < 5/8`. For rational calls, normalization charges the actual unreduced parts even when the reduced value is smaller; work limits 2 and 3 stop at normalize and result-retain respectively, while 4 completes. For integer calls work 2 stops at result-retain while 3 completes. Include integer results around `2^k` for k = 63, 64, 65, 127, 128, 200 and 511, verifying the charged bound covers the exact result bit length and remains operand-derived under cancellation. | Test |
| FR-362-AC-11 | Public scalar calls distinguish `Completed(false)`, `Undefined(DivisionByZero)`, typed domain `Refused`, and `Incomplete` with no partial value; `completed()` returns a value only for `Completed`, including false. On fresh meters, rational division by zero retains only operands; integer `2 + 2` outside the bound `[0, 3]` retains operands and arithmetic but no result unit; a rational result outside its domain retains operands, arithmetic and normalize but no result unit. A denied first operand charge admits nothing. Each stop preserves exactly the admitted prefix, work totals and size maxima, with zero results. | Test |
| FR-362-AC-12 | `Rational::new(1, -2)` exposes numerator -1 and denominator 2. For either denominator 5 or -5, a zero numerator exposes exactly numerator 0 and denominator 1. These unmetered constructor results use the same canonical rational value type as arithmetic. | Test |

## Status

Planned.

## Dependencies

- [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md) defines the one-shot denial used in AC-8. [TC-910](../test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md) is the test-case home. The kernel operations are in `src/numeric.rs`; callers retain ownership of expression evaluation and short-circuit choice.
