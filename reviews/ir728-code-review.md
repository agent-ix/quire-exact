---
id: SR-5660
title: code-review of IR-728 direct IEEE outcomes
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact@2ae01f9b55e3921734c6710391b445120a6270e0; src/ieee.rs,
  src/lib.rs, tests/ir673_ieee_text.rs; Ticket IR-728
review_set: subset
relationships:
- target: ix://agent-ix/quire-exact/FR-373
  type: references
- target: ix://agent-ix/quire-exact/TC-921
  type: references
---

## Summary

Ticket: IR-728. Independent frozen-code review; base 978fa7eaeacea77cb007d6004cd367dc167938d3.

## Verdict

PASS. No code/Rust defects in the exact three-path PR diff. The public API borrows RationalDomain and Meter and returns Outcome<IeeeExact> directly; the target-selector enum/re-export is removed, with no alias, compatibility wrapper or copied algorithm. The existing private to_exact body is byte-identical to base (SHA256 79989c8bbafd6daa0a79feea0e03eb511314c6d98a83a4b7541bdf1012a81c8e). Shared IllTypedCause::IeeeToNonRationalExact is unchanged. Nonfinite operands stop before intermediate; finite nonmembers stop after intermediate and before retain. Seven real public-API controls use independent IEEE bits and rational/charge constants; default and support assertions cover signed-zero loss, all incomplete fields, every counter, logs, original limits, injection spending, Requested/Deadline original-handle polls and precharged prefixes. Existing TC-914 value/loss assertions survive call-site adoption. Test-only unwrap/Mutex usage is deterministic, with Weak preventing an observer reference cycle. No production panic/unsafe/async/wire/config/CI changes, bypasses or weakened lanes were added.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Read receipts at /tmp/ix-handoff/ir728-ieee-focused/full-candidate/attempt-70fe3fedb06b4ab38b2d5f2cab5d0901: focused-receipt.json identifies exact reviewed head and records make --jobs=1 ci CARGO=cargo +1.98.1 exit 0 plus full spec/review validation exit 0. make-ci-stdout/stderr logs show fmt/lint, default/support tests, thumbv7em-none-eabi build, cargo deny and warnings-denied docs. scope receipt records terminal exit 0, removed cgroup, settled witnesses/helpers and 17.50888779759407s. No Kani/replay lane is defined in unchanged Makefile/CI. These are candidate receipts, not a promise that final merge gates ran.

Plan completion: not assessed

Full reviewed scope and verified bindings (clean units included):

