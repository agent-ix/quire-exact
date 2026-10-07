---
id: TC-441
title: "An unbounded collection never refuses for cardinality and stops only on the caller's meter"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-097
    type: verifies
---
# TC-441: An unbounded collection never refuses for cardinality and stops only on the caller's meter

## Description

Verify the kernel's optional bound end to end.

Scope: FR-097-AC-7 and FR-097-AC-8.

## Test Procedure

1. Form an unbounded `Sequence<Integer>` of 1000 elements, then the same elements under `[0, 1]`, then under a meter with 999 value occurrences.
2. Compare the types `Sequence<Int[0,9]>` unbounded and at `[0, u64::MAX]`.

3. From an integration test using only `quire_exact` exports, call the count query at the below/minimum/interior/maximum/above and extreme fixtures in AC-8, then call it repeatedly to check the same answer. For `[2, 4]` only, compare bounded collection admission at counts 1 through 5 using a sufficient public meter; extreme count queries do not require forming collections of those sizes.

## Expected Results

- Step 1: completes; `AboveMaximum`; `Incomplete` at `collection.bound`.
- Step 2: the types are different.
- Step 3: the public query returns precisely the typed inclusive-bound decisions in AC-8 without constructing a collection or charging a meter. Ordinary collection admission makes the same decision.

## Status

PR #9 adds executable Trace bindings for FR-097-AC-8. Run `quire matrix` for the current criterion-to-test mapping.
