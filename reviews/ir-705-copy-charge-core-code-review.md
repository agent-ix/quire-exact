---
id: SR-15254
title: IR-705 Copy charge core Rust and code review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; PR80; src/accounting.rs, src/accounting/{units.rs,units_reference.rs,units_proofs.rs,units_tests.rs,kani_proofs.rs},
  src/native_integer.rs, src/numeric.rs, scripts/meter_kani_gate.py, scripts/meter_kani_scope.sh,
  tests/meter_kani_gate.rs, tests/ir705_native_integer.rs; Cargo/Makefile/CI and owning
  FR358/359/371/383/384
review_set: subset
---

## Summary

Reviewed exact clean frozen 496a2f08db1a0e3382c551b3bdd996571fc3dc5f against merged main 2cc14. The private Copy units path preserves the inspected accounting semantics and actual real Meter; two conceptual defects require a same-PR fix: the canonical ALL-loop gate now mutates an unreachable fallback, and three new trace bindings claim a borrowed accessor they do not call. Full native proof acceptance and consumer fit are separate.

## Verdict

FAIL — FND-001 makes the documented mandatory mutation gate unusable on the new routing path. FND-002 through FND-004 are incorrect trace bindings. No other source semantic defect identified in the examined production/core/model/test scope.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The canonical ALL-traversal mutation still wraps charge.sizes.iter in accounting.rs, which now resides in charge_general. The unchanged meter schedule harness uses only zero/one/two u64 requests and therefore dispatches every case through charge_units, bypassing the mutated fallback. The documented make kani-meter-gate control consequently cannot produce its required mutated-loop unwinding failure. Retarget the canonical mutation and location classification to the actual selected common production validation path, preserve the original healthy harness/domain/settings, and qualify the actual canonical gate rather than replacing it with ad-hoc controls. | scripts/meter_kani_gate.py:87-111 |
| FND-002 | medium | units_injection_cancellation_and_saturation_match_general claims FR-383-AC-1 but never calls Meter::cancel and cannot detect a broken borrowed-accessor None/identity/replacement result. Its real assertions concern cancellation/accounting. Remove this incorrect binding or bind only criteria actually asserted, preserving the behavioral test. | src/accounting/units_tests.rs:215-216 |
| FND-003 | medium | units_boolean_retention_preserves_real_meter_stops claims FR-383-AC-1 but never calls Meter::cancel and cannot detect a broken borrowed-accessor None/identity/replacement result. Its real assertions concern cancellation/accounting. Remove this incorrect binding or bind only criteria actually asserted, preserving the behavioral test. | src/accounting/units_tests.rs:276-277 |
| FND-004 | medium | units_and_adapter_poll_cancel_exactly_once_per_attempt claims FR-383-AC-1 but never calls Meter::cancel and cannot detect a broken borrowed-accessor None/identity/replacement result. Its real assertions concern cancellation/accounting. Remove this incorrect binding or bind only criteria actually asserted, preserving the behavioral test. | src/accounting/units_tests.rs:308-309 |

## Coverage and evidence boundary

Read actual source and owning criteria, not ticket claims. The core is Copy/private with only u64 payloads. Native arithmetic/ordering/Boolean retention call the same actual Meter core; eligible public Charge adapts to it and unrestricted/negative/above-u64 requests retain the general path. Polling and final atomic commit are shared without a cfg replacement body, assumption, new dependency, public API or changed charge schedule.

The independent transition oracle names field-order failures and high-water/cumulative updates separately from production request-order scanning. Full-width direct/adapter harnesses draw all ten limits/counters, arbitrary u64 requests, valid zero/one/two shapes and all ten kinds. The injection proof uses arbitrary full ten-counter state, preserves all eight high-water counters, checks complete denial records and occurrence/cancellation/saturation state, with an actual pending-injection ordinary-refusal cover. The consultation gaps are fixed. No narrowing assumption or impossible cover was identified. Scopes remain explicit: ordinary transitions are universal without injection, while the separate stop-state proof has zero semantic requests. Native/support tests cover the actual ordered log and mixed common/public/general prefixes.

Five healthy proofs have actual source-bound successful receipts reported in the freeze-2 ledger; remaining canonical and substantive mutation controls are not yet qualified. A healthy proof alone cannot establish mutant sensitivity. No final aggregate or verifier command was run by this reviewer. The lead owns final aggregate after findings are fixed. Earlier pure numerical Integer/native/SMT refinement may carry only with exact dependency identity; the changed actual Meter/order integration has new-subject evidence and cannot be relabeled as an old-head theorem. Genuine IR917 still has MemoryExhausted on the prior consumer run. No 16 GiB consumer fit or whole IR-705 completion is claimed.

