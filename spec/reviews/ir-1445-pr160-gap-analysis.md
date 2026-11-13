---
id: SR-17940486709564847047937
title: "IR-1445 PR #160 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact PR #160 at 75efd007, git diff origin/main...HEAD; IR-1445; FR-401-AC-1..AC-9, FR-404-AC-7, FR-404-AC-9; tests/ir1501_normalize/*, src/native_text/normalize/trip_tests.rs, tests/ir1445_native_text.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-401
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-404
    type: reviews
---

## Summary

Ticket: IR-1445. Plan completion: not assessed.

`quoin matrix` (0.29.1) at 75efd007 shows:

- FR-401-AC-1 to AC-9 `tagged`.
- FR-404-AC-6 to AC-9 `method-without-symbol` (Inspection).
- FR-399-AC-6 and AC-7 `untagged`.

FR-401-AC-4's verification cell parses as `…`. That was there before this PR: the AC-4 text is unchanged.

Every new production item has an owning criterion:

- `normalize_text`, `NormalizeRefusal`, `NormalizedSize`: FR-401 Behaviors 1 to 5.
- `unwind_bound`: Behavior 6.
- `trips`: AC-9.
- The four harnesses: FR-404 Behavior 10.
- `proof_path_supports`: Behavior 10.

There are no stubs.

The bindings were checked against the AC text:

- AC-1: every scalar alone in all four forms, the 21x21 ordered pairs and the six sequences. A pair that does not fit asserts `ExceedsCapacity`.
- AC-2: every named refusal, plus extra invalid encodings, plus the scalar-alone sweep at max 1..4.
- AC-4: the scan and 37 seeds. Every construct AC-4 lists by name has a seed, including `self.buf == x.buf`, which is seeded as `x.buf == y.buf` and is equivalent by last segment.
- AC-7: the 31-mark run, D 31 and the 5-mark run.
- AC-8: regeneration equality, per-scalar decomposition and class, the triples and the ignored sweep. The sweep runs in 0.78 s.
- AC-9: both instantiations, bounds below 16, and `unwind_bound` +1.
- FR-404-AC-7: the Behavior 10 test now requires four executed rows, each with a wall time, a peak and a harness id, and support equal to all four VERIFIED within 16 GiB. It passes on the OUT OF MEMORY rows and would fail on NOT RUN.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | AC-3 coverage is a sample. `normalization_allocates_nothing` measures `inputs()` plus every 97th scalar. FR-401-AC-3 and TC-949 step 3 require "every input of AC-1 and AC-7": every scalar alone, plus the 31-mark and 5-mark runs, which are absent. Measure the full scalar set (it already runs in the AC-1 test) and add the AC-7 payloads. | tests/ir1501_normalize/main.rs:305-334 |
| FND-002 | medium | AC-5 and AC-6 coverage is a sample. The right operand is `texts.iter().step_by(5)`, so about a fifth of the ordered pairs are compared, while FR-401-AC-5 and TC-949 step 5 say "every ordered pair of AC-1". Iterate every right operand, or narrow AC-5 to the sampled set with a stated reason (cost). | tests/ir1501_normalize/main.rs:345-352 |
| FND-003 | low | Seeds are missing for four AC-4(b) banned words and two closure positions. AC-4 requires one seed per construct in (a) and (b). No seed exercises `alloc`, `Fn`, `FnMut` or `FnOnce` alone: the `dyn Fn` seed asserts only "banned word dyn". No seed puts a closure after `(` or `,`; only `=` and `move \|` are seeded. Add a seed for each. | tests/ir1501_normalize/scan.rs:518-556 |
| FND-004 | low | FR-401-AC-3 names "a `thread_local!` `Cell` initialized by a `const` block". The test uses `allocation-counter` 0.8.1, whose per-thread counter is a lazily initialized `thread_local!` `RefCell`. The per-thread intent holds, but the AC's literal mechanism does not. Either use a local const-initialized `Cell` allocator or align the AC wording. | tests/ir1501_normalize/main.rs:325-333 |
| FND-005 | low | The Behavior 10 binder counts four rows but does not check that they name four distinct forms. It also does not check that each harness id exists in `src/native_text/kani_proofs.rs`. Four `(nfd)` rows, or a renamed harness, would pass. Assert the set {nfc, nfd, nfkc, nfkd} and that each id resolves. | tests/ir1445_native_text.rs:1221-1267 |
| FND-006 | low | The scanner admits `matches!` through `MACROS_OUTSIDE_THE_SCAN`, which is not on the FR-401-AC-4(a) allow-list. AC-4 says that any other callee fails. Add `matches!` to AC-4(a), or rewrite the two uses as `match`. | tests/ir1501_normalize/scan.rs:22, src/native_text/normalize.rs:111-116 |

