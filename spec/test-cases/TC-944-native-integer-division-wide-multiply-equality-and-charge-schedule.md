---
id: TC-944
title: "Native division, i64 multiply, equality and charge schedule against the Integer path"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-397
    type: verifies
---
# TC-944: Native division, i64 multiply, equality and charge schedule against the Integer path

## Description

Verify FR-397-AC-1 through FR-397-AC-5 through the public API by running each native entry and the existing `Integer` entry on the same values with meters that capture every charge, and comparing outcomes, captured events and counters.

## Test Procedure

1. For each profile, member and edge pair, run `divide` / `modulo` and the native entry, with and without a result interval, at unlimited limits; compare the outcome and `take_charge_capture` events and counters. Repeat with `work_units` set one below the unlimited spend.
2. For the i64 multiply boundary pairs, compare with `evaluate_integer_arithmetic`; for rejected pairs assert the admission error and an empty capture.
3. Compare native equality with `planned_equality` over `Value::Integer`, and `evaluate_boolean` over every truth-table row.
4. Compare `evaluate_native_boolean_equality` with `planned_equality` over `Value::Boolean` for the four pairs, at unlimited limits, at every `value_occurrences`, `work_units` and `result_units` limit from zero to its unlimited spend (the schedule spends the other seven limit kinds as zero), and with an injected denial at each of the four `equality.*` points, comparing outcome, captured events and counters.
5. Run `charge_native_integer_schedule` and the matching evaluator on separate meters and compare captures and counters, at sufficient limits and one below each charge.

## Expected Results

- Every outcome, event and counter is identical between each native entry and the existing path.
- Rejected multiply operands charge nothing.
- Exception: the quotient `i128::MIN / -1` with no result interval is refused by the native entry as `CheckedInvariant(NativeIntegerCapacityBreach)` after the same charges as `divide` up to `*.domain`, where `divide` completes; consumers fall back.
- The charge-only schedule is success-path only, for use after bounds prove the result and domain fit.

## Status

Implemented.
