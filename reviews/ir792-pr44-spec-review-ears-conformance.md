---
id: SR-11062
title: "IR-792 PR 44 ears-conformance review"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-exact; spec/functional/FR-380-bounded-cancellable-numeric-membership.md; spec/test-cases/TC-928-bounded-numeric-membership.md; ticket IR-792; PR 44 head 75850d9"
review_set: subset
---

## Summary

The changed requirement statements have named actors and observable responses. The grammar check reports no findings; the QSL default, QSV no-B behavior and event order remain distinguishable obligations.

## Examined units

- `FR-380` (examined): When a supplied-admission caller checks an already constructed Integer or reduced Rational against an inclusive numeric domain, the exact kernel SHALL offer fallible membership under an explicit, finite magnitude ceiling supplied by that caller. The kernel SHALL retain mathematical membership, distinguish size and resource refusal from nonmembership, and observe the caller's original cancellation handle.
- `FR-380-AC-6` (examined): The QSV supplied caller documents and tests the authenticated finite B for value plus every endpoint, invokes the qualified helper after charging one initiating logical event, preserves original cancellation and locus, and leaves evaluator charges unchanged. A QSL caller resolving omitted B to QSpec FR-323's published 4096-bit default supplies that B; the exact helper and a direct QSV caller do not default B, and a direct QSV call without B returns `UnsupportedHelper::NumericDomain`.
- `TC-928` (examined): Verify [FR-380](../functional/FR-380-bounded-cancellable-numeric-membership.md) through the production fallible kernel entry points and the real supplied caller after an authentic finite magnitude ceiling is available. Construct already reduced rationals and domains before observing the membership helper.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean at PR #44 head `75850d9b6c894dfe0a5cc6b009179806a573a0a9`.
