---
id: SR-16297
title: "Spec review (base) — IR-497 FR-369 PackageReentry row and thirty-two-cause counts"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact@90954332cd146c4308a9844d25b1e5c6de0aea0e; spec/functional/FR-369-typed-checked-invariant-causes.md, spec/test-cases/TC-917-typed-checked-invariant-causes.md, spec/test-cases/TC-428-a-kernel-refusal-returns-its-code-and-cause.md"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-exact/FR-369", type: reviews }
---

## Summary

Ticket: IR-497. This is a base-checklist review of the spec edits in quire-exact PR #86 at 90954332cd146c4308a9844d25b1e5c6de0aea0e. The edits change one Description sentence, add the FR-369 `PackageReentry` table row, change the Behavior counts (thirty-two / twenty-eight), make the AC-1 count "thirty-two", and make FR-369/TC-917/TC-428 Status and procedure counts "thirty-two". `quire validate --scope .` passed on the three changed files: 3/3 grammar-clean, with no errors. The only messages were module-level warnings (DuplicateArchetype, inline-data-schema) unrelated to the diff. The review set is subset: base plus the integrity, ears-conformance and failure-domain lenses, each in its own artifact.

## Verdict

**PASS**: no base-checklist defect in the diff.

- **IDs.** ID formats are unchanged and valid (FR-369-AC-1/2/5/6/8 and TC-917/TC-428). No duplicate IDs.
- **No AC narrowed.** AC-1's only edit is "thirty-one" to "thirty-two", and AC-2, AC-5, AC-6 and AC-8 are byte-identical to `origin/main`. The Verification cells still name a method only.
- **Counts consistent.** No "thirty-one" or "twenty-seven" text is left anywhere in `spec/`, `src/` or `tests/`.
- **CG assignments unchanged.** The second (CG) table and its nine-assignment paragraph are unchanged. CG FR-042 on quire-contract-codegen `origin/main` 4788b467 still names "the nine CG-owned causes in the second cause table", and that table does not include `PackageReentry`.
- **No copy.** No cross-repo copy. The new row names the RT site by function and does not restate RT's requirement.
- **Pins.** The diff adds no hash, digest, pin or tracking record. FR-369's existing measured-revision SHAs (lines 82-86) are outside this diff and are informational provenance. Under the user's global rule this is ceremony, so it is not raised here.

## Examined scope

- `FR-369-Description` (examined): A caller SHALL distinguish each condition in the tables below, including a re-entrant meter borrow and a nested entry into an already-active checked package, by matching that cause without inspecting a message or guessing from a location.
- `FR-369-Behavior` (examined): The public `CheckedInvariantCause` SHALL be a closed enum of the thirty-two variants in the two tables. `ScheduledComparisonRefused`, `EqualityQuantityConversionRejected`, `GeneratedTypeCheckRejected` and `GeneratedEqualityCheckRejected` each SHALL carry `{ cause: IllTypedCause }`; the other twenty-eight variants are unit variants.
- `FR-369-AC-1` (examined): The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and four typed `IllTypedCause` payloads equal the thirty-two table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile.
- `FR-369-AC-2` (examined, unchanged): Every kernel producer in collection, value, division and equality returns exactly its table cause; two population values return `PopulationPair`, a population paired with another kind returns `ValueKindMismatch`, and unlike collection kinds return `CollectionKindMismatch`.
- `FR-369-AC-5` (examined, unchanged): For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are unchanged.
- `FR-369-AC-6` (examined, unchanged): Every received typed cause follows QSL's existing internal-fault path.
- `FR-369-AC-8` (examined, unchanged): The public carrier contains no QSL model/checker/catalog type or dependency on RT or QSV, RT and QSV own no second refusal enum or compatibility shim, and an exhaustive source inventory finds every kernel, RT, QSV and CG production checked-invariant constructor assigned exactly one cause from the two tables.
- `TC-917` steps 1 and 5 and Status (examined): thirty-two independently named expectations; thirty-two public cause variants.
- `TC-428` step 2 and Status (examined): each of FR-369's thirty-two typed causes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
