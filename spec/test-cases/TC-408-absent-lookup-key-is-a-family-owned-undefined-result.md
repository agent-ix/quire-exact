---
id: TC-408
title: "The kernel Undefined has no AbsentKey variant"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-090
    type: verifies
---
# TC-408: The kernel Undefined has no AbsentKey variant

## Description

Verify FR-090-AC-12 by inspection: the kernel's closed outcome set holds no cause that belongs to a caller.

Scope: FR-090-AC-12.

## Test Procedure

1. Read `src/outcome.rs`'s `Undefined` enum.

## Expected Results

- Step 1 finds no an `AbsentKey` variant.

## Status

Inspected.
