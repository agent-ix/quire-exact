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

Verify FR-096-AC-8: every kernel cause returns its code and cause spelling, and a `CheckedInvariant` returns no code. This catches a cause that returns another cause's code or drops the domain it was raised for.

Scope: FR-096-AC-8.

## Test Procedure

1. For each of the twelve kernel causes, build the refusal with the target FR-096-AC-8 names and read `Refusal::code()` and `Refusal::cause()`.
2. Build a `CheckedInvariant` and read `Refusal::code()`.

## Expected Results

- Step 1: each refusal returns the code and cause of its row, and carries the named target.
- Step 2: no code.

## Status

Implemented. The test is in `src/outcome.rs`, tagged `#[trace("TC-428", "FR-096-AC-8")]`.
