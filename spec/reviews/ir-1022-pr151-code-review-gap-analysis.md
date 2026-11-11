---
id: SR-17940486709564847047708
title: "IR-1022 PR #151 gap-analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-11
scope: "agent-ix/quire-exact PR #151 at f3a25430 (base main 1f07457); IR-1022; src/native_integer.rs, src/lib.rs, tests/ir1422_native_entries.rs, spec/functional/FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md, spec/test-cases/TC-944-native-integer-division-wide-multiply-equality-and-charge-schedule.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-397
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-944
    type: reviews
---

## Summary

Ticket: IR-1022. Planless gap analysis of the PR's change set. Plan completion: not assessed.

- Matrix: `quire matrix --scope .` lists FR-397-AC-5 as `tagged`, bound by `tests/ir1422_native_entries.rs:386` (`boolean_equality_matches_planned_equality_over_boolean_values`, `#[trace("TC-944", "FR-397-AC-5")]`). FR-397-AC-1 to AC-4 keep their binders. No FR-397 criterion is untagged.
- Code to requirement: the only new production item, `pub fn evaluate_native_boolean_equality` in `src/native_integer.rs` and its crate-root re-export in `src/lib.rs`, is owned by FR-397 Behavior 5, Outputs and AC-5. No new code lacks an owning requirement.
- Stubs and coverage inflation: none. The kernel is a real implementation (the shared `charge_equality_schedule` and `left == right`). The test is a live differential against `planned_equality` over `Value::Boolean` on all four pairs, comparing the full `(Outcome, Vec<ChargeEvent>, counters)` tuple at unlimited limits and at every bound `0..=spend` of four limit kinds. It is not tautological: four reviewer mutants (pair count 2, result-retain as work, Incomplete swallowed, plan-form sized 2) each fail it by name, and the coder reports three more.
- Semantic agreement (intent, test, code): the AC intent (identical observable behavior to `planned_equality` on two Booleans) matches the code and the test. The deviation from the ticket research (a differential against `sv::TypeEnvironment::check_equality(..).evaluate(..)`) is equivalent at the kernel level: in quire-semantic-value `CheckedEquality::run`, a typed `Boolean` operand with no conversion target passes through `operand_value` with no charge, the `Plan` schedule calls `quire_exact::planned_equality` directly, and `NotEqual` is the negation of its result after the same charges. So `planned_equality` over `Value::Boolean` is the reference that route reduces to; what remains (the operator negation for `!=` and the generation-time type check) is consumer-side and is correctly left to CG #800.

## Verdict

No coverage gap. AC-5 is bound to a test that asserts it, the test fails on behavior changes, and the new code is fully owned by FR-397. One low gap: the charge points `equality.pair` and `equality.result-retain` are never the stopping point anywhere in the limit sweep (see FND-001), so AC-5's denied-charge equality is evidenced only at plan-form and plan. That is a property of the meter's reservations, not a test defect, and the injected-denial facility could cover it if wanted.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The limit sweep never stops at `equality.pair` or `equality.result-retain`. Reviewer probe over `(true, false)`: `value_occurrences` bound 0 stops at `EqualityPlanForm`; `work_units` bounds 0-1 stop at `EqualityPlanForm` and 2-4 at `EqualityPlan` (the plan charge reserves pairs+2 work and one result unit); `result_units` bound 0 stops at `EqualityPlan`; `integer_bits` spend is 0. So AC-5's equal-outcome-on-denial obligation is evidenced at two of the four charge points; a Boolean-only fault in the denial handling of the last two is not exercised. Optionally add an injected denial (`InjectedDenial`, test-support feature) of each of the four points and compare both sides. | tests/ir1422_native_entries.rs:400-415 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-397-AC-5 | examined | `evaluate_native_boolean_equality` over the four `(left, right)` pairs returns `planned_equality`'s value over `Value::Boolean` operands with equal outcome, charge events and counters, at unlimited limits and at every limit from zero to the unlimited spend of each counted limit kind. |
| FR-397 Behavior 5 | examined | `evaluate_native_boolean_equality` ... charges the `equality.plan-form`, `equality.plan`, `equality.pair` and `equality.result-retain` points `planned_equality` charges for two `Boolean` values ... and not the `boolean.result-retain` point of `retain_boolean`. |
| src/native_integer.rs evaluate_native_boolean_equality | examined | Outcome::from_stop(charge_equality_schedule(meter).map(\|()\| left == right).map_err(Stop::from)) |
| src/lib.rs re-export | examined | pub use native_integer::{ ..., evaluate_native_boolean_equality, ... } |
| tests/ir1422_native_entries.rs boolean_equality_matches_planned_equality_over_boolean_values | examined | #[trace("TC-944", "FR-397-AC-5")] differential over four pairs and a 0..=spend sweep of four limit kinds |
| FR-397-AC-1 to AC-4 matrix rows | context_only | existing binders unchanged |
