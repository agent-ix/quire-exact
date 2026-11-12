---
id: SR-17940486709564847047711
title: "IR-1022 PR #152 gap-analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-12
scope: "agent-ix/quire-exact PR #152 at e88fc800 (base main e20118a); IR-1022; src/accounting/units.rs, src/native_integer.rs, src/equality.rs, spec/functional/FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md, spec/test-cases/TC-944-native-integer-division-wide-multiply-equality-and-charge-schedule.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-397
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-944
    type: reviews
---

## Summary

Ticket: IR-1022. Planless gap analysis of the PR diff. Plan completion: not assessed.

- `quire matrix --scope .`: FR-397-AC-1 to AC-6 all `tagged`. AC-6 is bound by `src/accounting/units.rs:257` (`the_equality_schedule_requests_are_native_common_charges`) and `:292` (`one_pair_plan_in_u64_equals_the_integer_plan_charge`), both `#[trace("TC-944", "FR-397-AC-6")]` (attribute form, not doc lines). AC-1 to AC-5 keep their existing binders in `tests/ir1422_native_entries.rs`, which the PR does not touch.
- `quire validate --scope .` on FR-397 and TC-944: exit 0; the only diagnostic beyond module-duplicate notices is the `semantic.inline-data-schema` line, which is pre-existing on main.
- New code has an owning requirement: `Meter::charge_plan_one_pair` and `UnitsCharge::work` exist only to serve `charge_equality_schedule`, owned by FR-397 Behavior 6 / AC-6. No stubs, no dead code, no test-only production branches.
- Coverage by charge point: plan-form, pair and result-retain go through `charge_units`, shared with `Meter::charge`; the plan charge is covered by the AC-6 grid (every check arm: `value_occurrences` 0, work 0-2, results 0, admit, injected denial) and by the AC-5 kernel sweep and per-point denial test. Each reviewer mutant was killed by a named test (see the code-review artifact). The grid starts every meter at zero consumption; the consumed > 0 case of the plan is reached through the AC-5 sweep (plan-form spends 2 work units first).
- Not covered by the grid: a cancelled `Cancel` handle and a full capture (`can_record` false). Both go through code shared verbatim with `charge_plan` (`injected_denial`, `record_capture`), so no divergence is possible there; not a finding.

## Verdict

No gaps: every new criterion has binding tests that can fail, the unchanged differential suite still guards both equality kernels, and no new code lacks an owner. The test-intent weakness of the AC-6 Common-form test is recorded once, in the code-review artifact (FND-001), not repeated here.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-397-AC-6 | examined | Each of the equality.plan-form, equality.pair and equality.result-retain requests of the Boolean equality schedule equals a Common (all-u64) Charge and not a General one, and the u64 equality.plan charge for a one-pair plan returns the same result, counters, admission count and captured event as Meter::charge_plan of one pair at every combination of work_units 0 to 5, result_units 0 to 2 and value_occurrences 0 to 2 and under an injected denial. |
| FR-397-AC-5 | examined | evaluate_native_boolean_equality over the four (left, right) pairs returns planned_equality's value over Value::Boolean operands with equal outcome, charge events and counters, at unlimited limits, at every value_occurrences, work_units and result_units limit ... and with a denial injected at each of the four equality points |
| FR-397-AC-3 | examined | evaluate_native_integer_equality over equal, unequal, signed and different-width operands returns planned_equality's value over Value::Integer operands with equal charge events |
| src/accounting/units.rs charge_plan_one_pair, UnitsCharge::work | examined | new production code, owned by FR-397 Behavior 6 |
| src/native_integer.rs charge_equality_schedule | examined | shared by evaluate_native_integer_equality and evaluate_native_boolean_equality |
