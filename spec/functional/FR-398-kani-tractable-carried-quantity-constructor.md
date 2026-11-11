---
id: FR-398
title: "A Kani-tractable carried-quantity constructor from physical bindings"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: references
  - target: ix://agent-ix/quire-exact/FR-384
    type: references
  - target: ix://agent-ix/quire-exact/FR-387
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-050
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-025
    type: references
---
# FR-398: A Kani-tractable carried-quantity constructor from physical bindings

## Description

The kernel SHALL provide a carried-quantity constructor that builds an owner carried quantity of a `CarriedQuantityType` from the physical bindings of that type's carrier, using only fixed-width native arithmetic, so that a generated Kani proof can draw the bindings symbolically and reach the quantity without arbitrary-precision allocation. The consumer is the quantity row of Contract CodeGen (FR-050, prerequisite P1, ticket IR-1443), which draws a carried-quantity leaf as the carrier's ordered `i64` bindings (CG FR-025-AC-18) and converts it to an exact `Quantity<u>`. The constructor is a production kernel facility available in ordinary host builds, in the manner of [FR-384](./FR-384-admitted-native-integer-refinement.md), and is not a verification-only replacement: its behaviour does not differ between a host build and a `cfg(kani)` build, so a host test of it is a test of what Kani proves. The existing `CarriedQuantityType` (FR-088 `UnitId`, `QuantityCarrier`) and its caller-performed membership are unchanged.

Three things are out of scope. Unit conversion, quantity arithmetic and comparison remain the owner's in `quire-semantic-value`. Decoding retained counterexample playback into bindings remains CG's (FR-025-AC-18, FR-050-AC-9). Choosing a smaller carrier domain so a proof fits is not done here: if the composed proof does not fit, the fallback is a QSpec finite-grid bound filed as its own ticket by the owner of IR-1443.

## Use case

A generated harness for `a + b`, each operand a `quire.op.quantity.convert` of an `Int[0, 100]` leaf in `cm`, calls `kani::any::<i64>()` once per leaf and constructs the carried quantity from that binding and the declared type. For `Decimal[0,100; 0,2]` the harness draws a coefficient then a scale, so `50` is the pair `(5000, 2)`, whose raw coefficient is outside `[0, 100]`. The constructor yields the exact magnitude `50`, and the owner's conversion takes it from there.

## Behavior

