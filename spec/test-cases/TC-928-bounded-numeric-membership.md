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
   Trigger a checked native capacity case. If the implementation allocates,
   deny its actual fallible reservation and assert the reported request; retry
   without denial.
4. Pre-cancel the original handle, then cancel it on an observed late limb
   step. Assert no later step or completed result, original cause, and no
   replacement handle. Check nonmembership remains Boolean false, distinct
   from all typed stops.
5. In QSV's supplied caller, use the authenticated bound for value and all
   endpoints; assert its independently enumerated logical event count and
   rooted locus, no extra limb event, no evaluator debit, and typed resource
   or cancellation projection. With the magnitude carrier absent, assert
   `UnsupportedHelper::NumericDomain` instead of completed membership.

## Expected Results

Membership agrees with the domain rule; exact-B operands complete and each
one-over operand is identified as unsupported size. Comparison and storage
stay inside the measured envelope. Native capacity, actual allocation denial
if reachable, cancellation, and nonmembership remain distinguishable. QSV
does not claim NumericDomain support until it authenticates a magnitude bound
for every compared operand.

## Status

PLANNED/UNRUN. The published QSV supplied interface has no measured magnitude
carrier covering all six Rational operands; this case cannot yet serve as
runtime qualification.
