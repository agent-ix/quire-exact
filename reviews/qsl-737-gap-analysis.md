---
id: SR-8687
title: "QSL-737 quire-exact PR 39 gap analysis: FR-376, FR-377, FR-378 to tests"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@17abd1ac1560ed855502bb0f6883b415428593bf; FR-376-AC-1..8; FR-377-AC-1..7; FR-378-AC-1..7; TC-924; TC-925; TC-926; tests/composite_declarations.rs; src/equality.rs; src/key.rs; src/collection.rs; src/value.rs"
review_set: subset
---

## Summary

Plan completion: not assessed

Ticket: QSL-737. `quoin matrix --json` (quoin 0.28.3) at the reviewed sha
shows all 21 Test criteria of FR-376, FR-377 and FR-378 `tagged`, each bound
to one test in tests/composite_declarations.rs. FR-378-AC-7 is an Inspection
criterion with no symbol, as expected; inspected by hand: the five rustdoc
items of FR-378 Behavior 8 carry the stated text. Evidence status is "no run
evidence" for every row, since no test run was made (reviewer-only brief).

Test-oracle strength, per criterion. "Reverts" means the test fails if the
src change is reverted.

- FR-376-AC-1: reverts (refusal). Asserts pair count 1 and `false` through
  `plan_equality`, `planned_equality_with_plan` and `planned_equality`, both
  orders. A slot-descending variant fails the count. See FND-001 for the
  missing case.
- FR-376-AC-2: reverts. Gated charge log, plus ungated admission count 4,
  `ResultUnits` 1, `ValueOccurrences` 2, `WorkUnits` 7.
- FR-376-AC-3: reverts, and a slot-first variant fails it (it would report
  `ForeignReference`). Same-declaration control asserts the exact
  `ForeignReference { required, supplied }`.
- FR-376-AC-4: regression guard for unchanged behaviour; pair counts 2.
- FR-376-AC-5: reverts. Option, one-slot record, Sequence, Set and Bag, both
  orders, pair count 2; same-declaration controls give 3.
- FR-376-AC-6: regression guard. Typed `ValueKindMismatch` for Integer,
  Option, Collection and Reference in both orders and for different slot
  counts; typed `PopulationPair`. Catches an "all composites unequal"
  variant.
- FR-376-AC-7: reverts. Gated log and test-charge `value_occurrences == 1`;
  ungated admission count 2 and work 5.
- FR-376-AC-8: reverts. Ungated supply order for all three sequences;
  membership-comparison counts 1, 1 and 2 under test-support.
- FR-377-AC-1: reverts (None). Includes slots that order the other way and
  different slot counts, so a slots-first variant fails.
- FR-377-AC-2: reverts. Last-byte and first-byte cases; a reversed or
  little-endian byte comparison fails.
- FR-377-AC-3, AC-5: reverts; AC-5 checks key-equal iff equality-true over
  all 16 ordered pairs and None/ValueKindMismatch on the malformed pair.
- FR-377-AC-4: reverts. Set and Bag, form and form_grouped, both supply
  orders, and the four-member Bag.
- FR-377-AC-6: regression guard. Typed `CanonicalKeyUnavailable` through
  `form_grouped` and `ValueKindMismatch` through `form`. The Float pair is
  two `Composite(P)` members with Float slots, which fits a `Set` of
  `Composite(P)`.
- FR-377-AC-7: reverts. Option, record, Sequence, Set and Bag.
- FR-378-AC-1..6: AC-1, AC-2 and AC-6 revert (Set/Bag formation refused
  before); AC-3, AC-4 and AC-5 guard paths the PR does not change. AC-5
  asserts all five exact-path refusals as typed values and that a `P`
  member is accepted by each. AC-6's "drops on the iterative paths" is
  shown only by an explicit `drop`, which cannot fail; the iterative drop
  itself is pre-existing.

Every assertion is on typed values, causes, orderings or counts. None
matches prose.

## Verdict

PASS WITH FINDINGS (one low). Every criterion is bound and discriminating.
The one gap is a single enumerated case of FR-376-AC-1 with no fixture.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-376-AC-1 names four operand shapes: equal slots, different slots, different slot counts and empty slots. The test covers equal slots (`p100`/`c100`, `p100`/`g100`, `c100`/`g100`), different counts (`p100`/`c5_100`) and empty. It has no pair with one slot count and different slot contents, such as `p200`/`c100`. TC-924 step 1 omits that case too. The pair count of 1 still catches slot descent, so the risk is low. | tests/composite_declarations.rs:414-429 |
