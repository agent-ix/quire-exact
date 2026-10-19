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

When a caller needs the cancellation handle used by a `Meter` for related
work, `quire-exact` SHALL expose that meter's installed handle without
creating cancellation or accounting state.

## Inputs

- A borrowed `Meter`, constructed with `Meter::new` and optionally configured
  with `Meter::with_cancel`.

## Outputs

- `Option<&Cancel>`: `None` when no handle is installed, otherwise a borrow of
  the handle installed on that meter.

## Behavior

The kernel SHALL add `Meter::cancel(&self) -> Option<&Cancel>`. For a meter
configured with `with_cancel(handle)`, the returned `Some` SHALL refer to the
same shared cancellation state that the meter polls at each charge.
The method SHALL create no new `Cancel`, clone no handle, copy no counters, or
change any limits, consumption, admission count, injection or diagnostic log.
When a later `with_cancel` replaces the handle, the accessor SHALL report the
replacement. The method SHALL remain available in the crate's `no_std` +
`alloc` build.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-383-AC-1 | `Meter::new(limits).cancel()` is `None`. After `with_cancel(handle.clone())`, `cancel()` is `Some(&Cancel)` equal by shared-state identity to `handle`, and unequal to an independently created handle. Replacing the handle reports only the replacement. | Test |
| FR-383-AC-2 | Cancel the handle obtained from `meter.cancel()` with `Requested`, then charge the meter under sufficient limits: the charge is denied at its named point, that handle records `tripped() = Some(Requested)`, and all accounting state stays unchanged. On a separate fresh meter, cancel an independent handle while its installed handle remains live: an otherwise identical charge is admitted. | Test |
| FR-383-AC-3 | The accessor's implementation borrows the existing optional handle without cloning it or constructing a second `Meter` or `Counters`; calling it changes no meter accounting or injection state. The public signature compiles in a `no_std` + `alloc` build. | Inspection |

## Status

PLANNED for IR-895. [TC-933](../test-cases/TC-933-meter-cancellation-handle-access.md)
defines the future direct public-API controls. No implementation or test Trace
binding is claimed; use the computed `quire matrix` for coverage.

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
An uncapped meter copy would also duplicate `Counters` (consumption, admission
count and injection state) and, with `test-support`, its diagnostic log; it
would leave two accounting states that can diverge. Borrowing the existing
`Cancel` adds no allocation or state to this `no_std` + `alloc` crate, is
additive to the public API, and permits direct identity and cancellation
tests. No compatibility layer is required.
