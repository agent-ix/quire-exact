---
id: TC-428
title: "A kernel refusal returns its code and cause"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-096
    type: verifies
---
# TC-428: A kernel refusal returns its code and cause

## Description

Verify FR-096-AC-8: every catalogued kernel refusal returns its code and cause
spelling, and every typed `CheckedInvariant` returns neither. This catches a
cause that returns another cause's code or turns an internal fault into an
ordinary refusal.

Scope: FR-096-AC-8.

## Test Procedure

1. For each kernel refusal in FR-096-AC-8's list, build it and read `Refusal::code()` and `Refusal::cause()`.
2. Build a `CheckedInvariant` with each of FR-369's twenty-three typed causes and
   read `Refusal::code()` and `Refusal::cause()`.

## Expected Results

- Step 1: each refusal returns the code and cause of its row.
- Step 2: no code and no cause.

## Status

The existing `src/outcome.rs` test, tagged `#[trace("TC-428",
"FR-096-AC-8")]`, covers the ordinary table and the former unit variant.
The twenty-three typed step-2 cases are planned for IR-707.
