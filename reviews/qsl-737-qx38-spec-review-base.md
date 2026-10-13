---
id: SR-7882
title: "Base spec review of quire-exact PR #38 (FR-376..378, TC-924..926)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact@e9c9ed4a05c3c58505200785a77929f737402029; spec/functional/FR-369, FR-375, FR-376, FR-377, FR-378; spec/test-cases/TC-917, TC-923, TC-924, TC-925, TC-926"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-376
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-377
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-378
    type: reviews
---

## Summary

Ticket: QSL-737. PR: quire-exact#38. Base checklist over the PR #38 spec diff: three new FRs, three new TCs and the FR-369/FR-375/TC-917/TC-923 amendments, checked against QSL FR-359 (cross-repo requirements 2 and 3, items 4, 5, 8) and the current kernel code at the merge base a58033d. IDs are unique across all remote branches and open PRs; quire validate is clean; quire lint warns on every Verification cell. No public-repo leak was found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-378 Behavior 1 adds admitting variants of construct_collection and form_collection, but the kernel already has the caller-admitted collection path: `form` (src/collection.rs:320-335) is documented as "the caller must already have admitted every occurrence ... nothing here re-checks it", and the QSL evaluator already charges collection.element itself and calls `form` (qsl-eval evaluate.rs:262, 1576). FR-378 treats Option this way (Behavior 6: from_admitted is the sanctioned path, no new constructor) but not collections, and names no caller that needs the new API. Either name `form` (after caller admission and a caller-charged collection.element loop) as the sanctioned path, or state which caller needs the closure-taking constructors and why `form` is not enough. | FR-378:37; src/collection.rs:320-335 |
| FND-002 | low | Question (b): no kernel gap for record fields and tuple positions. evaluate_record/evaluate_tuple stay exact (DeferredResultNotAdmitted via value.rs:1090-1099), and the caller-admitted path already exists as from_admitted_slots plus retain_composite (value.rs:1105, documented for a caller-built composite). FR-378 Behavior 7 names only from_admitted_slots; it should also name retain_composite so a caller-built record or tuple keeps the composite.result-retain charge of evaluate_record. The kernel has no union value on main, so the FR-359 union-payload slot has no kernel constructor to cover yet. | FR-378:43; src/value.rs:1090-1105,1246 |
| FND-003 | low | All 20 Test rows in FR-376/377/378 put a TC id in the Verification cell ("Test (TC-924)"). The checklist requires the method only, and quire lint (acceptance-criterion-verification-method) warns on every one of these rows. The TC link already lives in each TC's verifies relationship. | FR-376:44-50; FR-377:43-49; FR-378:50-55 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-376 | examined | When the kernel equality plan pairs two composite values whose declaration keys differ, the `quire-exact` kernel SHALL decide that pair as not equal, with no checked-invariant refusal. |
| FR-377 | examined | When `compare_keys` orders two composite values whose declaration keys differ, the `quire-exact` kernel SHALL order them by the ascending byte order of their 32-byte declaration keys, compared before any slot, so that set, bag and ordered-set formation over such values completes instead of refusing `CanonicalKeyUnavailable`. |
| FR-378 | examined | When a caller holds knowledge the kernel does not (which declaration conforms to which, so that a descendant-declared composite belongs at an ancestor-typed element or payload slot), the `quire-exact` kernel SHALL let the caller supply that admission to collection construction. The kernel accepts an already-admitted `Option` payload through `OptionValue::from_admitted`. The kernel keeps its exact, environment-less constructors and `ValueType::admits` unchanged. QSL FR-359 (items 5 and 8 and cross-repo requirement 3) states the language-level need. |
| FR-376-AC-1 | examined | `plan_equality`, `planned_equality` and `planned_equality_with_plan` on two composites whose declaration keys differ return `false`, with a pair count of 1, and no `Refusal`. The result is the same in either operand order, for operands with equal slots, with different slots and with different slot counts, and for a pair of empty-slot composites. |
| FR-376-AC-2 | examined | On that pair, `planned_equality` admits `equality.plan-form`, `equality.plan`, exactly one `equality.pair` and `equality.result-retain`, in that order, with the plan-form charge computed from the two operands' occurrence counts as FR-375 states. |
| FR-376-AC-3 | examined | A composite pair of different declarations whose slots hold `Reference` values of different universes returns `false` and does not report `foreign_reference`, because the slots are not visited. A same-declaration pair with the same slots still reports `foreign_reference`. |
| FR-376-AC-4 | examined | Two composites with equal declaration keys and equal slots give `true`. Equal declaration keys with a differing slot give `false`. The pair counts are those of the same plan before this requirement. |
| FR-376-AC-5 | examined | A pair of different declarations that sits as the payload of two present `Option`s, in a one-slot record, as the element of two one-element `Sequence`, `Set` and `Bag`, gives `false`, with pair counts of 2, 2 and 2, and no refusal. |
| FR-376-AC-6 | examined | A composite against an `Integer`, `Option`, `Collection` or `Reference`, in either order, still refuses `ValueKindMismatch`. Two composites of one declaration key and different slot counts still refuse `ValueKindMismatch`. Two populations still refuse `PopulationPair`. Neither refusal returns a completed Boolean. |
| FR-376-AC-7 | examined | `member_equal` of a composite candidate against a member of a different declaration returns `false`, charging `collection.member-walk` then `collection.member-test` with one pair. |
| FR-377-AC-1 | examined | `compare_keys(a, b)` for composites of declaration keys `[1; 32]` and `[2; 32]` is `Some(Less)` and `compare_keys(b, a)` is `Some(Greater)`, whatever their slots: slots equal, slots that would order the other way, and different slot counts. |
| FR-377-AC-2 | examined | Declaration keys that differ only in the last byte (`[1; 31]` followed by `0`, against the same followed by `1`) order by that byte; keys that differ in the first byte order by the first byte even when the later bytes order the other way. |
| FR-377-AC-3 | examined | Two composites of one declaration key keep the slot order of the rule before this requirement; `compare_keys(x, x)` is `Some(Equal)`; two composites of different declarations are never `Some(Equal)`, even with identical slots. |
| FR-377-AC-4 | examined | A `Set`, a `Bag` and an `OrderedSet` formed from one composite of each of two declarations, in each of the two supply orders, complete without `CanonicalKeyUnavailable` and visit the member of the smaller declaration key first. A `Bag` holding two equal members of each declaration visits the two groups in that order. |
| FR-377-AC-5 | examined | Equal keys occur exactly when values are equal: for a set of composites of two declarations with equal and unequal slots, every pair has `compare_keys == Some(Equal)` exactly when `planned_equality` gives `true`. |
| FR-377-AC-6 | examined | A composite against an `Integer`, two composites of one declaration key and different slot counts, and a pair of `Float` members each return `None` from `compare_keys` and, as set members, refuse `CanonicalKeyUnavailable`. |
| FR-377-AC-7 | examined | The key order of a pair of different declarations holds as the payload of two `Option`s, in a record slot and as an element of two `Sequence` values, ordering the enclosing values by the same declaration-key rule. |
| FR-378-AC-1 | examined | With an admission that accepts a composite of declaration `[2; 32]` at element type `Composite([1; 32])`, the admitting forms of `Sequence`, `Set`, `Bag` and `OrderedSet` over one `[1; 32]` member and one `[2; 32]` member complete, keep both members unchanged (declaration key and slots intact), and for the unique kinds hold two elements even with equal shared slots. |
| FR-378-AC-2 | examined | The exact `construct_collection` and `form_collection` over the same members still refuse: `CollectionElementNotAdmitted` from the deferred path and `Element(index)` `TypeMismatch` from the occurrence path. `ValueType::admits` of `Composite([1; 32])` on a `[2; 32]` composite is still `false`. |
| FR-378-AC-3 | examined | An admission that rejects the third of four elements refuses with the exact constructor's refusal for that path, admits calls for elements one to three only, and on the occurrence path leaves the meter unchanged. |
| FR-378-AC-4 | examined | The charge log of an admitting form is identical to the exact form's for members that both accept, in kind, order and totals, including the membership comparisons of a `Set` and a `Bag`, `collection.bound` and `collection.result-retain`. Cardinality bounds are enforced as before. |
| FR-378-AC-5 | examined | `OptionValue::from_admitted(Composite([1; 32]), Some(composite [2; 32]))` returns an `Option` whose `payload_type()` is `Composite([1; 32])` and whose payload has declaration `[2; 32]` and its slots unchanged. `OptionValue::present` with the same arguments refuses `TypeMismatch` at `Component::Payload`. A `[1; 32]` payload is accepted by both. |
| FR-378-AC-6 | examined | A collection formed by the admitting path holds its members after a clone, an equality plan against an identical collection (`true`) and against one that differs in a member's declaration (`false`, FR-376), and a drop, on the same iterative paths as any `Value`. A collection of element type `Composite([2; 32])` is not admitted by `ValueType::admits` at `Collection` of element type `Composite([1; 32])`. |
| FR-378-AC-7 | examined | The rustdoc of `OptionValue::from_admitted` and `ValueType::admits` carries the statements of Behavior item 8. |
| FR-369-AC-2 | examined | Every kernel producer in collection, value, division and equality returns exactly its table cause; two population values return `PopulationPair`, a population paired with another kind returns `ValueKindMismatch`, and unlike collection kinds return `CollectionKindMismatch`. Two composites of different declarations return `false` and a composite paired with a non-composite returns `ValueKindMismatch` (FR-376-AC-1, FR-376-AC-6). |
| FR-375-AC-4 | examined | With equal-shaped operands whose declared (static) composite types differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted. |
| TC-924 | examined | Verify [FR-376](../functional/FR-376-composite-equality-across-declarations.md) through the public kernel equality paths: `plan_equality`, `planned_equality`, `planned_equality_with_plan` and `member_equal`. Build values with `from_admitted_slots`, `OptionValue::from_admitted` and `collection::from_admitted`, so no declaration registry is involved. Fixtures. Declaration keys are `NodeKey::from_digest` of a repeated byte: `P = [1; 32]`, `C = [2; 32]`, `G = [3; 32]`. Integers are `Integer::from`. \| Name \| Declaration \| Slots \| \| --- \| --- \| --- \| \| `p100` \| `P` \| `[Present(Integer 100)]` \| \| |
| TC-925 | examined | Verify [FR-377](../functional/FR-377-composite-canonical-key-across-declarations.md) through `compare_keys` and the collection forms `form`, `form_grouped` and `form_collection`, with members built by `from_admitted_slots`. Reuse the fixtures of [TC-924](./TC-924-composite-equality-across-declarations.md) (`P = [1; 32]`, `C = [2; 32]`, `G = [3; 32]`, `p100`, `p200`, `c100`, `c5_100`, `g100`) and add: \| Name \| Declaration \| Slots \| \| --- \| --- \| --- \| \| `k0`, `k1` \| `[1; 31]` followed by byte `0`, then `1` \| `[Present(Integer 9)]` \| \| `hi` \| `[9; 32]` \| `[Present(Integer 0)]` \| \| `lo` \| `[2; |
| TC-926 | examined | Verify [FR-378](../functional/FR-378-caller-admitted-collection-elements-and-option-payloads.md) through the exact and admitting collection constructors and the two `Option` constructors. Reuse the fixtures of [TC-924](./TC-924-composite-equality-across-declarations.md) (`p100`, `c100`, `g100`, `P = [1; 32]`, `C = [2; 32]`). The test admission accepts a composite when its declaration key is `P`, `C` or `G` at the element type `Composite(P)`, and otherwise defers to `ValueType::admits`. It records each call. The collection types have element type `Composite(P)`, no cardinality bound unless st |

## Verdict

FINDINGS (1 medium, 2 low). Requirements 2 and 3 of QSL FR-359 are implemented in substance. FR-376 makes different declarations unequal, gives false, does not descend into slots, and keeps composite-vs-non-composite as ValueKindMismatch (FR-359 item 4). FR-377 orders by ascending declaration-key bytes before slots (FR-359 item 5). FR-378 keeps ValueKindMismatch-free admission in the caller and ValueType::admits exact (FR-359 item 8 and req 3), with OptionValue::from_admitted as the sanctioned Option path. Every claim about current code checks out: the equality.rs catch-all returns ValueKindMismatch for two composites of different declarations (218-219 guard, 246-268 arm); compare_keys returns None (key.rs:136-169); sort_by_key refuses CanonicalKeyUnavailable (collection.rs:488-503); OptionValue::from_admitted is at value.rs:844; collection::from_admitted is at collection.rs:253; and evaluate_record/evaluate_tuple refuse DeferredResultNotAdmitted (value.rs:1090-1099). FR-376..378 and TC-924..926 are unique across all remote branches and the open PRs (#11, #17). Leak check: the new text cites only the public QSL FR-359 and the public ticket id QSL-737 (PR body); no private repos, branch names, paths or ticket ids appear. quire validate is clean.

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-exact@5e67559b77c4d57501ad859d70c26f6c5e9e86ed.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | The fix round renamed FR-378 to FR-378-caller-admitted-values-in-collections-options-and-composites.md, but FR-376 Dependencies (line 60) and FR-377 Dependencies (line 61) still link the old file name FR-378-caller-admitted-collection-elements-and-option-payloads.md, which no longer exists. FR-377:61 also still says "the constructor that admits such members is FR-378", but FR-378 now adds no constructor. | FR-376:60; FR-377:61 |
| FND-005 | low | FR-378 Behavior 1 says form and form_grouped "charge membership comparison, collection.bound and collection.result-retain". form_grouped never coalesces, so it charges no membership comparison (src/collection.rs:364-379). form over a Sequence compares nothing either (collection.rs:344-346). Say that form charges membership comparison for Set, Bag and OrderedSet, and form_grouped charges only collection.bound and collection.result-retain. | FR-378:37; src/collection.rs:344-346,364-379 |

## Dispositions

Round 1, reviewed at agent-ix/quire-exact@5e67559b77c4d57501ad859d70c26f6c5e9e86ed.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5e67559 |
| FND-002 | fixed | 5e67559 |
| FND-003 | fixed | 5e67559 |
