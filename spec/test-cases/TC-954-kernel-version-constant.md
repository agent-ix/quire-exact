---
id: TC-954
title: "KERNEL_VERSION is public and the golden test fails on an unintended outcome or charge change"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-406
    type: verifies
---
# TC-954: KERNEL_VERSION is public and the golden test fails on an unintended outcome or charge change

## Description

Verify FR-406-AC-1 through FR-406-AC-5 through the public API, a golden outcome test and inspection.

## Test Procedure

1. For AC-1, use `quire_exact::KERNEL_VERSION` in a `const` item of an integration test (default features, then `test-support`) and in the `no_std` build of `make ci`; confirm the definition is an integer literal and read its doc comment.
2. For AC-2, with `test-support` enabled, run the golden test: for each case of FR-406 Behavior 5 format the outcome and its ordered charges to a stable string and compare with the recorded string.
3. For AC-3, call the comparison function of the golden test on the recorded grid with one outcome string altered, and again with one charge altered, and assert a difference is reported each time.
4. For AC-4, inspect the doc comment against Behavior 3.
5. For AC-5, at review of a change that alters an outcome or charge, confirm the diff bumps the constant, and that no diff lowers it.

### Mutants

| Mutant | Fails at |
| --- | --- |
| An IEEE add rounds a tie away instead of to even | the IEEE case of step 2 |
| A division charges one atom fewer | the charge comparison of step 2 |
| Integer modulo returns the dividend's sign | the modulo case of step 2 |

## Expected Results

The constant is readable and the golden grid passes on the unchanged source. An altered outcome or charge is reported as a failure. Whether a bump accompanies a change is checked at review, not by the test.

## Status

Steps 1 to 3 run in `tests/ir1532_kernel_version.rs` (default and `test-support` builds; the three mutants were applied by hand and each fails the grid at the named case). Steps 4 and 5 are inspection at review.
