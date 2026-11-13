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
3. For AC-4, run the host differential over the per-profile family of FR-404 Behavior 8 (L - 1 and L with L = min(A, C), and L_b, for each supported profile; each length with no admitted payload listed with its reason), for all six operators, at zero, exact and one-short budgets, comparing the replacement with the shipped `compare_text` on outcome, ordered charges, denied next charge and counters, plus pairs differing only in the last byte, proper prefixes, the pair `x` and `x` followed by one U+0000 byte, and operands of distinct profiles. The expected side is the shipped function and an independently written reference, not the replacement's helpers.
4. For AC-5, run the scanner of FR-401-AC-4 over the carrier module and the replacement: allow-list of callees, call-graph cycle check, banned-token and loop-header scan. Run it over one seeded violating copy per banned construct (`cfg(kani)`, `chars()`, an iterator adapter, a SWAR load, recursion, `while`, a data-dependent loop bound, a call of the shipped `compare_text`) and assert each is rejected.
5. For AC-8, run the local probe harness of FR-404 Behavior 9 for the 64 and then the 32 candidate (at most ten minutes each) under the authorized Kani runner and record peak memory with provenance, time, tool version and outcome as rows of the evidence-record section below; check that C is the largest VERIFIED candidate, or 32 with both rows present if neither verified (then AC-6 does not apply).
6. For AC-6, confirm the harness record at the chosen C is VERIFIED with peak memory under 16 GiB. For AC-7, run the FR-401 normalization at the chosen C under Kani, record the same fields, and check the supported or unsupported report of the normalizing profiles against the outcome.
7. For AC-9, confirm AC-6, AC-7 and AC-8 each cite an executed run (or the measured rows of the evidence-record section). Run each mutant through the step it names.

### Mutants

| Mutant | Fails at |
| --- | --- |
| The carrier holds C - 1 bytes at most | the C-byte payload of step 2, expected held |
| The constructor truncates a longer payload | the C + 1 and 2C payloads of step 2, expected `PayloadExceedsCapacity` |
| The constructor refuses only exactly C + 1 | the 2C payload of step 2 |
| The tail beyond the length is copied from the input buffer | the zero-tail check of step 2 |
| The replacement compares only the first C - 1 bytes | the pair that differs only in the last byte at length L (step 3) |
| The replacement reads or ignores bytes at or after a length | the pair `x` and `x` followed by one U+0000 byte (step 3) |
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

## Evidence record

