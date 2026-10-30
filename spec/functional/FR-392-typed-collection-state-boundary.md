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
- A caller-supplied schema-aware element-admission authority that can inspect nested composite declarations and slots. The caller owns this authority and its model registry; the kernel does not synthesize it from `ValueType::admits`.

## Outputs

- A completed `Value::Collection` prestate, or a borrowed typed poststate view exposing its declared kind, element type, bound, and canonical elements.
- A typed element, type, cardinality, duplicate, or meter refusal without a partial value or view.

## Behavior

1. **Checked prestate.** The public typed prestate builder accepts an optional caller-owned schema-aware admission authority and asks it to admit each completed occurrence against the declared element type before calling `form_collection` with the same `CollectionType` and meter. The existing `form_collection` remains a low-level exact outer-type check; its `ValueType::admits` check alone does not authenticate nested slots. The builder refuses the first unadmitted occurrence at its zero-based source index before collection work, preserves sequence order and duplicates, and retains only the first occurrence of each ordered-set member in source order. It applies the inclusive bound to sequence occurrences or ordered-set retained members and keeps the existing collection membership, bound and result-retain charges. A caller that has separately admitted a conforming descendant may use [FR-378](./FR-378-caller-admitted-values-in-collections-options-and-composites.md)'s trusted path, but cannot present that path as exact admission by this builder.
2. **Checked poststate.** A public read-only inspection operation accepts a declared `CollectionType`, a borrowed `Value`, a caller meter and an optional caller-owned admission authority. It returns a borrowed collection view only after checking exact outer type including bound presence, asking the authority to authenticate every stored element and nested member against the declared element type, checking the inclusive count, and checking the stored kind's observable canonical constraints. For `Sequence`, the stored occurrence order and duplicates are unchanged. For `OrderedSet`, the stored members are pairwise distinct under collection membership and the inspector preserves their stored order; only a construction path with the source occurrence stream can establish that this is the original first-occurrence order. Inspection never silently deduplicates, reorders, or converts a produced poststate.
3. **Trust boundary.** The caller owns and authenticates the admission authority against its checked model/declaration registry; exact owns the collection representation and guarantees that it invokes that authority for every element before returning a value or view. If no authority is supplied, the typed builder and inspector refuse with a typed missing-admission-authority cause rather than returning a value or view. A matching outer `CollectionType`, `ValueType::admits`, or a caller-supplied no-recheck `from_admitted` value is not evidence that nested slots are valid. The inspector applies the same checks to a generated subject result and a value built by `from_admitted`; a caller that incorrectly approves a forged nested value violates the authority's own contract, not the kernel's claim.
4. **Metering and failure.** The checked prestate path keeps the existing collection charge sequence after element admission; any admission-authority charge belongs to that caller's contract. Poststate inspection charges each collection membership comparison it performs through the collection membership charge points, and charges the bound check at `collection.bound`; it does not charge `collection.element` or `collection.result-retain` because it evaluates and retains no new element. A denied charge returns `Incomplete` at the denied point without a view. A wrong outer type, nonadmitted member, missing admission authority, duplicate ordered-set member, or violated bound returns a typed refusal with its originating component or bound cause; it never panics or returns a partial view.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-392-AC-1 | From outside the crate, a caller builds `Sequence<Integer>[1,3]` from `[2,2,1]` with the checked path and reads the same three elements in the same order; `OrderedSet<Integer>[1,3]` from `[2,2,1]` reads `[2,1]`. The values retain their exact kind, element type and bound. | Test |
| FR-392-AC-2 | For reachable counts 1, 2, 3, 4 and 5 under the inclusive bound `[2,4]`, checked prestate construction counts occurrences for `Sequence` and retained members for `OrderedSet`; counts 1 and 5 refuse below and above respectively, 2 and 4 are admitted, and no partial collection escapes. `[0,0]` admits empty and refuses one above. An absent bound and `[0,u64::MAX]` admit tested counts but remain different types. | Test |
| FR-392-AC-3 | A typed prestate with a wrong element or an element rejected by the caller-owned schema authority refuses at `Component::Element(index)` before collection membership or bound work; meter denial at each reached collection charge returns `Incomplete` at that charge with no partial value. | Test |
| FR-392-AC-4 | With a caller-authenticated schema-aware admission authority, the public poststate inspector yields a borrowed view of an exactly typed sequence or ordered set, exposing the declared type and elements without changing stored order or allocating a replacement collection; a wrong kind, element type, bound, or noncollection value is refused at the whole value. | Test |
| FR-392-AC-5 | Public `from_admitted` values with a wrong element, out-of-bound count, or duplicate ordered-set member are refused by poststate inspection at the offending element or bound, without silent normalization. A matching outer `Composite(P)` element whose nested required slot is forged or missing is rejected by the caller-authenticated schema authority at that element even though `ValueType::admits` is true; without that authority, inspection returns typed missing-admission-authority refusal and no view. | Test |
| FR-392-AC-6 | Inspection of an admitted sequence preserves stored duplicates and order; inspection of an admitted ordered set checks pairwise uniqueness and preserves stored order, while the builder establishes first-occurrence order from source occurrences. A denied membership or bound charge returns `Incomplete` at the denied point and no view; inspection does not charge element evaluation or result retention. | Test |
| FR-392-AC-7 | The public API exposes one collection representation (`Value::Collection` and `CollectionType`) for the prestate and poststate; no parallel RT collection type or generated-function ABI is defined by this kernel requirement. | Inspection |

## Status

Planned. Existing `form_collection` and `CollectionValue` support the outer prestate shape; the authority-taking typed builder, checked poststate inspection boundary and its typed refusal remain to be implemented and tested.

## Dependencies

- QSpec [FR-144](ix://agent-ix/quire-specification/FR-144) defines the collection algebra, exact type identity and inclusive bounds.
- [FR-097](./FR-097-unbounded-collection-in-the-kernel.md) defines absent-bound behavior; [FR-378](./FR-378-caller-admitted-values-in-collections-options-and-composites.md) keeps trusted caller admission separate.
- CG's generated subject invocation, state frame and clause assertion are downstream contracts; this requirement supplies their value form only.
