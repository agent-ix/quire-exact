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
| FR-359-AC-2 | After admitting a charge of `u64::MAX` result units, a charge requesting one result unit returns `Incomplete` with `limit_kind = ResultUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`. Its work unit is also unconsumed and its admission is unrecorded. | Test |
| FR-359-AC-3 | A cumulative work or result charge whose exact amount is `u64::MAX + 1` returns `Incomplete` with that exact `next_charge`, naming its counter and charge point. It neither wraps to a smaller amount nor changes any counter or admission count. | Test |

## Status

Planned.

## Dependencies

- None. This requirement concerns the existing `Meter::charge` and `Charge` accounting contract in `src/accounting.rs`.
