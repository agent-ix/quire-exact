---
id: SR-17940486709564847047636
title: "IR-1444 PR #146 FR-402 EARS conformance analysis"
type: SpecReview
analysis: ears-conformance
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #146 @ 1343d2aa on origin/main 9403a5c; IR-1444; spec/functional/FR-402-kani-tractable-native-decimal-carrier.md, spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-950
    type: reviews
---

## Summary

Ticket: IR-1444. Spec-only PR #146 (head 1343d2aa, branch spec/ir-1444-decimal-carrier, on origin/main 9403a5c) adding FR-402 and TC-950, a Kani-tractable native Decimal carrier built from the owner's Decimal bindings. Method: quoin spec-review/ears-conformance. Reviewed against quire-exact origin/main 9403a5c src/decimal.rs (DecimalRepresentation::normalized, Decimal::new, DecimalType::new, DecimalType::contains, compare_shifted, shifted_digits), src/decimal_membership.rs, src/native_integer.rs, src/integer.rs (From<i64>/From<i128>), Cargo.toml (no profile.dev override); merged FR-398/TC-946 and spec/reviews/ir-1443-*; quire-contract-codegen origin/main spec/kani/functional FR-051, FR-025 (AC-15), FR-028, spec/oracle/functional/FR-014; quire-specification FR-140; Linear IR-1444, IR-1508. Read-only: no cargo, no Kani, no edits to the author's worktree.

## Verdict

CHANGES REQUIRED (low only). The Description statement, Behaviors 1-3, 5, 8 and 9 conform (ubiquitous 'The <system> SHALL', with 'If ... then ... SHALL' for the unwanted-behaviour refusals in 2 and 5, and 'When ...' for the event-driven 10). Behavior 6's first sentences are descriptive, not requirements, which is acceptable as rationale. Two statements put the obligation on the wrong subject or omit their trigger.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Behavior 4 says the constructor 'SHALL return the carried Decimal retaining exactly the pair drawn' with no condition, which Behavior 5's refusals contradict. Use the EARS trigger: 'When the binding is admitted (Behavior 5), the constructor SHALL return the carried Decimal retaining exactly the pair drawn'. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:48 |
| FND-002 | low | Behavior 7 puts a SHALL on the owner ('The owner Decimal computes its own normalized form, which SHALL equal the native normalized view'), although the owner is declared unchanged. Make the native view the subject: 'The normalized view SHALL equal the owner's normalized() of the read-out'. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:51 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-402 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:26 | The kernel SHALL provide a native Decimal carrier, a native Decimal type and a constructor that builds a carried value of that type from the two `i64` physical bindings of a Decimal operand, a coefficient then a scale (Behavior 1 to 7). The constructor SHALL use only fixed-width native arithmetic... |
| FR-402-B1 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:42 | 1. The native Decimal type SHALL hold two signed 128-bit inclusive coefficient bounds, a minimum and a maximum membership scale (`u32`) and the owner's `RoundingMode`, and SHALL be `Copy` with no drop glue. Its checked constructor SHALL take `(lower: i128, upper: i128, min_scale: u64, max_scale: ... |
| FR-402-B2 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:43 | 2. The native Decimal type SHALL be convertible from an owner `DecimalType` by an ingress function. If a bound of the owner type lies outside signed 128 bits, then the ingress SHALL return `TypeNotRepresentable` and no native type. The owner `DecimalType` is well formed, so the ingress has no oth... |
| FR-402-B3 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:47 | 3. The carried Decimal SHALL hold the retained representation only: a signed 128-bit coefficient and a `u32` scale. It SHALL be `Copy`, SHALL have no drop glue and SHALL carry no `DecimalType`, `Integer`, `Decimal` or cached normalized form. Two carried Decimals that are numerically equal with di... |
| FR-402-B4 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:48 | 4. The constructor SHALL take a reference to the native Decimal type, the `i64` coefficient and the `i64` scale, in that order, and SHALL return the carried Decimal retaining exactly the pair drawn: no normalization, scaling or reordering. It SHALL perform no allocation, SHALL create no `Integer`... |
| FR-402-B5 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:49 | 5. The constructor SHALL admit a binding only when it is a member of the type by the owner's rule. If the scale is outside `0..=4294967295`, then the constructor SHALL return `OutsideCarrierDomain`. Otherwise let `(c', s')` be the normalized pair of the binding (trailing decimal zeros removed whi... |
| FR-402-B6 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:50 | 6. The refusal vocabulary is that of FR-398 reduced to what this carrier can produce. The causes are `TypeNotRepresentable` (at ingress, before any binding exists) and `OutsideCarrierDomain` (at construction), and no other refusal SHALL exist. FR-398's `CarrierMismatch`, `ZeroDenominator` and `No... |
| FR-402-B7 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:51 | 7. The carried Decimal SHALL provide a normalized view returning the pair `(c', s')` of Behavior 5 as a signed 128-bit coefficient and a `u32` scale, with no loop, no allocation and no `Integer`, and SHALL provide a read-out that yields the owner `Decimal` whose `representation()` equals `Decimal... |
| FR-402-B8 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:55 | 8. The kernel SHALL carry a host differential. For every boundary binding of TC-950 whose scale is in `0..=4294967295`, the oracle SHALL be the shipped owner route: `Decimal::new(Integer::from(coefficient), scale)` and `DecimalType::contains` on the owner type, with the owner's `normalized()` for... |
| FR-402-B9 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:56 | 9. The kernel SHALL carry a Kani proof, without a loop unwind bound, that over all symbolic `i64` bindings and all symbolic native type bounds (`lower <= upper` signed 128-bit, `min_scale <= max_scale` over `u32`, including values at which the lift of Behavior 5 wraps or exceeds the native width)... |
| FR-402-B10 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:57 | 10. When CG's decimal row (CG FR-051, ticket IR-259) consumes the delivered carrier, the memory and time of the proof harness that draws its operands through the carrier under the 16 GiB per-request ceiling of CG FR-028 SHALL be measured and recorded under IR-1444 with the peak provenance and the... |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified in b5c7fa1e: When the binding is admitted (Behavior 5), the constructor SHALL return the carried Decimal retaining exactly the pair drawn: no normalization, scaling or reordering. The constructor SHALL perform no allocation, SHALL create no `Integer`, `Decimal` or `Rational`, SHALL contain no loop and no iterato |
| FND-002 | fixed | Verified in b5c7fa1e: The native normalized view SHALL equal the `normalized()` of the owner `Decimal` that the read-out yields. The read-out may allocate, and its allocation is outside the constructor's zero-allocation window. |
