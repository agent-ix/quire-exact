---
id: SR-13010
title: "quire-exact PR 56 IR-878 owner allocation base review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact; PR #56 head 159f170d8ab7e339fe636934e26f602667fc0153 vs origin/main b83cab9; spec/functional/FR-369, FR-372, FR-375, FR-376, FR-380; spec/test-cases/TC-923; ticket IR-878"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: reviews
---

## Summary

Base spec review of the IR-878 item 4 owner allocation. For each criterion removed from quire-exact the reviewer compared the origin/main text with its new owner text in the companion PRs (RT #132 head 3009241, CG #449 head f69874b, QSV #47 head aea435e, QSL #995 head 85e6ccf).

Ticket: IR-878

## Reviewed units

- FR-375-AC-1..5,8 (examined): Removed from quire-exact; text re-homed verbatim (AC-8 renumbered) as RT FR-277-AC-1..6 in quire-contract-runtime#132.
- FR-369-AC-3,4 (examined): Removed; re-homed verbatim as RT FR-278-AC-1, AC-2 (quire-contract-runtime#132).
- FR-369-AC-7 (examined): Removed; re-homed verbatim as CG FR-042-AC-1 (quire-contract-codegen#449).
- FR-369-AC-9 (examined): Removed; re-homed verbatim as QSV FR-122-AC-1 (quire-semantic-value#47).
- FR-372-AC-5 (examined): Removed; split into QSV FR-122-AC-2 (supplied path; drops only 'evaluator meter change') and QSL FR-391-AC-3 (evaluator meter and charge sequence unchanged).
- FR-380-AC-6 (examined): Removed; split into QSV FR-122-AC-3 (supplied caller) and QSL FR-391-AC-2 (omitted-B 4096 default, evaluator charges unchanged).
- FR-375 (examined): When a checked equality caller evaluates two admitted operands, the `quire-exact` kernel SHALL expose the exact equality relation and planned `equality.pair` count from one plan, while its `Meter` records only admitted charges. The runtime's actual checked evaluation and its end-to-end controls are allocated to [RT FR-277](ix://agent-ix/quire-contract-runtime/FR-277).
- FR-375-AC-6 (examined): The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. The real evaluator is owned by quire-contract-runtime; a plan-only equality or duplicated QSL walker does not satisfy this criterion.
- FR-369 (examined): The shared cause carrier and kernel producers are owned here. The producer controls formerly FR-369-AC-3/4, AC-7 and AC-9 are allocated without weakening their text to RT FR-278, CG FR-042 and QSV FR-122, respectively. Their tests remain PLANNED at those owners.
- FR-372 (examined): The integrated supplied Decimal control formerly FR-372-AC-5 is allocated to [QSV FR-122-AC-2](ix://agent-ix/quire-semantic-value/FR-122) for the supplied path and [QSL FR-391-AC-3](ix://agent-ix/quire-spec-language/FR-391) for evaluator effects. Both remain PLANNED for owner tests.
- FR-380 (examined): The supplied-caller controls formerly FR-380-AC-6 are allocated to [QSV FR-122-AC-3](ix://agent-ix/quire-semantic-value/FR-122) and [QSL FR-391-AC-2](ix://agent-ix/quire-spec-language/FR-391); the evaluator charge control remains with QSL. Owner tests are PLANNED.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

No base defects. FR-375-AC-1..5 and AC-8 appear verbatim as RT FR-277-AC-1..6. FR-369-AC-3/4 appear verbatim as RT FR-278-AC-1/2, FR-369-AC-7 as CG FR-042-AC-1 and FR-369-AC-9 as QSV FR-122-AC-1. FR-372-AC-5 is split: QSV FR-122-AC-2 keeps every clause except 'evaluator meter change', and QSL FR-391-AC-3 carries that clause for the same fixture across membership, nonmembership, budget denial and cancellation. The QSL side also adds a `FunctionCall` projection clause, so the split is stronger than the original. FR-380-AC-6 is split: QSV FR-122-AC-3 keeps the authenticated B, the one initiating event, cancellation/locus, no-default and `NumericDomain` clauses; QSL FR-391-AC-2 keeps the 4096-bit omitted-B default and 'leaves evaluator charges unchanged'. No clause is lost or weakened. The rewritten FR-375 Description (planned count from one plan, with the Meter recording only admitted charges) matches the unchanged Behavior section. On success it is equivalent to the former 'count of charges admitted', so it does not weaken the statement. The reviewer also checked owner code paths: RT `src/exact/equality.rs:160` defines `CheckedEquality::evaluate`, and RT `src/exact/expression.rs` produces `UnknownCheckedFunction`/`ForeignCheckedExpression`/`MeterBorrowConflict`. CG `src/oracle/boolean_v1.rs` and `src/oracle/equality/mod.rs` hold the generated constructors that call `check_type`/`check_equality`. QSV `src/declaration.rs` produces `DeferredResultNotAdmitted`, `src/declaration/membership.rs` produces `NumericDomain` and `src/supplied_membership.rs` exists. QSL `qsl-eval/src/value/expression/evaluate.rs:365` charges `FunctionCall`, and `qsl-replay` resolves omitted B. Every new owner really owns the code path its criterion names. EARS: the rewritten FR-375 statement is event-driven ('When ... SHALL') and conforms. `quire validate --scope . 'spec/**/*.md'` exits 0. Its only warnings are module DuplicateArchetype warnings that are not caused by this PR.
