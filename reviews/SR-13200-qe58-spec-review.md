---
id: "SR-13200"
title: "quire-exact PR 58 spec review: FR-383 Meter cancellation handle accessor"
type: SpecReview
analysis: base
review_set: subset
scope: "agent-ix/quire-exact@53155ee7ec44b8b2738b0ec55f19e2840535c3da; spec/functional/FR-383-meter-cancellation-handle-access.md, spec/test-cases/TC-933-meter-cancellation-handle-access.md (diff origin/main...HEAD)"
relationships:
  - target: ix://agent-ix/quire-exact/FR-383
    type: reviews
---

# SR-13200: quire-exact PR 58 spec review

## Summary

Ticket: IR-895. The ticket text (IR-895, QSL-894) was treated as an unmeasured claim. Every
fact below was re-measured against the sources.

- quire-exact head 53155ee: `Meter` holds `cancel: Option<Cancel>` (src/accounting.rs:657), set
  by `with_cancel(mut self, cancel: Cancel)` (lines 696-700) and polled in `check_injected`
  (line 759). It has no accessor. `Cancel` is `#[derive(Clone)]` over `Arc<Shared>`
  (src/cancel.rs:50-55). Equality is `Arc::ptr_eq` (lines 202-208), so a clone shares one state
  and a clone is not a second cancellation state.
- QSL PR #1021 head b2efba5: `evaluate_tuple(..., meter: &mut Meter, cancel: Option<&Cancel>)`
  (tuple.rs:307-313) passes `meter` as `&Meter` to `false_position`
  (tuple.rs:237-244). That function builds a fresh search meter
  (`work_units: u64::MAX, ..*meter.limits()`) and installs `cancel.clone()` (lines 249-255). The
  test at tests/it/hyper_tuple.rs:390-410 passes the same handle twice.

Borrow check for decision A: `false_position` holds only `&Meter`, so
`meter.cancel().cloned()` borrows immutably and ends before any `&mut` charge. `with_cancel`
takes `Cancel` by value, so the consumer clones the borrowed handle anyway. The clone is an
`Arc` increment that shares state. Decision A is therefore sufficient for QSL's use, and B is not
required. The shared-borrow conflict with `&mut Meter` does not arise in the cited code.
FND-001 covers the point that the FR does not say this.

The PR body cites accounting.rs:654-657, 680-705 and 759-765, cancel.rs:50-55 and 202-208, and
QSL tuple.rs and hyper_tuple.rs. All of these match the sources. Its validation claims
reproduce. The diff contains no review file written by the author.

## Verdict

PASS with two low findings. Neither blocks the merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-383 never says how a caller uses the borrowed handle with a mutable meter or with another meter. `with_cancel` takes `Cancel` by value, so QSL must call `meter.cancel().cloned()`. That clone shares state through `Arc`. Behavior line 33 says "clone no handle", which is about the accessor, but a consumer can read it as forbidding the clone it needs. State that a clone of the returned handle shares its state, and that this clone is how a caller uses the handle across a `&mut Meter` borrow or installs it on a second meter | spec/functional/FR-383-meter-cancellation-handle-access.md:30-34,57-61 |
| FND-002 | low | The Decision rejects B because an uncapped copy "would duplicate Counters". That is true only of a `Meter` clone. A B-variant that builds a fresh meter (the caller's limits, uncapped work, the shared handle) copies no counters, and it is what QSL builds today. The real reasons to prefer A are that it is smaller and more general, and that the search meter's limits are the consumer's policy. The rationale should say so | spec/functional/FR-383-meter-cancellation-handle-access.md:63-72 |
