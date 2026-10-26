---
id: SR-16294
title: spec-review integrity review of IR-497 FR-369 status amendment
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@f01e3a97c7d9f408a64acb8a0568489d0c86ae26; spec/functional/FR-369-typed-checked-invariant-causes.md, spec/test-cases/TC-917-typed-checked-invariant-causes.md, spec/test-cases/TC-428-a-kernel-refusal-returns-its-code-and-cause.md"
review_set: subset
---

## Summary

Ticket: IR-497. Spec review of the Status edits in PR #85 at f01e3a97c7d9f408a64acb8a0568489d0c86ae26. Only the integrity sub-analysis applies: no requirement statement, AC, relationship or domain object changed, so EARS, dependency and object review were not run. `quire validate --scope .` on the three changed files reported no errors. It did report module warnings (DuplicateArchetype and an inline-data-schema note) unrelated to this diff.

## Verdict

**FAIL** — The new Status text claims a thirty-cause enum that the code does not have. Two FR-369 sentences outside the diff are now stale because the kernel cutover lands ahead of RT.

## Examined scope

- `FR-369-Status` (examined): The kernel carrier and kernel producer tests exist. `src/outcome.rs` exports the target thirty-cause enum without `CallDepthExceeded`, and the tagged `tests/checked_invariant.rs` controls cover the target shape.
- `TC-917-Status` (examined): `tests/checked_invariant.rs` exhaustively tests the target thirty-cause enum, checks that `CallDepthExceeded` cannot be constructed, and tests the kernel carrier behavior.
- `TC-428-Status` (examined): The current `tests/checked_invariant.rs` covers the target thirty typed causes and their `None` mapping. Its compile-fail control rejects the removed `CallDepthExceeded` variant.
- `FR-369-Description` (context_only): ... no call-depth ceiling or `CallDepthExceeded` checked-invariant cause decides an outcome.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The FR-369, TC-917 and TC-428 Status sections claim a thirty-cause enum, but the code has 31 variants; `NativeIntegerCapacityBreach` is missing from FR-369's closed tables | spec/functional/FR-369-typed-checked-invariant-causes.md:167-169 |
| FND-002 | low | Unchanged FR-369 prose is now stale: line 89 says RT retires depth stops before the thirty-cause API is implemented, and Dependencies calls this a spec-only slice with no code claims | spec/functional/FR-369-typed-checked-invariant-causes.md:88-89 |

## Dispositions

- FND-001 — fixed: FR-369 adds the native-capacity row and changes the required count to thirty-one, with TC-917 and TC-428 counts reconciled.
- FND-002 — fixed: FR-369 now states that RT must retire depth stops before pinning the kernel API and identifies downstream QSL fixture work without calling this a spec-only slice.
