---
id: TC-943
title: "Reservation-charged integer parse, sum, product, exact quotient and gcd"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-396
    type: verifies
---
# TC-943: Reservation-charged integer parse, sum, product, exact quotient and gcd

## Description

Verify FR-396-AC-1 through FR-396-AC-7 through the public `Integer` API with a recording reservation and an allocation counter. Prepare operands, the independent `num-bigint` oracle and both large storage forms outside each measurement window.

## Test Procedure

1. For each of zero, small signed values, the `i128` edges, the first values outside them and multi-limb spellings, run each reserved operation under a provision of `usize::MAX` that records the bytes and count of admitted requests. Compare the value with the oracle and the recorded bytes and count with the allocator's observed total bytes and allocation count.
2. Re-run each under a provision equal to the recorded bytes and assert the same value is admitted with nothing left over.
3. Re-run each under every provision from zero to one byte below the recorded bytes. Assert `Denied` and that the bytes the allocator observed equal the bytes that provision admitted.
4. Parse the non-canonical spellings under a provision of zero and assert `NonCanonical` and no request; parse an `i128` spelling under zero and assert success.
5. Observe comparison, equality, `is_zero` and `is_negative` over every pair with the allocation counter and assert zero allocations; compare adjacent orders with the oracle.
6. Compare the unmetered `add`, `mul`, `exact_div` and `gcd` with the oracle and the unlimited-provision reserved results.

## Expected Results

- Charged bytes equal requested bytes for every operation, one request per block.
- The exact provision admits the identical value and every smaller provision refuses with no value and no allocation beyond what it admitted.
- Reads allocate nothing; unmetered and reserved results agree.

## Status

Implemented.
