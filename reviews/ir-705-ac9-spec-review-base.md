---
id: SR-11710
title: IR-705 AC-9 log split spec-review/base review
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact@acc27246af8319efa79934291bb29437b8562e31; spec/functional/FR-359-cumulative-meter-boundary.md, spec/test-cases/TC-931-single-pass-meter-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-705. PR agent-ix/quire-exact#51, head acc2724, base origin/main 02625b8; diff `git diff origin/main...acc2724` (FR-359 + TC-931). Spec-only, reviewer-only: no cargo or Kani run; src/accounting.rs at acc2724 and the IR-705 code branch harness (55d7e36, src/accounting/kani_proofs.rs) read as context. Method: quoin:spec-review/base.

## Scope

```yaml
scope:
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'The `cfg(kani)` harness tagged `Trace: FR-359-AC-9` uses production `Meter::charge` for the exact three-charge
    FR-362 schedule ... over the full `i64` operand range, plus separate repeated-kind charge controls with at most
    two requests each. Symbolic `i64` operands and symbolic limits including `ResultUnits` cover first-short and
    simultaneous shortages and absence of the third admission. Under `--unwind 3 --solver cadical`, 16 GiB and 600
    seconds, the baseline terminates with no unwinding-assertion failure and agrees with an independent model of
    complete refusal fields, `ScalarLimits` field...'
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the per-admission maximum in the FR-358 log. With `IntegerBits`
    limit 5 and requests 7 then 9 for that kind in one charge, refusal reports `next_charge = 7`, the first-inserted
    failing amount; the existing 9 then 7 case alone cannot distinguish maximum coalescing before refusal. Simultaneous
    shortages refuse in `ScalarLimits` field order; ordinary refusal preserves counters, high-water, admissions,
    log and a pending injected occurrence. Cancellation and injection precedence, cumulative totals and `consumed()`
    for every kind remain unchanged. Zero, on...
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'AC-8''s `Test` evidence includes the following named `test-support` unit-test group; each test carries
    `Trace: FR-359-AC-8` and remains PLANNED / UNRUN: - `tc_931_admitted_log_matches_model_ordered_prefix` ... -
    `tc_931_admitted_log_matches_model_same_kind` ... - `tc_931_admitted_log_matches_model_cap`: compare the ordered
    record prefix and per-admission maxima through admission 4096, then confirm admission 4097 changes only the
    truncation flag and exact accounting, without extending the log.'
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The log is a test-support diagnostic absent from production builds; asserting its contents in the Kani
    obligation exhausted a 16 GiB memory ceiling (15.4 GiB measured), and no sound reduction was found. Kani retains
    the consumer-facing refusal order, atomicity, termination and counter proof, while the named AC-8 unit tests
    are planned to check the diagnostic log cases above exhaustively.
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'The explicit non-CI lane command is `cargo kani --harness "$HARNESS" --unwind 3 --solver cadical`, where
    `HARNESS` is the symbol of the `cfg(kani)` harness tagged `Trace: FR-359-AC-9`, with a 16 GiB memory ceiling
    and 600-second timeout.'
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: 'Under `test-support`, implement the named `tc_931_admitted_log_matches_model_ordered_prefix`, `tc_931_admitted_log_matches_model_same_kind`
    and `tc_931_admitted_log_matches_model_cap` unit tests from FR-359, each tagged `Trace: FR-359-AC-8`. Compare
    every logged point, absent-or-present semantic-size field and per-admission maximum with an independent model,
    including same-kind coalescing, refused and absent admissions, and the 4096-record prefix and truncation at
    admission 4097. Keep these tests separate from the Kani harness.'
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: 2. For AC-9, record that Kani compiles this `#![no_std]` crate for the tagged harness. ... Compare point
    order, complete `Incomplete` fields including the first-inserted failing amount for same-kind duplicates, `ScalarLimits`
    field-order refusal, refusal atomicity, the admitted prefix, all resulting counters and `consumed(kind)` for
    every kind with a separately written model. ... Do not assert diagnostic log contents in this harness; `Meter::charge`,
    counters and `consumed(kind)` need no `test-support` feature. ... run `cargo kani --harness "$HARNESS" --unwind
    3 --solver cadical` ...
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: 3. For AC-9 mutation controls, introduce a temporary `LimitKind::ALL` traversal of charge accounting
    and require Kani to report an unwinding-assertion failure at that loop under the identical command and bounds.
    Separately swap first-short refusal order and require a model-equivalence assertion failure with a concrete
    counterexample. ...
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: The named AC-8 log tests match the independent model for ordered records, per-admission maxima, same-kind
    coalescing and the 4096-entry cap. The standalone baseline Kani proof verifies under its specified ceiling;
    the ALL-traversal control reports the expected unwinding-assertion failure, and the refusal-order control reports
    the expected model-equivalence counterexample.
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently chosen semantic-size requests,
    including repeated same-kind requests and distinct amounts at admissions 4096 and 4097. The single record log
    contains exactly the first 4096 fixed-size records with their points and per-admission semantic-size maxima
    in admission order; truncation is false at admission 4096 and true at 4097. Work consumption and admission count
    reach 4097, including the omitted record; existing size maxima still include the 4097th charge.
