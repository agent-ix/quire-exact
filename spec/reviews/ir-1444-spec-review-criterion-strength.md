---
id: SR-17940486709564847047635
title: "IR-1444 PR #146 FR-402 criterion-strength analysis (manual)"
type: SpecReview
analysis: criterion-strength
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

Ticket: IR-1444. Spec-only PR #146 (head 1343d2aa, branch spec/ir-1444-decimal-carrier, on origin/main 9403a5c) adding FR-402 and TC-950, a Kani-tractable native Decimal carrier built from the owner's Decimal bindings. Method: quoin spec-review/criterion-strength. Reviewed against quire-exact origin/main 9403a5c src/decimal.rs (DecimalRepresentation::normalized, Decimal::new, DecimalType::new, DecimalType::contains, compare_shifted, shifted_digits), src/decimal_membership.rs, src/native_integer.rs, src/integer.rs (From<i64>/From<i128>), Cargo.toml (no profile.dev override); merged FR-398/TC-946 and spec/reviews/ir-1443-*; quire-contract-codegen origin/main spec/kani/functional FR-051, FR-025 (AC-15), FR-028, spec/oracle/functional/FR-014; quire-specification FR-140; Linear IR-1444, IR-1508. Read-only: no cargo, no Kani, no edits to the author's worktree.

## Verdict

CHANGES REQUIRED (low only in this file). Jev (typesafe.ai System One) is not available here, so the 29 TC-950 mutant rows were checked by hand against owner semantics. Distinguishing on their named input: retained-normalized (5000, 2); raw coefficient (5000, 2); raw scale (0, 7); '< smax' (10, 3); lift skipped (2, 0) in [0,100;2,3]; wrapping lift (10^128 = 0 mod 2^128); saturating lift (2*10^38 -> i128::MAX); >10^37 oversize (1, 0) at 38; oversize positive/negative inside; both off-by-one rows; scale `as` cast ((1, 2^32) -> 0, (1, -1) -> u32::MAX); `as u64 as i128` (-1000); structural equality; read-out normalizes (step 2); the three normalized-view rows; the four ingress rows; the type constructor row; heap field (Copy bound plus allocation counter); loop/cfg(kani) by inspection; Meter by fn-pointer coercion. The compile_fail controls pin E0308/E0451 with positive twins. AC-10 and AC-11 refuse credit for non-completion, missing covers and predictions.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The value-comparison mutant (compare the decimal value, not the lifted normalized coefficient, with [lo, hi]) that TC-946 lists is missing, though its distinguishing input (9999, 2) in Decimal[0, 100; 0, 2] is already in the set (value 99.99 lies in [0, 100]; the owner refuses). Add the row. | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:40 |
| FND-002 | low | The 'normalized view takes the absolute value' row is killed only by an overflow panic of i128::MIN.abs(). With overflow checks on (cargo test's dev profile; Cargo.toml has no override) it panics, but with them off abs wraps to i128::MIN and the result equals the expected (i128::MIN, 4294967295), so the mutant survives. State that the kill is the overflow panic under the test profile, or add a sign case that does not rely on overflow. | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:61 |
| FND-003 | low | Step 3's source inspection 'for Integer, Decimal, Rational, Vec and Box' cannot fail cleanly: the carrier's own names (NativeDecimal, NativeDecimalType, MalformedDecimalType) contain 'Decimal', so a token search always hits. Name the owner types by path or in type position. AC-7 and step 3 also inspect the type constructor and the total constructor and forbid recursion, which Behaviors 4 and 7 do not require. Align the two. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:69 |

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
| TC-950-S3 | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:32 | 3. For AC-7, inspect the source of the constructor, the type constructor, the normalized view and the total constructor for a `cfg(kani)` token, for `for`, `while`, `loop`, recursion and iterator adapters, and for `Integer`, `Decimal`, `Rational`, `Vec` and `Box`; build the crate for the host wit... |
| TC-950-S4 | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:33 | 4. For AC-9, compile `compile_fail,E0308` programs that pass an `Integer`, a `Decimal` and a `DecimalType` to the constructor in place of the `i64` bindings and the native type, and a `compile_fail,E0451` program that builds the carried Decimal by struct literal; each has a compiling positive twi... |
| TC-950-S5 | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:34 | 5. For AC-10, run the two Kani proofs `native_decimal_from_bindings_refines_owner_membership` and `native_decimal_normalized_is_least_scale_pair` with no unwind attribute and their covers, under the repository's authorized runner, and run the host test that binds each independent reference to the... |
| TC-950-S6 | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:35 | 6. Run each mutant of the next section: the input mutants through the checks of steps 1 and 2 (step 1 catches the retention, lift, scale-cast and ingress mutants; step 2 catches the membership mutants), the inspection mutants through step 3. |
| TC-950-MUT | examined | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:40 | \| Mutant \| Fails at \| |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | Regression from the FND-003 alignment: Behavior 7 requires the normalized view to use no allocation and no Integer, but AC-7 and step 3 now scan only the constructor for owner types, and the allocation window covers only 'the construction of the whole boundary set'. An Integer-based normalized view passes every check (its Kani proof would only fail to complete, which is inconclusive). Add the normalized view to the owner-type scan and call it inside the allocation window. | spec/functional/FR-402-kani-tractable-native-decimal-carrier.md:69 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified in b5c7fa1e: \| The decimal value rather than the lifted normalized coefficient is compared with `[lo, hi]` \| `(9999, 2)` in `Decimal[0, 100; 0, 2]`, expected `OutsideCarrierDomain` \| |
| FND-002 | fixed | Verified in b5c7fa1e: \| The normalized view strips zeros from the absolute value and returns it unsigned \| `(-1000, 3)`, expected `(-1, 0)` (fails by value, independent of overflow checks) \| |
| FND-003 | fixed | Verified in b5c7fa1e: and the source of the constructor for the owner types `Integer`, `Decimal` and `Rational` (matched as the owner paths `crate::integer::Integer`, `crate::decimal::Decimal` and `crate::rational::Rational`, or as a whole-word identifier, never as a substring of the carrier's own `NativeDecimal` names); |
| FND-004 | fixed | Verified in 7672924c (round 2): AC-7 and step 3 scan the constructor and the normalized view for owner types and run the normalized view of every normalized-view input in the allocation window; new mutant row kills an Integer-based view by the scan and an allocating view by the window over inputs with 18 and 38 trailing zeros. |
