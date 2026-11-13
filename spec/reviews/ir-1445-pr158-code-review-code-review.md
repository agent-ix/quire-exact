---
id: SR-17940486709564847047933
title: "IR-1445 PR #158 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact PR #158 at cf32ace5, git diff origin/main...HEAD of code/ir-1445-native-text-carrier; IR-1445; src/native_text.rs, src/native_text/normal_form.rs, src/native_text/kani_proofs.rs, src/lib.rs, src/text.rs, tests/ir1445_native_text.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-404
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-399
    type: reviews
---

## Summary

Ticket: IR-1445. The PR adds `NativeText` (C = 64, `Copy`, two fixed `[u8; 64]` arrays with
`usize` lengths, zero tail), `NativeText::from_admitted` (capacity refusal first, then the FR-399
decision: retained check, then profile length), `compare_native_text` (profile refusal, then the
shipped charge sequence with indexed loops bounded by `NATIVE_TEXT_CAPACITY` and
`NORMALIZED_OUTPUT_BOUND = 2C`), a host-only `normal_form::is_retained_of`, the
`TextNotAdmittedCause` enum in `src/text.rs`, the Kani probe harness and a host test file.

Reviewer reproduction at cf32ace5 (`CARGO_TARGET_DIR=<worktree>/target`, every cargo step under
`flock /tmp/cargo-slot.lock`, Kani in `systemd-run --user --scope -p MemoryMax=24G
-p MemorySwapMax=0`, foreground, one job):

- `cargo test --features test-support --test ir1445_native_text`: 12 passed.
- Kani 0.68.0 / CBMC 6.11.0, `cargo kani --harness
  native_text_kani_proofs::native_text_compare_matches_slice_order --exact --unwind 129 --jobs 1
  --solver cadical`, C = 64: VERIFICATION SUCCESSFUL, 3 of 3 covers, verification time 104.4 s,
  108.6 s wall including the cached build, scope `memory.peak` 3,850,371,072 B. Matches the
  record (103.0 s, 3,796,357,120 B). C = 32 was not re-run.
- Mutant (scratch copy): `order_bytes` stops comparing at index C - 1: VERIFICATION FAILED (assertion `assert_eq!(value, expected)`), 3 of 3 covers,
  109.2 s. Probe of `from_admitted(Text[0,8], b"\xff", b"\xff")`: `Ok` for `binary-utf8` and
  `unicode-scalars`, `NotAdmitted(RetainedNotNormalForm)` for `nfc` (FND-004).
- `cargo fmt --check` and `cargo clippy --all-targets --features test-support -- -D warnings`: clean.

