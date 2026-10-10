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

The kernel SHALL construct rational values with a positive denominator, numerator and denominator reduced to lowest terms, and a unique zero representation `0/1`.

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

Where `test-support` is enabled, the kernel SHALL expose each admitted rational normalize `integer_bits` request through the actual size observations defined by [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md), independently of the final high-water counter and the reduced result.

An integer result outside an optional bound refuses after arithmetic and before result retention. A zero rational divisor is `Undefined(DivisionByZero)` after operands, with no arithmetic, normalize or result retention charge. A rational result outside its optional domain refuses after normalization and before result retention.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-362-AC-1 | For binary operations, use pairs from a fixed set including `i128::MIN`, -1, 0, 1 and `i128::MAX`; for negate, use each member as a unary operand. Compare integer add, subtract, multiply and negate only when the corresponding checked `i128` operation returns a value. Repeat over reproducibly sampled signed inputs. Every compared kernel result equals the independent checked result. | Test |
| FR-362-AC-2 | Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes to the independently authored decimal value `340282366920938463463374607431768211454`; negating `Integer::from(i128::MIN)` completes to `170141183460469231731687303715884105728`. Neither result is narrowed to `i128`. | Test |
| FR-362-AC-3 | For fixed and reproducibly sampled rational pairs whose checked `i128` reference intermediates fit and whose denominators are nonzero, rational add, subtract, multiply, divide by nonzero and negate equal an independently reduced-fraction oracle. A zero rational divisor returns `Undefined(DivisionByZero)` after only the operand charge. | Test |
| FR-362-AC-4 | Integer and rational comparisons over fixed and reproducibly sampled pairs equal independently checked numeric comparisons for equality and order. Decimal comparison is specified by [FR-363](./FR-363-metered-decimal-ordering.md). | Test |
| FR-362-AC-5 | For a successful representative of each integer arithmetic operation, each rational arithmetic operation, integer ordering and rational ordering, the admitted point sequence equals its family row in Behavior. A successful already-decided Boolean retention admits only `boolean.result-retain`. | Test |
| FR-362-AC-6 | For those successful operations on fresh meters, independently calculate the maximum size amount across their points from the formulas in Behavior, using the unreduced numerator and denominator for rational normalization. The observed final `integer_bits` and `value_occurrences` equal those maxima; work consumption equals the number of admitted points and result consumption is one. | Test |
| FR-362-AC-7 | On one meter, directly add integers 5 and 7, compare rationals 1/2 and 2/3 for less-than, then retain an already-decided Boolean `true`. The admitted atom sequence consumes 7 work units and 3 result units, derived from the three family rows in Behavior. | Test |
| FR-362-AC-8 | Inject a one-shot denial at each named point in representative integer arithmetic, rational arithmetic, integer ordering, rational ordering and already-decided Boolean retention calls. Each returns `Incomplete` naming the denied point, with no completed value, no result unit for the denied charge and no later point admitted by that call. | Test |
| FR-362-AC-9 | For every already-decided Boolean connective, the completed value matches its truth table and the kernel admits `boolean.result-retain` once, consuming one work and one result unit. | Test |
| FR-362-AC-10 | For integer add, subtract, multiply and negate, rational add, subtract, multiply, divide and negate, and integer/rational ordering, independently calculate the required bit high-water amount from Behavior. On fresh meters with other limits sufficient, that exact bit limit completes with the expected value, while a limit one below stops at the first point whose amount exceeds it, with exact prior consumption, request and admitted prefix and no result unit. Include cancellation `1000 - 999`, `255 * 255`, rational `(2/3) * (3/2)`, `(3/4) / (5/7)`, `(5/7) - (4/7)`, negate `-5/8`, and `7/3 < 5/8`. Rational normalize sizes use actual unreduced parts even when the reduced value is smaller. | Test |
| FR-362-AC-11 | Public scalar calls distinguish `Completed(false)`, `Undefined(DivisionByZero)`, typed domain `Refused`, and `Incomplete` with no partial value; `completed()` returns a value only for `Completed`, including false. On fresh meters, rational division by zero retains only operands; integer `2 + 2` outside the bound `[0, 3]` retains operands and arithmetic but no result unit; a rational result outside its domain retains operands, arithmetic and normalize but no result unit. A denied first operand charge admits nothing. Each stop preserves exactly the admitted prefix, work totals and size maxima, with zero results. | Test |
| FR-362-AC-12 | `Rational::new(4, 8)` and `Rational::new(-4, -8)` each expose numerator 1 and denominator 2, proving reduction to lowest terms and cancellation of two negative signs. `Rational::new(1, -2)` exposes -1/2. For either denominator 5 or -5, a zero numerator exposes exactly 0/1. | Test |
| FR-362-AC-13 | For numerator interval `[0, 1]` and denominator interval `[1, 2]`, `RationalDomain::contains(1/3)` is false. Dividing `1/1` by `3/1` under that domain returns `Refused(RationalOutOfDomain { target })` after normalization with no result unit; the identical call without a domain completes to `1/3`. | Test |
| FR-362-AC-14 | Call the five scalar atoms `2 > 0`, `2 - 1`, `1 > 0`, `1 - 1`, `0 > 0` directly on one meter. They complete to true, 1, true, 0, false in order, consuming 15 work and 5 result units. With work limit 14, the final ordering stops at `ordering.result-retain` with `limit = consumed = 14`, `next_charge = 1` and four result units. | Test |
| FR-362-AC-15 | Directly divide rational `3/1` by `2/1` on a fresh meter. The result is `3/2`, with high-water `integer_bits = 3`, value occurrences 2, work 4 and results 1; work limit 3 stops at `rational-arithmetic.result-retain` after the first three points and consumes no result unit. | Test |
| FR-362-AC-16 | On one meter directly add integers `0 + 1` and then `1 + 2`. The results are 1 and 3; the integer point sequence occurs twice, high-water `integer_bits = 3`, occurrences 2, work 6 and results 2. Caller fold evaluation is outside this direct-atom criterion. | Test |
| FR-362-AC-17 | For integer `5 + 7`, work limit 2 stops at `integer-arithmetic.result-retain` after operands and arithmetic, with zero result units; work limit 3 completes to 12 with three work and one result unit. For rational `(2/3) * (3/2)`, work limit 2 stops at `rational-arithmetic.normalize`, work limit 3 stops at `rational-arithmetic.result-retain`, and work limit 4 completes to `1/1`; each denied call preserves exactly its admitted prefix and consumes zero result units. | Test |
| FR-362-AC-18 | For `k` in 63, 64, 65, 127, 128, 200 and 511, compare exact results and charged bit high-water sizes around `2^k`: `(2^k - 1)(2^k + 1)` charges `2k + 1` bits for a `2k`-bit result, `(2^k)^2` charges `2k + 2` for a `2k + 1`-bit result, and adding `(2^k - 1) + 1` charges `k + 1` for a `k + 1`-bit result. Subtracting `(2^k + 1) - 2^k` or `2^k - 2^k` still charges `k + 2` bits despite a one-bit result. | Test |
| FR-362-AC-19 | Through public `evaluate_integer_arithmetic` and a fresh public `Meter` for each call, add `8 + 3` and `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`, without a result bound and with all non-bit limits sufficient. The independently calculated `integer-arithmetic.arithmetic` bit amounts are respectively 5, 5, 5 and 6: add/subtract use `max(B(a), B(c)) + 1`, including the larger right operand and a result that cancels to one bit; multiply uses `B(a) + B(c)`. At each exact `integer_bits` limit, the call completes to respectively 11, 11, 1 and 24, admits operands, arithmetic and result-retain in order, and reports `integer_bits` consumed equal to the arithmetic amount, `value_occurrences = 2`, three work units and one result unit. At one below each exact limit, it returns `Incomplete` with no value at `integer-arithmetic.arithmetic`, `limit_kind = IntegerBits`, `limit` one below the arithmetic amount, `consumed = 4`, and `next_charge` equal to the arithmetic amount; only `integer-arithmetic.operands` was admitted, leaving two value occurrences, one work unit and zero result units. | Test |
| FR-362-AC-20 | On fresh sufficient public meters under `test-support`, evaluate the six rational calls listed below with no result domain. Each completes to the listed canonical value and admits operands, arithmetic, normalize and result-retain in order, consuming four work units and one result unit. The actual normalize record contains exactly one size request, IntegerBits with the listed amount independently derived from the unreduced parts and `B(0) = 1`. The recorded arithmetic amount equals its separate listed bound. The normalize observation is not replaced by that bound, the final bit maximum or the reduced value's part lengths. | Test |

