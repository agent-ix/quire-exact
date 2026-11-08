---
id: FR-386
title: "A Map value type with a key type"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-097
    type: references
  - target: ix://agent-ix/quire-exact/FR-262
    type: references
  - target: ix://agent-ix/quire-specification/FR-154
    type: references
---
# FR-386: A Map value type with a key type

## Description

The `quire-exact` kernel SHALL provide a value type `Map<K, V>[min, max]`, and the unbounded `Map<K, V>`, as the variant `ValueType::Map` holding a `MapType`. The type carries its key type `K`, its value type `V` and an optional `CardinalityBound`, and `MapType::key()` returns `K` so a caller can check a key against its own eligibility rule (QSL FR-230-AC-19 refuses `Map<Timer, Int>` at the key type). The kernel decides no key eligibility beyond the shape rule in Behavior 8. The kernel SHALL provide no map value in this requirement: the map type is type identity, so that a caller can declare, compare, hash and nest a package map (QSpec FR-154 "a map `values`") and read it as a clock collection of its kind (QSpec FR-154-AC-27) without a kernel value that inhabits it.

## Use case

A package map type `Map<Text, Timer>[0, 2]` is a field of an object type. The checker reads the field's type as `ValueType::map(MapType::new(String, Composite(Timer), Some(bound))?)`, asks for its value type to find the clock places and for its key type to refuse an ineligible key, and compares it with the same declaration read elsewhere.

## Inputs

- A key type `K`, a value type `V` and an optional `CardinalityBound`.

## Outputs

- `ValueType::Map(Arc<MapType>)`, built by `ValueType::map(MapType::new(key, values, bound)?)`. `MapType::key()` returns `K`, `MapType::values()` returns `V` and `MapType::bound()` returns the bound.

## Behavior

1. **Not a collection kind.** A map is not a `CollectionKind`. A collection's element is its member and its kinds are ordered by occurrence, key or first occurrence (FR-097); a map's declared nested type is the value type. `CollectionKind::ALL` is unchanged and a `Map` type is never a `Collection` type.
2. **Type identity.** Two map types are the same type exactly when their key types, value types and bounds are equal, and bound presence is part of the identity, as for `CollectionType`: `Map<K, V>` and `Map<K, V>[0, u64::MAX]` are different types, and `Map<K1, V>` and `Map<K2, V>` are different types for unequal `K1`, `K2`. A map type is unequal to the collection type of any kind over the same `V` and the same bound, and to `Option<V>`.
3. **Equality and hash.** Equality of `ValueType` and its `Hash` follow one rule: equal types hash equal. A map's key type, bound and value type, to any depth, take part in both.
4. **Nesting.** A map type nests any value type, and any type nests a map type, including `Option`, `Collection` and `Map` links in any order. A map link shares its `MapType` by reference count, as `Option` and `Collection` links do (FR-262): `Clone` costs one increment, and a type nested N levels deep holds O(N) type nodes.
5. **Deep types.** Clone, equality, hash, `Debug` and drop of a `ValueType` nested 100,000 levels deep through `Map` links, and through `Map`, `Option` and `Collection` links mixed, run on a 512 KiB stack with no recursion proportional to depth.
6. **Debug.** `Debug` prints a map type as `Map(MapType { key: .., values: .., bound: .. })` in compact and alternate mode.
7. **Admission.** `ValueType::admits` returns `false` for every `Value` at a `Map` type, because the kernel has no map value. A `Value` whose declared type contains a `Map` type is formed only through the types the value is built from, which this requirement does not change.
8. **Refusal.** `MapType::new` refuses a key type that nests another type (`Option`, a collection or a map) with `NestedMapKey`, so that the type chain the deep-type walks follow (Behavior 5) stays a chain with one nested value link. Every other key type, including `Text`, `String`, `Integer`, `Int` and `Composite`, is kept as declared.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-386-AC-1 | `ValueType::map(MapType::new(V, bound))` returns a `ValueType::Map` whose `MapType::key()` equals `K`, whose `MapType::values()` equals `V` and whose `MapType::bound()` equals `bound`, for `bound` both `None` and `Some`. | Test (TC-935) |
| FR-386-AC-2 | Two map types are equal exactly when key type, value type and bound are equal. Maps differing only in value type, only in bound, and only in bound presence (`None` against `Some([0, u64::MAX])`) are unequal, and equal maps hash equal. A map is unequal to every `Collection` type of every `CollectionKind` over the same value type and bound, and to `Option` of the same value type. | Test (TC-935) |
| FR-386-AC-3 | A map whose value type is a map, an `Option` or a `Collection` compares by its whole nested chain: two such types differing only at the innermost link are unequal, and equal chains hash equal. | Test (TC-935) |
| FR-386-AC-4 | On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Map` links around `Boolean`, and one of 100,000 links cycling `Map`, `Option` and `Sequence`, clone, compare equal to their clones, hash equal to their clones, format for debug with balanced brackets, and drop. Two chains differing only at the innermost link are unequal. | Test (TC-935) |
| FR-386-AC-5 | `Debug` of `Map<String, Boolean>` and of a bounded `Map<String, Option<Boolean>>[1, 3]` prints exactly `Map(MapType { key: String, values: Boolean, bound: None })` and `Map(MapType { key: String, values: Option(Boolean), bound: Some(CardinalityBound { minimum: 1, maximum: 3 }) })`. | Test (TC-935) |
| FR-386-AC-6 | `ValueType::admits` on a `Map` type returns `false` for a collection value, a text value and an option value. | Test (TC-935) |
| FR-386-AC-7 | `Debug` in alternate mode (`{:#?}`) of `Map<Text, Boolean>` and of the bounded `Map<Text, Option<Boolean>>[1, 3]` prints exactly what `#[derive(Debug)]` pretty-printing prints: `Map(\n    MapType {\n        key: String,\n        values: Boolean,\n        bound: None,\n    },\n)` and the same nesting, four spaces per level with a trailing comma on every field, for the bounded type. | Test (TC-935) |
| FR-386-AC-8 | `MapType::key()` returns the declared key type. Two maps differing only in key type (`String` against `Integer`) are unequal and equal maps hash equal. | Test (TC-935) |
| FR-386-AC-9 | `MapType::new` returns `NestedMapKey` for a key type that is an `Option`, a `Collection` or a `Map`, and builds a map for a leaf key. | Test (TC-935) |

## Status

Implemented. Map values (construction, key uniqueness, canonical order, metered charges) are not part of this requirement; a later requirement adds them if a caller needs a kernel map value.

## Dependencies

- Collection kinds and bounds: [FR-097](./FR-097-unbounded-collection-in-the-kernel.md). Deep types: [FR-262](./FR-262-deep-values-and-types-in-the-kernel.md).
- The package map this type represents is QSpec FR-154 (a map `values`, FR-154-AC-24 and FR-154-AC-27). A map of `Clock` or unit-typed values is a clock collection of its kind by QSpec FR-415; the kernel holds no unit or clock semantics here beyond the value type's own identity.
- The falsifying controls are [TC-935](../test-cases/TC-935-map-value-type-with-a-key-type.md).
