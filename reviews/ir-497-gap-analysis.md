---
id: SR-16296
title: "Gap analysis — IR-497 PackageReentry checked-invariant cause (quire-exact PR #86)"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@90954332cd146c4308a9844d25b1e5c6de0aea0e; spec/functional/FR-369-typed-checked-invariant-causes.md, spec/test-cases/TC-917-typed-checked-invariant-causes.md, spec/test-cases/TC-428-a-kernel-refusal-returns-its-code-and-cause.md, src/outcome.rs, tests/checked_invariant.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-exact/FR-369", type: references }
---

## Summary

Ticket: IR-497. This is a planless gap analysis of quire-exact PR #86 at 90954332cd146c4308a9844d25b1e5c6de0aea0e, scoped to the diff `origin/main...HEAD`. Plan completion: not assessed. The new `CheckedInvariantCause::PackageReentry` variant is owned by an FR-369 table row and exercised by the FR-369-AC-1/AC-5 tagged tests. The matrix shows every FR-369 Test criterion `tagged`. No untraced behavior, stub or coverage inflation was found in the diff.

## Verdict

**PASS** (diff-scoped): the changed behavior is fully specified (a table row plus the AC-1 count), tagged, and backed by real assertions. The 32 table rows equal the 32 enum variants, and the 32 entries in both the `expected_name` match and the case table. This PASS is a repository-traceability claim for the diff only. It says nothing about planned work. Downstream producer controls (RT FR-278, CG FR-042-AC-1, QSV FR-122) are out of scope.

## Examined scope

- `FR-369-AC-1` (examined): The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and four typed `IllTypedCause` payloads equal the thirty-two table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile.
- `FR-369-AC-5` (examined): For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are unchanged.
- `FR-369-AC-2` (context_only): Every kernel producer in collection, value, division and equality returns exactly its table cause. `PackageReentry` has no kernel producer, so AC-2 is unaffected.
- `FR-369-AC-8` (context_only, Inspection): the exhaustive source inventory of production constructors across kernel, RT, QSV and CG. Its RT `PackageReentry` producer lives on RT PR #161.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No gaps found (placeholder) | - |

## Coverage

- Reconciliation: quoin matrix (quoin 0.29.1, quire 0.36.2 engine 0.50.2), cross-checked with quire matrix (quire 0.36.2).
- Plan completion: not assessed
- FR-369 criteria: tagged 3 (AC-1, AC-2, AC-5) · untagged 0 · tagged-by-ignored-test 0 · method-without-symbol 2 (AC-6, AC-8 Inspection).
- Bindings: FR-369-AC-1 and FR-369-AC-5 are bound to `tests/checked_invariant.rs::all_typed_causes_are_distinct_internal_faults_with_required_traits` (line 46), `comparison_payloads_retain_the_original_typed_cause` (line 197) and `rust_compiler_rejects_unit_constructor_removed_cause_and_an_unhandled_new_cause` (line 236). The new variant is asserted by the first and third.
- Evidence: no run evidence for all 201 criteria (the evidence store has not been fed). No tests were run, per the brief.
- Repository-wide, outside this diff and present before it: quire matrix reports 171 tagged, 11 untagged (FR-359-AC-10, FR-371-AC-6, FR-383-AC-4, FR-384-AC-5, FR-385-AC-1..AC-7), 1 tagged-by-ignored-test and 18 method-without-symbol. None is in a file this PR touches. Under the skill's repository-wide rule they would fail a full-repo audit. This diff-scoped review records them but does not raise them as findings.
- quire coverage `untracked_symbols`: none in the changed files.
- Untraced behaviors / stubs in the diff: 0 / 0.
- Semantic review: skipped. The brief infers scope without the optional question; the code-review lane did a targeted intent check of the test.
