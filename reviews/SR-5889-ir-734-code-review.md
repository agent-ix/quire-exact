---
id: SR-5889
title: code-review of IR-734 nonzero Integer admission
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-exact@acb577ac11ecdd6afe02583f7dbb578f6be3469b (base d18927e849e8b575113fbeffb9041f5f8812d97a); src/integer.rs, src/division.rs, src/decimal.rs, src/rational.rs, src/lib.rs, tests/nonzero_integer.rs; branch ir734-nonzero-integer; Ticket IR-734'
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-374
    type: references
---

## Summary

Ticket: IR-734. Code review with the rust-review lane folded in, scoped to `git diff origin/main...acb577a` (6 files, +534/-36), against FR-374 AC-1..AC-6, TC-922, the FR-371 AC-5 amendment, FR-357 and the repository CLAUDE.md/AGENTS.md. No PR exists yet (pr=none).

Checked: the private tuple field and the absence of `Default`, `From`, `Deref`, serde or any public unchecked constructor (only `TryFrom<&'a Integer>`; `from_proven_nonzero` is `pub(crate)`); `is_zero` on both `Small` and `Large` representations (every public constructor normalizes through `from_big_normalized`, `BigInt` has no negative zero, and `Large` zero would still be detected by `as_big().is_zero()`); the metered charge order (admission sits at the same position as the former `reject_zero_divisor`, after the operands charge and before the arithmetic charge, in both `member_of` and `euclidean_remainder`); every production `from_proven_nonzero` site (decimal.rs 76, 285, 301, 837, 955, 1089, 1150; rational.rs 50, 93), each carrying a nonzero premise that holds for every public construction path of `Rational` (new, reduce via add/sub/mul/div/pow/divided_by_power_of_two all reach `reduce` with a nonzero denominator and a nonzero numerator); no new `unwrap`/`expect` on caller input; no integer casts, no `#[allow]`, no CI or dependency edits; commit subject carries the `IR-734:` prefix.

Each of the nine `compile_fail` doctests was compiled as a standalone example and fails for the reason its prose claims: E0308 x3 (plain `&Integer` divisor), E0423 (private field), E0594 (write through `get`), E0597 (token outlives referent), E0506 (owner replaced while borrowed), E0277 (no `Default`), E0624 (`from_proven_nonzero` private).

Focused runs at the frozen head, own worktree and target dir: `tests/nonzero_integer.rs` (default and `--features test-support`) 5/5; lib `division decimal rational integer` filters 50/50; doctests 11/11; `ir653_decimal_order`, `ir673_decimal`, `ir653_scalar`, `ir673_scalar`, `ir673_core`, `ir705_bounded_integer` all pass. Full `make ci` was not rerun (leader ran it).

Mutation results (own throwaway worktree, reverted after each):

| Mutant | Killed by |
| --- | --- |
| M1 token field made `pub` | doctest integer.rs:179 (compile_fail) |
| M2 admission accepts zero | `admission_denies_zero_and_borrows_each_original_nonzero_integer`, `metered_zero_stops_after_operands_and_denial_precedes_zero_classification`, lib division tests |
| M3a admit before operands charge in `member_of` | `metered_zero_stops_after_operands...`, lib division test |
| M3b admit before operands charge in `euclidean_remainder` | `metered_zero_stops_after_operands...` |
| M3c admit after arithmetic charge in `member_of` | `metered_zero_stops_after_operands...`, lib division test |
| M4a delete the `assert!` in `from_proven_nonzero` | SURVIVED (see SR-5890) |
| M4b metered `admit_divisor` unchecked (no assert) | `metered_zero_stops_after_operands...`, `division::tests::division_by_zero_is_undefined` |
| M4c metered `admit_divisor` via `from_proven_nonzero` | same as M4b (assert panics) |
| M5 `Rational::new` skips zero-denominator check | `rational::tests::zero_denominator_is_refused` |
| M6 `div_mod_floor` returns (remainder, quotient) | three nonzero_integer tests, five lib division tests |
| M7 truncating and floor bodies swapped | three nonzero_integer tests, `division::tests::profiles_differ_on_negative_operands` |
| M8 `exact_div` uses floor | `exact_div_keeps_infallible_signed_results_and_nondivisible_truncation` |
| M9 `from_proven_nonzero` made `pub` | doctest integer.rs:218 (compile_fail) |
| M10 admission via `as_big().is_zero()` (materializes) | SURVIVED (see SR-5890) |

## Verdict

PASS with two low findings. FR-374 behaviour is implemented as specified: zero admission is a typed `Err(ZeroDivisor)`, the token cannot be constructed, mutated or retained past its referent from outside the crate, the helper signatures match FR-374 exactly, `exact_div` keeps truncation for nondivisible inputs, and `DivisionByZero` plus the operands-first charge order are preserved byte for byte against origin/main. The private `assert!` cannot fire on any public path. Findings are documentation and doctest-robustness nits only.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | compile_fail doctests carry no expected error code, so a snippet that stops compiling for an unrelated reason still passes | src/integer.rs:159-222 |
| FND-002 | low | NonZeroInteger rustdoc shows only failing snippets; no positive example names TryFrom as the sole construction route | src/integer.rs:152-224 |

FND-001 failure scenario: a later rename of `get`, a removed crate-root re-export, or a typo in one snippet turns it into an E0425/E0432 failure and the control keeps passing while no longer testing the boundary it names. Pinning `compile_fail,E0308` (and E0423, E0594, E0597, E0506, E0277, E0624 respectively) ties each control to its claim.

FND-002 failure scenario: a reader of the public docs for the new type sees nine non-compiling examples and the `get` method but no statement or example of how to obtain a token (`NonZeroInteger::try_from(&x)`), which FR-374 names as the sole external construction route.

## Review Provenance

Model claude-opus-5-5, run 206b7470-6fa5-4c22-9d9a-bbd70032b444. Quire 0.36.2 (engine 0.50.2), Quoin 0.28.3, spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502. Reviewed in a detached throwaway worktree at acb577a with its own CARGO_TARGET_DIR; merge-base with origin/main verified as d18927e.

## Dispositions

Round 1, verified at 838d544eeb485c04af7b0e6bf25c5b0cda31baee (`git diff acb577a..838d544`).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | All nine compile_fail fences now carry error codes (E0308 x3, E0423, E0594, E0597, E0506, E0277, E0624); each snippet recompiled standalone emits exactly that code as its only error, and mutants M1, M9 and a Default impl (M11) each fail exactly the pinned doctest. |
| FND-002 | fixed | src/integer.rs:157-168 now states that external callers obtain the token through `TryFrom<&Integer>` and adds a passing doctest reusing one token across all three helpers; it compiles and passes standalone. |
