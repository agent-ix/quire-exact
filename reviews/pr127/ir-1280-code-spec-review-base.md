---
id: SR-2026110101313
title: spec-review/base of IR-1280 production interval capture
type: SpecReview
analysis: base
scope: 'agent-ix/quire-exact; PR #127; src/accounting.rs; src/lib.rs; tests/ir1280_charge_accounting.rs;
  spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md'
review_set: subset
---

## Summary

Ticket: IR-1280. Candidate: quire-exact PR #127 against main after merged specification PR #126. Review is limited to the four-file candidate diff and relevant unchanged admission paths. No applicable AssuranceProfile found. Kernel completion does not establish QSL-1548 or FR-520-AC-11; binder routing, read presence and accounting_version remain QSL responsibilities.

## Analysis

The only specification edit replaces the planned-binding status sentence with a claim that executable public-meter bindings now exist. Computed matrix and actual six #[trace] test bodies support that precise claim. Requirements, acceptance criteria, relationships and TC-906 procedures are unchanged from merged PR #126. The same paragraph explicitly excludes downstream QSL completion. Structural, EARS, dependency and other specification lenses add no changed obligation to inspect and were not rerun for this status-only edit.

## Examined Scope

```yaml
scope:
- id: FR-358-AC-16
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: In a default-feature build, a fresh meter returns `None` from take and
    disable. Admit work before enabling, enable capture, take twice and observe `Some`
    with all ten amounts zero both times. After an admission, enable again and verify
    the unread amounts survive. Disable returns that final aggregate, subsequent take/disable
    return `None`, and re-enable starts with zeros. Each control operation preserves
    limits, all consumed counters, admission count, cancellation and injected-denial
    state and consumes no work.
- id: FR-358-AC-17
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With sufficient limits and capture enabled, exercise ordinary general and
    unit-only charges with independently specified work/result increments and all
    eight semantic-size kinds, including repeated same-kind requests, explicit zeros
    and absent kinds. The aggregate exposes the sum of actual work/results and the
    maximum actual request for each size, through `amount` for all ten kinds. Separately
    admit IntegerBits 9 while disabled, enable, admit IntegerBits 3 and verify the
    interval reports 3 while global consumption remains 9.
- id: FR-358-AC-18
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With an active nonempty interval, independently attempt ordinary work,
    result and semantic-size refusals, an injected denial, cancellation, and failed
    plan work/result/size availability checks. Each leaves the entire interval equal
    to its pre-attempt value. A later successful admission extends the preceding successful
    prefix exactly once. On a sufficient meter, `charge_plan` with pairs 2 adds exactly
    work 1, results 0 and ValueOccurrences 2, rather than reservation work 4 or result
    1.
- id: FR-358-AC-19
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: On one shared meter with prior consumption, partition a fixed successful
    charge schedule across repeated takes, including an empty interval, repeated enable
    and a final disable. Summing the returned work/results and taking per-kind maxima
    of the eight sizes equals the independently expected actual schedule usage and
    an uninterrupted capture of that schedule; prior consumption remains excluded.
    Perform the same schedule with capture disabled and verify identical admission/refusal
    outcomes, global consumed counters and admission counts.
- id: FR-358-AC-20
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 'In default-feature and test-support builds, an enabled interval aggregates
    4097 one-work/one-result admissions, with an independently specified largest semantic-size
    request on admission 4097: work/results equal 4097 and the size includes that
    last request. Under test-support, the existing ordered diagnostic prefix remains
    exactly 4096 records and its existing truncation flag changes only at 4097.'
- id: FR-358-AC-21
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Inspect the production capture and `ChargeAccounting` types for private
    fixed-size heap-free state and absence of history, caller metadata, callbacks,
    trait objects or new Meter lifetime/generic parameters. Default construction and
    enabled capture/control operations allocate no heap storage. Verify the unchanged
    no_std build and Meter's Clone/Debug/Eq/PartialEq/Send/Sync contracts; cloning
    active capture copies unread amounts independently, so draining one clone does
    not drain the other.
- id: TC-906
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 'Verify [FR-358](../functional/FR-358-meter-denial-and-bounded-diagnostic-log.md)
    at the public meter seam. Scope: FR-358-AC-1 through AC-6 and AC-8 through AC-21.
    AC-7 was moved to [FR-368](../functional/FR-368-meter-charge-and-limit-vocabulary.md).'
bindings:
- test_id: lifecycle_preserves_budget_denial_and_unread_amounts
  ac_id: FR-358-AC-16
  trace: correct
- test_id: all_ten_amounts_capture_actual_requests_and_canonical_increments
  ac_id: FR-358-AC-17
  trace: correct
- test_id: every_refusal_preserves_prefix_and_plan_records_actual_commit
  ac_id: FR-358-AC-18
  trace: correct
- test_id: partitions_conserve_usage_and_capture_does_not_change_execution
  ac_id: FR-358-AC-19
  trace: correct
- test_id: capture_continues_after_diagnostic_prefix
  ac_id: FR-358-AC-20
  trace: correct
- test_id: clones_are_independent_and_capture_controls_allocate_nothing
  ac_id: FR-358-AC-21
  trace: correct
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Reviewed coder qualification logs under /home/peter/dev/qualification-logs/ir1280/code/: fmt and Clippy default/support passed; affected integrations passed 18 per mode; accounting unit tests passed 40 default and 43 support; thumbv7em-none-eabi no-default-features library build passed. Final cancellation strengthening was rerun in both modes and both lint modes. Independently reran all six new tests in default and test-support modes: passed. No Kani, IEEE16G proof family, broad aggregate, dependency/pin/workflow or release action was run or required for this scoped review.

Plan completion: not assessed

## Verdict

PASS for the scoped candidate. No findings.
