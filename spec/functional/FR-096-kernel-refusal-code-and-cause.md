---
id: FR-096
title: "A kernel refusal names its code and cause"
type: FR
relationships: []
---
# FR-096: A kernel refusal names its code and cause

## Description

A kernel `Refusal` SHALL return a stable code and cause spelling from `Refusal::code()` and `Refusal::cause()`, so a caller that builds a refusal record maps a kernel cause to its catalog without reading a message. Building the record, with its category and locus, is the caller's and keeps the id `FR-096` in the repository that owns it.

## Use case

A caller refuses an out-of-domain result and writes a record that names the code and the cause. It reads both from the kernel refusal.

## Behavior

1. **Code and cause.** Each of the twelve kernel refusal causes returns its catalog code from `Refusal::code()` and its cause spelling from `Refusal::cause()`. `CheckedInvariant` is an internal fault and returns no code.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-096-AC-8 | For each of the twelve kernel causes, `Refusal::code()` and `Refusal::cause()` return the code and the cause the key table names for it, and each variant carries the target domain or width the caller's record renders: `IntegerOutOfDomain` for target `Int[-5, 9]`, `DecimalOutOfDomain` for `Decimal[-100, 100; 0, 2]`, `RationalOutOfDomain` for `Rational[-9, 9; 1, 9]`, `TextLengthOutOfDomain` for `Text[1, 8; nfc]`, `InexactDecimal` for an integer target `Int[0, 9]`, `IeeeNotExact` for a `binary32` result whose `nearest-even` flags are inexact and overflow, and `IeeeNanPayloadNotRepresentable` for a `binary64` to `binary32` conversion. A kernel `CheckedInvariant` returns no code. | Test (TC-428) |

## Status

Implemented.

## Dependencies

- None. The caller's behaviour under the same id lives in `agent-ix/quire-spec-language`.
