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

Verify FR-401-AC-1 through FR-401-AC-9 by running the host-callable proof-path normalization beside `NormalizationForm::apply`, `compare_text`, `admit_text` and the public `unicode-normalization` functions. Inputs, buffers and the per-thread counting allocator are prepared before each measurement window.

## Test Procedure

1. At N = 64, D = 32, R = 64 and for each of the four forms, normalize every Unicode scalar alone (surrogates excluded), every ordered pair of the listed scalars, the six listed sequences and the empty payload with the function and with `apply`; compare scalars, order and count.
2. Run the refusals of FR-401-AC-2: U+1F82 under `nfd` at R of 7, 8 and 9, U+FDFA under `nfkd` at D of 17 and 18, the empty payload at R and D of 0, `len` = N + 1, the invalid byte strings. For every scalar alone and `max` from 1 to 4, when `Text[0,max]` admits the `apply` output, assert success at R = `4 * max` and D = `payload_scalar_ratio * max`.
3. Under a global allocator with a `thread_local!` `Cell` counter (const-initialized), assert zero allocations around the function for every input of steps 1 and 6.
4. Read the module with `include_str!`, strip comments and string literals, and run the FR-401-AC-4 scan: the callee allow-list, the banned tokens and closure pattern, range-in-index and buffer-equality bans, the call-graph cycle check, and the loop-header and `trip!` site checks. Run the same scanner on copies of the text seeded with one violation each (every construct of FR-401-AC-4, including `binary_search`, `contains`, `copy_from_slice`, `fill`, `iter_mut`, `into_iter`, `zip`, `windows`, `payload == OTHER`, `&a == &b`, `*a == *b`, `self.buf == x.buf`, `a < b` on arrays, `a.min(b)` on arrays, `DECOMP[i] == DECOMP[j]`, `let row = DECOMP[i]; row == other`, `[a..b]`, a self-call, a two-function call cycle, `for x in slice`, `for i in 0..len`, and `for i in 0..TABLE_LEN` without `trip!`) and assert it rejects each.
5. For each ordered pair and each form, derive the six operators from the lexicographic order of the two outputs and compare with `compare_text` over operands admitted under `Text[0,64]`; under `test-support` count the `text.normalize-output` entries and compare with the emitted counts, checking that the second operand continues the first's running count.
6. Normalize the 31-mark run of FR-401-AC-7 under the four forms against `apply`, at D of 32 and 31, and the 5-mark run.
7. Enumerate every scalar and compare the table decomposition, compatibility decomposition and combining class with the public `char` functions; check each table triple with `char::compose`; run the `#[ignore]`d pair sweep over every scalar a and every right element b with `cargo test --release -- --ignored` (a step to add under the Makefile target `test-exhaustive`); in the default run the generator only compares its in-memory regeneration with the checked-in table.
8. At (N, D, R) of (64, 32, 64) and (16, 8, 16), run the AC-2 boundary inputs with the `cfg(test)` trip counters; assert every module-constant bound is below 16, each site's maximum trips against its declared bound, equality for the N, D and R sites, and `unwind_bound::<N, D, R>()` = largest declared bound + 1 at each instantiation.

## Expected Results

- Output, order and count equal `apply` for every input, including reordering, blocked composition, excluded composites, Hangul, singletons and compatibility mappings.
- Refusals are exact at the byte and the scalar.
- No allocation occurs and the scanner rejects each seeded violation.
- A mutant that skips canonical reordering fails `a` U+0307 U+0323; one that composes through a blocked mark fails `a` U+0310 U+0300; one that composes an excluded composite fails U+0915 U+093C; one that omits Hangul composition fails U+1100 U+1161 U+11A8; one that skips compatibility decomposition fails U+FB03 and U+FDFA under `nfkc` and `nfkd`; one that treats `nfc` as `nfd` fails `u` U+0308 U+0301; a decomposition buffer of 4 entries fails step 6; a wrong table entry fails step 7; an off-by-one capacity fails step 2; a count that restarts for the second operand fails step 5; an allocation, iterator adapter or looping std call (`binary_search`, `contains`, `copy_from_slice`, `fill`) fails steps 3 and 4; an untracked loop or a mutual recursion fails step 4; a loop written `0..=N` runs N + 1 trips and fails step 8 and the scanner; an `unwind_bound` that ignores N, D or R fails step 8 at the second instantiation; a data-dependent header fails the scanner; an `unwind_bound` off by one fails step 8. A swap between `nfc` and `nfkc` is distinguished only by inputs with a compatibility mapping (U+FB03) and is claimed on those only.

## Status

Planned (IR-1501).
