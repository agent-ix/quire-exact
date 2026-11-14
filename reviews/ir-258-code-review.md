---
id: SR-17940486709564847047963
title: "IR-258 PR #166 Kani-tractable native Rational carrier code review"
type: SpecReview
analysis: code-review
date: 2026-11-14
scope: "agent-ix/quire-exact; PR #166 @ c7f5c7bb on origin/main fec2780; IR-258; src/lib.rs, src/native_fraction.rs, src/native_rational.rs, src/native_rational/kani_proofs.rs, src/native_rational/reference.rs, src/native_decimal/operations.rs, src/native_decimal/operations_tests.rs, src/native_quantity.rs, src/native_quantity/tests.rs, tests/ir258_native_rational.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-408
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-956
    type: reviews
---

## Summary

Ticket: IR-258. PR #166 (head c7f5c7bb, branch code/ir-258-rational-carrier), scoped to `git diff origin/main...HEAD` (10 files, +1969/-124). Methods: code-review with the rust-review lane folded in. Reviewed read-only in /home/peter/dev/worktrees/ir-258-rational-code against FR-408 and TC-956. Kani was not run (lead lane); the Kani verdicts cited are the coder/lead measurements posted on IR-258.

Reviewer runs, focused only, via `locked-build.sh` on the worktree target dir: `cargo test --release --test ir258_native_rational` 12 of 12 passed; `cargo test --release --lib native_` 34 of 34 passed; `cargo clippy --release --lib --test ir258_native_rational -- -D warnings` clean; `cargo fmt --check` clean.

## Verdict

**Changes requested: three medium, one low.** The production code is correct; no wrong result, panic, wrap or spec violation was found in the carrier, operations, charges or the moved gcd code. The medium findings are about the evidence and the spec text, not the arithmetic.

Checked and found correct:

