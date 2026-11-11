---
id: TC-949
title: "Proof-path text normalization against the shipped normalization"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-401
    type: verifies
---
# TC-949: Proof-path text normalization against the shipped normalization

## Description

Verify FR-401-AC-1 through FR-401-AC-6 by running the host-callable proof-path normalization beside `NormalizationForm::apply`, `compare_text` and `admit_text`. Inputs, buffers and the counting allocator are prepared before each measurement window.

## Test Procedure

1. For each of the four forms, normalize every Unicode scalar value alone (surrogates excluded), every ordered pair of the listed scalars, the six listed sequences and the empty payload with both the proof-path function and `apply`; compare scalars, order and count.
2. Run the capacity boundary of FR-401-AC-2: U+1F82 under `nfd` at R of 7, 8 and 9, U+FDFA under `nfkd` at its exact length and one short, the empty payload at R of 0. For every scalar alone and `max` from 1 to 4, when `Text[0,max]` admits the `apply` output assert success at R of `4 * max`.
3. Wrap each AC-1 normalization in a counting global allocator window and assert zero allocations. Read the proof-path module with `include_str!` and assert that none of the banned tokens occurs.
4. For each ordered pair and each profile, compute the six operators from the lexicographic order of the two proof-path outputs and compare each with `compare_text` over operands admitted under a type that admits both (a `Text[0,64]` for the profile); `binary-utf8` compares bytes.
5. Under `test-support`, run `compare_text` with a generous meter, count the `text.normalize-output` entries and compare with the proof-path emitted counts of both operands, checking that the second operand's entries continue the first's running count.
6. Read the proof-path module's loop bounds and `UNWIND_BOUND`, compute the largest loop trip count and assert `UNWIND_BOUND` equals it plus one.

## Expected Results

- Output, order and count equal `apply` for every input, including the reordering, blocked-composition, excluded-composite, Hangul, singleton and compatibility cases.
- Capacity is exact at the byte, and every admitted retained sequence fits `4 * max`.
- No allocation occurs and the module is free of banned constructs.
- A mutant that skips canonical reordering fails `a` U+0307 U+0323; one that composes through a blocked mark fails `a` U+0310 U+0300; one that composes an excluded composite fails U+0915 U+093C; one that omits Hangul composition fails U+1100 U+1161 U+11A8 and the Hangul boundary scalars; one that skips compatibility decomposition fails U+FB03 and U+FDFA under `nfkc` and `nfkd`; one that treats `nfc` as `nfd` fails `u` U+0308 U+0301; one that normalizes with the wrong form fails the singleton U+212B under `nfc` against `nfd`; an off-by-one capacity fails step 2; a count that restarts for the second operand fails step 5; an allocation or iterator adapter fails step 3; a data-dependent loop bound or an `UNWIND_BOUND` off by one fails step 6. A swap between `nfc` and `nfkc` is distinguished only by inputs with a compatibility mapping (U+FB03) and is claimed on those only.

## Status

Planned (IR-1501).
