---
id: SR-16299
title: "Integrity — IR-497 FR-369 PackageReentry row, counts and cross-document consistency"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@90954332cd146c4308a9844d25b1e5c6de0aea0e; spec/functional/FR-369-typed-checked-invariant-causes.md, spec/test-cases/TC-917-typed-checked-invariant-causes.md, spec/test-cases/TC-428-a-kernel-refusal-returns-its-code-and-cause.md, src/outcome.rs, tests/checked_invariant.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-exact/FR-369", type: reviews }
---

## Summary

Ticket: IR-497. This is a completeness and consistency check of the PR #86 spec edits against each other, the code, and CG FR-042. The FR-369 table rows (23 + 9 = 32), the enum (32 variants), `expected_name` (32 arms), the case table (32 rows) and every "thirty-two" count in FR-369, TC-917 and TC-428 all agree. FR-369 states four payload variants and twenty-eight unit variants, and both the enum and the tables have exactly that. The CG table and its nine assignments are unchanged, and they still agree with CG FR-042 on its `origin/main`. One low finding: the new row states a current RT trigger that exists only on the sibling RT PR, and it is not marked as pending the way the trigger-less CG rows are.

## Verdict

**CONDITIONAL**: there is one low finding. Verified clean:

- **One reading.** The row and the Description describe the same condition one way only: re-entry of an already-active package, checked before the nested charge. The row explicitly sets it apart from `MeterBorrowConflict` and `GeneratedUnexpectedOutcome`.
- **Nothing duplicated.** No other row or requirement restates the condition.
- **Closed set holds.** The FR-369 rule "a constructor added later SHALL ... extend this closed table and its tests before merging" is satisfied for the RT re-entry producer: the table and tests are extended here, before RT PR #161 can pin the variant.
- **ACs unchanged.** The AC set is unchanged apart from AC-1's count.

## Examined scope

- `FR-369-Behavior` (examined): The public `CheckedInvariantCause` SHALL be a closed enum of the thirty-two variants in the two tables. … the other twenty-eight variants are unit variants.
- `FR-369` PackageReentry row (examined): RT `CheckedPackage::call` or `CheckedPackage::evaluate` enters an already-active package from a host body before a nested charge: `src/exact/expression.rs`, `CheckedPackage::begin`. This is distinct from a shared meter borrow and a generated outcome adapter mismatch.
- `FR-369-AC-1` (examined): The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and four typed `IllTypedCause` payloads equal the thirty-two table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile.
- `FR-369-AC-8` (examined): … an exhaustive source inventory finds every kernel, RT, QSV and CG production checked-invariant constructor assigned exactly one cause from the two tables.
- `FR-369` CG table and nine-assignment paragraph (examined, unchanged): Completion and verification of all nine assignments under CG FR-042-AC-1 and TC-057 remain PLANNED in IR-709, including the two absent assignments.
- `TC-917` / `TC-428` counts and Status (examined).
- CG `spec/oracle/functional/FR-042-generated-checked-invariant-causes.md` at quire-contract-codegen 4788b467 (context_only): The code generator SHALL use the nine CG-owned causes in the second cause table of quire-exact FR-369.
- RT `src/exact/expression.rs` on main 48a11b1 and on PR #161 (context_only).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The `PackageReentry` row states a current RT trigger at `CheckedPackage::begin` under a header reading "on the measured main revisions", but that function exists only on unmerged RT PR #161. RT main 48a11b1 has no re-entry refusal and only bounds re-entrant depth with `CallDepthExceeded` (lines 804, 866, 935), and unlike the two trigger-less CG rows the row is not marked pending | spec/functional/FR-369-typed-checked-invariant-causes.md:48 |

FND-001 detail: the row's status is the issue, not a stale SHA. FR-369 marks causes with no current producer as `PLANNED` (the `GeneratedUnexpectedOutcome` and `GeneratedEnvironmentRejected` rows), and it records the later kernel producer `NativeIntegerCapacityBreach` explicitly (lines 87-88). An FR-369-AC-8 inspector reading RT main would not find this row's producer. Lines 91-93 only say RT must retire depth-stop producers before pinning. Options: add "pending RT FR-273 cutover (quire-contract-runtime#161)" to the row, or add one sentence beside the `NativeIntegerCapacityBreach` note.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Disposition round 1 at 7ae6cd74ccbee2b5b7508c260dc14fabe3bd89fa: the FR-369 `PackageReentry` row (line 48) now opens "PLANNED on the measured RT main" and attributes the `CheckedPackage::begin` producer to RT #161. It no longer claims a current trigger, which matches the PLANNED convention of the CG rows. No regression: the counts (32 variants, 28 unit), the distinction from `MeterBorrowConflict`/`GeneratedUnexpectedOutcome`, and TC-917/TC-428 are unchanged. |
