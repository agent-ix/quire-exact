---
id: SR-14202
title: spec-review/base review of IR-844 meter completion
type: SpecReview
analysis: base
scope: agent-ix/quire-exact; src/accounting.rs, src/accounting/kani_proofs.rs, spec/functional/FR-359-cumulative-meter-boundary.md,
  spec/test-cases/TC-931-single-pass-meter-accounting.md
review_set: subset
---

## Summary

Frozen head a4f22a9a5ee9d05dfcb5ab5ca3b8836c89bd59b0; base 441407020d76b8e9b63d1985701bbe78caaf2bfe. Ticket: IR-844. PR: agent-ix/quire-exact#65.

Two new recorded source revision hashes violate the explicit base spec-review checklist. Measured evidence and follow-up boundaries otherwise read consistently.

## Verdict

FAIL — Two new recorded source revision hashes violate the explicit base spec-review checklist. Measured evidence and follow-up boundaries otherwise read consistently.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The Status paragraph records a source revision hash inside the requirement; remove the recorded revision from spec prose and retain source identity in git and operational review evidence. | spec/functional/FR-359-cumulative-meter-boundary.md:67 |
| FND-002 | high | The Status paragraph records a source revision hash inside the test case; remove the recorded revision from spec prose and retain source identity in git and operational review evidence. | spec/test-cases/TC-931-single-pass-meter-accounting.md:31 |

## Coverage

Reviewer model identity: GPT-6; the harness does not expose a more specific SKU. Markers use `gpt-6`.

Base review applies to the two existing FR/TC evidence/status/procedure documents. The dispatch selected applicable review for wording and evidence updates, with no new public requirement behavior. No applicable AssuranceProfile exists. No new requirement statement, structural artifact, domain object or relationship edge was introduced, so ears, integrity, object and dependency sub-analyses are not applicable. Evidence-method catalog, risk allocation and system boundary are unchanged; no additional lens is required. Full changed-document validation passed with the installed quire CLI; the CLI emitted existing module duplicate/archetype diagnostics. Optional Jev criterion-strength analysis was not selected or needed for unchanged ACs.

Measured baseline, failed first observer attempt, authorized 40 GiB observation, executed native witness and unchanged healthy 16 GiB acceptance ceiling are distinguished. Source and raw logs support the recorded checks, time, peak and restoration claims. The observer drops baseline slice-formula; the prose records that confounder and makes no historical memory-dominator claim. The executable AC-9 binding/persistent gate remains IR-810; the consumer AC-10 receipt remains IR-17 and UNRUN. Trace attributes are explicitly confined to actual tests. Procedure bounds remain all original operands, symbolic quantities, six attempts and full refusal/counter oracle. No weakened domain, fake tag or claim that measured proof closes computed coverage is introduced. The source revision belongs to git and operations evidence, not requirement/test-case prose; exact tool versions here report the measured run rather than mandate a toolchain pin. Existing consumer receipts serve the explicit proof/content trust feature and are context, not a new tracking system.

## Examined Scope

```yaml
scope:
- id: FR-359-AC-1
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge of `u64::MAX - 1` followed by one work unit
    reaches `u64::MAX` exactly. The next one-unit charge returns `Incomplete` at its named point with `limit_kind
    = WorkUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; all consumed counters and admission count
    stay at their pre-refusal values.
- id: FR-359-AC-2
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With `work_units = result_units = u64::MAX`, admit one charge requesting `u64::MAX` result units. Its
    work consumption is 1. A second charge requesting one result unit returns `Incomplete` at its named point with
    `limit_kind = ResultUnits`, `limit = consumed = u64::MAX`, and `next_charge = 1`; work remains 1 and admission
    count remains 1.
- id: FR-359-AC-3
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a work charge whose exact amount is `u64::MAX + 1` returns
    `Incomplete` with `limit_kind = WorkUnits` and `next_charge = u64::MAX + 1`; no counter or admission count changes.
- id: FR-359-AC-4
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With both cumulative limits at `u64::MAX`, a charge with one work unit and `u64::MAX + 1` result units
    returns `Incomplete` with `limit_kind = ResultUnits` and `next_charge = u64::MAX + 1`; no counter or admission
    count changes.
- id: FR-359-AC-5
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: First admit an `integer_bits` high-water size of 8 and one result unit under limits of at least 16 bits,
    `u64::MAX` work units and one result unit. Then attempt a charge carrying `integer_bits = 16` and one result
    unit. The result refusal leaves `integer_bits = 8`, work units, result units and admission count at their pre-refusal
    values; under `test-support`, the admitted-charge log is unchanged too.
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: With no injection installed, independently make each of the ten counters the first unavailable counter
    of a charge, including several simultaneously short counters and semantic sizes attached in either order. The
    reported `limit_kind` follows `ScalarLimits` field order, with the exact limit, previously consumed amount,
    requested amount and point. A refusal after an earlier size check succeeds changes no counter, admission count
    or test-support log. A separate meter with a pending later named denial proves that this ordinary refusal does
    not advance its occurrence.
