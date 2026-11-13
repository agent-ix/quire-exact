---
id: SR-17940486709564847047961
title: "IR-1443 PR #162 Kani-tractable carried-quantity constructor code review"
type: SpecReview
analysis: code-review
date: 2026-11-13
scope: "agent-ix/quire-exact; PR #162 @ 32b6008f on origin/main b3bc38f; IR-1443; src/native_quantity.rs, src/native_quantity/kani_proofs.rs, src/native_quantity/reference.rs, src/native_quantity/tests.rs, tests/ir1443_native_quantity.rs, src/lib.rs, spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md, spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-398
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-946
    type: reviews
---

## Summary

Ticket: IR-1443. PR #162 (head 32b6008f, branch code/ir-1443-quantity-carrier), scoped to `git diff origin/main...HEAD` (8 files, +1791/-2). Methods: code-review with the rust-review lane folded in. Reviewed read-only in /home/peter/dev/worktrees/ir-1443-code against FR-398 and TC-946 (authoritative). Kani was not run (the lead runs the proofs).

Reviewer runs, focused only, through `locked-build.sh` on the worktree's own target dir: `cargo test --lib native_quantity` passed 4 of 4; `cargo test --test ir1443_native_quantity` passed 10 of 10; `cargo test --doc native_quantity` passed 4 of 4 (one example and the three `compile_fail,E0308` controls); `cargo fmt --check` clean; `cargo clippy --lib --test ir1443_native_quantity -- -D warnings` clean. No `make ci`, no full suite, no Kani, no mutant builds.

## Verdict

**PASS with findings (two medium, three low).** No wrong result, panic or spec violation found in the constructor. Mergeable after the two medium findings are fixed; the low ones may ride in the same fix round.

Checked and found correct:

