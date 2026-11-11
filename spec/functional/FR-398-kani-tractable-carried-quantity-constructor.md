---
id: FR-398
title: "A Kani-tractable carried-quantity constructor from physical bindings"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: references
  - target: ix://agent-ix/quire-exact/FR-372
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-384
    type: references
  - target: ix://agent-ix/quire-exact/FR-387
    type: references
  - target: ix://agent-ix/quire-specification/FR-140
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
---
# FR-398: A Kani-tractable carried-quantity constructor from physical bindings

## Description

The kernel SHALL provide a carried-quantity constructor that builds a carried quantity of a `CarriedQuantityType` from the physical bindings of that type's carrier (Behavior 1 to 4).

The constructor SHALL use only fixed-width native arithmetic and no allocation, so that a generated Kani proof can draw the bindings symbolically and reach the quantity without arbitrary-precision work (Behavior 1 and 2). The consumer is the quantity row of Contract CodeGen (FR-050, prerequisite P1, ticket IR-1443), which draws a carried-quantity leaf as the carrier's ordered `i64` bindings (CG FR-025-AC-18) and converts it to an exact `Quantity<u>`.

The constructor SHALL be available in ordinary host builds, in the manner of [FR-384](./FR-384-admitted-native-integer-refinement.md), with one source for a host build and a `cfg(kani)` build (Behavior 1), so that a host test of it is a test of what Kani proves. It is not a verification-only replacement. The existing `CarriedQuantityType` (FR-088 `UnitId`, `QuantityCarrier`) is unchanged. Membership follows the owner's rules: [FR-372](./FR-372-fallible-bounded-decimal-membership.md) and QSpec FR-140 for Decimal, and the `IntegerInterval` and `RationalDomain` membership already shipped.

Three things are out of scope. Unit conversion, quantity arithmetic and comparison remain the owner's in `quire-semantic-value`. Decoding retained counterexample playback into bindings remains CG's (FR-025-AC-18, FR-050-AC-9). Choosing a smaller carrier domain so a proof fits is not done here: if the composed proof does not fit, the fallback is a QSpec finite-grid bound filed as its own ticket by the owner of IR-1443. The sibling carriers are IR-1445 (text), IR-1446 (float) and IR-1444 (decimal); this constructor does not introduce a second native decimal encoding for general Decimal values. It only maps a Decimal binding to the native fraction of Behavior 2. The IR-1444 carrier (FR-402) retains the Decimal pair and never forms that fraction; the two share only the owner membership rule.

## Use case

A generated harness for `a + b`, each operand a `quire.op.quantity.convert` of an `Int[0, 100]` leaf in `cm`, calls `kani::any::<i64>()` once per leaf and constructs the carried quantity from that binding and the declared type. For `Decimal[0,100; 0,2]` the harness draws a coefficient then a scale, so `50` is the pair `(5000, 2)`, whose raw coefficient is outside `[0, 100]`. The constructor yields the exact magnitude `50`, and the owner's conversion takes it from there.

## Behavior

### Construction

1. The constructor SHALL take a reference to a `CarriedQuantityType` and the carrier's physical bindings in the order CG FR-025-AC-18 defines: an `Int` carrier one `i64`; a `Decimal` carrier an `i64` coefficient then an `i64` scale, exactly as the harness draws them; a `Rational` carrier an `i64` numerator then an `i64` denominator. It SHALL perform no allocation, SHALL create no `Integer`, `Decimal` or `Rational`, SHALL contain no loop and no iterator adapter (IR-1436 D4: no iterator loops on Kani carriers), SHALL take no `Meter` and SHALL charge nothing. Its body SHALL NOT be selected by `cfg(kani)`.
2. The result SHALL hold the magnitude only, as a canonical fraction (lowest terms, positive denominator, zero as `0/1`) of two native signed 128-bit integers, the width of [FR-384](./FR-384-admitted-native-integer-refinement.md)'s native carrier. The result SHALL be `Copy` with no drop glue and SHALL carry no `UnitId`, which is not `Copy`. The unit stays with the `CarriedQuantityType` argument. A read-out that takes the same type by reference SHALL yield the owner `Quantity` of that magnitude and the type's unit, equal under `Eq` to `Quantity::new` of the owner's canonical `Rational` for the same value. The read-out may allocate, and its allocation is outside the constructor's zero-allocation window.
3. An `Int` binding `n` SHALL give `n/1`; a `Decimal` binding `(c, s)` SHALL give the exact value `c / 10^s`; a `Rational` binding `(n, d)` SHALL give `n/d`.

