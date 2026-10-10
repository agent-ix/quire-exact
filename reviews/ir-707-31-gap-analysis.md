---
id: SR-4881
title: "Fresh IR-707 gap-analysis review"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@0d15b10d8dbad592f5c9c22bb0879a86638641b6; src/collection.rs, src/division.rs, src/equality.rs, src/lib.rs, src/outcome.rs, src/value.rs, tests/checked_invariant.rs, reviews/ir-707-code-review.md, reviews/ir-707-gap-analysis.md; FR-369, FR-362-AC-10, FR-089-AC-6, FR-090-AC-13, FR-096-AC-8"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-exact/FR-369", type: references }
---

## Summary

Reviewed PR 13 at 0d15b10d8dbad592f5c9c22bb0879a86638641b6 against the merged FR-369/FR-090/FR-096/FR-089 contract and TC-917. The planless computed matrix has five untagged criteria; the IR-707 kernel criteria are tagged.

## Verdict

**FAIL as a repository-wide gap claim** — five criteria remain untagged in separately owned work.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The computed Test Matrix still has no tagged test for the independent scalar bit high-water and normalize-charge boundary. This is pre-existing work owned by IR-667; it remains a repository-wide evidence gap. | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:46 |
| FND-002 | high | No test in this repository tags the RT checked-package/function producer migration; RT IR-708 owns these paths. | spec/functional/FR-369-typed-checked-invariant-causes.md:150 |
| FND-003 | high | No test in this repository tags the RT collection/composite/equality/quantity producer migration; RT IR-708 owns these paths. | spec/functional/FR-369-typed-checked-invariant-causes.md:151 |
| FND-004 | high | No test in this repository tags the CG generated-oracle producer migration; CG IR-709 owns these paths. | spec/functional/FR-369-typed-checked-invariant-causes.md:154 |
| FND-005 | high | No test in this repository tags the QSV deferred/equality/quantity producer migration; QSV IR-712 owns these paths. | spec/functional/FR-369-typed-checked-invariant-causes.md:156 |

## Coverage

Plan completion: not assessed

Computed `quoin matrix --repo . --json`: 77 tagged, 5 untagged and 9 inspection method-without-symbol criteria. FR-369-AC-1/2/5 have meaningful tags; the five listed gaps are source-verified downstream or pre-existing work. No run evidence was bound. Reverse changed-code inventory found no orphan producer, stub or ignored test.

## Dispositions

Round 1 rechecked the unchanged PR head `0d15b10d8dbad592f5c9c22bb0879a86638641b6` and the current computed matrix. All five criteria remain untagged. Their implementation and evidence belong to the verified tickets below, outside this kernel carrier PR; deferral does not mark any criterion complete.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | IR-667 owns the pre-existing FR-362-AC-10 normalize-charge observation seam and runnable exact-amount binder; this does not concern IR-707's cause carrier. |
| FND-002 | deferred | IR-708 owns RT checked-package and function raise-site migration and tests after the shared kernel API lands. |
| FND-003 | deferred | IR-708 owns RT collection, composite, equality and quantity raise-site migration and tests after the shared kernel API lands. |
| FND-004 | deferred | IR-709 owns CG generated-oracle constructors and terminal handling after the kernel and RT dependencies align. |
| FND-005 | deferred | IR-712 owns QSV deferred, equality and quantity producer classification, implementation and tests against the closed kernel cause set. |
