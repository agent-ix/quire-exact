---
id: TC-914
title: "IEEE NaNs signed zeros and total-order keys"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-366
    type: verifies
---
# TC-914: IEEE NaNs signed zeros and total-order keys

## Description

Verify [FR-366](../functional/FR-366-ieee-exceptional-value-semantics.md) through the public owner API.

## Test Procedure

1. Build NaNs from independently specified IEEE bit fields, rather than owner decoding helpers. Call Add in both orders, two-quiet Add and the stated FusedMultiplyAdd; inspect exact result bits and flags.
2. Narrow binary64 quiet NaNs of both signs with fitting payload `2^22 - 1` to binary32 and assert exact sign, quiet bit, payload and retained result. Then narrow both oversized payloads `2^22` and `2^30` and inspect typed refusals and result counters.
3. Convert each signed binary32 zero to the stated rational domain and inspect numerator, denominator and loss.
4. Construct the ten total-order classes for each width plus two payloads within each NaN sign/class. Compare their public keys with the independent class/payload order and call public TotalOrder/NumericEqual comparisons on the stated cases.

## Expected Results

NaN selection, signaling flags, preservation of fitting payloads, oversized-payload refusals, zero value/loss and strict key order match the respective ACs. No numeric floating-point host operation is used as an oracle for bit patterns.

## Status

Planned for IR-673; executable evidence has not yet been added.
