---
id: SR-2026110101302
title: code-review review of IR-1280 interval accounting
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-exact; PR #126; spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md;
  spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md'
review_set: all
---

## Summary

Ticket: IR-1280. Candidate: quire-exact PR #126 against main; spec-only review. FR-358 AC-16–21 and TC-906 additions are planned, with no executable implementation claim. AC-1–15 are context only and remain unchanged. No applicable AssuranceProfile exists in this repository; direct review is applicable, with no enforced workflow claimed. Review set: all seven specification lenses plus the always-required code-review. Source was read only for feasibility; no Cargo, Kani or production tests were run. QSL FR-520 on origin/main was read independently as external context, including its outcome-map sum/max conservation rule and AC-11.

The kernel owns a disabled-by-default ten-counter interval primitive. QSL owns binder context, read presence, unattributed routing and its semantic accounting_version. The primitive alone does not establish QSL-1548 or FR-520-AC-11 conformance.

## Analysis

The diff changes only FR-358 and TC-906; no implementation, dependency, CI or copied code is introduced. Read accounting.rs general validation, commit_charge, observe_size, admit, charge_plan and accounting/units.rs. Both general and unit-only paths reach commit_charge after all validation; charge_plan has its own successful write path and must receive the same capture update. Eight raw validated size slots are available before global maxima are updated. A request of 3 after global 9 can therefore capture 3. Cumulative fields passed to commit_charge are new global totals, so implementation must derive actual work/result increments or pass those increments separately; blindly summing new totals would violate AC-17.

Feasibility: interval sums are bounded by the monotonic admitted global work/results counters; checked global additions bound each to u64::MAX. A successful plan has nonnegative u64 pairs and at least pairs+2 work availability, so its actual +1 fits. No wrapping, saturation loss or impossible-panic addition is required for capture. Size requests naming cumulative kinds are validated then superseded by actual work/results totals; capture must likewise exclude them from the eight-size projection. Cancellation can trip shared Cancel state, and injection can become spent on refusal; AC-18 properly freezes only interval state. Existing admission counters saturate independently; the capture adds no cap. No runtime faithfulness verdict is claimed for the planned API.

## Examined Scope

```yaml
scope:
- id: FR-358-AC-1
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: With a denial installed at occurrence 2 of point P, first refuse an over-limit
    P charge under an ordinary limit, then admit P once and an unrelated point once.
    The next P charge returns `Incomplete` at P with the work counter as its limit
    kind, proving the ordinary refusal did not advance the named occurrence; all consumed
    counters, admission count, and test-support log equal their values immediately
    before each refusal. A subsequent P charge succeeds and updates ordinary accounting
    and the log exactly once.
- id: FR-358-AC-2
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan`
    returns `Incomplete` at `equality.plan` without changing any consumed counter,
    admission count, or test-support log; a subsequent valid `charge_plan` succeeds
    and records its ordinary plan charge.
- id: FR-358-AC-3
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction
    with zero is rejected by that type, while a nonzero occurrence constructs a denial.
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor
    returns all 4096 records with their points in order and `charge_log_truncated()`
    is false. On the 4097th admission the slice still contains exactly that prefix
    and the flag becomes true.
- id: FR-358-AC-5
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Under `test-support`, after the 4097th admitted charge, `admission_count()`
    and consumed work units reflect all 4097 admissions, including the one omitted
    from the diagnostic log.
- id: FR-358-AC-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Without `test-support`, production `Meter` contains no admitted-charge
    log and its `Counters` accounting state remains fixed-size and heap-free.
- id: FR-358-AC-8
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Under `test-support`, admit 4097 charges each consuming one work and one
    result unit. The log retains the first 4096 points and reports truncation, while
    `admission_count()`, consumed work and consumed results each equal 4097.
- id: FR-358-AC-9
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: With work limit 4097 and a sufficient result limit under `test-support`,
    admit 4097 one-work-unit charges and request one more work unit. The charge returns
    `Incomplete` at its requested point with `WorkUnits`, `limit = consumed = 4097`,
    `next_charge = 1`; every counter, admission count, prefix log and truncation flag
    are unchanged.
