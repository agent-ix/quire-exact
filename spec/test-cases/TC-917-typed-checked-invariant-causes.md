---
id: TC-917
title: "Typed checked-invariant causes identify their failing condition"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-369
    type: verifies
---
# TC-917: Typed checked-invariant causes identify their failing condition

## Description

Verify the public shared carrier and every kernel-produced cause of
[FR-369](../functional/FR-369-typed-checked-invariant-causes.md) without
substituting hand-built refusals for the production paths. The RT and QSV producers and
CG/QSL mapping controls are exercised in their owning repositories during the
coordinated cutover. Scope: FR-369-AC-1, FR-369-AC-2 and FR-369-AC-5.

## Test Procedure

1. Exhaustively match `CheckedInvariantCause` against thirty independently
   named expectations, including `ScheduledComparisonRefused`,
   `EqualityQuantityConversionRejected`, `GeneratedTypeCheckRejected` and
   `GeneratedEqualityCheckRejected` with
   concrete kernel-owned `IllTypedCause` payloads. Reject a
   `CallDepthExceeded` variant: RT FR-273's final explicit-frame scheduler
   has no depth-specific outcome, and denied `work_units` is `Incomplete`.
   Compile a use of the enum as `Copy`, `Clone`,
   `Debug`, `Eq`, `Hash` and `PartialEq`, and a `Refusal` as `Clone`, `Debug`,
   `Eq` and `PartialEq`. A source/compile check rejects construction of the
   former unit `Refusal::CheckedInvariant` and an unhandled new cause variant.
2. Reach kernel collection construction with an evaluated element outside its
   declared element type, and composite construction with a deferred result
   outside its declared field type. Assert distinct
   `CollectionElementNotAdmitted` and `DeferredResultNotAdmitted` causes.
3. Reach collection canonical sorting through `form_grouped` with two already
   grouped `Float` members, whose canonical key is unavailable. In the
   division module's unit seam, call the real `refused_interval` helper with
   `IntegerDomain::Mathematical`: the public `divide` path cannot produce a
   failed membership for that unbounded domain. Assert
   `CanonicalKeyUnavailable` and `BoundedDivisionExpected` at those actual
   producer sites, without claiming a reachable public `divide` refusal.
4. Give `equality::plan_pairs` two collection values with different kinds, two
   population values, a population and a non-population value, and another
   incompatible value-kind pair. Assert `CollectionKindMismatch`,
   `PopulationPair`, `ValueKindMismatch`, and `ValueKindMismatch` respectively.
   Check the pair is refused without a completed Boolean.
5. For each of the thirty public cause variants, construct the typed
   `CheckedInvariant` and assert both catalog methods return `None` and that
   refusals with different causes compare unequal. Re-run FR-096's ordinary
   refusal table unchanged so the new path cannot erase a catalog code.

## Expected Results

Every reachable kernel failure reports its exact table cause, and all
checked-invariant payloads stay internal faults with no catalog code or cause.
No test counts a manually constructed refusal as proof that a production
raise site used that cause. The RT and external mapping criteria remain
planned until their own tagged tests and inspection run.

## Status

The current `tests/checked_invariant.rs` exhaustively tests the thirty-one-cause
enum, including `CallDepthExceeded`; its target thirty-cause form awaits the
IR-497 coordinated code cutover. Current `src/outcome.rs` still exports
`CallDepthExceeded`, and IR-708's RT typed producer migration is in progress.
This spec amendment changes no Rust constructor or test and claims no target
test result.
