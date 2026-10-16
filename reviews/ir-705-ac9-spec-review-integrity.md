---
id: SR-11711
title: IR-705 AC-9 log split spec-review/integrity review
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@acc27246af8319efa79934291bb29437b8562e31; spec/functional/FR-359-cumulative-meter-boundary.md, spec/test-cases/TC-931-single-pass-meter-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-705. PR agent-ix/quire-exact#51, head acc2724, base origin/main 02625b8; diff `git diff origin/main...acc2724` (FR-359 + TC-931). Spec-only, reviewer-only: no cargo or Kani run; src/accounting.rs at acc2724 and the IR-705 code branch harness (55d7e36, src/accounting/kani_proofs.rs) read as context. Method: quoin:spec-review/integrity.

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
| FND-001 | medium | tc_931_admitted_log_matches_model_cap restates FR-358-AC-15 (and FR-358-AC-4): it covers the 4096 ordered prefix with per-admission maxima and truncation at 4097. FR-358-AC-15 already has an executable binding at tests/ir667_normalize.rs:215. It is a FR-358 property, not an FR-359-AC-8 one. The old AC-9 never covered the cap either, because three charges cannot reach 4096 admissions, so this is not a property moved out of Kani. Binding it to FR-359-AC-8 creates a duplicate test under the wrong criterion. Failure scenario: FR-358's cap changes; the FR-358 test is updated, the FR-359 duplicate is not, and the two disagree. Drop the cap bullet and cite FR-358-AC-15 as the existing cap evidence. If a new test is wanted, tag it FR-358-AC-15. | spec/functional/FR-359-cumulative-meter-boundary.md:45 |

## Assessment

Cross-checked FR-358-AC-4/AC-15 and the existing `Trace: FR-358-AC-15` binding at tests/ir667_normalize.rs:215. Status (FR:61) 'No ... Kani result ... is claimed' sits with Notes citing a measured Kani run. That is acceptable: the Notes give an exploratory measurement, not acceptance evidence.

## Verdict

CHANGES REQUESTED: one medium finding (FND-001, the cap test duplicates FR-358-AC-15 under the wrong criterion). Otherwise consistent: FR-359 and TC-931 agree on the command (no `--features test-support`), the three test names, the tag and the PLANNED/UNRUN status. TC-931 Expected Results matches the new split. AC ids are unique and validation is clean.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | AC-8's first sentence, 'Reordered and repeated requests preserve the per-admission maximum in the FR-358 log', now restates the coalescing property that AC-12 owns. The log-content tests moved to AC-12 and TC-931 step 1 no longer asks AC-8 tests to check log maxima, so AC-8 still names a log property that none of its procedures verifies. Failure scenario: a reader or coder treats AC-8 as requiring a log-maxima assertion and duplicates the AC-12 tests under AC-8, or a matrix audit flags AC-8 as covering a property its tests never check. Fix: change the sentence to say that reordered and repeated requests are processed without a new denial, and leave log maxima to AC-12, or say '(verified under AC-12)'. | spec/functional/FR-359-cumulative-meter-boundary.md:36 |

## Dispositions

Round 1 verified at head e25295d (diff acc2724..e25295d, FR-359 + TC-931 only; origin/main still 02625b8, so the base has not moved). Each fix was checked against the spec text at e25295d, plus tests/ir667_normalize.rs:215-217, `quire validate` (clean) and `quire coverage --strict` (163/192, 29 unbacked).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | round 1, head e25295d: The cap bullet is removed. FR-358-AC-15 is cited, and its test exists at tests/ir667_normalize.rs:215-217. TC-931 step 2 forbids a duplicate. |
| FND-002 | fixed | round 2, head b06fee1 (diff e25295d..b06fee1, FR-359:36 only; origin/main still 02625b8): AC-8 sentence 1 now reads 'preserve the admitted high-water maximum observable through `consumed()`; the diagnostic log record is verified under FR-359-AC-12', so the overlap with AC-12 is gone. TC-931 step 1 still checks counter coalescing under AC-8 (reordered and repeated same-kind requests; every `consumed(kind)` and high-water with independent expected values). validate clean; strict coverage 163/192, 29 unbacked, unchanged. |
