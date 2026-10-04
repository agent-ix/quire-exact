---
id: FR-097
title: "An unbounded collection never refuses for cardinality"
type: FR
relationships: []
---
# FR-097: An unbounded collection never refuses for cardinality

## Description

The `quire-exact` kernel SHALL treat a collection type with no bound as unbounded: its construction never refuses for cardinality, still charges `collection.bound`, and stops only when the caller's meter runs out. The checker's typing of `map`, `flatMap`, `filter` and `flatten` over an unbounded source keeps the id `FR-097` in the repository that owns it.

## Use case

A caller forms a collection of a million elements under no declared maximum. It completes unless the caller's own meter runs out.

## Behavior

1. **Unbounded.** `K<T>` and `K<T>[0, u64::MAX]` are different types. An unbounded type admits a collection of any size with no cardinality refusal and charges `collection.bound`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-097-AC-7 | An unbounded collection type admits a collection value of any size with no cardinality refusal; it still charges `collection.bound`, and stops with `Incomplete` at that charge point only when the caller's meter runs out. `K<T>` and `K<T>[0, u64::MAX]` are different types. | Test (TC-441) |

## Status

Implemented.

## Dependencies

- None. The caller's behaviour under the same id lives in `agent-ix/quire-spec-language`.
