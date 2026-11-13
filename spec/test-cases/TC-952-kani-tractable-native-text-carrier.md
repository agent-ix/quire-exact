---
id: TC-952
title: "The native Text carrier holds C bytes, refuses C + 1 and verifies within the memory budget"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-404
    type: verifies
---
# TC-952: The native Text carrier holds C bytes, refuses C + 1 and verifies within the memory budget

## Description

Verify FR-404-AC-1 through FR-404-AC-9 through the public kernel API, the host differential, a source scan and the recorded Kani measurements. Each mutant below is a buildable variant in a scratch copy and names the input or inspection that fails it.

## Test Procedure

1. For AC-1, check `Copy` by a generic bound and no drop glue by a `needs_drop` assertion, coerce the constructor and `compare_native_text` to function pointers of the signatures of FR-404 Behaviors 3 and 6, and run construction of a C-byte payload inside a per-thread counting allocator window, asserting zero allocations.
2. For AC-2 and AC-3, build payloads of C - 1 and C bytes (one ending in a one-byte scalar, one ending in a continuation byte of a multibyte scalar), read them back, and assert every byte at or after the length is zero. Build the same two at C + 1 and at 2C bytes, and a retained sequence of C + 1 bytes under a payload of at most C, and assert `PayloadExceedsCapacity` and no carrier; assert it is never returned at C bytes.
3. For AC-4, run the host differential over the per-profile family of FR-404 Behavior 8 (L - 1 and L with L = min(A, C), and L_b, for each supported profile; each length with no admitted payload listed with its reason), for all six operators, at zero, exact and one-short budgets, comparing the replacement with the shipped `compare_text` on outcome, ordered charges, denied next charge and counters, plus pairs differing only in the last byte, proper prefixes, and operands of distinct profiles. The expected side is the shipped function and an independently written reference, not the replacement's helpers.
4. For AC-5, run the scanner of FR-401-AC-4 over the carrier module and the replacement: allow-list of callees, call-graph cycle check, banned-token and loop-header scan. Run it over one seeded violating copy per banned construct (`cfg(kani)`, `chars()`, an iterator adapter, a SWAR load, recursion, `while`, a data-dependent loop bound, a call of the shipped `compare_text`) and assert each is rejected.
5. For AC-8, run the local probe harness of FR-404 Behavior 9 for the 64 and then the 32 candidate (at most ten minutes each) under the authorized Kani runner and record peak memory with provenance, time, tool version and outcome as rows of the Kani tractability table; check that C is the largest VERIFIED candidate, or 32 with both rows present if neither verified.
6. For AC-6, confirm the harness record at the chosen C is VERIFIED with peak memory under 16 GiB. For AC-7, run the FR-401 normalization at the chosen C under Kani, record the same fields, and check the supported or unsupported report of the normalizing profiles against the outcome.
7. For AC-9, confirm AC-6, AC-7 and AC-8 each cite an executed run (or the measured rows for IR-1526). Run each mutant through the step it names.

### Mutants

| Mutant | Fails at |
| --- | --- |
| The carrier holds C - 1 bytes at most | the C-byte payload of step 2, expected held |
| The constructor truncates a longer payload | the C + 1 and 2C payloads of step 2, expected `PayloadExceedsCapacity` |
| The constructor refuses only exactly C + 1 | the 2C payload of step 2 |
| The tail beyond the length is copied from the input buffer | the zero-tail check of step 2 |
| The replacement compares only the first C - 1 bytes | the pair that differs only in the last byte at length L (step 3) |
| The replacement orders a proper prefix as equal | the proper-prefix pair (step 3) |
| The replacement charges nothing, or charges from a truncated length | the charge, denied-charge and counter comparison of step 3 |
| The replacement does not refuse distinct profiles | the distinct-profile pair of step 3 |
| The replacement delegates to the shipped `compare_text` | the call scan of step 4 (the differential would pass vacuously) |
| The replacement body is selected by `cfg(kani)` | the token scan of step 4 |
| The replacement uses `chars()`, an iterator adapter or a SWAR load | the scan of step 4 (a Kani run of it would be inconclusive) |
| The replacement is recursive | the call-graph cycle check of step 4 |
| The carrier owns a `Box<str>` or `String` | the `Copy` bound and the allocation window of step 1 |
| C is recorded as 32 with a VERIFIED record for 64 | the probe record check of step 5 |
| The harness record cites a configured limit as the peak | the provenance check of steps 5 and 6 |
| The normalizing profiles are reported supported on a TIMEOUT record | the outcome check of step 6 |

## Expected Results

The carrier holds up to C bytes with a zero tail and refuses every longer payload. The replacement agrees with the shipped function on outcome and charges over the per-profile family and is separate from the expected side. Each Kani record shows a terminal outcome, peak memory and provenance. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass.

## Status

Not run. No carrier, probe or harness exists.
