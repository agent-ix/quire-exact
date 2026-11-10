---
id: SR-20261110053339850002
title: QSL-1826 decimal implementation gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact; QSL-1826 PR143; src/integer.rs, src/lib.rs; FR-390 B7/AC8-9
  and legacy AC2-5
review_set: subset
relationships:
- target: ix://agent-ix/quire-exact/FR-390
  type: references
---

## Summary

Independent runtime review of quire-exact PR143 at `d88d5a65ebbe1a7d40d9952ddf519d813ddcd2cc`, actual merged spec base `e27ef6c1f7ccf7afc431961890be39f05ef26999`. Ticket QSL-1826.

## Verdict

**CONDITIONAL for the PR delta** — one medium coverage defect in full-capacity admission must be corrected in this PR. Full repository qualification is not assessed as passing.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Budgeted-decimal tests derive all cumulative storage limits from the implementation's own request trace and never independently check full-capacity charging, so forwarding additional bytes instead of full new capacity still passes the asserted controls. | src/integer.rs:2044-2108 |

## Assurance Context

No applicable active AssuranceProfile was supplied or found in the examined local spec scope. No active exception used. This is a delegated PR review, not a complete repository or release qualification.

## Coverage

Frozen implementation candidate `d88d5a65ebbe1a7d40d9952ddf519d813ddcd2cc`, actual merged specification/main base `e27ef6c1f7ccf7afc431961890be39f05ef26999`. Both changed files (`src/integer.rs`, `src/lib.rs`) inspected in full, with FR-390 B7/AC8-9, TC-945, legacy decimal controls AC2-5 and existing Reservation semantics. No spec/plan, manifest, dependency, lockfile, CI or gate changes; no spec-review submethod applies. The input scalar's bit-size admission remains the enclosing consumer's responsibility. No bounded QSL raw-key adoption is claimed by this leaf PR.

API/implementation: `IntegerDecimalBudget` extends the existing public `Reservation`; both callbacks use allocation-free `ReservationDenied`. A private generic control shares one radix algorithm between legacy and budgeted calls. Budget denial maps directly to the closed Copy `Denied` variant while decimal byte/count/allocator errors retain their existing typed payload. No error-text branching, unsafe code, ecosystem dependency, copied formatter or semantic JSON encoding was introduced. Both deliberate public items and the method are re-exported/documented.

Resource/safety inspection: setup, each source limb, each radix multiply-add/append and output batches have preceding work admission. Scratch growth computes requested full capacity with checked arithmetic and admits it before `try_reserve_exact`; final output is separately admitted after exact checked byte counting. Each push has established capacity; radix arithmetic fits u64 under the maintained base-1e9/u32 carry invariant, and fixed digit buffers have widths 1..9. Partial scratch/output is local and dropped on refusal; the source and its lazy cache are borrowed without `as_big`, clone, formatting or hidden collection. Legacy canonical text/error payloads and public signatures are preserved. Concurrency/Tokio/wire dispatch are not changed.

Evidence inspected rather than rerun: `/tmp/qsl1826-decimal-tests.log` 23/23 decimal-filter tests; `/tmp/qsl1826-decimal-integer-tests.log` 22/22 integer tests with test-support; `/tmp/qsl1826-decimal-clippy-final.log` all-target/all-feature Clippy passes; `/tmp/qsl1826-decimal-fmt-check.log` fmt check passes with existing stable-toolchain warnings about nightly-only import options. `git diff --check` passes. No new heavy build, aggregate CI, Kani, cargo-deny, docs or no_std target build was run in this review. No dependency/safety delta justifies broadening the supplied focused gate evidence.

Reconciliation: `quoin matrix --repo /home/peter/dev/worktrees/QSL1826-decimal-owner --json` (quoin 0.29.1, quire 0.36.2/engine 0.50.2), exact output retained in `matrix.json`. The computed whole-repository matrix contains 47 requirements/297 criteria: tagged 261, method-without-symbol 23, untagged 13. All 297 report no run evidence in the store; inspected local logs are distinct evidence, not invented bound store records. FR-390 has nine tagged criteria; AC8 and AC9 bind the real new conversion tests. Whole-repository untagged criteria are FR-359-AC-9, FR-359-AC-10, FR-371-AC-6, FR-384-AC-5, FR-385-AC-1, FR-385-AC-2, FR-385-AC-3, FR-385-AC-4, FR-385-AC-5, FR-385-AC-6, FR-385-AC-7, FR-388-AC-3, FR-391-AC-4; these are outside the two-file behavioral delta and no repository-wide PASS is claimed or demanded from this PR. `quire coverage` reports seven unmatched-archetype diagnostics and 118 repository-wide unmatched trace symbols; none names FR-390. The module vocabulary does not match every repository tag (including TC tags), so these are limitations of a full-repository assurance claim, not invented new PR regressions.

