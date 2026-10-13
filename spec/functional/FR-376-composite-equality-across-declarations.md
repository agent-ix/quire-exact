---
id: FR-376
title: "Composite values of different declarations are unequal, not a checked-invariant fault"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-369
    type: references
  - target: ix://agent-ix/quire-exact/FR-375
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-359
    type: references
---
# FR-376: Composite values of different declarations are unequal, not a checked-invariant fault

## Description

When the kernel equality plan pairs two composite values whose declaration keys differ, the `quire-exact` kernel SHALL decide that pair as not equal, with no checked-invariant refusal.

A caller may hold a value whose most-specific declaration is a descendant of the declaration a slot names. Two operands of one static type then carry different declarations, and that case is ordinary, not a malformed program. The kernel carries no declaration graph. It decides the pair from the two declaration keys alone. QSL FR-359 (item 4) states the language-level rule and names this requirement as the kernel's part.

## Inputs

- Two admitted values reaching `plan_equality`, `planned_equality`, `planned_equality_with_plan` or the membership comparison `member_equal`, directly or as the payload, field, element or tuple position of another value.

## Outputs

- The Boolean relation, the exact pair count and the charge schedule of FR-375.

## Behavior

1. **Different declarations are unequal.** For a pair `(Composite, Composite)` whose declaration keys are not byte-equal, the pair's result is `false`. The pair contributes exactly one occurrence-path pair, and the plan does not descend into the slots of either value. The relation of the whole plan is `false` when any pair is `false`, as before. The traversal and the charge schedule do not short-circuit (FR-375-AC-2): the remaining pairs are still planned and charged.
2. **The decision is by declaration key, not by shape.** Two composites of different declarations are unequal whatever their slot count and slot contents, including identical slots. The declaration check precedes any slot, text-leaf or reference comparison, so a slot of one declaration is never compared with a slot of another, and a reference pair inside the skipped slots cannot report `foreign_reference`.
3. **Same declaration.** Two composites with the same declaration key and the same slot count are equal exactly when every slot pair is equal, as today.
4. **Faults that remain.** Two composites with the same declaration key and different slot counts are malformed and refuse `CheckedInvariant { cause: ValueKindMismatch }`. A composite paired with a non-composite value, and every other pairing of unlike value variants, refuses `ValueKindMismatch` (FR-369, FR-370-AC-3). Two population values refuse `PopulationPair` (FR-089). Two collections of different kinds refuse `CollectionKindMismatch`.
5. **Inside another value.** The rule applies at every depth: an `Option` payload, a record slot, a collection element and a tuple position that pair two composites of different declarations give `false` for that pair and leave the enclosing pair counts to the rules above. The `Option` or collection shape itself is compared first, so a `none` against a present payload and two collections of different lengths are decided without reaching the payloads.
6. **Negation.** The caller's `!=` is the Boolean negation of this relation. The kernel exposes no separate inequality.
7. **No ancestry in the kernel.** The kernel treats no declaration as the ancestor of another and never compares a descendant with an ancestor by their shared slots. Whether a value conforms to a slot's declared type is the caller's check (FR-378).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-376-AC-1 | `plan_equality`, `planned_equality` and `planned_equality_with_plan` on two composites whose declaration keys differ return `false`, with a pair count of 1, and no `Refusal`. The result is the same in either operand order, for operands with equal slots, with different slots and with different slot counts, and for a pair of empty-slot composites. | Test (TC-924) |
| FR-376-AC-2 | On that pair, `planned_equality` admits `equality.plan-form`, `equality.plan`, exactly one `equality.pair` and `equality.result-retain`, in that order, with the plan-form charge computed from the two operands' occurrence counts as FR-375 states. | Test (TC-924) |
| FR-376-AC-3 | A composite pair of different declarations whose slots hold `Reference` values of different universes returns `false` and does not report `foreign_reference`, because the slots are not visited. A same-declaration pair with the same slots still reports `foreign_reference`. | Test (TC-924) |
| FR-376-AC-4 | Two composites with equal declaration keys and equal slots give `true`. Equal declaration keys with a differing slot give `false`. The pair counts are those of the same plan before this requirement. | Test (TC-924) |
| FR-376-AC-5 | A pair of different declarations that sits as the payload of two present `Option`s, in a one-slot record, as the element of two one-element `Sequence`, `Set` and `Bag`, gives `false`, with pair counts of 2, 2 and 2, and no refusal. | Test (TC-924) |
| FR-376-AC-6 | A composite against an `Integer`, `Option`, `Collection` or `Reference`, in either order, still refuses `ValueKindMismatch`. Two composites of one declaration key and different slot counts still refuse `ValueKindMismatch`. Two populations still refuse `PopulationPair`. Neither refusal returns a completed Boolean. | Test (TC-924) |
| FR-376-AC-7 | `member_equal` of a composite candidate against a member of a different declaration returns `false`, charging `collection.member-walk` then `collection.member-test` with one pair. | Test (TC-924) |

## Status

Specified. Code lands in a later slice.

## Dependencies

- Text that changes with this requirement: the `ValueKindMismatch` row and the sentence on the equality catch-all in [FR-369](./FR-369-typed-checked-invariant-causes.md), FR-369-AC-2, step 4 of [TC-917](../test-cases/TC-917-typed-checked-invariant-causes.md), the module and `plan_pairs` doc comments of `src/equality.rs` ("operands that are not of one type, which a checked program never produces"), and [FR-375](./FR-375-composable-equality-effect-seam.md) AC-4 with step 5 of [TC-923](../test-cases/TC-923-composable-equality-effect-seam.md), which concern operands of different static types and keep their text with that clarification.
- The key order for the same pairs is [FR-377](./FR-377-composite-canonical-key-across-declarations.md); the admission of such values is [FR-378](./FR-378-caller-admitted-collection-elements-and-option-payloads.md).
- The falsifying controls are [TC-924](../test-cases/TC-924-composite-equality-across-declarations.md).
- The language-level statement is QSL FR-359 (item 4 and cross-repo requirement 2).
