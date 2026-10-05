---
id: SR-005
title: "Gap analysis of quire-exact PR #2 (FR-357-AC-1..5, FR-096-AC-8)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@45c000f1e28f5b2443e636721b98ba2b8a5937f9; spec/functional/FR-357, FR-096; spec/test-cases/TC-905, TC-428; the tests that carry their trace tags in src/division.rs and src/outcome.rs; compared against agent-ix/quire-specification PR #188 head f0002b92"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-357
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-905
    type: reviews
---
# Gap analysis of quire-exact PR #2

## Summary

Ticket: IR-601. PR: quire-exact#2.

Each of FR-357-AC-1..5 has exactly one test tagged `#[trace("TC-905", "FR-357-AC-n")]` in
src/division.rs, and each test checks what its AC states. Every binding is correct.
FR-096-AC-8 stays bound to the TC-428 table test in src/outcome.rs, which now has 14 rows,
one per cause. The QSpec-TC-192 / QSpec-FR-147-AC-2 tag on the zero-divisor test is still
accurate.

Ids (owner question 5): FR-357 and TC-905 are the next free numbers after QSL's current
highest (FR-356, TC-904). Ids are scoped to their repo by their `ix://agent-ix/quire-exact/`
URI, so they are valid. QSL's next allocation may reuse the same numbers for something else.
That is not a defect, and no change is needed.

## Verdict

Request changes: one medium and one low finding. The acceptance criteria are covered. The
charge behaviour this PR changed has no requirement and no test in this repo.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The PR changes the division charges from 2 to 1 at the domain point and from 2 to 1 at the result point. A refusal also skips the result charge. No AC in FR-357 owns this sequence, and no test checks it. `Meter::admitted_charges` (test-support) exists for this, but no division test uses it. A wrong amount or order would pass `make ci`. QSpec #188 states it in FR-147-AC-6, DIV-08 and DIV-15. Fix: add an FR-357 AC for the four named charges with their amounts, and a test that checks the admitted sequence for `div` and `rem`, the missing result charge on a refusal, and an exact-bound limit with one too few. | src/division.rs:141-163; spec/functional/FR-357-single-member-integer-division.md |
| FND-002 | low | `exposed_member_outside_domain_refuses_with_its_cause` runs only `DivisionProfile::Truncating`, and checks `code()` only for the quotient case. QSpec #188 FR-147-AC-9 says "under each profile". Fix: loop over `DivisionProfile::ALL` (both vectors keep q or r of the same sign, so they hold under all three laws) and check the code in both cases. | src/division.rs:276-303 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 8137134a5aad2ac467c5905304fda644e9a13ea5 |
| FND-002 | fixed | 8137134a5aad2ac467c5905304fda644e9a13ea5 |
