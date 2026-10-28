---
id: SR-16642
title: 'IR-1030 spec-review-evidence of quire-exact #89'
type: SpecReview
analysis: evidence
scope: agent-ix/quire-exact; spec/functional/FR-368-meter-charge-and-limit-vocabulary.md;
  spec/test-cases/TC-916-meter-charge-and-limit-vocabulary.md
review_set: subset
---

## Summary

AC-3 Test matches catalog advice and TC-916; AC-4 Inspection is appropriate for source ownership despite an automated example-shape mismatch. Both remain Planned. Frozen head e1375c894bf63fa3d2df8704032a6d22ba3bad44 against main ce027b3eefc35f0980f72d289254d974a14ea597. Ticket IR-1030.

## Verdict

PASS for this method; code and executable evidence remain planned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

```yaml
scope:
- id: FR-368-Description
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: The `quire-exact` meter SHALL expose a complete, uniquely spelled public `ChargePoint` vocabulary
    and a `LimitKind` vocabulary in `ScalarLimits` field order.
- id: FR-368-Behavior
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: The `ChargePoint` vocabulary SHALL append `FrameWrite` (`frame.write`) at index 63 and `FrameRedefinesLink`
    (`frame.redefines-link`) at index 64, in that order, without changing the identity or order of entries
    0 through 62. These names identify the frame events defined by the authoritative operation-frame and
    value-accounting contracts. The operation-frame caller SHALL own detection and ordering of field writes,
    traversal of `redefines` links, and admission into its own frame meter. This vocabulary extension
    SHALL NOT move frame checking into `quire-exact`, merge the caller's `ModelNormalizat
- id: FR-368-Dependency
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: '- [TC-916](../test-cases/TC-916-meter-charge-and-limit-vocabulary.md) verifies the vocabulary
    criteria through the public enum APIs. [FR-359](./FR-359-cumulative-meter-boundary.md) owns first-short-counter
    behavior that uses the limit-kind order. Operation-frame event and amount semantics remain with `ix://agent-ix/quire-specification/FR-013`
    and its value-accounting definition; QSL owns the independent caller meter.

    '
