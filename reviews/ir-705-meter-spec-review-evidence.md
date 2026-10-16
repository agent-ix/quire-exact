---
id: SR-11704
title: IR-705 meter single-pass spec-review/evidence review
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-exact@74265e6a50c3116571a6de2c11bfc649878208da; spec/functional/FR-359-cumulative-meter-boundary.md, spec/test-cases/TC-931-single-pass-meter-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-705. PR agent-ix/quire-exact#50, head 74265e6, base 37aa172; diff `git diff origin/main...74265e6` (FR-359 + new TC-931 only). Spec-only, reviewer-only: no cargo, Kani or solver run; source read as context. Method: quoin:spec-review/evidence.

## Scope

```yaml
scope:
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'For each `Meter::charge` admission, validation and update of semantic-size requests SHALL
    use one traversal of that charge''s own requests, bounded by its request count. It SHALL NOT traverse
    `LimitKind::ALL` or another collection whose length is independent of that count. The FR-362 integer-arithmetic
    schedule supplies at most two requests per charge. This proof bound does not add a public request-count
    limit: FR-358 permits arbitrarily many repeated requests. Field-order refusal, same-kind per-admission
    maxima, atomicity, injection and cancellation order, `consumed()` for every kind, and t...'
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The kernel SHALL own a standalone Kani obligation for the three integer-arithmetic charges
    over symbolic bounded `i64` operands. The obligation SHALL exercise production `Meter::charge` with
    unwinding assertions at `--unwind 3` and compare field-order refusal, coalescing, admitted prefix
    and counters to an independent reference model. Test-only `cfg(kani)` harness code may construct inputs
    and the model. Such harness code SHALL NOT change any production path. The lane command is `cargo
    kani --harness <integer-arithmetic-meter-harness> --unwind 3 --solver cadical`, with a 16 GiB memory
    ceili...
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: A double-pass or `LimitKind::ALL`-traversal mutant SHALL fail an unwinding assertion. A refusal-order
    mutant SHALL fail model equivalence. `MemoryExhausted`, `UnwindBoundExhausted`, timeout, compiler
    error or missing assertion is not accepted evidence.
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'The unchanged consumer-owned CG scalar proofs are the external IR-705/IR-17 acceptance receipt
    after re-locking to the reviewed kernel: TC-027 attested receipt twice, TC-025 mutated-oracle arithmetic,
    routed healthy, and routed violating-bound. Each SHALL pass with its original domain, oracle, completion
    and counterexample checks at unwind 3, Cadical, 16 GiB and 600 seconds. This is not a quire-exact
    test. The integer-arithmetic `Result` to `Outcome::from_stop` carrier is a separate open risk outside
    this Meter-only slice. If the five unchanged proofs still fail after the Meter change, inve...'
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection and tests show `Meter::charge` validates and updates one charge's semantic-size
    requests in a single traversal bounded by that charge's request count, without traversing `LimitKind::ALL`
    or an independently sized collection. For FR-362 integer arithmetic the bound is two requests per
    charge. Reordered and repeated requests preserve the per-admission maximum in the FR-358 log; simultaneous
    shortages refuse in `ScalarLimits` field order; ordinary refusal preserves counters, high-water, admissions,
    log and a pending injected occurrence. Cancellation and injection precedence, cumulat...
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: A standalone quire-exact Kani harness over symbolic bounded `i64` operands executes the real
    three-charge FR-362 schedule and bounded charge-level repeated-kind controls, proves accounting terminates
    under `--unwind 3`, and agrees with an independent model of field-order refusal, coalescing, admitted
    prefix and counters. At Cadical, 16 GiB and 600 seconds, a double-pass or `LimitKind::ALL` mutant
    fails unwinding and a refusal-order mutant fails equivalence. Test-only `cfg(kani)` harness code changes
    no production path; `MemoryExhausted`, `UnwindBoundExhausted` or another inconclusive outcom...
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After the consumer re-locks to the reviewed kernel, the five unchanged CG scalar proofs—TC-027
    attested receipt twice, TC-025 mutated-oracle arithmetic, routed healthy and routed violating-bound—pass
    with their original input bounds, oracle and resource ceilings. Record each result as an external
    IR-705/IR-17 receipt; no quire-exact test or altered consumer harness substitutes for it.
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: 1. For AC-8, inspect production `Meter::charge` and its called helpers to establish that each
    charge validates and updates its own semantic-size requests with one traversal bounded by the request
    count, with no `LimitKind::ALL` or other independently sized traversal. Exercise zero, one, two, reordered
    and repeated same-kind requests, then larger request counts to ensure there is no new count limit.
    With independent expected values, check same-kind per-admission maxima in the test-support log, every
    `consumed(kind)`, high-water and cumulative counters, exact first-short `ScalarLimits` field-...
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: '2. For AC-9, implement a standalone `cfg(kani)` proof harness that constructs bounded symbolic
    `i64` operands and runs the production integer-arithmetic path''s three charges: operands with at
    most two size requests, arithmetic with one, and result-retain with none. Vary limits so `IntegerBits`,
    `ValueOccurrences`, `WorkUnits` and `ResultUnits` can each be first short, including simultaneous
    shortages. Use charge-level controls for repeated same-kind requests. Compare point order, complete
    `Incomplete` fields, admitted prefix, all counters and per-admission maxima with a separately written
    m...'
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: 3. For AC-9 mutation controls, separately introduce a temporary double-pass mutant and a temporary
    `LimitKind::ALL` traversal mutant of charge accounting, then require an unwinding assertion failure
    for each under the identical command and bounds. Separately swap first-short refusal order and require
    a model-equivalence failure with a concrete counterexample. Restore production code after each control
    and record the baseline and mutant results independently; a compiler error or resource failure is
    not a substantive falsification.
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: 4. For AC-10, the consumer owner re-locks CG to the reviewed kernel and executes the unchanged
    two TC-027 attested-receipt cases, TC-025 mutated-oracle arithmetic, routed healthy and routed violating-bound.
    Record the kernel identity, exact selected harnesses, original input/domain bounds, oracle, completion
    and counterexample checks, unwind 3, Cadical, 16 GiB and 600 seconds, individual verdicts and peak
    memory. Do not copy or edit CG harnesses into this repository. If any proof fails after the Meter
    change, report the failure and investigate the separate `Result`/`Outcome::from_stop` carr...
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: One request-proportional traversal preserves all previously observable accounting and denial
    behavior without a new count limit. The standalone Kani proof verifies under its specified ceiling,
    both traversal mutants fail unwinding, and the refusal-order mutant fails equivalence. `MemoryExhausted`,
    `UnwindBoundExhausted`, timeout, compiler error, missing mutation execution or an altered harness
    never counts as acceptance. All five unchanged consumer proofs pass and are recorded as an external
    receipt. An inconclusive result remains unaccepted even where FR-371-AC-6 classifies it as inconclus...
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: AC-8 through AC-10 are PLANNED / UNRUN. No implementation, Kani result, mutation control or
    consumer receipt is claimed by this proposal.
- id: FR-359-AC-5
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: context_only
  excerpt: First admit an `integer_bits` high-water size of 8 and one result unit under limits of at least
    16 bits, `u64::MAX` work units and one result unit. Then attempt a charge carrying `integer_bits =
    16` and one result unit. The result refusal leaves `integer_bits = 8`, work units, result units and
    admission count at their pre-refusal values; under `test-support`, the admitted-charge log is unchanged
    too.
- id: FR-359-AC-6
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: context_only
  excerpt: With no injection installed, independently make each of the ten counters the first unavailable
    counter of a charge, including several simultaneously short counters and semantic sizes attached in
    either order. The reported `limit_kind` follows `ScalarLimits` field order, with the exact limit,
    previously consumed amount, requested amount and point. A refusal after an earlier size check succeeds
    changes no counter, admission count or test-support log. A separate meter with a pending later named
    denial proves that this ordinary refusal does not advance its occurrence.
- id: FR-359-AC-7
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: context_only
  excerpt: 'On fresh, charged and stopped meters, `Meter::consumed(kind)` answers without panic for every
    member of `LimitKind::ALL`: untouched counters are zero, semantic sizes are admitted high-water maxima
    and work/results are admitted cumulative totals.'
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: For each semantic-size kind present in the admitted charge, the recorded amount SHALL equal
    the maximum of the actual validated requests for that kind within that admission. ... Arbitrarily
    many repeated or zero requests SHALL remain subject only to existing admission rules, without a new
    request-count denial or limit.
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original
    input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion
    cover and counterexample checks. Healthy verification and both substantive falsifications are required.
    Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler
    error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and
    does not satisfy this criterion.
- id: TC-919
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: 6. For AC-6, the consumer owner selects the original genuine healthy TC027, TC027 bound mutant
    and TC025 arithmetic mutant against the actual reviewed/adopted kernel identity. Preflight the exact
    selection without execution, then use the separately allocated runner/resource envelope. Preserve
    original domains, oracle, unwind3, Cadical, 600s, 16GiB, completion cover and falsification witnesses.
- id: AD-003
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: context_only
  excerpt: '| Qualification claim | Keep TC-919 planned and require the unchanged CG TC-025 arithmetic
    mutant, TC-027 healthy case, and TC-027 bound mutant at their existing 16GiB/600-second/unwind3/Cadical
    settings. |'
- id: src/accounting.rs
  path: src/accounting.rs
  role: context_only
  excerpt: for kind in LimitKind::ALL { for (requested_kind, amount) in charge.sizes.iter() { if *requested_kind
    != kind { continue; } match amount.to_u64() { Some(amount) if amount <= kind.limit(&self.counters.limits)
    => {} _ => return Err(self.incomplete(kind, amount.clone(), point)), } } } ... let work = cumulative(LimitKind::WorkUnits,
    charge.work_units)?; let results = cumulative(LimitKind::ResultUnits, charge.result_units)?; ... for
    kind in LimitKind::ALL { for (requested_kind, amount) in charge.sizes.iter() { ... record.observe(kind,
    amount); let slot = &mut self.counters.consumed[kind.index()]...
- id: src/numeric.rs
  path: src/numeric.rs
  role: context_only
  excerpt: meter.charge(Charge::new(ChargePoint::IntegerArithmeticOperands).size(LimitKind::IntegerBits,
    bits).size(LimitKind::ValueOccurrences, count))?; meter.charge(Charge::new(ChargePoint::IntegerArithmeticArithmetic).exact_size(LimitKind::IntegerBits,
    integer_arithmetic_bits(operation)))?; ... meter.charge(Charge::new(ChargePoint::IntegerArithmeticResultRetain).results(1))?;
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-8 opens with 'Inspection and tests show' but declares only 'Test'. Its first clause, a single traversal bounded by request count with no LimitKind::ALL traversal, is a structural property that no unit test can observe, because behaviour is identical under the old and new loops. In this AC that clause rests only on inspection, which the method column does not record. Machine evidence for it comes only from AC-9's unwinding check. Split the structural clause into an Inspection criterion (or rely on AC-9 and say so), and keep AC-8 'Test' for the behaviour-preservation clauses. | spec/functional/FR-359-cumulative-meter-boundary.md:44 |
| FND-002 | medium | AC-10 is a quire-exact criterion verified by 'Test', but its own text forbids any quire-exact test from backing it. quire matrix computes it untagged, and it can never be tagged in this repo honestly. Measured: untagged 14 -> 17 (strict exit 1 before and after; FR-371-AC-6 already has the same permanent-external shape). Leaving it unbacked is honest, but the Status calls it only 'PLANNED / UNRUN', implying it will be bound later. The pressure to get a green strict run invites tagging an unrelated in-repo test, which is exactly what AC-10 forbids. Switching to Inspection would make it compute method-without-symbol, which is falsely green. State in Status that AC-10 is permanently externally evidenced and stays untagged by design. Better still, own the receipt in the consumer (CG) spec or in the IR-705 ticket acceptance, and cite it from FR-359. | spec/functional/FR-359-cumulative-meter-boundary.md:46 |

## Assessment

quoin advise --json (quoin 0.28.3) reports mismatch=false for AC-8, AC-9 and AC-10. For AC-9 it also recommends Analysis/model-checking. A Kani harness is model checking, but the Test class is listed and a cfg(kani) harness is an executable artifact, so Test is acceptable and not raised. The author reported strict 24 -> 27 unbacked. This review measured 14 -> 17 untagged with quire 0.36.2 (engine 0.50.2) at 37aa172 and 74265e6, so the delta (+3) agrees but the absolute counts do not.

## Verdict

CHANGES REQUESTED: method labelling for AC-8 and an honest permanent-external status for AC-10.

## Dispositions

Round 1 verified at head 62a5954 (diff 74265e6..62a5954; origin/main still 37aa172, so no base movement). Each fix was checked against the spec text and against src/accounting.rs and CG tests/it/kani_obligations.rs at CG origin/main.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | round 1, head 62a5954 |
| FND-002 | fixed | round 1, head 62a5954 |