```yaml
clean: true
scope:
- id: FR-373-AC-1
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: The public crate-root `ieee_to_exact` has the exact direct signature in
    Behavior. `IeeeExactTarget` and its re-export are absent; no compatibility entry
    point or copied conversion algorithm remains. A non-Rational target cannot be
    supplied to this signature.
- id: FR-373-AC-2
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Binary32 `0x3fc00000` and binary64 `0x3ff8000000000000` complete as `3/2`
    with no loss in numerator domain `[-3,3]`, denominator domain `[1,2]`. Both signed
    zeros at both widths complete as `0/1`; only negative zero carries `NegativeZeroSign`,
    preserving FR-366's original binary32 control.
- id: FR-373-AC-3
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Quiet NaN, signaling NaN and positive/negative infinity at each width return
    exactly `Undefined(IeeeNotFinite)` after one operand charge, one work unit, source-width
    IntegerBits and one ValueOccurrences high-water unit, with zero result units and
    no intermediate or retain admission.
- id: FR-373-AC-4
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Converting `3/2` to numerator domain `[-1,1]`, denominator domain `[1,2]`,
    and separately to numerator domain `[-3,3]`, denominator domain `[1,1]`, returns
    `Refused(IeeeRationalOutOfDomain { target })` with the corresponding exact target.
    Two work units are consumed; no result unit or retain charge is admitted.
- id: FR-373-AC-5
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: Successful finite conversion admits exactly operands, intermediate and
    retain in that order, consuming three work units and one result unit. Ordinary
    work limits 0, 1 and 2 deny the respective next point with exact limit, consumed
    and next-charge fields. Injected occurrence-1 denials at each point report WorkUnits
    with limit equal to pre-charge consumed work and next charge 1. Result limit 0
    denies retain atomically after two work units. Smallest positive subnormals at
    either width under IntegerBits equal to the source width deny intermediate with
    next-charge sizes 150 and 1075 respectively. E
- id: FR-373-AC-5
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: very denial leaves all counters and admission count at the independently
    specified admitted prefix and exposes no completed value.
- id: FR-373-AC-6
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: For Requested and Deadline, pre-cancelling the original handle attached
    to the supplied meter returns WorkUnits incomplete at operands with limit = consumed
    = 0 and next charge 1. Deterministic cancellation at the original handle's second
    and third meter polls returns the same form at intermediate and retain with limit
    = consumed = 1 and 2 respectively. The original handle records the actual cause,
    only the prior charge prefix is admitted and no result is retained. A live handle
    completes the same fixture.
- id: FR-373-AC-7
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: 'Precharge the supplied meter once at FunctionCall with work 2, result
    1, IntegerBits 80, ValueOccurrences 5 and TextScalars 7. Direct conversion of
    either width''s `3/2` under sufficient limits ends with work 5, results 2 and
    admission count 4, preserving all three high-water values and the FunctionCall
    log prefix. Independently repeat with work limit 4, result limit 1, an occurrence-1
    retain injection and original-handle cancellation at the third conversion poll.
    Each stops at retain with work 4, results 1 and admission count 3; ordinary records
    report respectively WorkUnits(limit=consumed=4) '
- id: FR-373-AC-7
  path: spec/functional/FR-373-rational-target-ieee-outcome.md
  role: examined
  excerpt: and ResultUnits(limit=consumed=1), while injection/cancellation report
    WorkUnits(limit=consumed=4); all report next charge 1. High-water values, original
    limits and admitted prefix remain intact, and cancellation records the original
    Requested or Deadline cause. No already-spent accounting is reset or refunded.
- id: src/ieee.rs
  path: src/ieee.rs
  role: examined
  excerpt: "/// Explicitly convert an IEEE value to a declared `Rational[..]` target.\n\
    /// NaN and the infinities are undefined; a finite exact value outside the\n///\
    \ target domain is refused before retention. Charges use the supplied meter,\n\
    /// preserving its already-spent accounting and original cancellation handle.\n\
    pub fn ieee_to_exact(\n    value: IeeeValue,\n    domain: &RationalDomain,\n \
    \   meter: &mut Meter,\n) -> Outcome<IeeeExact> {\n    Outcome::from_stop(to_exact(value,\
    \ domain, meter))\n}"
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: "pub use ieee::{\n    compare_ieee, convert_ieee_width, evaluate_ieee,\
    \ exact_to_ieee, ieee_intrinsic_identities,\n    ieee_to_exact, ExactScalar, FloatType,\
    \ IeeeComparison, IeeeExact, IeeeExactLoss, IeeeFlag,\n    IeeeFlags, IeeeOperand,\
    \ IeeeOperation, IeeeOperationKind, IeeeProvenance, IeeeResult,\n    IeeeValue,\
    \ IeeeWidth, IEEE_DEFINITION,\n};"
- id: tests/ir673_ieee_text.rs
  path: tests/ir673_ieee_text.rs
  role: examined
  excerpt: "/// Trace: TC-914, FR-366-AC-3\n#[test]\nfn ieee_signed_zero_converts_to_canonical_rational_with_loss()\
    \ {\n    let domain = RationalDomain::new(\n        IntegerInterval::new(Integer::from(-1_i64),\
    \ Integer::from(1_i64)).unwrap(),\n        IntegerInterval::new(Integer::from(1_i64),\
    \ Integer::from(1_i64)).unwrap(),\n    )\n    .unwrap();\n    for (bits, loss)\
    \ in [\n        (0_u32, None),\n        (0x8000_0000, Some(IeeeExactLoss::NegativeZeroSign)),\n\
    \    ] {\n        let result = ieee_to_exact(IeeeValue::binary32(bits), &domain,\
    \ &mut meter())\n            .completed()\n            .unwrap();\n        assert_eq!"
- id: TC-921
  path: spec/test-cases/TC-921-rational-target-ieee-outcome.md
  role: context_only
  excerpt: 'Verify [FR-373](../functional/FR-373-rational-target-ieee-outcome.md)
    through

    the public crate-root operation and real meter. Specify expected bit patterns,

    reduced rational values and charge prefixes independently. Do not retain a

    legacy conversion API as a parity oracle or replace the conversion algorithm.'
- id: accounting.rs
  path: src/accounting.rs
  role: context_only
  excerpt: 'pub fn charge(&mut self, mut charge: Charge) -> Result<(), Incomplete>
    {'
- id: cancel.rs
  path: src/cancel.rs
  role: context_only
  excerpt: pub fn poll(&self) -> bool {
findings: []
bindings:
- test_id: rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  ac_id: FR-373-AC-1
  trace: correct
- test_id: rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  ac_id: FR-373-AC-2
  trace: correct
- test_id: rational_target_ieee::nonfinite_values_stop_after_operands
  ac_id: FR-373-AC-3
  trace: correct
- test_id: rational_target_ieee::numerator_and_denominator_refusals_retain_the_declared_target
  ac_id: FR-373-AC-4
  trace: correct
- test_id: rational_target_ieee::finite_values_and_both_signed_zeros_complete_directly
  ac_id: FR-373-AC-5
  trace: correct
- test_id: rational_target_ieee::ordinary_and_injected_denials_preserve_the_admitted_prefix
  ac_id: FR-373-AC-5
  trace: correct
- test_id: rational_target_ieee::subnormal_intermediates_require_the_exact_denominator_bits
  ac_id: FR-373-AC-5
  trace: correct
- test_id: rational_target_ieee::original_cancellation_handle_stops_each_poll_and_live_handle_completes
  ac_id: FR-373-AC-6
  trace: correct
- test_id: rational_target_ieee::already_spent_meter_prefix_survives_success_and_every_retain_stop
  ac_id: FR-373-AC-7
  trace: correct
- test_id: ieee_signed_zero_converts_to_canonical_rational_with_loss
  ac_id: FR-366-AC-3
  trace: correct
limitations: PR diff review only; no source edits, Cargo/build/probe/locks, merge
  or publication by reviewer. Root supplied authenticated PUBLIC repository/PR metadata;
  local origin agrees. Exact backend deployment model ID is unexposed; model=unavailable,
  system identifies GPT-6/configured Codex. Gates are independently read supplied
  receipts/logs, not reviewer-executed runs. No applicable AssuranceProfile is present.
  No downstream QSL/RT adoption or language-boundary behavior is qualified. Spec-review
  and sub-analyses skipped because spec/plan diff is empty; React/Python lanes inapplicable.
  Optional full-repository semantic gap review and plan completion not assessed.
```
