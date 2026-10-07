---
id: FR-365
title: "Decimal ties follow the selected mode and default to Exact"
type: FR
relationships: []
---
# FR-365: Decimal ties follow the selected mode and default to Exact

## Description

When rounding a decimal to scale zero, the kernel SHALL apply the selected `RoundingMode`.

An omitted mode represented by `RoundingMode::default()` SHALL be `Exact` and refuse any nonzero discarded digit.

## Behavior

For +0.5 and -0.5, Exact refuses both; TowardZero produces 0/0, TowardPositive 1/0, TowardNegative 0/-1, NearestEven 0/0 and NearestAway 1/-1. Target bounds include these results. A strict refusal is typed `InexactDecimal` and does not retain a result.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-365-AC-1 | Round retained `(5, 1)` and `(-5, 1)` to a scale-zero target with coefficient bounds -10 through 10 under each of the six modes. Values or `Refused(InexactDecimal { target })` match the tie table in Behavior; each completed representation has scale zero, and refusals consume no result unit. | Test |
| FR-365-AC-2 | `RoundingMode::default()` and the mode read back from a target constructed with it are `Exact`. Rounding retained `(1, 1)` to that scale-zero target returns `Refused(InexactDecimal { target })`, rather than silently rounding 0.1 to zero, with no retained result. | Test |

## Status

PR #9 adds executable Trace bindings for AC-1 and AC-2. Run `quire matrix` for the current criterion-to-test mapping.

## Dependencies

- [TC-913](../test-cases/TC-913-decimal-rounding-ties-and-default.md) is the verification home. Verification uses only the public `quire_exact` API; no downstream evaluator or copied tests are required.