Separation (question 1): `compare_native_text` shares no ordering or equality helper with
`compare_text` (which orders by `TextProfile::order`, `chars().cmp`) or with the test's
`reference_holds` / the harness's slice `==` and `cmp`. Its loops are all `for index in
0..NATIVE_TEXT_CAPACITY` or `0..NORMALIZED_OUTPUT_BOUND` with in-body `index < len` guards; no
`while`, `loop`, recursion, `chars()`, iterator adapter or word load. No `cfg(kani)` body; the only
`cfg(kani)` is the harness module mount in `src/lib.rs`. Both functions are `pub` and host-called by
the differential. The charge sequence matches `prepare` (input bytes with 2 occurrences, decode
scalars, then for normalizing profiles normalize-input and one cumulative normalize-output per
retained scalar, which equals the payload's normal-form scalar count because the retained sequence
is that normal form, then result-retain). Byte order equals scalar order on valid UTF-8, so one
byte loop serves every profile.

Harness (question 2): it draws symbolic arrays and lengths with an assumed zero tail, builds both
carriers with `zero_padded` (no UTF-8 decision), passes an all-`u64::MAX` meter and asserts the
`Completed` variant for all six operators against slice `==` / `cmp`. The "full equal witness"
cover forces a 64-byte operand to be reachable, and the mutant above shows the final index is
checked, so the run is not vacuous.

`TextNotAdmittedCause` (question 4) has exactly FR-399's two variants under FR-399's name and
module; it is consistent with FR-399 Outputs. FR-399's `TextNotAdmitted` struct and
`Text::from_admitted` are not added, which this PR does not need.

The AC-5 scanner was probed in a scratch copy with extra seeds (FND-001, FND-002) and over
`normal_form.rs` (FND-003).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | The AC-5 scanner's `calls_in` pops the previously recorded callee whenever it sees `!(`. The macro name itself is never pushed (the `(` follows `!`, so no identifier precedes it), so the pop removes a real call. Probe: appending `fn seeded(a: u32) { let _ = unlisted_helper(a); assert!(a > 0); }` to the carrier source scans clean (`[]`), so any unlisted callee followed by a macro passes. Delete the pop and add this seed. | tests/ir1445_native_text.rs:544-574 |
| FND-002 | medium | The scanner does not reject every construct AC-5 bans, and AC-5 requires a rejected seeded copy for each one. Probes scanning clean: the multiply-and-mask SWAR step `(w * 0x0101010101010101) & 0x8080808080808080`, and slice comparisons `a == b` and `a < b` (std memcmp or lexicographic, not indexed loops; FR-401-AC-4(b), whose method AC-5 adopts, bans comparison operators on array or slice operands). No seed exists for `read_unaligned`, the `as u64` cast, `bytes`, `map`, `zip`, `enumerate`, `fold`, `all`, `any`, `position`, `windows`, `chunks`, `chunks_exact`, or a call of an expected-side helper. Add the detections and one seed per listed construct. | tests/ir1445_native_text.rs:490-506, tests/ir1445_native_text.rs:576-600, tests/ir1445_native_text.rs:682-760 |
| FND-003 | medium | The AC-5 scan reads only `src/native_text.rs`, and the allow-list admits `is_retained_of`. That function lives in the carrier's child module `src/native_text/normal_form.rs` and is called by `from_admitted`. Scanning that file returns `[BannedToken("chars"), Callee("from_utf8"), Callee("apply"), Callee("eq"), Callee("chars")]`. AC-5 covers "the carrier module", and FR-399-B4 says the proof-path counterpart uses no `chars()`. The exclusion is not stated anywhere in the spec. A host-only normal-form check is acceptable while the normalizing profiles are unsupported on the proof path (FR-404-B10), but only if it is declared: scan `normal_form.rs` too and assert its exact, listed violations, or move the check out of the carrier module, and record the exemption with its reason (FR-401 absent) in FR-404 Status or TC-952. | src/native_text/normal_form.rs:10-18, src/native_text.rs:19, src/native_text.rs:66-69, tests/ir1445_native_text.rs:474, tests/ir1445_native_text.rs:670-678 |
| FND-004 | medium | `from_admitted` accepts parts that are not UTF-8 for `binary-utf8` and `unicode-scalars`: `from_admitted(Text[0,8], b"\xff", b"\xff")` returns `Ok`, because the check is only `same_bytes` plus a lead-byte count. Owner admission never produces such a `Text`, since `TextPayload` is UTF-8 by type. The `NotAdmitted` doc says it refuses "parts not what admission would have produced". The four normalizing profiles refuse the same bytes as `RetainedNotNormalForm` (`from_utf8` fails in `is_retained_of`), so the profiles disagree. FR-404-B3 fixes `&[u8]` and FR-399 has no invalid-UTF-8 cause, so either document and test the UTF-8 precondition on the constructor (spec Status or B3 note), or refuse non-UTF-8 parts uniformly. | src/native_text.rs:58-91 |
| FND-005 | low | `compare_native_text` decides the operator with `operator.holds(ordering)`, the same helper the shipped `compare_text` (the differential's expected side) uses. FR-404-B6 says "no shared helper that decides equality or order". The risk is mitigated: the differential checks shipped against the independent `reference_holds`, and the Kani harness checks against slice `==` / `cmp`. Either inline the six-arm match or record in TC-952 that `holds` is allowed because it maps an already-decided ordering. | src/native_text.rs:246, tests/ir1445_native_text.rs:472 |
| FND-006 | low | The TC-952 probe rows give the harness name, but no source revision or run identity for the measured code. FR-404-AC-6 requires the "harness run recorded". A later change to `src/native_text.rs` would leave the rows looking current. Add the commit the rows were measured at (cf32ace5 reproduces them). | spec/test-cases/TC-952-kani-tractable-native-text-carrier.md:50-63 |

## Verdict

Not mergeable as is. The carrier and the replacement are correct. The replacement is separate
from the expected side, host-callable, has no `cfg(kani)` body and uses only constant-bound indexed
loops. The charges match the shipped function, and the C = 64 verdict reproduces at 104.4 s and
3.85 GB with a non-vacuous harness. The AC-5 scanner does not yet bind: FND-001 and FND-002 let
seeded violations through, and FND-003 leaves part of the carrier module out of the scan without
saying so. FND-004 is a constructor-domain gap. Fix FND-001 to FND-004 in this PR. FND-005 and
FND-006 are low. The spec-status findings that block merge are in the spec-review-base and
gap-analysis artifacts.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-007 | high | The FND-004 fix puts `std` UTF-8 validation on the constructor path of the two profiles the proof path supports. `from_admitted` for `binary-utf8` and `unicode-scalars` now calls `normal_form::are_utf8`, which is `core::str::from_utf8` on both parts. TC-952's own row "`std` UTF-8 validation of ASCII, at most 32 bytes" is TIMEOUT at 600.9 s, and FR-404-B7 says CG's Kani harness builds its operands with this constructor, so the supported-profile constructor is no longer Kani-tractable (FR-399-B4: indexed byte loops bounded by C). The TC-952 exemption text ("for the profiles the proof path does not support") and the `host_only_gate_has_exactly_the_declared_violations` doc are now false for `are_utf8`. The probe harness does not catch this because it uses `zero_padded`. Fix: decide UTF-8 validity with an indexed loop over `0..NATIVE_TEXT_CAPACITY` in `src/native_text.rs` (scan-clean), with a host differential against `core::str::from_utf8` (every 1- to 4-byte sequence plus the AC-3 payloads). Alternatively, drop the check and state the UTF-8 precondition on B3 / in the doc, the other option FND-004 named. | src/native_text.rs:73, src/native_text/normal_form.rs:23-26, spec/test-cases/TC-952-kani-tractable-native-text-carrier.md:75 |
| FND-008 | low | `slice_comparisons` recognises only bare names declared `&[u8]` or `[u8;`. Method-returned and sub-sliced operands still scan clean (probed at 24c41ac2): `l.retained() == r.retained()`, `l.payload() < r.payload()`, `a[..] == b[..]`. These are the carrier's own accessors, so they are the likeliest regression form. Treat a comparison operator with a `)` or `]` operand whose callee is a slice accessor (`payload`, `retained`, `payload_storage`, `retained_storage`) or a range index as a slice comparison, and seed both forms. | tests/ir1445_native_text.rs:672-730 |

## Dispositions

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | `calls.pop()` on `!(` removed. The unlisted-callee-then-macro seed is in the test and rejected (probed: `[Callee("unlisted_helper")]`). |
| FND-002 | fixed | A multiply-and-mask (word-sized hex mask) detection and `slice_comparisons` were added. Seeds now exist for every listed adapter, `from_le/be/ne_bytes`, `read_unaligned`, the `as u64` cast, multiply-and-mask, all six slice operators, `compare_text`, `admit_text`, `reference_holds` and `order`. Probes for mul-mask, `a == b` and `a < b` are now rejected. Residual form: new FND-008. |
| FND-003 | fixed | `normal_form.rs` is scanned and its exact violation set asserted (`host_only_gate_has_exactly_the_declared_violations`), and the exemption is recorded in FR-404 Status and TC-952 Status. Its rationale is now partly wrong: new FND-007. |
| FND-004 | fixed | All six profiles refuse non-UTF-8 parts with `RetainedNotNormalForm` (probed; test `parts_that_are_not_utf8_refuse_under_every_profile`). The means chosen introduces new FND-007. |
| FND-005 | fixed | A comment at the call and a TC-952 Status note record that `holds` maps an already-decided ordering, and that the reference and the harness are separate. |
| FND-006 | fixed | TC-952 now records that both probe rows were measured at cf32ace5 and that the compare path and the harness are unchanged since. Confirmed: `git diff cf32ace5..24c41ac2` leaves `kani_proofs.rs` and `lib.rs` untouched and changes `compare` only by a comment, so the C = 64 verdict stands. |
| FND-007 | fixed | Round 2 (5c0665ce): `are_utf8` is removed. The `binary-utf8` and `unicode-scalars` constructor path is again `same_bytes` plus `scalar_starts` (indexed loops only, scan-clean). The UTF-8 precondition is stated on FR-404-B3, in the `from_admitted` doc and in TC-952. `normal_form.rs` now serves only the four normalizing profiles, so the exemption text is accurate. |
| FND-008 | fixed | Round 2: `through_accessor_or_subslice` plus `slice_returning_functions` flag comparisons through `&[u8`-returning functions and range sub-slices, with seeds for all six operators. Probes are rejected: `l.retained() == r.retained()`, `l.payload() < r.payload()`, `l.payload_storage() == r.payload_storage()`, `a[..] == b[..]`. An integer `a < b` still scans clean. |
