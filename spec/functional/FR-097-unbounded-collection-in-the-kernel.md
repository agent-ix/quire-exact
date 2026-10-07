---
id: FR-097
title: "An unbounded collection never refuses for cardinality"
type: FR
relationships: []
---
# FR-097: An unbounded collection never refuses for cardinality

## Description

The `quire-exact` kernel SHALL treat a collection type with no bound as unbounded: its construction never refuses for cardinality, still charges `collection.bound`, and stops only when the caller's meter runs out. The kernel SHALL expose the pure count check as `pub fn violation(self, count: u64) -> Option<BoundViolation>` as an inherent method of `CardinalityBound` for callers that form their own collection values. The checker's typing of `map`, `flatMap`, `filter` and `flatten` over an unbounded source keeps the id `FR-097` in the repository that owns it.

## Use case

A caller forms a collection of a million elements under no declared maximum. It completes unless the caller's own meter runs out.

## Behavior

1. **Unbounded.** `K<T>` and `K<T>[0, u64::MAX]` are different types. An unbounded type admits a collection of any size with no cardinality refusal and charges `collection.bound`.

2. **Inclusive public check.** `violation` returns `Some(BelowMinimum)` below the minimum, `Some(AboveMaximum)` above the maximum, and `None` at either endpoint or inside the interval. It requires no meter, changes no state and performs no collection construction. Owner collection admission and this public query use the same bound decision.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-097-AC-7 | An unbounded collection type admits a collection value of any size with no cardinality refusal; it still charges `collection.bound`, and stops with `Incomplete` at that charge point only when the caller's meter runs out. `K<T>` and `K<T>[0, u64::MAX]` are different types. | Test (TC-441) |
| FR-097-AC-8 | A caller outside the crate can call `CardinalityBound::violation(count)` and receive the existing `Option<BoundViolation>` type. For `[2, 4]`, counts 1, 2, 3, 4 and 5 return below-minimum, none, none, none and above-maximum respectively; singleton `[2, 2]` admits only 2. `[0, u64::MAX]` reports no violation at either extreme, `[0, 0]` rejects 1 above, and `[u64::MAX, u64::MAX]` rejects `u64::MAX - 1` below. The query is pure and unmetered; for `[2, 4]`, collection admission agrees with its decisions at counts 1 through 5. | Test |

## Status

AC-7 implemented; AC-8 planned for IR-673.

## Dependencies

- None. The caller's behaviour under the same id lives in `agent-ix/quire-spec-language`.
