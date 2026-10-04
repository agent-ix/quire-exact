---
id: TC-407
title: "The kernel Undefined has no PreconditionFalse variant"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-090
    type: verifies
---
# TC-407: The kernel Undefined has no PreconditionFalse variant

## Description

Verify FR-090-AC-11 by inspection: the kernel's closed outcome set holds no cause that belongs to a caller.

Scope: FR-090-AC-11.

## Test Procedure

1. Read `src/outcome.rs`'s `Undefined` enum.

## Expected Results

- Step 1 finds no a `PreconditionFalse` variant.

## Status

Inspected.
