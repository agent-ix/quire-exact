---
id: TC-926
title: "Check caller-admitted collection elements and Option payloads"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-378
    type: verifies
---
# TC-926: Check caller-admitted collection elements and Option payloads

## Description

Verify [FR-378](../functional/FR-378-caller-admitted-collection-elements-and-option-payloads.md) through the exact and admitting collection constructors and the two `Option` constructors. Reuse the fixtures of [TC-924](./TC-924-composite-equality-across-declarations.md) (`p100`, `c100`, `g100`, `P = [1; 32]`, `C = [2; 32]`). The test admission accepts a composite when its declaration key is `P`, `C` or `G` at the element type `Composite(P)`, and otherwise defers to `ValueType::admits`. It records each call. The collection types have element type `Composite(P)`, no cardinality bound unless stated, and one of each kind: `Sequence`, `Set`, `Bag`, `OrderedSet`. The meter is a test meter with a recorded charge log.

## Test Procedure

1. For each collection kind, build `[p100, c100]` through the admitting occurrence form and the admitting deferred form. Record the elements, the declaration key and slots of each, and the length.
2. For each kind, build `[p100, c100]` through the exact `form_collection` and the exact `construct_collection`. Call `ValueType::Composite(P).admits(c100)`.
3. Build `[p100, c100, g100, p200]` with an admission that rejects `g100`, through the admitting deferred form and the admitting occurrence form. Record the admission call log and the meter before and after.
4. For each kind and each of `[p100, p200]`, `[p100, c100]` and `[p100, p100, p200]`, build through the exact path (where its element test passes) and through the admitting path with the same admission and compare the full charge logs, the totals and the resulting visiting order. Add a `Set` with a cardinality bound of at most 1 over `[p100, c100]` and record the result.
5. Call `OptionValue::from_admitted(Composite(P), Some(c100))`, `OptionValue::present(Composite(P), c100)`, `OptionValue::present(Composite(P), p100)` and `OptionValue::from_admitted(Composite(P), None)`. Read `payload_type()` and `payload()`.
6. Take the collection of step 1 for a `Set`. Clone it, run `planned_equality` against an identical independent build, against one built from `[p100, g100]`, and drop it. Call `ValueType::admits` of a `Collection` type of element `Composite(C)` on the step-1 `Set` of element `Composite(P)`, and the reverse.

## Expected Results

1. All eight builds complete. Each keeps `c100` with declaration `C` and its slots, and `p100` with declaration `P`. The `Set`, `Bag` and `OrderedSet` each hold two elements ordered `p100`, `c100` (FR-377); the `Sequence` keeps supply order. The admission was called once per element, in supply order.
2. The exact paths refuse: the deferred path with `CheckedInvariant { cause: CollectionElementNotAdmitted }`, the occurrence path with `ConstructionRefusal { component: Element(1), cause: TypeMismatch }`. `ValueType::Composite(P).admits(c100)` is `false`.
3. Both admitting paths refuse as step 2 does for the deferred path and the occurrence path respectively, naming element index 2 on the occurrence path. The admission log holds three calls on the deferred path (the fourth element is never reached) and three on the occurrence path. The meter is unchanged on the occurrence path.
4. For members both paths accept (`[p100, p200]` and `[p100, p100, p200]` built with an admission that is the exact test), the two charge logs and totals are identical. For `[p100, c100]` the admitting path's log has the same charge kinds as a two-distinct-member build of the exact path. The `Set` bounded at 1 over `[p100, c100]` reports the cardinality violation of the exact path.
5. `from_admitted` returns an `Option` with `payload_type() == Composite(P)` and a payload of declaration `C`. `present(Composite(P), c100)` refuses `ConstructionRefusal { component: Payload, cause: TypeMismatch }`. `present(Composite(P), p100)` completes. `from_admitted(..., None)` is `none` of `Option<Composite(P)>`.
6. The clone equals the original (`true`), the build from `[p100, g100]` gives `false`, and the drop completes. Neither collection element type is admitted by `ValueType::admits` at the other element type, so no covariance exists.
