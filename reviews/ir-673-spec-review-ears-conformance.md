---
id: SR-2421
title: "EARS and atomicity review of quire-exact IR-673 spec candidate 05ffdbe"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-exact@05ffdbe0f6b9f5a2be69fef52874eb1776e18cc3; origin/main...05ffdbe: spec/functional/FR-097, FR-358, FR-359, FR-361, FR-362, FR-363, FR-364, FR-365, FR-366, FR-367"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-361
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-362
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-364
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-365
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-366
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-367
    type: reviews
---
# EARS and atomicity review of quire-exact IR-673 spec candidate 05ffdbe

## Summary

Ticket: IR-673. Pre-PR candidate `05ffdbe0f6b9f5a2be69fef52874eb1776e18cc3`. Reviewer model `claude-opus-5-5`, run `720faf90-5d3c-40d8-be4d-1aaae6430466`.

`quire validate --scope . --summary` on the 20 changed documents reports 20/20 grammar-clean with 0 grammar findings. A manual read agrees on the statement patterns:

- FR-364 and the new FR-097 and FR-362 SHALLs are ubiquitous.
- FR-365 and FR-367 are event-driven ("When ...").
- FR-366 is event-driven plus one ubiquitous SHALL.

Every statement names the kernel as its subject and uses a single SHALL per sentence.

FR-097, FR-362 and FR-366 now carry two SHALLs each. Where that is an ownership problem it is recorded once in SR-2420 FND-004 and FND-002, not repeated here.

## Verdict

**PASS with two low findings.**

## Examined scope

| Unit | Role | Path:line | Excerpt |
| --- | --- | --- | --- |
| FR-364 | examined | spec/functional/FR-364-ieee-flag-iteration.md:11 | The kernel SHALL iterate an `IeeeFlags` set in `IeeeFlag::ALL` vocabulary order, independently of insertion order. |
| FR-365 | examined | spec/functional/FR-365-decimal-rounding-ties-and-default.md:11 | When rounding a decimal to scale zero, the kernel SHALL apply the selected `RoundingMode`. |
| FR-366 | examined | spec/functional/FR-366-ieee-exceptional-value-semantics.md:11 | When evaluating or converting IEEE exceptional values, the kernel SHALL preserve NaN selection, payload representability and signed-zero loss ... |
| FR-367 | examined | spec/functional/FR-367-text-profile-mismatch-before-charging.md:11 | When `compare_text` receives admitted text values with distinct profiles, the kernel SHALL return `Err(IllTyped { cause: DistinctTextProfiles })` before any meter charge. |
| FR-097, FR-362 (amended statements) | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:15 | The kernel SHALL construct rational values with a positive denominator and a unique zero representation `0/1`. |
| FR-361-AC-3 | examined | spec/functional/FR-361-admission-before-large-exact-work.md:31 | ... completes with `integer_bits` equal to the dividend magnitude bit length to the independently calculated floor quotient ... |
| FR-362-AC-10 | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:46 | For integer add, subtract, multiply and negate, rational add, subtract, multiply, divide and negate, and integer/rational ordering, independently calculate ... |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | low | FR-361-AC-3's added sentence is garbled: "completes with `integer_bits` equal to the dividend magnitude bit length to the independently calculated floor quotient". A reader must guess which phrase "to the ... quotient" attaches to. Fix: "completes to the independently calculated floor quotient, with final `integer_bits` equal to the dividend magnitude bit length". | spec/functional/FR-361-admission-before-large-exact-work.md:31 | wrong-requirement |
| FND-002 | low | FR-362-AC-10 bundles more than a dozen independent checks in one AC: exact and one-under bit limits for 11 operations, seven named fixtures, rational and integer work boundaries, and seven power-of-two result edges. FR-362-AC-7 and FR-358-AC-5 are similarly compound. A single binder cannot show which sub-obligation failed, and partial tagging looks like full coverage. Fix: split AC-10 into bit-limit, work-boundary and power-of-two ACs at least. | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:46 | wrong-requirement |

## Dispositions

Round 1, reviewed `41d35ea4741dcbe9a5df33961a9e6358c5357222` against prior `05ffdbe0f6b9f5a2be69fef52874eb1776e18cc3`. Model `claude-opus-5-5`, run `720faf90-5d3c-40d8-be4d-1aaae6430466`, session `5600b5d7-c445-43b9-8eba-c1cdee75cf9f`. Each outcome was checked against the 05ffdbe..41d35ea diff, owner source and `quire matrix`; the author's report was not relied on. Spec-only: nothing was built or run.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 41d35ea4741dcbe9a5df33961a9e6358c5357222 |
| FND-002 | fixed | 41d35ea4741dcbe9a5df33961a9e6358c5357222 |

### Round 1 after-excerpts

- FND-001: FR-361-AC-3 restored to original text; new FR-361-AC-7: On a fresh meter, floor division of a positive at-least-65,536-bit dividend by 3, exposing the quotient in the mathematical domain, completes to the independently calculated floor quotient with `integer_bits` limit equal to the dividend magnitude bit length. Final consumed `integer_bits` equals that exact bit length.
- FND-002: FR-362-AC-10 split into AC-10 (bit limits), AC-17 (work boundaries), AC-18 (power-of-two edges); FR-362-AC-7 additions into AC-14..16; FR-358-AC-5 additions into AC-8..11. AC-17: For integer `5 + 7`, work limit 2 stops at `integer-arithmetic.result-retain` after operands and arithmetic, with zero result units; work limit 3 completes to 12 with three work and one result unit. For rational `(2/3) * (3/2)`, work limit 2 stops at `rational-arithmetic.normalize`, work limit 3 stops at `rational-arithmetic.result-retain`, and work limit 4 completes to `1/1`; each denied call preserves exactly its...