Plan completion: not assessed

Scoped reverse inventory: three new public API behaviors (budget trait, typed result/error, budgeted method) are owned by FR-390 B7/AC8-9. The two private adapters and admission/output helper are internal parts of that behavior, not unowned capabilities. No changed source/test stub, false completion, ignored new case, or mock of the unit under test was found. No repository-wide reverse-coverage inventory is asserted. Optional semantic gap-analysis mode was not requested and was not run; code/Rust behavior and code-test alignment were reviewed under code-review.

## Examined Units

```yaml
scope:
- id: FR-390-AC-2
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: '`try_to_decimal_string` returns `0`, `1`, `-1` and independently supplied
    large positive and negative decimal literals byte for byte, matching `Display`
    on success without a leading plus, leading zero or negative zero. The large controls
    cross multiple internal magnitude limbs.'
- id: FR-390-AC-3
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: 'For each successful spelling in AC-2, a bound equal to its actual UTF-8
    length permits output, and a bound one byte smaller returns `ByteLimit { required:
    actual_length, limit }` with no output. The minus sign contributes one byte. A
    synthetic output count that cannot fit `usize` returns `OutputSizeOverflow` before
    output allocation under both a finite bound and `max_bytes = usize::MAX`; no `ByteLimit`
    result has `required <= limit`. Neither count nor byte-limit refusal is reported
    as allocation failure.'
- id: FR-390-AC-4
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: With the same prepared integers and a sufficient bound, injected refusal
    at every actual decimal scratch or output reservation yields `Allocation { requested
    }`, no partial output and an unchanged source. Each amount equals the minimum
    additional payload bytes of the failed request, including multiplication by element
    size where applicable; a synthetic checked-count overflow yields `usize::MAX`.
    An independent source inspection confirms that production calls the checked-count
    calculation tested with synthetic overflow inputs and has no hidden infallible
    allocation on either fallible route.
- id: FR-390-AC-5
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: A consumer can compile a call to both public fallible operations without
    consuming its `Integer`; copy and decimal failures are typed so that allocation
    can be mapped with its actual `requested` amount, decimal byte-limit refusal maps
    separately, and output-size overflow is distinguishable from both. The existing
    `Clone`, `Display`, parsing and mathematical operations retain their public signatures
    and successful numeric behavior.
- id: FR-390-AC-8
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: Budgeted decimal conversion returns independently supplied zero, positive,
    negative, inline and multi-limb literals byte for byte. Exact output-byte, cumulative
    storage and work bounds succeed; one-short output bytes report the existing byte
    refusal independently of the budget. Refusing each actual scratch or output reservation
    reports `Denied` with the caller's storage cause and no later reservation, preserving
    the source.
- id: FR-390-AC-9
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: examined
  excerpt: A zero work budget refuses before scratch or output allocation. A denied
    scratch reservation leaves the actual conversion vector empty with zero capacity.
    Cancellation triggered after admitted conversion scratch refuses at the next conversion
    work admission with the caller's cancellation cause; an uncancelled control follows
    the same budgeted algorithm and returns the literal bytes. One-short work refuses
    distinctly from output bytes and storage. No test depends on a fixed poll number,
    elapsed time or an allocation threshold.
- id: FR-390-AC-1
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: context_only
  excerpt: '`try_clone` returns an equal, independently owned value for zero, a small
    positive and negative value, and positive and negative values larger than one
    machine limb; changing or dropping the returned value leaves the source unchanged.
    Each actual copy reservation is injectable in a crate test: refusal yields `Allocation
    { requested }`, no copy and an unchanged source, with `requested` equal to that
    reservation''s minimum additional payload bytes.'
- id: FR-390-AC-6
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: context_only
  excerpt: For zero, `-7`, `i128::MAX`, `i128::MIN`, `2^127`, `-(2^127 + 1)`, `2^4096`
    and `-2^4096`, `copy_footprint` returns `limbs` of 0, 0, 0, 0, 4, 4, 129 and 129
    with `bytes` four times `limbs`, identically for the original and for a fresh
    `try_clone` of it. For each, the total bytes and allocation count observed around
    each actual `try_clone` and `Clone::clone` equal `bytes` and one allocation (none
    for an inline value).
- id: FR-390-AC-7
  path: spec/functional/FR-390-fallible-integer-copy-and-decimal-output.md
  role: context_only
  excerpt: Observing only the call, `copy_footprint` and `magnitude_bits` on a freshly
    copied value perform zero allocations for every AC-6 value, and `magnitude_bits`
    returns 1, 127, 128, 128, 128, 129 and 4097 for zero, `i128::MAX`, `i128::MIN`,
    `2^127`, `-(2^127 + 1)`, `2^128` and `2^4096`, for the original and for a fresh
    copy.
- id: TC-945
  path: spec/test-cases/TC-945-budgeted-decimal-conversion.md
  role: examined
  excerpt: 'Exercise the real `Integer::try_to_decimal_string_with_budget` conversion
    on

    independently supplied zero, signed inline and multi-limb decimal literals.

    Keep the source prepared before the conversion and preserve each caller''s

    typed denial cause.'
bindings:
- test_id: integer::tests::budgeted_decimal_preserves_literal_bytes_and_exact_bounds
  ac_id: FR-390-AC-2
  trace: correct
- test_id: integer::tests::budgeted_decimal_preserves_literal_bytes_and_exact_bounds
  ac_id: FR-390-AC-3
  trace: correct
- test_id: integer::tests::budgeted_decimal_preserves_literal_bytes_and_exact_bounds
  ac_id: FR-390-AC-8
  trace: correct
- test_id: integer::tests::budgeted_decimal_denies_each_actual_growth_and_work_without_losing_its_cause
  ac_id: FR-390-AC-8
  trace: correct
- test_id: integer::tests::budgeted_decimal_denies_each_actual_growth_and_work_without_losing_its_cause
  ac_id: FR-390-AC-9
  trace: correct
- test_id: integer::tests::decimal_scratch_denial_precedes_growth_and_cancellation_interrupts_conversion
  ac_id: FR-390-AC-9
  trace: correct
- test_id: integer::tests::large_decimal_tiny_caps_stop_at_admission_without_materializing_source_cache
  ac_id: FR-390-AC-8
  trace: correct
- test_id: integer::tests::large_decimal_tiny_caps_stop_at_admission_without_materializing_source_cache
  ac_id: FR-390-AC-9
  trace: correct
```

