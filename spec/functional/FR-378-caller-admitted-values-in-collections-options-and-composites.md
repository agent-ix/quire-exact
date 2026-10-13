---
id: FR-378
title: "Collections, Options and composites hold values the caller admitted by conformance"
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
# FR-378: Collections, Options and composites hold values the caller admitted by conformance

## Description

Where a caller has admitted a composite element, payload or slot by conformance with knowledge the kernel does not hold (which declaration conforms to which), the `quire-exact` kernel SHALL store that value unchanged when the caller builds a collection with `form` or `form_grouped`, an `Option` with `OptionValue::from_admitted`, or a composite with `from_admitted_slots`.

The kernel has no declaration graph and takes none, so these existing caller-admitted paths are the sanctioned ones and no new constructor is added. The kernel's exact constructors and `ValueType::admits` stay exact. QSL FR-359 (items 5 and 8 and cross-repo requirement 3) states the language-level need.

## Inputs

- A `CollectionType` whose element type is the slot's declared type, and occurrences the caller has already admitted and charged `collection.element` for.
- For an `Option`, the declared payload `ValueType` and a payload the caller has admitted.
- For a composite, a declaration key and slots the caller has admitted.

## Outputs

- A `Value::Collection` whose `collection_type` is the declared type, whose members are the supplied values unchanged, ordered as FR-376 and FR-377 state.
- A `Value::Option` whose payload type is the declared type and whose payload is the supplied value unchanged.
- A composite whose slots are the supplied values unchanged.

## Behavior

1. **Collections.** `form` and `form_grouped` do not test an occurrence against the element type: the caller has admitted every occurrence, and the caller's evaluator charges `collection.element` for each, as `construct_collection` does for its own. `form` over a `Set`, `Bag` or `OrderedSet` charges one membership comparison per candidate-member pair it compares; `form` over a `Sequence` and `form_grouped` compare nothing. Both then charge `collection.bound` and `collection.result-retain`. Members of different declarations are stored unchanged, a `Set` and `Bag` order them by FR-377 and an `OrderedSet` keeps first-occurrence order.
2. **Options.** `OptionValue::from_admitted(payload_type, payload)` stores the payload as it is and records the declared `payload_type`. It checks nothing.
3. **Composites.** `from_admitted_slots` builds a composite from admitted slots. The caller then passes it to `retain_composite` so that `composite.result-retain` is charged as `evaluate_record` charges it.
4. **Exact paths unchanged.** `construct_collection`, `form_collection`, `ValueType::admits`, `OptionValue::present`, `evaluate_record` and `evaluate_tuple` admit a composite only when its declaration key equals the declared one. A descendant-declared value is refused by each of them with its existing refusal: `CollectionElementNotAdmitted`, `ConstructionRefusal { component: Element(index), cause: TypeMismatch }`, `false`, `ConstructionRefusal { component: Payload, cause: TypeMismatch }` and `DeferredResultNotAdmitted`.
5. **No covariance.** A collection's type identity stays its declared element type. `ValueType::admits` on a collection value of element type `S` at a collection type of element type `P` is `false`. Admitting a descendant-typed collection at an ancestor-typed collection slot is a caller decision the kernel does not provide.
6. **The kernel does not verify the admission.** A caller that stores a value that does not conform has a defect. The kernel's total key order, equality, charges and absence of panics hold for any stored composite, because they rest on the declaration key alone (FR-376, FR-377).
7. **Union payloads.** A union member payload follows the same rule once the kernel carries a union value; this requirement adds no union behaviour before then.
8. **Doc comments.** The rustdoc of `form`, `form_grouped`, `OptionValue::from_admitted` and `from_admitted_slots` says they are the paths for a caller-admitted value, including a conforming descendant, and that they check nothing. The rustdoc of `ValueType::admits` says it is the exact predicate and not the conformance check for a slot that can hold a descendant.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-378-AC-1 | `form` and `form_grouped` over an element type `Composite(P)` and one `P` member and one `C` member complete for `Sequence`, `Set`, `Bag` and `OrderedSet`, keep both members unchanged (declaration key and slots), and for `Set` and `OrderedSet` hold two members even with equal shared slots. No collection call tests an occurrence against the element type. | Test |
| FR-378-AC-2 | `form` over a `Set` of those two members charges, in order, `collection.member-walk`, `collection.member-test` with one pair, `collection.bound` and `collection.result-retain`, and no `collection.element`. | Test |
| FR-378-AC-3 | `from_admitted(Composite(P), Some(c))` returns an `Option` whose `payload_type()` is `Composite(P)` and whose payload has declaration `C` and its slots unchanged. | Test |
| FR-378-AC-4 | `from_admitted_slots` of a declaration whose declared field type is `Composite(P)` and whose slot holds a `C` composite keeps the slot unchanged; `retain_composite` charges `composite.result-retain` once; the composite is unequal to one with the same declaration and a `P` composite in that slot (FR-376). | Test |
| FR-378-AC-5 | For the same `C` composite at element type `Composite(P)`: `construct_collection` refuses `CollectionElementNotAdmitted`; `form_collection` refuses `Element(index)` `TypeMismatch`; `ValueType::admits` is `false`; `OptionValue::present` refuses `Payload` `TypeMismatch`; `evaluate_record` with that field refuses `DeferredResultNotAdmitted`. A `P` composite is accepted by each. | Test |
| FR-378-AC-6 | A collection of element type `Composite(C)` is not admitted by `ValueType::admits` at the collection type of element type `Composite(P)`, nor the reverse. A stored collection clones, compares equal to an identical build (`true`), compares unequal to one that differs in a member's declaration (`false`), and drops on the iterative paths of any `Value`. | Test |
| FR-378-AC-7 | The rustdoc of the five items in Behavior 8 carries the stated text. | Inspection |

## Status

Implemented. The slice changed documentation and added tests, and added no constructor.

## Dependencies

- Equality and key order of the held members are [FR-376](./FR-376-composite-equality-across-declarations.md) and [FR-377](./FR-377-composite-canonical-key-across-declarations.md); the closed cause set is [FR-369](./FR-369-typed-checked-invariant-causes.md).
- Text that changes with this requirement: the doc comments on the items in Behavior 8. No existing FR or AC text changes.
- The falsifying controls are [TC-926](../test-cases/TC-926-caller-admitted-values-in-collections-options-and-composites.md).
- QSL FR-359 requirement 3 asked for an environment-taking collection constructor; this requirement shows `form` already provides the caller-admitted path and asks for none.
