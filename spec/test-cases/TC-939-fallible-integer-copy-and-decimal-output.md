---
id: TC-939
title: "Fallible integer copy and bounded canonical decimal output"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-390
    type: verifies
---
# TC-939: Fallible integer copy and bounded canonical decimal output

## Description

Verify [FR-390](../functional/FR-390-fallible-integer-copy-and-decimal-output.md) through the exact kernel's public `Integer` API and a private reservation-failure seam. Prepare all test inputs and independent expected spellings before arming that seam.

## Test Procedure

1. Prepare zero, `1`, `-1`, one positive and one negative decimal literal of at least 100 digits whose first digit is nonzero and whose remaining digits vary. Parse the two large literals before testing; retain their literal bytes as the independent output oracle. Call `try_clone` for each, compare numeric equality with the source and verify the returned value can be dropped independently. Inspect the source again.
2. Call `try_to_decimal_string` with a bound equal to the expected byte length for every input. Compare the returned bytes against the literal oracle and the existing `Display` spelling. Repeat with a bound one byte smaller and assert the exact `ByteLimit { required, limit }` values, including the minus sign for the negative control. Confirm no output is returned on refusal.
3. Use the crate's test-only callback around each production reservation site in `try_clone` and `try_to_decimal_string`. For each site, inject one refusal after input preparation, assert `Allocation { requested }`, no copy or decimal output, and an unchanged source. Compare `requested` with the actual additional element count times element size, or with the output byte reservation. Exercise zero or inline paths with a callback that fails if called where no reservation is required.
4. Exercise the same private checked-count calculations used by production with synthetic near-`usize::MAX` counts and element sizes; assert the exact reservation product at the representable boundary and `requested = usize::MAX` above it. Supply a synthetic overflowing output length to its production count helper under both a finite bound and `max_bytes = usize::MAX`; assert `OutputSizeOverflow` in both cases, never `ByteLimit { required: usize::MAX, limit: usize::MAX }`. Verify by source inspection that every allocating operation on the public fallible routes uses checked arithmetic and a fallible reservation, with no infallible radix conversion, clone or collection.
5. Compile a client that borrows one `Integer`, obtains an owned copy and decimal text, and distinguishes allocation, byte-limit failure and output-size overflow using typed variants. Verify the existing `Clone`, `Display`, parsing and arithmetic signatures and successful values remain available.

## Expected Results

- Copies preserve numeric value, own their storage independently and leave each source unchanged; every injected copy refusal has the exact failed request amount and no copy.
- Decimal output matches the independent canonical literal bytes for both signs and the small controls. The exact byte bound succeeds; one fewer byte yields the exact byte-limit refusal without output.
- Every injected decimal reservation refusal returns allocation with the minimum additional payload bytes of that reservation and no partial output. Production and synthetic controls share one checked arithmetic path; overflow reports `usize::MAX` without an attempted allocation.
- Public callers distinguish allocation, byte limit and output-size overflow without inspecting error text; existing public successful behavior remains available. No byte-limit refusal states `required <= limit`.

## Status

Specified. Exact implementation and QSV readback are pending IR-1149.
