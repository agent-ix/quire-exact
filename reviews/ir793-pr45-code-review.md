---
id: SR-11063
title: "IR-793 PR 45 retained numeric type equality code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; PR #45 head 10ae170d986d63dfeef46ea5fc05127cf1ddd8ce vs base 66c6e05; src/lib.rs, src/numeric_type_equality.rs; ticket IR-793"
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
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS** — Rust and code review found no implementation defect. The helper checks all four/eight endpoints in specified order before comparison; zero has one magnitude bit; the original Cancel is polled before bounds, before each limb pair, and before result. Borrowed limb iterators and fixed arrays use constant auxiliary storage; no native-size conversion or allocation is reachable in this function. Existing value membership remains unchanged. The full make ci receipt in /tmp/ir793-exact-ci-network.log passes fmt, clippy, default and test-support tests, no_std build, deny, and docs. The separate gap-analysis artifact records acceptance-test coverage.