## Finding Detail

FND-001: the existing baseline-derived exact, prefix and one-short storage limits all use `control.requests`. If the adapter forwards `additional` rather than `capacity`, they measure the same wrong schedule and still pass. The independently prepared value `4294967296 = 2^32` needs two base-1e9 u32 chunks under the specified algorithm: requested scratch capacities 4 then 8 bytes, followed by ten output bytes (cumulative 22). Assert the requested schedule or equivalent actual-growth oracle so the additional-bytes counterfactual fails, without pinning allocator over-allocation or cancellation poll numbers.

## Disposition Scope — Round 1

Frozen fix `c3a815f44aa3804d6c0514ae30b07baf9d9fbf75` changes only 35 test lines. Production/API source is unchanged from the original reviewed candidate. Independently inspected the new `decimal_reservations_charge_full_scratch_capacity_not_growth_increment` test. Its [4,8,10] oracle follows the specified exact requested-capacity schedule for `2^32`, not allocator over-allocation, elapsed time, a cancellation poll offset or a tuned failure threshold. It tests a semantic cumulative storage cap rather than the system allocator.

Observed logs: `/tmp/qsl1826-decimal-capacity-red.log` shows the intended failing counterfactual ([4,4,10] != [4,8,10]); `/tmp/qsl1826-decimal-capacity-restored-tests.log` passes 24/24 decimal tests; `/tmp/qsl1826-decimal-capacity-clippy.log` passes all-target/all-feature Clippy. Fresh remote head and clean worktree match the frozen fix. No new production concern or additional heavy check is needed.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The prepared 2^32 literal now independently asserts requested capacities [4,8,10], exact cumulative22, denied scratch11 and denied final-output21. Forwarding additional bytes instead of capacity fails the new assertion with [4,4,10]; production restored byte-for-byte, decimal24/24 and all-target/all-feature Clippy pass. |

## Latest Verdict

**PASS for the PR delta** — the sole change finding is fixed. Whole-repository qualification remains unclaimed: the recorded matrix retains 13 unrelated untagged criteria, unmatched-archetype diagnostics and no run evidence in the store.