## Examined scope and bindings

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
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362
    schedule and counters on native, native-overflow and arbitrary-size cases. Test
    sufficient limits, independently calculated exact/one-less bit and work boundaries,
    denial injected at every named point, and outside-domain results. Compare all
    completed/refused/incomplete fields and admitted prefixes with the independent
    normative formulas. No representation transition manufactures a FunctionCall or
    other charge.
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
- id: FR-384-AC-3
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Fresh and cumulative real Meter comparisons against the normative formulas
    and general exact calls agree on completed values, domain refusal, cancellation,
    every named injected denial, all counters, admitted prefix and complete incomplete
    fields at sufficient, exact and one-less limits. Every operation and Boolean/comparison
    consumer used by the genuine profiles is exercised; arithmetic equivalence alone
    does not satisfy accounting refinement.
bindings:
- test_id: units_charge_matches_independent_boundary_model
  ac_id: FR-359-AC-1
  trace: correct
- test_id: units_charge_matches_independent_boundary_model
  ac_id: FR-359-AC-2
  trace: correct
- test_id: units_adapter_keeps_exact_and_unrestricted_fallback
  ac_id: FR-359-AC-3
  trace: correct
- test_id: units_adapter_keeps_exact_and_unrestricted_fallback
  ac_id: FR-359-AC-4
  trace: correct
- test_id: units_charge_matches_independent_boundary_model
  ac_id: FR-359-AC-5
  trace: correct
- test_id: units_charge_matches_independent_boundary_model
  ac_id: FR-359-AC-6
  trace: correct
- test_id: units_charge_matches_independent_mixed_state_model
  ac_id: FR-359-AC-6
  trace: correct
- test_id: units_injection_cancellation_and_saturation_match_general
  ac_id: FR-359-AC-6
  trace: correct
- test_id: units_boolean_retention_preserves_real_meter_stops
  ac_id: FR-359-AC-6
  trace: correct
- test_id: units_public_and_fallback_share_one_meter_and_pending_denial
  ac_id: FR-359-AC-6
  trace: correct
- test_id: units_charge_matches_independent_mixed_state_model
  ac_id: FR-359-AC-7
  trace: correct
- test_id: units_charge_matches_independent_boundary_model
  ac_id: FR-359-AC-8
  trace: correct
- test_id: units_charge_matches_independent_mixed_state_model
  ac_id: FR-359-AC-8
  trace: correct
- test_id: units_adapter_keeps_exact_and_unrestricted_fallback
  ac_id: FR-359-AC-8
  trace: correct
- test_id: units_injection_cancellation_and_saturation_match_general
  ac_id: FR-359-AC-8
  trace: correct
- test_id: units_boolean_retention_preserves_real_meter_stops
  ac_id: FR-359-AC-8
  trace: correct
- test_id: units_and_adapter_poll_cancel_exactly_once_per_attempt
  ac_id: FR-359-AC-8
  trace: correct
- test_id: units_public_and_fallback_share_one_meter_and_pending_denial
  ac_id: FR-359-AC-8
  trace: correct
- test_id: units_public_and_fallback_share_one_meter_and_pending_denial
  ac_id: FR-359-AC-12
  trace: correct
- test_id: units_injection_cancellation_and_saturation_match_general
  ac_id: FR-383-AC-1
  trace: wrong
- test_id: units_boolean_retention_preserves_real_meter_stops
  ac_id: FR-383-AC-1
  trace: wrong
- test_id: units_and_adapter_poll_cancel_exactly_once_per_attempt
  ac_id: FR-383-AC-1
  trace: wrong
- test_id: units_public_and_fallback_share_one_meter_and_pending_denial
  ac_id: FR-383-AC-1
  trace: correct
