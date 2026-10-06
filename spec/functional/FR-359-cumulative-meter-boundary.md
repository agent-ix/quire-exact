---
id: FR-359
title: "Cumulative meter counters refuse at the u64 boundary"
type: FR
relationships: []
---
# FR-359: Cumulative meter counters refuse at the u64 boundary

## Description

When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an `Incomplete` record for the first unavailable counter without wrapping or changing any consumed counter or admission count.

## Behavior

`work_units` and `result_units` accumulate across admitted charges. An addition that equals its limit is admissible; an addition beyond its limit, including one whose exact amount cannot fit `u64`, is refused. A refused charge preserves the previously admitted state even when its work amount fits but its result amount does not. Semantic-size counters remain high-water values.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-359-AC-1 | With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX - 1` followed by one work unit reaches `u64::MAX` exactly. The next one-unit charge returns `Incomplete` at its named point with `limit_kind = WorkUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; all consumed counters and admission count stay at their pre-refusal values. | Test |
| FR-359-AC-2 | With `work_units = result_units = u64::MAX`, admit one charge requesting `u64::MAX` result units. Its work consumption is 1. A second charge requesting one result unit returns `Incomplete` at its named point with `limit_kind = ResultUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; work remains 1 and admission count remains 1. | Test |
| FR-359-AC-3 | With both cumulative limits at `u64::MAX`, a work charge whose exact amount is `u64::MAX + 1` returns `Incomplete` with `limit_kind = WorkUnits` and `next_charge = u64::MAX + 1`; no counter or admission count changes. | Test |
| FR-359-AC-4 | With both cumulative limits at `u64::MAX`, a charge with one work unit and `u64::MAX + 1` result units returns `Incomplete` with `limit_kind = ResultUnits` and `next_charge = u64::MAX + 1`; no counter or admission count changes. | Test |
| FR-359-AC-5 | First admit an `integer_bits` high-water size of 8 and one result unit under limits of at least 16 bits, `u64::MAX` work units and one result unit. Then attempt a charge carrying `integer_bits = 16` and one result unit. The result refusal leaves `integer_bits = 8`, work units, result units and admission count at their pre-refusal values; under `test-support`, the admitted-charge log is unchanged too. | Test |

## Status

Planned.

## Dependencies

- [TC-907](../test-cases/TC-907-cumulative-meter-boundary.md) is the test-case home for these criteria. This requirement concerns the existing `Meter::charge` and `Charge` accounting contract in `src/accounting.rs`.
