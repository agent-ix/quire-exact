---
id: SR-11050
title: "IR-792 PR 42 bounded numeric membership gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; PR #42 head 7d28552040da7334265c287260cee0a75da70b65 vs origin/main c269ebf; FR-380 AC1-5, TC-928 kernel steps 1-4, src/lib.rs, src/numeric_membership.rs; ticket IR-792"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-380
    type: reviews
---

## Summary

Planless diff-scoped spec-to-test-to-code audit of the bounded numeric kernel. The computed quire matrix binds AC1-4 to four changed tests and reports AC5 as method-without-symbol because its method is Inspection. The new source behavior has FR-380 ownership and is exercised through the production comparison functions. FR-380-AC-6 was read as QSV context only; this kernel PR makes no QSV completion claim.

## Reviewed units

- FR-380-AC-1 (examined): With an authenticated B, Integer and reduced Rational membership agree with the inclusive existing rule at both endpoints, one outside each endpoint and a denominator boundary, without mutating operands.
- FR-380-AC-2 (examined): Exact-B value and every exact-B endpoint are accepted for comparison; one-over-B in each of the three Integer or six Rational operand positions returns typed unsupported size naming that position and measured bits before any comparison. A value-only guard mutant fails an endpoint case.
- FR-380-AC-3 (examined): Observed production comparison steps do not exceed 2L for Integer or 4L for Rational, with fixed-width per-step work and constant auxiliary storage on the allocation-free path; source inspection confirms magnitude checks and borrowed limb iteration do not hide input-sized work/allocation.
- FR-380-AC-4 (examined): A pre-cancelled original handle prevents input-sized work. Deterministically cancelling during a longest admitted comparison returns that cause before the next step/result; a fresh-handle mutant fails.
- FR-380-AC-5 (examined): Inspection identifies whether the production comparison actually performs a fallible native conversion or storage request. For each reachable boundary, a denied checked conversion is typed capacity/resource refusal and an actual reservation denial carries its measured request; neither becomes false, unsupported size or traversal Limit, and retry after removing allocation denial agrees with a fresh run. When neither boundary exists, no artificial capacity/allocator refusal fixture or ceiling is required.
- FR-380-AC-6 (context_only): The QSV supplied caller documents and tests the authentic magnitude ceiling for value plus every endpoint, invokes the qualified helper under one initiating logical event, preserves original cancellation and locus, and leaves evaluator charges unchanged. Without that carrier it returns `UnsupportedHelper::NumericDomain`; no numeric default is added.
- TC-928 (examined, kernel steps 1-4): Verify FR-380 through the production fallible kernel entry points and the real supplied caller after an authentic finite magnitude ceiling is available.

## Verdict

PASS for the kernel slice. AC1-4 have real traced assertions for inclusive membership, every oversized operand position, observed limb steps and zero allocation, and original-handle cancellation. AC5 source inspection found no reachable native conversion or reservation boundary. Repo-wide `quire matrix` still marks AC6 untagged; QSV integration awaits an authenticated ceiling carrier and lies outside this diff. This subset PASS makes no claim that all of FR-380 or IR-792 is complete.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

- FR-380-AC-1 through AC-4: tagged by the four new tests in src/numeric_membership.rs; manually checked assertions against each criterion and TC-928 steps 1-4.
- FR-380-AC-5: Inspection; reviewed the actual magnitude and borrowed limb implementations, with no fallible native conversion or allocation path to deny.
- FR-380-AC-6: context only; computed matrix status `untagged`, pending the separate QSV caller slice. No QSV acceptance claim.
- Reverse gap: both new public methods, failure types, and observation seam belong to FR-380; no source stub, test stub, or unrelated behavior found in the diff.
- Plan completion: not assessed
