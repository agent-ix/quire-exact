---
id: SR-17940486709564847047709
title: "IR-1022 PR #151 spec-review (base, with EARS and integrity checks)"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-exact PR #151 at f3a25430 (base main 1f07457); IR-1022; spec/functional/FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md, spec/test-cases/TC-944-native-integer-division-wide-multiply-equality-and-charge-schedule.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-397
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-944
    type: reviews
---

## Summary

Ticket: IR-1022. Spec review of the FR-397 and TC-944 edits in `git diff origin/main...HEAD`: a new Outputs line, Behavior 5, FR-397-AC-5, and TC-944 step 4 (old step 4 renumbered to 5) with the Description range widened to AC-5. EARS conformance and structural integrity were checked inline here rather than as separate sub-analysis artifacts, because the change adds one behavior, one AC and one procedure step and touches no relationship edge.

Checks run: `quire validate --scope .` on both files and on `spec/**/*.md` (exit 0; the only diagnostics are pre-existing module-duplicate notices and FR-390 EARS warnings, none on FR-397 or TC-944). `quire matrix --scope .` lists FR-397-AC-5 as `tagged` by `tests/ir1422_native_entries.rs:386` (the new test), and AC-1 to AC-4 keep their existing binders.

Semantics checked against the code: Behavior 5 and AC-5 are true of the implementation. `planned_equality` on two `Value::Boolean` charges `equality.plan-form` (size `value_occurrences` 1, work 2), `equality.plan` (pairs 1), one `equality.pair` and `equality.result-retain` (`results(1)`) through `equality::charge_plan_form` / `equality::charge_pairs`, because `Value::Boolean(_).occ()` is 1 and `plan_pairs` forms one pair. The native kernel calls the same two functions with `Integer::one()` for both occurrence counts and the pair count, so the amounts, units and order are equal by construction.

## Verdict

The behavior and AC text is accurate and AC-5 is one observable obligation (differential equality with `planned_equality` over `Value::Boolean`, at unlimited limits and over a limit sweep). Like AC-1 to AC-4 it is written in the repo's declarative AC style rather than an EARS `shall` sentence; that matches its siblings, and the EARS linter does not flag it. The TC-944 step and the test agree with it.

One medium finding: the FR's requirement statement, title and Dependencies were not widened, so AC-5 and Behavior 5 require an entry the SHALL sentence does not name. Two low findings on wording. Not blocking for the code, but FND-001 should be fixed in this PR since it is a two-line edit in the same file.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-397's requirement statement still reads "the exact kernel SHALL expose a native entry for division, modulus, an i64-wide multiply and Integer equality, and a charge-only entry" and names only "QSL FR-562 (AC-2, AC-3, AC-4)" as the consumer; the title says "Native integer ... equality" and Dependencies lists only FR-384 and QSL FR-562. Behavior 5 and FR-397-AC-5 add a native Boolean equality entry whose consumer is the CG clause harness (IR-1022, CG #800), so the AC exceeds the statement it verifies. Add the Boolean equality entry to the SHALL sentence (and the title if the title is meant to enumerate entries), and name the CG consumer in Description or Dependencies. | spec/functional/FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md:3, :18, :55-57 |
| FND-002 | low | Behavior 5 ends "This is the kernel the Kani-tractable Boolean equality of a generated-code consumer calls", a claim about another repository's generator that nothing in quire-exact can make true or test, and says the charges go "through the function that `evaluate_native_integer_equality` charges through", an internal call-structure claim no public test can observe. Move the consumer sentence to Description or Dependencies as rationale, and state the observable property (equal charge points, amounts and order) instead of the shared-function clause, or mark that clause as a design note. | spec/functional/FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md:38 |
| FND-003 | low | FR-397-AC-5 says "at every limit from zero to the unlimited spend of each counted limit kind", and TC-944 step 4 says "each counter's spend". "Counted limit kind" is not defined in FR-397, and the test sweeps four of the ten kinds (`integer_bits`, `value_occurrences`, `work_units`, `result_units`) and skips the other six, although `integer_bits` has the same zero spend as the skipped six. Two readers could pick different kind sets. Name the kinds (the four limits the equality schedule can stop on, or all ten) in the AC and the TC step. | spec/functional/FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md:48, spec/test-cases/TC-944-native-integer-division-wide-multiply-equality-and-charge-schedule.md:21 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-397 statement | examined | When a generated-code consumer evaluates an operation on native operands (FR-384) that its checked bounds prove fits a machine word, the exact kernel SHALL expose a native entry for division, modulus, an i64-wide multiply and Integer equality, and a charge-only entry, so that the consumer can skip the arbitrary-precision call and still produce the value, the refusal and the Meter charge sequence of the existing `Integer` path. |
| FR-397 Outputs (new line) | examined | `evaluate_native_boolean_equality`, returning `Outcome` of `bool`. |
| FR-397 Behavior 5 | examined | `evaluate_native_boolean_equality` over two native `bool` operands returns whether they are equal and charges the `equality.plan-form`, `equality.plan`, `equality.pair` and `equality.result-retain` points `planned_equality` charges for two `Boolean` values ... and not the `boolean.result-retain` point of `retain_boolean`. |
| FR-397-AC-5 | examined | `evaluate_native_boolean_equality` over the four `(left, right)` pairs returns `planned_equality`'s value over `Value::Boolean` operands with equal outcome, charge events and counters, at unlimited limits and at every limit from zero to the unlimited spend of each counted limit kind. |
| TC-944 Description | examined | Verify FR-397-AC-1 through FR-397-AC-5 through the public API ... |
| TC-944 step 4 | examined | Compare `evaluate_native_boolean_equality` with `planned_equality` over `Value::Boolean` for the four pairs, at unlimited limits and at every limit from zero to each counter's spend. |
| TC-944 step 5 | examined | Run `charge_native_integer_schedule` and the matching evaluator on separate meters ... (renumbered from 4, text unchanged) |
| FR-397 Behavior 3, Behavior 4, AC-3, AC-4 | context_only | equality and charge-only entry text the new behavior is modelled on |
