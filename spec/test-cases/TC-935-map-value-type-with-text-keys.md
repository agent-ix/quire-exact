---
id: TC-935
title: "Map value type with Text keys"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-386
    type: verifies
---
# TC-935: Map value type with Text keys

## Description

Verify [FR-386](../functional/FR-386-map-value-type-with-text-keys.md) through the public `ValueType::map`, `MapType`, `ValueType::admits` and the traits of `ValueType`. Scope: FR-386-AC-1 through FR-386-AC-6.

## Test Procedure

1. Build `Map<Text, Boolean>` unbounded and at `[1, 3]`; read `values()` and `bound()`.
2. Compare and hash the map types of FR-386-AC-2 against each other, against `Sequence`, `Set`, `Bag` and `OrderedSet` over `Boolean` with the same bounds, and against `Option<Boolean>`.
3. Build `Map<Text, Map<Text, Boolean>>`, `Map<Text, Option<Boolean>>` and `Map<Text, Sequence<Boolean>>`, and each with an innermost `Integer`; compare and hash.
4. On a 512 KiB stack thread build the two 100,000-link chains of FR-386-AC-4, clone, compare, hash, format and drop them.
5. Format the two types of FR-386-AC-5 in compact mode.
6. Call `admits` on a `Map` type with a collection, a text and an option value.

## Expected Results

1. `values()` equals `Boolean`; `bound()` is `None` and `Some([1, 3])`.
2. Equal exactly for equal value type and bound; equal maps hash equal; every cross-variant pair is unequal.
3. Differing innermost links make the chains unequal.
4. Every operation completes with no stack overflow; balanced brackets; the innermost difference is unequal.
5. The exact strings of FR-386-AC-5.
6. `false` each time.