### Admission

4. The constructor SHALL admit a binding only when it is a member of the type's carrier by the owner's own membership: an `Int` binding by `IntegerInterval::contains`; a `Decimal` binding by `DecimalType::contains` of the pair as the owner defines it; a `Rational` binding by `RationalDomain::contains` of the reduced value, and only when the pair is already canonical. The constructor SHALL NOT normalize an input to make it admissible. The Decimal rule is: remove trailing decimal zeros to the normalized pair `(c', s')` (zero is `(0, 0)`); let `s*` be `max(s', smin)`; the binding is a member exactly when `s*` is at most `smax` and `lo <= c' x 10^(s* - s') <= hi`. A lifted coefficient whose magnitude exceeds the native width is compared by sign alone (it exceeds `hi` when positive and falls below `lo` when negative, since the bounds fit the native width by item 5), and is never materialized. A scale outside `0..=4294967295` is outside every carrier.
5. A `CarriedQuantityType` whose bound is not representable in the native width (an `Int` endpoint, a `Decimal` `lo` or `hi`, or a `Rational` numerator or denominator endpoint outside signed 128 bits) is not representable by this carrier, and the constructor SHALL return `TypeNotRepresentable` for every binding against it, distinct from `OutsideCarrierDomain` so that a replay can tell a non-member from an unsupported type. CG FR-050-AC-13 already refuses non-`i64` endpoints upstream, so this refusal is reachable only from a hand-built type.
6. If the bindings are not of the type's carrier, then the constructor SHALL return `CarrierMismatch`. If a `Rational` denominator is zero, then it SHALL return `ZeroDenominator`. If a `Rational` pair is not in lowest terms, has a negative denominator, or is zero over a denominator other than one, then it SHALL return `NonCanonical`; canonical form is judged on the raw pair before membership. If item 5 applies, then it SHALL return `TypeNotRepresentable` before any check of the binding. If a canonical binding is not a member, or its scale is outside `0..=4294967295`, then it SHALL return `OutsideCarrierDomain`. If the canonical numerator or denominator of an admitted `Decimal` does not fit the native width, then it SHALL return `NotRepresentable`. Each refusal SHALL carry no quantity, and no other refusal SHALL exist: the six causes are `CarrierMismatch`, `TypeNotRepresentable`, `ZeroDenominator`, `NonCanonical`, `OutsideCarrierDomain` and `NotRepresentable`. The checks run in the order `CarrierMismatch`, `TypeNotRepresentable`, `ZeroDenominator`, `NonCanonical`, `OutsideCarrierDomain`, `NotRepresentable`. `Int` and `Rational` bindings of `i64` always fit, so only `Decimal` can be `NotRepresentable`.
7. The constructor SHALL accept no `BoundedQuantityType` (FR-387), no bare unit and no caller-supplied `Rational`, since none forms a finite binding domain.

### Evidence

