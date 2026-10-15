---
id: SR-11064
title: "IR-793 PR 45 retained numeric type equality gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; PR #45 head 10ae170d986d63dfeef46ea5fc05127cf1ddd8ce vs base 66c6e05; src/numeric_type_equality.rs, spec/functional/FR-381-bounded-retained-numeric-type-equality.md, spec/test-cases/TC-929-bounded-retained-numeric-type-equality.md; ticket IR-793"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-381
    type: reviews
---

## Summary

Reviewed PR #45 at 10ae170d986d63dfeef46ea5fc05127cf1ddd8ce against base 66c6e05, FR-381 AC1-5, TC-929 and the expected-before-actual envelope rule attributed to QSpec FR-323. The changed files are src/lib.rs and src/numeric_type_equality.rs.

## Scope examined

- FR-381-AC-1 (examined): With authenticated B, equal and unequal Integer intervals and Rational domains return the same Boolean equality as independent endpoint arithmetic, including negative, zero, and denominator endpoints, without mutation. Equal spellings or identity of borrowed objects do not substitute for endpoint equality.
- FR-381-AC-2 (examined): At B >= 1, every one of the four Integer and eight Rational endpoint positions completes at exact B and returns typed unsupported size when made one bit over B, before endpoint comparison. Two oversized endpoints report the first in the stated expected-before-actual order. At B = 0, even zero (one magnitude bit) is refused at the first expected endpoint.
- FR-381-AC-3 (examined): Observed production comparisons use no more than 2L Integer or 4L Rational limb-pair steps, fixed-width work per step, and the measured temporary-storage bound, including inline/Large and 32-bit limb boundaries. Inspection finds no input-sized clone or unbounded fallback.
- FR-381-AC-4 (examined): A pre-cancelled original handle stops before bound validation, including when the first endpoint exceeds B; cancellation on a late observed limb step stops before the next step or equality result with the original cause.
- FR-381-AC-5 (examined): Source inspection establishes whether production performs native-size conversion or allocation. For each reachable boundary, an executed denial/retry test establishes the typed measured request and retry result. An allocation-free route with no failing conversion has no synthetic resource fixture.
- TC-929 (examined): Steps 1-4, including all exact-B and one-over-B endpoint cases, borrowed limb bounds, and cancellation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Rational exact-B evidence omits the denominator lower endpoint: the baseline uses one bit there while B is eight | src/numeric_type_equality.rs:426 |

## Verdict

**CONDITIONAL** — FR-381-AC-2 is tagged, but its exact-B assertion does not exercise all eight Rational endpoint positions. A mutant that applies a stricter limit only to denominator lower would survive this test. Construct a valid Rational domain with denominator lower and upper both eight bits, then assert successful comparison at B=8; keep the existing one-over-B cases.

## Coverage

Plan completion: not assessed

The strict computed matrix finds FR-381-AC-1 through AC-5 tagged by the new tests. This diff-scoped finding concerns semantic coverage within AC-2, not tag presence. Production source has no input-sized clone, unchecked native-size conversion, allocation, fallback, or unowned behavior in the changed path. The wider repository matrix reports 19 untagged criteria outside this diff, which this review did not attribute to PR #45. The existing full make ci receipt passes; spec validation receipt /tmp/ir793-validate.log reports 1/1 docs grammar-clean.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At head 2152d1e553d280c0b6786fe0c684319fd6afeca4, the Rational test compares a valid domain whose denominator lower and upper are both exactly eight bits against its clone at B=8. This covers expected and actual denominator lower; a denominator-lower-only `>= B` mutant would return unsupported size and fail the Ok(true) assertion. The focused test passed. |
