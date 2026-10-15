---
id: SR-11049
title: "IR-792 PR 42 bounded numeric membership code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; PR #42 head 7d28552040da7334265c287260cee0a75da70b65 vs origin/main c269ebf; src/lib.rs, src/numeric_membership.rs; ticket IR-792"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-380
    type: reviews
---

## Summary

Rust-review lane folded into code-review. Reviewed the two changed source files against FR-380 AC1-5 and TC-928 kernel steps 1-4. Checked Integer's inline/Large representation and borrowed `num-bigint` digit iterator, Rational domain accessors, and the original Cancel handle as supporting context. FR-380-AC-6 is the QSV supplied-caller requirement and was read as context only.

## Reviewed units

- FR-380-AC-1 (examined): With an authenticated B, Integer and reduced Rational membership agree with the inclusive existing rule at both endpoints, one outside each endpoint and a denominator boundary, without mutating operands.
- FR-380-AC-2 (examined): Exact-B value and every exact-B endpoint are accepted for comparison; one-over-B in each of the three Integer or six Rational operand positions returns typed unsupported size naming that position and measured bits before any comparison. A value-only guard mutant fails an endpoint case.
- FR-380-AC-3 (examined): Observed production comparison steps do not exceed 2L for Integer or 4L for Rational, with fixed-width per-step work and constant auxiliary storage on the allocation-free path; source inspection confirms magnitude checks and borrowed limb iteration do not hide input-sized work/allocation.
- FR-380-AC-4 (examined): A pre-cancelled original handle prevents input-sized work. Deterministically cancelling during a longest admitted comparison returns that cause before the next step/result; a fresh-handle mutant fails.
- FR-380-AC-5 (examined): Inspection identifies whether the production comparison actually performs a fallible native conversion or storage request. For each reachable boundary, a denied checked conversion is typed capacity/resource refusal and an actual reservation denial carries its measured request; neither becomes false, unsupported size or traversal Limit, and retry after removing allocation denial agrees with a fresh run. When neither boundary exists, no artificial capacity/allocator refusal fixture or ceiling is required.
- FR-380-AC-6 (context_only): The QSV supplied caller documents and tests the authentic magnitude ceiling for value plus every endpoint, invokes the qualified helper under one initiating logical event, preserves original cancellation and locus, and leaves evaluator charges unchanged. Without that carrier it returns `UnsupportedHelper::NumericDomain`; no numeric default is added.
- TC-928 (examined, kernel steps 1-4): Verify FR-380 through the production fallible kernel entry points and the real supplied caller after an authentic finite magnitude ceiling is available.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for the frozen kernel diff. All three Integer and six Rational operands are checked before comparison; borrowed sign, bit-length and limb iteration impose at most two/four bounded comparisons. The retained `num-bigint` iterator borrows its digit slice and advances with fixed-width operations; `magnitude_bits` reads the top digit and length. No native-size conversion or storage reservation occurs on the production path, so FR-380-AC-5 requires no fabricated capacity or allocator refusal. The same caller Cancel is polled before validation, each limb pair, and result publication. Test tags bind AC1-4; AC5 is an Inspection criterion. No full build was run in this reviewer-only pass; the PR reports a passing pre-PR `make ci` gate, which this review did not independently rerun.
