---
id: SR-17940486709564847047633
title: "IR-1444 PR #146 FR-402 integrity analysis"
type: SpecReview
analysis: integrity
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

Ticket: IR-1444. Spec-only PR #146 (head 1343d2aa, branch spec/ir-1444-decimal-carrier, on origin/main 9403a5c) adding FR-402 and TC-950, a Kani-tractable native Decimal carrier built from the owner's Decimal bindings. Method: quoin spec-review/integrity. Reviewed against quire-exact origin/main 9403a5c src/decimal.rs (DecimalRepresentation::normalized, Decimal::new, DecimalType::new, DecimalType::contains, compare_shifted, shifted_digits), src/decimal_membership.rs, src/native_integer.rs, src/integer.rs (From<i64>/From<i128>), Cargo.toml (no profile.dev override); merged FR-398/TC-946 and spec/reviews/ir-1443-*; quire-contract-codegen origin/main spec/kani/functional FR-051, FR-025 (AC-15), FR-028, spec/oracle/functional/FR-014; quire-specification FR-140; Linear IR-1444, IR-1508. Read-only: no cargo, no Kani, no edits to the author's worktree.

## Verdict

CHANGES REQUIRED. Cross-checked every AC literal against the TC-950 boundary set and the mutant table. AC-1 and AC-2 match the set exactly; AC-6, AC-7, AC-9, AC-10 and AC-11 map one-to-one onto steps 2, 3, 4, 5 and 7. Mismatches are an AC type that the TC never builds, normalized-view inputs with no stated literal, an AC missing a binding that a mutant names, an incomplete mutant-to-step attribution, and a refusal-closure sentence that contradicts Behavior 1.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-4 states '(1, 4294967295) and (10, 4294967295)' are OutsideCarrierDomain 'In Decimal[0,1; 0,2]', but TC-950 builds no Decimal[0, 1; 0, 2]; it places both bindings under Decimal[0, 100; 0, 2] (boundary-set line 20). The outcome is the same either way (the scale exceeds smax), but the AC literal is not what the case executes. Make the type agree in one of the two files. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:66 |
| FND-002 | medium | TC-950 adds five total-constructor normalized-view inputs, (10^37, 40), (-(10^38), 38), (i128::MIN, 0), (i128::MIN, 4294967295) and (10^38, 4294967295), and step 1 checks 'each ... normalized pair against independently written literals', but neither AC-5 nor TC-950 states their expected pairs. The only kill of the 'normalized view takes the absolute value' mutant is one of them. Add the literals to AC-5: (1, 3), (-1, 0), (i128::MIN, 0), (i128::MIN, 4294967295) and (1, 4294967257). | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:67 |
| FND-003 | low | The mutant 'An oversize positive lift is treated as inside' fails at (1, 0) in Decimal[0, 100; 60, 70], and the boundary set contains that binding, but AC-3 states only (-1, 0) and (0, 0) for that type. Add '(1, 0) is OutsideCarrierDomain by sign' so the mutant's input has a stated expectation. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:65 |
| FND-004 | low | Step 6 attributes 'retention, lift, scale-cast and ingress' mutants to step 1 and 'the membership mutants' to step 2, but the lift mutants are membership mutants and step 1 already checks every literal refusal; the 'read-out normalizes' mutant is caught only by step 2's representation() comparison and is not attributed; the normalized-view, structural-equality and type-constructor mutants are not attributed. Give each mutant row its step, or state the rule (step 1 for literals, step 2 for read-out and owner agreement). | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:35 |
| FND-005 | low | Behavior 6 says 'no other refusal SHALL exist' beyond TypeNotRepresentable and OutsideCarrierDomain, while Behavior 1 gives the native type constructor a third refusal, IllTyped(MalformedDecimalType). Scope the closure sentence to ingress and the binding constructor. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:50 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-402-AC-1 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:63 | \| FR-402-AC-1 \| The checked type constructor and the ingress agree with the owner. `NativeDecimalType::new(lo, hi, smin, smax, mode)` returns `MalformedDecimalType` for `(1, 0, 0, 2)`, `(0, 1, 3, 2)` and `(0, 1, 0, 4294967296)`, and a native type for `(i128::MIN, i128::MAX, 0, 4294967295)` and `(... |
| FR-402-AC-2 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:64 | \| FR-402-AC-2 \| The constructor retains the drawn pair and judges membership on the normalized pair. In `Decimal[0,100; 0,2]`, `(5000, 2)` is admitted and retained as `(5000, 2)` (not `(50, 0)`), `(10, 3)` is admitted and retained as `(10, 3)`, and `(1, 3)`, `(10001, 2)` and `(9999, 2)` are `Outs... |
| FR-402-AC-3 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:65 | \| FR-402-AC-3 \| The lift to `smin` and its overflow are exercised. In `Decimal[0,100; 2,3]`, `(1, 0)` is admitted (lifted coefficient 100) and `(2, 0)` is `OutsideCarrierDomain` (200). In `Decimal[0, i128::MAX; 38, 38]`, `(1, 0)` is admitted (lifted `10^38` fits) and `(2, 0)` is `OutsideCarrierDo... |
| FR-402-AC-4 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:66 | \| FR-402-AC-4 \| The scale range and the maximum scale are exact. In `Decimal[-1000,1000; 0,2]`, `(1, 4294967296)` is `OutsideCarrierDomain`. In `Decimal[0,1; 0,4294967295]`, `(1, -1)` is `OutsideCarrierDomain` (cast to `u32` it would be `(1, 4294967295)`, a member), `(1, 4294967295)` is admitted ... |
| FR-402-AC-5 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:67 | \| FR-402-AC-5 \| Canonical and trailing-zero forms are kept apart. In `Decimal[0,100; 0,2]`, `(100, 2)` and `(1, 0)` are both admitted, retained as different values with equal normalized view `(1, 0)`, and `(1, 0)` is not equal to `(100, 2)` under structural equality. The normalized view is `(0, 0... |
| FR-402-AC-6 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:68 | \| FR-402-AC-6 \| For every admitted boundary binding, the read-out `Decimal` has `representation()` equal to the owner's `DecimalRepresentation::new(Integer::from(coefficient), scale)`, including `i64::MIN`, and its `normalized()` equals the native normalized view. \| Test (TC-950) \| |
| FR-402-AC-7 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:69 | \| FR-402-AC-7 \| The constructor and the normalized view are built for the host and for `cfg(kani)` from one body, with no `cfg(kani)` token in them, no `for`, `while`, `loop` or iterator adapter, no recursion and no `Meter` in their signatures (a function-pointer coercion to a `Meter`-free signat... |
| FR-402-AC-8 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:70 | \| FR-402-AC-8 \| Over the boundary set of TC-950, the constructor and the owner route of Behavior 8 agree on admission and on the retained and normalized pairs, with the one intended difference of Behavior 8, which the case lists exhaustively and checks against its named refusal. Bindings with a s... |
| FR-402-AC-9 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:71 | \| FR-402-AC-9 \| The type system offers no way to build the carried Decimal from an `Integer`, a `Decimal`, a `DecimalType` or an unchecked pair of public fields: each such program fails to compile with its pinned error code (`E0308` for a wrong argument type, `E0451` for a private field), with a ... |
| FR-402-AC-10 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:72 | \| FR-402-AC-10 \| A Kani proof with no unwind bound verifies the constructor and a second proof verifies the normalized view, over the symbolic domains of Behavior 9, against their independent references, with the covers of Behavior 9 satisfied. A proof over one concrete type, one that does not co... |
| FR-402-AC-11 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:73 | \| FR-402-AC-11 \| When CG's decimal row (CG FR-051, IR-259) consumes the delivered carrier, the peak memory, time, tool version and terminal outcome of the proof harness that draws its operands through the carrier are recorded under IR-1444 with the harness run and the peak provenance. A fit or a ... |
| FR-402-B1 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:42 | 1. The native Decimal type SHALL hold two signed 128-bit inclusive coefficient bounds, a minimum and a maximum membership scale (`u32`) and the owner's `RoundingMode`, and SHALL be `Copy` with no drop glue. Its checked constructor SHALL take `(lower: i128, upper: i128, min_scale: u64, max_scale: ... |
| FR-402-B6 | examined | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:50 | 6. The refusal vocabulary is that of FR-398 reduced to what this carrier can produce. The causes are `TypeNotRepresentable` (at ingress, before any binding exists) and `OutsideCarrierDomain` (at construction), and no other refusal SHALL exist. FR-398's `CarrierMismatch`, `ZeroDenominator` and `No... |
| TC-950-SET | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:18 | The boundary set, used by steps 1 to 4, is fixed before the constructor is written and is derived from independent literals, not from the constructor. A binding is `(coefficient, scale)`, both `i64`. Types are written `Decimal[lo, hi; smin, smax]`; the rounding spelling is `Exact` throughout and ... |
| TC-950-S1 | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:30 | 1. For AC-1 to AC-5, build every type and construct every boundary binding with the public API. Check each retained and normalized pair against independently written literals and each refusal against its named cause. Check that `(100, 2)` and `(1, 0)` in `Decimal[0, 100; 0, 2]` are unequal under ... |
| TC-950-S6 | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:35 | 6. Run each mutant of the next section: the input mutants through the checks of steps 1 and 2 (step 1 catches the retention, lift, scale-cast and ingress mutants; step 2 catches the membership mutants), the inspection mutants through step 3. |
| TC-950-MUT | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:40 | \| Mutant \| Fails at \| |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified in b5c7fa1e: In `Decimal[0,100; 0,2]`, `(1, 4294967295)` and `(10, 4294967295)` are `OutsideCarrierDomain`. \| Test (TC-950) \| |
| FND-002 | fixed | Verified in b5c7fa1e: For the total constructor: `(1, 3)` for `(10^37, 40)`, `(-1, 0)` for `(-(10^38), 38)`, `(i128::MIN, 0)` for `(i128::MIN, 0)`, `(i128::MIN, 4294967295)` for `(i128::MIN, 4294967295)` (`2^127` has no factor of five) and `(1, 4294967257)` for `(10^38, 4294967295)` (38 zeros removed). \| Test (TC-950) \| |
| FND-003 | fixed | Verified in b5c7fa1e: and in `Decimal[0,100; 60,70]`, `(1, 0)` and `(-1, 0)` are `OutsideCarrierDomain` and `(0, 0)` is admitted. On the negative side of the width, in `Decimal[i128::MIN, 0; 38, 38]`, `(-1, 0)` is admitted (lifted `-10^38` fits) and `(-2, 0)` is `OutsideCarrierDomain` (`-2 x 10^38` is below `i128::MIN`). |
| FND-004 | fixed | Verified in b5c7fa1e: 6. Run each mutant of the next section: the input mutants through the checks of steps 1 and 2 by this rule: a mutant whose failing input has a literal expectation in the boundary set (retention, membership, lift, scale-cast, normalized-view, structural-equality, type-constructor and ingress mutants) |
| FND-005 | fixed | Verified in b5c7fa1e: and the ingress and the constructor SHALL return no other refusal (the checked type constructor's `MalformedDecimalType` of Behavior 1 is the owner's `IllTyped`, not a carrier refusal). FR-398's `CarrierMismatch`, `ZeroDenominator` and `NonCanonical` have no counterpart because there is one carrier  |
