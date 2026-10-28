---
id: SR-16300
title: "Failure domain — IR-497 FR-369 PackageReentry cause"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-exact@90954332cd146c4308a9844d25b1e5c6de0aea0e; spec/functional/FR-369-typed-checked-invariant-causes.md, src/outcome.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-exact/FR-369", type: reviews }
---

## Summary

Ticket: IR-497. This applies the failure-domain checklist to the new `PackageReentry` failure mode in quire-exact PR #86. The checks are: the extension-point failure policy, identity confusion with neighbouring causes, purity, and termination. All four are clean for the kernel carrier slice.

## Verdict

**PASS**: no missing failure-domain constraint in the diff.

- **Extension point.** A host-supplied body re-entering its package is a trust-boundary extension point, and its failure policy is explicit and strict. It is a typed internal fault, never a catalog refusal, proof or `Completed` (FR-369 Description and the AC-5/AC-6 mappings).
- **Identity.** The row names the condition by package activity, not by meter or adapter state. It explicitly sets it apart from `MeterBorrowConflict` (a failed `RefCell` borrow) and `GeneratedUnexpectedOutcome` (a CG adapter arm). In RT PR #161 (context only), the guard is checked before `charge_call`, so a re-entering call cannot be misreported as a meter borrow conflict or a work-fuel stop.
- **Purity.** Unaffected: the cause is a `Copy` unit variant with no payload.
- **Termination.** Re-entry is refused instead of recursing on the host stack. Unbounded nesting is handled by RT FR-273 explicit frames and `work_units` fuel, which FR-369 already references.

Package identity (which instances share the `active` flag) is RT's contract under RT FR-273. It is not this kernel carrier's, so its absence here is not a finding.

## Examined scope

- `FR-369-Description` (examined): A caller SHALL distinguish each condition in the tables below, including a re-entrant meter borrow and a nested entry into an already-active checked package, by matching that cause without inspecting a message or guessing from a location. These failures remain internal faults, not catalog refusals or proof results.
- `FR-369` PackageReentry row (examined): RT `CheckedPackage::call` or `CheckedPackage::evaluate` enters an already-active package from a host body before a nested charge … This is distinct from a shared meter borrow and a generated outcome adapter mismatch.
- `FR-369-AC-5` (examined): For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ.
- `src/outcome.rs` `PackageReentry` doc (examined): A checked package was entered again while one of its bodies was active.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
