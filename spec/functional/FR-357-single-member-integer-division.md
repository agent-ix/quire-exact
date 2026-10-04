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

A caller evaluates `10 / y` over `1..=10` at `y = 5`. The remainder 0 is outside the domain, but the expression exposes the quotient 2, so the result is 2.

## Behavior

1. **Typed outcomes.** A zero divisor is `Undefined(DivisionByZero)`. An exposed member outside a bounded domain is `Refused(DivisionMemberOutOfDomain { member })`, whose cause is `quotient-outside-domain` or `remainder-outside-domain`. Otherwise the member value completes.
2. **Laws.** Truncating rounds the quotient toward zero, floor toward negative infinity, and Euclidean leaves a nonnegative remainder.
3. **Exact.** `i64::MIN / -1` is the exact integer 2^63, never an overflow.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-357-AC-1 | Given a zero divisor under any profile and either member, `divide` returns `Undefined(DivisionByZero)`. | Test (TC-905) |
| FR-357-AC-2 | Given `10 / y` over `1..=10` at `y = 5`, `divide` of the quotient returns 2; given `x % -1` over `-10..=5` at `x = -10`, `divide` of the remainder returns 0. | Test (TC-905) |
| FR-357-AC-3 | Given a quotient outside the domain, `divide` refuses with cause `quotient-outside-domain`; given a remainder outside the domain, it refuses with `remainder-outside-domain`. | Test (TC-905) |
| FR-357-AC-4 | Given -7 and 2, truncating returns (-3, -1), floor (-4, 1) and Euclidean (-4, 1); given -7 and -2, truncating returns (3, -1), floor (3, -1) and Euclidean (4, 1). | Test (TC-905) |
| FR-357-AC-5 | Given `i64::MIN` and -1, `divide` of the quotient returns 2^63 in a domain that holds it and refuses in one that does not. | Test (TC-905) |

## Status

Implemented.

## Dependencies

- None. The caller's behaviour lives in `agent-ix/quire-spec-language`.
