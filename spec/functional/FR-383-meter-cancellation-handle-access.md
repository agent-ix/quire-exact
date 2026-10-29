---
id: FR-383
title: "Expose the meter's cancellation handle"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-358
    type: references
---
# FR-383: Expose the meter's cancellation handle

## Description

`quire-exact` SHALL expose a meter's installed cancellation handle without
creating cancellation or accounting state.

## Inputs

- A borrowed `Meter`, constructed with `Meter::new` and optionally configured
  with `Meter::with_cancel`.

## Outputs

- `Option<&Cancel>`: `None` when no handle is installed, otherwise a borrow of
  the handle installed on that meter.

## Behavior

`Meter` SHALL provide `cancel(&self) -> Option<&Cancel>`. The accessor returns
a borrowed handle. A caller can clone that handle before using it alongside a
`&mut Meter` borrow or installing it on another meter with `with_cancel`.
`Cancel::clone` shares the one cancellation state, and cloned handles compare
equal by shared-state identity. For a meter configured with
`with_cancel(handle)`, the returned `Some` SHALL refer to the
same shared cancellation state that the meter polls at each charge.
The accessor itself SHALL create no new handle or state and copy no counters.
The accessor SHALL NOT change any limits, consumption, admission count,
injection or diagnostic log.
When a later `with_cancel` replaces the handle, the accessor SHALL report the
replacement. The method SHALL remain available in the crate's `no_std` +
`alloc` build.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-383-AC-1 | `Meter::new(limits).cancel()` is `None`. After `with_cancel(handle.clone())`, `cancel()` is `Some(&Cancel)` equal by shared-state identity to `handle`, and unequal to an independently created handle. Replacing the handle reports only the replacement. | Test |
| FR-383-AC-2 | Cancel the handle obtained from `meter.cancel()` with `Requested`, then charge the meter under sufficient limits: the charge is denied at its named point, that handle records `tripped() = Some(Requested)`, and all accounting state stays unchanged. On a separate fresh meter, cancel an independent handle while its installed handle remains live: an otherwise identical charge is admitted. | Test |
| FR-383-AC-3 | The accessor's implementation borrows the existing optional handle without cloning it or constructing a second `Meter` or `Counters`; calling it changes no meter accounting or injection state. | Inspection |
| FR-383-AC-4 | The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles in the crate's `no_std` + `alloc` build. | Test (`make build-no-std`) |

## Status

Implemented for IR-895. [TC-933](../test-cases/TC-933-meter-cancellation-handle-access.md)
has direct public-API Trace bindings for AC-1 and AC-2. AC-3 is verified by
inspection of the borrowing accessor; AC-4 is verified by `make build-no-std`.
The computed `quire matrix` reports AC-1, AC-2, and AC-4 as tagged, and AC-3
as method-without-symbol. AC-4's Trace-bound public signature probe is paired
with `make build-no-std`; the host test alone does not establish `no_std`
availability.

## Dependencies

- [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md) defines the
  meter's cancellation-before-injection behavior and accounting invariants.
- QSL's proposed FR-175 tuple evaluator in PR #1021 builds an uncapped
  position-search meter from the original meter's limits and a separately
  supplied `Option<&Cancel>` (`qsl-eval/src/temporal/tuple.rs:237-255,307-312`).
  The separate argument can disagree with the original meter. This requirement
  lets that consumer derive the search handle from the original meter.

## Decision

Choose **A, the borrowed accessor**. QSL needs the original handle to attach
to its own position-search meter, whose work cap differs from the original.
The accessor is smaller and more general: consumers choose the new meter's
limits and clone the borrowed handle when `with_cancel` needs ownership. That
clone shares the original cancellation state. Cloning the original `Meter`
would also copy its `Counters` (consumption, admission count and injection
state) and, with `test-support`, its diagnostic log. Constructing a fresh
uncapped meter instead copies no counters, but prescribing that construction
here would take the search meter's limits out of the consumer's hands.
Borrowing the existing `Cancel` adds no allocation or state to this `no_std` +
`alloc` crate, is additive to the public API, and permits direct identity and
cancellation tests. No compatibility layer is required.
