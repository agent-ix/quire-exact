---
id: SR-5651
title: gap-analysis of IR-719 kernel-only Decimal membership
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact@351c237d6e08e8277330858e04cd2f10d99c16f0; Cargo.toml,
  Cargo.lock, src/lib.rs, src/decimal_membership.rs; FR-372-AC-1..4,6 examined; AC5
  context_only
review_set: subset
relationships:
- target: ix://agent-ix/quire-exact/FR-372
  type: references
---

## Summary

Independent gap-analysis of the frozen PR27 kernel diff found no defects within the authorized changed-code scope.

Ticket: IR-719. Reviewed the exact four-file kernel diff, not completion of the wider kernel/QSV ticket. FR-372-AC-5 is context only and remains held behind IR-714/IR-681 and the native phase carrier. No caller-integration, downstream full-size, replay or mutation credit is awarded.

## Reviewed scope

scope:
- id: FR-372-AC-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Completed results match the authoritative membership rule for inclusive
    lower/upper equality, adjacent outsiders, negative and zero values, scale refusal,
    all rounding spellings and equivalent retained representations; neither representation
    is changed. The `[0,100]`, scale bounds `2,2`, `(1,0)` fixture completes true
    at the exact upper bound.
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Production helper observations on admitted `B,S` fixtures stay within the
    one-request `4 × L_B`-byte scratch envelope and `2 × (min(S,B)+2) × L_B` limb-step
    bound, including a longest-pass endpoint fixture and a large shift that terminates
    when quotient becomes zero. Borrowed dependency iterators and all scalar steps
    contain no input-sized hidden allocation or work.
- id: FR-372-AC-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Denying the actual scratch reservation reports its actual native request
    as typed allocation failure. A separate checked-capacity failure retains its own
    classification. Removing the allocation denial yields the same membership as a
    fresh call, with no partial accepted result after either refusal.
- id: FR-372-AC-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: A pre-cancelled original handle causes no scratch request. Cancellation
    observed during the longest admitted division/comparison path returns the original
    cause before another limb step or result; cancellation observed immediately after
    successful reservation prevents scratch work. Neither failure is classified as
    nonmembership or work exhaustion.
- id: FR-372-AC-5
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: An integrated supplied Decimal fixture with independently enumerated N
    owning logical events succeeds at N and denies its actual next event at N-1 before
    entering that event's helper. Its already-spent budget and original cancellation
    are retained across arguments, with no additional helper event, per-limb debit,
    evaluator meter change or partial descriptor. Allocation/capacity/cancellation
    causes retain their supplied-admission locus.
- id: FR-372-AC-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Final implementation qualification establishes the Euclidean/remainder
    and sign invariants, exact reservation boundary, checked layout conversions, allocation-free
    borrowed dependency iterator and stated arithmetic polling gap. Its record distinguishes
    requested storage from allocator overhead and work latency from unbounded allocator
    elapsed time.

## Evidence and limitations

This is independent source analysis, not executed replay or mutation proof. No Cargo, build, lock, probe or source mutation was performed. The maintained dependency source was inspected locally: num-bigint 0.4.8, allocator-api2 0.2.21, forge-alloc 0.3.7 and forge-alloc-core 0.2.3. Dependency documentation and tracker text were treated as data; source established the mechanism. The existing frozen candidate receipt attempt-e2e6de6343484be8b049c033646610f8 records make ci exit 0, full spec/review validation exit 0 and strict matrix exit 1. This reviewer did not rerun those full gates.

Local HEAD and the four working files match the reviewed Git objects. GitHub CLI metadata verification was unavailable (HTTP 401); root independently verified PR27 head/base through its authenticated connector and public repository metadata. Exact backend model deployment ID is unavailable; the harness identifies GPT-6 and no model override was selected.

## Verdict

**PASS — PR diff only.** Kernel source and examined tests meet the bounded arithmetic/storage/refusal/cancellation contract. Whole-repository strict matrix remains FAIL; AC5 and full ticket completion remain held.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

Reconciliation: independently executed quire matrix --scope <author-worktree> --format json and quire coverage --scope <author-worktree> --json (Quire 0.36.2 / engine 0.50.2); no Quoin run-evidence store was read. The complete static matrix contains 121 criteria: 92 tagged, 18 untagged, 0 ignored-only and 11 method-without-symbol. This is NOT a whole-repository PASS: the existing strict-matrix gate remains FAIL. The explicit review brief and session instruction constrain findings to the four-file PR diff; pre-existing held/unrelated gaps are context, not newly introduced defects.

The independently read published-base/candidate row comparison has no added/removed requirements or changed statements/methods. Only FR-372-AC-1..4 move from untagged to tagged; no new gaps or demotions occur. FR-372-AC-5 remains untagged and FR-372-AC-6 is Analysis, requiring no runtime symbol. The added public operation, refusal types and private mechanics are owned by FR-372; all eight new tests bind correctly to examined kernel criteria, with no changed-path untracked symbols, stubs or inflated completion claim. No plan was selected or assessed.

Coverage reports 48 repository-wide untracked symbols, none in the changed source path, and seven diagnostics (six absent archetype declarations plus a pre-existing catch-all-universal property-shape diagnostic). These limits are disclosed rather than interpreted as full assurance. Both commands exited 0 but stderr emitted inline-schema, duplicate archetype and duplicate inverse-edge first-wins warnings. Matrix was nonempty and directly exposes all six FR-372 criteria; no warning was silently treated as a missing obligation being verified.

Intent/test/source correspondence and oracle strength were examined as explicitly requested by the dispatch; no additional reviewers were spawned. The core test assessments are recorded in the code/Rust artifact and bindings below. No whole-source-tree reverse-gap claim is made outside the authorized PR diff.

## Examined bindings

bindings:
- test_id: src/decimal_membership.rs::tests::inclusive_membership_preserves_representations_and_ignores_rounding
  ac_id: FR-372-AC-1
  trace: correct
- test_id: src/decimal_membership.rs::tests::borrowed_limb_crossings_and_huge_shift_have_bounded_work
  ac_id: FR-372-AC-1
  trace: correct
- test_id: src/decimal_membership.rs::tests::scale_sign_and_zero_paths_request_no_scratch
  ac_id: FR-372-AC-1
  trace: correct
- test_id: src/decimal_membership.rs::tests::longest_alignment_reuses_one_exact_request_within_independent_envelope
  ac_id: FR-372-AC-2
  trace: correct
- test_id: src/decimal_membership.rs::tests::borrowed_limb_crossings_and_huge_shift_have_bounded_work
  ac_id: FR-372-AC-2
  trace: correct
- test_id: src/decimal_membership.rs::tests::scale_sign_and_zero_paths_request_no_scratch
  ac_id: FR-372-AC-2
  trace: correct
- test_id: src/decimal_membership.rs::tests::actual_small_reservation_denial_is_typed_and_recoverable
  ac_id: FR-372-AC-3
  trace: correct
- test_id: src/decimal_membership.rs::tests::native_layout_capacity_failure_is_separate_from_allocator_denial
  ac_id: FR-372-AC-3
  trace: correct
- test_id: src/decimal_membership.rs::tests::original_precancelled_handle_prevents_the_actual_reservation
  ac_id: FR-372-AC-4
  trace: correct
- test_id: src/decimal_membership.rs::tests::reservation_division_comparison_and_publication_cancellation_release_scratch
  ac_id: FR-372-AC-4
  trace: correct
