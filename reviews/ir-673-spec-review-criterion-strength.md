---
id: SR-2422
title: "Criterion-strength review of quire-exact IR-673 spec candidate 05ffdbe"
type: SpecReview
analysis: criterion-strength
scope: "agent-ix/quire-exact@05ffdbe0f6b9f5a2be69fef52874eb1776e18cc3; origin/main...05ffdbe: new and amended ACs in FR-097, FR-358, FR-359, FR-361, FR-362, FR-363, FR-364, FR-365, FR-366, FR-367"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-366
    type: reviews
---
# Criterion-strength review of quire-exact IR-673 spec candidate 05ffdbe

## Summary

Ticket: IR-673. Pre-PR candidate `05ffdbe0f6b9f5a2be69fef52874eb1776e18cc3`. Reviewer model `claude-opus-5-5`, run `720faf90-5d3c-40d8-be4d-1aaae6430466`.

Jev (typesafe.ai System One), the skill's calibrated judge, is not installed on this machine (`which jev` finds nothing). This is a manual judgment over the spec text and owner source, recorded as such.

For each new or amended AC, the question was whether a plausible wrong implementation would still pass. Most ACs pin exact values, exact counters and exact refusal shapes at both sides of a boundary:

- FR-097-AC-8 checks below, endpoints, interior, above and both `u64` extremes.
- FR-365 covers all six modes on both signs, plus the default.
- FR-362-AC-10 checks exact and one-under limits.
- FR-367 uses all-zero limits, so any charge before the profile check would show up.

FR-364-AC-1 can fail despite the bitmask representation, because it fixes the order of `iter()`. FR-366-AC-4 pins strict key order across all ten classes and payload direction.

## Verdict

**CONDITIONAL**: one medium finding. All other examined ACs can fail on a plausible defect.

## Examined scope

| Unit | Role | Path:line | Excerpt |
| --- | --- | --- | --- |
| FR-097-AC-8 | examined | spec/functional/FR-097-unbounded-collection-in-the-kernel.md:28 | For `[2, 4]`, counts 1, 2, 3, 4 and 5 return below-minimum, none, none, none and above-maximum ... |
| FR-358-AC-5, FR-358-AC-7 | examined | spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md:27 | post-cap exact accounting; exhaustive vocabulary census |
| FR-359-AC-6, FR-359-AC-7 | examined | spec/functional/FR-359-cumulative-meter-boundary.md:26 | first-short counter in field order; total `consumed` reader |
| FR-361-AC-3, FR-361-AC-6 | examined | spec/functional/FR-361-admission-before-large-exact-work.md:31 | exact/one-under floor division; Multiply retain-upscale record |
| FR-362-AC-3, AC-7, AC-10, AC-11, AC-12 | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:39 | domain refusal; P11/Q11 schedules; exact/one-under bits; dispositions; constructor canonical form |
| FR-363-AC-1, FR-363-AC-3 | examined | spec/functional/FR-363-metered-decimal-ordering.md:35 | normalized equality vs retained charges |
| FR-364-AC-1 | examined | spec/functional/FR-364-ieee-flag-iteration.md:21 | For every subset of the five public flags ... |
| FR-365-AC-1, FR-365-AC-2 | examined | spec/functional/FR-365-decimal-rounding-ties-and-default.md:22 | six-mode tie table; default Exact refuses 0.1 |
| FR-366-AC-1, AC-3, AC-4 | examined | spec/functional/FR-366-ieee-exceptional-value-semantics.md:21 | leftmost NaN, signaling Invalid; signed zero loss; total-order keys |
| FR-366-AC-2 | examined | spec/functional/FR-366-ieee-exceptional-value-semantics.md:22 | Converting a binary64 NaN with payload `2^22` or `2^30` to binary32 returns `Refused(IeeeNanPayloadNotRepresentable { source: Binary64, target: Binary32 })` without truncating the payload or retaining a result unit. |
| FR-367-AC-1 | examined | spec/functional/FR-367-text-profile-mismatch-before-charging.md:21 | ... whose ten limits are zero ... returns precisely `Err(IllTyped { cause: DistinctTextProfiles })` |

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | FR-366-AC-2 tests only payloads that must refuse (`2^22`, `2^30`). An implementation that refuses every binary64-to-binary32 NaN narrowing, or truncates fitting payloads, still passes. The Behavior rule "refuses a payload that cannot fit below the target quiet bit" therefore has no lower side. Source refuses at `payload >= target quiet_bit` (`src/ieee.rs:1667-1668`). Fix: add a fitting case at the boundary: payload `2^22 - 1`, positive and negative, converts to binary32 with sign and payload preserved and the quiet bit set. | spec/functional/FR-366-ieee-exceptional-value-semantics.md:22 | wrong-requirement |
