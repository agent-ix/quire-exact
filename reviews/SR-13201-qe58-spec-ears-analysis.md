---
id: "SR-13201"
title: "quire-exact PR 58 EARS analysis: FR-383 statements"
type: SpecReview
analysis: ears-conformance
review_set: subset
scope: "agent-ix/quire-exact@53155ee7ec44b8b2738b0ec55f19e2840535c3da; spec/functional/FR-383-meter-cancellation-handle-access.md (Description 14-16, Behavior 30-37) (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-exact/FR-383
    type: reviews
---

# SR-13201: quire-exact PR 58 EARS analysis

## Summary

Ticket: IR-895. This review covers only the statements the PR adds.

Three statements conform. "For a meter configured with `with_cancel(handle)`, the returned `Some`
SHALL refer to the same shared cancellation state" uses the state form. "When a later
`with_cancel` replaces the handle, the accessor SHALL report the replacement" uses the event form.
"The method SHALL remain available in the crate's `no_std` + `alloc` build" is ubiquitous.

`quire validate` passes on both new files and on `spec/**/*.md`, with no AC-shape warnings. The
acceptance criteria assert outcomes and contain no SHALL.

## Verdict

PASS with three low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The Description's trigger, "When a caller needs the cancellation handle ... for related work", is a caller's intent, not an event the system can observe. The obligation is ubiquitous: "`quire-exact` SHALL expose a meter's installed cancellation handle without creating cancellation or accounting state." | spec/functional/FR-383-meter-cancellation-handle-access.md:14-16 |
| FND-002 | low | "The kernel SHALL add `Meter::cancel(&self) -> Option<&Cancel>`" places an obligation on the change, not on the system. State the behaviour instead: "`Meter` SHALL provide `cancel(&self) -> Option<&Cancel>` ...". Such a statement remains true after the change lands | spec/functional/FR-383-meter-cancellation-handle-access.md:30 |
| FND-003 | low | "The method SHALL create no new `Cancel`, clone no handle, copy no counters, or change any limits, consumption, ..." The final clause has no negation of its own. Read literally, "or change any" is a positive alternative. Write "and SHALL NOT change any limits, ..." | spec/functional/FR-383-meter-cancellation-handle-access.md:33-34 |
