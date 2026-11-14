---
id: SR-17940486709564847047964
title: "IR-258 PR #166 Kani-tractable native Rational carrier gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-14
scope: "agent-ix/quire-exact; PR #166 @ c7f5c7bb on origin/main fec2780; IR-258; FR-408, TC-956; tests/ir258_native_rational.rs, src/native_rational/kani_proofs.rs, src/native_rational.rs, src/native_fraction.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-408
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-956
    type: reviews
---

## Summary

Ticket: IR-258. Planless gap analysis of FR-408 against the PR diff. Plan completion: not assessed. `quoin matrix --json` (quoin 0.30.1) reported every one of the repo's 410 criteria as `untagged`, including FR-398 whose tests on main carry `#[trace]`. That is a tool or config issue across the whole repo, not something this PR caused, so the AC-to-test mapping below was checked by hand from the `#[trace("TC-956", "FR-408-AC-n")]` attributes.

Manual mapping: AC-1 to AC-9 each have at least one tagged test in `tests/ir258_native_rational.rs` (AC-7 two, AC-9 three), and all 12 pass. AC-10 is inspection, covered in the code-review artifact. AC-11 is the Kani row, honestly PLANNED/PARTIAL. No production code lacks an owning requirement: `native_fraction.rs` is owned by FR-408's shared-module rule and FR-398/FR-403. No stubs, no `todo!`, and no tests that cannot fail.

## Verdict

**PASS with two low findings.** The tests back the FR-408 ACs they are tagged with. The host differential over the CG domain compares outcome, value, every charge event, counters and admissions at sufficient, exact and one-less limits for all eight operations. No test claims a Kani verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | AC-6 and TC-956 step 5 ask the test to "assert that no pair of `i64` parts is a capacity refusal for any operation over a boundary sample and a random sample". The test covers only a four-value boundary sample (16 pairs) and has no random sample. Behavior 19 also asks the differential to "list the capacity inputs exhaustively". The boundary-set test only asserts `capacity_cases > 0`; it never states which pairs are capacity cases, so a native path that refused more boundary pairs, in agreement with the computed `expected_capacity`, would not be noticed as a change in the listed set. Fix: add a seeded random `i64` sample to the no-refusal loop, and assert the boundary capacity count, or list the pairs, as a literal. | tests/ir258_native_rational.rs:451-468, tests/ir258_native_rational.rs:872-891 |
| FND-002 | low | TC-956 step 4 asks for the four orderings and equality "over every ordered pair of `{-1/2, 0/1, 1/3, 1/2, 2/3}` against a table written independently". The AC-5 test checks only spot literals. The CG differential against the owner covers these pairs, so behaviour is backed, but the test procedure and the test disagree. Fix: add the 25-pair table, or amend TC-956 step 4 to point at the differential. | tests/ir258_native_rational.rs:345-373, spec/test-cases/TC-956-kani-tractable-native-rational-carrier.md:21 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-408-AC-1 | examined | carrier_is_copy_without_drop_glue_and_allocation_free |
| FR-408-AC-2 | examined | from_i64_parts_admits_canonical_pairs_and_refuses_the_rest_in_order |
| FR-408-AC-3 | examined | owner_conversion_round_trips_and_refuses_oversize_parts |
| FR-408-AC-4 | examined | add_subtract_multiply_complete_to_the_reduced_value |
| FR-408-AC-5 | examined | ordering_and_equality_follow_the_numeric_order |
| FR-408-AC-6 | examined | capacity_names_the_first_failing_quantity_and_charges_nothing |
| FR-408-AC-7 | examined | charges_match_the_literal_schedule; denial_stops_at_the_named_charge_and_consumes_no_result_unit |
| FR-408-AC-8 | examined | equality_charges_the_owner_schedule_and_a_shared_meter_accumulates |
| FR-408-AC-9 | examined | CG-domain, boundary-set and 252-step differential tests |
| FR-408-AC-10 | context_only | inspection item, see the code-review artifact |
| FR-408-AC-11 | examined | Kani harnesses; PLANNED; no tagged test claims it |

## Dispositions

Round 1, fix commit 6fcae3f (from c7f5c7b), checked against the code on the branch.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The no-refusal loop now covers a 7-value boundary sample plus a seeded xorshift sample of canonical i64 pairs, 96 values in all. The boundary differential asserts the literal `CAPACITY_LISTING` (operation and cause, with counts) and that the completions are the complement. |
| FND-002 | fixed | Added `ordering_and_equality_agree_with_the_rank_table_over_twenty_five_pairs`, which checks the four orderings and equality over all 25 ordered pairs of the five values against their position ranks. |
