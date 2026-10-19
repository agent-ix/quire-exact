---
id: TC-928
title: "Bounded Integer and Rational membership preserves size and cancellation"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-380
    type: verifies
---
# TC-928: Bounded Integer and Rational membership preserves size and cancellation

## Description

Verify [FR-380](../functional/FR-380-bounded-cancellable-numeric-membership.md)
through the production fallible kernel entry points and the real supplied
caller after an authentic finite magnitude ceiling is available. Construct
already reduced rationals and domains before observing the membership helper.

## Test Procedure

1. For Integer and Rational domains, check both inclusive endpoints, one
   adjacent nonmember on each side, negative/zero values, and rational
   numerator/positive-denominator boundaries. Compare expected truth values
   with an independently authored mathematical oracle, not a second call to
   the helper under test.
2. Set B to the greatest operand magnitude length in each fixture. Run the
   exact-B case. Independently make each of the three Integer and six Rational
   positions B+1 bits while retaining well-formed intervals/domains, and
   assert the typed operand-specific refusal before comparison. Include a
   small supplied value beside an oversized endpoint.
3. Observe production limb-pair steps on a longest admitted comparison, at
   inline/Large representation and 32-bit limb boundaries. Assert the 2L/4L
   ceilings, no input-sized hidden work/storage, and unchanged operands.
   Inspect whether the production path has a fallible native-size conversion
   or allocation. Exercise a typed capacity refusal only at a genuinely
   reachable checked conversion. If it allocates, deny its actual fallible
   reservation, assert the reported request, and retry without denial. Do not
   fabricate a failure case for an allocation-free path with no failing
   conversion.
4. Pre-cancel the original handle, then cancel it on an observed late limb
   step. Assert no later step or completed result, original cause, and no
   replacement handle. Check nonmembership remains Boolean false, distinct
   from all typed stops.
5. In QSV's supplied caller, use the authenticated bound for value and all
   endpoints, including B = 4096 supplied by QSL after caller API omission;
   assert its independently enumerated logical event count, with the
   initiating event charged before the B check, rooted locus, no extra limb
   event, no evaluator debit, and typed cancellation
   or reachable resource-failure projection. On a direct QSV supplied call
   without B, assert `UnsupportedHelper::NumericDomain` instead of completed
   membership; the direct QSV caller selects no default.
6. Inspect the `IntegerInterval::try_contains_bounded` and
   `RationalDomain::try_contains_bounded` signatures and implementations: both
   require a finite `magnitude_bits: u64` argument, use it for the value and
   every endpoint, and select no default. An omitted-bound call has no kernel
   API or kernel result.

## Expected Results

Membership agrees with the domain rule; exact-B operands complete and each
one-over operand is identified as unsupported size. Comparison and storage
stay inside the measured envelope. Reachable native-capacity or actual
allocation denial, cancellation, and nonmembership remain distinguishable. QSV
does not claim NumericDomain support until it authenticates a magnitude bound
for every compared operand.

## Status

PLANNED/UNRUN. QSpec FR-323 defines the QSL-owned magnitude carrier; the QSL
producer and QSV caller are pending, so this case cannot yet serve as runtime
qualification.