- id: FR-359-AC-7
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'On fresh, charged and stopped meters, `Meter::consumed(kind)` answers without panic for every member
    of `LimitKind::ALL`: untouched counters are zero, semantic sizes are admitted high-water maxima and work/results
    are admitted cumulative totals.'
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the admitted high-water maximum observable through `consumed()`;
    the diagnostic log record is verified under FR-359-AC-12. With `IntegerBits` limit 5 and requests 7 then 9 for
    that kind in one charge, refusal reports `next_charge = 7`, the first-inserted failing amount; the existing
    9 then 7 case alone cannot distinguish maximum coalescing before refusal. Simultaneous shortages refuse in `ScalarLimits`
    field order; ordinary refusal preserves counters, high-water, admissions, log and a pending injected occurrence.
    Cancellation and injection precedence, c
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference` uses production `Meter::charge`
    for the exact three-charge FR-362 schedule (operand charge with at most two semantic-size requests, arithmetic
    charge with one, result-retain charge with none) over the full `i64` operand range, plus separate repeated-kind
    charge controls with at most two requests each. Symbolic `i64` operands and symbolic limits including `ResultUnits`
    cover first-short and simultaneous shortages and absence of the third admission. Under `--unwind 3 --solver
    cadical`, 16 GiB and 600 seconds, the basel
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their own assertions
    under their original domains, oracle, completion and counterexample checks, unwind 3, Cadical, 16 GiB and 600
    seconds. Record their individual Kani verdicts as an external IR-705/IR-17 receipt; no quire-exact test or altered
    consumer harness substitutes for it.
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and called helpers finds no traversal
    with a trip count greater than that charge's semantic-size request count, including no traversal of `LimitKind::ALL`
    or another independently sized collection. The FR-362 proof schedule has at most two requests per charge, while
    the public `Charge` API retains no request-count ceiling.
- id: FR-359-AC-12
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Under `test-support`, the admitted-charge log for the three-charge FR-362 schedule contains exactly the
    admitted prefix in charge order, with each record's point and absent-or-present per-admission semantic-size
    maxima matching an independent model. Reordered and repeated requests of the same kind, including zero and distinct
    amounts, coalesce to the per-admission maximum in that record. A refused charge appends no record and leaves
    the existing log unchanged. The named `tc_931_admitted_log_matches_model_ordered_prefix` and `tc_931_admitted_log_matches_model_same_kind`
    tests verify these prope
- id: FR-359.Status
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'IR-844 baseline evidence on source `c824270f3820ea04187f5689bc73677285987bfa`: Kani 0.68.0 / CBMC 6.11.0
    / CaDiCaL 3.0.0 returned `Verified`, with 0 failed checks out of 2,565 (2,534 successful and 31 unreachable);
    all six unwinding checks succeeded. Verification took 131.5852 seconds and the command took 137.15 seconds.
    The cgroup peak was 8,183,332,864 bytes under an enforced 17,179,869,184-byte ceiling with swap disabled; memory-event
    counters stayed zero. Native qualification passed all eleven phases, including 73 test executions, default/test-support
    accounting, cancellation, admission, n'
- id: TC-931.Status
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: 'IR-844 baseline on source `c824270f3820ea04187f5689bc73677285987bfa` is `Verified` with Kani 0.68.0,
    CBMC 6.11.0 and CaDiCaL 3.0.0: 0 / 2,565 checks failed, 2,534 succeeded, 31 were unreachable and all six unwinding
    checks succeeded. Verification took 131.5852 s; command wall time was 137.15 s. The cgroup measured 8,183,332,864
    bytes peak under an enforced 17,179,869,184-byte ceiling, zero swap allowance and zero memory events. The actual
    timed command was:'
- id: FR-359.Description
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an `Incomplete`
    record for the first unavailable counter without wrapping or changing any consumed counter or admission count.
    The meter SHALL keep semantic-size charge traversal proportional to that charge's request count while preserving
    refusal order and admitted accounting. A standalone meter proof and an external consumer receipt qualify these
    claims as specified below.
- id: FR-359.Behavior
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: '`work_units` and `result_units` accumulate across admitted charges. An addition that equals its limit
    is admissible; an addition beyond its limit, including one whose exact amount cannot fit `u64`, is refused.
    A refused charge preserves the previously admitted state even when its work amount fits but its result amount
    does not. Semantic-size counters remain high-water values.


    For each `Meter::charge`, validation and update of semantic-size requests SHALL use only traversals whose trip
    counts do not exceed that charge''s request count. The charge path SHALL NOT traverse `LimitKind::ALL` or anot'
- id: FR-359.Status
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'AC-1 through AC-5 retain earlier Trace bindings. PR #9 adds executable Trace bindings for AC-6 and AC-7.
    Run `quire matrix` for the current criterion-to-test mapping.


    AC-8, AC-11 and AC-12 are delivered by IR-705. AC-8 is bound by the `Trace: FR-359-AC-8` unit tests in `src/accounting.rs`
    (including `first_inserted_failing_size_precedes_later_same_kind_maximum`, `size_shortages_use_field_order_across_attachment_orders`
    and `request_counts_from_zero_to_many_gain_no_denial`). AC-11 is verified by inspection: the only semantic-size
    loop in `Meter::charge` is over the charge''s own requests. AC-12'
- id: FR-359.Notes
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'The log is a test-support diagnostic absent from production builds. AC-12 supplies sampled unit-test
    evidence for log contents; AC-9 covers high-water coalescing symbolically through `consumed()` for every kind
    and does not assert diagnostic log contents.


    Historical runs below used earlier encodings or smaller obligations. They remain inconclusive and are distinct
    from the verified IR-844 baseline above; their measurements do not establish a memory-dominator explanation.


    | Earlier configuration | Recorded result |

    | --- | --- |

    | Original full modeled obligation at 40 GiB | Symex 67.4 s, 1,7'
- id: FR-359.Dependencies
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: '- [TC-907](../test-cases/TC-907-cumulative-meter-boundary.md) is the test-case home for AC-1 through
    AC-7. This requirement concerns the existing `Meter::charge` and `Charge` accounting contract in `src/accounting.rs`.

    - [TC-931](../test-cases/TC-931-single-pass-meter-accounting.md) owns AC-8 through AC-12 procedures and the
    external receipt record. FR-362 defines the three-charge schedule; FR-358 defines the diagnostic log, with its
    cap covered by FR-358-AC-15; FR-368 defines the limit vocabulary.'
- id: TC-931.Description
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: Verify FR-359-AC-8, AC-11 and AC-12 (delivered), AC-9 (IR-844 standalone proof and mutation controls)
    and record the permanently external AC-10 receipt. Kernel accounting and its Kani obligation belong to quire-exact.
    The five unchanged CG scalar tests are a separate consumer-owned IR-705/IR-17 acceptance receipt, not kernel
    test cases.
- id: TC-931.Test Procedure
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: 1. For AC-11, inspect production `Meter::charge` and called helpers for every semantic-size processing
    traversal. Record its collection and maximum trip count relative to that charge's request count; reject any
    `LimitKind::ALL` or independently sized traversal. A single request traversal is the intended design, but do
    not infer it from `--unwind 3`, which bounds each loop separately. Confirm the public `Charge` API adds no request-count
    ceiling. For AC-8, exercise zero, one, two, reordered and repeated same-kind requests and larger request counts.
    With `IntegerBits` limit 5, request 7 then 9 i
- id: TC-931.Expected Results
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: Charge processing has no traversal whose trip count exceeds the charge's semantic-size request count
    and preserves all previously observable accounting and denial behavior without a new count limit. The named
    AC-12 log tests match the independent model for sampled ordered records, per-admission maxima, same-kind coalescing
    and refusal atomicity; the existing FR-358-AC-15 test covers the 4096-entry cap. The standalone baseline Kani
    proof must verify under its specified ceiling for three modeled charges plus the repeated-kind control, without
    log assertions; record the actual measured result for
- id: TC-931.Status
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: 'AC-8, AC-11 and AC-12 are delivered by IR-705: AC-8 and AC-12 by the Trace-tagged unit tests in `src/accounting.rs`
    (AC-12 by the two named `tc_931_admitted_log_matches_model_*` tests), AC-11 by inspection.


    IR-844 baseline on source `c824270f3820ea04187f5689bc73677285987bfa` is `Verified` with Kani 0.68.0, CBMC 6.11.0
    and CaDiCaL 3.0.0: 0 / 2,565 checks failed, 2,534 succeeded, 31 were unreachable and all six unwinding checks
    succeeded. Verification took 131.5852 s; command wall time was 137.15 s. The cgroup measured 8,183,332,864 bytes
    peak under an enforced 17,179,869,184-byte ceiling, zero'
```

## Dispositions

Round 1, inspected frozen fix head `8cacb30872e724094260d8bc92ec6432a58c3232`. Original finding rows and review scope are preserved.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Actual fix removes the recorded source revision; the complete measured result remains unchanged. |
| FND-002 | fixed | Actual fix removes the recorded source revision; the complete measured result remains unchanged. |

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: 'IR-844 baseline evidence: Kani 0.68.0 / CBMC 6.11.0 / CaDiCaL 3.0.0 returned `Verified`, with
    0 failed checks out of 2,565 (2,534 successful and 31 unreachable); all six unwinding checks succeeded.'
- fnd: FND-002
  outcome: fixed
  after_excerpt: 'The IR-844 baseline is `Verified` with Kani 0.68.0, CBMC 6.11.0 and CaDiCaL 3.0.0: 0 / 2,565 checks
    failed, 2,534 succeeded, 31 were unreachable and all six unwinding checks succeeded.'
```

## Current Disposition Verdict

Both requested revision-record removals are fixed at the actual frozen fix head. No new finding. Current specification disposition is PASS.

Only two Status wording changes and verbatim custody of the original three SR files landed in the fix. Both Rust files still byte-match qualified source; no new tests or heavy checks were warranted.
