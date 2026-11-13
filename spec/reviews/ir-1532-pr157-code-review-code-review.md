---
id: SR-17940486709564847047768
title: "IR-1532 PR #157 KERNEL_VERSION constant code review"
type: SpecReview
analysis: code-review
date: 2026-11-13
scope: "agent-ix/quire-exact; PR #157 @ c50883c7 on origin/main a4184c4; IR-1532; src/lib.rs, tests/ir1532_kernel_version.rs, spec/functional/FR-406-kernel-version-constant.md, spec/test-cases/TC-954-kernel-version-constant.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-406
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-954
    type: reviews
---

## Summary

Ticket: IR-1532. PR #157 (head c50883c7, branch code/ir-1532-kernel-version), scoped to `git diff origin/main...HEAD` (4 files, +508/-2). Methods: code-review with the rust-review lane folded in. Reviewed read-only in /home/peter/dev/worktrees/ir-1532-code against FR-406 and TC-954 (merged in QE #156).

Reviewer runs, focused only, through `locked-build.sh` with a throwaway target dir: `cargo test --features test-support --test ir1532_kernel_version -- --include-ignored` passed 4 of 4; `cargo test --test ir1532_kernel_version` (default features) passed 1 of 1; `make build-no-std` (`cargo build --locked --target thumbv7em-none-eabi`) succeeded; `make fmt-check` passed; `cargo clippy --features test-support --test ir1532_kernel_version --lib -- -D warnings` passed. `cargo deny` not run: the diff changes no dependency. No `make ci`, no full suite, no Kani.

Recorded strings are real observed output: the `#[ignore]`d `print_observed_grid` output, run by the reviewer, is identical line for line to `golden()`.

Mutation controls, each in a throwaway detached worktree, one focused run, then removed:

- **TC-954 mutant 3**, modulo returns the dividend's sign (`DivisionProfile::Euclidean.apply` replaced with `Truncating` in `euclidean_remainder`, src/division.rs:245). KILLED: `integer.modulo outcome recorded: Completed(Integer(1)) observed: Completed(Integer(-1))`.
- **TC-954 mutant 2**, a division charges one atom fewer (`charge_operands` `ValueOccurrences, 2` to `1`, src/division.rs:126). KILLED at six cases (`integer.divide.*`, `integer.modulo*`, `native-integer.divide` charges).
- **Reviewer mutant R1**, integer arithmetic retains two result units instead of one (`IntegerArithmeticResultRetain` `.results(1)` to `.results(2)`, src/numeric.rs:289). **SURVIVED**: the grid passed. See FND-001.

## Verdict

**FAIL (one high, two low).** Not mergeable until FND-001 is fixed.

Checked and found correct:

