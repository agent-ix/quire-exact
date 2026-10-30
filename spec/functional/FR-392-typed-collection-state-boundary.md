---
id: FR-392
title: "Typed collection state construction and poststate inspection"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-specification/FR-144
    type: derives_from
  - target: ix://agent-ix/quire-exact/FR-097
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-378
    type: depends_on
---
# FR-392: Typed collection state construction and poststate inspection

## Description

When a caller supplies a collection-valued subject state, `quire-exact` SHALL construct and inspect it through the single `Value::Collection` representation carrying `CollectionType`. This boundary does not define a generated subject function signature or claim that a code generator uses it.

## Inputs

- A declared `CollectionType` with kind, element `ValueType`, and optional inclusive `CardinalityBound`.
- Prestate occurrences in authored order, or a poststate `Value` produced by a subject.
- A caller-owned `Meter` for checked construction and poststate inspection.

## Outputs

- A completed `Value::Collection` prestate, or a borrowed typed poststate view exposing its declared kind, element type, bound, and canonical elements.
- A typed element, type, cardinality, duplicate, or meter refusal without a partial value or view.

## Behavior

1. **Checked prestate.** The public checked prestate path uses `form_collection` with the declared `CollectionType`, completed occurrences and caller meter. It refuses an element outside the exact element type at its zero-based source index before collection work, preserves sequence order and duplicates, and retains only the first occurrence of each ordered-set member in source order. It applies the inclusive bound to sequence occurrences or ordered-set retained members. It uses the existing collection membership, bound and result-retain charges. A caller that has separately admitted a conforming descendant uses the explicitly trusted paths of [FR-378](./FR-378-caller-admitted-values-in-collections-options-and-composites.md); it cannot present that path as checked element admission.
2. **Checked poststate.** A public read-only inspection operation accepts a declared `CollectionType`, a borrowed `Value`, and a caller meter. It shall return a borrowed collection view only after checking that the value is a collection of exactly that type, including bound presence, every stored element is admitted by the exact element type, its count satisfies its inclusive bound, and its stored elements satisfy the kind's canonical form. For `Sequence`, occurrence order and duplicates are unchanged. For `OrderedSet`, every stored member is distinct under collection membership and retained in the supplied first-occurrence order. Inspection never silently deduplicates, reorders, or converts a produced poststate.
3. **Trust boundary.** The inspection operation applies the same checks to a value returned by a generated subject and to a value built by the public no-recheck `from_admitted` path. Neither `ValueType::admits` nor a matching `CollectionType` alone authenticates nested members or canonical order. A caller claiming conformance-based descendant admission supplies its own checked admission evidence above this kernel; the kernel's exact inspection shall refuse a descendant-declared element where exact element identity is required.
4. **Metering and failure.** The checked prestate path keeps its established charge sequence. Poststate inspection charges each membership comparison it performs through the collection membership charge points, and charges the bound check at `collection.bound`; it does not charge `collection.element` or `collection.result-retain` because it evaluates and retains no new element. A denied charge returns `Incomplete` at the denied point without a view. A wrong outer type, nonadmitted member, duplicate ordered-set member, or violated bound returns a typed refusal with its originating component or bound cause; it never panics or returns a partial view.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-392-AC-1 | From outside the crate, a caller builds `Sequence<Integer>[1,3]` from `[2,2,1]` with the checked path and reads the same three elements in the same order; `OrderedSet<Integer>[1,3]` from `[2,2,1]` reads `[2,1]`. The values retain their exact kind, element type and bound. | Test |
| FR-392-AC-2 | At counts below, at and above each inclusive endpoint, checked prestate construction counts occurrences for `Sequence` and retained members for `OrderedSet`; below and above refuse with the corresponding `BoundViolation`, and no partial collection escapes. An absent bound stays distinct from `[0,u64::MAX]`. | Test |
| FR-392-AC-3 | A checked prestate with a wrong element refuses at `Component::Element(index)` with `ConstructionCause::TypeMismatch` before collection membership or bound work; meter denial at each reached collection charge returns `Incomplete` at that charge with no partial value. | Test |
| FR-392-AC-4 | The public poststate inspector yields a borrowed view of an exactly typed, admitted sequence or ordered set, exposing the declared type and elements without changing their order or allocating a replacement collection; a wrong kind, element type, bound, or noncollection value is refused at the whole value. | Test |
| FR-392-AC-5 | A `Value::Collection` built through public `from_admitted` with a mismatched element, out-of-bound count, or duplicate ordered-set member is refused by poststate inspection at the offending element or bound. The same inspection does not silently normalize it; a conforming descendant stored under an ancestor exact element type is refused until a higher-level caller admits it under its own contract. | Test |
| FR-392-AC-6 | Inspection of an admitted sequence preserves duplicates and order; inspection of an admitted ordered set preserves first-occurrence order. A denied membership or bound charge returns `Incomplete` at the denied point and no view; inspection does not charge element evaluation or result retention. | Test |
| FR-392-AC-7 | The public API exposes one collection representation (`Value::Collection` and `CollectionType`) for the prestate and poststate; no parallel RT collection type or generated-function ABI is defined by this kernel requirement. | Inspection |

## Status

Planned. Existing `form_collection` and `CollectionValue` support the prestate shape; the checked poststate inspection boundary and its typed refusal remain to be implemented and tested.

## Dependencies

- QSpec [FR-144](ix://agent-ix/quire-specification/FR-144) defines the collection algebra, exact type identity and inclusive bounds.
- [FR-097](./FR-097-unbounded-collection-in-the-kernel.md) defines absent-bound behavior; [FR-378](./FR-378-caller-admitted-values-in-collections-options-and-composites.md) keeps trusted caller admission separate.
- CG's generated subject invocation, state frame and clause assertion are downstream contracts; this requirement supplies their value form only.
