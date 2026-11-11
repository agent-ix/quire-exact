---
id: SR-17940486709564847047685
title: "IR-1508 PR #149 native Decimal operations gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #149 @ 0cc3bb82 on origin/main 62de2aa; IR-1508; FR-403 AC-1..AC-13 against tests/ir1508_native_decimal_ops.rs, src/native_decimal/operations_tests.rs, src/native_decimal/kani_proofs.rs and src/native_decimal/{operations,readout,reference}.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-951
    type: reviews
---

## Summary

Ticket: IR-1508. This is a planless gap analysis of PR #149 against FR-403 (13 ACs) and TC-951, using the computed matrix (`quire matrix --scope .`) and a reading of every tagged test. Plan completion: not assessed.

Matrix state at 0cc3bb82: AC-1 to AC-11 are tagged by 25 tests (16 integration, 9 unit). AC-12 is tagged by three host tests. AC-13 is `method-without-symbol`, which is correct, because it waits for IR-259.

Owner differential, checked directly: `native_and_owner_agree_over_the_boundary_set` runs every operation and all four orderings over 19 types and 38 operands (the brief's "24 types" is wrong: there are 14 fixed types plus 5 mode variants). Each case runs once natively, through `admit` and then `evaluate_native_decimal` / `order_native_decimals`, and once through the owner's `evaluate_decimal` / `order_numbers` over `Decimal::new` operands, both under a recording `Meter` with the same `bounded()` limits. The test asserts equality of the outcome, the read-out `DecimalResult`, the charge events, the consumed counters and the admission count. A native capacity refusal is checked against `model()`, which is written independently with owner `Integer`/`Rational` from the Behavior 3 grounds. The owner is not asserted on refused cases, as TC-951 step 2 requires. The test also requires more than 10,000 compared cases and more than 1,000 refused ones.

Source-inspection scanner (AC-10): it is an allowlist. Any `.name(` method call outside `ALLOWED_METHODS` is flagged, and so are the tokens `cfg(kani)`, `Vec`, `Box`, `alloc`, `..`, the loop keywords and the owner type names as whole words, and any call-graph cycle. `the_inspection_flags_every_forbidden_shape` holds 14 positive controls, each of which must raise its flag, plus one negative control, so the scanner can fail. The 256-step gcd ladder is checked structurally.

## Verdict

**CHANGES REQUIRED (two medium).** Nothing is a stub, and the tests assert literals that can fail. The reviewer's mutation controls confirm this (see the code-review SR). The gaps are one partly covered criterion and one tag that inflates the matrix.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-403-AC-12 is a Kani-proof criterion, and the FR-403 Status says it is not credited. Three host tests are tagged `#[trace("TC-951", ..., "FR-403-AC-12")]`: the two reference-binding tests and `the_reference_rejects_wrong_decisions`. `quire matrix` therefore reports FR-403-AC-12 as `tagged` by passing host tests. That is coverage inflation, and it contradicts the Status. Behavior 38's host binding has no AC of its own. Drop FR-403-AC-12 from the three tags. AC-11 alone is enough for the binding tests, and the control test can carry AC-11 or no AC. | src/native_decimal/operations_tests.rs:185, src/native_decimal/operations_tests.rs:243, src/native_decimal/operations_tests.rs:458 |
| FND-002 | medium | FR-403-AC-7 requires the limit sweep for every operation and each ordering "over operands with equal scales, unequal scales (a scale-expansion shift above zero) and a rounding step". `swept()` sweeps Subtract only at equal scales ((5,1)-(1,1)), Negate only at (0,0), and Multiply never at unequal scales. Subtract has no rounding-step case and no unequal-scale case. Negate has neither. The boundary-set differential runs only at `bounded()` limits, not at exact or one-less limits, so it does not close the gap. Add, for example, `Subtract((1,0),(1,3))` in Decimal[-10000,10000;0,3], `Subtract((1,3),(0,0))` under a rounding mode with T=2, `Negate((-125,3))` under nearest-even in [-100,100;0,2], and `Multiply((5,1),(4,0))` to `swept()`. | tests/ir1508_native_decimal_ops.rs:1034-1072 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-403-AC-1 | examined | Admission is exact at its edges and precedes any charge (sum, product, shift, exponent, negation, doubled remainder, canonical quotient, zero lift cases). |
| FR-403-AC-2 | examined | Retained representation: unnormalized retained pairs at T. |
| FR-403-AC-3 | examined | Typed outcomes and their charge points (DivisionByZero, InexactDecimal, DecimalOutOfDomain). |
| FR-403-AC-4 | examined | Rounding results in the five modes. |
| FR-403-AC-5 | examined | The loss record is the owner's, factored. |
| FR-403-AC-6 | examined | Charge amounts are the owner's, in the owner's order. |
| FR-403-AC-7 | examined | Charges hold at every limit, on fresh and cumulative Meters. |
| FR-403-AC-8 | examined | Ordering is by value and charges as Behaviors 23 to 25 state. |
| FR-403-AC-9 | examined | bits(10^k) exact over the whole shift range; all-u32 approximation agreement by proof. |
| FR-403-AC-10 | examined | Proof-path bodies: one body, no cfg(kani), loop, adapter, recursion, owner type, Vec; Copy; zero allocations. |
| FR-403-AC-11 | examined | Native and owner agree over the boundary set except listed capacity refusals. |
| FR-403-AC-12 | examined | Kani proof per operation and ordering with covers. |
| FR-403-AC-13 | examined | CG measurement under IR-259. |
