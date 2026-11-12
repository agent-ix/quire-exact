---
id: SR-17940486709564847047713
title: "IR-1444/IR-1508 PR #153 code-review (rust-review lane)"
type: SpecReview
analysis: code-review
date: 2026-11-12
scope: "agent-ix/quire-exact PR #153 at 1ead0352 (base e20118a; main e82db48, merge-tree clean); IR-1508, IR-1444; src/native_decimal.rs, src/native_decimal/kani_proofs.rs (deleted), src/native_decimal/operations_tests.rs, tests/ir1508_native_decimal_ops.rs, with src/native_decimal/operations.rs, src/native_decimal/reference.rs, src/native_decimal/tests.rs as context"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
---

## Summary

Ticket: IR-1508 (also IR-1444). Code review with the rust-review lane folded in, scoped to `git diff origin/main...HEAD` (8 files, +70/-248). The PR deletes the ten decimal Kani harnesses, narrows `reference` to `cfg(test)`, adds one exhaustive host test and adds `FR-403-AC-12` trace tags to four existing tests.

Reviewer runs, in a private detached worktree with its own target dir, under `flock /tmp/cargo-slot.lock`:

- `cargo fmt --check`: clean. `cargo clippy --all-targets -D warnings`, `--features test-support`, and `--lib --target thumbv7em-none-eabi`: all clean.
- `cargo test --lib native_decimal`: 12 passed, 5.5 s. With `--features test-support`: 12 passed. `cargo test --test ir1508_native_decimal_ops`: 17 passed, 3.4 s.
- `bits_of_a_power_of_ten_matches_the_reference_for_every_u32_shift` alone: 4.4 s wall, 69.7 s user CPU in the debug test profile on a 32-core host. It is not `#[ignore]`d, so it runs in every `cargo test` and twice in `make test`.
- `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features`: clean.
- Mutation M1: `power_of_ten_bits` returns one extra bit only at `k = 4_000_000_007`. The new test FAILED at `k = 4000000007`, and it is the only test that fails. The test is real, iterates all 2^32 shifts and can fail.
- Mutations M3 and M4: `LOG2_10_LOWER` changed by -1 and by +1 ulp. The exhaustive test passed both. Only the literal-denominator assertion of the closed-form test `the_log2_ten_approximations_share_a_floor_over_every_u32` failed. This is expected, since both constants still give correct floors over `u32`, but it shows the exhaustive test does not pin the constant (see FND-001).
- `operations.rs` was restored after each mutant, and `git status --short` was empty afterwards.

The diff to the test files is attribute-only apart from the new test: three `#[trace]` lines in `operations_tests.rs`, one in `tests/ir1508_native_decimal_ops.rs`, and the new function plus its reference helper appended. No existing test body changed. A grep of src/, tests/, docs/, Makefile, CLAUDE.md, README, Cargo.toml and .github finds no reference to the deleted harnesses or to `native_decimal::kani_proofs`. The remaining Kani mentions are the units gate (FR-359), the `cfg(kani)` source-inspection tokens, and the `unexpected_cfgs` check-cfg entry, which is still needed by `src/accounting.rs`.

## Verdict

Mergeable once the medium finding is addressed (a wording change is enough). The deletion is complete and nothing dangles: `reference.rs` compiles as `cfg(test)` under every `make ci` lint and test configuration, and the thumb lib build does not see it. The new exhaustive test is a real all-`u32` check that a single-k kernel defect fails. FND-001 is about what the spec sentence says the test proves. FND-002 is stale comment text.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-403-AC-14 and Behavior 40 say the kernel's `bits(10^k)` "equals the independent reference `floor(k x log2(10)) + 1`". `ten_power_bits_reference` is the kernel's own fixed-point scheme with the upper constant `L + 1` written out as a literal. Its assertion is therefore implied by the test's first assertion (lower floor == upper floor), and the third assertion restates the kernel formula. Nothing establishes that log2(10)·2^90 lies in [L, L+1]. Mutants M3 and M4 (L ± 1) pass the exhaustive test. The only independent anchor is the owner's exact `power_of_ten_bits` at about 206 sampled shifts. Reword AC-14 and Behavior 40 to say "the upper-approximation reference", and state that exactness additionally rests on the bracket of `LOG2_10_LOWER`. Or add an independent bracket check. | src/native_decimal/operations_tests.rs:478-516, spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:97,117 |
| FND-002 | low | Kani-only remnants left after the harness removal. `LOG2_10_UPPER` is still `#[cfg(any(test, kani))]`, although no kani consumer remains. The `reference.rs` doc says the multiplicity helper is "unrolled without a loop so a proof needs no unwind bound", but the module is now test-only. Narrow both to `cfg(test)` and update the comment. | src/native_decimal/operations.rs:25, src/native_decimal/reference.rs:207-209 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| src/native_decimal/kani_proofs.rs | examined | deleted; ten #[kani::proof] harnesses (from_bindings, normalized, six ops, ordering, log2) |
| src/native_decimal.rs | examined | `#[cfg(kani)] mod kani_proofs;` removed; `#[cfg(any(test, kani))] mod reference;` -> `#[cfg(test)]` |
| src/native_decimal/operations_tests.rs | examined | trace tags on grid/random/gcd tests; new `ten_power_bits_reference` and `bits_of_a_power_of_ten_matches_the_reference_for_every_u32_shift` |
| tests/ir1508_native_decimal_ops.rs | examined | trace tag FR-403-AC-12 on `native_and_owner_agree_over_the_boundary_set` |
| src/native_decimal/operations.rs | context_only | `power_of_ten_bits`, `LOG2_10_LOWER`, `LOG2_10_UPPER` |
| src/native_decimal/reference.rs | context_only | module doc, `native_decision`, `is_least_scale_pair`, `is_member` |
