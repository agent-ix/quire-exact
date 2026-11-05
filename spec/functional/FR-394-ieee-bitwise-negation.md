---
id: FR-394
title: IEEE negation flips only the sign bit
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-specification/FR-148
    type: implements
---
# FR-394: IEEE negation flips only the sign bit

## Description

When evaluating negation of a binary32 or binary64 IEEE value, the exact kernel SHALL
return a value of the same width whose bits differ from the operand only
at the sign bit.

## Inputs

- One `IeeeValue` of width `Binary32` or `Binary64`.
- A selected `RoundingMode`, including `Exact`.
- The caller's mutable `Meter`.

## Outputs

- A public `IeeeOperation::Negate` evaluated through `evaluate_ieee`, with a
  completed `IeeeResult` carrying the result value, empty flags and the selected
  width and rounding provenance.
- If the meter denies a charge, its original `Incomplete` outcome, with no
  completed bits or flags.

## Behavior

1. The kernel SHALL expose unary `Negate` in the public IEEE operation and
   operation-kind vocabularies and evaluate it for either supported width.
2. The kernel SHALL flip only bit 31 for binary32 or bit 63 for binary64.
   The kernel SHALL preserve all exponent and fraction bits, including the NaN payload
   and quiet/signaling bit. Positive and negative zero and infinity therefore
   exchange signs; a signaling NaN remains signaling.
3. The kernel SHALL complete negation under every selected rounding mode,
   including strict `Exact`, without rounding, refusal or IEEE exception flags.
   The completed result SHALL retain the selected rounding provenance.
4. The kernel SHALL charge the supplied meter at `ieee.operands` with
   `IntegerBits` equal to the source width and `ValueOccurrences` equal to one,
   followed by `ieee.result-retain` with one `ResultUnits`.
   Each admitted point SHALL consume one `WorkUnits`; no intermediate, rounding or other charge
   point belongs to negation.
5. If either charge is denied, then the kernel SHALL return `Incomplete` with
   the meter's denial details and only the prior admitted charge prefix.
   The kernel SHALL expose no completed result on that path.

The language-level behavior and vectors are specified by
[QSpec FR-148-AC-11](ix://agent-ix/quire-specification/FR-148) and
[QSpec FR-148-AC-13](ix://agent-ix/quire-specification/FR-148). This requirement
owns the kernel operation and its native meter effects; it does not define a
new source-language operation identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-394-AC-1 | Through the public `evaluate_ieee` API, binary32 `0x3f800000`, `0x00000000`, `0x80000000`, `0x7f800000`, signaling NaN `0x7f800001` and quiet NaN `0xffc00005` complete respectively as `0xbf800000`, `0x80000000`, `0x00000000`, `0xff800000`, `0xff800001` and `0x7fc00005`. Binary64 `0x3ff0000000000000`, `0x0000000000000000`, `0x8000000000000000`, `0x7ff0000000000000`, signaling NaN `0x7ff0000000000001` and quiet NaN `0xfff8000000000005` complete respectively as `0xbff0000000000000`, `0x8000000000000000`, `0x0000000000000000`, `0xfff0000000000000`, `0xfff0000000000001` and `0x7ff8000000000005`. For every vector, `Negate` appears as the public operation kind, the output width is unchanged, flags are empty, and applying negation twice restores the original bits. | Test |
| FR-394-AC-2 | Under `NearestEven`, `TowardZero` and `Exact`, each vector in AC-1 has the same expected bits and empty flags, with result provenance recording the selected width and rounding mode. In particular, signaling NaNs stay signaling and strict `Exact` completes without refusal. | Test |
| FR-394-AC-3 | On a fresh meter with `IntegerBits = 32`, `ValueOccurrences = 1`, `WorkUnits = 2` and `ResultUnits = 1`, with unrelated limits zero, negating binary32 `0x3f800000` completes as `0xbf800000`; the only admitted points are `ieee.operands` and `ieee.result-retain`, in that order, consuming two work units and one result unit. The analogous binary64 case succeeds at `IntegerBits = 64` with the same other limits and charge order. | Test |
| FR-394-AC-4 | For the binary32 control in AC-3, `WorkUnits = 1` denies `ieee.result-retain` with `limit_kind = WorkUnits`, `limit = consumed = 1`, `next_charge = 1`, only `ieee.operands` admitted and no result unit or bits. `WorkUnits = 0` denies `ieee.operands` with no admitted charge or bits. An `IntegerBits` limit of 31 also denies `ieee.operands` with `next_charge = 32` and no admitted charge or bits. | Test |

## Dependencies

- [QSpec FR-148](ix://agent-ix/quire-specification/FR-148) owns the language's
  binary32 and binary64 negate semantics and its acceptance criteria 11 and 13.
- The existing [FR-366](./FR-366-ieee-exceptional-value-semantics.md) describes
  IEEE exceptional-value bit behavior in the kernel.
