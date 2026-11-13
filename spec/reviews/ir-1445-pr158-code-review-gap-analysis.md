---
id: SR-17940486709564847047934
title: "IR-1445 PR #158 gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact PR #158 at cf32ace5, git diff origin/main...HEAD; IR-1445; FR-404-AC-1..AC-9, FR-399-AC-6, FR-399-AC-7; src/native_text.rs, src/native_text/normal_form.rs, src/native_text/kani_proofs.rs, src/text.rs, tests/ir1445_native_text.rs, spec/test-cases/TC-952-kani-tractable-native-text-carrier.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-404
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-399
    type: reviews
---

## Summary

Ticket: IR-1445. Plan completion: not assessed. `quire matrix` at cf32ace5 (quoin 0.29.1) shows
FR-404-AC-1 to AC-5 `tagged` by `tests/ir1445_native_text.rs`, and AC-6 to AC-9
`method-without-symbol` (Inspection), bound by `probe_records_are_executed_and_choose_the_capacity`
(AC-6, AC-8, AC-9) and `normalizing_profiles_are_supported_only_on_a_verified_record` (AC-7).
All 12 tests pass. Every new production item has an owning criterion: `NativeText`
(FR-404-B1/B2), `from_admitted` (B3/B4 with FR-399-B2), `compare_native_text` (B6/B7),
`proof_path_supports` (B10), the harness (B9), `TextNotAdmittedCause` (FR-399 Outputs). No stubs.

Bindings checked: AC-1 (Copy bound, `needs_drop`, fn-pointer coercions, `allocation_counter`
window at C bytes), AC-2 (zero tail and equality over 0, 1, C - 1, C with one- to four-byte tails),
AC-3 (C - 1 and C held, C + 1 and 2C refused with ASCII and continuation-byte tails, retained
C + 1 refused, C never refused), and AC-4 (all six operators; zero, exact and one-short budgets;
outcome, counters and, under `test-support`, the ordered charge log; prefixes, last-byte pairs,
`x` and `x\0`, distinct profiles). These are correct. AC-6 and AC-8 read real rows and choose C
correctly; the reviewer reproduced the C = 64 row.

Behavior 8 / L_b (question 6): CG FR-052-AC-14 is on CG main. Its L_b is L - 1 by dropping the
final byte for `binary-utf8`, and, for `unicode-scalars`, the length-L payload with its final
four-byte scalar replaced by a three-byte one. The PR's family `filler(C, "🦀")` against
`filler(C - 1, "€")`, and `filler(C, "b")` against `filler(C - 1, "b")`, contains exactly those
length pairs, plus last-byte pairs at L. So the assumed multibyte-tail family covers the AC-14
lengths for both supported profiles. It does not build AC-14's first-byte pair (FND-003).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | Coverage inflation on AC-7 and AC-9. `normalizing_profiles_are_supported_only_on_a_verified_record` (tagged FR-404-AC-7) passes on the row outcome `NOT RUN`: it only asserts `!= "PENDING"` and supported == (outcome == VERIFIED). But AC-7 requires "the measurement of Behavior 10 is recorded, with the FR-401 normalization instantiated at the chosen C and run under Kani". `probe_records_are_executed_and_choose_the_capacity` is tagged FR-404-AC-9 but never reads the FR-401 row, while AC-9 requires AC-7 to cite an executed run. Both tags report AC-7 and AC-9 as covered when neither criterion holds. Remove the AC-7 and AC-9 tags, or make the AC-9 binding assert that the FR-401 row is an executed terminal outcome, which correctly fails today. Do not merge with a binder that passes on NOT RUN. | tests/ir1445_native_text.rs:784-786, tests/ir1445_native_text.rs:827-845 |
| FND-002 | medium | FR-399-AC-6 and AC-7 apply "when the carrier of FR-404 exists". This PR makes the carrier exist, and both criteria are `untagged`. Their owner, IR-1499, is Done with spec only (FR-399 Status: "Planned (IR-1499). No Rust, test or proof exists"), so the activated criteria have no live ticket. Name a live owning ticket for them in the FR-399 / FR-404 Status, or make FR-404 Status say they stay inactive until the FR-399 constructor lands. | spec/functional/FR-399-non-metered-text-constructor-from-admitted-parts.md:58-59 |
| FND-003 | low | The Behavior 8 family is not built by the construction of CG FR-052-AC-14. The lengths and length pairs are covered (see Summary), but: there is no first-byte pair at L or L_b (AC-14 (iii) requires one for `binary-utf8` and `unicode-scalars`); the length-L `unicode-scalars` payload is 'a' filler plus a tail, not four-byte self-normal scalars; and the family `TextType` is `Text[0, 8C]` rather than the operand's declared `max`. FR-404-AC-4 names only lengths and its own pairs, so this is low. Cite AC-14 in the `family()` comment and add first-byte pairs at C and C - 1 so the family matches CG's when CG consumes it. | tests/ir1445_native_text.rs:268-305 |
| FND-004 | high | `probe_records_are_executed_and_choose_the_capacity` is tagged FR-404-AC-9 (with AC-6 and AC-8) but never reads the FR-401 row. AC-9 makes Done conditional on AC-7 citing an executed run, so this binder passes while AC-9 is unmet. Drop the AC-9 tag, or assert that the FR-401 row is an executed terminal outcome, which correctly fails today. (FND-001 is the AC-7 binder of the same inflation.) | tests/ir1445_native_text.rs:784-825 |

## Verdict

Not mergeable. AC-1 to AC-6 and AC-8 are bound by real, passing tests. AC-6 and AC-8 are backed
by executed probe rows that reproduce. AC-7 and AC-9 are claimed by binders that cannot fail on the
current `NOT RUN` record (FND-001, FND-004); that tag inflation must be removed in this PR. FND-002 needs an
owning-ticket statement. FND-003 is low.

## Dispositions

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | The AC-7 tag was removed from `normalizing_profiles_are_supported_only_on_a_verified_record`, whose doc now says it is a Behavior 10 consistency check that binds no AC. |
| FND-002 | deferred: the owner ruling belongs to the lead and the planner (the relay is down). Merge is acceptable only while IR-1445 stays open as the holding ticket for this AC; the lead must record the ruling on IR-1445 and replace 'owner pending planner ruling' with the ticket id before IR-1445 closes. FR-404 and TC-952 Status now state that FR-399-AC-6 and AC-7 are active and untagged and that IR-1499 is spec-only. |
| FND-003 | fixed | `ac14_family` builds the CG FR-052-AC-14 family at the operand `max` over `Text[0, max]`: L = C, L_b = C - 1, and final-byte, length and first-byte pairs for `binary-utf8` (max C) and `unicode-scalars` (max C/4, a U+1F980 unit, a U+1F981 final-byte variant, a U+5F980 lead variant, a U+20AC L_b unit). Differential test `replacement_matches_shipped_compare_over_the_ac14_family` passes. |
| FND-004 | fixed | `probe_records_are_executed_and_choose_the_capacity` is now tagged AC-6 and AC-8 only; AC-9 is unclaimed. |
