---
id: FR-381
title: "Bounded cancellable equality of retained numeric types"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-semantic-value/FR-117
    type: references
  - target: ix://agent-ix/quire-exact/FR-380
    type: references
---
# FR-381: Bounded cancellable equality of retained numeric types

## Description

When a supplied-admission caller compares two retained Integer intervals or
two retained Rational domains as type links, `quire-exact` SHALL offer a
fallible equality operation under that caller's authenticated finite magnitude
ceiling `B`. This is type equality over both retained domains, separate from
[FR-380](./FR-380-bounded-cancellable-numeric-membership.md)'s value membership.

## Inputs

- Borrowed left and right `IntegerInterval` values, or borrowed left and right
  `RationalDomain` values.
- The original borrowed `Cancel` handle and a finite caller-authenticated
  maximum magnitude bit length `B`; zero has magnitude length one. The kernel
  assigns no default and does not derive `B` from one operand or a work budget.

## Outputs

- Boolean equality of the retained domains without mutating either; or
- A typed stop distinguishing an operand above `B`, with its side, endpoint,
  measured bits and `B`, from cancellation with the original `CancelCause`.
  Any genuinely reachable checked-size or storage failure has its own typed
  classification and measured request; an allocation-free implementation with
  no fallible native conversion has no such artificial failure.

## Behavior

1. The kernel SHALL expose separate fallible borrowed operations for Integer
   interval equality and Rational domain equality. A failed bound or
   cancellation check SHALL NOT fall through to ordinary `Eq`, return `false`,
   or invoke the bool-only membership helpers.
2. When endpoint comparison is requested, the kernel SHALL first check every endpoint's
   magnitude length against `B`, in this stable order: left lower, right lower,
   left upper, right upper for Integer; left numerator lower, right numerator
   lower, left numerator upper, right numerator upper, left denominator lower,
   right denominator lower, left denominator upper, right denominator upper for
   Rational. The first oversized endpoint SHALL produce typed unsupported size
   naming that exact position and measured bits. Magnitude checking SHALL NOT
   materialize an absolute value, `BigInt`, clone, or radix string.
3. When all endpoint checks succeed, Integer equality SHALL compare the lower
   endpoints and upper endpoints for mathematical integer equality. Rational
   equality SHALL compare the corresponding numerator and positive-denominator
   interval endpoints. A completed unequal comparison SHALL return `false`,
   including when an early endpoint differs; no normalization, cross
   multiplication, semantic evaluator debit, or operand mutation occurs.
4. Let `L = ceil(B / 32)`. The four Integer endpoints require at most two
   borrowed sign/length/`u32` limb comparisons and `2L` limb-pair steps; the
   eight Rational endpoints require at most four such comparisons and `4L`
   steps. Each step SHALL perform fixed-width work. The production route SHALL
   establish finite bound-check, comparison and temporary-storage costs from
   the retained representation. An allocation-free route SHALL use constant
   auxiliary storage. If scratch or native layout conversion is actually
   required, the kernel SHALL use checked sizing and fallible reservation and
   report capacity or actual allocation denial separately from inequality and
   unsupported size.
5. The kernel SHALL poll the original `Cancel` before input-sized work, before
   each limb-pair step, and before publishing equality. Cancellation SHALL
   retain the original cause and prevent the next step or result. Its arithmetic
   response gap is at most one fixed-width limb-pair step; an actual allocator
   call, if present, is indivisible.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-381-AC-1 | With authenticated B, equal and unequal Integer intervals and Rational domains return the same Boolean equality as independent endpoint arithmetic, including negative, zero, and denominator endpoints, without mutation. Equal spellings or identity of borrowed objects do not substitute for endpoint equality. | Test |
| FR-381-AC-2 | Every one of the four Integer and eight Rational endpoint positions completes at exact B and returns typed unsupported size when made one bit over B, before endpoint comparison. Two oversized endpoints report the first in the stated order; an oversized endpoint on the right is never skipped by a left-only check. | Test |
| FR-381-AC-3 | Observed production comparisons use no more than 2L Integer or 4L Rational limb-pair steps, fixed-width work per step, and the measured temporary-storage bound, including inline/Large and 32-bit limb boundaries. Inspection finds no input-sized clone or unbounded fallback. | Test |
| FR-381-AC-4 | A pre-cancelled original handle prevents input-sized work; cancellation on a late observed limb step stops before the next step or equality result with the original cause. | Test |
| FR-381-AC-5 | Inspection establishes whether production performs native-size conversion or allocation. Each reachable denial is typed with its measured request and retry behavior; an allocation-free route with no failing conversion has no synthetic resource fixture. | Inspection |

## Dependencies

- [FR-380](./FR-380-bounded-cancellable-numeric-membership.md) supplies the
  existing borrowed-limb numeric value membership precedent, but its three/six
  operand checks do not establish this four/eight endpoint contract.
- [QSV FR-117](ix://agent-ix/quire-semantic-value/FR-117) owns caller `B`,
  type-link event order, rooted locus and supplied-phase failure projection.

## Status

SPEC CANDIDATE; kernel implementation and acceptance tests are unrun.
