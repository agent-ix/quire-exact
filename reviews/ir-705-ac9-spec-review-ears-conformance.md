---
id: SR-11713
title: IR-705 AC-9 log split spec-review/ears-conformance review
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-exact@acc27246af8319efa79934291bb29437b8562e31; spec/functional/FR-359-cumulative-meter-boundary.md, spec/test-cases/TC-931-single-pass-meter-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-705. PR agent-ix/quire-exact#51, head acc2724, base origin/main 02625b8; diff `git diff origin/main...acc2724` (FR-359 + TC-931). Spec-only, reviewer-only: no cargo or Kani run; src/accounting.rs at acc2724 and the IR-705 code branch harness (55d7e36, src/accounting/kani_proofs.rs) read as context. Method: quoin:spec-review/ears-conformance.

## Scope

```yaml
scope:
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: When a charge would exceed a cumulative `u64` limit, the `quire-exact` meter SHALL return an `Incomplete`
    record for the first unavailable counter without wrapping or changing any consumed counter or admission count.
    The meter SHALL keep semantic-size charge traversal proportional to that charge's request count while preserving
    refusal order and admitted accounting.
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The meter SHALL preserve field-order refusal, the first-inserted failing amount within a kind, same-kind
    per-admission maxima, atomicity, injection and cancellation order, `consumed()` for every kind, and the FR-358
    log.
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The standalone Kani obligation uses production `Meter::charge` ... The implementation lane must record
    that Kani compiles this `#![no_std]` crate for the tagged harness.
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: 'The explicit non-CI lane command is `cargo kani --harness "$HARNESS" --unwind 3 --solver cadical`, where
    `HARNESS` is the symbol of the `cfg(kani)` harness tagged `Trace: FR-359-AC-9`, with a 16 GiB memory ceiling
    and 600-second timeout.'
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: examined
  excerpt: The log is a test-support diagnostic absent from production builds; asserting its contents in the Kani
    obligation exhausted a 16 GiB memory ceiling (15.4 GiB measured), and no sound reduction was found. Kani retains
    the consumer-facing refusal order, atomicity, termination and counter proof, while the named AC-8 unit tests
    are planned to check the diagnostic log cases above exhaustively.
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Assessment

Examined FR-359 Description, Behavior :17, :19, :21 and Notes :65.

## Verdict

PASS: no EARS defects. The FR-359 Description requirement statements are unchanged and remain event-driven EARS ('When a charge would exceed ..., the meter SHALL ...'). The changed Behavior lines 19 and 21 are implementation-lane process instructions ('must record'), as before, not requirement statements. The new Notes section is non-normative (no SHALL). AC rows are verification criteria, not EARS statements.

## Dispositions

Round 1 re-check at head e25295d (diff acc2724..e25295d). The review pass found nothing, so there is nothing to dispose. The re-check is still clean. The new FR-359-AC-12 is a verification criterion, not an EARS requirement statement. The FR-359 Description and Behavior SHALL statements are unchanged. The rewritten Notes include a lane process instruction ('The implementation lane must measure and report that configuration'). That matches the existing Behavior process instructions and adds no new SHALL requirement.
