---
id: SR-16644
title: IR-1030 PR 90 code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; src/accounting.rs; tests/ir673_core.rs
review_set: subset
---

# SR-16644: IR-1030 PR 90 code-review

## Summary

Independent code-review on frozen head 1c88adf1e0748c7b1de9600f44de5a0d63e489a3 against main bff0970a0b9a027819e38c475fe3b309e0c86856. Ticket IR-1030. Model gpt-6.1-sol; run ee6396ab-9bc6-45cc-b3a4-10938c6e1c6f. No Cargo build run by reviewer; author has focused checks queued. No applicable AssuranceProfile found.

## Verdict

PASS: no defects in the Rust code or tests.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The two documented enum variants are appended at 63/64, all old 63 entries remain, the exhaustive test independently names every variant and tests exact names, round trips and near misses. QSL origin/main 1348ba0 retains its independent FrameCharge decision and model Meter. No copied code, shared meter, new traversal, public counter or gate/config change.

## Review data

```yaml
clean: true
scope:
- id: FR-368-AC-1
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL`,
    has a unique documented spelling from `as_str()` and round-trips through `from_code()`;
    an unknown spelling returns `None`. `TemporalVisit` spells `temporal.visit` and
    is at index 62 after `ModelNavigate` at index 61; entries 0 through 61 retain
    their pre-extension order. Exhaustive variant matching makes an added but unaccounted
    variant fail the check.
- id: FR-368-AC-2
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: 'Every public `LimitKind` variant occurs exactly once in `LimitKind::ALL`,
    has a unique documented `as_str()` spelling, and the sequence is exactly the ten
    `ScalarLimits` fields: integer bits, decimal digits, scale expansion, text input
    bytes, text scalars, normalized scalars, unit edges, value occurrences, work units
    and result units. Exhaustive variant matching makes an added but unaccounted variant
    fail the check.'
- id: FR-368-AC-3
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: The public typed variants `FrameWrite` and `FrameRedefinesLink` spell exactly
    `frame.write` and `frame.redefines-link`; each round-trips through `from_code()`,
    while misspellings return `None`. `ChargePoint::ALL` contains exactly 65 distinct
    variants, with `TemporalVisit` at index 62, `FrameWrite` at 63 and `FrameRedefinesLink`
    at 64; entries 0 through 62 retain their earlier identities and order.
- id: FR-368-AC-4
  path: spec/functional/FR-368-meter-charge-and-limit-vocabulary.md
  role: examined
  excerpt: Inspection of the public kernel API and the caller's frame path identifies
    the two typed names in the kernel vocabulary and the caller's independent frame
    event, meter and `ModelNormalizationLimits.work_units` ownership. No new kernel
    frame traversal or counter sharing with `ScalarLimits` is introduced.
findings: []
bindings:
- test_id: charge_point_vocabulary_is_unique_and_round_trips
  ac_id: FR-368-AC-1
  trace: correct
- test_id: charge_point_vocabulary_is_unique_and_round_trips
  ac_id: FR-368-AC-3
  trace: correct
```
