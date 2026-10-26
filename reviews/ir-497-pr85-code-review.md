---
id: SR-16292
title: code-review review of IR-497 kernel CallDepthExceeded removal
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@f01e3a97c7d9f408a64acb8a0568489d0c86ae26; src/outcome.rs, tests/checked_invariant.rs, spec/functional/FR-369-typed-checked-invariant-causes.md, spec/test-cases/TC-917-typed-checked-invariant-causes.md, spec/test-cases/TC-428-a-kernel-refusal-returns-its-code-and-cause.md"
review_set: subset
---

## Summary

Ticket: IR-497. Reviewed quire-exact PR #85 at head f01e3a97c7d9f408a64acb8a0568489d0c86ae26, diff `origin/main...HEAD` only, with the rust-review lane folded in (rust-style baseline, repository `clippy.toml`, `rustfmt.toml`). The Rust change removes `CheckedInvariantCause::CallDepthExceeded`, drops it from the independent expectation match and case table, and adds a real-compiler control that `CheckedInvariantCause::CallDepthExceeded` fails with E0599.

Focused gates run: `cargo fmt --check` passed; `cargo clippy --all-targets -- -D warnings` passed; `cargo test --test checked_invariant` passed 3/3. Full gates were not run, per the brief.

## Verdict

**FAIL** — The Rust removal itself is correct and its compile-fail control can fail (it compiles successfully on `origin/main`). But the public enum still has thirty-one variants, while FR-369-AC-1 and the changed Status text claim the closed set equals the thirty table rows. `NativeIntegerCapacityBreach` (added by IR-705 #78, produced at `src/native_integer.rs:386`) has no FR-369 table row, so AC-1 is not met at this head.

## Examined scope

- `FR-369-AC-1` (examined): The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and four typed `IllTypedCause` payloads equal the thirty table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile.
- `FR-369-AC-5` (examined): For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are unchanged.
- `src/outcome.rs` (examined): `CheckedInvariantCause` enum, with the `CallDepthExceeded` variant and doc removed.
- `tests/checked_invariant.rs` (examined): `expected_name`, the case table, and `rust_compiler_rejects_unit_constructor_removed_cause_and_an_unhandled_new_cause`.
- `src/native_integer.rs` (context_only): `NativeIntegerCapacityBreach` producer at line 386.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The enum exports 31 variants, but FR-369-AC-1 requires the closed set to equal the 30 table rows; `NativeIntegerCapacityBreach` has no FR-369 row | src/outcome.rs:120-122 |

## Dispositions

- FND-001 — fixed: FR-369 now includes the genuine `NativeIntegerCapacityBreach` kernel producer and requires the full thirty-one-cause set; the exhaustive carrier test asserts that count. The enum retains all genuine causes and excludes `CallDepthExceeded`.
