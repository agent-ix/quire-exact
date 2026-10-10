---
id: FR-373
title: "Rational-target IEEE conversion returns Outcome directly"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-366
    type: references
  - target: ix://agent-ix/quire-exact/FR-358
    type: references
  - target: ix://agent-ix/quire-specification/FR-148
    type: references
---
# FR-373: Rational-target IEEE conversion returns Outcome directly

## Description

When a caller converts an IEEE value to a declared Rational domain, the exact
kernel SHALL return the conversion's typed `Outcome<IeeeExact>` directly through
its public `ieee_to_exact` operation.

## Inputs

- An `IeeeValue`, retaining its binary32 or binary64 bit pattern.
- A borrowed, well-formed `RationalDomain`.
- The caller's borrowed mutable `Meter`, including its already-spent counters,
  injected denial and original shared cancellation handle when installed.

## Outputs

`Outcome<IeeeExact>`: a completed reduced rational and optional negative-zero
loss, typed undefined, typed refusal or the original meter's incomplete record.
There is no outer `Result` or cancellation variant.

## Behavior

1. The kernel SHALL expose and re-export
   `ieee_to_exact(value: IeeeValue, domain: &RationalDomain, meter: &mut Meter)
   -> Outcome<IeeeExact>` in place of the target-selector signature.
2. The kernel SHALL remove `IeeeExactTarget` and its re-export without retaining
   a wrapper, alias or second conversion algorithm. The existing owner conversion
   and charging implementation remains the single authority.
3. The kernel SHALL preserve finite conversion, negative-zero loss, non-finite
   undefined and target-domain refusal semantics of the existing conversion.
   [FR-366](./FR-366-ieee-exceptional-value-semantics.md) owns signed-zero
   semantics. NaN and either infinity return `Undefined::IeeeNotFinite`;
   finite nonmembers return `Refusal::IeeeRationalOutOfDomain` carrying the exact
   declared target, without retaining the refused value.
4. The kernel SHALL use the supplied meter in the existing order:
   `IeeeOperands` at source width with one value occurrence; for finite values,
   `IeeeExactIntermediate` at the reduced rational's maximum numerator or
   denominator magnitude bit length; uncharged domain membership; then
   `IeeeResultRetain` with one result unit. Zero's intermediate size is one.
   Each admitted named charge consumes one work unit. Non-finite values stop
   after the operand charge; domain refusal stops after the intermediate charge.
5. When a charge is denied, the kernel SHALL return that meter's `Incomplete`
   unchanged without a partial result or consumption of the denied charge.
   [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md) owns injected
   denial behavior. When installed cancellation is observed by the meter,
   the operation preserves its `WorkUnits` incomplete record and the original
   shared handle's observed cause; caller cancellation reporting remains outside
   the kernel outcome. The operation never replaces the supplied meter or handle.

The direct signature cannot express a Decimal, Integer or bounded Integer target.
[QSpec FR-148](ix://agent-ix/quire-specification/FR-148) owns the language's
uncharged `ill_typed` rejection of those conversions at its type boundary.
Removing or changing the shared `IllTypedCause::IeeeToNonRationalExact` vocabulary
is outside this requirement. QSL and RT API adoption belongs to their owning
lanes; this requirement claims neither consumer migration nor deletion of QSL's
temporary defensive outer-error mapping. No numeric default changes.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-373-AC-1 | The public crate-root `ieee_to_exact` has the exact direct signature in Behavior. `IeeeExactTarget` and its re-export are absent; no compatibility entry point or copied conversion algorithm remains. A non-Rational target cannot be supplied to this signature. | Inspection |
| FR-373-AC-2 | Binary32 `0x3fc00000` and binary64 `0x3ff8000000000000` complete as `3/2` with no loss in numerator domain `[-3,3]`, denominator domain `[1,2]`. Both signed zeros at both widths complete as `0/1`; only negative zero carries `NegativeZeroSign`, preserving FR-366's original binary32 control. | Test |
| FR-373-AC-3 | Quiet NaN, signaling NaN and positive/negative infinity at each width return exactly `Undefined(IeeeNotFinite)` after one operand charge, one work unit, source-width IntegerBits and one ValueOccurrences high-water unit, with zero result units and no intermediate or retain admission. | Test |
| FR-373-AC-4 | Converting `3/2` to numerator domain `[-1,1]`, denominator domain `[1,2]`, and separately to numerator domain `[-3,3]`, denominator domain `[1,1]`, returns `Refused(IeeeRationalOutOfDomain { target })` with the corresponding exact target. Two work units are consumed; no result unit or retain charge is admitted. | Test |
| FR-373-AC-5 | Successful finite conversion admits exactly operands, intermediate and retain in that order, consuming three work units and one result unit. Ordinary work limits 0, 1 and 2 deny the respective next point with exact limit, consumed and next-charge fields. Injected occurrence-1 denials at each point report WorkUnits with limit equal to pre-charge consumed work and next charge 1. Result limit 0 denies retain atomically after two work units. Smallest positive subnormals at either width under IntegerBits equal to the source width deny intermediate with next-charge sizes 150 and 1075 respectively. Every denial leaves all counters and admission count at the independently specified admitted prefix and exposes no completed value. | Test |
| FR-373-AC-6 | For Requested and Deadline, pre-cancelling the original handle attached to the supplied meter returns WorkUnits incomplete at operands with limit = consumed = 0 and next charge 1. Deterministic cancellation at the original handle's second and third meter polls returns the same form at intermediate and retain with limit = consumed = 1 and 2 respectively. The original handle records the actual cause, only the prior charge prefix is admitted and no result is retained. A live handle completes the same fixture. | Test |

## Dependencies

- [TC-921](../test-cases/TC-921-rational-target-ieee-outcome.md) defines the
  direct public API controls using independently specified bits, outcomes and
  charge prefixes, rather than a retained old API as oracle.
- Existing [TC-914](../test-cases/TC-914-ieee-exceptional-value-semantics.md)
  signed-zero evidence remains required; adopting the direct signature does not
  weaken its value/loss assertions.

## Status

SPEC DRAFT. Code changes and all runtime controls are PLANNED/UNRUN. No consumer
migration, runtime qualification or full-gate success is claimed.
