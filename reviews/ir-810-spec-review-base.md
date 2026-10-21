---
id: SR-14222
title: spec-review/base review of IR-810 Meter proof gate
type: SpecReview
analysis: base
scope: agent-ix/quire-exact; Makefile, README.md, docs/meter-kani-gate.md, scripts/meter_kani_gate.py,
  scripts/meter_kani_scope.sh, scripts/test_meter_kani_gate.py, tests/meter_kani_gate.rs,
  spec/functional/FR-359-cumulative-meter-boundary.md, spec/test-cases/TC-931-single-pass-meter-accounting.md
review_set: subset
---

## Summary

Frozen candidate b4a47bf8aec7805d42627561544564cb7fd5773c; base 204fb0c3637947b87d885eaaa325126f8d75016d. Ticket: IR-810. PR: agent-ix/quire-exact#66. Reviewer model: GPT-6; exact deployment SKU is unavailable, so markers use gpt-6. No applicable AssuranceProfile was found. No source edits, commits, pushes, Cargo/native/Kani executions or aggregate gate were performed by this reviewer. Scope excerpts over 600 characters use a verbatim prefix; the complete units were read.

## Verdict

PASS — the changed existing requirement/procedure records preserve their authority and accurately qualify the evidence; operational code acceptance remains subject to SR-14220.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Base checklist reviewed existing FR-359/TC-931 status/procedure and unchanged AC-8..12 authority. IDs and relationships are unchanged; changed Markdown validation passed. Recorded app versions and measured run outcomes describe actual executions, rather than pinning dependencies in prose. The runner uses Git for detached input checkout, not a second hash/manifest catalog. Existing external receipt instructions retain consumer ownership and do not manufacture local tags. The procedure permits actual ignored tests, confines trace to #[test], requires unchanged proof domains/model/production calls and retains the separately qualified refusal-order counterexample. The documented weekly reserved-host lane satisfies the ticket's scheduled-lane option. No new requirement statement, artifact structure, relationship edge, domain object, verification catalog, risk allocation or system boundary changed, so ears/integrity/dependency/object/evidence/risk/scope/failure-domain sub-analyses are not applicable. Optional Jev criterion-strength was not selected; no calibrated claim is made. Broader inherited trace gaps are reported in SR-14221. Operational cleanup correctness is reported in SR-14220; the intended cleanup procedure itself is not contradictory. No dummy proof tag, computed Covered claim or consumer acceptance is stated.

## Examined Scope

```yaml
scope:
- id: FR-359-AC-8
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Reordered and repeated requests preserve the admitted high-water maximum observable through
    `consumed()`; the diagnostic log record is verified under FR-359-AC-12. With `IntegerBits` limit 5
    and requests 7 then 9 for that kind in one charge, refusal reports `next_charge = 7`, the first-inserted
    failing amount; the existing 9 then 7 case alone cannot distinguish maximum coalescing before refusal.
    Simultaneous shortages refuse in `ScalarLimits` field order; ordinary refusal preserves counters,
    high-water, admissions, log and a pending injected occurrence. Cancellation and injection precedence,
    c
- id: FR-359-AC-9
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference` uses production
    `Meter::charge` for the exact three-charge FR-362 schedule (operand charge with at most two semantic-size
    requests, arithmetic charge with one, result-retain charge with none) over the full `i64` operand
    range, plus separate repeated-kind charge controls with at most two requests each. Symbolic `i64`
    operands and symbolic limits including `ResultUnits` cover first-short and simultaneous shortages
    and absence of the third admission. Under `--unwind 3 --solver cadical`, 16 GiB and 600 seconds, the
    basel
- id: FR-359-AC-10
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: After CG re-locks to the reviewed kernel, the five unchanged consumer tests below pass their
    own assertions under their original domains, oracle, completion and counterexample checks, unwind
    3, Cadical, 16 GiB and 600 seconds. Record their individual Kani verdicts as an external IR-705/IR-17
    receipt; no quire-exact test or altered consumer harness substitutes for it.
- id: FR-359-AC-11
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Inspection of semantic-size processing in production `Meter::charge` and called helpers finds
    no traversal with a trip count greater than that charge's semantic-size request count, including no
    traversal of `LimitKind::ALL` or another independently sized collection. The FR-362 proof schedule
    has at most two requests per charge, while the public `Charge` API retains no request-count ceiling.
- id: FR-359-AC-12
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: Under `test-support`, the admitted-charge log for the three-charge FR-362 schedule contains
    exactly the admitted prefix in charge order, with each record's point and absent-or-present per-admission
    semantic-size maxima matching an independent model. Reordered and repeated requests of the same kind,
    including zero and distinct amounts, coalesce to the per-admission maximum in that record. A refused
    charge appends no record and leaves the existing log unchanged. The named `tc_931_admitted_log_matches_model_ordered_prefix`
    and `tc_931_admitted_log_matches_model_same_kind` tests verify these prope
- id: spec/functional/FR-359-cumulative-meter-boundary.md
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: '---

    id: FR-359

    title: "Cumulative meter boundaries and request-proportional charging"

    type: FR

    relationships: []

    ---

    # FR-359: Cumulative meter boundaries and request-proportional charging


    ## Description


    When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an `Incomplete`
    record for the first unavailable counter without wrapping or changing any consumed counter or admission
    count. The meter SHALL keep semantic-size charge traversal proportional to that charge''s request
    count while preserving refusal order and admitted accounting. A standalone meter proof '
- id: spec/test-cases/TC-931-single-pass-meter-accounting.md
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: "---\nid: TC-931\ntitle: \"Request-proportional meter accounting and bounded integer-arithmetic\
    \ proof\"\ntype: TC\nrelationships:\n  - target: ix://agent-ix/quire-exact/FR-359\n    type: verifies\n\
    ---\n# TC-931: Request-proportional meter accounting and bounded integer-arithmetic proof\n\n## Description\n\
    \nVerify FR-359-AC-8, AC-11 and AC-12 (delivered), AC-9 (IR-844 standalone proof and mutation controls,\
    \ with the IR-810 persistent gate) and record the permanently external AC-10 receipt. Kernel accounting\
    \ and its Kani obligation belong to quire-exact. The five unchanged CG scalar tests are a separate\
    \ consum"
- id: docs/meter-kani-gate.md
  path: docs/meter-kani-gate.md
  role: examined
  excerpt: '# Meter proof gate


    `make kani-meter-gate` runs the real standalone Meter proof and its production

    `LimitKind::ALL` trip-count mutation control. Its ignored integration test is the

    executable FR-359-AC-9 binding; it invokes the gate and asserts its result. The

    Kani proof itself stays untagged. This lane is separate from `make ci`.


    Run from a clean, committed checkout on a Linux host with cgroup v2, a systemd

    user manager with memory-controller delegation, Python 3, and at least 20 GiB

    free disk. Install Kani and its backend first (`cargo kani --version` must report

    both Kani and CBMC), and pr'
```
