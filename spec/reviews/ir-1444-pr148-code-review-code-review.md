---
id: SR-17940486709564847047681
title: "IR-1444 PR #148 native Decimal carrier code review"
type: SpecReview
analysis: code-review
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #148 @ 7b451716 on origin/main 885efce; IR-1444; src/native_decimal.rs, src/native_decimal/{kani_proofs,reference,tests}.rs, src/lib.rs, tests/ir1444_native_decimal.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-950
    type: reviews
---

## Summary

Ticket: IR-1444. PR #148 (head 7b451716, branch code/ir-1444-decimal-carrier), scoped to `git diff origin/main...HEAD` (8 files, +963/-2). Methods: code-review with the rust-review lane folded in. Reviewed read-only in /home/peter/dev/worktrees/IR-1444-review against FR-402 and TC-950 (merged in QE #146), FR-398 and FR-403 (QE #147), and the owner code in src/decimal.rs and src/integer.rs.

Reviewer test job (flock'd, `--locked`, own target dir, no Kani): `--test ir1444_native_decimal` passed 11 of 11. `--lib native_decimal` passed 2 of 2. `--doc native_decimal` passed 5 of 5: the positive twin and the four compile_fail programs. Mutation control: I deleted the `strip_zeros(.., 32, POWERS_OF_TEN[32])` step from `normalized()`. Two tests failed. `canonical_and_trailing_zero_forms_are_distinct` gave `(1000000, 9)` for `(10^37, 40)` where `(1, 3)` was expected. `references_agree_with_the_owner_route_over_the_grid` also failed. I reverted the mutation and the worktree was clean.

## Verdict

**CHANGES REQUIRED (one medium; the rest are low).** No defect in production code was found. I checked each of the following and found it correct:

- **`NativeDecimalType::new`.** It is the owner's `DecimalType::new` check, token for token, on `i128` bounds: `max_scale` is narrowed first, then `min_scale`, then `lower > upper || min > max`, all giving `MalformedDecimalType`.
- **Ingress.** It uses `Integer::native()`, which returns `None` for `LargeDigits` and never wraps or saturates. Its only refusal is `TypeNotRepresentable`.
- **Refusal order in `from_bindings`.** Ingress comes first, by construction. The `i64` scale is narrowed with `u32::try_from` before any membership check, so no `as` cast occurs. Membership is judged on the normalized pair, and the retained pair is returned unchanged.
- **`admits_normalized`.** It matches `DecimalType::contains`. Zero short-circuits on the bounds only after the `s* <= smax` test. A shift above 38 is false, and so is a `checked_mul` of `None`. Both bounds fit `i128`, so an out-of-width lift lies outside them in its own sign. Nothing wraps, saturates or materializes.
- **`normalized()`.** It strips 32, 16, 8, 4, 2 and 1 zeros, each step guarded by `scale >= places && c % 10^places == 0`. Let `k = min(trailing zeros, scale)`. Each step strips `p` exactly when the remaining `k` is at least `p`, so the steps remove the binary digits of `k`. Since `k <= 38 < 63`, the result is the least-scale pair. `i128::MIN % 10^p` and `/ 10^p` cannot overflow, and the zero case is `(0, 0)`.
- **Boundary literals, recomputed.** `(10^37, 40)` normalizes to `(1, 3)`. `(10^38, u32::MAX)` normalizes to `(1, 4294967257)`. `(9*10^18, 20)` normalizes to `(9, 2)`. `(10^18, 5)` normalizes to `(10^13, 0)`. `i128::MIN` has no factor of five. The `POWERS_OF_TEN` const block reaches `10^38` (below `i128::MAX`) without overflow. 
- **AC-9 inspection.** The proof-path bodies have no loop, no iterator adapter, no `cfg(kani)` and no owner type. The `while` is in a `const` initializer, which is evaluated at compile time.
- **Panic and `unsafe` surface.** Production code has no `unsafe`, no `unwrap` and no panic. Every table index is bounded by `<= 38` or is a constant.
- **`cfg` gating.** `kani_proofs` is `cfg(kani)` and `reference` is `cfg(any(test, kani))`. `unexpected_cfgs` already declares `cfg(kani)`, so the host build is unaffected.
- **Kani harnesses.** Both use `kani::any`, `kani::assume`, `kani::cover!(cond, msg)` and `#[kani::proof]` correctly. `RoundingMode` reaches them through `use super::*`, as in tests.rs. Their names match TC-950 step 5, they carry no unwind attribute, and they name four and three covers. Proof 1 fixes the least-scale pair by assumption, which is sound only because that pair exists for every input. Proof 2 establishes that existence by asserting it for the real `normalized()` over all `i128`/`u32`, so the two proofs must be credited together.
- **compile_fail doctests.** Passing an `Integer` or a `Decimal` where `i64` is expected, and a `&DecimalType` where `&NativeDecimalType` is expected, gives E0308. A struct literal with private fields gives E0451. The codes are right for the right reasons, and rustdoc ran all four.
- **Counting allocator.** allocation-counter 0.8.1 installs `#[global_allocator]` unconditionally and counts per thread, so the window does observe the calls. It is not vacuous.
- **Source inspection.** It can fail: it matches whole words, so a `for`, `while`, `loop` or owner-type token in the four bodies fails it.
- **Names and exports.** `NativeDecimal`, `NativeDecimalType` and `NativeDecimalRefusal` are exported. The two variant names are FR-398's, as FR-402 Behavior 6 prescribes. FR-398 has no code yet, so the enum is FR-402's reduced vocabulary and not an invented API. `normalized()` mirrors the owner. `to_decimal()` mirrors `Decimal::to_rational`. `from_bindings` is the name the TC-950 proof names imply.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-402-AC-7 and TC-950 step 3 require "the construction of the whole boundary set and the normalized view of every normalized-view input" inside the counting-allocator window. The window builds 7 bindings in 2 types, out of roughly 38 bindings in 11 types, and normalizes 3 of the 14 inputs. The checked_mul-overflow lift ((2,0) in Decimal[0,i128::MAX;38,38]) is never built in the window. Neither is the oversize-shift path ((1,0) in [0,100;60,70] and [0,100;128,130]), the i64::MIN/MAX bindings, or the (10^37,40) and i128::MIN normalized inputs. AC-7 is credited in the FR-402 Status on this partial evidence. Hoist the boundary tables and build or normalize every entry inside `measure`, with the types constructed outside it. | tests/ir1444_native_decimal.rs:273-293 |
| FND-002 | low | `refusal_carries_no_decimal_and_has_two_causes` is tagged FR-402-AC-3. Its second assertion, `assert_ne!(TypeNotRepresentable, OutsideCarrierDomain)`, compares two distinct unit variants and cannot fail. Its first assertion repeats the AC-2 input (1,3). It is FR-402 Behavior 6 evidence, not AC-3 evidence. Drop the tautology and either retag it or remove the test. | tests/ir1444_native_decimal.rs:354-360 |
| FND-003 | low | The cover "oversize lift decided by sign" is gated on `shift > 38`, so it witnesses only the oversize-power branch. FR-402 Behavior 9 includes symbolic types "at which the lift ... wraps or exceeds the native width". The `checked_mul` returns None branch has no cover: shift <= 38 with \|c' x 10^shift\| beyond i128, as with (2,0) in [0,i128::MAX;38,38]. The universal assertion still covers that branch, so a reachability check is what is missing. Add a cover such as `c2 != 0 && lifted <= max_scale && shift <= 38 && i128::from(c2).checked_mul(10^shift).is_none()`, written with the reference's own table. | src/native_decimal/kani_proofs.rs:52-56 |
| FND-004 | low | The AC-7 inspection's adapter list covers .iter(, .map(, .filter(, .fold(, .all(, .any(, .find( and into_iter. It misses .for_each(, .try_fold(, .zip(, .rev(, .take( and .sum(. `for_each` is a single word, so the whole-word `for` check does not catch it either. If a later edit replaced the 32/16/8/4/2/1 strip with `[32,16,8,4,2,1].into_iter()` it would be caught, but `.for_each(` over a range would not. A simpler rule is to ban any `..` range and any `.` method call outside an allowlist. | tests/ir1444_native_decimal.rs:338-351 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| src/native_decimal.rs | examined | NativeDecimalRefusal, NativeDecimalType::new, admits_normalized, TryFrom<&DecimalType>, NativeDecimal::{from_retained, from_bindings, coefficient, scale, normalized, to_decimal}, strip_zeros, POWERS_OF_TEN, module doctests |
| src/native_decimal/kani_proofs.rs | examined | native_decimal_from_bindings_refines_owner_membership, native_decimal_normalized_is_least_scale_pair |
| src/native_decimal/reference.rs | examined | TEN_POWERS, split, order, is_least_scale_pair, is_member |
| src/native_decimal/tests.rs | examined | references_agree_with_the_owner_route_over_the_grid, least_scale_reference_rejects_wrong_pairs |
| tests/ir1444_native_decimal.rs | examined | 11 integration tests tagged TC-950 / FR-402-AC-1..9, AC-12 |
| src/lib.rs | examined | pub use native_decimal::{NativeDecimal, NativeDecimalRefusal, NativeDecimalType} |
| FR-402 Behavior 1-9 | context_only | Behaviors 1 to 9 of FR-402 |
| TC-950 steps 1-6 | context_only | Test Procedure steps 1 to 6 and the Mutants table |

## Dispositions

Round 1, verified at 5331d53e on origin/main 885efce. Merge-tree against origin/main is clean, and the one other open PR (#141) touches no file in this PR. Reviewer test job, flock'd, `--locked`, no Kani: `--test ir1444_native_decimal` passed 11/11, `--lib native_decimal` passed 3/3, `--doc native_decimal` passed 5/5. Mutation control: `NativeDecimalType::admits` was changed to judge the retained pair instead of `normalized_pair()`. Four tests failed: `retention_and_normalized_membership`, `read_out_and_owner_differential`, `references_agree_with_the_owner_route_over_the_grid` and `membership_is_judged_on_the_normalized_pair_even_for_a_retained_pair`. The mutation was reverted. No new findings.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | `copy_function_pointers_and_zero_allocation` builds every binding of the shared `BOUNDARY` table, normalizes every input of `NORMALIZED`, and asserts `count_total == 0`; the types are built outside the window. `BOUNDARY` holds all 11 TC-950 types and every one of their bindings, plus three extra types. `NORMALIZED` holds all 14 TC-950 inputs. The window therefore covers the overflowing lift ((2,0) in [0,MAX;38,38]), the oversize power ([0,100;60,70], [0,100;128,130]), i64::MIN/MAX, the out-of-range scales, and the i128::MIN and 10^37/10^38 normalizations. |
| FND-002 | fixed | `refusal_carries_no_decimal_and_has_two_causes` is removed. `read_out_and_owner_differential` now asserts that every `Err` is exactly `OutsideCarrierDomain`, and that an out-of-range scale equals `OUT`, so a mutant returning `TypeNotRepresentable` fails it. The ingress refusal is still asserted in `type_constructor_and_ingress_agree_with_the_owner`. |
| FND-003 | fixed | A new cover, "overflowing lift decided by sign", fires when `c2 != 0 && lifted <= max_scale && shift <= 38 && c2.unsigned_abs() > (u128::MAX >> 1) / TEN_POWERS[shift as usize]`. The old cover is renamed "oversize power decided by sign". `TEN_POWERS` is `pub(super)` u128 and `unsigned_abs` gives u128, so the cover type-checks, and `&&` keeps the index at 38 or below. (2,0) in [0,i128::MAX;38,38] reaches it. Unexecuted, as the brief requires. |
| FND-004 | fixed | `violations()` now treats any `.method(` call outside an explicit allowlist, and any `..`, as a violation. The allowlist is checked_mul, try_from, normalized, normalized_pair, admits, admits_normalized and from_retained. The inspected bodies are now from_bindings, normalized, normalized_pair, strip_zeros, admits_normalized and admits, and the production bodies pass. `source_inspection_flags_each_banned_construct` flags nine samples, including `(0..3).for_each`, and has a checked_mul negative control, so the inspection can fail. |
