---
id: SR-15256
title: IR-705 Copy charge core base
type: SpecReview
analysis: base
scope: agent-ix/quire-exact; PR80; frozen496a2f08 versus merged2cc14; full computed
  repository matrix plus actual src/accounting.rs, units core/reference/proofs/tests,
  native integer and Boolean meter routing; owning FR358/359/371/383/384 and AD003;
  canonical gate scripts and trace bindings
review_set: subset
---

## Summary

Private common-charge representation and real Meter routing conform to the unchanged inspected source-level API/accounting contract. The canonical gate/control repair and incorrect trace bindings are tracked explicitly by SR-15254 and SR-15255; this base PASS is not qualification or merge acceptance.

## Verdict

PASS — No additional normative API/semantic design mismatch found in the source-contract scope. Independent evidence gates remain blocked until the code findings and qualification are complete.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

Independently read the five actual freeze-2 raw successful logs and summaries, exact source archives, caps and teardown records at 496a2f08db1a0e3382c551b3bdd996571fc3dc5f. Direct transition: 0/2909 failed, four covers, 27.3154s; public adapter: 0/2910 failed, four covers, 69.79147s; injection/cancellation state: 0/2533 failed, four covers, 17.933363s; original Meter schedule: 0/2763 failed, 186.32742s; public native ordering: 0/1230 failed, one cover. The largest reported cgroup peak is 11,824,492,544 B for the schedule proof, under actual16GiB/swap0 with zero memory events and complete teardown. This is kernel selected-proof fit, not genuine CG consumer fit. No verifier or aggregate rerun by reviewer.

The first two proofs quantify arbitrary full ten-counter limits/consumption and u64 amounts over valid zero/one/two request shapes and all ten counter kinds against a separately field-ordered transition model. The stop-state proof quantifies arbitrary ten-counter prestate, cancellation and named occurrence/saturation while using zero semantic requests and maximal configured limits. It preserves all eight high-water counters on every branch. These scopes compose with actual poll-first/shared-commit source; they are not advertised as one universal combined injection/requests/log theorem. Ordered-log behavior remains native test-support evidence, including existing 4096-entry/truncation tests now routed through public adapter/common path.

The general fallback remains unrestricted, including negative and above-u64 exact requests. Its shared poll/commit extraction retains the existing atomic semantics and exact Incomplete payloads. The private Copy common representation introduces no public API, altered normative schedule, dependency, cfg production replacement, copied evaluator or added proof assumption. Pure numerical NativeInteger plan/admit/calculate/capacity/Math/SMT results may carry only with exact dependency-closure identity. The actual native evaluator's Meter call path changed, so fresh transition, schedule and public ordering receipts are explicitly new-subject evidence; old PR78 logs are not relabeled. Actual native arithmetic/Boolean/default/support differential tests remain required alongside these scoped proofs.

Production is owned by FR-359/358/371/383/384; reverse-gap and stub scan found no other new unspecified behavior or hollow test beyond the three reported incorrect trace bindings. No plan was supplied; none was created. The full computed matrix is the engine's output, not a handwritten table. Direct user staging requires kernel implementation/gates before consumer qualification; inherited and future consumer findings retain concrete ownership and do not waive program obligations.

No requirement/plan text changes in PR80; EARS and structural specification-authoring sub-analyses are inapplicable. Base scope distinguishes unchanged normative semantics from current canonical proof-lane nonconformance already reported in code/gap findings.

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
  role: context_only
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
- test_id: units_public_and_fallback_share_one_meter_and_pending_denial
  ac_id: FR-383-AC-1
  trace: correct
```

## Round 1 source-contract checkpoint

PASS at 2bacb3af84debce437e174d6e083d18bf326877d. Actual canonical gate now exercises the common production path with the original healthy harness/settings, strict exact mutant classification and complete source patch provenance. The guard follows current Kani-only serialization while ordinary bounded workloads remain permitted. Incorrect borrowed-accessor trace claims are removed without changing assertions. Production, reference/proof helpers, arithmetic contracts, manifest/lock and numerical dependency closure are unchanged. Five healthy proof receipts retain their actual496a execution identity; canonical healthy/ALL-loop receipts belong to the actual new head. Other2790ALL-loop checks remain UNDETERMINED. Four substantive controls still pending under SR15255 FND005, so this source-contract PASS is not final qualification or consumer fit.

## Final component conformance checkpoint

PASS at 2bacb3af84debce437e174d6e083d18bf326877d. No further source change since the reviewed canonical/trace fix. All actual healthy/semantic-control/native witnesses now support the unchanged normative accounting contract, including field priority, first failing duplicate, atomic commit and preservation of nonzero high-water state. No private numerical, public arithmetic, refusal, cancellation, Meter or diagnostic-log specification was weakened. Independent reference and full-domain proof populations remain intact; no new public API or specification amendment is needed. Kernel component is review-mergeable after custody import and the lead final aggregate, while inherited and future IR-705 consumer obligations remain open.

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
