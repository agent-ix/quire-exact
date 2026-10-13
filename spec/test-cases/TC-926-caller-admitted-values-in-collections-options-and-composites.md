---
id: TC-926
title: "Check caller-admitted values in collections, Options and composites"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-378
    type: verifies
---
# TC-926: Check caller-admitted values in collections, Options and composites

## Description

Verify [FR-378](../functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md) through `form`, `form_grouped`, `OptionValue::from_admitted`, `from_admitted_slots` and `retain_composite`, and through the exact paths that stay exact. Reuse the fixtures of [TC-924](./TC-924-composite-equality-across-declarations.md) (`p100`, `c100`, `P = [1; 32]`, `C = [2; 32]`). The collection types have element type `Composite(P)` and no cardinality bound. The meter is a test meter with a recorded charge log. A record declaration `R = [5; 32]` has one field of declared type `Composite(P)`.

## Test Procedure

1. For each kind `Sequence`, `Set`, `Bag`, `OrderedSet`, call `form` and then `form_grouped` over `[p100, c100]`. Record the outcome, the declaration key and slots of each member, and the length.
2. On a fresh meter call `form` for a `Set` over `[c100, p100]` and record the charge log.
3. Call `OptionValue::from_admitted(Composite(P), Some(c100))` and read `payload_type()` and `payload()`.
4. Call `from_admitted_slots(R, [Present(c100)])` and then `retain_composite` on a fresh meter. Record the slot, the charge log, and `planned_equality` against `from_admitted_slots(R, [Present(p100)])`.
5. With `c100` at element type `Composite(P)`, call `construct_collection` with a deferred element, `form_collection`, `ValueType::Composite(P).admits(c100)`, `OptionValue::present(Composite(P), c100)` and `evaluate_record` for `R` with that field. Repeat each with `p100`.
6. Call `ValueType::admits` of a `Collection` type of element `Composite(C)` on a built `Set` of element `Composite(P)`, and the reverse. Clone the `Set` of step 1, run `planned_equality` against an identical build and against one built from `[p100, g100]`, and drop it.

## Expected Results

1. All calls complete. Each member keeps its declaration and slots (`p100` declaration `P`, `c100` declaration `C`). `Sequence` and `OrderedSet` keep supply order and hold two members; `Set` holds two members ordered `p100`, `c100`; `Bag` holds two members. No call tests an occurrence against the element type.
2. The log is `collection.member-walk`, `collection.member-test` (one pair), `collection.bound`, `collection.result-retain`, with no `collection.element` and no other charge.
3. The `Option` has `payload_type() == Composite(P)` and a payload of declaration `C` with its slots unchanged.
4. The slot holds `c100` unchanged; the log holds one `composite.result-retain`; `planned_equality` gives `false`.
5. With `c100`: `CheckedInvariant { cause: CollectionElementNotAdmitted }`, `ConstructionRefusal { component: Element(0), cause: TypeMismatch }`, `false`, `ConstructionRefusal { component: Payload, cause: TypeMismatch }`, `CheckedInvariant { cause: DeferredResultNotAdmitted }`. With `p100` each accepts.
6. Neither collection type admits the other's value. The clone equals the original (`true`), the build from `[p100, g100]` gives `false`, and the drop completes.
