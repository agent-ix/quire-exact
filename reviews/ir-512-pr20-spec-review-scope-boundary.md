---
id: SR-5104
title: spec-review/scope-boundary review of IR-512 exact cause amendment
type: SpecReview
analysis: scope-boundary
scope: agent-ix/quire-exact@8e2ad31d15e9ac946fad9187b1cb63c01f061967; spec/functional/FR-369-typed-checked-invariant-causes.md,
  spec/test-cases/TC-917-typed-checked-invariant-causes.md, spec/test-cases/TC-428-a-kernel-refusal-returns-its-code-and-cause.md
review_set: subset
---

## Summary

Ticket: IR-512. Reviewed PR #20 at 8e2ad31d15e9ac946fad9187b1cb63c01f061967. Kernel carrier ownership and RT, QSV, CG, and QSL producer responsibilities remain allocated to their owning repositories.

## Verdict

**PASS** — No defect found in this method on the frozen diff. Current Rust behavior remains a staged 31-cause implementation.

## Examined scope

- `FR-369-AC-1` (examined): The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and four typed `IllTypedCause` payloads equal the thirty table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile.
- `FR-369-AC-2` (examined): Every kernel producer in collection, value, division and equality returns exactly its table cause; two population values return `PopulationPair`, a population paired with another kind returns `ValueKindMismatch`, and unlike collection kinds return `CollectionKindMismatch`.
- `FR-369-AC-3` (examined): Every RT checked-package/function invariant producer returns its table cause: unknown name is `UnknownCheckedFunction`, a foreign expression is `ForeignCheckedExpression`, and each meter reborrow is `MeterBorrowConflict`. Runtime-managed calls use explicit frames and `work_units` fuel under RT FR-273; the final carrier has no `CallDepthExceeded` variant, and fuel exhaustion is `Incomplete` with `limit_kind: work_units`, never `CheckedInvariant`.
- `FR-369-AC-4` (examined): Every RT collection, composite, equality and quantity producer returns its table cause, including all six `operand_value` failures and the original kernel-owned `IllTypedCause` when a scheduled comparison or checked equality quantity conversion refuses; no site flattens a comparison or conversion error to a message.
- `FR-369-AC-5` (examined): For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are unchanged.
- `FR-369-AC-6` (examined): Every received typed cause follows QSL's existing internal-fault path: no refusal record or `Evaluation`, `runtime_invariant`/`S6a`/`checked-program-invariant` at S6a, `CallFailure::Fault` at the public call, and `Failed` with unavailable basis for an internal replay fault. It never becomes an ordinary replay refusal, `Inconclusive`, `Incomplete(Cancelled)`, a proved result or a certification rejection; an unknown received cause retains fault provenance.
- `FR-369-AC-7` (examined): CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default.
- `FR-369-AC-8` (examined): The public carrier contains no QSL model/checker/catalog type or dependency on RT or QSV, RT and QSV own no second refusal enum or compatibility shim, and an exhaustive source inventory finds every kernel, RT, QSV and CG production checked-invariant constructor assigned exactly one cause from the two tables.
- `FR-369-AC-9` (examined): QSV's deferred field or tuple result uses `DeferredResultNotAdmitted`; missing enum variant and unresolved unit use their distinct rows; schedule mismatch and comparator refusal retain their existing rows; each equality operand failure uses its exact table row; and scale-zero integer placement uses `ExpectedIntegerPlacement`. The comparator and quantity conversion retain their original `IllTypedCause` payload, while prior non-result and charge behavior is unchanged.
- `TC-917` (context_only): Verify the public shared carrier and every kernel-produced cause of
- `TC-428` (context_only): Verify FR-096-AC-8: every catalogued kernel refusal returns its code and cause
- `FR-096-AC-8` (context_only): and a kernel `CheckedInvariant` returns neither.
- `QSpec FR-146` (context_only): Runtime fuel exhaustion remains incomplete.
- `QSpec FR-460` (context_only): No stage has a depth limit kind,

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
