---
id: FR-379
title: "Carry a union member and its positional payload as a kernel value"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-262
    type: references
  - target: ix://agent-ix/quire-exact/FR-369
    type: references
  - target: ix://agent-ix/quire-exact/FR-376
    type: references
  - target: ix://agent-ix/quire-exact/FR-377
    type: references
  - target: ix://agent-ix/quire-exact/FR-378
    type: references
---
# FR-379: Carry a union member and its positional payload as a kernel value

## Description

The `quire-exact` kernel SHALL carry one active member of a union declaration, with its payload values in declared position order, as `Value::Union`, with construction, comparison, key order, clone, `Debug` and drop that do not grow the native stack with the depth.

This requirement states what the kernel does. The language-level source is QSL FR-319, FR-321, FR-322 and FR-323, cited by number only. The kernel carries no member registry and mints no identity. The checking stage supplies the declaration key and exact member name already verified under QSpec FR-441, and the caller's type environment supplies the declared position types.

## Inputs

- A `UnionMember::from_admitted(declaration, member_name)`: the union declaration `NodeKey` and its exact admitted non-empty UTF-8 member name. The resulting `VariantId` carries that structural pair ([FR-389](./FR-389-carry-admitted-structural-identities.md)); no graph-node key is retyped as member identity.
- The declared position types of the member, as a `ValueType` slice, for `union` and `evaluate_union`.
- Payload values, or deferred payload expressions and a `Meter`, in position order.

## Outputs

- A `Value::Union` holding the member, the payload and an occurrence count of one plus the occurrences of the payload values.
- The Boolean relation, the pair count, the canonical key order and the collection orders that FR-376, FR-377 and the collection forms define for it.

## Behavior

