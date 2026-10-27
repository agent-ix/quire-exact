---
id: FR-387
title: "A bounded quantity value type over a unit"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: references
  - target: ix://agent-ix/quire-exact/FR-262
    type: references
  - target: ix://agent-ix/quire-exact/FR-386
    type: references
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-230
    type: references
---
# FR-387: A bounded quantity value type over a unit

## Description

The `quire-exact` kernel SHALL provide a value type `Quantity<U>[min, max]` as the variant `ValueType::BoundedQuantity` holding a `BoundedQuantityType`. The source of the type is QSpec FR-154 and FR-154-AC-31, which define `Quantity<u>[min, max]` as a refinement of `Quantity<u>` with exact `Rational` magnitudes and no implicit unit conversion; QSL FR-230-AC-26 (min/max on a unit alias) is its consumer. The type carries a unit `U` (a `UnitId`, FR-088) and an optional inclusive lower bound and an optional inclusive upper bound on the magnitude, each bound a `Quantity` in `U`. The bounds are the folded minimum and folded maximum (the bounds of Behavior 2), the greatest `min` and least `max` of the alias chain: the caller (the QSL checker) folds the constraints of an alias and its parents into one folded `min` and one folded `max` before construction, and the kernel never sees or merges a constraint chain. The existing `ValueType::Quantity(UnitId)` is unchanged and is the type with no bounds. The type admits a quantity value in `U` whose magnitude lies within the bounds and refuses every other value, so that QSL can read a unit alias with `min`/`max` (QSL FR-230-AC-26) as a domain of bounded quantities of that unit. Quantity values, arithmetic and comparison are unchanged; this requirement adds type identity and admission only.

## Use case

A package unit alias `Level` over unit `percent` is declared with `min: 0` and `max: 100`. The checker builds `ValueType::bounded_quantity(BoundedQuantityType::new(percent, Some(zero), Some(hundred))?)`, asks whether `50 percent`, `101 percent`, `-1 percent` and `50 metres` are in the domain, and compares the type with the same declaration read elsewhere.

## Inputs

- A `UnitId` `U`, an optional `Quantity` lower bound and an optional `Quantity` upper bound.

## Outputs

- `Ok(BoundedQuantityType)` from `BoundedQuantityType::new(unit, min, max)`, or `Err(IllTyped)` with cause `MalformedBoundedQuantityType`. `unit()`, `min()` and `max()` return the parts. `ValueType::bounded_quantity(t)` returns `ValueType::BoundedQuantity(Arc<BoundedQuantityType>)`.

## Behavior

