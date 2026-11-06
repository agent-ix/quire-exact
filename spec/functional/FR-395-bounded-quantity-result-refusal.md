---
id: FR-395
title: "A bounded quantity result refuses with its exact target"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: references
  - target: ix://agent-ix/quire-exact/FR-096
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
---
# FR-395: A bounded quantity result refuses with its exact target

## Description

The kernel SHALL check the already converted exact magnitude against a bounded quantity result type and return a typed refusal if it is outside the inclusive interval. QSpec FR-322-AC-384 and BQ-09 own the checked-package conversion and diagnostic location. The caller owns unit conversion, the FR-154 unit identity spelling and the `/body/result_type` locus; the kernel owns exact membership and the target facts carried by its refusal.

## Use case

A caller has converted a quantity to unit `m` and has an exact magnitude of `2/1`. The target type is `Quantity<m>[-1/1, 1/1]`. The kernel refuses and carries unit `m` and both exact endpoints, so the caller can render the QSpec `expected` value without the converted magnitude.

## Behavior

1. A bounded result target contains a `UnitId` and exact inclusive `Rational` minimum and maximum. Reversed bounds form a distinct empty target and retain their original order. The caller supplies an already converted magnitude in that target unit; the kernel does no unit graph traversal.
2. A magnitude equal to either endpoint completes as a `Quantity` in the target unit. A magnitude strictly outside either endpoint, or any magnitude against a reversed target, returns `Refusal::BoundedQuantityOutOfDomain` with the typed target and no converted value.
3. The kernel meters both exact rational comparisons before cross multiplication. If a charge is unavailable, it returns an incomplete outcome without a partial value.
4. The new refusal's code is `rational_out_of_domain` and cause is `outside-domain`. The existing `Refusal::RationalOutOfDomain` retains its `RationalDomain` target and behavior.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-395-AC-1 | For a target `Quantity<U>[-1/1, 1/1]`, exact results at `-1/1` and `1/1` complete in `U`; results of `2/1` and `-2/1` refuse with no value. | Test (TC-942) |
| FR-395-AC-2 | The refusals from `2/1` and `-2/1` carry `U`, `-1/1` and `1/1` as typed values, returns `rational_out_of_domain`/`outside-domain`, and contains no converted magnitude. | Test (TC-942) |
| FR-395-AC-4 | For a reversed target `Quantity<U>[5/1, 3/1]`, construction retains `U`, `5/1`, and `3/1`, and results `2/1`, `4/1`, and `6/1` each refuse with that same target. | Test (TC-942) |
| FR-395-AC-5 | When a comparison charge is unavailable, result admission returns an incomplete outcome before exact comparison. | Test (TC-942) |
| FR-395-AC-3 | The old `RationalOutOfDomain` variant still carries its `RationalDomain` and returns `rational_out_of_domain`/`outside-domain`. | Test (TC-942) |

## Status

Implemented.

## Dependencies

- QSpec FR-322-AC-384 and BQ-09 define the public diagnostic payload and locus. FR-088 owns `UnitId`; FR-096 owns kernel code and cause mapping.
