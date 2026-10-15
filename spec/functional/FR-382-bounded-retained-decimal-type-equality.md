---
id: FR-382
title: "Bounded cancellable equality of retained Decimal types"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-381
    type: references
  - target: ix://agent-ix/quire-exact/FR-372
    type: references
  - target: ix://agent-ix/quire-semantic-value/FR-118
    type: references
---
# FR-382: Bounded cancellable equality of retained Decimal types

## Description

When a supplied-admission caller compares two retained Decimal types as type
links, `quire-exact` SHALL offer fallible equality under that caller's
authenticated finite magnitude ceiling `B`. This compares retained type
identity, separate from [FR-372](./FR-372-fallible-bounded-decimal-membership.md)
Decimal value membership and parallel to
[FR-381](./FR-381-bounded-retained-numeric-type-equality.md) Integer and Rational
type equality.

## Inputs

- Borrowed expected and actual `DecimalType` values, in that order.
- The original borrowed `Cancel` handle and a finite caller-authenticated
  maximum coefficient-endpoint magnitude bit length `B`; zero has magnitude
  length one. The kernel assigns no default and does not derive `B` from an
  operand or work budget.

## Outputs

- Boolean equality of the retained Decimal types without mutation; or
- A typed stop distinguishing an endpoint above `B`, with exact expected/actual
  position, measured bits and `B`, from cancellation with the original
  `CancelCause`. The allocation-free route has no scratch or native-size
  conversion failure to classify.

## Behavior

1. The kernel SHALL expose `DecimalType::try_eq_bounded(&self, actual: &Self,
   magnitude_bits: u64, cancel: &Cancel) -> Result<bool,
   NumericTypeEqualityFailure>`. A failed bound or cancellation check SHALL NOT
   fall through to ordinary `Eq`, become inequality, or invoke value membership.
2. When retained type equality is requested, the kernel SHALL check the four
   coefficient endpoint magnitude lengths against `B` in this stable order:
   expected lower, expected upper, actual lower, actual upper. The first
   oversized endpoint SHALL return `UnsupportedSize` with its
   `NumericTypeEndpoint` position, measured bits and `B`. The kernel SHALL poll
   the original `Cancel` before bound validation and before each endpoint check,
   so pre-cancellation wins over an oversized first endpoint. The kernel SHALL
   use retained borrowed representation for magnitude checks without absolute-value,
   `BigInt`, clone or radix-string materialization.
3. If all endpoint checks succeed, the kernel SHALL compare corresponding
   lower and upper endpoints by mathematical integer equality and both
   `min_scale` and `max_scale` and the `RoundingMode` by retained identity. A
   completed difference in any field SHALL return `false`; equivalent values
   or overlapping ranges alone do not establish type equality.
4. Let `L = ceil(B / 32)`. The kernel SHALL perform at most two borrowed
   sign/length/`u32`-limb comparisons and `2L` limb-pair steps. Each step SHALL
   use fixed-width work, with constant auxiliary storage. The kernel SHALL
   perform no input-sized clone or allocation, scratch reservation,
   native-size conversion, normalization, semantic evaluator debit, or operand
   mutation. No artificial capacity or storage stop is introduced when no
   fallible request exists.
5. The kernel SHALL poll the original `Cancel` before each limb-pair step and
   before publishing equality. Cancellation SHALL return `Cancelled` with the
   original cause before another step or result. Arithmetic response gap is at
   most one fixed-width limb-pair step; the kernel does not promise a wall-clock
   bound on scheduling.
6. The caller SHALL own the initiating type-link event, finite `B`, phase
   budget, and stop projection under [QSV FR-118](ix://agent-ix/quire-semantic-value/FR-118).
   This helper SHALL make no QSV logical event, per-limb supplied-budget debit,
   or semantic meter charge.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-382-AC-1 | Equal distinct Decimal types complete `true`; a difference in either signed endpoint, either scale or rounding mode completes `false` under authenticated `B`. Endpoint equality is mathematical integer equality. Borrowed inputs remain unchanged. | Test |
| FR-382-AC-2 | At `B >= 1`, every endpoint position completes at exact `B` and returns typed `UnsupportedSize` with the correct position, measured `B+1` bits and `B` when one bit over. Two oversized endpoints report the first in expected-lower, expected-upper, actual-lower, actual-upper order, even if an earlier retained field differs. At `B = 0`, zero is refused at expected lower with measured one bit. | Test |
| FR-382-AC-3 | Observed production comparisons at inline/Large and 32-bit limb boundaries perform at most `2 ceil(B/32)` limb-pair steps of fixed-width work and no input-sized clone or allocation; auxiliary storage is constant. Source inspection finds no hidden unbounded fallback or reachable storage/capacity failure. | Test |
| FR-382-AC-4 | A pre-cancelled original handle stops before bound validation, including when expected lower exceeds `B`; cancellation observed after a late limb step stops before another step or Boolean publication with the original cause. A fresh handle completes the same operands. | Test |

## Dependencies

- [FR-381](./FR-381-bounded-retained-numeric-type-equality.md) supplies the
  borrowed-limb type-equality mechanism and typed stop precedent.
- [FR-372](./FR-372-fallible-bounded-decimal-membership.md) concerns Decimal
  value membership; its scratch and shift bounds do not apply to type identity.
- [QSV FR-118](ix://agent-ix/quire-semantic-value/FR-118) owns caller event order,
  bound authentication and supplied-phase failure projection.

## Status

SPEC CANDIDATE; kernel acceptance results are unrun.
