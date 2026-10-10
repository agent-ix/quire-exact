---
id: SR-4864
title: spec-review/evidence review of IR-711/IR-712 cause table
type: SpecReview
analysis: evidence
scope: agent-ix/quire-exact@4fc8b25c7c0080d591c78e6cfb00dbba9e440cb6; spec/functional/FR-090,
  FR-369; spec/test-cases/TC-428, TC-917
review_set: subset
---

## Summary

Quoin advice for the new FR-369-AC-9 has authored Test with no mismatch; unit testing and BDD examples are recommended by its example property shape. The criterion is testable through QSV producer paths and captured IllTypedCause values; its unchanged charge and non-result clauses require assertions in QSV-owned tests. TC-917 and TC-428 correctly cover only kernel/public-carrier behavior.

## Verdict

**PASS** — no findings in the frozen four-file diff.

## Examined Units

- `FR-369-Description` (examined): When the exact kernel, the checked-expression residue in `quire-contract-runtime`, the shared `quire-semantic-value` leaf, or a generated CG oracle detects a checked-program invariant failure, the shared `quire-exact::Refusal` SHALL carry the failure's closed, typed cause. A caller SHALL distinguish a depleted call-depth budget from a re-entrant meter borrow, and each other condition in the tables below, by matching that cause without inspecting a message or guessing from a location. These failures remain internal faults, not catalog refusals or proof results.
- `FR-369-Behavior` (examined): The public `CheckedInvariantCause` SHALL be a closed enum of the thirty-one variants in the two tables. `ScheduledComparisonRefused`, `EqualityQuantityConversionRejected`, `GeneratedTypeCheckRejected` and `GeneratedEqualityCheckRejected` each SHALL carry `{ cause: IllTypedCause }`; the other twenty-seven variants are unit variants. The existing typed refusal returned by a scheduled comparator, an equality quantity conversion, or CG's RT type/equality check SHALL be retained in that payload without string conversion. `IllTypedCause` is already owned and exported by `quire-exact::comparison`, wh
- `FR-369-AC-1` (examined): The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and four typed `IllTypedCause` payloads equal the thirty-one table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile.
- `FR-369-AC-4` (examined): Every RT collection, composite, equality and quantity producer returns its table cause, including all six `operand_value` failures and the original kernel-owned `IllTypedCause` when a scheduled comparison or checked equality quantity conversion refuses; no site flattens a comparison or conversion error to a message.
- `FR-369-AC-8` (examined): The public carrier contains no QSL model/checker/catalog type or dependency on RT or QSV, RT and QSV own no second refusal enum or compatibility shim, and an exhaustive source inventory finds every kernel, RT, QSV and CG production checked-invariant constructor assigned exactly one cause from the two tables.
- `FR-369-AC-9` (examined): QSV's deferred field or tuple result uses `DeferredResultNotAdmitted`; missing enum variant and unresolved unit use their distinct rows; schedule mismatch and comparator refusal retain their existing rows; each equality operand failure uses its exact table row; and scale-zero integer placement uses `ExpectedIntegerPlacement`. The comparator and quantity conversion retain their original `IllTypedCause` payload, while prior non-result and charge behavior is unchanged.
- `FR-090-Description` (examined): The `quire-exact` kernel `Refusal` and `Undefined` enums SHALL name only causes the kernel itself raises, except for the closed `CheckedInvariantCause` transport shared with `quire-contract-runtime`'s extracted exact checking residue, `quire-semantic-value`'s shared value operations and CG's generated checked oracles under [FR-369](./FR-369-typed-checked-invariant-causes.md). A cause whose meaning depends on a QSL checker, model or evaluator remains that caller's and stays out of the kernel's closed sets. QSL's existing direct construction of the unit `CheckedInvariant` SHALL migrate under QSL
- `FR-090-Behavior-1` (examined): 1. **Closed kernel sets.** `Refusal` has no variant that carries a wrong-snapshot cause, and `Undefined` has no precondition-false variant and no absent-key variant. The shared checked-invariant carrier accepts only FR-369's exact/kernel, RT residue, QSV shared-value and generated-oracle causes, not a general caller code or message.
- `TC-917-Step-1` (examined): 1. Exhaustively match `CheckedInvariantCause` against thirty-one independently
   named expectations, including `ScheduledComparisonRefused`,
   `EqualityQuantityConversionRejected`, `GeneratedTypeCheckRejected` and
   `GeneratedEqualityCheckRejected` with
   concrete kernel-owned `IllTypedCause` payloads.
   Compile a use of the enum as `Copy`, `Clone`,
   `Debug`, `Eq`, `Hash` and `PartialEq`, and a `Refusal` as `Clone`, `Debug`,
   `Eq` and `PartialEq`. A source/compile check rejects construction of the
   former unit `Refusal::CheckedInvariant` and an unhandled new cause variant.
- `TC-917-Step-5` (examined): 5. For each of the thirty-one public cause variants, construct the typed
   `CheckedInvariant` and assert both catalog methods return `None` and that
   refusals with different causes compare unequal. Re-run FR-096's ordinary
   refusal table unchanged so the new path cannot erase a catalog code.
- `TC-428-Step-2` (examined): 2. Build a `CheckedInvariant` with each of FR-369's thirty-one typed causes and
   read `Refusal::code()` and `Refusal::cause()`.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