1. **Carrier.** `UnionValue` retains its `UnionMember` and an immutable payload. `Clone` shares the payload by reference counting. `ValueType::Composite(K)` admits a union whose declaration is `K` and no other value kind.
2. **Exact construction.** `union(shape, member, payload)` refuses `ConstructionRefusal { component: Value, cause: WrongArity { declared, supplied } }` when the lengths differ, and otherwise `ConstructionRefusal { component: Position(i), cause: TypeMismatch }` for the first position whose value `ValueType::admits` refuses. It checks no registry membership and no reference conformance. `UnionValue::from_admitted` checks nothing (FR-378).
3. **Evaluation.** `evaluate_union` checks the arity before it runs any argument, runs the arguments in position order, and returns the first stop of an argument (`Undefined`, `Refused` or `Incomplete`) without running a later argument and without any charge. A completed argument that its position type does not admit refuses `CheckedInvariant { cause: DeferredResultNotAdmitted }`. A completed evaluation charges `composite.result-retain` once, sized by the occurrence count of the result.
4. **Equality.** For two unions of one declaration, different `VariantId`s give `false` with one pair. Equal `VariantId`s with equal payload lengths pair the payload positions in order, each as a pair of FR-376, and the union adds one pair. Equal `VariantId`s with different payload lengths refuse `ValueKindMismatch`.
5. **Unlike unions and kinds.** Two unions of different declarations refuse `ValueKindMismatch` in equality and give `None` from `compare_keys`: a union declaration has no descendant, and QSL FR-319-AC-3 makes comparing members of two union declarations a type error, so such a pair never meets in a checked program. A union paired with an enum, a composite or any other value kind refuses `ValueKindMismatch` and gives `None`.
6. **Key order.** For two unions of one declaration, `compare_keys` orders by the UTF-8 bytes of the member identifier. Equal identifiers identify the same `VariantId` under the admitted FR-441 pair and compare the payload positions in order. Different payload lengths give `None`. Equal member identifiers in different union declarations give `None` at the cross-declaration boundary, and a union of another declaration is not admitted by `ValueType::Composite` of the first. Neither the member identity's representation nor the declaration position of the member takes part in the within-declaration order.
7. **Collections.** `Set` and `Bag` hold unions in key order and merge equal members, `OrderedSet` keeps first-occurrence order, and `Sequence` keeps every occurrence, each from any supply order. A union that holds an IEEE value has no key: a `Set` formed through `form_grouped` refuses `CheckedInvariant { cause: CanonicalKeyUnavailable }`, and a `Sequence` keeps it.
8. **Depth.** Clone, equality, key comparison, `Debug` and `Drop` of a union nested ten thousand levels deep complete on a small stack (FR-262). The storage a nested union holds grows linearly with its depth, and a child shared by two parents counts once for each occurrence.
9. **Option.** `OptionValue::present(Composite(K), union)` admits a union of declaration `K`; equality and key comparison go through the `Option` payload to the union by Behavior 4 to 6.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-379-AC-1 | A nullary member has an occurrence count of 1 and a two-position member over two integers has 3. The value reports its declaration, `VariantId`, identifier and payload in position order. `ValueType::Composite` of its declaration admits it, `Composite` of another declaration and `Integer` do not. | Test |
| FR-379-AC-2 | `union` refuses an arity of 0 against 1 supplied and 2 against 1 supplied with `Value` `WrongArity`, and a `Boolean` at an `Integer` position with `Position(0)` `TypeMismatch`. | Test |
| FR-379-AC-3 | `evaluate_union` returns `Undefined` for the first argument that is undefined, runs no later argument and admits no charge. A completed `Boolean` at an `Integer` position refuses `DeferredResultNotAdmitted`. A completed evaluation runs the arguments in order and admits one `composite.result-retain` with the occurrence count of the result, and a retain limit of one fewer returns `Incomplete` naming the charge point, the limit kind and the limit. | Test |
| FR-379-AC-4 | Unions of one declaration and member with equal payloads are equal, with differing payload positions or members are not, and the pair count is one plus the payload pairs. | Test |
| FR-379-AC-5 | A union against a union of another declaration, an enum member and a composite refuses `ValueKindMismatch` in `plan_equality` and gives `None` from `compare_keys`. One member with payload lengths 1 and 2 refuses `ValueKindMismatch` and gives `None`. | Test |
| FR-379-AC-6 | Members `Alpha` and `Zulu` of one declaration order by identifier regardless of their payloads; one member orders by the payload positions in order. Declaration order does not reach the within-declaration order. The member `Alpha` of another union declaration is a different `VariantId`: `ValueType::Composite` of the first declaration refuses it, and `compare_keys` between the two unions gives `None` rather than ordering or equating them by their matching identifier. | Test |
| FR-379-AC-7 | `Set`, `Bag`, `OrderedSet` and `Sequence` over four unions, from a supply order and its reverse, hold the stated elements. A union with an IEEE payload refuses `CanonicalKeyUnavailable` through `form_grouped` for a `Set` and is kept by a `Sequence`. | Test |
| FR-379-AC-8 | Clone, equality, key comparison, `Debug` and drop of a union nested ten thousand levels complete on a 512 KiB stack, a mixed union, composite and collection nest does the same, storage grows linearly with depth, and a shared child counts each occurrence. | Test |
| FR-379-AC-9 | A present `Option` of declared type `Composite(K)` over a union of declaration `K` is admitted, two such `Option` values with equal unions are equal and with different members are not, and they compare by the key of the union. | Test |

## Status

Implemented. This requirement states the behavior of code written for QSL-500 before the requirement; it adds no behavior.

## Dependencies

- Equality and key order of a payload that pairs composites of different declarations are [FR-376](./FR-376-composite-equality-across-declarations.md) and [FR-377](./FR-377-composite-canonical-key-across-declarations.md); the caller-admitted payload is [FR-378](./FR-378-caller-admitted-values-in-collections-options-and-composites.md); the cause names are [FR-369](./FR-369-typed-checked-invariant-causes.md); the depth rule is [FR-262](./FR-262-deep-values-and-types-in-the-kernel.md).
- The falsifying controls are [TC-927](../test-cases/TC-927-union-carrier.md).
- The language-level sources are QSL FR-319, FR-321, FR-322 and FR-323.
