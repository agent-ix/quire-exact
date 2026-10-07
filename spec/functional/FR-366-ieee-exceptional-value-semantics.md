---
id: FR-366
title: "IEEE exceptional values preserve the stated bits and order"
type: FR
relationships: []
---
# FR-366: IEEE exceptional values preserve the stated bits and order

## Description

When evaluating or converting IEEE exceptional values, the kernel SHALL preserve NaN selection, payload representability and signed-zero loss according to the public value contracts below. The public total-order key SHALL distinguish signed zeros and NaN classes.

## Behavior

Arithmetic selects the leftmost NaN in operand order, preserves its sign and payload, and quiets it. Any signaling NaN operand raises Invalid, including a later operand that is not selected. Width conversion refuses a payload that cannot fit below the target quiet bit. Exact rational zero has no sign; conversion records the loss of a negative IEEE zero. Within one width, total-order keys place negative quiet NaNs, negative signaling NaNs, negative infinity, negative finite values, negative zero, positive zero, positive finite values, positive infinity, positive signaling NaNs and positive quiet NaNs in that order.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-366-AC-1 | For same-width Add with a positive quiet NaN of payload 5 and a negative signaling NaN of payload 9 in either operand order, the result bits are the first NaN quieted and Invalid is present. Two quiet NaNs select the first without Invalid. For FusedMultiplyAdd with a quiet first NaN, finite second operand and signaling third NaN, the first NaN remains selected and Invalid is present. Exercise both binary widths. | Test |
| FR-366-AC-2 | Converting a binary64 NaN with payload `2^22` or `2^30` to binary32 returns `Refused(IeeeNanPayloadNotRepresentable { source: Binary64, target: Binary32 })` without truncating the payload or retaining a result unit. | Test |
| FR-366-AC-3 | Converting either signed binary32 zero to the rational domain with numerator interval `[-1, 1]` and denominator interval `[1, 1]` completes to canonical `0/1`. Positive zero has no loss; negative zero has exactly `IeeeExactLoss::NegativeZeroSign`. | Test |
| FR-366-AC-4 | For each width, distinct bit patterns representing the ten classes in Behavior have strictly increasing `total_order_key()` values in that stated order, including quiet/signaling NaNs of both signs and both zeros. Positive NaN payloads of the same class ascend by payload and negative ones descend. The public TotalOrder comparison agrees with the key order for these pairs; NumericEqual considers the two zeros equal despite their distinct keys. | Test |

## Status

Planned for IR-673 owner evidence. Existing source behavior requires direct owner tests.

## Dependencies

- [TC-914](../test-cases/TC-914-ieee-exceptional-value-semantics.md) is the verification home. Verification uses only the public `quire_exact` API; no downstream evaluator or copied tests are required.
