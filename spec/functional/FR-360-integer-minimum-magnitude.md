---
id: FR-360
title: "Integer absolute value retains the magnitude of i64 minimum"
type: FR
relationships: []
---
# FR-360: Integer absolute value retains the magnitude of i64 minimum

## Description

When `Integer::abs` receives the value `i64::MIN`, the `quire-exact` kernel SHALL return its exact positive magnitude as an arbitrary-precision `Integer`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-360-AC-1 | `Integer::from(i64::MIN).abs()` equals the positive integer 9,223,372,036,854,775,808 (`2^63`), is nonnegative, and differs from the original negative value. It neither overflows nor returns `i64::MIN`. | Test |

## Status

Planned.

## Dependencies

- None. This requirement concerns the existing `Integer::abs` operation in `src/integer.rs`.
