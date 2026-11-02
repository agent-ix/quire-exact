---
id: SR-2026110101300
title: "IR-1268 code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; IR-1268; src/, spec/; PR 125 frozen 4c1df8e"
review_set: subset
---

## Summary

Independent code-review of agent-ix/quire-exact PR #125, ticket IR-1268, frozen head 4c1df8eeb896e470d7192ceb6a0020041f0e288b against 87e40238870b80ec4fd1bf17a69aeb8edec5d1ae. No applicable AssuranceProfile exists. Source is read-only; no Cargo or backend execution was performed by this reviewer.

## Verdict

**PASS**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Rust-review ran as the code-review lane. Both replacements construct exactly the fields previously extracted by UnitsCharge::from_charge: ordered IntegerBits then ValueOccurrences, default work 1/results 0; retention work 1/results 1. Charge::size preserves zero and all u64 values; there is no normalization lost. All callers supply u64 amounts directly: widths 32/64, operation arity 1/2/3, or ExactScalar::operand_bits over Integer/Rational/retained Decimal. No narrowing assumption is needed. Units admission, injected_denial and commit_charge remain unchanged, preserving native public Incomplete/Outcome construction, first-field refusal, atomic commits, high-water, occurrence tracking, cancellation and log semantics.

The one-line AtomicU32 rename preserves callback and Relaxed orderings. Existing native tests and new public IEEE versus charge_general comparisons cover off/on test-support, boundary/overflow/high-water, injection attempts, cancellation original handle/polls and log cap. The independent reference calls general admission directly, not Meter::charge or the changed helper. Arithmetic arities, zero exact conversion and width conversion use real public operations. No duplicated production rule, public API, dependency, source fixture, suppression, gate removal or cfg bypass was added. Test reference schedules are expected-value oracles over the existing independent path.

Reviewed author receipts /tmp/ir1268-final-focused.sh and .log: fmt, strict all-target/all-feature Clippy, MSRV 1.98 check and focused native cancellation 7/7, new parity 4/5, units 7/7, IEEE 13/13 pass. Aggregate native and hosted final-head gates are pending the lead; this reviewer did not rerun Cargo as instructed.

Compile-only evidence was checked against separate coherent before/after logs and GOTO artifacts, same QSV 25535387 and unchanged RT head 67780bb. Linked bytes 2396381 to 1521102; defined bodies 900 to 597; general-charge-related bodies 20 to 0; BigInt 39 to 35, NodeData drop remains 1. The measurements distinguish symbol IDs and defined bodies, and timings are explicitly not benchmarks. Read the unchanged full-u32 kani::any proposition, independent NaN masks, required successful Completed, native Meter and unwind 8. No backend was rerun and no proof credit is earned; original memory-exhausted proof remains unresolved. Build-only warnings/unsupported constructs are not solver success. Temporary dependency restoration is evidenced by clean RT and Exact status, not by a committed dependency pin.

Reconciliation: quoin matrix 0.29.1 and quire matrix — no run evidence.
Plan completion: not assessed

## Examined Scope