- id: FR-358-AC-10
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: With result limit 4097 and a sufficient work limit under `test-support`,
    admit 4097 charges each consuming one result unit, then request one more result
    unit. The charge returns `Incomplete` with `ResultUnits`, `limit = consumed =
    4097`, `next_charge = 1`; all counters, admission count, prefix log and truncation
    flag remain unchanged, including the work counter.
- id: FR-358-AC-11
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: After 4097 admissions under `test-support`, inject an occurrence-1 denial
    for one named point. A charge at that point returns `Incomplete` with `WorkUnits`
    and leaves all counters, admission count, prefix log and truncation flag unchanged.
    Retrying that charge under sufficient ordinary limits succeeds once, increments
    work and admission count once, and does not extend the 4096-entry log.
- id: FR-358-AC-12
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 'After admitting two work units, inject occurrence 1 at point P and attempt
    a three-work-unit `charge` at P under otherwise sufficient limits whose configured
    work limits are 10 and 100 on separate meters. Both refusals report exactly `limit_kind
    = WorkUnits`, `charge_point = P`, `limit = consumed = 2`, and `next_charge = 3`,
    independently of either configured limit. Repeat with an injected `equality.plan`
    denial, `pairs = 2`, the same pre-consumed work and limits, and sufficient value-occurrence
    and result limits: both `charge_plan` refusals report `WorkUnits`, `charge_point
    = equality.plan`, '
- id: FR-358-AC-13
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: 'With an occurrence-1 injected denial selected, make the same `charge`
    independently fail an ordinary semantic-size counter and make a `charge_plan`
    with `pairs = 2` independently fail its work reservation under a configured work
    limit of 3 after two admitted work units, with all other limits sufficient. In
    each case, the injected `WorkUnits` record is returned before the ordinary refusal:
    `limit = consumed` equals the pre-charge work, `next_charge` is the charge''s
    work amount or the plan''s reservation respectively, and every consumed counter
    and admission count remain unchanged. The criterion '
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request
    of 9 followed by 3 and then a charge with no size request. The records expose
    respectively 9, 3 and no IntegerBits request while the consumed IntegerBits maximum
    remains 9. A separate charge with IntegerBits requests 9, 3 and zero exposes present
    9; a charge with any positive number of IntegerBits zero requests exposes present
    zero. Both have the same fixed payload size as a single-request record, with no
    heap-owned request collection and no retained request list, order or multiplicity.
    A successful `charge_plan` with pai
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently
    chosen semantic-size requests, including repeated same-kind requests and distinct
    amounts at admissions 4096 and 4097. The single record log contains exactly the
    first 4096 fixed-size records with their points and per-admission semantic-size
    maxima in admission order; truncation is false at admission 4096 and true at 4097.
    Work consumption and admission count reach 4097, including the omitted record;
    existing size maxima still include the 4097th charge.
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
- id: FR-358-Behavior-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: "The public, non-generic `Meter` SHALL expose `enable_charge_accounting(&mut\
    \ self)`, `take_charge_accounting(&mut self) -> Option<ChargeAccounting>` and\
    \ `disable_charge_accounting(&mut self) -> Option<ChargeAccounting>`.\n\n   Enabling\
    \ disabled capture SHALL start an empty interval without incorporating earlier\
    \ admissions.\n\n   Enabling already active capture SHALL preserve its unread\
    \ amounts.\n\n   Taking active capture SHALL return its current aggregate and\
    \ reset only the interval to empty while leaving capture enabled.\n\n   Taking\
    \ disabled capture SHALL return `None`.\n\n   Disabling active capture "
- id: FR-358-Behavior-7
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: "The public `ChargeAccounting` SHALL expose `amount(&self, kind: LimitKind)\
    \ -> u64` for all ten kinds.\n\n   Its private state SHALL contain fixed-size,\
    \ heap-free counters only, without charge points, ordered history, caller identities\
    \ or arbitrary metadata.\n\n   For each interval, `WorkUnits` and `ResultUnits`\
    \ SHALL sum the actual successfully committed increments.\n\n   Each of the eight\
    \ semantic-size kinds named in behavior 2 SHALL retain the maximum actual validated\
    \ request admitted during that interval, coalescing repeated same-kind requests.\n\
    \n   An absent request and an explicit zero SHALL bot"
