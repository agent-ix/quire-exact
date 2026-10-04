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

Verify iterative handling of 100,000-deep values and value types. Scope: FR-262-AC-2.

Scope: FR-262-AC-2.

## Test Procedure

Run every step on a thread spawned with a 512 KiB stack.

1. Build a `ValueType` of 100,000 nested `Option`s around `Boolean`; clone, compare, hash, format for debug and drop it and its clone.
2. Build a `Value` of the same depth; clone it, compare it with its clone, and drop both.

## Expected Results

- Step 1: the clone compares equal and hashes equal, and formatting and drops complete.
- Step 2: the clone compares equal, and the drops complete.

## Status

Implemented. The tests are in `src/value/value_type.rs`, tagged `#[trace("TC-735", "FR-262-AC-2")]`.
