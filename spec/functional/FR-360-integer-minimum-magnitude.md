---
id: FR-360
title: "Integer absolute value retains the magnitude of i64 minimum"
type: FR
relationships: []
---
# FR-360: Integer absolute value retains the magnitude of i64 minimum

## Description

The `quire-exact` kernel SHALL return the exact positive magnitude `2^63` when `Integer::abs` is called on an `Integer` equal to `i64::MIN`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-360-AC-1 | `Integer::from(i64::MIN).abs()` equals the positive integer 9,223,372,036,854,775,808 (`2^63`), is nonnegative, and differs from the original negative value. It neither overflows nor returns `i64::MIN`. | Test |

## Status

Planned.

## Dependencies

- [TC-908](../test-cases/TC-908-integer-minimum-magnitude.md) is the test-case home for this criterion. The operation is in `src/integer.rs`.
