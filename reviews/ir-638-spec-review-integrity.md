---
id: SR-008
title: "Integrity review of quire-exact PR #4 meter specification"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@a4b7031f68077ff9de4be0bad3ab28f9186df413; spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md, spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-358
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-906
    type: reviews
---
# Integrity review of quire-exact PR #4

## Summary

Ticket: IR-638. The six new acceptance criteria and TC-906 were reviewed against the current `Meter` and `Counters` declarations, and the local QSL consumer usages were inspected. The requirement is coherent on the usual path, but its tests leave a failed same-point charge unexamined and its production boundary checks only one field-bearing type.

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-358-AC-1 | examined | With a denial installed at occurrence 2 of point P, admit P once and an unrelated point once. The next P charge returns `Incomplete` at P with the work counter as its limit kind; all consumed counters, admission count, and test-support log equal their values immediately before refusal. A subsequent P charge succeeds and updates ordinary accounting and the log exactly once. |
| FR-358-AC-2 | examined | With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan` returns `Incomplete` at `equality.plan` without changing any consumed counter, admission count, or test-support log; a subsequent valid `charge_plan` succeeds and records its ordinary plan charge. |
| FR-358-AC-3 | examined | The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction with zero is rejected by that type, while a nonzero occurrence constructs a denial. |
| FR-358-AC-4 | examined | Under `test-support`, after 4096 admissions `admitted_charges()` returns all 4096 points in order and `charge_log_truncated()` is false. On the 4097th admission the slice still contains exactly that prefix and the flag becomes true. |
| FR-358-AC-5 | examined | Under `test-support`, after the 4097th admitted charge, `admission_count()` and consumed work units reflect all 4097 admissions, including the one omitted from the diagnostic log. |
| FR-358-AC-6 | examined | Without `test-support`, the production `Counters` type remains fixed-size with no heap-owned charge log. |
| TC-906 | examined | Verify FR-358-AC-1 through FR-358-AC-6 at the public meter seam. |

## Verdict

Request changes for two medium gaps. AC-2 tests the separate `charge_plan` path, AC-3 gives a concrete public type, and AC-4/5 observe the 4096/4097 boundary and independent accounting. The QSL checkout has integer-literal and integer-variable `InjectedDenial::occurrence` initializers, so adopting `NonZeroU64` requires a coordinated QSL change as the dependency note says.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-1 and TC-906 step 1 admit only successful charges before the selected denial. An implementation that advances the named occurrence on an ordinary-limit refusal could pass this test, despite Behavior 1 counting admitted charges. Add an ordinary-limit refusal at P before the first admitted P, then demonstrate that the injected denial still occurs on the selected admitted-position attempt. | spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md:23; spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md:17 |
| FND-002 | medium | AC-6 and TC-906 step 5 inspect only `Counters`, while the current diagnostic log is a field of `Meter`. A production `Meter` could keep an unbounded log outside `Counters` and pass the stated criterion. Require and inspect that a production `Meter` has no admitted-charge log, while its accounting state remains fixed-size. | spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md:28; spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md:21 |

## Dispositions

| FND | Outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 6f8e7d366be7020e53ba8be0bdb41cb1397bd7c3 |
| FND-002 | fixed | 6f8e7d366be7020e53ba8be0bdb41cb1397bd7c3 |
