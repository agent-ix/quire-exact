---
id: SR-11052
title: spec-review/base review of IR-793 exact-kernel equality
type: SpecReview
analysis: base
scope: agent-ix/quire-exact; spec/functional/FR-381-bounded-retained-numeric-type-equality.md,
  spec/test-cases/TC-929-bounded-retained-numeric-type-equality.md; ticket IR-793
review_set: subset
---

## Summary

Reviewed FR-381 and TC-929 against merged FR-380/TC-928 and QSV FR-109/FR-117. Identity, operand order and bound rejection are stated, but the test case misses B = 0.

## Verdict

**FAIL** — The endpoint refusal order contradicts the authoritative QSpec type-link envelope; the other finding requires a zero-bound fixture.

## Scope examined

- `FR-381` (examined): When a supplied-admission caller compares two retained Integer intervals or two retained Rational domains as type links, `quire-exact` SHALL offer a fallible equality operation under that caller's authenticated finite magnitude ceiling `B`.
- `FR-381-AC-1` (examined): With authenticated B, equal and unequal Integer intervals and Rational domains return the same Boolean equality as independent endpoint arithmetic, including negative, zero, and denominator endpoints, without mutation. Equal spellings or identity of borrowed objects do not substitute for endpoint equality.
- `FR-381-AC-2` (examined): Every one of the four Integer and eight Rational endpoint positions completes at exact B and returns typed unsupported size when made one bit over B, before endpoint comparison. Two oversized endpoints report the first in the stated order; an oversized endpoint on the right is never skipped by a left-only check.
- `FR-381-AC-3` (examined): Observed production comparisons use no more than 2L Integer or 4L Rational limb-pair steps, fixed-width work per step, and the measured temporary-storage bound, including inline/Large and 32-bit limb boundaries. Inspection finds no input-sized clone or unbounded fallback.
- `FR-381-AC-4` (examined): A pre-cancelled original handle prevents input-sized work; cancellation on a late observed limb step stops before the next step or equality result with the original cause.
- `FR-381-AC-5` (examined): Inspection establishes whether production performs native-size conversion or allocation. Each reachable denial is typed with its measured request and retry behavior; an allocation-free route with no failing conversion has no synthetic resource fixture.
- `TC-929` (examined): 2. Set B to the largest endpoint magnitude length. For each of the four Integer and eight Rational positions, exercise exact B and then a valid domain whose selected endpoint is B+1 bits. Assert the typed side/endpoint, measured bits and B before comparison. Make two positions oversized to verify the stated first-refusal order, including a small left domain with an oversized right endpoint.

- `FR-323-AC-16` (context_only): NORM, not yet implementation-qualified (depends on the IR type-link follow-up; the bounded cancellable type-to-type equality helper and its QSV caller are not built): for a nested retained type link comparison, the `type-comparison` event is charged first and then every numeric endpoint (4 Integer, 8 Rational) is checked against B by the AC-15 bit-length rule; the first endpoint over B refuses by the same `limit` cause with `event: type-comparison` and a `type-link-` `operand`, before the endpoint equality runs, and an endpoint at exactly B admits.
- `FR-117-AC-2` (context_only): With authentic B, equal retained domains continue and a completed unequal domain returns invalid type at the compared link. Every one of the four Int and eight Rational endpoint positions at B completes; one-over-B on either side is projected as typed unsupported size with position, measured bits, B and rooted locus, never as unequal or traversal Limit.
- `FR-380` (context_only): This requirement assigns no numeric default and does not treat a logical event ceiling, a value-only bound, or a type interval as proof of `B`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-929 omits B = 0. Because zero has magnitude length one, even an all-zero domain must return typed unsupported size at B = 0; the exact-B fixture cannot cover this boundary. | spec/test-cases/TC-929-bounded-retained-numeric-type-equality.md:24-29 |
| FND-002 | high | FR-381 scans alternating actual-left/expected-right endpoints, contradicting QSpec FR-323 AC-16's expected-first type-link order. With expected-lower and actual-lower both oversized, FR-381 reports actual while the authoritative envelope requires expected. Use expected lower/upper then actual lower/upper, and for Rational each side's numerator interval before denominator interval; update TC-929's first-refusal oracle. | spec/functional/FR-381-bounded-retained-numeric-type-equality.md:45-50,80 |
