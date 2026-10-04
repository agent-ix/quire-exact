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

Verify the kernel's optional bound end to end. Scope: FR-097-AC-7.

Scope: FR-097-AC-7.

## Test Procedure

1. Form an unbounded `Sequence<Integer>` of 1000 elements, then the same elements under `[0, 1]`, then under a meter with 999 value occurrences.
2. Compare the types `Sequence<Int[0,9]>` unbounded and at `[0, u64::MAX]`.

## Expected Results

- Step 1: completes; `AboveMaximum`; `Incomplete` at `collection.bound`.
- Step 2: the types are different.

## Status

Implemented. The tests are in `src/collection.rs`, tagged `#[trace("TC-441", "FR-097-AC-7")]`.
