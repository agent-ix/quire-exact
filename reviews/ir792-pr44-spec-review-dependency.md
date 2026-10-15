---
id: SR-11061
title: "IR-792 PR 44 dependency review"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-exact; spec/functional/FR-380-bounded-cancellable-numeric-membership.md; spec/test-cases/TC-928-bounded-numeric-membership.md; ticket IR-792; PR 44 head 75850d9"
review_set: subset
---

## Summary

The new FR-323 relationship points to the owning effective B carrier and preserves QSV consumer and quire-exact kernel responsibilities. The pending implementation dependency remains explicit.

## Examined units

- `FR-380-AC-6` (examined): The QSV supplied caller documents and tests the authenticated finite B for value plus every endpoint, invokes the qualified helper after charging one initiating logical event, preserves original cancellation and locus, and leaves evaluator charges unchanged. A QSL caller resolving omitted B to QSpec FR-323's published 4096-bit default supplies that B; the exact helper and a direct QSV caller do not default B, and a direct QSV call without B returns `UnsupportedHelper::NumericDomain`.
- `TC-928` (examined): Verify [FR-380](../functional/FR-380-bounded-cancellable-numeric-membership.md) through the production fallible kernel entry points and the real supplied caller after an authentic finite magnitude ceiling is available. Construct already reduced rationals and domains before observing the membership helper.
- `FR-323-AC-15` (context_only): A request's `limits.supplied.numeric_magnitude_bits` B (published default 4096; omitted resolves to it, so omitted and stated 4096 have the same request identity and 4095 or 4097 differ) admits a supplied Integer and Rational whose value and every value-in-domain operand (3 Integer, 6 Rational) have bit length at most B, with zero counted as one bit, so B = 0 admits nothing; at the default a 4096-bit operand admits and a 4097-bit operand refuses. An operand at B+1 bits refuses by the `limit` cause with field `supplied.numeric_magnitude_bits`, `limit` B, `consumed` 0, `next_charge` and `counter
- `FR-109-AC-7` (context_only): Each claimed supported helper path has a source-backed input bound, finite comparison/arithmetic and temporary-storage bound, cancellation response boundary and fallible storage route where needed. Unqualified Decimal and future union paths remain implementation dependencies; logical-event totals alone do not establish those helper premises.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean at PR #44 head `75850d9b6c894dfe0a5cc6b009179806a573a0a9`.
