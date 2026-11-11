---
id: FR-403
title: "Native Decimal operations refine the owner's metered Decimal path"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-384
    type: references
  - target: ix://agent-ix/quire-exact/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-140
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-051
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
---
# FR-403: Native Decimal operations refine the owner's metered Decimal path

## Description

Where a caller selects the native Decimal API over the carried Decimals of [FR-402](./FR-402-kani-tractable-native-decimal-carrier.md), the kernel SHALL evaluate the six Decimal operations and the four ordering comparisons with fixed-width native arithmetic, returning the typed outcome, the retained representation, the loss record and the ordered Meter charges that the owner's `evaluate_decimal` and `order_numbers` return for the same operands and target. The native API is a production facility available under ordinary builds, in the manner of [FR-384](./FR-384-admitted-native-integer-refinement.md); it is not a verification-only implementation, and it does not replace the owner's arbitrary-precision path, which is unchanged.

This is the native expression that CG FR-051 (prerequisite P1, ticket IR-1444) compares with the generated oracle. The owner's arithmetic, rounding and charge schedule ([FR-362](./FR-362-exact-scalar-arithmetic-and-atom-charges.md), QSpec FR-140) are the specification; this requirement adds no new arithmetic, charge point or refusal for any operand the native path admits. Where an operand or an intermediate does not fit the native width, the native path refuses admission and claims nothing (Behavior 2): it never narrows, wraps, saturates or approximates.

## Inputs

- A closed operation: `Add`, `Subtract`, `Multiply` or `Divide` over two carried Decimals (built from bindings, from an earlier result or from the total retained-pair constructor of FR-402 Behavior 3); `Negate` or `Round` over one. Orderings take an owner `OrderingOperator` (`lt`, `le`, `gt`, `ge`) and two carried Decimals.
- The native Decimal type of the result (arithmetic operations only; orderings have none) and the caller's `Meter`.

## Outputs

An admitted operation, or a typed capacity refusal; then the owner's `Outcome` over a native result (a carried Decimal retaining `(v x 10^T, T)` with `T` the type's `max_scale`, and the optional native loss record) or over a Boolean.

## Behavior

### Admission

1. Before any charge, the kernel SHALL admit an operation from its operands and type. The admission SHALL use only native arithmetic and no allocation, SHALL take no `Meter` and SHALL charge nothing. A refusal SHALL be a typed capacity refusal that names the quantity of Behavior 2 that does not fit, and SHALL NOT be an arithmetic refusal, a Meter denial, a completed value or a proof result.
2. The admission SHALL refuse exactly when at least one of the following exact integers, for the owner's evaluation plan of the same operation and operands, lies outside signed 128 bits: each operand coefficient shifted by the plan's power of ten; the aligned sum, difference, product or negation; the numerator and the denominator of the quotient; a power of ten by which the owner divides an intermediate to reach the target scale (formed only when the owner forms it); every integer that a nearest rounding mode compares, including twice the remainder; the numerator and the cofactor of the loss record; and the retained coefficient after the lift to `T`. For an ordering, the aligned magnitude of the operand of the smaller scale. Charge amounts are not among them. Every operand pair and type that fits SHALL be admitted. The capacity region is value-dependent and is stated by this list, not by a narrower fixed interval.

### Evaluation

3. For an admitted operation the kernel SHALL return the owner's typed outcome. The completed value's retained representation SHALL equal the owner's `(v x 10^T, T)`, not only its normalized value. `Undefined(DivisionByZero)` SHALL be returned for a divisor whose normalized coefficient is zero after the `decimal.operands` charge. Under `exact` rounding, `Refused(InexactDecimal)` SHALL be returned before any `decimal.rounding` charge or loss record whenever the exact result is not a multiple of `10^-T`. `Refused(DecimalOutOfDomain)` SHALL be returned when the placed result is not a member of the result type, including after rounding. No other refusal SHALL exist.
4. The six rounding spellings SHALL round as the owner rounds: toward zero, toward positive, toward negative, nearest even, nearest away, with `exact` refusing. A rounding step SHALL record a native loss record that reads out to the owner's `DecimalLoss` (exact numerator and denominator, rounded representation, mode).
5. The kernel SHALL charge the actual `Meter` with the owner's charge points, limit kinds, amounts and order (`decimal.operands`, `decimal.scale-expansion`, `decimal.arithmetic`, `decimal.rounding` on a rounding step only, `decimal.result-retain`; for orderings `ordering.operands`, `ordering.arithmetic`, `ordering.result-retain`), including cumulative work and results, cancellation and injected-denial precedence. A denied charge SHALL stop the operation with the owner's incomplete record and no later charge. Admission work SHALL NOT manufacture evaluator charges.
6. An ordering SHALL order by mathematical value, not by retained representation or raw coefficient.

