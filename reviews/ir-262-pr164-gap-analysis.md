---
id: SR-2026111319100500000002
title: "IR-262 PR #164 Kani-tractable native enum carrier gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-exact; PR #164 @ 7a219689 on origin/main 8a51569; IR-262; FR-407, TC-955; src/native_enum.rs, src/native_enum/kani_proofs.rs, tests/ir262_native_enum.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-407
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-955
    type: reviews
---

## Summary

Ticket: IR-262. Planless gap analysis of PR #164 against FR-407 and TC-955. Plan completion: not assessed.

`quoin matrix --repo .` (quoin 0.30.1) reports every FR-407 criterion `untagged`, and every criterion repo-wide (FR-402, FR-404 too), while `quire trace --id FR-407-AC-n` resolves the `rust-trace-attribute` claims for AC-1..AC-6 and AC-8. The matrix does not read this repo's `ix_trace_rs` tags in the current configuration; a tooling observation, not a PR defect. The mapping below is manual, from `quire trace` and reading each test.

- AC-1: `carrier_is_copy_without_drop_glue_and_constructor_takes_no_meter` (Copy bound, `needs_drop` const assert, which also excludes `NodeKey`/`VariantId`/`Arc`/`String`/`Box`/`Vec`), `construction_and_round_trip_allocate_nothing`, `comparison_allocates_nothing` (runs in the no-feature `cargo test` pass of `make test`), `parts_round_trip_and_equality_covers_all_four_fields`. No public-field check (FND-001).
- AC-2: `from_index_refuses_out_of_range_and_admits_the_edges`: (3,3), (4,3), (u32::MAX,3), (0,0), (u32::MAX,u32::MAX) refused; 0, 2 and u32::MAX-1 admitted; Meter-free signature by fn-pointer coercion in the AC-1 test.
- AC-3: `from_member_agrees_with_from_index_...` (ordered and unordered, three members), `from_member_refuses_wrong_rank_and_foreign_variant` (wrong rank, foreign variant, minted variant under another owner key).
- AC-4: two tests, all six operators over all nine pairs, independent `native` oracle; unordered orderings refused with no admission.
- AC-5: tag, count and flag differences for every operator, cause precedence, Meter equality before/after on every refusal.
- AC-6: sufficient, exact (2,1), one-less per limit kind ((1,1), (2,0)) and zero occurrences, literal `AdmittedCharge` sequence under `test-support`, consumed counters and full incomplete record; equality and ordering charged identically on one Meter.
- AC-7: Planned; lives in quire-semantic-value under its own ticket. No test here, correctly.
- AC-8: token scan test (see the code-review SR, FND-001).
- AC-9: Planned; harnesses written, no verdict. Untagged, correctly.

Host tests executed from the existing `ir262-target` binaries: 15/15 (no features), 14/14 (`test-support`). No untraced production code: every public item of `native_enum.rs` is exercised by a tagged test; the `u32::try_from` count refusal in `from_member` is unreachable (over 4 billion variants) and untested, acceptable.

## Verdict

**PASS with one low finding.** Every implemented AC has a real test that can fail; AC-7 and AC-9 are honestly untested and Planned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-407-AC-1 "has no public field" has no test. TC-955 step 1 asks for a struct-literal program failing with `E0451`; none exists, and the TC Status claims step 1 is implemented. Private fields are what keep `rank < count` true. Fix: add a `compile_fail,E0451` doctest on `NativeEnum` building it by struct literal (the repo already uses `compile_fail` doctests, e.g. `native_quantity`) | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:62, spec/test-cases/TC-955-kani-tractable-native-enum-carrier.md:18, src/native_enum.rs:29 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-407-AC-1 | examined | NativeEnum is Copy, has no drop glue, holds no NodeKey, VariantId, Arc, String, Box or Vec, and has no public field; construction, comparison and the round-trip conversion perform zero allocations under a counting allocator. |
| FR-407-AC-2 | examined | from_index refuses index == member_count, index == member_count + 1, u32::MAX against a smaller count and any index with a count of zero, each with IndexOutOfRange and no carrier, takes no Meter, never clamps or wraps, and admits 0 and member_count - 1. |
| FR-407-AC-3 | examined | from_member agrees with from_index for every member of an ordered and an unordered EnumShape, refuses a member whose rank differs from the shape's rank for its variant and a foreign variant with MemberNotInShape, and takes the count and ordered flag from the shape. |
| FR-407-AC-4 | examined | For the six operators over every ordered pair of an ordered enum of three members, the Boolean equals the native operator over the indices; for an unordered enum Equal and NotEqual equal the same and the four orderings are refused. |
| FR-407-AC-5 | examined | Operands with different tags, or the same tag but a different count or ordered flag, are refused with DistinctEnumDeclarations, which takes precedence over UnorderedEnumOrdering; a refused call leaves the Meter unchanged. |
| FR-407-AC-6 | examined | The charge points, order, limit kinds and amounts of Behavior 7, the denied charge and the incomplete record are as stated at a sufficient, an exact and a one-less limit, and equality and ordered comparison are charged identically on one Meter. |
| FR-407-AC-7 | examined | The differential of Behavior 10, in quire-semantic-value's tests/ under its own QSV ticket, passes beside the owner's compare_enum. |
| FR-407-AC-8 | examined | The comparison source contains no cfg(kani), loop, iterator adapter, allocation, num_bigint item, owner Integer, NodeKey or EnumValue. |
| FR-407-AC-9 | examined | PLANNED: the Kani harness of Behavior 9 returns an executed terminal VERIFIED verdict. |