- id: FR-368-AC-1
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL`, has a unique
    documented spelling from `as_str()` and round-trips through `from_code()`; an unknown spelling returns
    `None`. `TemporalVisit` spells `temporal.visit` and is at index 62 after `ModelNavigate` at index
    61; entries 0 through 61 retain their pre-extension order. Exhaustive variant matching makes an added
    but unaccounted variant fail the check.
- id: FR-368-AC-2
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 'Every public `LimitKind` variant occurs exactly once in `LimitKind::ALL`, has a unique documented
    `as_str()` spelling, and the sequence is exactly the ten `ScalarLimits` fields: integer bits, decimal
    digits, scale expansion, text input bytes, text scalars, normalized scalars, unit edges, value occurrences,
    work units and result units. Exhaustive variant matching makes an added but unaccounted variant fail
    the check.'
- id: FR-368-AC-3
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: The public typed variants `FrameWrite` and `FrameRedefinesLink` spell exactly `frame.write`
    and `frame.redefines-link`; each round-trips through `from_code()`, while misspellings return `None`.
    `ChargePoint::ALL` contains exactly 65 distinct variants, with `TemporalVisit` at index 62, `FrameWrite`
    at 63 and `FrameRedefinesLink` at 64; entries 0 through 62 retain their earlier identities and order.
- id: FR-368-AC-4
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Inspection of the public kernel API and the caller's frame path identifies the two typed names
    in the kernel vocabulary and the caller's independent frame event, meter and `ModelNormalizationLimits.work_units`
    ownership. No new kernel frame traversal or counter sharing with `ScalarLimits` is introduced.
- id: TC-916-Description
  path: spec/test-cases/TC-916-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 'Verify [FR-368](../functional/FR-368-meter-charge-and-limit-vocabulary.md) through the public
    `ChargePoint` and `LimitKind` enums. Scope: FR-368-AC-1 through AC-3. AC-4 is an inspection of the
    kernel/caller ownership boundary.'
- id: TC-916-Expected
  path: spec/test-cases/TC-916-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Every enum variant occurs once, each spelling is unique and documented, charge points round-trip,
    unknown points refuse, and limit kinds follow the ten-field order. The temporal-visit point remains
    after `model.navigate` and before the two ordered frame points. Adding an unaccounted enum variant
    fails the exhaustive match. The frame caller retains its own traversal, admission and work counter.
- id: QSpec-FR-013-AC-7
  path: spec/functional/foundation/FR-013-validate-operation-frames.md
  role: context_only
  excerpt: A write whose member reaches its granted field through `k` `redefines` links consumes exactly
    `1 + k` work units at one `frame.write` and `k` `frame.redefines-link` charges; with `work_units`
    limited to `k`, validation returns `incomplete` at the last `frame.redefines-link` with `consumed
    = k` and `next_charge = 1`, and reports no frame verdict.
- id: QSpec-value-accounting
  path: proposals/quire-v1/definitions/value-accounting.md
  role: context_only
  excerpt: 'for each field write of an invocation, in ascending (object reference key, field declaration
    key) order: one `frame.write` before the write is tested against the frame''s `modifies` grants, then
    one `frame.redefines-link` before each `redefines` link followed from the written member, stopping
    at the first member a `modifies` entry names or at a member with no `redefines` link'
- id: QSL-FrameCharge
  path: qsl-semantics/src/model/observation/frame.rs
  role: context_only
  excerpt: "FrameCharge::Write => ChargePoint::FrameWrite,\n                    FrameCharge::RedefinesLink\
    \ => ChargePoint::FrameRedefinesLink,"
- id: Exact-ChargePoint
  path: src/accounting.rs
  role: context_only
  excerpt: 'pub const ALL: [Self; 63] = ['
- id: TC-916-Procedure-1
  path: spec/test-cases/TC-916-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 1. For AC-1, exhaustively match every public charge-point variant against an independently
    authored name expectation; compare the set with `ChargePoint::ALL`, check uniqueness and each `from_code(as_str())`,
    and reject an unknown spelling. Check that `TemporalVisit` spells `temporal.visit` and is at index
    62 after `ModelNavigate` at index 61, without changing entries 0 through 61.
- id: TC-916-Procedure-2
  path: spec/test-cases/TC-916-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 2. For AC-2, exhaustively match every public limit-kind variant against the named ten-field
    sequence in `ScalarLimits`; compare with `LimitKind::ALL` and check name uniqueness. Include work
    before results at the end.
- id: TC-916-Procedure-3
  path: spec/test-cases/TC-916-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 3. For AC-3, independently enumerate the pre-extension 63 charge-point identities and spellings,
    then require the appended `FrameWrite` and `FrameRedefinesLink` variants at indices 63 and 64 of an
    exactly 65-entry `ChargePoint::ALL`. Assert the exact two spellings, their `from_code()` round-trips,
    refusal of near-miss names, and unchanged first 63 entries. The exhaustive match from step 1 must
    fail to compile if a new variant lacks a name expectation.
- id: TC-916-Procedure-4
  path: spec/test-cases/TC-916-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 4. For AC-4, inspect the kernel's public charge-point/meter API and the caller's operation-frame
    decision path. Identify which component detects writes, follows links, and charges its own frame meter.
    Confirm its `ModelNormalizationLimits.work_units` counter is independent of the kernel's `ScalarLimits.work_units`,
    with no new kernel frame traversal or shared counter.
```

## Coverage

One frozen spec-only diff; no Cargo, Rust test, gate, or QSL consumer acceptance run. Quire 0.36.2; Quoin 0.29.1. No applicable AssuranceProfile in the repo spec tree.