- **Refusal cause order (Behavior 6).** `construct` matches `Unrepresentable(kind)` first and returns `TypeNotRepresentable` only when the binding kind equals the type kind, otherwise `CarrierMismatch`, so mismatch precedes type representability. Rational: `ZeroDenominator`, then `NonCanonical` on the raw pair, then `OutsideCarrierDomain`. Decimal: `OutsideCarrierDomain` (scale outside `u32`, or non-member, from `NativeDecimal::from_bindings`) before `NotRepresentable`. `Int` and `Rational` never yield `NotRepresentable`.
- **Decimal path.** Membership is delegated to `NativeDecimal::from_bindings` (FR-402's owner-rule implementation), as FR-398's description allows ("the two share only the owner membership rule"); it creates no owner `Decimal`. The canonical fraction is computed from the normalized pair: `twos = min(v2(c'), s')`, `fives = min(v5(c'), s')`, numerator `c'/(2^twos x 5^fives)`, denominator `2^(s'-twos) x 5^(s'-fives)` with explicit exponent ceilings 126 and 54 and a `checked_mul`; correct for every admitted pair, never builds `10^s`, no overflow path. `factors_of_five` uses 16+8+4+2+1 = 31 places against a maximum of 27 factors of 5 in `|c| <= 2^63`; greedy descending strip is exact because divisibility by `5^p` is monotone.
- **Binary gcd unroll (128 steps).** Each non-terminal step replaces the larger odd value by the odd part of the difference, which is below half of it, so the product of the pair at least halves; it starts below `2^126` (odd parts of `|n| <= 2^63`) and a non-terminal step needs a product of at least 3 (`1 x 3`), so at most 125 non-terminal steps precede the terminal `(g, 0)` step; later steps are the identity. 128 suffices for every pair, including the full `u64` range the host test feeds (product below `2^128`, at most 127 steps). The both-even early exit and stripping twos from the other operand preserve coprimality. No `for`, `while`, `loop` or iterator adapter; the unroll is by function composition.
- **No heap, no Meter, one body.** `NativeQuantity` is two `i128`, `Copy`, no drop glue, no `UnitId`; `from_bindings` and `construct` take no `Meter`; no `cfg(kani)` in the production part of the module (the only `cfg` tokens gate the `kani_proofs`, `reference` and `tests` modules). `NativeQuantityShape::of` reads bounds via `Integer::native()` (no allocation). `to_quantity` allocates and is documented as outside the window; its `expect` is guarded by the private-field invariant `denominator > 0`.
- **Reference independence.** `reference.rs` imports only the refusal enum; its tables, Euclidean gcd, multiplicity-by-divisibility and membership are written separately. Its Decimal membership re-derives the lift on the least-scale pair and agrees with FR-398 Behavior 4 (zero lifted to `smin`, sign-only decision past the native width).
- **Kani harness (not run).** Three `#[kani::proof]` with no unwind attribute; all type bounds symbolic within the native width (`i128` endpoints, `u32` `smin`/`smax`), each assume satisfiable and matching the owner type's own constructor invariants (`lower <= upper`, `smin <= smax`, Rational `d_lower >= 1` as `RationalDomain::new` enforces), a `cover!` on the success path, and `assert_eq!` against the reference over every outcome, so any divergence of the constructor, including a shortened gcd unroll, fails it. Rounding is fixed to `Exact`, which is not a gap: `NativeDecimalType::admits` does not read the rounding mode. `TypeNotRepresentable`/`CarrierMismatch` and `NativeQuantityShape::of` are outside the proofs and stated so in the module doc; they are host-tested.
- **Status honesty.** FR-398 Status says AC-10 is written, not executed, with no verdict, and AC-11 waits on IR-263 with no fit claimed. TC-946 Status says the step 6 mutants were not built and run. Both accurate (but see FND-002).
- **No ceremony.** No pins, SHAs, digests, manifests or version records added. `size_of::<NativeQuantity>() == 32` in the AC-7 test is a layout assertion standing in for "no hidden field"; harmless.
- **`#[trace]` tags.** Integration tests carry `#[trace("TC-946", "FR-398-AC-n")]` for AC-1..9; the in-crate grid carries AC-8/AC-10; Kani proofs untagged, matching the repo's other `kani_proofs.rs` files.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-398 Behavior 9 requires "A host test SHALL bind that reference to the owner route over the boundary set." The in-crate tests bind `reference::{int,decimal,rational}` to the owner route over a separate grid, not over the TC-946 boundary set, and the grid misses boundary members: `Rational[2,5;3,7]` corners `(2,3)`, `(2,7)`, `(5,3)` and the one-at-a-time violations `(1,4)`, `(3,2)`, `(2,9)` are not in `PAIRS`; the Decimal scale `4294967296` (not in `SCALES`), the `smin` range `(128,130)` and the bound `549755813888` are not in the grid. So the reference the Kani proofs compare against is not shown to agree with the owner route on the cases the spec names. Fix: add the TC-946 boundary bindings and types to the grid constants (owner route for all Int/Rational and Decimal scale <= 64, literal `OutsideCarrierDomain` for scales `-1` and `4294967296`). | src/native_quantity/tests.rs:18-59, src/native_quantity/tests.rs:168-191, spec/functional/FR-398-kani-tractable-carried-quantity-constructor.md:58 |
| FND-002 | medium | TC-946 Description still says "This case is planned; no part of it has run." while the updated Status says steps 1 to 4 and the host half of step 5 have run. The file contradicts itself on what has run, the exact thing Status is meant to state honestly. Fix: drop or update that sentence in the Description. | spec/test-cases/TC-946-kani-tractable-carried-quantity-constructor.md:14 |
| FND-003 | low | The comment on the reference's `FIVE_POWERS` says "`5^55` exceeds `u128`". It does not: `5^55` is about `2.78e38`, below `u128::MAX` (about `3.40e38`); it exceeds `i128::MAX`. The table bound is right for the reason the constructor's table states ("inside signed 128 bits"); the comment is wrong. | src/native_quantity/reference.rs:84 |
| FND-004 | low | The three in-crate grid tests are tagged `FR-398-AC-10`. AC-10 is credited only by a completed Kani proof with its cover ("A proof ... that does not complete ... is not credited"); these host tests bind the reference to the owner route (Behavior 9's second sentence), which no AC names. A matrix reading the tags will show AC-10 as test-backed while no proof has run. Fix: tag them AC-8 only, or leave AC-10 to the Kani proofs. | src/native_quantity/tests.rs:128, src/native_quantity/tests.rs:156, src/native_quantity/tests.rs:194 |
| FND-005 | low | `the_constructor_takes_a_carried_quantity_type` is tagged `FR-398-AC-9` but only asserts that an `Int` binding constructs; it cannot fail for any AC-9 defect. The AC-9 evidence is the three `compile_fail,E0308` doctests in the module header, which carry no tag. A tagged test that asserts nothing about its AC inflates coverage. Fix: drop the AC-9 tag from that test (say in its doc that the doctests are the evidence), or remove the test. | tests/ir1443_native_quantity.rs:308-314, src/native_quantity.rs:33-51 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-398-AC-1 | examined | Int[i64::MIN, i64::MAX] bindings yield n/1; Int[-5,100] admits -5, 100, refuses -6, 101 |
| FR-398-AC-2 | examined | Decimal (c, s) yields canonical c/10^s; normalized-pair membership; smin lift; sign-only oversize lift; scale outside u32 refused |
| FR-398-AC-3 | examined | Rational canonical (n, d) yields n/d; NonCanonical, ZeroDenominator; Rational[2,5;3,7] single violations and corners |
| FR-398-AC-4 | examined | Decimal NotRepresentable by canonical fraction: (1,38) admitted, (1,39), (1,4294967295) NotRepresentable, (2^39,39) admitted |
| FR-398-AC-5 | examined | six ordered CarrierMismatch pairs and the three precedence cases |
| FR-398-AC-6 | examined | oversize bound refused with TypeNotRepresentable for every binding of its carrier |
| FR-398-AC-7 | examined | one body, no cfg(kani), no loop/iterator, no Meter, Copy, no UnitId, zero allocations |
| FR-398-AC-8 | examined | host differential with exactly three intended differences |
| FR-398-AC-9 | examined | compile_fail E0308 for BoundedQuantityType, UnitId, Rational |
| FR-398-AC-10 | examined | Kani proof per carrier, no unwind bound, symbolic native-width bounds, success cover |
| FR-398-AC-11 | context_only | composed-harness measurement under IR-263; not claimed |
| src/native_quantity.rs | examined | constructor, shape ingress, decimal fraction, factors_of_five, unrolled binary gcd |
| src/native_quantity/reference.rs | examined | independent reference |
| src/native_quantity/kani_proofs.rs | examined | three proofs |
| src/native_decimal.rs | context_only | NativeDecimal::from_bindings, admits_normalized, normalized_pair |
| src/rational.rs | context_only | RationalDomain::new denominator lower bound >= 1 |
