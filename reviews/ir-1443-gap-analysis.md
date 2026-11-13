---
id: SR-17940486709564847047962
title: "IR-1443 PR #162 Kani-tractable carried-quantity constructor gap analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-exact; PR #162 @ 32b6008f on origin/main b3bc38f; IR-1443; FR-398, TC-946; src/native_quantity.rs, src/native_quantity/*.rs, tests/ir1443_native_quantity.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-398
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-946
    type: reviews
---

## Summary

Ticket: IR-1443. Planless gap analysis of PR #162 against FR-398 and TC-946. Plan completion: not assessed.

`quoin matrix --repo . --json` (quoin 0.30.1) was run and reports every FR-398 criterion `untagged`, and FR-402's criteria too, although both repos' tests carry `#[trace("TC-…", "FR-…-AC-n")]` attributes. The matrix does not read this repo's `ix_trace_rs` tags in the current configuration, so it gives no signal for this PR; this is a tooling/config observation, not a PR defect. The AC-to-test mapping below is therefore manual.

Manual AC-to-test check (each test read and its assertions matched to the AC text; host tests run by the reviewer: lib `native_quantity` 4/4, `tests/ir1443_native_quantity.rs` 10/10, doctests `native_quantity` 4/4 including the three `compile_fail,E0308`):

- AC-1: `int_bindings_give_n_over_one`. Every listed binding, real literals.
- AC-2: `decimal_bindings_give_the_exact_canonical_value`. Every listed binding and carrier, including `(0,7)` with `smax = 2`, the `smin` lift, the 60..70 and 128..130 sign-only lifts, `(1, 4294967296)` and `(1, -1)`.
- AC-3: `rational_bindings_give_n_over_d_in_order`. Corners, single violations, the four `NonCanonical` and two `ZeroDenominator` pairs.
- AC-4: `decimal_fit_is_decided_by_the_canonical_fraction`. `(1,38)`, `(1,39)`, `(1,4294967295)`, `(2^39,39)` to `1/5^39`.
- AC-5: `a_binding_of_another_carrier_is_a_mismatch_before_anything_else`. Six ordered pairs and the three precedence cases.
- AC-6: `an_oversize_type_bound_refuses_every_binding_of_its_carrier`. `2^127` and `2^128+1` for each carrier, owner-admitted binding, zero-denominator and non-canonical bindings, plus an oversize lower bound.
- AC-7: `the_constructor_is_loop_free_copy_unit_free_and_allocation_free`. Token scan of the production part, fn-pointer coercion, `Copy`, `needs_drop`, zero allocations over the whole boundary set, read-out after the window.
- AC-8: `the_constructor_agrees_with_the_owner_route_except_three_sets` (exact set equality of the differences against an independently listed expectation) and `the_exponent_reference_agrees_with_the_owner_route_to_scale_64`.
- AC-9: three module doctests, `compile_fail,E0308`, all passing.
- AC-10: three Kani proofs written; not executed (lead runs them). Not credited until `VERIFIED` with the cover; Status says so.
- AC-11: not executable until IR-263; not claimed.

Mutant sensitivity, by inspection (TC-946 step 6 was not run; see FND-001): every input mutant of the TC-946 table has its named kill input among the literal cases of AC-1..6 (for example `(549755813888, 39)` for "fits by scale <= 38", `(2,0)` in `Decimal[0,100;2,3]` for the skipped lift, `(1,0)` in `Decimal[0,100;128,130]` for the wrapping lift, `(1,0)` against `Rational[1,5;2,2^127]` for type representability after binding checks); the inspection mutants are caught by the AC-7 token scan, `Copy`/`needs_drop` checks and the fn-pointer coercion. The Kani harness is not vacuous: assumptions are satisfiable, each proof has a success cover, and it asserts full outcome equality with the reference.

No production code without an owning requirement: every item in the module traces to FR-398 Behavior 1 to 7; `reference.rs` and `kani_proofs.rs` are cfg-gated evidence for Behavior 9. No stubs, no `todo!`, no coverage inflation beyond the tag issues in the code-review artifact.

## Verdict

**PASS with two low findings.** AC-1..9 are each backed by a real, assertion-bearing test that passes; AC-10 is honestly reported as written, not executed; AC-11 is honestly not claimed. Neither finding blocks merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-946 step 6 ("Run each mutant ... Each must fail as named; a mutant that passes is a defect in this case") was not executed; Status records this honestly. By inspection each mutant's kill input is present, so the risk is low, but the case's own mutant-kill claim stays unverified. Fix: run the mutant table once (scratch worktree) before closing IR-1443, or record in TC-946 Status which mutants were inspected rather than run. | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:32, spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:80 |
| FND-002 | low | TC-946 step 2 says to "check the reference itself against the owner route for every Decimal binding of scale at most 64". `the_exponent_reference_agrees_with_the_owner_route_to_scale_64` checks the test-side exponent reference (`exponent_fraction`) over a fixed coefficient list (`0, ±1, 2, 5, 10, 2^39, 5^27, i64::MIN, i64::MAX, 999999`), not over the Decimal boundary bindings: `-5`, `±1000`, `100000`, `±1001`, `5000`, `9999`, `10001` are absent. The exponent reference is the oracle only for scale `4294967295`, so the gap does not hide a wrong result today. Fix: add the boundary Decimal coefficients to that list. | tests/ir1443_native_quantity.rs:605-639, spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:28 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-398-AC-1 | examined | Int bindings n/1 and Int[-5,100] edges |
| FR-398-AC-2 | examined | Decimal canonical value and normalized-pair membership |
| FR-398-AC-3 | examined | Rational order, canonical refusals, single violations, corners |
| FR-398-AC-4 | examined | Decimal NotRepresentable by canonical fraction |
| FR-398-AC-5 | examined | CarrierMismatch pairs and precedence |
| FR-398-AC-6 | examined | TypeNotRepresentable for oversize bounds |
| FR-398-AC-7 | examined | loop-free, Meter-free, Copy, unit-free, zero allocations |
| FR-398-AC-8 | examined | host differential, three intended differences |
| FR-398-AC-9 | examined | compile_fail E0308 controls |
| FR-398-AC-10 | examined | Kani proofs written, not executed |
| FR-398-AC-11 | examined | not executable until IR-263; not claimed |
| TC-946 | examined | procedure steps 1-7 and mutant table |