### Shape and evidence

7. The admission, evaluation and ordering bodies on the proof path SHALL contain no loop, no iterator adapter and no allocation, SHALL NOT be selected by `cfg(kani)`, and SHALL create no `Integer`, `Decimal`, `Rational` or `Vec`. The result and the loss record SHALL be `Copy`. The read-out of a result SHALL yield an owner `DecimalResult` and MAY allocate outside the proof path.
8. The kernel SHALL carry a host differential. For every boundary operand and type of TC-951 the oracle SHALL be the owner route (`evaluate_decimal`, `order_numbers` over `Decimal::new` operands) run under a Meter with the same limits. The native path and the oracle SHALL agree on the typed outcome, the read-out `DecimalResult` under its `Eq`, the ordered charges, every counter and the denied next charge at sufficient, exact and one-less limits of each charge, with exactly one intended difference: an operation the native admission refuses, where the oracle returns whatever the owner decides, including a completed value.
9. The kernel SHALL carry, for each of `Add`, `Subtract`, `Multiply`, `Divide`, `Negate`, `Round` and the ordering, a Kani proof without a loop unwind bound that, over all symbolic carried operands and all symbolic native types within the native width, the admission decision, the outcome and the completed value and loss record equal an independent bounded mathematical reference that shares no code with the native path, with covers that the completed path, a rounding step, each refusal and a capacity refusal are reachable. A host test SHALL bind each reference to the owner route over the boundary set.
10. When CG's decimal row (CG FR-051, ticket IR-259) consumes the delivered operations, the memory and time of the composed proof under the 16 GiB per-request ceiling of CG FR-028 SHALL be measured and recorded under IR-1444 with the peak provenance and the terminal outcome. This requirement states no memory budget and claims no fit; a prediction is not a measurement.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-403-AC-1 | Admission is exact at its edges and precedes any charge. With a type `Decimal[-2^126, 2^126; 0, 0; Exact]`, `Add` of `(2^125, 0)` and `(2^125, 0)` is admitted and `Add` of `(2^126, 0)` and `(2^126, 0)` is a capacity refusal naming the sum (2^127 does not fit). `Multiply` of `(2^63, 0)` and `(2^63, 0)` (product `2^126`) is admitted and `Multiply` of `(2^64, 0)` and `(2^64, 0)` is a capacity refusal naming the product (`2^128`). `Add` of `(1, 0)` and `(1, 39)` is a capacity refusal naming the shifted operand (`10^39`) and is admitted for `(1, 0)` and `(1, 38)`. A `Meter` presented to a refused admission is unchanged. | Test (TC-951) |
| FR-403-AC-2 | Retained representation and values. In `Decimal[0,100; 0,2; Exact]`, `Add` of `(5, 1)` and `(5, 1)` completes with the retained pair `(100, 2)`, `Subtract` of `(5, 1)` and `(1, 1)` with `(40, 2)`, `Multiply` of `(5, 1)` and `(4, 1)` with `(20, 2)`, `Negate` of `(0, 0)` with `(0, 2)`, `Round` of `(5, 1)` with `(50, 2)`, and `Divide` of `(1, 0)` by `(4, 0)` with `(25, 2)`; none of these retained pairs is normalized. | Test (TC-951) |
| FR-403-AC-3 | Typed outcomes. `Divide` by `(0, 3)` is `Undefined(DivisionByZero)` after the `decimal.operands` charge and is `Incomplete` at `decimal.operands` under a zero budget. Under `exact`, `Divide` of `(1, 0)` by `(3, 0)` in `Decimal[0,100; 0,2]`, `Multiply` of `(1, 2)` and `(1, 2)` (operand scales sum above `T = 2`), and `Add` of `(1, 3)` and `(0, 0)` are `Refused(InexactDecimal)` with no `decimal.rounding` charge and no loss record. `Add` of `(100, 0)` and `(1, 0)` in `Decimal[0,100; 0,2]` is `Refused(DecimalOutOfDomain)`, and so is a result that rounding moves outside the domain (`Round` of `(1005, 1)` into `Decimal[0,100; 0,0]` under nearest-away is `101`; under toward-zero it is `100` and is completed). | Test (TC-951) |
| FR-403-AC-4 | Rounding. For each of the five rounding spellings other than `exact` over `Decimal[-100,100; 0,2]`, `Divide` of `(1, 0)` by `(8, 0)` (0.125, a tie), of `(-1, 0)` by `(8, 0)`, of `(3, 0)` by `(8, 0)` and `Round` of `(125, 3)` and `(-125, 3)` and `(135, 3)` yield the owner's retained pair and loss record (exact numerator and denominator, rounded pair, mode); nearest-even and nearest-away differ on `0.125` and on `-0.125`. | Test (TC-951) |
| FR-403-AC-5 | Charges. For `Add`, `Multiply`, `Divide`, `Round`, `Negate`, `Subtract` and each ordering, over operands with equal scales, unequal scales (a scale-expansion shift above zero) and a rounding step, at sufficient, exact and one-less limits of each of the owner's limit kinds, the native path and the owner agree on every charge point reached, its amount, the order, the denied next charge, the counters and the incomplete record; an operation that is stopped by a denied charge returns no value. | Test (TC-951) |
| FR-403-AC-6 | Ordering is by value. `lt` of `(10, 1)` (1.0) and `(2, 0)` is true, `le` of `(10, 1)` and `(1, 0)` is true, `gt` of `(10, 1)` and `(1, 0)` is false, `ge` of `(-1, 0)` and `(-10, 1)` is true, and the raw-coefficient order disagrees with the value order for the first pair. An ordering whose aligned operand does not fit is a capacity refusal (`(1, 0)` against `(1, 39)`). | Test (TC-951) |
| FR-403-AC-7 | The proof-path bodies are built for the host and for `cfg(kani)` from one body with no `cfg(kani)` token, no loop, iterator adapter or recursion, and none of `Integer`, `Decimal`, `Rational`, `Vec`, `Box` or `alloc` in the module; the result and the loss record are `Copy`; the counting allocator observes zero allocations around admission over the boundary set. | Test (TC-951) |
| FR-403-AC-8 | Over the boundary set of TC-951 the native path and the owner route agree as Behavior 8 states, with the single intended difference (capacity refusal) listed exhaustively and checked against its named quantity. A capacity refusal is never reported as a completed, undefined, refused or incomplete outcome, and an input the owner refuses for a domain reason that the native admission admits is refused by the native path with the same domain refusal. | Test (TC-951) |
| FR-403-AC-9 | For each of the six operations and the ordering, a Kani proof with no unwind bound verifies the refinement of Behavior 9, with its covers. A proof over one concrete type, one that does not complete, exhausts memory or lacks a cover is not credited. | Test (TC-951) |
| FR-403-AC-10 | When CG's decimal row (IR-259) consumes the delivered operations, the peak memory, time, tool version and terminal outcome of the composed proof on the six arithmetic operations and one ordering are recorded under IR-1444 with the harness run and the peak provenance. A fit or non-fit at 16 GiB is stated as measured. | Inspection (TC-951) |

## Status

Planned (IR-1444). No implementation, proof result or measurement exists; this specification claims none.

## Dependencies

- [FR-402](./FR-402-kani-tractable-native-decimal-carrier.md) owns the carrier, the type and the constructor. FR-362 and QSpec FR-140 own the arithmetic and the charge schedule that this requirement refines. FR-384 is the precedent for admission before charge and for the typed capacity refusal.
- CG FR-051 is the consumer; CG's consumption is CG's ticket IR-259 and CG's credit. CG FR-028 owns the 16 GiB ceiling.
- The metered `Meter` is the owner's and unchanged; its own allocation behaviour is outside this requirement.
