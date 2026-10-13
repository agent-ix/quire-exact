---
id: FR-377
title: "Canonical key orders composite values of different declarations by declaration key"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-376
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-369
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-359
    type: references
---
# FR-377: Canonical key orders composite values of different declarations by declaration key

## Description

When `compare_keys` orders two composite values whose declaration keys differ, the `quire-exact` kernel SHALL order them by the ascending byte order of their 32-byte declaration keys, compared before any slot, so that set, bag and ordered-set formation over such values completes instead of refusing `CanonicalKeyUnavailable`.

Two values of one type have equal keys exactly when they are equal under the kernel equality (the module rule of `src/key.rs`). FR-376 makes two composites of different declarations unequal, so their keys must differ and the key must order them. QSL FR-359 (item 5) states the language-level order and names this requirement as the kernel's part.

## Inputs

- Two values passed to `compare_keys`, or the members of a set, bag or ordered set reaching canonical ordering through `form`, `form_collection`, `form_grouped` or `construct_collection`.

## Outputs

- `Some(Ordering)`, or `None` where the pair has no key, as before.

## Behavior

1. **Different declarations.** For `(Composite, Composite)` whose declaration keys are not byte-equal, the key order is the byte-wise lexicographic order of the two `NodeKey::as_bytes()` arrays, most significant byte first. The slots of neither value are visited. The order is total and has no relation to ancestry, to the declaration's name or to the slot contents.
2. **Same declaration.** Two composites with the same declaration key and the same slot count continue by slots in declaration order, `absent` before `null` before a present value, each present pair ordered by this same key (the rule before this requirement).
3. **Equal keys.** `compare_keys` returns `Some(Equal)` for two composites exactly when they have the same declaration key and every slot pair has an equal key; for composites that is exactly when FR-376 equality gives `true`. Nested inside an `Option` payload, a record slot or a collection element, the same holds at every depth.
4. **Where `None` remains.** `compare_keys` still returns `None` for a composite against a non-composite, for two composites of one declaration key and different slot counts, for unlike value variants, for two enum members of equal rank and different `VariantId`, and for a `Float`. A population has no key. The doc comment "which a checked program never produces" no longer describes the composite case: a checked program that admits a descendant at an ancestor-typed slot does produce two declarations in one collection.
5. **Collections.** `form`, `form_collection`, `form_grouped` and `construct_collection` over a `Set`, `Bag` or `OrderedSet` whose members are composites of different declarations sort them by this key, stably, and complete. They report `CanonicalKeyUnavailable` only where `compare_keys` still returns `None` for a pair of members, as a `Float` pair does. The visiting order is a function of the member values alone, whatever the order in which they were supplied.
6. **Same key bytes, different content.** The kernel treats the 32-byte declaration key as the declaration's whole identity. It does not compare names, field lists or the caller's alias for a declaration.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-377-AC-1 | `compare_keys(a, b)` for composites of declaration keys `[1; 32]` and `[2; 32]` is `Some(Less)` and `compare_keys(b, a)` is `Some(Greater)`, whatever their slots: slots equal, slots that would order the other way, and different slot counts. | Test (TC-925) |
| FR-377-AC-2 | Declaration keys that differ only in the last byte (`[1; 31]` followed by `0`, against the same followed by `1`) order by that byte; keys that differ in the first byte order by the first byte even when the later bytes order the other way. | Test (TC-925) |
| FR-377-AC-3 | Two composites of one declaration key keep the slot order of the rule before this requirement; `compare_keys(x, x)` is `Some(Equal)`; two composites of different declarations are never `Some(Equal)`, even with identical slots. | Test (TC-925) |
| FR-377-AC-4 | A `Set`, a `Bag` and an `OrderedSet` formed from one composite of each of two declarations, in each of the two supply orders, complete without `CanonicalKeyUnavailable` and visit the member of the smaller declaration key first. A `Bag` holding two equal members of each declaration visits the two groups in that order. | Test (TC-925) |
| FR-377-AC-5 | Equal keys occur exactly when values are equal: for a set of composites of two declarations with equal and unequal slots, every pair has `compare_keys == Some(Equal)` exactly when `planned_equality` gives `true`. | Test (TC-925) |
| FR-377-AC-6 | A composite against an `Integer`, two composites of one declaration key and different slot counts, and a pair of `Float` members each return `None` from `compare_keys` and, as set members, refuse `CanonicalKeyUnavailable`. | Test (TC-925) |
| FR-377-AC-7 | The key order of a pair of different declarations holds as the payload of two `Option`s, in a record slot and as an element of two `Sequence` values, ordering the enclosing values by the same declaration-key rule. | Test (TC-925) |

## Status

Specified. Code lands in a later slice.

## Dependencies

- Text that changes with this requirement: the `compare_keys` doc comment and the module doc of `src/key.rs`; the `CanonicalKeyUnavailable` row of [FR-369](./FR-369-typed-checked-invariant-causes.md) and the third step of [TC-917](../test-cases/TC-917-typed-checked-invariant-causes.md), which already uses a `Float` pair and stays valid; the `compare_keys` mentions in FR-370-AC-3 and FR-088-AC-11 keep their text.
- Equality of the same pairs is [FR-376](./FR-376-composite-equality-across-declarations.md); the constructor that admits such members is [FR-378](./FR-378-caller-admitted-collection-elements-and-option-payloads.md).
- The falsifying controls are [TC-925](../test-cases/TC-925-composite-canonical-key-across-declarations.md).
- The language-level statement is QSL FR-359 (item 5 and cross-repo requirement 3).
