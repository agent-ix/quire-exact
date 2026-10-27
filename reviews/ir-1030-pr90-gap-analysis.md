---
id: SR-16645
title: IR-1030 PR 90 gap-analysis
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact; spec/functional/FR-368-meter-charge-and-limit-vocabulary.md;
  src/accounting.rs; tests/ir673_core.rs
review_set: subset
---

# SR-16645: IR-1030 PR 90 gap-analysis

## Summary

Independent gap-analysis on frozen head 1c88adf1e0748c7b1de9600f44de5a0d63e489a3 against main bff0970a0b9a027819e38c475fe3b309e0c86856. Ticket IR-1030. Model gpt-6.1-sol; run ee6396ab-9bc6-45cc-b3a4-10938c6e1c6f. No Cargo build run by reviewer; author has focused checks queued. No applicable AssuranceProfile found.

## Verdict

PASS for this PR diff; whole-repository strict traceability remains non-green from inherited obligations.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed. Quire matrix for this head: 203 criteria, 172 tagged, 19 inspection method-without-symbol, 12 untagged. FR-368-AC-1, AC-2 and AC-3 bind to real TC-916 tests; AC-4 is Inspection. The 12 untagged criteria are inherited and unrelated to this vocabulary change. Quire coverage reports 86 untracked symbols and 36 unmatched tags, none of the changed FR-368/TC-916 binding. Reverse ownership and stub scan of the changed source/tests found no new unowned behavior, placeholder or hollow assertion. Optional repository-wide semantic review not requested.

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
- test_id: limit_kind_vocabulary_matches_scalar_limit_fields
  ac_id: FR-368-AC-2
  trace: correct
```