- id: FR-358-Behavior-8
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: "While capture is active, every successful admission through ordinary `charge`,\
    \ its unit-only fast path and `charge_plan` SHALL contribute exactly once.\n\n\
    \   A successful `charge_plan` SHALL contribute work 1, results 0 and its actual\
    \ `ValueOccurrences` request, never its `pairs + 2` work reservation or result\
    \ availability check.\n\n   Ordinary refusals, injected denials, cancellation\
    \ and failed plan reservations SHALL leave the interval unchanged and preserve\
    \ the successful prefix.\n\n   Capture SHALL continue aggregating admissions after\
    \ diagnostic entry 4096, independently of diagnostic-log trunc"
- id: FR-358-Behavior-9
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: "Enabling, taking and disabling capture SHALL leave limits, consumed counters,\
    \ admission counts, cancellation state and injected-denial state unchanged.\n\n\
    \   Capture control operations SHALL charge no accounting overhead.\n\n   The\
    \ fixed-size capture SHALL introduce no allocation, request-count cap, admission\
    \ cap, callback, trait object, lifetime parameter or generic parameter into `Meter`.\n\
    \n   `Meter` SHALL retain `Clone`, `Debug`, `Eq`, `PartialEq`, `Send`, `Sync`\
    \ and no_std compatibility.\n\n   Cloning SHALL copy the enabled/disabled state\
    \ and unread aggregate as independent owned state.\n\n   The "
- id: TC-906-procedure-13
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: For AC-16, in a default-feature build exercise disabled take/disable, admissions
    before enable, enabled empty takes, repeated enable with unread work, disable
    and re-enable. Snapshot limits, global counters, admissions, cancellation and
    denial state around every control operation; follow with cancellation and the
    pending selected denial to verify their behavior is preserved.
- id: TC-906-procedure-14
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: For AC-17, use an independently written expected ten-counter schedule containing
    general charges and the unit-only fast path, distinct work/results, each of the
    eight size kinds, repeated requests, zeros and omissions. Compare every `amount(kind)`.
    Separately admit IntegerBits 9 before enabling, then admit 3 during capture and
    compare captured and global amounts.
- id: TC-906-procedure-15
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: For AC-18, on separate active meters retain a known successful prefix before
    each ordinary work/result/size refusal, injected denial, cancellation and failed
    plan work/result/size check; compare all ten interval amounts before and after.
    Retry where permitted and check one successful extension. Independently admit
    a plan with pairs 2 under sufficient limits and inspect its work, results and
    ValueOccurrences amounts.
- id: TC-906-procedure-16
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: 'For AC-19, replay the same explicit schedule on three meters with identical
    prior consumption: repeated-drain capture, uninterrupted capture and disabled
    capture. Include an empty drain, repeated enable and final disable. Recombine
    drained work/results by sum and sizes by maximum; compare against the independent
    schedule oracle, uninterrupted interval and unchanged global admission behavior.'
- id: TC-906-procedure-17
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: For AC-20, run 4097 one-work/one-result admissions in both default and
    test-support builds with a distinct largest size on the final admission. Read
    capture independently of the diagnostic accessor; under test-support also inspect
    the existing prefix and truncation flag at 4096 and 4097.
- id: TC-906-procedure-18
  path: spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: For AC-21, inspect production types and control/commit paths for fixed-size
    heap-free capture with private fields, no stored history or caller metadata, no
    callbacks/trait objects and no new Meter generic/lifetime parameters. Check default
    construction and active interval operations for allocations, and verify the unchanged
    no_std and public trait contracts. Clone an active nonempty meter, drain one copy
    and confirm the other's unread interval remains intact.
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

PASS for this specification-only candidate. Implementation and downstream acceptance remain pending.