### FR-362-AC-20

| Call | Unreduced numerator/denominator | Arithmetic IntegerBits | Normalize IntegerBits | Canonical result |
| --- | --- | --- | --- | --- |
| `(1/2) + (2/3)` | `7/6` | 5 | 3 | `7/6` |
| `(5/7) - (4/7)` | `7/49` | 7 | 6 | `1/7` |
| `(2/3) * (3/2)` | `6/6` | 4 | 3 | `1/1` |
| `(3/4) / (5/7)` | `21/20` | 6 | 5 | `21/20` |
| negate `-5/8` | `5/8` | 4 | 4 | `5/8` |
| `(1/2) - (1/2)` | `0/4` | 4 | 3 | `0/1` |

## Status

PR #9 adds executable Trace bindings for AC-11 through AC-18. AC-1 through AC-9 retain earlier bindings. AC-19 is the separately testable public-meter integer add/subtract/multiply boundary slice; its executable binding is planned under IR-678. AC-10 remains planned and untagged for IR-667; the current point-only public test-support log cannot expose the rational normalize charge bit amount. AC-20 allocates the direct normalize-size observation separately and remains planned until implementation and executable bindings land. The enriched observation contract alone does not discharge AC-10's complete exact-limit and one-under-limit scope or claim runtime conformance. AC-19 does not discharge AC-10's negate, rational or ordering cases. Run `quire matrix` for the current criterion-to-test mapping.

## Dependencies

- [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md) defines the one-shot denial used in AC-8. [TC-910](../test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md) is the test-case home. The kernel operations are in `src/numeric.rs`; callers retain ownership of expression evaluation and short-circuit choice.
