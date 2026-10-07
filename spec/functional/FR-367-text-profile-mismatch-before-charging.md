---
id: FR-367
title: "Text comparison rejects distinct profiles before charging"
type: FR
relationships: []
---
# FR-367: Text comparison rejects distinct profiles before charging

## Description

When `compare_text` receives admitted text values with distinct profiles, the kernel SHALL return `Err(IllTyped { cause: DistinctTextProfiles })` before any meter charge.

## Behavior

The profile check precedes comparison and budget enforcement even when the text sequences are identical or every comparison limit is zero. The fault is an `IllTyped` result beside `Outcome`, not an evaluator outcome. Operand admission is measured on separate meters from the comparison.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-367-AC-1 | Admit the same UTF-8 text "a" once with Nfc and once with BinaryUtf8 using separate sufficient meters. On a fresh comparison meter whose ten limits are zero, public `compare_text(Equal, left, right)` returns precisely `Err(IllTyped { cause: DistinctTextProfiles })`. Every consumed counter and admission count remains zero; under `test-support` the charge log is empty and truncation is false. | Test |

## Status

PR #9 adds executable Trace bindings for AC-1. Run `quire matrix` for the current criterion-to-test mapping.

## Dependencies

- [TC-915](../test-cases/TC-915-text-profile-mismatch-before-charging.md) is the verification home. Verification uses only the public `quire_exact` API; no downstream evaluator or copied tests are required.
