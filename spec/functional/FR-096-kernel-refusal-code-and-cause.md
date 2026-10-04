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
| FR-096-AC-8 | Each kernel refusal below returns the code and cause spelling named for it from `Refusal::code()` and `Refusal::cause()`, and a kernel `CheckedInvariant` returns neither. `InexactDecimal`: `inexact_decimal`, `nonzero-discarded-digit`. `DecimalOutOfDomain`, `ModuloOutOfDomain`, `TextLengthOutOfDomain`, `IntegerOutOfDomain`, `RationalOutOfDomain`, `IeeeRationalOutOfDomain`: `<name in snake case>`, `outside-domain`. `DivisionOutOfDomain`: `division_out_of_domain` with `quotient-outside-domain` for the quotient member or `remainder-outside-domain` for the remainder member. `IeeeNotExact`: `ieee_not_exact`, `rounding-required`. `IeeeNanPayloadNotRepresentable`: `ieee_nan_payload_not_representable`, `payload-exceeds-target`. `ForeignReference`: `foreign_reference`, `foreign-universe`. `CardinalityOutOfBound`: `cardinality_out_of_bound` with `above-maximum` or `below-minimum`. | Test (TC-428) |

## Status

Implemented.

## Dependencies

- None. The caller's behaviour under the same id lives in `agent-ix/quire-spec-language`.
