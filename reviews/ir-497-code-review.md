---
id: SR-16295
title: "Code review — IR-497 PackageReentry checked-invariant cause (quire-exact PR #86)"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@90954332cd146c4308a9844d25b1e5c6de0aea0e; src/outcome.rs, tests/checked_invariant.rs, spec/functional/FR-369-typed-checked-invariant-causes.md, spec/test-cases/TC-917-typed-checked-invariant-causes.md, spec/test-cases/TC-428-a-kernel-refusal-returns-its-code-and-cause.md"
review_set: subset
---

## Summary

Ticket: IR-497. This review covers quire-exact PR #86 at frozen head 90954332cd146c4308a9844d25b1e5c6de0aea0e, diff `origin/main...HEAD` (one commit over main e603fc02d51238fe8a6ec540dff46a34861d21d6). The rust-review lane is folded in, using the rust-style baseline, repo `CLAUDE.md`/`AGENTS.md`, `clippy.toml`, `rustfmt.toml` and `Cargo.toml` lints. The change adds a unit variant `CheckedInvariantCause::PackageReentry` after `MeterBorrowConflict`. It adds the variant to the independent `expected_name` match and the 32-entry case table, and raises the count assertion from 31 to 32. The spec counts in FR-369, TC-917 and TC-428 change to match. The new variant is correct and distinct, and the counts agree everywhere. One low finding: the test's case table would not notice if a unit-variant case were replaced by a second instance of a payload variant.

## Verdict

**CONDITIONAL**: there is only one low test-intent finding. Verified clean:

- **Distinct cause.** `PackageReentry` is a separate unit variant with its own doc comment. It is distinct from `MeterBorrowConflict` (a failed `RefCell` meter borrow) and `GeneratedUnexpectedOutcome` (a CG outcome-adapter arm). The RT consumer on sibling PR quire-contract-runtime#161 (context only) emits `PackageReentry` only when `CheckedPackage::begin` finds the package `active` flag already set (`src/exact/expression.rs:712,745`). It emits `MeterBorrowConflict` separately at line 788.
- **Counts.** The FR-369 tables have 23 + 9 = 32 rows, which equal the 32 enum variants. Exactly four variants carry a payload (`ScheduledComparisonRefused`, `EqualityQuantityConversionRejected`, `GeneratedTypeCheckRejected`, `GeneratedEqualityCheckRejected`), leaving 28 unit variants.
- **Test can fail.** The exhaustive `expected_name` match stops compiling if a variant is missing or extra. `cases.len() == 32` catches a dropped case, and the pairwise-equality loop catches an exact duplicate case.
- **Derives and visibility.** No `#[non_exhaustive]` is added; FR-369 deliberately specifies a closed enum. The derives (`Clone, Copy, Debug, Eq, Hash, PartialEq`) are unchanged.
- **Docs and lints.** The new public item has a `///` doc that matches the FR-369 row. Nothing is `unsafe`, and no lint, CI or gate is narrowed: the diff has no workflow, `clippy.toml`, `deny.toml` or `Cargo.toml` change.
- **No duplication or ceremony.** No vendoring or cross-repo copy: the only `include_str!` reads this repo's own `src/outcome.rs` and test file, and it predates this PR. No unrequested ceremony was added.
- **Downstream.** On `origin/main`, quire-spec-language a2e4f192e, quire-contract-codegen 4788b467, quire-semantic-value 0a365db and quire-contract-runtime 48a11b1 have no arm-per-variant `CheckedInvariantCause::X =>` match that adding the variant would break (grep heuristic).

Gates (`cargo fmt --check`, `clippy -D warnings`, `cargo test`, `cargo deny`, `make ci`) were **not run**. Per the brief this is a read-only review with no builds or tests, and those gates are left to the final aggregate gate.

## Assurance Context

No `AssuranceProfile` exists under `spec/` (the only `spec/assurance/` documents are `ArchitectureDescription` AD-002 and AD-003, and neither covers this carrier). The ordinary rubric applies.

## Examined scope

- `FR-369-AC-1` (examined): The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and four typed `IllTypedCause` payloads equal the thirty-two table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile.
- `FR-369-AC-5` (examined): For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are unchanged.
- `FR-096-AC-8` (context_only): every FR-369 `CheckedInvariant { cause }` returns neither code nor cause.
- `src/outcome.rs` `CheckedInvariantCause` (examined): new `PackageReentry` variant and doc at lines 143-144.
- `tests/checked_invariant.rs` (examined): `expected_name` arm (line 20), case row (line 88), `cases.len() == 32` (line 178), and the compile-control test.
- quire-contract-runtime PR #161 `src/exact/expression.rs` (context_only): `CheckedPackage::begin`/`call`/`evaluate` producers.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The 32-row case table cannot detect a unit cause replaced by a second instance of a payload variant: a variant could be dropped while length, name and pairwise-`Eq` checks still pass, because names are not checked for uniqueness | tests/checked_invariant.rs:178-192 |

FND-001 detail: the test checks that `cases.len() == 32`, that each row's `expected_name` matches, and that rows are pairwise unequal under `Eq`. Suppose the `(PackageReentry, "package-reentry")` row (or any unit row) were replaced by `(ScheduledComparisonRefused { cause: IllTypedCause::DistinctUnits }, "scheduled-comparison")`. All three checks would still pass, because the two `ScheduledComparisonRefused` rows carry different `IllTypedCause` payloads. The dropped variant would then skip the per-cause AC-5 `code()`/`cause()` assertions. The compile-time `expected_name` match still forces the enum itself to match exactly, so this weakens AC-5's "every cause" coverage, not AC-1's variant-set check. The pattern predates this PR, but the PR edits the table. One fix is to assert that the 32 names are distinct (for example, collect them into a set and check its length).

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Disposition round 1 at 7ae6cd74ccbee2b5b7508c260dc14fabe3bd89fa: `tests/checked_invariant.rs:179-184` now inserts each row's name into a `BTreeSet` and asserts insertion succeeds. The 32 `expected_name` arms map to 32 distinct strings, so with `cases.len() == 32`, unique names and `expected_name(*cause) == *name`, every variant must appear in exactly one row; the unit-replaced-by-payload substitution now fails. No regression: the `std::` path matches the file's existing fully qualified idiom, and the vertical `assert!` layout matches rustfmt's default `fn_call_width`. |
