---
id: SR-16293
title: gap-analysis review of IR-497 kernel CallDepthExceeded removal
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@f01e3a97c7d9f408a64acb8a0568489d0c86ae26; src/outcome.rs, tests/checked_invariant.rs, spec/functional/FR-369-typed-checked-invariant-causes.md"
review_set: subset
---

## Summary

Ticket: IR-497. Planless gap analysis of PR #85 at f01e3a97c7d9f408a64acb8a0568489d0c86ae26 for FR-369-AC-1. Plan completion: not assessed. `quoin matrix --repo .` reports FR-369-AC-1, AC-2 and AC-5 as `tagged`, with no run evidence bound. The binding `tests/checked_invariant.rs` to FR-369-AC-1 (three tests) is the right binding. No stubs, no code without an owning requirement, and no coverage inflation were found in the diff.

Downstream consumers of the removed variant, measured on each repo's `origin/main` (for merge sequencing only):
- quire-contract-runtime main 48a11b1 has three producers (`src/exact/expression.rs:804,866,935`) and three test uses. RT draft PR #161 (head 240703a) contains no `CallDepthExceeded`. The RT lock pins quire-exact 1c6cbcb, so RT main breaks only when its lock is bumped before #161 lands.
- quire-spec-language main e08cceeb8: `qsl-semantics/src/check/family.rs:1914,1957` (test/test-support fixture) still names `C::CallDepthExceeded`. No open QSL PR removes it. The QSL lock pins f6d62c0.
- quire-contract-codegen 9196cbe4 and quire-semantic-value 0a365db have no references.

## Verdict

**FAIL** — FR-369-AC-1 is not discharged: the tagged tests pass against a 31-variant enum, so they do not show the closed set equals the 30 table rows.

## Examined scope

- `FR-369-AC-1` (examined): The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and four typed `IllTypedCause` payloads equal the thirty table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile.
- `FR-369-Behavior` (context_only): The public `CheckedInvariantCause` SHALL be a closed enum of the thirty variants in the two tables.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-369-AC-1 is tagged but not discharged: the tests enumerate 31 causes, including `NativeIntegerCapacityBreach`, which no FR-369 table row lists | tests/checked_invariant.rs:9 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Round 1, head 8340a6f674da35da80eeeea93dd861648d25c2d6: AC-1 now names the thirty-one table rows, the table set equals the enum set, and the tagged test counts 31 cases and passes (3/3). The QSL consumer is recorded in FR-369 Dependencies. In Linear, QSL-1111 `blocks` IR-497 and matches the measured `family.rs:1914,1957` references. |
