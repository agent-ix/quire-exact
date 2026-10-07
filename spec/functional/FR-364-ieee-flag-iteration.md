---
id: FR-364
title: "IEEE exception flags iterate in vocabulary order"
type: FR
relationships: []
---
# FR-364: IEEE exception flags iterate in vocabulary order

## Description

The kernel SHALL iterate an `IeeeFlags` set in `IeeeFlag::ALL` vocabulary order, independently of insertion order.

## Behavior

`IeeeFlag::ALL` orders invalid, divide-by-zero, overflow, underflow and inexact. Iteration includes each present flag once and omits absent flags; the empty set has no items.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-364-AC-1 | For every subset of the five public flags, collect it into `IeeeFlags` in forward and reverse vocabulary order. Both `iter()` results equal that subset filtered from `IeeeFlag::ALL`, without duplicates. The empty set iterates empty and the full set iterates all five flags in the stated order. | Test |

## Status

PR #9 adds executable Trace bindings for AC-1. Run `quire matrix` for the current criterion-to-test mapping.

## Dependencies

- [TC-912](../test-cases/TC-912-ieee-flag-iteration.md) is the verification home. Verification uses only the public `quire_exact` API; no downstream evaluator or copied tests are required.
