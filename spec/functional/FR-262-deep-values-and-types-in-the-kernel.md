---
id: FR-262
title: "Deep values and value types clone, compare, hash, format and drop"
type: FR
relationships: []
---
# FR-262: Deep values and value types clone, compare, hash, format and drop

## Description

The `quire-exact` kernel SHALL handle a `Value` and a `ValueType` of any depth without growing the native stack with the depth, in clone, equality, hash, debug format and drop. The caller's evaluation of a deep value keeps the id `FR-262` in the repository that owns it.

## Use case

A caller builds a `ValueType` of 100,000 nested `Option`s. Cloning, comparing, hashing, formatting and dropping it run on a 512 KiB thread.

## Behavior

1. **Iterative traits.** `Clone`, `PartialEq`, `Hash`, `Debug` and `Drop` on `Value` and `ValueType` run on an explicit stack.
2. **Shared payload types.** An `Option` value type's payload type and a `Collection` value type's collection type are shared by reference count, not copied. A value nested N levels deep, each level declared with its own type, holds a number of distinct type nodes linear in N.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-262-AC-2 | On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option`s around `Boolean` clones, compares equal to its clone, hashes equal to its clone, formats for debug and drops. A `Value` of the same depth compares equal to its clone, and it and its clone drop. | Test (TC-735) |
| FR-262-AC-3 | A `Value` nested N levels deep, alternating `Option` and `Sequence` levels around an `Integer` with each level declared with the type of its depth, holds at least N - 10 and at most 2N distinct `Option` and `Collection` type nodes, and the same value 2N levels deep holds at most twice the nodes of the N-level value plus 2. On a thread with a 512 KiB stack, such a value 100,000 levels deep and its 100,000-deep type are built, the type admits the value, and the type clones, compares equal to its clone, hashes equal to its clone, formats for debug with balanced brackets and drops, as does the value. | Test (TC-735) |

## Status

Implemented.

## Dependencies

- None. The caller's behaviour under the same id lives in `agent-ix/quire-spec-language`.
