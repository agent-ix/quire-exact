---
id: SR-11055
title: spec-review/dependency review of IR-793 exact-kernel equality
type: SpecReview
analysis: dependency
scope: agent-ix/quire-exact; spec/functional/FR-381-bounded-retained-numeric-type-equality.md,
  spec/test-cases/TC-929-bounded-retained-numeric-type-equality.md; ticket IR-793
review_set: subset
---

## Summary

Checked new FR-380 and QSV FR-117 reference edges and the TC-929 verification edge. FR-381 is the exact-kernel prerequisite for the QSV numeric type-link path; no cycle appears in this diff.

## Verdict

**PASS** — No defects found in this lens.

## Scope examined

- `FR-381` (examined): When a supplied-admission caller compares two retained Integer intervals or two retained Rational domains as type links, `quire-exact` SHALL offer a fallible equality operation under that caller's authenticated finite magnitude ceiling `B`.
- `FR-381-AC-1` (examined): With authenticated B, equal and unequal Integer intervals and Rational domains return the same Boolean equality as independent endpoint arithmetic, including negative, zero, and denominator endpoints, without mutation. Equal spellings or identity of borrowed objects do not substitute for endpoint equality.
- `FR-381-AC-2` (examined): Every one of the four Integer and eight Rational endpoint positions completes at exact B and returns typed unsupported size when made one bit over B, before endpoint comparison. Two oversized endpoints report the first in the stated order; an oversized endpoint on the right is never skipped by a left-only check.
- `FR-381-AC-3` (examined): Observed production comparisons use no more than 2L Integer or 4L Rational limb-pair steps, fixed-width work per step, and the measured temporary-storage bound, including inline/Large and 32-bit limb boundaries. Inspection finds no input-sized clone or unbounded fallback.
- `FR-381-AC-4` (examined): A pre-cancelled original handle prevents input-sized work; cancellation on a late observed limb step stops before the next step or equality result with the original cause.
- `FR-381-AC-5` (examined): Inspection establishes whether production performs native-size conversion or allocation. Each reachable denial is typed with its measured request and retry behavior; an allocation-free route with no failing conversion has no synthetic resource fixture.
- `TC-929` (examined): 2. Set B to the largest endpoint magnitude length. For each of the four Integer and eight Rational positions, exercise exact B and then a valid domain whose selected endpoint is B+1 bits. Assert the typed side/endpoint, measured bits and B before comparison. Make two positions oversized to verify the stated first-refusal order, including a small left domain with an oversized right endpoint.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