## Verdict

Mergeable once FND-001 and FND-002 are fixed. They are cheap to fix and are real gaps between what AC-3 and AC-5 say ("every") and what is measured (a sample). Under the repo's "findings in the same PR" rule they belong here, not in a follow-up. FND-003 to FND-006 are low. FR-404-AC-7 and AC-9 are now honestly bound to executed rows.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-007 | medium | The new FR-399-AC-6 binder `proof_path_constructor_decides_like_the_owner_over_the_enumeration` enumerates strings of at most three scalars over a four-scalar alphabet (a, U+00E9, U+0301, U+1F82). AC-6 asks for "every string that the correspondence enumeration of CG FR-052-AC-2 covers", and CG FR-052-AC-2 is "every byte string up to a stated small length, filtered by `std::str::from_utf8`". The alphabet is a sample, not every string. The comparator is also an oracle the test builds (`expected_decision` from `admit_text` and the normal form), because the host `from_admitted` that AC-6 names does not exist. That deviation is stated only in TC-952 Status. Add every UTF-8 byte string up to a stated small length (2 bytes is cheap; it also covers U+00E9 under `Text[0,1]`) beside the alphabet. Record the host-counterpart substitution in FR-399 (see spec-review FND-006). | tests/ir1445_native_text.rs:263-322, spec/functional/FR-399-non-metered-text-constructor-from-admitted-parts.md:55 |
| FND-008 | low | The zero-allocation check for FR-399-AC-7 runs over the supported profiles only. The four normalizing profiles take the host-only `std` path and allocate. AC-7 says "over the AC-6 inputs" with no carve-out, and the carve-out is stated only in TC-952 Status. Put it in FR-399-AC-7 or in FR-399 Status. | tests/ir1445_native_text.rs:326-356 |

## Dispositions

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | `normalization_allocates_nothing` now measures `inputs()`, both AC-7 payloads (`ac7_texts`) and every scalar from `generator::scalars()`. |
| FND-002 | fixed | `order_and_counts_match_the_shipped_compare` iterates every ordered pair of the fitting AC-1 payloads, for all six operators. The charge amounts are captured on the first operator of each pair, which AC-6 ("for each operand pair") allows. |
| FND-003 | fixed | Seeds were added for `alloc`, `impl Fn`, `impl FnMut`, `impl FnOnce`, a closure after `(` and a closure after `,`. |
| FND-004 | fixed | FR-401-AC-3 and TC-949 step 3 now name the per-thread `allocation-counter` dev-dependency, with the reason: the crate sets `unsafe_code = "forbid"` (Cargo.toml lints) and a counting global allocator needs `unsafe`. The AC and the test agree. |
| FND-005 | fixed | The binder groups the rows by shape and requires each shape's forms to equal {nfc, nfd, nfkc, nfkd}. Each harness id must be `native_text_normalize_<form>` and `fn native_text_normalize_<form>()` must exist in `kani_proofs.rs`. An OOM peak must be marked "cap ... not a measured peak". |
| FND-006 | fixed | `MACROS_OUTSIDE_THE_SCAN` was removed, and the two `matches!` uses are now `match`. |
