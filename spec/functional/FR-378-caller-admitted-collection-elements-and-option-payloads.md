---
id: FR-378
title: "Collections and Options hold payloads the caller admitted by conformance"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-376
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-377
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-369
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-359
    type: references
---
# FR-378: Collections and Options hold payloads the caller admitted by conformance

## Description

When a caller holds knowledge the kernel does not (which declaration conforms to which, so that a descendant-declared composite belongs at an ancestor-typed element or payload slot), the `quire-exact` kernel SHALL let the caller supply that admission to collection construction. The kernel accepts an already-admitted `Option` payload through `OptionValue::from_admitted`. The kernel keeps its exact, environment-less constructors and `ValueType::admits` unchanged. QSL FR-359 (items 5 and 8 and cross-repo requirement 3) states the language-level need.

The kernel has no declaration graph and takes none. The caller admits; the kernel orders by the canonical key (FR-377), compares by equality (FR-376) and charges.

## Inputs

- A `CollectionType` whose element type is the slot's declared type, and the element values or deferred element expressions.
- An admission supplied by the caller: a function of the declared element `ValueType` and a `Value` that answers whether the caller admits the value at that type.
- For an `Option`, the declared payload `ValueType` and a payload the caller has already admitted.

## Outputs

- A `Value::Collection` whose `collection_type` is the declared type, whose elements are the supplied values unchanged, in canonical order, with the charges of the exact constructors.
- A `Value::Option` whose payload type is the declared type and whose payload is the supplied value unchanged.

## Behavior

1. **Admitting constructors.** The kernel SHALL provide, beside `construct_collection` and `form_collection`, a construction path for each that takes the caller's admission. The names and the admission's type (a trait, a closure or an equivalent) are implementation-owned. Each runs the exact constructor's steps with one change: the per-element test `collection_type.element().admits(value)` is replaced by the caller's admission. Order of charges, the first-stopped rule for deferred elements, membership comparison, the cardinality bound, canonical ordering and result retention are as for the exact constructors.
2. **Admitted elements keep their identity.** An element the admission accepts is stored as it is: not retagged, copied into the declared type or converted. A collection holding a descendant-declared composite and an ancestor-declared composite of equal shared slots holds two elements (FR-376), ordered by FR-377.
3. **Refusals.** An element the admission rejects refuses exactly as the exact constructor refuses a non-member: `CheckedInvariant { cause: CollectionElementNotAdmitted }` from the deferred path, and `ConstructionRefusal { component: Element(index), cause: TypeMismatch }` from the occurrence path, before any charge. The kernel calls the admission once per element, in supply order, and not after the first rejection.
4. **No covariance.** A collection's type identity stays its declared element type, so `ValueType::admits` and the collection and `Option` equality of the exact kernel are unchanged: a collection built with element type `S` is not admitted at element type `P` by `ValueType::admits`. Admitting a descendant-typed collection at an ancestor-typed collection slot is a caller decision the kernel does not provide.
5. **The kernel does not verify the admission.** An admission that accepts a value that does not conform is a caller defect. The kernel's own guarantees (total key order, equality, charges, no panic) hold for any accepted element of any declaration, because they rest on the declaration key alone (FR-376, FR-377).
6. **`Option` payloads.** `OptionValue::from_admitted(payload_type, payload)` is the sanctioned path for a payload the caller has admitted by conformance. It stores the payload as it is and records the declared `payload_type`. `OptionValue::present` stays the exact kernel constructor: it admits a payload only through `ValueType::admits` and refuses a descendant-declared composite at an ancestor-declared composite payload type with `ConstructionRefusal { component: Payload, cause: TypeMismatch }`. No third `Option` constructor is added.
7. **Composite slots.** `from_admitted_slots` is the existing path for a composite whose slots the caller admitted. The exact `evaluate_record` and `evaluate_tuple` stay exact and are unchanged by this requirement.
8. **Doc comments.** The documentation of `OptionValue::from_admitted` SHALL say that it is the path for a caller-admitted payload, including a conforming descendant, and that it checks nothing. The documentation of `ValueType::admits` SHALL say that it is the exact predicate and is not the conformance check for a slot that can hold a descendant.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-378-AC-1 | With an admission that accepts a composite of declaration `[2; 32]` at element type `Composite([1; 32])`, the admitting forms of `Sequence`, `Set`, `Bag` and `OrderedSet` over one `[1; 32]` member and one `[2; 32]` member complete, keep both members unchanged (declaration key and slots intact), and for the unique kinds hold two elements even with equal shared slots. | Test (TC-926) |
| FR-378-AC-2 | The exact `construct_collection` and `form_collection` over the same members still refuse: `CollectionElementNotAdmitted` from the deferred path and `Element(index)` `TypeMismatch` from the occurrence path. `ValueType::admits` of `Composite([1; 32])` on a `[2; 32]` composite is still `false`. | Test (TC-926) |
| FR-378-AC-3 | An admission that rejects the third of four elements refuses with the exact constructor's refusal for that path, admits calls for elements one to three only, and on the occurrence path leaves the meter unchanged. | Test (TC-926) |
| FR-378-AC-4 | The charge log of an admitting form is identical to the exact form's for members that both accept, in kind, order and totals, including the membership comparisons of a `Set` and a `Bag`, `collection.bound` and `collection.result-retain`. Cardinality bounds are enforced as before. | Test (TC-926) |
| FR-378-AC-5 | `OptionValue::from_admitted(Composite([1; 32]), Some(composite [2; 32]))` returns an `Option` whose `payload_type()` is `Composite([1; 32])` and whose payload has declaration `[2; 32]` and its slots unchanged. `OptionValue::present` with the same arguments refuses `TypeMismatch` at `Component::Payload`. A `[1; 32]` payload is accepted by both. | Test (TC-926) |
| FR-378-AC-6 | A collection formed by the admitting path holds its members after a clone, an equality plan against an identical collection (`true`) and against one that differs in a member's declaration (`false`, FR-376), and a drop, on the same iterative paths as any `Value`. A collection of element type `Composite([2; 32])` is not admitted by `ValueType::admits` at `Collection` of element type `Composite([1; 32])`. | Test (TC-926) |
| FR-378-AC-7 | The rustdoc of `OptionValue::from_admitted` and `ValueType::admits` carries the statements of Behavior item 8. | Inspection |

## Status

Specified. Code lands in a later slice.

## Dependencies

- Equality and key order of the held members are [FR-376](./FR-376-composite-equality-across-declarations.md) and [FR-377](./FR-377-composite-canonical-key-across-declarations.md); the closed cause set is [FR-369](./FR-369-typed-checked-invariant-causes.md).
- Text that changes with this requirement: the doc comments on `OptionValue::from_admitted`, `ValueType::admits`, `construct_collection`, `form_collection` and `form` (`src/value.rs`, `src/collection.rs`). No existing FR or AC text changes.
- The falsifying controls are [TC-926](../test-cases/TC-926-caller-admitted-collection-elements-and-option-payloads.md).
- The language-level statement is QSL FR-359 (items 5 and 8 and cross-repo requirement 3).
