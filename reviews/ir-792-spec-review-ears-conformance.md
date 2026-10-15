---
id: SR-11045
title: "IR-792 ears-conformance review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-exact; spec/functional/FR-380-bounded-cancellable-numeric-membership.md; spec/test-cases/TC-928-bounded-numeric-membership.md; ticket IR-792"
review_set: subset
---

## Summary

FR-380 Description and seven behavior clauses have explicit subjects, finite conditions, and measurable responses. The validator reports both new documents grammar-clean.

## Reviewed units

- FR-380-AC-1: With an authenticated B, Integer and reduced Rational membership agree with the inclusive existing rule at both endpoints, one outside each endpoint and a denominator boundary, without mutating operands.
- FR-380-AC-2: Exact-B value and every exact-B endpoint are accepted for comparison; one-over-B in each of the three Integer or six Rational operand positions returns typed unsupported size naming that position and measured bits before any comparison. A value-only guard mutant fails an endpoint case.
- FR-380-AC-3: Observed production comparison steps do not exceed 2L for Integer or 4L for Rational, with fixed-width per-step work and constant auxiliary storage on the allocation-free path; source inspection confirms magnitude checks and borrowed limb iteration do not hide input-sized work/allocation.
- FR-380-AC-4: A pre-cancelled original handle prevents input-sized work. Deterministically cancelling during a longest admitted comparison returns that cause before the next step/result; a fresh-handle mutant fails.
- FR-380-AC-5: Checked B-to-native/layout failure is typed capacity/resource refusal, not false, unsupported size or a traversal limit. If implementation requests storage, denial at the actual reservation returns a distinct typed allocation failure carrying its measured request and retry without denial agrees with a fresh run.
- FR-380-AC-6: The QSV supplied caller documents and tests the authentic magnitude ceiling for value plus every endpoint, invokes the qualified helper under one initiating logical event, preserves original cancellation and locus, and leaves evaluator charges unchanged. Without that carrier it returns `UnsupportedHelper::NumericDomain`; no numeric default is added.
- TC-928: Verify [FR-380](../functional/FR-380-bounded-cancellable-numeric-membership.md) through the production fallible kernel entry points and the real supplied caller after an authentic finite magnitude ceiling is available. Construct already reduced rationals and domains before observing the membership helper.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Reviewed FR-380-AC-1 through FR-380-AC-6 and TC-928 steps 1 through 5 on clean candidate a27498a. Source context: quire-exact Integer::magnitude_bits, magnitude_limbs, IntegerInterval::contains, RationalDomain::contains, DecimalType::try_contains and QSV supplied NumericDomain refusal. No further finding in this lens.
