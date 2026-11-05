---
id: SR-2026110101322
title: "IR-1327 PR 131 Gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; src/ieee.rs, tests/ir1327_ieee_negate.rs"
review_set: subset
---

## Summary

Ticket: IR-1327. Reviewed PR #131 at frozen head ecb60d53a92c3083f7998094b0c749ed68d4de90 against FR-394-AC-1 through AC-4 and QSpec FR-148-AC-11/13 by reference. No applicable AssuranceProfile exists in the repository spec tree.

## Examined scope

- `FR-394-AC-1` (examined): For each binary32 and binary64 bit pattern identified by [QSpec FR-148-AC-11](ix://agent-ix/quire-specification/FR-148), public `evaluate_ieee(IeeeOperation::Negate(value), mode, meter)` completes with the same width and bits equal to the operand bits XOR the width's sign mask (`0x80000000` or `0x8000000000000000`). `IeeeOperation::Negate(value).kind()` is `IeeeOperationKind::Negate`; a second public negation returns the original bits, including each NaN's signaling bit and payload. The test asserts completed value bits for every input.
- `FR-394-AC-2` (examined): For every rounding mode selected by [QSpec FR-148-AC-11](ix://agent-ix/quire-specification/FR-148), including strict `Exact`, public negation of the referenced patterns completes with the AC-1 sign-bit result, empty flags and provenance recording that mode and the input width. No mode causes a refusal or quiets a signaling NaN.
- `FR-394-AC-3` (examined): On a fresh meter with `IntegerBits = 32`, `ValueOccurrences = 1`, `WorkUnits = 2` and `ResultUnits = 1`, with unrelated limits zero, negating binary32 `0x40000000` completes with bits `0xc0000000`; the only admitted points are `ieee.operands` and `ieee.result-retain`, in that order, consuming two work units and one result unit. The analogous binary64 case uses `0x4000000000000000`, completes with `0xc000000000000000` at `IntegerBits = 64`, and has the same other limits and charge order.
- `FR-394-AC-4` (examined): For the binary32 control in AC-3, `WorkUnits = 1` denies `ieee.result-retain` with `limit_kind = WorkUnits`, `limit = consumed = 1`, `next_charge = 1`, only `ieee.operands` admitted and no result unit or bits. `WorkUnits = 0` denies `ieee.operands` with no admitted charge or bits. An `IntegerBits` limit of 31 also denies `ieee.operands` with `next_charge = 32` and no admitted charge or bits.
- `FR-148-AC-11` (context_only): Negation of `0x3f800000` is `0xbf800000`, of `0x00000000` is `0x80000000`, of `0x80000000` is `0x00000000`, of `0x7f800000` is `0xff800000`, of the signaling NaN `0x7f800001` is `0xff800001` and of the quiet NaN `0xffc00005` is `0x7fc00005`, at binary64 the same patterns widened (`0x7ff0000000000001` is `0xfff0000000000001`); each under `nearest-even`, `toward-zero` and strict `exact` with no flag and no refusal, and `-(-x)` is `x` bit for bit for every listed pattern.
- `FR-148-AC-13` (context_only): Negation under `ScalarLimitsV1 { integer_bits: 32, decimal_digits: 0, scale_expansion: 0, text_input_bytes: 0, text_scalars: 0, normalized_scalars: 0, unit_edges: 0, value_occurrences: 1, work_units: 2, result_units: 1 }` of binary32 `0x3f800000` is `0xbf800000` after exactly `ieee.operands` and `ieee.result-retain`; with `work_units: 1` it is `incomplete { limit_kind: work_units, limit: 1, consumed: 1, next_charge: 1, charge_point: ieee.result-retain }`, and denying `ieee.operands` is incomplete with no bits.

## Verdict

**PASS** — No findings in the changed scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Gap analysis: PASS for the changed requirements. FR-394-AC-1..4 are tagged to executable tests with value, flag, provenance, charge and denial assertions. No new unowned behavior, stub, ignored test or coverage suppression was found. Plan completion: not assessed. Existing repository-wide matrix gaps outside this diff were not reviewed.