```yaml
scope:
- id: FR-358-AC-1
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 2 of point P, first refuse an over-limit
    P charge under an ordinary limit, then admit P once and an unrelated point once.
    The next P charge returns `Incomplete` at P with the work counter as its limit
    kind, proving the ordinary refusal did not advance the named occurrence; all consumed
    counters, admission count, and test-support log equal their values immediately
    before each refusal. A subsequent P charge succeeds and updates ordinary accounting
    and the log exactly once.
- id: FR-358-AC-2
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With a denial installed at occurrence 1 of `equality.plan`, the first `charge_plan`
    returns `Incomplete` at `equality.plan` without changing any consumed counter,
    admission count, or test-support log; a subsequent valid `charge_plan` succeeds
    and records its ordinary plan charge.
- id: FR-358-AC-3
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: The public `InjectedDenial::occurrence` field has type `NonZeroU64`; construction
    with zero is rejected by that type, while a nonzero occurrence constructs a denial.
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor
    returns all 4096 records with their points in order and `charge_log_truncated()`
    is false. On the 4097th admission the slice still contains exactly that prefix
    and the flag becomes true.
- id: FR-358-AC-5
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after the 4097th admitted charge, `admission_count()`
    and consumed work units reflect all 4097 admissions, including the one omitted
    from the diagnostic log.
- id: FR-358-AC-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Without `test-support`, production `Meter` contains no admitted-charge
    log and its `Counters` accounting state remains fixed-size and heap-free.
- id: FR-358-AC-8
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 charges each consuming one work and one
    result unit. The log retains the first 4096 points and reports truncation, while
    `admission_count()`, consumed work and consumed results each equal 4097.
- id: FR-358-AC-9
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With work limit 4097 and a sufficient result limit under `test-support`,
    admit 4097 one-work-unit charges and request one more work unit. The charge returns
    `Incomplete` at its requested point with `WorkUnits`, `limit = consumed = 4097`,
    `next_charge = 1`; every counter, admission count, prefix log and truncation flag
    are unchanged.
- id: FR-358-AC-10
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: With result limit 4097 and a sufficient work limit under `test-support`,
    admit 4097 charges each consuming one result unit, then request one more result
    unit. The charge returns `Incomplete` with `ResultUnits`, `limit = consumed =
    4097`, `next_charge = 1`; all counters, admission count, prefix log and truncation
    flag remain unchanged, including the work counter.
- id: FR-358-AC-11
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: After 4097 admissions under `test-support`, inject an occurrence-1 denial
    for one named point. A charge at that point returns `Incomplete` with `WorkUnits`
    and leaves all counters, admission count, prefix log and truncation flag unchanged.
    Retrying that charge under sufficient ordinary limits succeeds once, increments
    work and admission count once, and does not extend the 4096-entry log.
- id: FR-358-AC-12
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
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
  role: examined
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
  role: examined
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
  role: examined
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently
    chosen semantic-size requests, including repeated same-kind requests and distinct
    amounts at admissions 4096 and 4097. The single record log contains exactly the
    first 4096 fixed-size records with their points and per-admission semantic-size
    maxima in admission order; truncation is false at admission 4096 and true at 4097.
    Work consumption and admission count reach 4097, including the omitted record;
    existing size maxima still include the 4097th charge.
- id: FR-359-AC-1
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX -
    1` followed by one work unit reaches `u64::MAX` exactly. The next one-unit charge
    returns `Incomplete` at its named point with `limit_kind = WorkUnits`, `limit
    = consumed = u64::MAX`, and `next_charge = 1`; all consumed counters and admission
    count stay at their pre-refusal values.
- id: FR-359-AC-2
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With `work_units = result_units = u64::MAX`, admit one charge requesting
    `u64::MAX` result units. Its work consumption is 1. A second charge requesting
    one result unit returns `Incomplete` at its named point with `limit_kind = ResultUnits`,
    `limit = consumed = u64::MAX`, and `next_charge = 1`; work remains 1 and admission
    count remains 1.
- id: FR-359-AC-3
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge whose exact amount
    is `u64::MAX + 1` returns `Incomplete` with `limit_kind = WorkUnits` and `next_charge
    = u64::MAX + 1`; no counter or admission count changes.
- id: FR-359-AC-4
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a charge with one work unit
    and `u64::MAX + 1` result units returns `Incomplete` with `limit_kind = ResultUnits`
    and `next_charge = u64::MAX + 1`; no counter or admission count changes.
- id: FR-359-AC-5
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: First admit an `integer_bits` high-water size of 8 and one result unit
    under limits of at least 16 bits, `u64::MAX` work units and one result unit. Then
    attempt a charge carrying `integer_bits = 16` and one result unit. The result
    refusal leaves `integer_bits = 8`, work units, result units and admission count
    at their pre-refusal values; under `test-support`, the admitted-charge log is
    unchanged too.
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With no injection installed, independently make each of the ten counters
    the first unavailable counter of a charge, including several simultaneously short
    counters and semantic sizes attached in either order. The reported `limit_kind`
    follows `ScalarLimits` field order, with the exact limit, previously consumed
    amount, requested amount and point. A refusal after an earlier size check succeeds
    changes no counter, admission count or test-support log. A separate meter with
    a pending later named denial proves that this ordinary refusal does not advance
    its occurrence.
