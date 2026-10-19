---
id: SR-13011
title: "quire-exact PR 56 IR-878 scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-exact; PR #56 head 159f170d8ab7e339fe636934e26f602667fc0153 vs origin/main b83cab9; spec/functional/FR-369, FR-372, FR-375, FR-376, FR-380; spec/test-cases/TC-923; ticket IR-878"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: reviews
---

## Summary

Responsibility allocation check: does each criterion now sit in the repo that owns the code path it names, and does quire-exact keep only kernel (Meter, equality planning, cause carrier) obligations?

Ticket: IR-878

## Reviewed units

- FR-375-AC-6 (examined): The IR-241 Kani claim remains `plan_equality(left, right).pair_events() == count_of_EqualityPair_charges_from_the_same_real_CheckedEquality::evaluate(left, right, meter)` for both equal and first-head-unequal bounded operands, with the existing exact operand bound and proof ceiling. The real evaluator is owned by quire-contract-runtime; a plan-only equality or duplicated QSL walker does not satisfy this criterion.
- FR-376 (examined): The traversal and the charge schedule do not short-circuit (RT FR-277-AC-2): the remaining pairs are still planned and charged.
- FR-375 (context_only): When a checked equality caller evaluates two admitted operands, the `quire-exact` kernel SHALL expose the exact equality relation and planned `equality.pair` count from one plan, while its `Meter` records only admitted charges. The runtime's actual checked evaluation and its end-to-end controls are allocated to [RT FR-277](ix://agent-ix/quire-contract-runtime/FR-277).
- FR-375-AC-1..5,8 (context_only): Removed from quire-exact; text re-homed verbatim (AC-8 renumbered) as RT FR-277-AC-1..6 in quire-contract-runtime#132.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-375-AC-6 stays in quire-exact but quantifies over the real `CheckedEquality::evaluate`, which the PR itself places in quire-contract-runtime. quire-exact cannot link RT, because RT depends on quire-exact, so no Kani harness in this leaf can execute the claim. It remains unbacked under `quire coverage --strict`. This conflicts with the binding ruling that no unbacked criterion stays in a leaf that cannot execute it. 'Stays in the Kani lane' names no owning repo or ticket. Needs a planner disposition: re-home it verbatim to RT, where both crates link, or name the Kani-lane owner ticket and repo. | spec/functional/FR-375-composable-equality-effect-seam.md:31 |
| FND-002 | low | FR-376 Behavior 1 now cites RT FR-277-AC-2 as the authority for the kernel `planned_equality` no-short-circuit property. This inverts ownership: a kernel behavior is grounded in a downstream consumer's criterion, and quire-exact no longer states that property in a criterion of its own (see SR-13014 FND-001). | spec/functional/FR-376-composite-equality-across-declarations.md:32 |

## Verdict

The moved criteria are correctly allocated. RT owns `CheckedEquality::evaluate` and its producer sites, CG owns the generated constructors, QSV owns the supplied caller and its producers, and QSL owns the evaluator meter and the omitted-B default. Two boundary defects remain on the quire-exact side: FR-375-AC-6 and the FR-376 citation.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | high | The new FR-380-AC-7 says a direct kernel helper call lacking B returns 'the kernel's typed unsupported numeric-domain result'. No such call or result exists. `IntegerInterval::try_contains_bounded` and `RationalDomain::try_contains_bounded` (src/numeric_membership.rs:60,77) take a required `magnitude_bits: u64`, and `NumericMembershipFailure` has only `UnsupportedSize` and `Cancelled`. The original FR-380-AC-6 attributed `NumericDomain` to the direct QSV call only, and so does FR-380 Behavior 6. The split moved a QSV result onto the kernel, which no test can satisfy without an API change. Keep only the kernel-true clause: B is a required finite parameter and no default is selected. That clause can be verified by signature or inspection. | spec/functional/FR-380-bounded-cancellable-numeric-membership.md:119 |

## Dispositions

Round 1, PR head c58ab92.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-375-AC-6 has been removed from quire-exact. RT #132 head 5312d30 has FR-277-AC-7, which is byte-identical to the 159f170 AC-6 text, and that text is a superset of origin/main AC-6. The RT matrix row maps it to TC-203 with owner titan:kani:plan. |
| FND-002 | fixed | FR-376 Behavior 1 now states the property at kernel level and no longer cites RT. |
| FND-003 | fixed | Round 2 (452e01a): Re-measured against src/numeric_membership.rs at 452e01a. Both helpers take `magnitude_bits: u64` (lines 63, 80). integer_contains checks value, lower and upper (lines 129-133), and rational_contains checks all six operand positions (lines 152-175) through check_operand. There is no Option, default constant or numeric-domain variant, so the criterion is true and checkable by Inspection. TC-928 step 6 matches. QSV #47 d6f631d FR-122-AC-3 keeps NumericDomain on the direct QSV call and points to FR-380. QSL #995 17dc4c4 FR-394-AC-2 keeps the 4096 default, and its Dependencies allocate the exact-helper and direct-QSV clauses to FR-380-AC-7 and FR-122-AC-3. Every clause of the former FR-380-AC-6 now has one owner. |
