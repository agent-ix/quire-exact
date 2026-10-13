---
id: SR-7884
title: "Failure-domain review of quire-exact PR #38 (FR-376..378, TC-924..926)"
type: SpecReview
analysis: failure-domain
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

Ticket: QSL-737. PR: quire-exact#38. Refusal paths, malformed-pair edges and nested placement in FR-376..378 and their TCs, traced against plan_pairs, compare_keys, coalesce and sort_by_key. Which refusal comes first depends on the formation path, a TC fixture contains a malformed pair it does not account for, and one AC contradicts itself.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-377-AC-6 says a composite against an Integer, two same-declaration composites of different slot counts, and two Float members "as set members, refuse CanonicalKeyUnavailable", but names no formation path. Through `form` (and form_collection/construct_collection), coalesce runs member_equal first (src/collection.rs:431-437), and plan_pairs refuses each of these pairs with ValueKindMismatch before any sort. Only form_grouped reaches sort_by_key and CanonicalKeyUnavailable. TC-925 step 6 forms only the Float pair, through form_grouped. State that the path is form_grouped (or give the ValueKindMismatch cause for `form`), and cover the other two pairs. | FR-377:48; TC-925:30,40; src/collection.rs:431-437,488-503 |
| FND-002 | medium | TC-925 step 5 includes c100 and c5_100, which share declaration C and have different slot counts. Under FR-376 Behavior 4 that pair is malformed: planned_equality refuses ValueKindMismatch and compare_keys returns None, in both orders. Expected 5 ("the two relations agree for all sixteen ordered pairs") gives no expected outcome for those two pairs, and a test that unwraps a completed Boolean panics. FR-377-AC-5 says "a set of composites of two declarations with equal and unequal slots", which does not admit a malformed pair. Replace c5_100 with a well-formed C value, or state the refusal and None for those two pairs. | TC-925:29,39 |
| FND-003 | medium | The second sentence of FR-376-AC-3, "A same-declaration pair with the same slots still reports foreign_reference", contradicts itself. Identical slots hold references of one universe, so they never refuse ForeignReference. The intended control is TC-924 step 3, (p_ref, p_ref2): same declaration, references in U1 and U2. Say "whose slots hold References of different universes". | FR-376:46; TC-924:33 |
| FND-004 | low | FR-359 item 4 applies the identity-first rule to "a union payload" as well. FR-376 Behavior 5 and FR-377-AC-7 list Option payload, record slot, collection element and tuple position, but not a union payload. Main has no kernel union value, but draft quire-exact PR #17 (QSL-500) adds a kernel union carrier with equality and key arms. Name the union payload, or say it is covered when the union carrier lands, so the two PRs do not miss each other. | FR-376:36; FR-377:49 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
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
| TC-925 | examined | For the occurrences `[c100, p100, p200, c5_100]`, check for every pair `(x, y)` that `compare_keys(x, y) == Some(Equal)` exactly when `planned_equality(x, y)` gives `true`. |
| TC-925 | examined | Call `compare_keys` on `(p100, Integer 100)`, `(p100, bad)` where `bad` has declaration `P` and two slots, and `(f1, f2)`. Form a `Set` from `[f1, f2]` through `form_grouped`. |

## Verdict

FINDINGS (3 medium, 1 low). The fault boundaries are right: same declaration with different slot counts, composite against non-composite, PopulationPair, CollectionKindMismatch, and foreign-reference suppression when slots are skipped. Option-shape and collection-length checks precede the payload. FR-378 refusal parity (deferred CollectionElementNotAdmitted, occurrence Element(i) TypeMismatch before any charge, first-rejection stop) matches the exact constructors.
