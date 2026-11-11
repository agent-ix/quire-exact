---
id: SR-17940486709564847047615
title: "IR-1443 PR #144 FR-398 criterion-strength analysis (manual)"
type: SpecReview
analysis: criterion-strength
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #144 @ 5183708 on origin/main 4fe7df6; IR-1443; spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md, spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-398
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-946
    type: reviews
---

## Summary

Ticket: IR-1443. Spec-only PR #144 (head 5183708, one commit on origin/main 4fe7df6) adding FR-398 and TC-946, a Kani-tractable carried-quantity constructor from the carrier's physical bindings. Reviewed against quire-exact src/quantity.rs, src/decimal.rs, src/rational.rs, src/integer.rs, src/identity.rs, src/node.rs and src/native_integer.rs at the PR head; CG origin/main 42db5c62 spec/kani/functional/FR-050 and FR-025 (AC-18 and the Decimal, Rational and carried-leaf rows); QSV origin/main src/quantity.rs (convert_quantity); Linear IR-1443, IR-1497 (decision A2), IR-1436 (D4, D5 and the correction), IR-1444, IR-1445, IR-1446. Read-only: no edit, build or cargo command ran.

## Verdict

CHANGES REQUIRED. Jev (typesafe.ai System One) is not available here, so each of the 18 TC-946 mutants was checked by hand against its named input and against quire-exact 4fe7df6 owner semantics. Distinguishing as claimed: scale ignored, unreduced, scale <= 38, scale <= 37 / <= 39, raw coefficient (5000, 2), value comparison (9999, 2), sign drop, Rational swap (1, 2) -> 2/1, reduce-instead-of-refuse, negative denominator, zero-denominator reason, numerator-only check (caught by (1, 5)), Int off-by-one, i64::MIN abs, and the heap-magnitude mutant (Copy bound fails to compile; a counting allocator in an integration-test binary is buildable). The AC-8 compile_fail controls are buildable. Defects: one mutant can never fail, one is caught only by an outcome the TC calls inconclusive, one names no input, and some controls cannot fail.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The mutant 'a body selected by cfg(kani) that differs from the host body' is said to fail 'the host differential on (10, 1)', but a host build never compiles the cfg(kani) body, so the host differential runs the unmutated body and passes. This control can never fail as stated. Name a check that sees the cfg(kani) body: the Kani proof against the independent reference on that input, or a source inspection that fails on any cfg(kani) item in the constructor module (AC-6's inspection). | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:56 |
| FND-002 | medium | The loop mutant's kill is 'the Kani proof with no unwind bound fails to complete', but TC-946's Expected Results class a timeout or memory exhaustion as inconclusive, never a pass or fail, and a fixed-bound loop (for k in 0..39, an iterator loop IR-1436 D4 bans) completes and is not caught. Use a control that fails decisively, for example the proof under #[kani::unwind(1)] with unwinding assertions, which fails on any loop that iterates, or a source lint. | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:54 |
| FND-003 | medium | The order mutant's failing input is 'the NonCanonical-and-outside pair of AC-5', but neither AC-5 nor the boundary set names that pair, and whether 'outside' is judged on the raw or reduced pair changes the answer ((2, 4) in Rational[1,5;2,4] reduces to 1/2, which is inside). Name it, for example (2, 10) in Rational[1,5;2,4] (outside raw and reduced), and add a mutant row for the second rule (NotRepresentable before OutsideCarrierDomain), whose input (1, 39) in Decimal[0,1;0,2] already exists. | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:53 |
| FND-004 | medium | AC-9 quantifies over 'all symbolic bindings' but not over the carrier type: a proof per carrier over one concrete CarriedQuantityType proves the bounds logic for that type only (smin lift, bound edges, big bounds untouched). State which types each proof fixes or draws (for example symbolic native lo, hi, smin, smax under assumptions) so 'for all' has a stated domain. | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:53 |
| FND-005 | low | A compile_fail control passes on any compile error, including a typo, so as stated it distinguishes nothing. The repository already pins codes (src/integer.rs:622 `compile_fail,E0308`); require the expected error code for each of the three programs and a compiling positive twin. 'No way' is also broader than three spellings; say the three spellings are the checked ones. | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:52 |
| FND-006 | low | 'leaves a Meter the caller holds unchanged' (and Behavior 4's 'a refusal consumes nothing') cannot fail when the constructor has no Meter parameter; the static no-Meter-parameter bound is the real control. Drop the runtime Meter check or say it is a corollary. | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:50 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-398 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:22 | The kernel SHALL provide a carried-quantity constructor that builds an owner carried quantity of a `CarriedQuantityType` from the physical bindings of that type's carrier, using only fixed-width native arithmetic, so that a generated Kani proof can draw the bindings symbolically and reach the quanti |
| FR-398-B1 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:32 | 1. **Bindings.** The constructor takes a `CarriedQuantityType` and the carrier's physical bindings in the order CG FR-025-AC-18 defines: an `Int` carrier one `i64`; a `Decimal` carrier an `i64` coefficient then a `u32` scale; a `Rational` carrier an `i64` numerator then an `i64` denominator. It perf |
| FR-398-B2 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:33 | 2. **Exact magnitude.** An `Int` binding `n` gives `n/1`. A `Decimal` binding `(c, s)` gives the exact value `c / 10^s`. A `Rational` binding `(n, d)` gives `n/d`. The magnitude is held as a canonical fraction (lowest terms, positive denominator, zero as `0/1`) of two native signed 128-bit integers, |
| FR-398-B3 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:34 | 3. **Admission.** The constructor admits a binding when it is a member of the type's carrier by the owner's own membership: an `Int` binding by `IntegerInterval::contains`; a `Decimal` binding by `DecimalType::contains` of the pair as the owner defines it, which first removes trailing decimal zeros, |
| FR-398-B4 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:35 | 4. **Refusals.** The constructor returns a typed refusal and no quantity for each of these, and for no other reason: `CarrierMismatch` when the bindings are not of the type's carrier; `ZeroDenominator` for a `Rational` denominator of zero; `NonCanonical` for a `Rational` pair that is not in lowest t |
| FR-398-B5 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:36 | 5. **Host differential.** For every carrier, each binding in the boundary set of the test case is built twice, once by the constructor and once by the shipped owner route (`Integer`, `Decimal::to_rational` or `Rational::new`, then `Quantity::new`, with `DecimalType::contains`, `IntegerInterval::cont |
| FR-398-B6 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:37 | 6. **No other source.** The constructor accepts no `BoundedQuantityType` (FR-387), no bare unit and no caller-supplied `Rational`, since none forms a finite binding domain. |
| FR-398-B7 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:38 | 7. **Proof.** For each carrier the kernel carries a Kani proof, without a loop unwind bound, that over all symbolic bindings the constructor's outcome and magnitude equal an independent bounded mathematical reference that shares no code with the constructor, and a cover that the success path is reac |
| FR-398-B8 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:39 | 8. **Fit.** The memory and time of the composed proof (leaf construction, owner conversion and owner operation in one harness) under the 16 GiB per-request ceiling of CG FR-028 are measured in the code slice of IR-1443 by running CG's FR-050 harness against the delivered constructor, and recorded wi |
| FR-398-Dependencies | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:64 | - The sibling Kani carriers IR-1445 (text) and IR-1446 follow the same host-differential pattern and are not prerequisites. |
| FR-398-AC-1 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:45 | For an `Int[lo, hi]` carrier, bindings `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1` and `i64::MAX`, each inside a carrier that contains it, yield magnitude `n/1` in the type's unit, equal to the owner route; the endpoints `lo` and `hi` are admitted and `lo - 1` and `hi + 1` are `Outsid |
| FR-398-AC-2 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:46 | For a `Decimal[lo, hi; smin, smax; mode]` carrier, `(c, s)` yields the exact canonical value `c / 10^s`: `(10, 1)` is `1/1`, `(-5, 1)` is `-1/2`, `(0, 7)` is `0/1`, and `(5000, 2)` in `Decimal[0,100; 0,2]` is admitted as `50/1` although its raw coefficient is outside `[0, 100]`. `(10001, 2)` and `(9 |
| FR-398-AC-3 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:47 | For a `Rational[n1, n2; d1, d2]` carrier, a canonical `(n, d)` inside both ranges yields `n/d` in the type's unit with numerator and denominator in that order (`(1, 2)` is `1/2`, not `2/1`); `(2, 4)`, `(1, -2)`, `(-1, -2)` and `(0, 2)` are `NonCanonical`, `(1, 0)` is `ZeroDenominator`, and a canonic |
| FR-398-AC-4 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:48 | A `Decimal` binding whose canonical numerator or denominator exceeds the native 128-bit width is `NotRepresentable`: `(1, 38)` is admitted as `1/10^38`, `(1, 39)` and `(1, u32::MAX)` are `NotRepresentable`, and `(2^39, 39)`, whose canonical form is `1/5^39`, is admitted. The decision is by the canon |
| FR-398-AC-5 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:49 | A binding of one carrier presented with a type of another (an `i64` pair against an `Int` type, a single `i64` against a `Decimal` or `Rational` type) is `CarrierMismatch`. For a binding that is both `NonCanonical` and outside the domain, `NonCanonical` is reported; for one that is both outside the  |
| FR-398-AC-6 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:50 | The constructor builds in an ordinary host build and in a `cfg(kani)` build from the same source, with no `cfg(kani)`-selected body. Its result is `Copy`, its construction performs zero heap allocations as observed by a counting allocator around the call, and it has no `Meter` parameter and leaves a |
| FR-398-AC-7 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:51 | Over the boundary set of TC-946, the constructor and the shipped owner route agree on admission, magnitude, unit and the refusal reasons of Behavior 5, and the read-out `Quantity` equals the owner's under `Eq`; the sole disagreement is the `NonCanonical` pairs that the owner normalizes. |
| FR-398-AC-8 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:52 | The type system offers no way to build the carrier from a `BoundedQuantityType`, a bare `UnitId` or a `Rational`; a program that tries fails to compile. |
| FR-398-AC-9 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:53 | For each of `Int`, `Decimal` and `Rational`, a Kani proof with no unwind bound verifies that, for all symbolic bindings, the constructor's outcome and magnitude equal the independent reference, with a success-reachable cover. A proof that does not complete, exhausts memory or lacks the cover is not  |
| FR-398-AC-10 | examined | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:54 | The peak memory, time, tool version and terminal outcome of the composed convert-and-operation proof with the delivered constructor are recorded under IR-1443, naming the harness run and the peak provenance. A fit or a non-fit at 16 GiB is stated as measured; a non-fit is followed by a filed QSpec f |
| TC-946-boundary-int | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:20 | - `Int[-5, 100]`: `i64::MIN`, `i64::MIN + 1`, `-1`, `0`, `1`, `i64::MAX - 1`, `i64::MAX` against carriers `Int[i64::MIN, i64::MAX]` (all admitted) and `Int[-5, 100]` for `-6`, `-5`, `100`, `101` (the first and last refused, the middle two admitted). |
| TC-946-boundary-decimal | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:21 | - `Decimal[-1000, 1000; 0, 2; Exact]`: `(10, 1)`, `(-5, 1)`, `(0, 7)`, `(1000, 0)`, `(100000, 2)`, `(100001, 2)`, `(i64::MIN, 0)`, `(i64::MAX, 0)`. `Decimal[0, 100; 0, 2; Exact]`: `(5000, 2)`, `(10001, 2)`, `(9999, 2)`, `(1, 3)`. `Decimal[0, 549755813888; 0, 4294967295; Exact]`: `(1, 37)`, `(1, 38)` |
| TC-946-boundary-rational | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:22 | - `Rational[-9223372036854775808, 9223372036854775807; 1, 9223372036854775807]`: `(0, 1)`, `(1, 1)`, `(-1, 1)`, `(1, 2)`, `(2, 1)`, `(i64::MIN, 1)`, `(1, i64::MAX)`, `(i64::MIN, i64::MAX)`, and the non-canonical or zero pairs `(2, 4)`, `(1, -2)`, `(-1, -2)`, `(0, 2)`, `(1, 0)`, `(0, 0)`. `Rational[1 |
| TC-946-step-1 | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:25 | 1. For AC-1, AC-2, AC-3 and AC-4, construct every boundary binding with the public constructor. Check the magnitude against independently written decimal literals as a numerator and denominator pair, the unit against the type's `UnitId`, and each refusal against its named cause. Include the order co |
| TC-946-step-2 | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:26 | 2. For AC-7, build every admitted boundary binding a second time by the shipped owner route (`Integer::from`, `Decimal::new` then `to_rational`, or `Rational::new`, then `Quantity::new`) and check admission with `IntegerInterval::contains`, `DecimalType::contains` and `RationalDomain::contains`. Ass |
| TC-946-step-3 | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:27 | 3. For AC-6, build the crate for the host with default features and compile `Copy` and a no-`Meter`-parameter use as a static bound. Run the construction of the whole boundary set inside a counting global allocator window and assert zero allocations and unchanged counters of a `Meter` created before |
| TC-946-step-4 | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:28 | 4. For AC-8, compile `compile_fail` programs that pass a `BoundedQuantityType`, a bare `UnitId` and a `Rational` to the constructor. |
| TC-946-step-5 | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:29 | 5. For AC-9, run the three Kani proofs with no unwind attribute and the cover, under the repository's authorized runner. Run the reference comparison on the host over the boundary set to bind the reference to the owner route. The proofs are credited only on `VERIFIED` with the cover satisfied. |
| TC-946-step-6 | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:30 | 6. Run each mutant of the next section, host differential first, then, for the loop and allocation mutants, the Kani proof. Each must fail at its named input; a mutant that passes is a defect in this case, not a result. |
| TC-946-step-7 | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:31 | 7. For AC-10, run CG's FR-050 composed harness against the delivered constructor on a Decimal, an Int and a Rational leaf under the authorized runner, and record the peak memory and its provenance, time, tool version and terminal outcome on IR-1443. If the harness cannot be built yet, record that th |
| TC-946-mutant-order | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:53 | / Order reversed so the domain is checked before canonical form / the `NonCanonical`-and-outside pair of AC-5 / |
| TC-946-mutant-loop | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:54 | / A loop over the scale digits to compute `10^s` / the Kani proof with no unwind bound fails to complete / |
| TC-946-mutant-heap | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:55 | / The magnitude is held in a heap type / the `Copy` bound does not compile and the allocation counter is nonzero / |
| TC-946-mutant-cfg-kani | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:56 | / A body selected by `cfg(kani)` that differs from the host body / the host differential on `(10, 1)` fails / |
| TC-946-mutant-rational-numerator | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:50 | / `Rational` checks only the numerator range / `(1, 3)` in `Rational[1, 5; 2, 4]` is admitted, `(1, 5)` and `(6, 5)` are refused / |
| TC-946-expected | examined | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:60 | Every boundary binding yields the independently written magnitude or the named refusal, and the constructor and the shipped owner route agree on the whole set except the `NonCanonical` pairs. Construction allocates nothing, charges nothing and is `Copy`. The three proofs verify with no unwind bound  |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | medium | The wrapping-lift mutant cannot fail at its named input. In Decimal[0,100;60,70], (1, 0) lifts by 10^60, and 10^60 mod 2^128 is 2^60 times the odd number 5^60 mod 2^68: nonzero, and either negative or at least 2^60, so the wrapped coefficient is outside [0, 100] and the mutant also returns OutsideCarrierDomain. Use a lift of at least 128 digits, where 10^k = 2^k x 5^k wraps to 0: for example Decimal[0, 100; 128, 130] with (1, 0), which the mutant admits as 0 and the constructor refuses. | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:46 |
| FND-008 | medium | The mutant 'treats the sign of an oversize lift as positive' is equivalent: by item 4 the type's bounds fit the native width, so an oversize lifted coefficient is outside [lo, hi] whatever its sign, and the mutant (exceeds hi) and the constructor (below lo) both return OutsideCarrierDomain on (-1, 0) in Decimal[-100,0;60,70] and on every other input. The TC admits no mutant that nothing distinguishes; drop it, or replace it with one that can fail (for example 'treats an oversize lift as inside'). | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:47 |
| FND-009 | low | 'compared after a lossy cast' fails only for a saturating cast. A wrapping `as` cast takes 2^127 to i128::MIN, reversing [0, 2^127], [0, 2^127] and [2, 2^127], so the bindings 1, (1, 0) and (1, 2) are refused by the mutant too. Name the saturating cast, or add a bound such as 2^128 + 1 that wraps to 1 and makes the wrapping mutant admit. | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:63 |
| FND-010 | low | The boundary set lists 'the scales -1 and 4294967296' with no coefficient. The u32-truncation mutant is caught only when the truncated binding (c, 0) is admitted, so c must lie in [-1000, 1000]; name the bindings, for example (1, -1) and (1, 4294967296). | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:21 |

## Dispositions

Round 1, fix commit 441e6d85 on head 5183708 (origin/main 4fe7df6 unchanged).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
| FND-004 | fixed | - |
| FND-005 | fixed | - |
| FND-006 | fixed | - |
| FND-007 | fixed | - |
| FND-008 | fixed | - |
| FND-009 | fixed | - |
| FND-010 | fixed | - |

Round 2 rows above follow fix commit b68819e on 441e6d8 (origin/main 4fe7df6 unchanged); FND-004 closed by the lead's edit of the IR-1443 ticket text.

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-011 | low | Item 6 now orders CarrierMismatch before TypeNotRepresentable, but no boundary binding or mutant exercises that pair: the six mismatches use native-width types and the too-wide types get only matching bindings. A constructor that checks the type first passes. Add a mismatched binding against a too-wide type, for example the Decimal binding (1, 0) against Int[0, 2^127], expected CarrierMismatch, and a mutant row for the swapped order. | spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:52 |
