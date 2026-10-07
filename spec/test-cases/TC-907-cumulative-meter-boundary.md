---
id: TC-907
title: "Cumulative meter charges at and beyond the u64 boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-359
    type: verifies
---
# TC-907: Cumulative meter charges at and beyond the u64 boundary

## Description

Verify [FR-359](../functional/FR-359-cumulative-meter-boundary.md), AC-1 through AC-7, through the existing `Meter::charge` seam.

## Test Procedure

1. Use `u64::MAX` cumulative limits, admit work of `u64::MAX - 1` and then one, snapshot all counters and admissions, then request one more work unit.
2. On a fresh meter with both cumulative limits at `u64::MAX`, admit `u64::MAX` result units, snapshot, then request one more result unit.
3. On separate fresh meters, request `u64::MAX + 1` work units and `u64::MAX + 1` result units as exact `Integer` amounts.
4. Admit an `integer_bits` high-water size of 8 and one result unit. Attempt a charge with size 16 and one result unit under a result limit of 1; snapshot every counter, admissions and test-support log before and after.

5. For each limit kind, construct a charge where that counter is first short; repeat simultaneous-short semantic sizes in reversed attachment order and include work/result shortages. Snapshot all state on a later-counter refusal; on a separate meter install a later occurrence denial and prove the ordinary refusal does not advance it.
6. Read every counter on fresh, charged and stopped meters and compare with independently tracked high-water sizes and cumulative totals.

## Expected Results

The charge reaching the work limit succeeds. Every later charge refuses at the counter, point and exact amount in its corresponding AC without wrapping or altering the pre-refusal state, including the high-water size on the result refusal. The first shortage follows field order independently of size attachment order. Every consumed reader is total and equals admitted accounting.

## Status

Planned for the IR-653 CODE stage.
