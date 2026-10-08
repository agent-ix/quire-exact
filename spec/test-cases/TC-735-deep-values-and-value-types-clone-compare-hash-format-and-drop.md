---
id: TC-735
title: "Deep values and value types clone, compare, hash, format and drop"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-262
    type: verifies
---
# TC-735: Deep values and value types clone, compare, hash, format and drop

## Description

Verify iterative handling of 100,000-deep values and value types, and that the type nodes a deep value holds grow linearly with its depth.

Scope: FR-262-AC-2, FR-262-AC-3.

## Test Procedure

Run steps 1, 2 and 4 on a thread spawned with a 512 KiB stack.

1. Build a `ValueType` of 100,000 nested `Option`s around `Boolean`; clone, compare, hash, format for debug and drop it and its clone.
2. Build a `Value` of the same depth; clone it, compare it with its clone, and drop both.
3. Build values 1,000 and 2,000 levels deep, alternating `Option` and `Sequence` levels around an `Integer`, each level declared with the type of its depth. Count the distinct `Option` and `Collection` type nodes each holds, by allocation identity.
4. Build such a value and its type 100,000 levels deep. Check the type admits the value and the value holds at most 200,000 type nodes. Clone, compare, hash and format the type for debug, then drop the type, its clone and the value.

## Expected Results

- Step 1: the clone compares equal and hashes equal, and formatting and drops complete.
- Step 2: the clone compares equal, and the drops complete.
- Step 3: the 1,000-level value holds at least 990 type nodes, and the 2,000-level value holds at most twice that plus 2.
- Step 4: the type admits the value, the node bound holds, the clone compares equal and hashes equal, the debug output opens at least 100,000 brackets and closes as many as it opens, and the drops complete.

## Status

Implemented. The tests are in `src/value/value_type.rs` and `src/value.rs`, tagged `#[trace("TC-735", "FR-262-AC-2")]` for steps 1 and 2 and `#[trace("TC-735", "FR-262-AC-3")]` for steps 3 and 4.
