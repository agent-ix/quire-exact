---
id: SR-17940486709564847047712
title: "IR-1022 PR #152 spec-review (base, with EARS and integrity checks)"
type: SpecReview
analysis: base
date: 2026-11-12
scope: "agent-ix/quire-exact PR #152 at e88fc800 (base main e20118a); IR-1022; spec/functional/FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md, spec/test-cases/TC-944-native-integer-division-wide-multiply-equality-and-charge-schedule.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-397
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-944
    type: reviews
---

## Summary

Ticket: IR-1022. Spec review of FR-397 Behavior 6, FR-397-AC-6, TC-944 Description (range widened to AC-6) and the new TC-944 step 5 (old step 5 renumbered to 6, text unchanged). EARS and integrity were checked inline here: the change adds one behavior, one AC and one step and no relationship edge.

`quire validate --scope .` on both files: exit 0 (pre-existing `semantic.inline-data-schema` and module-duplicate notices only). `quire matrix --scope .`: FR-397-AC-6 `tagged` by the two new unit tests.

Truth of the text against the code: Behavior 6 and AC-6 are true of the implementation (see the code-review artifact for the line-by-line comparison). The grid bounds in AC-6 (work 0-5, results 0-2, occurrences 0-2) bracket every threshold of the one-pair plan (occurrences 1, work reservation 3, result reservation 1). On the by-construction clause: absence of bignum arithmetic is in fact visible at the type level, since `charge_units` takes only `UnitsCharge` (u64 fields) and `charge_plan_one_pair` takes no amount; the cheap stronger check is to have the AC-6 test assert the schedule's own requests (code-review FND-001), after which the by-construction sentence is an honest residue.

## Verdict

Accurate, testable and bounded, but AC-6 bundles two obligations, its subjects are crate-internal while TC-944 still claims public-API verification, and Behavior 6 names only the Boolean kernel although the Integer kernel shares the schedule. One medium, two low. Small text edits in the same files; recommended in this PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-397-AC-6 is compound: obligation (a) "each of the plan-form, pair and result-retain requests equals a Common Charge and not a General one" and obligation (b) "the u64 equality.plan charge for a one-pair plan returns the same result, counters, admission count and captured event as Meter::charge_plan of one pair over the grid and an injected denial" are independent and verified by two different tests, and a third, non-normative sentence ("Absence of bignum arithmetic ... is by construction") sits in the criteria cell. Split into AC-6 (requests are Common) and AC-7 (one-pair plan equals charge_plan), each with its own binder, and move the by-construction sentence to Behavior 6 or Description. | spec/functional/FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md:51 |
| FND-002 | low | TC-944 Description still says it verifies "FR-397-AC-1 through FR-397-AC-6 through the public API", but step 5 runs crate-internal accounting unit tests over `pub(crate)`/private items (`UnitsCharge`, `ChargeStorage::Common`/`General`, the unnamed u64 plan charge). Say that AC-6 is verified by in-crate unit tests, or scope the public-API phrase to AC-1 to AC-5; and name the u64 plan charge (`Meter::charge_plan_one_pair`) in AC-6 so the subject is unambiguous. | spec/test-cases/TC-944-native-integer-division-wide-multiply-equality-and-charge-schedule.md:14, :22 |
| FND-003 | low | Behavior 6 names only `evaluate_native_boolean_equality`, but `evaluate_native_integer_equality` charges through the same `charge_equality_schedule` and is changed identically; and "the observable outcome of the charges is unchanged" is history-relative (unchanged from a previous implementation a reader cannot see). Name both equality entries, and state the reference: the outcome, events and counters equal `planned_equality`'s (as AC-3 and AC-5 already require). | spec/functional/FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md:40 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-397 Behavior 6 | examined | Native Boolean schedule. evaluate_native_boolean_equality makes its four charges as all-u64 requests, so no charge it makes carries arbitrary-precision Integer storage; the observable outcome of the charges is unchanged. |
| FR-397-AC-6 | examined | Each of the equality.plan-form, equality.pair and equality.result-retain requests of the Boolean equality schedule equals a Common (all-u64) Charge and not a General one, and the u64 equality.plan charge for a one-pair plan returns the same result, counters, admission count and captured event as Meter::charge_plan of one pair at every combination of work_units 0 to 5, result_units 0 to 2 and value_occurrences 0 to 2 and under an injected denial. Absence of bignum arithmetic in the code path is by construction and is not otherwise host-observable. |
| TC-944 Description | examined | Verify FR-397-AC-1 through FR-397-AC-6 through the public API by running each native entry and the existing Integer entry on the same values ... |
| TC-944 step 5 | examined | In the accounting unit tests, build the three u64 requests of the Boolean schedule and assert each equals the Common form of the matching Charge, and run the one-pair u64 plan charge and charge_plan of one pair on separate meters over the limit grid and an injected denial, comparing result, counters, admissions and captured events. |
| TC-944 step 6 | examined | renumbered from 5, text unchanged |
| FR-397 statement, Behavior 5, AC-5 | context_only | SHALL sentence and the Boolean equality behavior Behavior 6 refines |

## Dispositions

Round 1, fix head e34ec5c3 (reviewed head e88fc800); merge-tree with origin/main e20118a clean. `quire validate` exit 0 on FR-397, TC-944 and the three committed review files (pre-existing inline-data-schema notice only); `quire matrix` tags FR-397-AC-6 at src/accounting/units.rs:282 and FR-397-AC-7 at :306.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-6 now covers only the three requests being Common; AC-7 covers charge_plan_one_pair vs charge_plan over the grid and injected denial; the by-construction sentence moved to a new Notes section; each AC has its own binder. |
| FND-002 | fixed | TC-944 Description scopes "through the public API" to AC-1 through AC-5; step 5 says in-crate accounting unit tests on private types; AC-7 names Meter::charge_plan_one_pair. |
| FND-003 | fixed | Behavior 6 names both evaluate_native_boolean_equality and evaluate_native_integer_equality and states they return planned_equality's value, outcome, charge events and counters. |
