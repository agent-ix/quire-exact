---
id: TC-954
title: "KERNEL_VERSION is public, stable, and an outcome change without a bump fails the golden test"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-406
    type: verifies
---
# TC-954: KERNEL_VERSION is public, stable, and an outcome change without a bump fails the golden test

## Description

Verify FR-406-AC-1 through FR-406-AC-5 through the public API, a golden outcome test and a source inspection.

## Test Procedure

1. For AC-1, use `quire_exact::KERNEL_VERSION` in a `const` item of an integration test (default features, then `test-support`), and in the `no_std` build of `make ci`; read its doc comment.
2. For AC-2, assert the value equals a literal recorded in the golden test, run the test in two feature sets, and search `src/`, `Cargo.toml` and the repo root for `build.rs`, `env!`, `option_env!`, `include_str!` and git invocations feeding the constant; none exist.
3. For AC-3, run the golden test: for each family of FR-406 Behavior 5 (integer add, multiply and division by zero; decimal rounding at a tie; rational reduction; IEEE add with inexact and overflow flags; text comparison at a prefix; quantity unit mismatch; collection insert past a bound; identity construction; a meter denied at the exact limit), format the outcome and its ordered charges to a stable string and compare with the recorded string.
4. For AC-4, call the comparison function of the golden test on the recorded grid with (a) one outcome string altered and (b) one charge altered and assert a difference is reported each time; call the version check with a recorded version of `KERNEL_VERSION - 1` and assert it fails.
5. For AC-5, inspect the doc comment against Behavior 3.

### Mutants

| Mutant | Fails at |
| --- | --- |
| An IEEE add rounds a tie away instead of to even, no bump | the IEEE case of step 3 |
| A division charges one atom fewer, no bump | the charge comparison of step 3 |
| The golden strings are re-recorded but the recorded version is left | the version check of step 4 |
| `KERNEL_VERSION` is derived from the crate version string | the source search of step 2 |

## Expected Results

The constant is readable, identical across builds, and the golden grid passes on the unchanged source. An altered outcome, an altered charge or a stale recorded version is reported as a failure, so an outcome change without a bump cannot pass.

## Status

Not run. No constant or golden test exists.