1. **Bindings.** The constructor takes a `CarriedQuantityType` and the carrier's physical bindings in the order CG FR-025-AC-18 defines: an `Int` carrier one `i64`; a `Decimal` carrier an `i64` coefficient then a `u32` scale; a `Rational` carrier an `i64` numerator then an `i64` denominator. It performs no allocation, creates no `Integer`, `Decimal` or `Rational`, has no loop whose bound depends on a binding, and takes no `Meter` and charges nothing. The magnitude storage of the result is `Copy` with no drop glue. The unit of the result is the type's `UnitId`; a unit is never a binding.
2. **Exact magnitude.** An `Int` binding `n` gives `n/1`. A `Decimal` binding `(c, s)` gives the exact value `c / 10^s`. A `Rational` binding `(n, d)` gives `n/d`. The magnitude is held as a canonical fraction (lowest terms, positive denominator, zero as `0/1`) of two native signed 128-bit integers, the width of [FR-384](./FR-384-admitted-native-integer-refinement.md)'s native carrier. A read-out yields the owner `Quantity` of that magnitude and unit, equal under `Eq` to `Quantity::new` of the owner's canonical `Rational` for the same value.
3. **Admission.** The constructor admits a binding when it is a member of the type's carrier by the owner's own membership: an `Int` binding by `IntegerInterval::contains`; a `Decimal` binding by `DecimalType::contains` of the pair as the owner defines it, which first removes trailing decimal zeros, so that a raw coefficient outside `[lo, hi]` is admitted when its normalized coefficient is inside; a `Rational` binding by `RationalDomain::contains` of the reduced value, and only when the pair is already canonical. The constructor does not normalize an input to make it admissible.
4. **Refusals.** The constructor returns a typed refusal and no quantity for each of these, and for no other reason: `CarrierMismatch` when the bindings are not of the type's carrier; `ZeroDenominator` for a `Rational` denominator of zero; `NonCanonical` for a `Rational` pair that is not in lowest terms, has a negative denominator, or is zero over a denominator other than one; `OutsideCarrierDomain` for a binding that is canonical but not a member; and `NotRepresentable` when the canonical numerator or denominator of an admitted `Decimal` does not fit the native width. `Int` and `Rational` bindings of `i64` always fit, so only `Decimal` can be `NotRepresentable`. The checks run in the order listed, and a refusal consumes nothing.
5. **Host differential.** For every carrier, each binding in the boundary set of the test case is built twice, once by the constructor and once by the shipped owner route (`Integer`, `Decimal::to_rational` or `Rational::new`, then `Quantity::new`, with `DecimalType::contains`, `IntegerInterval::contains` and `RationalDomain::contains` for membership). The magnitude, unit and admission decision agree. Where the owner has a reason the constructor has the same: `Rational::new`'s `ZeroDenominator`, and a failed owner membership is `OutsideCarrierDomain`. Where the owner normalizes (a `Rational` pair not in lowest terms, which `Rational::new` reduces) the constructor refuses `NonCanonical`; that is the only intended difference.
6. **No other source.** The constructor accepts no `BoundedQuantityType` (FR-387), no bare unit and no caller-supplied `Rational`, since none forms a finite binding domain.
7. **Proof.** For each carrier the kernel carries a Kani proof, without a loop unwind bound, that over all symbolic bindings the constructor's outcome and magnitude equal an independent bounded mathematical reference that shares no code with the constructor, and a cover that the success path is reachable. A host test binds that reference to the owner route over the boundary set, as the native-integer proofs do.
8. **Fit.** The memory and time of the composed proof (leaf construction, owner conversion and owner operation in one harness) under the 16 GiB per-request ceiling of CG FR-028 are measured in the code slice of IR-1443 by running CG's FR-050 harness against the delivered constructor, and recorded with the peak provenance and the terminal outcome. A configured ceiling or a prediction is not a measurement, and this requirement claims no fit.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-398-AC-1 | For an `Int[lo, hi]` carrier, bindings `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1` and `i64::MAX`, each inside a carrier that contains it, yield magnitude `n/1` in the type's unit, equal to the owner route; the endpoints `lo` and `hi` are admitted and `lo - 1` and `hi + 1` are `OutsideCarrierDomain` with no quantity. | Test (TC-946) |
| FR-398-AC-2 | For a `Decimal[lo, hi; smin, smax; mode]` carrier, `(c, s)` yields the exact canonical value `c / 10^s`: `(10, 1)` is `1/1`, `(-5, 1)` is `-1/2`, `(0, 7)` is `0/1`, and `(5000, 2)` in `Decimal[0,100; 0,2]` is admitted as `50/1` although its raw coefficient is outside `[0, 100]`. `(10001, 2)` and `(9999, 2)` in that carrier (the owner compares the normalized coefficient with `[lo, hi]`, and both exceed `100`) and a scale above `smax` are `OutsideCarrierDomain`. | Test (TC-946) |
| FR-398-AC-3 | For a `Rational[n1, n2; d1, d2]` carrier, a canonical `(n, d)` inside both ranges yields `n/d` in the type's unit with numerator and denominator in that order (`(1, 2)` is `1/2`, not `2/1`); `(2, 4)`, `(1, -2)`, `(-1, -2)` and `(0, 2)` are `NonCanonical`, `(1, 0)` is `ZeroDenominator`, and a canonical pair with a numerator or denominator one outside its range is `OutsideCarrierDomain`. | Test (TC-946) |
| FR-398-AC-4 | A `Decimal` binding whose canonical numerator or denominator exceeds the native 128-bit width is `NotRepresentable`: `(1, 38)` is admitted as `1/10^38`, `(1, 39)` and `(1, u32::MAX)` are `NotRepresentable`, and `(2^39, 39)`, whose canonical form is `1/5^39`, is admitted. The decision is by the canonical fraction, not by a fixed scale limit. | Test (TC-946) |
| FR-398-AC-5 | A binding of one carrier presented with a type of another (an `i64` pair against an `Int` type, a single `i64` against a `Decimal` or `Rational` type) is `CarrierMismatch`. For a binding that is both `NonCanonical` and outside the domain, `NonCanonical` is reported; for one that is both outside the domain and `NotRepresentable`, `OutsideCarrierDomain` is reported. | Test (TC-946) |
| FR-398-AC-6 | The constructor builds in an ordinary host build and in a `cfg(kani)` build from the same source, with no `cfg(kani)`-selected body. Its result is `Copy`, its construction performs zero heap allocations as observed by a counting allocator around the call, and it has no `Meter` parameter and leaves a `Meter` the caller holds unchanged. | Test (TC-946) |
| FR-398-AC-7 | Over the boundary set of TC-946, the constructor and the shipped owner route agree on admission, magnitude, unit and the refusal reasons of Behavior 5, and the read-out `Quantity` equals the owner's under `Eq`; the sole disagreement is the `NonCanonical` pairs that the owner normalizes. | Test (TC-946) |
| FR-398-AC-8 | The type system offers no way to build the carrier from a `BoundedQuantityType`, a bare `UnitId` or a `Rational`; a program that tries fails to compile. | Test (TC-946) |
| FR-398-AC-9 | For each of `Int`, `Decimal` and `Rational`, a Kani proof with no unwind bound verifies that, for all symbolic bindings, the constructor's outcome and magnitude equal the independent reference, with a success-reachable cover. A proof that does not complete, exhausts memory or lacks the cover is not credited. | Test (TC-946) |
| FR-398-AC-10 | The peak memory, time, tool version and terminal outcome of the composed convert-and-operation proof with the delivered constructor are recorded under IR-1443, naming the harness run and the peak provenance. A fit or a non-fit at 16 GiB is stated as measured; a non-fit is followed by a filed QSpec finite-grid ticket rather than a narrowed domain. | Inspection (TC-946) |

## Status

Planned (IR-1443). No implementation, proof result or measurement exists; this specification claims none.

## Dependencies

- FR-088 owns `UnitId`; FR-387 owns the bounded quantity type this constructor refuses; FR-384 is the native-width and host-availability precedent.
- CG FR-050 (merged, IR-1497) is the consumer and names this constructor as prerequisite P1; CG FR-025-AC-18 defines the binding order. CG consumption of the constructor is CG's ticket and credit.
- The sibling Kani carriers IR-1445 (text) and IR-1446 follow the same host-differential pattern and are not prerequisites.
- The conversion and operation owners (`convert_quantity`, `evaluate_quantity_unit`) are in `quire-semantic-value`, which depends on this crate; the composed proof of Behavior 8 therefore runs in CG's harness, not in this repository.