8. The kernel SHALL carry a host differential. For every boundary binding of TC-946, the oracle SHALL be the shipped owner route (`Integer`, `Decimal::to_rational` or `Rational::new`, then `Quantity::new`, with `DecimalType::contains`, `IntegerInterval::contains` and `RationalDomain::contains` for membership) wherever that route can be run, and an independent reference formula wherever it cannot. The owner route cannot be run for a `Decimal` whose scale makes `Decimal::to_rational` build a power of ten too large to hold (scale `4294967295`), so the reference there derives the canonical fraction and its fit from the exponents of 2 and 5 in the coefficient, without building `10^s`; `DecimalType::contains` is still run. The constructor and the oracle SHALL agree on magnitude, unit and admission with exactly three intended differences: a `Rational` pair that is not canonical and that the owner normalizes (`NonCanonical`), a `Decimal` that the owner admits and the native width cannot hold (`NotRepresentable`), and any binding of the type's own carrier against a type with a bound outside the native width, whatever the owner decides for that binding (`TypeNotRepresentable`, item 5). A binding of another carrier than the type's has no owner-route result; it is checked only against its literal `CarrierMismatch` and sits outside the differential, so it is neither an agreement nor a difference.
9. For each carrier the kernel SHALL carry a Kani proof, without a loop unwind bound, that over all symbolic bindings and all symbolic type bounds within the native width (including `smin` and `smax`, so that the lift of item 4 is exercised) the constructor's outcome and magnitude equal an independent bounded mathematical reference that shares no code with the constructor, with a cover that the success path is reachable. A host test SHALL bind that reference to the owner route over the boundary set.
10. When CG's FR-050 composed harness (ticket IR-263, the MVP quantity row; specification IR-1439) consumes the delivered constructor, the memory and time of the composed proof (leaf construction, owner conversion and owner operation in one harness) under the 16 GiB per-request ceiling of CG FR-028 SHALL be measured and recorded under IR-1443 with the peak provenance and the terminal outcome. A configured ceiling or a prediction is not a measurement, and this requirement claims no fit.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-398-AC-1 | For `Int[i64::MIN, i64::MAX]`, bindings `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1` and `i64::MAX` yield magnitude `n/1`, equal to the owner route. For `Int[-5, 100]`, `-5` and `100` are admitted and `-6` and `101` are `OutsideCarrierDomain` with no quantity. | Test (TC-946) |
| FR-398-AC-2 | For a `Decimal[lo, hi; smin, smax; mode]` carrier, `(c, s)` yields the exact canonical value `c / 10^s`: `(10, 1)` is `1/1`, `(-5, 1)` is `-1/2`, `(0, 7)` is `0/1` (its normalized scale is 0, so it is admitted even when `smax` is below 7), and `(5000, 2)` in `Decimal[0,100; 0,2]` is admitted as `50/1` although its raw coefficient is outside `[0, 100]`. Membership is judged on the normalized pair: `(10, 3)` in `Decimal[0,100; 0,2]` is admitted as `1/100`; `(1, 3)`, `(10001, 2)` and `(9999, 2)` there are `OutsideCarrierDomain`. In `Decimal[0,100; 2,3]`, which lifts to `smin`, `(1, 0)` is admitted as `1/1` and `(2, 0)` is `OutsideCarrierDomain` (lifted coefficient 200). In `Decimal[-1000,1000; 0,2]`, `lo` and `hi` are admitted and `lo - 1` and `hi + 1` are `OutsideCarrierDomain`. In `Decimal[0,100; 60,70]` and `Decimal[0,100; 128,130]`, whose lifts exceed the native width (the second wraps to zero modulo 2^128), `(1, 0)` is `OutsideCarrierDomain` by sign, `(-1, 0)` is `OutsideCarrierDomain` in `Decimal[-100,0; 60,70]`, and `(0, 0)` is admitted as `0/1` in the first. The binding `(1, 4294967296)` in `Decimal[-1000,1000; 0,2]` and `(1, -1)` in `Decimal[0, 549755813888; 0, 4294967295]` are `OutsideCarrierDomain`. | Test (TC-946) |
| FR-398-AC-3 | For a `Rational[n1, n2; d1, d2]` carrier, a canonical `(n, d)` inside both ranges yields `n/d` in that order (`(1, 2)` is `1/2`, not `2/1`). `(2, 4)`, `(1, -2)`, `(-1, -2)` and `(0, 2)` are `NonCanonical`, `(1, 0)` and `(0, 0)` are `ZeroDenominator`. In `Rational[2, 5; 3, 7]` each bound is violated alone by a canonical pair: numerator below `n1` by `(1, 4)`, above `n2` by `(6, 7)`, denominator below `d1` by `(3, 2)`, above `d2` by `(2, 9)`; and the four corners `(2, 3)`, `(2, 7)`, `(5, 3)` and `(5, 7)` are admitted. | Test (TC-946) |
| FR-398-AC-4 | A `Decimal` binding whose canonical numerator or denominator exceeds the native width is `NotRepresentable`: in a carrier that admits it, `(1, 38)` is admitted as `1/10^38`, `(1, 39)` and `(1, 4294967295)` are `NotRepresentable`, and `(2^39, 39)`, whose canonical form is `1/5^39`, is admitted. The decision is by the canonical fraction, not by a fixed scale limit. | Test (TC-946) |
| FR-398-AC-5 | A binding of one carrier presented with a type of another is `CarrierMismatch`, for each of the six ordered pairs: an `Int` binding against a `Decimal` type and a `Rational` type; a `Decimal` binding against an `Int` type and a `Rational` type; a `Rational` binding against an `Int` type and a `Decimal` type. For the precedence of item 6: a `Decimal` binding `(1, 0)` against `Int[0, 2^127]` is `CarrierMismatch`, not `TypeNotRepresentable`; `Rational[1,5; 2,4]` `(6, 10)` (not canonical, and outside the ranges by its raw and its reduced pair) is `NonCanonical`; `Decimal[0,1; 0,2]` `(1, 39)` (outside the domain and not representable) is `OutsideCarrierDomain`. | Test (TC-946) |
| FR-398-AC-6 | A type with a bound outside the native width (`Int[0, 2^127]` and `Int[0, 2^128 + 1]`, a `Decimal` with `hi = 2^127` or `2^128 + 1`, a `Rational` with `d2 = 2^127` or `2^128 + 1`) is refused for every binding with `TypeNotRepresentable` (not `OutsideCarrierDomain`), including a binding that the owner's membership admits and a binding that would otherwise be `ZeroDenominator` or `NonCanonical`. | Test (TC-946) |
| FR-398-AC-7 | The constructor is built for the host and for `cfg(kani)` from one body, with no `cfg(kani)` token in it, no `for`, `while`, `loop` or iterator adapter, and no `Meter` in its signature (a function-pointer coercion of the constructor to a `Meter`-free signature compiles). Its result is `Copy` and holds no `UnitId`, and its construction performs zero heap allocations as observed by a counting allocator around the call (the read-out is outside that window). | Test (TC-946) |
| FR-398-AC-8 | Over the boundary set of TC-946, the constructor and the oracle of item 8 agree on admission, magnitude and unit, and the read-out `Quantity` equals the owner's under `Eq`. Bindings of another carrier than the type's are checked only against their literal `CarrierMismatch` and are outside this comparison. Among the rest, the only disagreements are the three intended differences of item 8, each of which the case lists exhaustively and checks against its named refusal. | Test (TC-946) |
| FR-398-AC-9 | The type system offers no way to build the carrier from a `BoundedQuantityType`, a bare `UnitId` or a `Rational`; each such program fails to compile with `E0308`. | Test (TC-946) |
| FR-398-AC-10 | For each of `Int`, `Decimal` and `Rational`, a Kani proof with no unwind bound verifies, over all symbolic bindings and all symbolic native-width type bounds, that the constructor equals the independent reference, with a success-reachable cover. A proof over one concrete type, one that does not complete, exhausts memory or lacks the cover is not credited. | Test (TC-946) |
| FR-398-AC-11 | When CG's FR-050 composed harness (IR-263) consumes the delivered constructor, the peak memory, time, tool version and terminal outcome of the composed convert-and-operation proof on an `Int`, a `Decimal` and a `Rational` leaf are recorded under IR-1443 with the harness run and the peak provenance. A fit or a non-fit at 16 GiB is stated as measured; a non-fit is followed by a filed QSpec finite-grid ticket rather than a narrowed domain. | Inspection (TC-946) |

## Status

Planned (IR-1443). No implementation, proof result or measurement exists; this specification claims none.

## Dependencies

- FR-088 owns `UnitId`; FR-387 owns the bounded quantity type this constructor refuses; FR-384 is the native-width and host-availability precedent; FR-372 and QSpec FR-140 own Decimal membership, on which this constructor depends.
- CG FR-050 (merged, IR-1497) is the consumer and names this constructor as prerequisite P1; CG FR-025-AC-18 defines the binding order, CG FR-028 the 16 GiB ceiling. CG's consumption of the constructor is CG's ticket IR-263 and CG's credit.
- IR-1445 (text), IR-1446 (float) and IR-1444 (decimal) are sibling carriers following the same host-differential pattern and are not prerequisites. IR-1444 (FR-402) retains the Decimal pair rather than the fraction; it and this constructor share the owner Decimal membership rule and must not diverge in it.
- The conversion and operation owners (`convert_quantity`, `evaluate_quantity_unit`) are in `quire-semantic-value`, which depends on this crate; the composed proof of item 10 therefore runs in CG's harness, not in this repository.
