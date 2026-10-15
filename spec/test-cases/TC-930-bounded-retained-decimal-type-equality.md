---
id: TC-930
title: "Bounded retained Decimal type equality checks both sides"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-382
    type: verifies
---
# TC-930: Bounded retained Decimal type equality checks both sides

## Description

Verify [FR-382](../functional/FR-382-bounded-retained-decimal-type-equality.md)
through the production borrowed `DecimalType::try_eq_bounded` operation.
Construct valid retained types before observing the helper.

## Test Procedure

1. Compare distinct types with arithmetically equal signed endpoints. Vary
   lower, upper, minimum scale, maximum scale and rounding independently;
   derive equality from retained fields and check both borrowed inputs are
   unchanged.
2. For each of four endpoint positions, construct valid types with that
   endpoint exactly `B` bits and then `B+1` bits. Assert the typed position,
   measured bits and limit, including a small expected type with oversized
   actual endpoints, two oversized endpoints in different positions, and
   unlike scales that must not hide an oversized endpoint. With `B = 0`,
   assert an all-zero type refuses at expected lower with measured one bit.
3. Observe production limb-pair steps on equal and late-unequal types at
   32-bit limb, inline and Large representation boundaries. Assert the
   `2 ceil(B/32)` bound and no production allocation. Inspect the borrowed
   source path for input-sized clone, conversion, scratch or fallback.
4. Cancel the original handle before both in-bound and oversized calls, then
   cancel it after a late observed limb step. Assert original cause, no next
   step or Boolean result, and successful retry with a fresh handle.

## Expected Results

Completed results match retained-field equality without mutation. Exact-B
endpoints complete; the first over-B endpoint has its precise typed refusal.
Work and storage stay within the measured envelope, and cancellation retains
the original cause. This allocation-free route has no synthetic resource
failure fixture.

## Status

PLANNED/UNRUN; no kernel acceptance result is claimed.
