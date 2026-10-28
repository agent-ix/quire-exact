---
id: SR-16666
title: IR-1051 PR 92 code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; src/comparison.rs; src/lib.rs; src/quantity.rs; src/value.rs; src/value/value_type.rs;
  tests/ir1051_bounded_quantity_type.rs
review_set: subset
---

# SR-16666: IR-1051 PR 92 code-review

## Summary

Independent code-review of frozen head e8475a07778cafab8667a46ef84224a49b011936 against main 1e984c13902c689c40b1f46c6fc71ba8d68769dc. Ticket IR-1051. Model gpt-6.1-sol; run def077df-4ac3-4029-a68f-bc616d4841f2. No applicable AssuranceProfile found.

## Verdict

PASS for the PR diff; no findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Rust lane reviewed the six changed files: constructor validates both bound units; inverted ranges remain distinct empty types; admission is an inclusive exact Rational comparison; ValueType clone/equality/hash/debug/drop include the new leaf. Tests bind all ten ACs and TC-936 through the public API. No copied code, unsafe change, CI narrowing, or applicable AssuranceProfile. Reviewer ran no Cargo build; final aggregate gate belongs to the code lead.

## Review data

```yaml
clean: true
scope:
- id: FR-387-AC-1
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: '`BoundedQuantityType::new(U, min, max)` returns a type whose `unit()`, `min()` and `max()` equal the
    arguments, for bounds `None`/`None`, `Some`/`None`, `None`/`Some` and `Some`/`Some`; `ValueType::bounded_quantity`
    wraps it as `ValueType::BoundedQuantity`.'
- id: FR-387-AC-2
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: '`new` returns `Err` with cause `MalformedBoundedQuantityType` for a `min` quantity in a unit other than
    `U` and for a `max` quantity in a unit other than `U`, and for no other input; it returns `Ok` for `min` equal
    to `max`, for `min` 3, `max` 5, and for inverted bounds `min` 5, `max` 3 (and `min` 4, `max` 2), which are valid
    empty types whose accessors return the arguments, including fractional magnitudes such as `1/3` and `1/2`.'
- id: FR-387-AC-3
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: Two bounded quantity types are equal exactly when unit, minimum and maximum are all equal, and equal
    types hash equal. Types differing only in unit, only in minimum, only in maximum, and only in bound presence
    (`[None, None]` against `[0, 100]`; `[0, None]` against `[None, 100]`) are unequal. A bounded quantity type
    with `[None, None]` is unequal to `ValueType::Quantity(U)`, and every bounded type is unequal to `Option` of
    itself.
- id: FR-387-AC-4
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: 'A bounded quantity type nested in `Option`, `Collection` and `Map` compares by its whole chain: two
    chains differing only in the innermost bound are unequal and equal chains hash equal.'
- id: FR-387-AC-5
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option` links around a bounded quantity
    type clones, compares equal to its clone, hashes equal to its clone, formats for debug with balanced brackets,
    and drops; two chains differing only in the innermost bound are unequal.
- id: FR-387-AC-6
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: '`Debug` of a bounded quantity type over unit `U` with `min` 0 and `max` 100 prints exactly what `#[derive(Debug)]`
    prints for `BoundedQuantity(BoundedQuantityType { unit: .., min: Some(..), max: Some(..) })`, and `{:#?}` prints
    the four-space nested form with a trailing comma on every field. The unbounded `[None, None]` type prints `min:
    None` and `max: None`.'
- id: FR-387-AC-7
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: 'At a `BoundedQuantity` type over `U` with effective bounds `[0, 100]`, `admits` returns `true` (admitted)
    for in-range magnitudes: for quantities of `U` with magnitudes 0, 50 and 100 (both bounds inclusive) and `false`
    (not admitted) for out-of-range magnitudes -1 and 101 and for `1/2` below a `[1, 100]` type. For `[0, None]`
    it returns `true` for 1000000 and `false` for -1; for `[None, 100]` it returns `true` for -1000000 and `false`
    for 101.'
- id: FR-387-AC-9
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: '`BoundedQuantityType::new` is the only constructor refusal: it raises no `invalid_model_binding`, `ill_typed`/`type-mismatch`
    or run-time invariant cause. Its sole cause is `MalformedBoundedQuantityType`, raised only for a bound in a
    unit other than `U`, never for inverted bounds, and the QSpec intake refusal of contradictory same-`appliesTo`
    constraints is QSpec''s, not the kernel''s; a type built from valid effective bounds reports the same `unit()`,
    `min()` and `max()` whatever alias chain the caller folded them from, and is unequal to a type with different
    effective bounds even where one ref'
- id: FR-387-AC-10
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: 'A `BoundedQuantity` type over `U` with `min` 5 and `max` 3 is constructed and `admits` returns `false`
    for quantities of `U` with magnitudes 2, 3, 4, 5 and 100, for a magnitude 4 quantity in another unit, and for
    a `Text` value, a `Boolean` value and an `Option` value. It equals a second type built from the same unit and
    bounds and hashes equal to it, it is unequal to the `[4, 2]` type over `U`, `Debug` of it prints `min: Some(..)`
    and `max: Some(..)` retaining both bounds, and `min()` and `max()` return 5 and 3.'
- id: FR-387-AC-8
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: examined
  excerpt: At a `BoundedQuantity` type over `U`, `admits` returns `false` for a quantity of magnitude 50 in a different
    unit, for a `Text` value, a `Boolean` value and an `Option` value; `ValueType::Quantity(U)` still returns `true`
    for magnitude 101 of `U`, so the bounded type does not behave as the unbounded one.
- id: TC-936
  path: spec/test-cases/TC-936-bounded-quantity-value-type.md
  role: examined
  excerpt: Verify FR-387 through the public BoundedQuantityType::new, ValueType::bounded_quantity, ValueType::admits
    and ValueType traits; scope FR-387-AC-1 through AC-10.
findings: []
bindings:
- test_id: a_bounded_quantity_type_reports_its_unit_and_bounds
  ac_id: FR-387-AC-1
  trace: correct
- test_id: only_a_bound_in_another_unit_is_refused
  ac_id: FR-387-AC-2
  trace: correct
- test_id: bounded_quantity_types_are_equal_exactly_when_unit_and_bounds_are_equal
  ac_id: FR-387-AC-3
  trace: correct
- test_id: a_bounded_quantity_type_compares_by_its_whole_nested_chain
  ac_id: FR-387-AC-4
  trace: correct
- test_id: deep_bounded_quantity_types_clone_compare_hash_format_and_drop
  ac_id: FR-387-AC-5
  trace: correct
- test_id: a_bounded_quantity_type_debug_prints_the_derived_layout
  ac_id: FR-387-AC-6
  trace: correct
- test_id: a_bounded_quantity_type_alternate_debug_prints_the_nested_layout
  ac_id: FR-387-AC-6
  trace: correct
- test_id: admission_is_the_inclusive_range_check
  ac_id: FR-387-AC-7
  trace: correct
- test_id: identity_is_the_effective_bounds_whatever_the_caller_folded_them_from
  ac_id: FR-387-AC-9
  trace: correct
- test_id: an_inverted_type_is_a_valid_empty_type_that_keeps_both_bounds
  ac_id: FR-387-AC-10
  trace: correct
- test_id: admission_refuses_other_units_and_non_quantities_unlike_the_unbounded_type
  ac_id: FR-387-AC-8
  trace: correct
```