1. **Bound validity.** `MalformedBoundedQuantityType` is only the refusal of an invalid direct construction. `BoundedQuantityType::new` refuses with it when a present bound is a quantity in a unit other than `U`, or when both bounds are present and `min > max` by exact rational magnitude comparison. `min == max` is valid and admits exactly one magnitude. A bound is exact: the magnitude is a `Rational`, so no scale or rounding applies to a bound; a caller holding a decimal or other-unit bound converts it exactly before construction. This cause does not replace and is not a substitute for any other refusal: QSpec intake's `invalid_model_binding` for a malformed declaration (FR-154-AC-29), the checker's `ill_typed`/`type-mismatch` for a literal or expression outside the bounds or in another unit (FR-154-AC-31), and the run-time invariant refusal (FR-154-AC-28) stay the caller's and are not raised by the kernel constructor.
1a. **Refinement.** `BoundedQuantity` over `U` refines `Quantity(U)`: every value it admits is a quantity of `U`, which `Quantity(U)` admits. The kernel records type identity only and implements no subtype, flow or assignability relation between the two or between two bounded types; the QSL checker owns subtype and flow decisions (FR-154-AC-31) and uses `admits` as the run-time predicate. Type identity is unit plus effective minimum plus effective maximum.
2. **Type identity.** Two bounded quantity types are the same type exactly when their units are equal, their effective minimums are equal as magnitudes and their effective maximums are equal as magnitudes. A refinement relation between two different types never makes them equal. Bound presence is part of the identity: `Quantity<U>[None, None]` as a `BoundedQuantity` is a different type from `ValueType::Quantity(U)`, and `[0, None]` differs from `[None, 100]`. A bounded quantity type is unequal to `Quantity(U)`, to `Decimal` and `Rational` types, and to `Option` of either.
3. **Equality and hash.** Equality of `ValueType` and its `Hash` follow one rule: equal types hash equal. The unit and both bounds take part in both. Equal magnitudes in different `Rational` spellings that normalise to the same value are equal.
4. **Nesting.** A bounded quantity type nests in `Option`, `Collection` and `Map` links in any order and shares its `BoundedQuantityType` by reference count (FR-262).
5. **Deep types.** Clone, equality, hash, `Debug` and drop of a `ValueType` nested 100,000 levels deep through `Option` links around a bounded quantity type run on a 512 KiB stack with no recursion proportional to depth.
6. **Debug.** `Debug` prints the type as `BoundedQuantity(BoundedQuantityType { .. })` with fields `unit`, `min`, `max`, in compact and alternate mode, as `#[derive(Debug)]` prints.
7. **Admission.** `ValueType::admits` at a `BoundedQuantity` type is the run-time predicate of QSpec FR-154-AC-28. It returns `true` exactly for a `Value::Quantity` whose unit equals `U` and whose magnitude `m` satisfies `min <= m` when `min` is present and `m <= max` when `max` is present, so an in-range magnitude (bounds included) is admitted (`true`). It returns `false` for a quantity with `m < min`, a quantity with `m > max`, a quantity in another unit, and any non-quantity value; a refused value is reported as `false`, never as an accept and never as an error. It never returns `true` for a value outside the effective bounds, and a type with a present bound never behaves as the unbounded `Quantity(U)`. `ValueType::Quantity(U)` admission is unchanged.
8. **Refusal.** Constructor refusal of an invalid direct construction is the only kernel refusal this requirement adds; admission returns `false` and does not raise. Intake, checker and run-time-invariant refusals are the caller's (Behavior 1).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-387-AC-1 | `BoundedQuantityType::new(U, min, max)` returns a type whose `unit()`, `min()` and `max()` equal the arguments, for bounds `None`/`None`, `Some`/`None`, `None`/`Some` and `Some`/`Some`; `ValueType::bounded_quantity` wraps it as `ValueType::BoundedQuantity`. | Test (TC-936) |
| FR-387-AC-2 | `new` returns `Err` with cause `MalformedBoundedQuantityType` for `min` 5, `max` 3 in `U`, for a `min` quantity in a unit other than `U`, and for a `max` quantity in a unit other than `U`; it returns `Ok` for `min` equal to `max` and for `min` 3, `max` 5, including fractional magnitudes such as `1/3` and `1/2`. | Test (TC-936) |
| FR-387-AC-3 | Two bounded quantity types are equal exactly when unit, minimum and maximum are all equal, and equal types hash equal. Types differing only in unit, only in minimum, only in maximum, and only in bound presence (`[None, None]` against `[0, 100]`; `[0, None]` against `[None, 100]`) are unequal. A bounded quantity type with `[None, None]` is unequal to `ValueType::Quantity(U)`, and every bounded type is unequal to `Option` of itself. | Test (TC-936) |
| FR-387-AC-4 | A bounded quantity type nested in `Option`, `Collection` and `Map` compares by its whole chain: two chains differing only in the innermost bound are unequal and equal chains hash equal. | Test (TC-936) |
| FR-387-AC-5 | On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option` links around a bounded quantity type clones, compares equal to its clone, hashes equal to its clone, formats for debug with balanced brackets, and drops; two chains differing only in the innermost bound are unequal. | Test (TC-936) |
| FR-387-AC-6 | `Debug` of a bounded quantity type over unit `U` with `min` 0 and `max` 100 prints exactly what `#[derive(Debug)]` prints for `BoundedQuantity(BoundedQuantityType { unit: .., min: Some(..), max: Some(..) })`, and `{:#?}` prints the four-space nested form with a trailing comma on every field. The unbounded `[None, None]` type prints `min: None` and `max: None`. | Test (TC-936) |
| FR-387-AC-7 | At a `BoundedQuantity` type over `U` with effective bounds `[0, 100]`, `admits` returns `true` (admitted) for in-range magnitudes: for quantities of `U` with magnitudes 0, 50 and 100 (both bounds inclusive) and `false` (not admitted) for out-of-range magnitudes -1 and 101 and for `1/2` below a `[1, 100]` type. For `[0, None]` it returns `true` for 1000000 and `false` for -1; for `[None, 100]` it returns `true` for -1000000 and `false` for 101. | Test (TC-936) |
| FR-387-AC-9 | `BoundedQuantityType::new` is the only constructor refusal: it raises no `invalid_model_binding`, `ill_typed`/`type-mismatch` or run-time invariant cause. Its sole cause is `MalformedBoundedQuantityType`, raised only for `min > max` or a bound in a unit other than `U`; a type built from valid effective bounds reports the same `unit()`, `min()` and `max()` whatever alias chain the caller folded them from, and is unequal to a type with different effective bounds even where one refines the other. | Test (TC-936) |
| FR-387-AC-8 | At a `BoundedQuantity` type over `U`, `admits` returns `false` for a quantity of magnitude 50 in a different unit, for a `Text` value, a `Boolean` value and an `Option` value; `ValueType::Quantity(U)` still returns `true` for magnitude 101 of `U`, so the bounded type does not behave as the unbounded one. | Test (TC-936) |

## Status

Proposed.

## Dependencies

- Unit identity: [FR-088](./FR-088-enum-and-unit-identity-in-the-kernel.md). Deep types: [FR-262](./FR-262-deep-values-and-types-in-the-kernel.md). Sibling type-identity requirement: [FR-386](./FR-386-map-value-type-with-text-keys.md).
- The source is QSpec [FR-154](ix://agent-ix/quire-specification/FR-154) and FR-154-AC-31 (`Quantity<u>[min, max]`, refinement of `Quantity<u>`, run-time invariant FR-154-AC-28); the consumer is QSL FR-230-AC-26 (min/max on a unit alias). The kernel holds no alias, name or unit semantics beyond the type's own identity.
- The falsifying controls are [TC-936](../test-cases/TC-936-bounded-quantity-value-type.md).
