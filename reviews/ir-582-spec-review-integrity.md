---
id: SR-007
title: "Spec review of quire-exact PR #3 (IR-582: kernel inspections moved from QSL)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@0edd08bfde39e76db368b2435f069d1a3834eae8; spec/functional/FR-089-kernel-refuses-a-population-pair.md, spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md, spec/test-cases/TC-292-*.md, TC-388-*.md, TC-407-*.md, TC-408-*.md"
review_set: subset
---

## Summary

Ticket: IR-582. This PR receives the inspections QSL #632 removes: FR-089-AC-2
with TC-292, and a new FR-090 with AC-7, AC-11 and AC-12 and TC-388, TC-407 and
TC-408.

- **No collision.** quire-exact main holds FR-088, 089, 096, 097 and 262 and
  TC-297, 409, 411, 428, 441 and 735. quire-exact#2 (8137134a) adds FR-357 and
  TC-905, edits FR-096, and allocates SR-004..006. None of these ids clash
  with FR-089-AC-2, FR-090, TC-292, TC-388, TC-407 or TC-408. This SR takes
  SR-007 to stay clear of #2.
- **Each step is honest at this head.**
  - `src/value.rs:310` is `Population(PopulationId)`.
  - `[dependencies]` are exactly `num-bigint`, `num-integer`, `num-traits`,
    `thiserror` and `unicode-normalization`, which matches TC-292 step 2.
  - `src/outcome.rs` `Refusal` holds `InexactDecimal` .. `CheckedInvariant`
    and no wrong-snapshot payload. `Undefined` holds `DivisionByZero`,
    `IeeeNotFinite`, `EmptyReduction`, `NoneValue` and `SumOutOfDomain`, with
    no `PreconditionFalse` and no `AbsentKey`.
  - quire-exact#2 renames `DivisionPairOutOfDomain` to `DivisionOutOfDomain`
    and adds no caller cause, so the claims still hold after it merges.
- **Each AC states only the kernel half** and keeps QSL's id. That is the same
  pattern as FR-089-AC-6/TC-297.
- `quire validate --scope . 'spec/**/*.md'` passes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The Expected Results read "finds no a variant", "finds no a `PreconditionFalse` variant" and "finds no an `AbsentKey` variant" | spec/test-cases/TC-388-wrong-anchor-reaches-the-caller-as-a-coded-refusal.md:23; TC-407:23; TC-408:23 |
| FND-002 | low | The TC-388, TC-407 and TC-408 file names keep QSL's caller-behaviour slugs (`wrong-anchor-reaches-the-caller-as-a-coded-refusal`, `precondition-false-is-a-family-owned-undefined-result`, `absent-lookup-key-is-a-family-owned-undefined-result`). In this repo they are kernel-enum inspections, and their titles already say so | spec/test-cases/TC-388-wrong-anchor-reaches-the-caller-as-a-coded-refusal.md:1 |

## Verdict

Two low findings, no medium or high. Ids are clean and every inspection step
is true of the code. Mergeable once the wording fixes land, or accept them.
QSL #632 relies on this PR for TC-292 and the FR-090 kernel clauses, so merge
it before or with #632.

## Dispositions

Round 1, reviewed at d85256e9262bccb3002030f9c192e3ee2e37fa84.

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | d85256e9: "finds no variant", "finds no `PreconditionFalse` variant", "finds no `AbsentKey` variant" |
| FND-002 | fixed | d85256e9: files renamed to TC-388-kernel-refusal-has-no-wrong-snapshot-variant.md, TC-407-kernel-undefined-has-no-precondition-false-variant.md, TC-408-kernel-undefined-has-no-absent-key-variant.md; no live reference to the old names remains |
