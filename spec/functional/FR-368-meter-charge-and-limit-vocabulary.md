---
id: FR-368
title: "Public meter charge and limit vocabularies are complete and uniquely spelled"
type: FR
relationships: []
---
# FR-368: Public meter charge and limit vocabularies are complete and uniquely spelled

## Description

The `quire-exact` meter SHALL expose a complete, uniquely spelled public `ChargePoint` vocabulary and a `LimitKind` vocabulary in `ScalarLimits` field order.

## Behavior

`ChargePoint::ALL` and `LimitKind::ALL` enumerate their respective public enum variants once. Each `as_str()` returns its documented name. `ChargePoint::from_code()` resolves precisely those spellings and rejects unknown spellings. The public limit-kind order is the order in which a charge tests semantic sizes before cumulative work and results.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-368-AC-1 | Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL`, has a unique documented spelling from `as_str()` and round-trips through `from_code()`; an unknown spelling returns `None`. Exhaustive variant matching makes an added but unaccounted variant fail the check. | Test |
| FR-368-AC-2 | Every public `LimitKind` variant occurs exactly once in `LimitKind::ALL`, has a unique documented `as_str()` spelling, and the sequence is exactly the ten `ScalarLimits` fields: integer bits, decimal digits, scale expansion, text input bytes, text scalars, normalized scalars, unit edges, value occurrences, work units and result units. Exhaustive variant matching makes an added but unaccounted variant fail the check. | Test |

## Status

AC-1 and AC-2 are planned for IR-673 and untagged.

## Dependencies

- [TC-916](../test-cases/TC-916-meter-charge-and-limit-vocabulary.md) verifies both vocabulary criteria through the existing public enum APIs. [FR-359](./FR-359-cumulative-meter-boundary.md) owns first-short-counter behavior that uses this field order.
