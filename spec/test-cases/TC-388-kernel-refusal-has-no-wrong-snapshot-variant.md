---
id: TC-388
title: "The kernel Refusal has no wrong-snapshot variant"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-090
    type: verifies
---
# TC-388: The kernel Refusal has no wrong-snapshot variant

## Description

Verify FR-090-AC-7 by inspection: the kernel's closed outcome set holds no cause that belongs to a caller.

Scope: FR-090-AC-7.

## Test Procedure

1. Read `src/outcome.rs`'s `Refusal` enum.

## Expected Results

- Step 1 finds no variant whose payload is a wrong-snapshot cause.

## Status

Inspected.
