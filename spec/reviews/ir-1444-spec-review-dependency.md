---
id: SR-17940486709564847047637
title: "IR-1444 PR #146 FR-402 dependency analysis"
type: SpecReview
analysis: dependency
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

Ticket: IR-1444. Spec-only PR #146 (head 1343d2aa, branch spec/ir-1444-decimal-carrier, on origin/main 9403a5c) adding FR-402 and TC-950, a Kani-tractable native Decimal carrier built from the owner's Decimal bindings. Method: quoin spec-review/dependency. Reviewed against quire-exact origin/main 9403a5c src/decimal.rs (DecimalRepresentation::normalized, Decimal::new, DecimalType::new, DecimalType::contains, compare_shifted, shifted_digits), src/decimal_membership.rs, src/native_integer.rs, src/integer.rs (From<i64>/From<i128>), Cargo.toml (no profile.dev override); merged FR-398/TC-946 and spec/reviews/ir-1443-*; quire-contract-codegen origin/main spec/kani/functional FR-051, FR-025 (AC-15), FR-028, spec/oracle/functional/FR-014; quire-specification FR-140; Linear IR-1444, IR-1508. Read-only: no cargo, no Kani, no edits to the author's worktree.

## Verdict

CHANGES REQUIRED. Edges checked: depends_on FR-372 (owner Decimal membership; the differential actually uses DecimalType::contains of FR-140, as FR-398 does, acceptable); references FR-384 (native width and host-availability precedent; the total constructor mirrors NativeInteger::new), FR-398 (refusal names and check order reused; FR-398 does not depend on FR-402, correct), QSpec FR-140, and CG FR-051, FR-025 (AC-15 exists, PLANNED IR-1440) and FR-028 (16 GiB ceiling). CG consumption is IR-259, named in CG FR-051 and TC-065; it is correctly kept out of this requirement's acceptance. IR-1445 and IR-1446 are correctly non-prerequisites. The dropped FR-398 causes are justified: one carrier (CarrierMismatch), no Rational (ZeroDenominator, NonCanonical), no fraction formed (NotRepresentable). Two consumer-contract gaps remain.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The out-of-scope paragraph defers 'Operations over the carrier ... and CG FR-051 cannot be credited with a native expression until they exist' with no owner. CG FR-051 defines P1 (IR-1444) as 'a symbolic exact decimal operand and a native expression over it', so FR-402 delivers half of P1 and the other half is the follow-up Linear IR-1508 (native Decimal operations over the carrier, split from IR-1444). Name IR-1508 in the out-of-scope paragraph and in Dependencies, as the owner of the native expression CG FR-051 needs. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:32 |
| FND-002 | medium | CG FR-051 draws each operand 'within a finite coefficient envelope that the carrier (P1) states' and refuses 'a scale endpoint beyond the carrier' (FR-051 Behavior; AC-2, AC-5). FR-402 never states that envelope; it is only implicit in the i64 coefficient binding and the 0..=4294967295 scale range. State it as a carrier fact (coefficient in i64, scale in 0..=u32::MAX, retained coefficient i128) so CG can cite it. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:26 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-402 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:26 | The kernel SHALL provide a native Decimal carrier, a native Decimal type and a constructor that builds a carried value of that type from the two `i64` physical bindings of a Decimal operand, a coefficient then a scale (Behavior 1 to 7). The constructor SHALL use only fixed-width native arithmetic... |
| FR-402-OOS | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:32 | Four things are out of scope. Operations over the carrier (arithmetic, rounding and ordering, with their charges) are not specified here, and CG FR-051 cannot be credited with a native expression until they exist. Decoding retained counterexample playback into bindings is CG's (CG FR-051, FR-025-... |
| FR-402-DEPS | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:81 | - FR-372 and QSpec FR-140 own Decimal membership, on which the constructor depends. FR-384 is the native-width and host-availability precedent. FR-398 (IR-1443) is the sibling constructor and the precedent for the boundary vocabulary, the refusal names and the differential. |
| FR-402-B2 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:43 | 2. The native Decimal type SHALL be convertible from an owner `DecimalType` by an ingress function. If a bound of the owner type lies outside signed 128 bits, then the ingress SHALL return `TypeNotRepresentable` and no native type. The owner `DecimalType` is well formed, so the ingress has no oth... |
| FR-402-B3 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:47 | 3. The carried Decimal SHALL hold the retained representation only: a signed 128-bit coefficient and a `u32` scale. It SHALL be `Copy`, SHALL have no drop glue and SHALL carry no `DecimalType`, `Integer`, `Decimal` or cached normalized form. Two carried Decimals that are numerically equal with di... |
| FR-402-B6 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:50 | 6. The refusal vocabulary is that of FR-398 reduced to what this carrier can produce. The causes are `TypeNotRepresentable` (at ingress, before any binding exists) and `OutsideCarrierDomain` (at construction), and no other refusal SHALL exist. FR-398's `CarrierMismatch`, `ZeroDenominator` and `No... |
| FR-402-B10 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:57 | 10. When CG's decimal row (CG FR-051, ticket IR-259) consumes the delivered carrier, the memory and time of the proof harness that draws its operands through the carrier under the 16 GiB per-request ceiling of CG FR-028 SHALL be measured and recorded under IR-1444 with the peak provenance and the... |
| FR-402-AC-11 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:73 | \| FR-402-AC-11 \| When CG's decimal row (CG FR-051, IR-259) consumes the delivered carrier, the peak memory, time, tool version and terminal outcome of the proof harness that draws its operands through the carrier are recorded under IR-1444 with the harness run and the peak provenance. A fit or a ... |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified in b5c7fa1e: Operations over the carrier (arithmetic, rounding and ordering, with their charges) are not specified here, and are owned by Linear IR-1508 (native Decimal operations over the carrier), which CG FR-051's prerequisite P1 also needs: CG FR-051 cannot be credited with a native expression until IR-1508  |
| FND-002 | fixed | Verified in b5c7fa1e: The carrier's envelope, which CG FR-051 asks its prerequisite P1 to state, SHALL be: the coefficient binding is an `i64`, the scale binding is accepted over `0..=4294967295`, the retained coefficient is an `i128` and the retained scale is a `u32`. A total constructor from any signed 128-bit coeffici |