```

## FND-001 canonical gate repair detail

FND-001 canonical gate repair must update every coupled authority: mutate the actually selected common production path; pin the classifier to the observed new function/file/mutant line and exact unwinding property/index without accepting arbitrary failures; include src/accounting/units.rs in the persisted source.patch rather than only accounting.rs. Also bring reject_unrelated_heavy_processes into the current direct user policy: serialize Kani/CBMC/goto drivers, while allowing ordinary Cargo/rustc/clippy/rustfmt and native tests in their own bounded scopes. The existing guard rejects those ordinary processes and every target/deps binary, so it still blocks permitted concurrent work. The lead will invoke the canonical gate under /tmp/agent-e-heavy-build.lock; this finding requires no global Make policy change, security/account changes or weakened proof checks. Canonical healthy+ALL-loop control must actually run after the fix, with original settings and complete source/resource/teardown evidence.

## Dispositions

Round 1 at 2bacb3af84debce437e174d6e083d18bf326877d. Original finding text is preserved.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Canonical mutation now targets the actual common production request validation in src/accounting/units.rs. The actual canonical binding ran one test with no filter loss and passed: healthy 0/2763 failed, and the ALL-loop control failed exactly accounting::units::<impl accounting::Meter>::charge_units.unwind.0 at src/accounting/units.rs:98:9, description unwinding assertion loop 0. Its remaining 2790 checks are UNDETERMINED, not proved. Strict classifier, complete units.rs source.patch and Kani-only competing-process guard all inspected; ordinary bounded builds/tests remain allowed. Both proof scopes enforce16GiB/swap0/unwind3/Cadical/600s with zero memory events, complete teardown and source integrity. The recorded through-proof-return memory.peak values are 11829903360B and10399846400B; the separate native wrapper sampled263139328B is only a lower bound because final systemd MemoryPeak is unavailable. |
| FND-002 | fixed | units_injection_cancellation_and_saturation_match_general: only the incorrect borrowed-accessor tag removed; behavioral assertions unchanged and real accounting bindings retained. The separate mixed-prefix test retains its valid meter.cancel identity observation. |
| FND-003 | fixed | units_boolean_retention_preserves_real_meter_stops: only the incorrect borrowed-accessor tag removed; behavioral assertions unchanged and real accounting bindings retained. The separate mixed-prefix test retains its valid meter.cancel identity observation. |
| FND-004 | fixed | units_and_adapter_poll_cancel_exactly_once_per_attempt: only the incorrect borrowed-accessor tag removed; behavioral assertions unchanged and real accounting bindings retained. The separate mixed-prefix test retains its valid meter.cancel identity observation. |

## Round 1 verdict

PASS — all four code findings fixed; complete common-core qualification remains held under SR-15255 FND-005 pending the four substantive controls.

Canonical mutation now targets the actual common production request validation in src/accounting/units.rs. The actual canonical binding ran one test with no filter loss and passed: healthy 0/2763 failed, and the ALL-loop control failed exactly accounting::units::<impl accounting::Meter>::charge_units.unwind.0 at src/accounting/units.rs:98:9, description unwinding assertion loop 0. Its remaining 2790 checks are UNDETERMINED, not proved. Strict classifier, complete units.rs source.patch and Kani-only competing-process guard all inspected; ordinary bounded builds/tests remain allowed. Both proof scopes enforce16GiB/swap0/unwind3/Cadical/600s with zero memory events, complete teardown and source integrity. The recorded through-proof-return memory.peak values are 11829903360B and10399846400B; the separate native wrapper sampled263139328B is only a lower bound because final systemd MemoryPeak is unavailable.

All production/helper/proof/manifest/lock dependencies are byte-equal to the five successful healthy496a receipts. Candidate delta is only canonical gate script/tests, trace tags and exact initial review custody. Numerical theorem carry-forward is an explicit dependency-equivalence claim, not a claim of new-head numerical reruns. No Cargo/Kani/gate job was run by reviewer.

## Final evidence checkpoint

PASS at 2bacb3af84debce437e174d6e083d18bf326877d. All four original code findings remain fixed, and all remaining substantive common-core controls now have independently checked actual semantic counterexamples/native failing-restored pairs. No additional code or test issue found. Latest complete evidence closes SR-15255 FND-005; the component is review-mergeable subject only to review custody import and the lead final ordinary aggregate.

Actual control details (all proof settings16GiB/swap0/Cadical/unwind3/600s, zero memory events, complete scopesGone/sourceStable):

| Production control | Proof result | Verification seconds | Final proof peak B | Final native failing/restored peaks B |
| --- | --- | --- | --- | --- |
| Wrong field priority | 1/2909 failed; 4 covers | 60.531868 | 9381089280 | 494374912 / 498556928 |
| Replace first failing duplicate | 1/2909 failed; 4 covers | 61.091534 | 9387487232 | 493977600 / 490311680 |
| Commit size before cumulative validation | 1/2911 failed; 4 covers | 60.985455 | 9942237184 | 494362624 / 498118656 |
| Reset nonzero high-water at entry | 1/2541 failed; 4 covers | 61.684925 | 7996887040 | 492990464 / 495222784 |

The native failures reproduce the intended assertions: wrong WorkUnits versus ValueOccurrences at units_proofs.rs102; wrong exact next_charge at105; modified IntegerBits counter at107; reset MAX high-water tozero at212. Each native failure has innerCargo101/outerKani1 and exactly one failed test; each identical restored witness has inner/outer0 and exactly one passed test. No filtered-zero-test result or cover-generated witness is accepted.

The first field-priority native failure was configured with an unused1800-second ordinary wrapper ceiling and completed6.87seconds; the root explicitly carried this deterministic semantic falsification with the deviation retained. It is not claimed as a600-configured native run or backend qualification relaxation. The actual CBMC proof stayed600, and all remaining native playbacks were configured600. Preliminary sampled native peaks were superseded by the final recorded values above. The canonical gate's separate native wrapper sampled263139328B remains only a lower bound; its unavailable final systemd MemoryPeak is not invented.

Production is fully restored to the clean reviewed2bac candidate after all cases. The five healthy proof receipts remain their actual496a execution-head results supported by byte-identical production/proof dependency closure; canonical and mutation receipts retain actual2bac source/provenance, reconstructing deliberate mutations with their source.patch. Pure numerical NativeInteger/Integer/SMT theorems retain their earlier actual heads and only verified unchanged numerical dependency closure; the new public ordering/Meter proof is separately qualified. No proof/ordinary aggregate job was run by the reviewer. No C2 timeout, previous CG917 MemoryExhausted result or sampled peak is relabeled as success.