This section is the versioned home of the Kani measurement rows that FR-404-AC-7, AC-8 and AC-9 consume; it is copied from a local, unversioned tractability table. Columns: operation, outcome, wall time, peak memory, Kani version, harness id. A dash means the peak was not recorded. The probe rows are executed runs: Kani 0.68.0, CBMC 6.11.0, `cargo kani --harness <id> --exact --unwind 129 --jobs 1 --solver cadical`, one job under `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0`; wall time is Kani's reported verification time (total wall including the build was 107.2 s and 51.4 s); peak memory is the scope's cgroup `memory.peak`; each run satisfied 3 of 3 cover properties and fits the 16 GiB ceiling. Both probe rows were measured at source revision cf32ace5 (the `compare_native_text` path and the harness are unchanged since; only the constructor's UTF-8 refusal changed, which the harness does not call). The scope held the whole `cargo kani` invocation, so `memory.peak` includes the compile step as well as CBMC and is an upper bound on the solver's peak; a reviewer rerun with a cached build read 3850371072 bytes at C = 64. C = 64 is the largest VERIFIED candidate. The eight FR-401 rows are executed runs of the harness `normalization_probe` of `src/native_text/kani_proofs.rs` (N = C = 64, D = 32, R = 64, a symbolic payload of symbolic length at most 64; it asserts no panic, and on success that the size lies within the buffers), one harness per form with `#[kani::unwind(65)]` (the `unwind_bound::<64, 32, 64>()` of FR-401), `cargo kani --harness <id> --exact --jobs 1 --solver cadical`, inside `systemd-run --user --scope -p MemoryMax=24G -p MemorySwapMax=0` under `flock /tmp/cargo-slot.lock`. Wall time is the scope's wall clock (it includes the build). In every row the scope was killed by the kernel OOM killer, so the "Peak memory" cell holds the configured cap (25769803776 bytes, journal `Consumed ... 24G memory peak`), marked as a cap: it is not a measured peak, it is a lower bound of the need and above the 16 GiB ceiling. Two shapes of the normalization were run, one per half of the rows.

First shape (commit 75efd00; the decomposition copy calls `class_of`, a binary search of the combining-class ranges, for every copied member): the run stopped in CBMC symbolic execution. About 19,400 `class_of` loop instances (about 19,100 for `nfkc` and `nfkd`) had been unwound, the outer decode loop was at slot 59 of 64 (58 for `nfkc` and `nfkd`) and the decomposition copy at iteration 32 (27), before any equation was built, so the SAT solver was never reached.

Second shape (commit c5d0681, ruled by the code lead as one try: the generator emits the combining class of every decomposition member, so the copy path does no class search and `class_of` runs only for a scalar with no table entry; `normalize.rs` and `tables.rs` are unchanged after that commit): symbolic execution now completes (Kani's `Runtime Symex` 191.5 s for `nfd`, 186.2 s for `nfc`, 197.4 s for `nfkc`, 217.1 s for `nfkd`; the decode loop reached slot 64 of 64), CBMC reports a program of 11,389,655 (`nfd`), 11,516,947 (`nfc`), 12,062,098 (`nfkc`) and 11,934,806 (`nfkd`) steps and 324,772, 344,563, 423,603 and 403,812 verification conditions (74,788, 79,221, 106,037 and 101,604 after simplification), the equation post-processing finished (about 6 to 7 s), and the scope was killed by the OOM killer after that, in the stage that builds the SAT problem and runs the solver (no `Passing problem` or solver line was logged). The class column therefore moved the exhaustion from symbolic execution to the solving stage and did not make the run fit 16 GiB. No further variant was tried and no smaller instantiation was run, and none is claimed. Because no row is VERIFIED, the normalizing profiles are unsupported on the proof path (FR-404 Behavior 10), no owner `TextType` bound is narrowed, and the host function `normalize_text` is unaffected.

| Operation | Outcome | Wall time | Peak memory | Kani version | Harness id |
| --- | --- | --- | --- | --- | --- |
| Byte-loop equality, two payloads of at most 8 bytes | VERIFIED | 0.8 s | - | 0.68.0 | p-text-eq_loop_vs_slice_8 |
| Byte-loop equality, at most 32 bytes | VERIFIED | 1.1 s | - | 0.68.0 | p-text-eq_loop_vs_slice_32 |
| Byte-loop equality, at most 128 bytes | VERIFIED | 3.8 s | 197312512 bytes | 0.68.0 | p-text-eq_loop_vs_slice_128 |
| Byte-loop ordering, at most 8 bytes | VERIFIED | 1.0 s | - | 0.68.0 | p-text-order_loop_vs_slice_8 |
| Byte-loop ordering, at most 32 bytes | VERIFIED | 1.6 s | - | 0.68.0 | p-text-order_loop_vs_slice_32 |
| `std` UTF-8 validation of ASCII, at most 32 bytes | TIMEOUT | 600.9 s | 5891395584 bytes | 0.68.0 | p-text-utf8_std_accepts_ascii_32 |
| `std` scalar count, at most 8 bytes | TIMEOUT | 600.6 s | 4044283904 bytes | 0.68.0 | p-text-char_count_bounds_8 |
| Probe harness of FR-404 Behavior 9, C = 64 | VERIFIED | 103.0 s | 3796357120 bytes | 0.68.0 | native_text_kani_proofs::native_text_compare_matches_slice_order |
| Probe harness of FR-404 Behavior 9, C = 32 | VERIFIED | 49.2 s | 2419408896 bytes | 0.68.0 | native_text_kani_proofs::native_text_compare_matches_slice_order |
| FR-401 normalization at the chosen C (nfd, class search per member at 75efd00) | OUT OF MEMORY | 402.5 s | cap 25769803776 bytes (OOM kill; not a measured peak) | 0.68.0 | native_text_kani_proofs::native_text_normalize_nfd |
| FR-401 normalization at the chosen C (nfc, class search per member at 75efd00) | OUT OF MEMORY | 400.7 s | cap 25769803776 bytes (OOM kill; not a measured peak) | 0.68.0 | native_text_kani_proofs::native_text_normalize_nfc |
| FR-401 normalization at the chosen C (nfkc, class search per member at 75efd00) | OUT OF MEMORY | 429.5 s | cap 25769803776 bytes (OOM kill; not a measured peak) | 0.68.0 | native_text_kani_proofs::native_text_normalize_nfkc |
| FR-401 normalization at the chosen C (nfkd, class search per member at 75efd00) | OUT OF MEMORY | 426.3 s | cap 25769803776 bytes (OOM kill; not a measured peak) | 0.68.0 | native_text_kani_proofs::native_text_normalize_nfkd |
| FR-401 normalization at the chosen C (nfd, class column at c5d0681) | OUT OF MEMORY | 214.4 s | cap 25769803776 bytes (OOM kill; not a measured peak) | 0.68.0 | native_text_kani_proofs::native_text_normalize_nfd |
| FR-401 normalization at the chosen C (nfc, class column at c5d0681) | OUT OF MEMORY | 206.4 s | cap 25769803776 bytes (OOM kill; not a measured peak) | 0.68.0 | native_text_kani_proofs::native_text_normalize_nfc |
| FR-401 normalization at the chosen C (nfkc, class column at c5d0681) | OUT OF MEMORY | 218.1 s | cap 25769803776 bytes (OOM kill; not a measured peak) | 0.68.0 | native_text_kani_proofs::native_text_normalize_nfkc |
| FR-401 normalization at the chosen C (nfkd, class column at c5d0681) | OUT OF MEMORY | 239.4 s | cap 25769803776 bytes (OOM kill; not a measured peak) | 0.68.0 | native_text_kani_proofs::native_text_normalize_nfkd |

## Expected Results

The carrier holds up to C bytes with a zero tail and refuses every longer payload. The replacement agrees with the shipped function on outcome and charges over the per-profile family and is separate from the expected side. Each Kani record shows a terminal outcome, peak memory and provenance. A timeout, memory exhaustion, missing cover or missing run is inconclusive and never a pass.

## Status

Executed for AC-1 through AC-9: host tests `tests/ir1445_native_text.rs` and the probe rows for AC-1 through AC-6 and AC-8; AC-7 and AC-9 by the eight FR-401 rows of the evidence record (two shapes of the normalization, all OUT OF MEMORY at the 24 GiB cap, so the normalizing profiles are excluded, citing the record) and `normalizing_profiles_are_supported_only_on_a_verified_record`. FR-404-AC-7 is met by that exclusion; the FR-401 normalization itself is met on the host (FR-401, TC-949).

FR-399-AC-6 and FR-399-AC-7 are active now that the carrier exists and are owned by IR-1445 (planner ruling). They are bound by `proof_path_constructor_decides_like_the_owner_over_the_enumeration`, `proof_path_constructor_refuses_c_plus_one_and_allocates_nothing` and `proof_path_constructor_module_scans_clean` (TC-947). The host `Text::from_admitted` that AC-6 names does not exist, so the host counterpart of the decision is the owner's admission `admit_text`: the test enumerates every string of at most three scalars over a four-scalar alphabet for every profile and two type bounds, plus the boundary payloads at C - 1 and C, each with the three retained sequences, and expects the owner's decision. The boundary family is the one the owner admits (A - 1 is not built for `nfc` and `nfkc`, where no admitted payload of that length exists, and the nearest admitted payloads of `max` repetitions of `nfd(U+1F82)` are used). The zero-allocation check of AC-7 covers the supported profiles only; the four normalizing profiles take the host-only `std` path.

Scan exemption (AC-5): `src/native_text/normal_form.rs` holds the host-only constructor decision that uses `std` UTF-8 validation and the owner's normalization, for the four normalizing profiles only (the profiles the proof path does not support). It is scanned in `host_only_gate_has_exactly_the_declared_violations`, which asserts its exact violations; `src/native_text.rs` scans clean. The constructor's precondition is that its parts were already admitted by the owner (valid UTF-8): the two proof-path profiles do not re-validate UTF-8 (not Kani-tractable), and the four normalizing profiles, host-only, refuse non-UTF-8 parts with `RetainedNotNormalForm`. `compare_native_text` maps its decided ordering to the operator's truth value with the owner's `ComparisonOperator::holds`, which decides neither equality nor order; the independent reference of the differential and the harness's slice `==` and `cmp` are separate.
