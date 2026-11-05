---
id: SR-2026110101319
title: IR-1327 PR 130 Scope and boundary
type: SpecReview
analysis: scope-boundary
scope: agent-ix/quire-exact; spec/functional/FR-394-ieee-bitwise-negation.md
review_set: subset
---

## Summary

FR-394 adds a kernel-owned unary IEEE operation. Examined its statement, four ACs, QSpec FR-148-AC-11/13 at origin/main e85daed58afbd89a21ebbbf98f10c4a4ab83e98439, and the current kernel API and meter conventions. No applicable AssuranceProfile was found. The four criteria are planned and currently untagged in the computed matrix.

## Verdict

**PASS — no finding in this lens.**

## Examined scope

- `FR-394` (examined): When evaluating negation of a binary32 or binary64 IEEE value, the exact kernel SHALL return a value of the same width whose bits differ from the operand only at the sign bit.
- `FR-394-AC-1` (examined): Through the public `evaluate_ieee` API, binary32 `0x3f800000`, `0x00000000`, `0x80000000`, `0x7f800000`, signaling NaN `0x7f800001` and quiet NaN `0xffc00005` complete respectively as `0xbf800000`, `0x80000000`, `0x00000000`, `0xff800000`, `0xff800001` and `0x7fc00005`. Binary64 `0x3ff0000000000000`, `0x0000000000000000`, `0x8000000000000000`, `0x7ff0000000000000`, signaling NaN `0x7ff0000000000001` and quiet NaN `0xfff8000000000005` complete respectively as `0xbff0000000000000`, `0x8000000000000000`, `0x0000000000000000`, `0xfff0000000000000`, `0xfff0000000000001` and `0x7ff8000000000005`. Fo
- `FR-394-AC-2` (examined): Under `NearestEven`, `TowardZero` and `Exact`, each vector in AC-1 has the same expected bits and empty flags, with result provenance recording the selected width and rounding mode. In particular, signaling NaNs stay signaling and strict `Exact` completes without refusal.
- `FR-394-AC-3` (examined): On a fresh meter with `IntegerBits = 32`, `ValueOccurrences = 1`, `WorkUnits = 2` and `ResultUnits = 1`, with unrelated limits zero, negating binary32 `0x3f800000` completes as `0xbf800000`; the only admitted points are `ieee.operands` and `ieee.result-retain`, in that order, consuming two work units and one result unit. The analogous binary64 case succeeds at `IntegerBits = 64` with the same other limits and charge order.
- `FR-394-AC-4` (examined): For the binary32 control in AC-3, `WorkUnits = 1` denies `ieee.result-retain` with `limit_kind = WorkUnits`, `limit = consumed = 1`, `next_charge = 1`, only `ieee.operands` admitted and no result unit or bits. `WorkUnits = 0` denies `ieee.operands` with no admitted charge or bits. An `IntegerBits` limit of 31 also denies `ieee.operands` with `next_charge = 32` and no admitted charge or bits.
- `FR-148-AC-11` (context_only): Negation of `0x3f800000` is `0xbf800000`, of `0x00000000` is `0x80000000`, of `0x80000000` is `0x00000000`, of `0x7f800000` is `0xff800000`, of the signaling NaN `0x7f800001` is `0xff800001` and of the quiet NaN `0xffc00005` is `0x7fc00005`.
- `FR-148-AC-13` (context_only): Negation under `ScalarLimitsV1` of binary32 `0x3f800000` is `0xbf800000` after exactly `ieee.operands` and `ieee.result-retain`; with `work_units: 1` it is incomplete at `ieee.result-retain`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Analysis

The text allocates native bit transformation and meter effects to quire-exact and source-language identity to QSpec. The duplicate examples are recorded by the base/code-review methods.