- **Moved, not copied.** `gcd_step`..`gcd256`, `GcdState` and `gcd_steps_128` are deleted from `native_decimal/operations.rs`; `coprime` and `step`..`steps_128` are deleted from `native_quantity.rs`; each exists once, in `src/native_fraction.rs`. FR-403 imports `gcd256` from there; FR-398 calls `is_canonical_pair`. Bodies are byte-identical apart from visibility. FR-398 behaviour unchanged: the zero-denominator check still precedes, and the old `denominator < 0` false branch is now `<= 0`, unreachable for zero there.
- **Width-tiered reduction (Behavior 17a).** `native_fraction::reduce` takes `gcd64` plus `u64` division when both unreduced magnitudes fit `u64`, else `gcd256` plus `u128` division. One source, an ordinary `match` on `u64::try_from`, no `cfg(kani)`; same result for every input. `gcd64` reuses `coprime`'s 128-step ladder; for full-`u64` odd parts the product is below `2^128`, so at most 127 halving steps precede equality, which suffices. The sign restore is right for `i128::MIN`. The 17a probe was run and recorded on IR-258 before the tiered shape was chosen. Both tiers are hit by the host differential (CG domain narrow; boundary set and the 252-step pair wide).
- **Capacity before charge.** `form` runs `LeftProduct`, `RightProduct`, `Combination`, `Denominator` (multiply: `LeftProduct`, `Denominator`) before `charge_arithmetic`; ordering forms both products before `charge_ordering`. Subtract uses `checked_sub` of the two products, so `-1/1 - i128::MIN/1` completes. Equality forms no product.
- **Charge sequence.** operands (`max B`, occurrences 2), arithmetic (`max(N, D)` with the `+ 1` for add/subtract), normalize (unreduced bit lengths), result-retain; ordering operands/arithmetic/result-retain; equality via the existing `charge_native_integer_schedule`. All match FR-408 Behavior 11-13 and the reference.
- **Reference independence.** `reference.rs` imports only the refusal/capacity enums; its gcd is Euclid's remainder ladder, its bit length is a comparison ladder, its capacity is its own `checked_*`; the harnesses compute the order and equality oracles with their own `a * q` / `c * p`.
- **Shape.** No loop, iterator adapter, recursion, allocation or `cfg(kani)` in the proof-path bodies (the only `cfg(kani)` gates the proof and reference modules); fields private (the `compile_fail,E0451` doctest); result, refusal and capacity `Copy`.
- **No ceremony, no vendoring, no source-text scans.** None added. `size_of == 32` is a layout check standing in for "no hidden field".

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The spec this PR implements still says the opposite of the PR. FR-408 Status: "Not implemented; no Rust accompanies this requirement, no Kani harness has run and no measurement exists." TC-956 Status: "No step has run."; its evidence record: "No measurement exists." The FR-408 Description still says the shared helpers "exist only on open PR #162" (merged) and that "`gcd256` is `pub(super)` in the decimal operations module" (now moved). The lead has measured verdicts (7 VERIFIED, 8 TIMED OUT at 900 s, 1 not run) that appear nowhere in the repo. #162 set the precedent of recording measured rows in FR-398 Status. Fix: update FR-408 and TC-956 Status and the TC-956 evidence record with the measured rows, writing TIMED OUT/PARTIAL with no proof claimed for add, subtract, commutative, from_i64_parts and the full-i64 rows, and AC-11 still PLANNED; drop the stale PR #162 / pub(super) prose. | spec/functional/FR-408-kani-tractable-native-rational-carrier.md:30-31, spec/functional/FR-408-kani-tractable-native-rational-carrier.md:121, spec/test-cases/TC-956-kani-tractable-native-rational-carrier.md:57, spec/test-cases/TC-956-kani-tractable-native-rational-carrier.md:68 |
| FND-002 | medium | The Kani arithmetic harnesses cannot see the normalize charge amount, though `arithmetic_completes` is described as checking "the literal four-charge sequence" and Behavior 18 requires the literal charge sequence. The check asserts only the aggregate `consumed(IntegerBits) == max(operands, arithmetic, normalize)`. Normalize is never above arithmetic (`B(aq + cp) <= max(B(a)+B(q), B(c)+B(p)) + 1` and `B(pq) <= B(p)+B(q)`), so the max always hides it. TC-956's mutant "The normalize charge uses the reduced parts" passes every Kani harness. Only the host tests catch it. Fix: observe the per-point amounts (for example an IntegerBits limit equal to arithmetic on a meter that records the normalize request), or state in the harness doc and in TC-956 that the Kani rows do not cover the normalize amount. | src/native_rational/kani_proofs.rs:108-137, src/native_rational/reference.rs:155-170 |
| FND-003 | medium | Behavior 18(b) asks for each of add, subtract, multiply, order and equal to be harnessed with sufficient and insufficient Meter limits. Insufficient-limit harnesses exist only for add (`native_rational_add_denies_*`). There is none for subtract, multiply, ordering or equality, so the incomplete-record and admitted-prefix paths of those four operations have no harness at all, not even a measured timeout. Subtract also has no full-i64 harness, while add and multiply do. Fix: add the denial harnesses for the other operations (at least the occurrences and result-unit denials, which are cheap) and a full-i64 subtract row, and let the lead record their measured verdicts. | src/native_rational/kani_proofs.rs:270-338, src/native_rational/kani_proofs.rs:181-189 |
| FND-004 | low | FR-408 Behavior 6 still says the result is "reduced by the 256-step gcd of FR-403 Behavior 16", and AC-10 says "the reduction calls the one `gcd256`". The shipped code reduces the `u64` tier with the new `gcd64`, which is allowed only through Behavior 17a's conditional. Read literally, AC-10 inspection fails. Fix: amend Behavior 6 and AC-10 to name the 17a tiered reduction, with `gcd64` sharing `coprime`'s ladder in the shared module. | spec/functional/FR-408-kani-tractable-native-rational-carrier.md:65, spec/functional/FR-408-kani-tractable-native-rational-carrier.md:110, src/native_fraction.rs:38-78 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-408-AC-1 | examined | Copy, no drop glue, no heap types, no public field (E0451), zero allocations |
| FR-408-AC-2 | examined | from_i64_parts admitted/ZeroDenominator/NonCanonical sets, precedence, no normalization, no Meter |
| FR-408-AC-3 | examined | from_rational/to_rational round trip; 2^127/1 is Capacity(Input) |
| FR-408-AC-4 | examined | add/subtract/multiply literal reduced values; results canonical |
| FR-408-AC-5 | examined | four orderings and equality literals |
| FR-408-AC-6 | examined | capacity order LeftProduct, RightProduct, Combination, Denominator; Meter unchanged; -1/1 - i128::MIN/1 completes |
| FR-408-AC-7 | examined | literal charge sequences; sufficient/exact/one-less limits |
| FR-408-AC-8 | examined | equality schedule; shared Meter accumulation; denial in second addition |
| FR-408-AC-9 | examined | host differential over 12,321 pairs and boundary set |
| FR-408-AC-10 | examined | inspection: shape and one shared module |
| FR-408-AC-11 | examined | Kani harnesses of Behavior 18; PLANNED; verdicts measured by the lead |
| src/native_fraction.rs | examined | moved coprime/gcd256, is_canonical_pair, gcd64, reduce |
| src/native_rational.rs | examined | carrier, form, charges, operations |
| src/native_rational/kani_proofs.rs | examined | 17 harnesses |
| src/native_rational/reference.rs | examined | independent reference |
| src/native_decimal/operations.rs | examined | gcd256 removed, imported |
| src/native_quantity.rs | examined | coprime removed, is_canonical_pair called |

## Dispositions

Round 1, fix commit 6fcae3f (from c7f5c7b), checked against the code on the branch.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-408 Status now reads "Implemented (IR-258, PR #166)" and keeps AC-11 PLANNED. The stale PR #162 and pub(super) prose is replaced. The TC-956 evidence record lists 25 harness rows (15 VERIFIED, 10 TIMED OUT, no proof claimed), and TC-956 Status matches. |
| FND-002 | fixed | The harness module doc and TC-956 step 9 and its mutant row now say the Kani rows check only the high-water mark and that host steps 6 and 7 own the normalize amount. The host test `charges_match_the_literal_schedule` asserts normalize 3 for `1/2 + 1/2`; the reduced-parts mutant would give 1. |
| FND-003 | fixed | Added denial harnesses for subtract, multiply, order and equal (occurrences and result unit), a full-i64 subtract row, and a CG subtract row with from_i64_parts. Their measured outcomes are in TC-956. The equality denial prefixes match `charge_plan`'s result-unit reservation. |
| FND-004 | fixed | Behavior 6 and AC-10 now name the Behavior 17a width-tiered reduction (`gcd64` for the u64 tier, `gcd256` otherwise). |
