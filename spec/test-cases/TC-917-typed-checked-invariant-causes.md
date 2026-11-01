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

1. Exhaustively match `CheckedInvariantCause` against thirty-one independently
   named expectations, including `ScheduledComparisonRefused`,
   `EqualityQuantityConversionRejected`, `GeneratedTypeCheckRejected` and
   `GeneratedEqualityCheckRejected` with
   concrete kernel-owned `IllTypedCause` payloads. Reject a
   `CallDepthExceeded` variant: RT FR-273's final explicit-frame scheduler
   has no depth-specific outcome, and denied `work_units` is `Incomplete`.
   Reject the retired `GeneratedBodyPlaceholderInvoked` variant: IR-595 removed
   its owning placeholder, and no other trigger inherits it.
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
   incompatible value-kind pair (a composite against a non-composite counts; two
   composites of different declarations do not, they give `false` under TC-924). Assert `CollectionKindMismatch`,
   `PopulationPair`, `ValueKindMismatch`, and `ValueKindMismatch` respectively.
   Check the pair is refused without a completed Boolean.
5. For each of the thirty-one public cause variants, construct the typed
   `CheckedInvariant` and assert both catalog methods return `None` and that
   refusals with different causes compare unequal. Re-run FR-096's ordinary
   refusal table unchanged so the new path cannot erase a catalog code.

## Expected Results

Every reachable kernel failure reports its exact table cause, and all
checked-invariant payloads stay internal faults with no catalog code or cause.
The generated `GeneratedUnexpectedOutcome` name denotes a completed value
rejected by the checker-issued result type, not an unhandled outcome arm.
`GeneratedEnvironmentRejected` denotes the selected owner's actual declaration
constructor rejection, not resource exhaustion. This kernel target tests their
carrier and non-catalog mapping only; CG FR-042 owns the real producer controls
and the generated-body versus leaf-failure boundary inspection in FR-369-AC-10.
No test counts a manually constructed refusal as proof that a production
raise site used that cause. The RT and external mapping criteria remain
planned until their own tagged tests and inspection run.

## Status

The target is the thirty-one-cause carrier after retirement of
`GeneratedBodyPlaceholderInvoked`. The current carrier and exhaustive test
still include that variant; their removal is implementation work under IR-709,
not acceptance supplied by this specification-only correction. The remaining
kernel carrier controls and the RT explicit-frame/work-fuel cutover and external
producer controls retain their own evidence and ownership.