- **The constant.** `pub const KERNEL_VERSION: u32 = 1;` at the crate root, a plain integer literal with no `cfg`, so it is present in the `#![no_std]` build (thumbv7em build succeeds). The integration test uses it in a `const` item, in default and `test-support` builds (FR-406-AC-1).
- **No ceremony.** No `build.rs`, no `env!`, no hash, digest, manifest, ledger or recorded-version check. The golden strings are behaviour expectations, not version tracking. The `#[ignore]`d `print_observed_grid` helper prints `observe()` as Rust source; it is a re-record tool reading the same single grid definition, not a second source of truth.
- **Doc comment against Behavior 3 (FR-406-AC-4).** It names every bump class: a result (returned value, flag, refusal or its cause, outcome kind), a charge (point, order, limit kind, amount), and the verification-cost class (cost or size of the Kani verification problem for harnesses running `Value`, `Integer`, `Meter`, `build_typed_prestate` or collection types, including a semantics-preserving carrier change). It names the no-bump classes (rename, comment, unused new operation or type, refactor with no effect on results or verification cost), states the value never decreases (Behavior 4), and states the golden test's limits honestly (Behavior 6).
- **AC-5 by inspection.** This change introduces the constant at 1 and alters no operation outcome or charge; nothing lowers it.
- **Grid coverage against Behavior 5.** Every listed operation is present: integer add, subtract, multiply, negate; non-zero truncating and floor divide and modulo; divide and modulo by zero; the six comparisons (`ComparisonOperator::ALL` over quantities 1/2 vs 1/3); decimal multiply and round at a tie; native integer add, divide and order (FR-384) and native decimal add and round at a tie (FR-402, FR-403); `build_typed_prestate`; rational reduction (1/6 + 1/3 = 1/2); IEEE add with inexact and overflow (`f32::MAX + f32::MAX` = +inf, flags 20 = overflow|inexact) plus a tie-to-even case for TC mutant 1 (1.0 + 2^-24 stays 0x3f800000); text compare at a prefix; quantity unit mismatch; collection past its bound (`form_collection`, the kernel has no separate insert operation); identity construction; a meter denied at the exact limit (operands admitted at exactly 4 of 4 bits, the next charge denied). Recorded values are arithmetically right where checked.
- **Comparison function.** `differences` reports outcome and charge mismatches separately, missing or renamed cases, and extra observed cases. The AC-3 test exercises both alterations and asserts the named part.
- **Status lines.** FR-406 and TC-954 Status lines are accurate: AC-1..3 test-backed, AC-4..5 inspection; the two TC mutants re-run here were killed as claimed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The golden grid does not compare `work_units` or `result_units` charges, so FR-406 Behavior 5's "the test fails when any outcome or charge in the grid differs from the record" is false for two of the ten limit kinds. `render_charge` renders `AdmittedCharge`, which records only the eight semantic-size requests (accounting.rs:875 returns early for `WorkUnits`/`ResultUnits`). Reproduced: changing `IntegerArithmeticResultRetain` from `.results(1)` to `.results(2)` (src/numeric.rs:289) leaves `golden_grid_matches_the_record` passing, although it changes a charge amount in five grid cases. `work_units` is also the charge amount most tied to the verification-cost class CG rows measure. Fix: record per case the per-event `work_units` and `result_units` (from `Meter::enable_charge_capture` / `ChargeEvent`) or at least `meter.consumed(kind)` for every `LimitKind::ALL` member, re-record, and confirm the R1 mutant is killed. | tests/ir1532_kernel_version.rs:87-124, src/accounting.rs:875 |
| FND-002 | low | `let _ = LimitKind::ALL;` is a no-op statement whose only effect is to keep the otherwise unused `LimitKind` import from warning. Hiding an unused-import warning with dead code; remove both, or use `LimitKind::ALL` for real as FND-001's fix suggests. | tests/ir1532_kernel_version.rs:31, tests/ir1532_kernel_version.rs:370 |
| FND-003 | low | `render_charge` lists `AdmittedCharge`'s size fields by hand. A size field added to `AdmittedCharge` later is silently left out of the grid, the same blind spot as FND-001. Destructure `AdmittedCharge { point, integer_bits, .. }` exhaustively (no `..`) so a new field is a compile error in the golden test. | tests/ir1532_kernel_version.rs:96-114 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| src/lib.rs | examined | `pub const KERNEL_VERSION: u32 = 1;` and its doc comment (bump rule, no-bump classes, never decreases, golden-test limits) |
| tests/ir1532_kernel_version.rs | examined | const-context test; `golden` module: `observe`, `differences`, `golden`, `print_observed_grid`, `golden_grid_matches_the_record`, `altered_outcome_or_charge_is_reported` |
| spec/functional/FR-406-kernel-version-constant.md | examined | Status line: AC-1..3 test-backed, AC-4..5 inspection |
| spec/test-cases/TC-954-kernel-version-constant.md | examined | Status line: steps 1-3 run, three mutants applied by hand |
| FR-406-AC-1 | examined | public u32 const, crate root, default / test-support / no_std, integer literal, const context, doc comment |
| FR-406-AC-2 | examined | golden test evaluates the Behavior 5 grid with test-support and passes on unchanged source |
| FR-406-AC-3 | examined | altered expected outcome and altered expected charge each reported |
| FR-406-AC-4 | examined | doc comment names Behavior 3 classes including verification cost |
| FR-406-AC-5 | examined | bump accompanies outcome/charge change; never lowered |
| src/accounting.rs | context_only | `AdmittedCharge` fields, `LimitKind`, `ChargeEvent`, work/result unit handling |
| src/division.rs, src/numeric.rs | context_only | mutation sites |
