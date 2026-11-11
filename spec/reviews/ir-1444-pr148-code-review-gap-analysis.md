---
id: SR-17940486709564847047682
title: "IR-1444 PR #148 native Decimal carrier gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #148 @ 7b451716; IR-1444; FR-402-AC-1 to FR-402-AC-12, TC-950, FR-403 seam (Inputs, Behaviors 3, 10, 31-34, Dependencies); src/native_decimal.rs, src/native_decimal/**, tests/ir1444_native_decimal.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-950
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
---

## Summary

Ticket: IR-1444. Planless gap analysis over `quire matrix --scope .` at head 7b451716, the tagged tests and the new source. Plan completion: not assessed.

The matrix shows FR-402-AC-1 to AC-10 and AC-12 as `tagged`, and AC-11 as `method-without-symbol`. All new production code is owned by FR-402, and none is orphaned. The crate-private `admits_normalized` is there for FR-403 to call.

Seam check against FR-403. FR-403 needs these operand and type accessors, and each exists: `coefficient() -> i128`, `scale() -> u32`, `normalized() -> (i128, u32)`, `from_retained` (FR-403 Inputs builds operands "by the total retained-pair constructor of FR-402 Behavior 3"), `NativeDecimalType::max_scale()` (target T) and `rounding()`. Membership of the placed result (Behavior 10) can be decided crate-internally by `normalized()` followed by `admits_normalized`. FR-403 Behavior 32 allows the native types to carry `Decimal` in their names, which agrees with this PR's inspection rule.

## Verdict

**CHANGES REQUIRED: the AC-7 coverage gap (code-review FND-001) is the blocking item. The two findings here are low.**

On the matrix, AC-10 shows as `tagged` only through the host reference-binding tests (src/native_decimal/tests.rs:51 and :91). That is a matrix artifact, not proof evidence. TC-950 step 5 itself includes the host binding as the host half of AC-10, and the FR-402 Status says plainly that AC-10 and AC-11 are not credited, that the proofs are unexecuted and that no measurement exists. The wording is honest, and the tag is acceptable as it stands.

AC-11 correctly waits on IR-259.

FR-402 Behavior 7 names no API for the normalized view or the read-out. `normalized()` mirrors the owner's `normalized()`, which Behavior 7 itself cites, and `to_decimal()` mirrors `Decimal::to_rational`. Both names are acceptable. No spec amendment is needed in this PR; FR-403 can cite the names when IR-1508 lands.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-403 Dependencies says "IR-1444 (merged, quire-exact PR 146) delivered the carrier, the native type and the constructor of FR-402". PR 146 was the spec only. The code is this PR, #148, which has not merged. FR-403 would therefore cite the wrong delivery PR for its operand. Correct the sentence to PR 148, either in this PR (a one-line spec edit) or as the first edit under IR-1508. | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:122 |
| FND-002 | low | `admits_normalized(coefficient, scale)` is correct only when given a normalized pair, and nothing enforces that. A retained pair gives the wrong answer: (100, 2) in Decimal[0,100;0,1] has s* = 2 > smax and returns false, where the owner admits it. FR-403's result-membership check (Behavior 10) is the next caller. Make the precondition impossible to miss: take a `&NativeDecimal` and normalize inside, or add a `debug_assert_eq!((coefficient, scale), NativeDecimal::from_retained(coefficient, scale).normalized())`. | src/native_decimal.rs:129-151 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-402-AC-1 | examined | NativeDecimalType::new returns MalformedDecimalType for (1,0,0,2), (0,1,3,2), (0,1,0,4294967296) ... ingress ... TypeNotRepresentable |
| FR-402-AC-2 | examined | The constructor retains the drawn pair and judges membership on the normalized pair. |
| FR-402-AC-3 | examined | The lift to smin and its overflow are exercised. |
| FR-402-AC-4 | examined | The scale range and the maximum scale are exact. |
| FR-402-AC-5 | examined | Canonical and trailing-zero forms are kept apart. |
| FR-402-AC-6 | examined | the read-out Decimal has representation() equal to the owner's DecimalRepresentation::new(...) ... and its normalized() equals the native normalized view |
| FR-402-AC-7 | examined | no cfg(kani) token ... no for, while, loop or iterator adapter ... zero heap allocations ... around the calls |
| FR-402-AC-8 | examined | Over the boundary set of TC-950, the constructor and the owner route ... agree on admission and on the retained and normalized pairs |
| FR-402-AC-9 | examined | each such program fails to compile with its pinned error code (E0308 ..., E0451 ...), with a compiling positive twin |
| FR-402-AC-10 | examined | A Kani proof with no unwind bound verifies the constructor and a second proof verifies the normalized view ... not credited unless ... |
| FR-402-AC-11 | examined | When CG's decimal row (CG FR-051, IR-259) consumes the delivered carrier, the peak memory ... are recorded under IR-1444 |
| FR-402-AC-12 | examined | ... read through an i128 accessor ... read through a u32 accessor |
| FR-403 | context_only | Inputs: carried Decimals built from bindings, from an earlier result or by the total retained-pair constructor of FR-402 Behavior 3; Behavior 10 membership; Behaviors 31-34 |
| FR-398 | context_only | the six causes are CarrierMismatch, TypeNotRepresentable, ZeroDenominator, NonCanonical, OutsideCarrierDomain and NotRepresentable |

## Dispositions

Round 1, verified at 5331d53e. No new findings.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-403:122 now reads "IR-1444 (specified in quire-exact PR 146, implemented in PR 148) delivers the carrier, the native type and the constructor of FR-402". |
| FND-002 | fixed | `admits_normalized` is now private and takes a crate-private `NormalizedPair`, whose fields are private and which only `NativeDecimal::normalized_pair()` produces. The crate-facing seam is `NativeDecimalType::admits(&NativeDecimal)`, which normalizes internally, and `from_bindings` uses it. `membership_is_judged_on_the_normalized_pair_even_for_a_retained_pair` checks (100,2) in Decimal[0,100;0,1] (admitted by the owner, by `admits` and by `from_bindings`) and (1,2) (refused). The reviewer's retained-pair mutant failed it. FR-403 names no membership API, and its Behavior 10 needs only a native membership test of the placed result, which `ty.admits(&NativeDecimal::from_retained(v x 10^T, T))` provides. No mismatch with FR-403. |
