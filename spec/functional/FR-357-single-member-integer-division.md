---
id: FR-357
title: "The kernel divides to one member under the selected law"
type: FR
relationships: []
---
# FR-357: The kernel divides to one member under the selected law

## Description

The `quire-exact` kernel SHALL offer `divide`, which evaluates the one member of the `(q, r)` pair that an expression exposes, under the selected `DivisionProfile` (truncating, floor or Euclidean). Both members are computed exactly; only the exposed member must be in the consumer's domain. The quotient is exposed by `div` and the remainder by `rem`. The language-level rule is the caller's and keeps the id `FR-147` in the repository that owns it.

## Use case

A caller evaluates `10 div y` over `1..=10` at `y = 5`. The remainder 0 is outside the domain, but the expression exposes the quotient 2, so the result is 2.

## Behavior

1. **Typed outcomes.** A zero divisor is `Undefined(DivisionByZero)`. An exposed member outside a bounded domain is `Refused(DivisionOutOfDomain { member })`, whose cause is `quotient-outside-domain` or `remainder-outside-domain`. Otherwise the member value completes.
2. **Laws.** Truncating rounds the quotient toward zero, floor toward negative infinity, and Euclidean leaves a nonnegative remainder.
3. **Exact.** `i64::MIN div -1` is the exact integer 2^63, never an overflow. In a signed 64-bit domain the quotient refuses and the remainder, 0, completes.
4. **Charges.** The admitted charge points are, in order, `integer-division.operands`, `integer-division.arithmetic`, `integer-division.domain` (one value occurrence) and `integer-division.result-retain` (one result unit). A zero divisor stops after operands, and a refusal stops after domain.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-357-AC-1 | Given a zero divisor under any profile and either member, `divide` returns `Undefined(DivisionByZero)`. | Test (TC-905) |
| FR-357-AC-2 | Given `10 div y` over `1..=10` at `y = 5`, `divide` of the quotient returns 2; given `x rem -1` over `-10..=5` at `x = -10`, `divide` of the remainder returns 0. | Test (TC-905) |
| FR-357-AC-3 | Given a quotient outside the domain, `divide` refuses with cause `quotient-outside-domain`; given a remainder outside the domain, it refuses with `remainder-outside-domain`. | Test (TC-905) |
| FR-357-AC-4 | Given -7 and 2, truncating returns (-3, -1), floor (-4, 1) and Euclidean (-4, 1); given -7 and -2, truncating returns (3, -1), floor (3, -1) and Euclidean (4, 1). | Test (TC-905) |
| FR-357-AC-5 | Given `i64::MIN` and -1 under any profile, `divide` of the quotient returns 2^63 in a domain that holds it. In a signed 64-bit domain the quotient refuses with `quotient-outside-domain` and the remainder completes with 0. | Test (TC-905) |
| FR-357-AC-6 | Given a successful `div` or `rem`, the meter admits `integer-division.operands`, `integer-division.arithmetic`, `integer-division.domain` (one value occurrence) and `integer-division.result-retain` (one result unit), in that order. A refusal admits no `result-retain`, a zero divisor admits only operands, and a result limit of 0 reports `integer-division.result-retain` as incomplete. | Test (TC-905) |

## Status

Implemented.

## Dependencies

- None. The caller's behaviour lives in `agent-ix/quire-spec-language`.
