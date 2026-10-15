---
id: TC-929
title: "Bounded retained numeric type equality checks both sides"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-381
    type: verifies
---
# TC-929: Bounded retained numeric type equality checks both sides

## Description

Verify [FR-381](../functional/FR-381-bounded-retained-numeric-type-equality.md)
through the production fallible Integer-interval and Rational-domain equality
operations. Construct valid retained domains before observing the helper.

## Test Procedure

1. Compare distinct objects with equal endpoints and domains differing in each
   endpoint, including negative and zero Integer bounds and positive Rational
   denominator bounds. Derive expected equality independently from endpoint
   arithmetic. Check that neither borrowed domain changes.
2. Set B to the largest endpoint magnitude length. For each of the four
   Integer and eight Rational positions, exercise exact B and then a valid
   domain whose selected endpoint is B+1 bits. Assert the typed side/endpoint,
   measured bits and B before comparison. Make two positions oversized to
   verify the stated first-refusal order, including a small left domain with
   an oversized right endpoint.
3. Observe production limb-pair steps for longest equal and late-unequal
   comparisons at inline/Large and 32-bit boundaries. Assert `2L`/`4L` bounds,
   fixed-width steps and the actual storage envelope. Inspect the production
   source for input-sized cloning or fallback. Exercise capacity or allocation
   denial only if an actual reachable conversion/reservation exists; check its
   native request and retry without denial.
4. Pre-cancel the original handle and cancel it at an observed late limb step.
   Assert the original cause, no further comparison step, and no Boolean result.

## Expected Results

Completed results match endpoint equality. Each exact-B endpoint completes,
and every oversized endpoint produces its precise typed refusal before
comparison. Work and storage stay within the measured envelope; cancellation
retains the original cause. A failed resource request, when reachable, never
becomes inequality, size refusal or membership work exhaustion.

## Status

PLANNED/UNRUN; no kernel acceptance result is claimed.
