---
id: SR-11701
title: IR-705 meter single-pass spec-review/integrity review
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@74265e6a50c3116571a6de2c11bfc649878208da; spec/functional/FR-359-cumulative-meter-boundary.md, spec/test-cases/TC-931-single-pass-meter-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-705. PR agent-ix/quire-exact#50, head 74265e6, base 37aa172; diff `git diff origin/main...74265e6` (FR-359 + new TC-931 only). Spec-only, reviewer-only: no cargo, Kani or solver run; source read as context. Method: quoin:spec-review/integrity.

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
  excerpt: When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an
    `Incomplete` record for the first unavailable counter without wrapping or changing any consumed counter
    or admission count.
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: '- [TC-907](../test-cases/TC-907-cumulative-meter-boundary.md) is the test-case home for these
    criteria. This requirement concerns the existing `Meter::charge` and `Charge` accounting contract
    in `src/accounting.rs`. - [TC-931](../test-cases/TC-931-single-pass-meter-accounting.md) owns the
    planned single-pass, standalone proof and external-receipt procedures.'
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
- id: AD-002
  path: spec/assurance/AD-002-composable-equality-proof-seam.md
  role: context_only
  excerpt: No cross-crate Kani contract is authorized by this decision. ... A same-crate proof or a toy
    control is not sufficient.
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | 'Field-order refusal ... SHALL remain unchanged' pins behaviour that no criterion states. When two same-kind requests both exceed the limit, current code reports the first-inserted failing request's amount as next_charge (src/accounting.rs:799-808, comment :797-798). AC-6 says only 'requested amount'. A natural single-pass design coalesces to the per-kind maximum and then compares it with the limit, which reports the maximum instead. Example: limit 5 with IntegerBits requests 7 then 9. Today next_charge = 7; the max-based single pass gives 9. The existing test duplicate_size_requests_preserve_first_refusal_and_are_atomic uses 9 then 7 (:1118-1133), where both designs report 9, so it cannot catch the change. Add the tie rule to AC-8: for a refused kind, next_charge is the first request in insertion order that exceeds its limit. Add a control in which a later failing request exceeds an earlier one. | spec/functional/FR-359-cumulative-meter-boundary.md:17 |
| FND-002 | medium | Two external CG receipts in this repo now disagree. FR-371-AC-6, TC-919 step 6 and AD-003 name three controls: healthy TC027, the TC027 bound mutant, and the TC025 arithmetic mutant. They require 'healthy verification and both substantive falsifications'. FR-359-AC-10 names five: 'TC-027 attested receipt twice', TC-025, routed healthy and routed violating-bound. It says all five 'pass', and FR-359 line 31 says 'Each SHALL pass ... counterexample checks'. The spec does not map 'TC-027 attested receipt twice' to healthy plus bound mutant. 'Pass' for the TC-025 mutant can be read as Kani VERIFICATION SUCCESSFUL, which is the opposite of the falsification FR-371-AC-6 requires. Failure scenario: a run where the mutated-oracle harness verifies is recorded as an AC-10 pass. State the expected per-harness Kani verdict (verified, or a counterexample found), map the names to FR-371-AC-6, and cross-reference the shared receipt. | spec/functional/FR-359-cumulative-meter-boundary.md:31 |
| FND-003 | low | FR-359's title and Description still cover only the cumulative u64 boundary (line 11). The FR now also carries a structural single-traversal constraint, a Kani proof obligation and an external consumer receipt, none of which the Description mentions. Dependencies line 56 still says TC-907 'is the test-case home for these criteria', but line 57 now gives AC-8..AC-10 to TC-931. Narrow line 56 to AC-1..AC-7, and either extend the Description or move the traversal/proof obligation to its own FR. | spec/functional/FR-359-cumulative-meter-boundary.md:11 |

## Assessment

Checked the 'unchanged' list against current code and the existing FRs. Injection and cancellation come first (check_injected :749-782, FR-358 Behavior 5), and the AC-8 text preserves that order. Every size check precedes WorkUnits, which precedes ResultUnits (:799-818, FR-368 ordering). Atomicity holds because validation completes before any update (:821), so a single traversal must stage its updates; the spec allows that. The per-admission maximum is AdmittedCharge::observe (:609-622, FR-358 B2). consumed() reads an index (:711-713, AC-7). The log cap is 4096 in admit() (:845-865, FR-358 AC-4). An ordinary refusal leaves a pending injected occurrence untouched because admit() is not called. Each of these is consistent with the new text, except the unstated duplicate tie rule (FND-001). The error payload identity (limit/consumed taken from self.incomplete) is unchanged by construction. FR-371-AC-6's 'MemoryExhausted is inconclusive' agrees with FR-359 (an inconclusive run does not satisfy), so they do not contradict. AD-002 forbids relying on a cross-crate contract, which FR-359 respects by keeping AC-10 external. No existing AC text is modified.

## Verdict

CHANGES REQUESTED: pin the duplicate-refusal tie rule and reconcile the AC-10 receipt with FR-371-AC-6/TC-919.

## Dispositions

Round 1 verified at head 62a5954 (diff 74265e6..62a5954; origin/main still 37aa172, so no base movement). Each fix was checked against the spec text and against src/accounting.rs and CG tests/it/kani_obligations.rs at CG origin/main.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | round 1, head 62a5954 |
| FND-002 | fixed | round 1, head 62a5954 |
| FND-003 | fixed | round 1, head 62a5954 |
