---
id: SR-4705
title: spec-review/evidence review of IR-706 typed checked-invariant causes
type: SpecReview
analysis: evidence
scope: agent-ix/quire-exact@39554b826741a2f3b5f57fd8972184440f546902; spec/functional/FR-089-kernel-refuses-a-population-pair.md,
  spec/functional/FR-090-kernel-outcome-carries-no-caller-cause.md, spec/functional/FR-096-kernel-refusal-code-and-cause.md,
  spec/functional/FR-369-typed-checked-invariant-causes.md, spec/test-cases/TC-297-kernel-refuses-a-population-pair.md,
  spec/test-cases/TC-428-a-kernel-refusal-returns-its-code-and-cause.md, spec/test-cases/TC-917-typed-checked-invariant-causes.md
review_set: subset
---

## Summary

Test and Inspection choices fit the implementation split. Advisor mismatches for AC-6/8 are broad property-shape suggestions; static boundary and fault-path inspection can discharge these obligations. Ticket: IR-706.

## Verdict

**PASS** — No defects found at the reviewed head.

## Examined units

- `FR-089` (examined): The kernel refuses every population pair
- `FR-089-AC-2` (examined): The kernel `Value::Population` variant's payload type is `PopulationId`; no kernel source file imports a population binding or any other caller's model type to define, construct or match this variant, and the crate's `[dependencies]` name no crate that owns one.
- `FR-089-AC-6` (examined): Given any `ValueType::Population(maximum)` and any `Value::Population(population_id)`, kernel `ValueType::admits` returns false; given two `Value::Population` operands, kernel `equality::plan_pairs` returns `Err(Refusal::CheckedInvariant { cause: PopulationPair })` and kernel `key::compare_keys` returns `None`.
- `FR-090` (examined): The kernel outcome has a closed checked-failure cause boundary
- `FR-090-AC-7` (examined): `Refusal` has no variant whose payload is a wrong-snapshot cause.
- `FR-090-AC-11` (examined): `Undefined` has no `PreconditionFalse` variant.
- `FR-090-AC-12` (examined): `Undefined` has no `AbsentKey` variant.
- `FR-090-AC-13` (examined): `Refusal::CheckedInvariant` carries only FR-369's closed typed cause; it has no arbitrary caller message, QSL model/checker type, catalog code or generic caller-cause escape hatch.
- `FR-096` (examined): A kernel refusal names its code and cause
- `FR-096-AC-8` (examined): Each catalogued kernel refusal below returns the code and cause spelling named for it from `Refusal::code()` and `Refusal::cause()`, and every FR-369 `CheckedInvariant { cause }` returns neither. `InexactDecimal`: `inexact_decimal`, `nonzero-discarded-digit`. `DecimalOutOfDomain`, `ModuloOutOfDomain`, `TextLengthOutOfDomain`, `IntegerOutOfDomain`, `RationalOutOfDomain`, `IeeeRationalOutOfDomain`: `<name in snake case>`, `outside-domain`. `DivisionOutOfDomain`: `division_out_of_domain` with `quotient-outside-domain` for the quotient member or `remainder-outside-domain` for the remainder member.
- `FR-369` (examined): Carry the precise cause of a checked-invariant failure
- `FR-369-AC-1` (examined): The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and three typed `IllTypedCause` payloads equal the twenty-three table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile.
- `FR-369-AC-2` (examined): Every kernel producer in collection, value, division and equality returns exactly its table cause; two population values return `PopulationPair`, a population paired with another kind returns `ValueKindMismatch`, and unlike collection kinds return `CollectionKindMismatch`.
- `FR-369-AC-3` (examined): Every RT checked-package/function producer returns its table cause: depth exhaustion on all three entry paths is `CallDepthExceeded`, unknown name is `UnknownCheckedFunction`, a foreign expression is `ForeignCheckedExpression`, and each meter reborrow is `MeterBorrowConflict`.
- `FR-369-AC-4` (examined): Every RT collection, composite, equality and quantity producer returns its table cause, including the original kernel-owned `IllTypedCause` when a scheduled comparison refuses after checking; no site flattens a comparison error to a message.
- `FR-369-AC-5` (examined): For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are unchanged.
- `FR-369-AC-6` (examined): Every received typed cause follows QSL's existing internal-fault path: no refusal record or `Evaluation`, `runtime_invariant`/`S6a`/`checked-program-invariant` at S6a, `CallFailure::Fault` at the public call, and `Failed` with unavailable basis for an internal replay fault. It never becomes an ordinary replay refusal, `Inconclusive`, `Incomplete(Cancelled)`, a proved result or a certification rejection; an unknown received cause retains fault provenance.
- `FR-369-AC-7` (examined): CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default.
- `FR-369-AC-8` (examined): The public carrier contains no QSL model/checker/catalog type or dependency on RT, RT owns no second refusal enum or compatibility shim, and an exhaustive source inventory finds every kernel, RT and CG production checked-invariant constructor assigned exactly one cause from the two tables.
- `TC-297` (examined): Kernel admits, plan_pairs and compare_keys refuse a population pair
- `TC-428` (examined): A kernel refusal returns its code and cause
- `TC-917` (examined): Typed checked-invariant causes identify their failing condition

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
