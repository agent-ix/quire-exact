---
id: SR-16298
title: "EARS conformance — IR-497 FR-369 edited requirement statements"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-exact@90954332cd146c4308a9844d25b1e5c6de0aea0e; spec/functional/FR-369-typed-checked-invariant-causes.md"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-exact/FR-369", type: reviews }
---

## Summary

Ticket: IR-497. Two FR-369 requirement statements changed in quire-exact PR #86 (Description and Behavior). The engine check (`quire validate --scope . <FR-369, TC-917, TC-428> --summary`, quire 0.36.2) reports 3/3 documents grammar-clean with 0 grammar findings. A semantic read finds both edited statements clean.

## Verdict

**PASS**: both edited statements keep their ubiquitous `SHALL` form with a named subject ("A caller", "The public `CheckedInvariantCause`") and a concrete, checkable response. The response is either matching a typed cause, or a closed enum of exactly thirty-two variants with four payload variants and twenty-eight unit variants. The added clause "a nested entry into an already-active checked package" names a specific condition, not a vague one, and it maps one-to-one to the new `PackageReentry` row. No trigger keyword was introduced, so the When/While/If check does not apply.

## Examined scope

- `FR-369-Description` (examined): A caller SHALL distinguish each condition in the tables below, including a re-entrant meter borrow and a nested entry into an already-active checked package, by matching that cause without inspecting a message or guessing from a location.
- `FR-369-Behavior` (examined): The public `CheckedInvariantCause` SHALL be a closed enum of the thirty-two variants in the two tables. `ScheduledComparisonRefused`, `EqualityQuantityConversionRejected`, `GeneratedTypeCheckRejected` and `GeneratedEqualityCheckRejected` each SHALL carry `{ cause: IllTypedCause }`; the other twenty-eight variants are unit variants.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
