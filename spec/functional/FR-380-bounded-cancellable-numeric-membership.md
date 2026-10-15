---
id: FR-380
title: "Bounded cancellable Integer and Rational supplied membership"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-semantic-value/FR-109
    type: references
  - target: ix://agent-ix/quire-specification/FR-147
    type: references
---
# FR-380: Bounded cancellable Integer and Rational supplied membership

## Description

When a supplied-admission caller checks an already constructed Integer or
reduced Rational against an inclusive numeric domain, the exact kernel SHALL
offer fallible membership under an explicit, finite magnitude ceiling supplied
by that caller. The kernel SHALL retain mathematical membership, distinguish
size and resource refusal from nonmembership, and observe the caller's original
cancellation handle.

## Inputs

- A borrowed `Integer` and `IntegerInterval`, or a borrowed reduced `Rational`
  and `RationalDomain`.
- The initiating caller's original borrowed `Cancel` handle.
- A finite maximum `B` in magnitude **bits**, where zero has magnitude length
  one, authenticated by the supplied-admission owner for this invocation.
  This requirement assigns no numeric default and does not treat a logical
  event ceiling, a value-only bound, or a type interval as proof of `B`.

## Outputs

- Completed Boolean membership, with no mutation of the value or domain; or
- A typed failure distinguishing an operand exceeding `B`, checked native
  capacity/size failure, and cancellation with the original `CancelCause`.
  A comparison requiring any allocation SHALL additionally distinguish actual
  allocation denial and report its real request; an allocation-free comparison
  has no such reachable outcome.

## Behavior

1. The kernel SHALL expose separate fallible Integer-interval and
   Rational-domain membership entry points that borrow their operands,
   explicit `B`, and original `Cancel`. They SHALL NOT silently fall back to
   the bool-only `contains` paths when the bound or resource check fails.
2. When either membership comparison is requested, the kernel SHALL check `B` and the
   magnitude bit length of every operand it would compare. Integer membership
   checks **three** integers: the supplied value, lower endpoint, and upper
   endpoint. Rational membership checks **six** integers: the reduced value's
   numerator and denominator, and both endpoints of each numerator and
   denominator interval. Any one operand above `B` SHALL return typed
   unsupported size identifying that operand and its measured bit length,
   without performing a membership comparison or returning nonmembership.
   The kernel SHALL check conversion of `B` to a native limb count and any storage layout
   before using that conversion, returning a distinct capacity/size
   failure on overflow. The bound check itself SHALL NOT materialize a
   `BigInt`, absolute value, clone, or radix string.
3. When input bounds admit the operands, completed Integer membership SHALL be the existing
   inclusive `lower <= value <= upper`; completed Rational membership SHALL
   check the reduced numerator and positive denominator against their
   respective inclusive intervals. This operation SHALL NOT cross-multiply
   rationals, normalize them again, narrow the value, or charge a semantic
   evaluator meter.
4. Let `L = ceil(B / 32)` for the largest admitted magnitude bound. A
   comparison over borrowed signs, lengths and `u32` magnitude limbs SHALL
   inspect at most `L` limbs per pair. Integer membership has at most two
   pairs and Rational membership at most four pairs, so at most `2L` and `4L`
   limb-pair steps respectively, plus a fixed number of magnitude/sign
   checks and constant scalar state. Each limb-pair step SHALL do fixed-width
   work; it SHALL NOT hide an input-sized clone, conversion or allocation.
   An allocation-free implementation SHALL use constant auxiliary storage;
   if the maintained dependency requires scratch, the owner SHALL revise the
   measured request/storage envelope and use checked, fallible reservation
   before claiming this criterion. These are kernel work bounds, not new
   supplied logical events or an arbitrary work allowance.
5. The helper SHALL poll the original `Cancel` before bound validation, before
   every limb-pair step, and before publishing a completed result. A poll that
   observes cancellation SHALL return the original cause before another step
   or completed result. A pre-cancelled invocation SHALL do no input-sized
   work. No replacement handle, clock-derived cancellation or partial
   admission result is permitted. The arithmetic response gap is at most one
   fixed-width limb-pair step; allocator calls, if any, remain indivisible and
   carry no wall-clock latency claim.
6. The supplied-admission consumer SHALL establish the authentic `B` premise
   for the value **and all endpoints** before invoking these helpers. It SHALL
   retain [QSV FR-109](ix://agent-ix/quire-semantic-value/FR-109)'s cumulative
   logical-event budget, rooted locus and original cancellation handle, and
   project helper failure without recoding it as traversal Limit or Invalid
   membership. Limb steps SHALL NOT debit an evaluator meter or create a
   per-limb supplied event. If no authoritative admitted or caller-owned `B`
   exists, the consumer SHALL retain typed `NumericDomain` unsupported status;
   a convenient default or value-only measurement is not qualification.
7. When claiming production acceptance, the owner SHALL verify the retained
   Integer representation and actual comparison path against these work,
   storage, cancellation and failure bounds. The current bool-only
   `IntegerInterval::contains` and `RationalDomain::contains` are not evidence
   that a fallible, cancellable path has landed.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-380-AC-1 | With an authenticated B, Integer and reduced Rational membership agree with the inclusive existing rule at both endpoints, one outside each endpoint and a denominator boundary, without mutating operands. | Test |
| FR-380-AC-2 | Exact-B value and every exact-B endpoint are accepted for comparison; one-over-B in each of the three Integer or six Rational operand positions returns typed unsupported size naming that position and measured bits before any comparison. A value-only guard mutant fails an endpoint case. | Test |
| FR-380-AC-3 | Observed production comparison steps do not exceed 2L for Integer or 4L for Rational, with fixed-width per-step work and constant auxiliary storage on the allocation-free path; source inspection confirms magnitude checks and borrowed limb iteration do not hide input-sized work/allocation. | Test |
| FR-380-AC-4 | A pre-cancelled original handle prevents input-sized work. Deterministically cancelling during a longest admitted comparison returns that cause before the next step/result; a fresh-handle mutant fails. | Test |
| FR-380-AC-5 | Checked B-to-native/layout failure is typed capacity/resource refusal, not false, unsupported size or a traversal limit. If implementation requests storage, denial at the actual reservation returns a distinct typed allocation failure carrying its measured request and retry without denial agrees with a fresh run. | Test |
| FR-380-AC-6 | The QSV supplied caller documents and tests the authentic magnitude ceiling for value plus every endpoint, invokes the qualified helper under one initiating logical event, preserves original cancellation and locus, and leaves evaluator charges unchanged. Without that carrier it returns `UnsupportedHelper::NumericDomain`; no numeric default is added. | Test |

## Dependencies

- [TC-928](../test-cases/TC-928-bounded-numeric-membership.md) defines the
  falsifying kernel and supplied-caller controls.
- [QSpec FR-147](ix://agent-ix/quire-specification/FR-147) retains Integer and
  Rational domain meaning. This requirement changes the bounded helper
  response, not mathematical membership.
- [QSV FR-109](ix://agent-ix/quire-semantic-value/FR-109) owns supplied phase,
  cumulative events, locus and consumer projection. Its current logical
  budget is **not** a magnitude carrier. Establishing an authentic source for
  `B` is an open prerequisite; until then NumericDomain remains unsupported.

## Status

SPEC CANDIDATE. Kernel and QSV implementation/tests are unrun. No admitted or
caller-owned magnitude ceiling covering the value and all endpoints has been
identified in the measured published QSV supplied API.
