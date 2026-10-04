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

1. For each kernel refusal in FR-096-AC-8's list, build it and read `Refusal::code()` and `Refusal::cause()`.
2. Build a `CheckedInvariant` and read `Refusal::code()` and `Refusal::cause()`.
3. Build a value refusal with a distinct target (`Int[-5, 9]`, `Decimal[-100, 100; 0, 2]`, `Rational[-9, 9; 1, 9]`, `Text[1, 8; nfc]`, an integer target `Int[0, 9]` for `InexactDecimal`, a `binary32` width with its flags, a `binary64` to `binary32` conversion) and read the target back from the variant.

## Expected Results

- Step 1: each refusal returns the code and cause of its row.
- Step 2: no code and no cause.
- Step 3: each variant returns the target it was built with.

## Status

Implemented. The tests are in `src/outcome.rs`, tagged `#[trace("TC-428", "FR-096-AC-8")]`.
