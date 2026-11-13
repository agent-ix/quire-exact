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

Verify FR-404-AC-1 through FR-404-AC-8 through the public kernel API, the host differential, a source scan and the recorded Kani measurements. Each mutant below is a buildable variant in a scratch copy and names the input or inspection that fails it.

## Test Procedure

1. For AC-1, check `Copy` by a generic bound and no drop glue by a `needs_drop` assertion, and run construction of a C-byte payload inside a per-thread counting allocator window, asserting zero allocations.
2. For AC-3, build payloads of C - 1 and C bytes (one ending in a one-byte scalar, one ending in a multibyte scalar's continuation byte) and read them back; build the same two at C + 1 bytes and assert the encoding refusal and no carrier.
3. For AC-4, run the host differential: every operator over pairs at lengths C - 1 and C that are equal, differ only in the last byte, or are proper prefixes of one another, comparing the replacement with the shipped `compare_text` on typed outcome. The expected side is the shipped function and an independently written reference, not the replacement's own helpers.
4. For AC-5, scan the comment-stripped source of the carrier module and the replacement for the banned tokens and loop-header form, and for calls into the expected side.
5. For AC-2, run the probe of FR-404 Behavior 2 for the 32 and 64 candidates (at most ten minutes each) under the authorized Kani runner and record peak memory with provenance, time, tool version and outcome; check that C is the largest candidate recorded VERIFIED within the K-DR budget.
6. For AC-6, run the CG text row's harness at the chosen C and record the same fields. For AC-7, run the FR-401 normalization at the chosen C and record the same fields.
7. For AC-8, confirm that AC-2, AC-6 and AC-7 each cite an executed run. Run each mutant through the step it names.

### Mutants

| Mutant | Fails at |
| --- | --- |
| The carrier holds C - 1 bytes at most | the C-byte payload of step 2, expected held |
| The carrier accepts C + 1 bytes by truncation | the C + 1 payload of step 2, expected the encoding refusal |
| The replacement compares only the first C - 1 bytes | the pair that differs only in the last byte at length C (step 3) |
| The replacement orders a proper prefix as equal | the proper-prefix pair (step 3) |
| The replacement delegates to the shipped `compare_text` | the call scan of step 4 (the differential would pass vacuously) |
| The replacement body is selected by `cfg(kani)` | the token scan of step 4 |
| The replacement uses `chars()` | the token scan of step 4 (a Kani run of it would be inconclusive) |
| The carrier owns a `Box<str>` or `String` | the `Copy` bound and the allocation window of step 1 |
| C is recorded as the larger candidate with no VERIFIED record | the probe record check of step 5 |
| The harness record cites a configured limit as the peak | the provenance check of steps 5 and 6 |

## Expected Results

The carrier holds up to C bytes and refuses C + 1. The replacement agrees with the shipped function on the boundary family and is separate from the expected side. Each Kani record shows a terminal outcome, peak memory and provenance. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass.

## Status

Not run. No carrier, probe or harness exists.
