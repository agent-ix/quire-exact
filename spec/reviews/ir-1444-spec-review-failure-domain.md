---
id: SR-17940486709564847047634
title: "IR-1444 PR #146 FR-402 failure-domain analysis"
type: SpecReview
analysis: failure-domain
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

Ticket: IR-1444. Spec-only PR #146 (head 1343d2aa, branch spec/ir-1444-decimal-carrier, on origin/main 9403a5c) adding FR-402 and TC-950, a Kani-tractable native Decimal carrier built from the owner's Decimal bindings. Method: quoin spec-review/failure-domain. Reviewed against quire-exact origin/main 9403a5c src/decimal.rs (DecimalRepresentation::normalized, Decimal::new, DecimalType::new, DecimalType::contains, compare_shifted, shifted_digits), src/decimal_membership.rs, src/native_integer.rs, src/integer.rs (From<i64>/From<i128>), Cargo.toml (no profile.dev override); merged FR-398/TC-946 and spec/reviews/ir-1443-*; quire-contract-codegen origin/main spec/kani/functional FR-051, FR-025 (AC-15), FR-028, spec/oracle/functional/FR-014; quire-specification FR-140; Linear IR-1444, IR-1508. Read-only: no cargo, no Kani, no edits to the author's worktree.

## Verdict

CHANGES REQUIRED. Boundaries present: scale 0, scale u32::MAX (member and non-member), scale -1 and 2^32 (cast traps), the positive 10^38/10^39 lift edge, a wrap-to-zero lift (10^128), far-oversize lifts of both signs, zero under a huge lift, i64::MIN/i64::MAX coefficients, i128::MIN/MAX type bounds at the type constructor and ingress, and i128 bounds just outside (2^127, -2^127-1, 2^128+1). Check order (ingress, scale range, membership) matches FR-398 and IR-1444; ingress precedes any binding structurally, and scale-before-membership is observable through the cast mutant. Gaps: the negative-side lift overflow edge, a differential difference with no bindings to compare, and proof covers that miss the lift branches.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The lift-overflow edge is tested only on the positive side (Decimal[0, i128::MAX; 38, 38]: (1, 0) in, (2, 0) out). The negative side has only far-oversize lifts (10^60). A body that is correct for positive lifts but saturates negative ones (-2*10^38 saturates to i128::MIN, which is >= lower) or forms the magnitude in u128 and negates with a wrapping cast passes every listed input. Add Decimal[i128::MIN, 0; 38, 38] with (-1, 0) admitted and (-2, 0) OutsideCarrierDomain, and a 'negative lift saturates' mutant row. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:65 |
| FND-002 | medium | The one intended difference is that each of the three non-representable owner types 'exists and admits bindings', but the boundary set gives those types no bindings (they appear only under 'Type-construction and ingress inputs'), so step 2 has nothing per binding to compare for them and 'the complete list of disagreements' is never computed for them. FR-398's TC-946 used bindings against such types. Either list bindings (for example (1, 0), which the owner admits in each) and expect TypeNotRepresentable at ingress, or restate the difference as a type-level comparison (owner DecimalType::new is Ok, native ingress is Err). | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:70 |
| FND-003 | low | The constructor proof's covers are member, out-of-scale-range and non-member in-range. None requires the oversize (sign-decided) lift branch or an in-range nonzero lift, which are the branches Behavior 5 singles out. A proof whose symbolic bounds never reach them still satisfies all three covers. Add a cover for each. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:56 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-402-B2 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:43 | 2. The native Decimal type SHALL be convertible from an owner `DecimalType` by an ingress function. If a bound of the owner type lies outside signed 128 bits, then the ingress SHALL return `TypeNotRepresentable` and no native type. The owner `DecimalType` is well formed, so the ingress has no oth... |
| FR-402-B4 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:48 | 4. The constructor SHALL take a reference to the native Decimal type, the `i64` coefficient and the `i64` scale, in that order, and SHALL return the carried Decimal retaining exactly the pair drawn: no normalization, scaling or reordering. It SHALL perform no allocation, SHALL create no `Integer`... |
| FR-402-B5 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:49 | 5. The constructor SHALL admit a binding only when it is a member of the type by the owner's rule. If the scale is outside `0..=4294967295`, then the constructor SHALL return `OutsideCarrierDomain`. Otherwise let `(c', s')` be the normalized pair of the binding (trailing decimal zeros removed whi... |
| FR-402-B6 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:50 | 6. The refusal vocabulary is that of FR-398 reduced to what this carrier can produce. The causes are `TypeNotRepresentable` (at ingress, before any binding exists) and `OutsideCarrierDomain` (at construction), and no other refusal SHALL exist. FR-398's `CarrierMismatch`, `ZeroDenominator` and `No... |
| FR-402-B8 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:55 | 8. The kernel SHALL carry a host differential. For every boundary binding of TC-950 whose scale is in `0..=4294967295`, the oracle SHALL be the shipped owner route: `Decimal::new(Integer::from(coefficient), scale)` and `DecimalType::contains` on the owner type, with the owner's `normalized()` for... |
| FR-402-B9 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:56 | 9. The kernel SHALL carry a Kani proof, without a loop unwind bound, that over all symbolic `i64` bindings and all symbolic native type bounds (`lower <= upper` signed 128-bit, `min_scale <= max_scale` over `u32`, including values at which the lift of Behavior 5 wraps or exceeds the native width)... |
| FR-402-AC-3 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:65 | \| FR-402-AC-3 \| The lift to `smin` and its overflow are exercised. In `Decimal[0,100; 2,3]`, `(1, 0)` is admitted (lifted coefficient 100) and `(2, 0)` is `OutsideCarrierDomain` (200). In `Decimal[0, i128::MAX; 38, 38]`, `(1, 0)` is admitted (lifted `10^38` fits) and `(2, 0)` is `OutsideCarrierDo... |
| FR-402-AC-4 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:66 | \| FR-402-AC-4 \| The scale range and the maximum scale are exact. In `Decimal[-1000,1000; 0,2]`, `(1, 4294967296)` is `OutsideCarrierDomain`. In `Decimal[0,1; 0,4294967295]`, `(1, -1)` is `OutsideCarrierDomain` (cast to `u32` it would be `(1, 4294967295)`, a member), `(1, 4294967295)` is admitted ... |
| FR-402-AC-8 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:70 | \| FR-402-AC-8 \| Over the boundary set of TC-950, the constructor and the owner route of Behavior 8 agree on admission and on the retained and normalized pairs, with the one intended difference of Behavior 8, which the case lists exhaustively and checks against its named refusal. Bindings with a s... |
| FR-402-AC-10 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:72 | \| FR-402-AC-10 \| A Kani proof with no unwind bound verifies the constructor and a second proof verifies the normalized view, over the symbolic domains of Behavior 9, against their independent references, with the covers of Behavior 9 satisfied. A proof over one concrete type, one that does not co... |
| TC-950-SET | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:18 | The boundary set, used by steps 1 to 4, is fixed before the constructor is written and is derived from independent literals, not from the constructor. A binding is `(coefficient, scale)`, both `i64`. Types are written `Decimal[lo, hi; smin, smax]`; the rounding spelling is `Exact` throughout and ... |
| TC-950-S2 | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:31 | 2. For AC-6 and AC-8, build every boundary binding with scale in `0..=4294967295` a second time by the owner route: `Decimal::new(Integer::from(coefficient), scale)` and `DecimalType::contains` on an owner `DecimalType` built from the same bounds, and the owner's `normalized()`. Compare admission... |
| TC-950-MUT | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:40 | \| Mutant \| Fails at \| |