- id: src/accounting.rs
  path: src/accounting.rs
  role: context_only
  excerpt: 'pub fn consumed(&self, kind: LimitKind) -> u64 { ... } pub fn admission_count(&self) -> u64 { ... }
    #[cfg(feature = "test-support")] pub fn admitted_charges(&self) -> &[AdmittedCharge] ... pub fn charge(&mut
    self, charge: Charge) -> Result<(), Incomplete> (no feature gate)'
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The Notes rationale is incomplete and partly misattributed against the measured numbers. It gives one figure, '(15.4 GiB measured)', and attaches it to 'the Kani obligation', which is the three-charge schedule. The measurements were 64 s / 15.4 GiB for TWO modeled charges with log assertions and 80 s / 14.9 GiB for three. The note omits the without-log comparison (33 s / 1.8 GiB for two modeled charges) that justifies the split. 'Exhausted a 16 GiB ceiling (15.4 GiB measured)' also reads as self-contradictory, because 15.4 < 16, and it does not say the run died (OOM/MemoryExhausted). Failure scenario: a later lane re-measures the full three-charge obligation, sees a peak under 16 GiB, concludes the note is wrong, and re-adds log assertions. Give the four figures with their charge counts and with/without log, and state the outcome as MemoryExhausted. | spec/functional/FR-359-cumulative-meter-boundary.md:65 |
| FND-002 | medium | The three log tests are bound to FR-359-AC-8, but AC-8's text states only same-kind per-admission maxima and log preservation on refusal. It does not state the ordered admitted-record prefix over the FR-362 schedule (ordered_prefix test) or the 4096 cap and 4097 truncation (cap test). The tests therefore assert properties their AC does not state. AC-8 is also backed by other tests, so the matrix row goes green from any one AC-8 tag. Quire strict coverage cannot detect that a named log test was never written, and the planner condition 'not demoted to prose' is met only by a prose bullet list (lines 41-45). Failure scenario: the coder lands the AC-8 7-then-9 refusal test and omits all three log tests; coverage shows AC-8 backed and the log contents lose their only verification silently. Either add an AC (for example FR-359-AC-12, method Test) stating the log-content properties and bind the three tests to it, or extend AC-8's text to name them. | spec/functional/FR-359-cumulative-meter-boundary.md:36,41-45 |

## Assessment

Checked (a) old-vs-new AC-9 diff; (b) named log tests: method Test via AC-8, `Trace: FR-359-AC-8` tag form, PLANNED/UNRUN stated at FR:41 and TC Status, names unique in the repo (no existing symbol). (c) The claim that no test-support is needed holds against src/accounting.rs: `Meter::charge` (:790), `consumed` (:711), `admission_count` (:716), `limits` (:706), `Charge::new/size/results` and `Meter: Clone + Eq` are all ungated; only `admitted_charges`/`charge_log_truncated`/`AdmittedCharge` are gated (:570-:731). The IR-705 code harness (55d7e36 kani_proofs.rs) confines its log assertions to a `#[cfg(feature = "test-support")]` block, so it compiles and keeps every counter/refusal assertion without the feature. (d) The mutation controls hold without the log: the ALL mutant still gives a 10-trip production loop under unwind 3, and the refusal-order mutant changes the `Incomplete` the model compares. (f) TC-931 matches FR-359. `quire validate` is clean (module-level warnings only). Strict coverage is 163/191 (28 unbacked) at both base 02625b8 and head, delta 0. FR-359 AC ids are unique.

## Verdict

CHANGES REQUESTED: two medium findings. The core split is sound. Line by line, old AC-9 required: termination ('verifies unwinding'), complete refusal fields, field order, first-inserted failing amount, per-admission maxima, admitted prefix and counters. New AC-9 keeps all of these except per-admission maxima, and adds refusal atomicity, symbolic operands and consumed() for every kind. Per-admission maxima move to the ordered_prefix and same_kind tests, so nothing silently vanished. The CG-facing refusal properties are not weakened. The binding and rationale defects are FND-001 and FND-002.

## Dispositions

Round 1 verified at head e25295d (diff acc2724..e25295d, FR-359 + TC-931 only; origin/main still 02625b8, so the base has not moved). Each fix was checked against the spec text at e25295d, plus tests/ir667_normalize.rs:215-217, `quire validate` (clean) and `quire coverage --strict` (163/192, 29 unbacked).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | round 1, head e25295d: Notes (FR-359:65-70) now give all five configurations with charge counts, with/without log, times and peaks. They match the measured figures exactly and explain why MemoryExhausted peaks sit below 16 GiB. |
| FND-002 | fixed | round 1, head e25295d: New FR-359-AC-12 (Test) states the ordered prefix, per-admission maxima, same-kind coalescing and unchanged log on refusal. Both named tests are now tagged `Trace: FR-359-AC-12`, and the row is counted as unbacked until they land. |