- id: FR-359-AC-7
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'On fresh, charged and stopped meters, `Meter::consumed(kind)` answers
    without panic for every member of `LimitKind::ALL`: untouched counters are zero,
    semantic sizes are admitted high-water maxima and work/results are admitted cumulative
    totals.'
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the admitted high-water maximum
    observable through `consumed()`; the diagnostic log record is verified under FR-359-AC-12.
    With `IntegerBits` limit 5 and requests 7 then 9 for that kind in one charge,
    refusal reports `next_charge = 7`, the first-inserted failing amount; the existing
    9 then 7 case alone cannot distinguish maximum coalescing before refusal. Simultaneous
    shortages refuse in `ScalarLimits` field order; ordinary refusal preserves counters,
    high-water, admissions, log and a pending injected occurrence. Cancellation and
    injection precedence, c
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference`
    uses production `Meter::charge` for the exact three-charge FR-362 schedule (operand
    charge with at most two semantic-size requests, arithmetic charge with one, result-retain
    charge with none) over the full `i64` operand range, plus separate repeated-kind
    charge controls with at most two requests each. Symbolic `i64` operands and symbolic
    limits including `ResultUnits` cover first-short and simultaneous shortages and
    absence of the third admission. Under `--unwind 3 --solver cadical`, 16 GiB and
    600 seconds, the basel
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests
    below pass their own assertions under their original domains, oracle, completion
    and counterexample checks, unwind 3, Cadical, 16 GiB and 600 seconds. Record their
    individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test
    or altered consumer harness substitutes for it.
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and
    called helpers finds no traversal with a trip count greater than that charge's
    semantic-size request count, including no traversal of `LimitKind::ALL` or another
    independently sized collection. The FR-362 proof schedule has at most two requests
    per charge, while the public `Charge` API retains no request-count ceiling.
- id: FR-359-AC-12
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Under `test-support`, the admitted-charge log for the three-charge FR-362
    schedule contains exactly the admitted prefix in charge order, with each record's
    point and absent-or-present per-admission semantic-size maxima matching an independent
    model. Reordered and repeated requests of the same kind, including zero and distinct
    amounts, coalesce to the per-admission maximum in that record. A refused charge
    appends no record and leaves the existing log unchanged. The named `tc_931_admitted_log_matches_model_ordered_prefix`
    and `tc_931_admitted_log_matches_model_same_kind` tests verify these prope
- id: FR-383-AC-1
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: '`Meter::new(limits).cancel()` is `None`. After `with_cancel(handle.clone())`,
    `cancel()` is `Some(&Cancel)` equal by shared-state identity to `handle`, and
    unequal to an independently created handle. Replacing the handle reports only
    the replacement.'
- id: FR-383-AC-2
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: 'Cancel the handle obtained from `meter.cancel()` with `Requested`, then
    charge the meter under sufficient limits: the charge is denied at its named point,
    that handle records `tripped() = Some(Requested)`, and all accounting state stays
    unchanged. On a separate fresh meter, cancel an independent handle while its installed
    handle remains live: an otherwise identical charge is admitted.'
- id: FR-383-AC-3
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The accessor's implementation borrows the existing optional handle without
    cloning it or constructing a second `Meter` or `Counters`; calling it changes
    no meter accounting or injection state.
- id: FR-383-AC-4
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles
    in the crate's `no_std` + `alloc` build.
- id: src/accounting.rs
  path: src/accounting.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: src/accounting/ieee_units_tests.rs
  path: src/accounting/ieee_units_tests.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
- id: src/cancel.rs
  path: src/cancel.rs
  role: examined
  excerpt: .try_update(Ordering::Relaxed, Ordering::Relaxed, |charges| {
- id: src/ieee.rs
  path: src/ieee.rs
  role: examined
  excerpt: 'fn charge_operand_bits(meter: &mut Meter, integer_bits: u64, arity: u64)
    -> Result<(), Stop> {'
- id: src/accounting/units.rs
  path: src/accounting/units.rs
  role: examined
  excerpt: '// SPDX-License-Identifier: AGPL-3.0-or-later'
```
