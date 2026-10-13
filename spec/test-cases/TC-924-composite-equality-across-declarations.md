---
id: TC-924
title: "Check equality of composite values across declarations"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-376
    type: verifies
---
# TC-924: Check equality of composite values across declarations

## Description

Verify [FR-376](../functional/FR-376-composite-equality-across-declarations.md) through the public kernel equality paths: `plan_equality`, `planned_equality`, `planned_equality_with_plan` and `member_equal`. Build values with `from_admitted_slots`, `OptionValue::from_admitted` and `collection::from_admitted`, so no declaration registry is involved.

Fixtures. Declaration keys are `NodeKey::from_digest` of a repeated byte: `P = [1; 32]`, `C = [2; 32]`, `G = [3; 32]`. Integers are `Integer::from`.

| Name | Declaration | Slots |
| --- | --- | --- |
| `p100` | `P` | `[Present(Integer 100)]` |
| `p200` | `P` | `[Present(Integer 200)]` |
| `c100` | `C` | `[Present(Integer 100)]` (the same slots as `p100`) |
| `c5_100` | `C` | `[Present(Integer 5), Present(Integer 100)]` |
| `g100` | `G` | `[Present(Integer 100)]` |
| `p_empty`, `c_empty` | `P`, `C` | `[]` |
| `p_ref`, `c_ref` | `P`, `C` | `[Present(Reference in universe U1)]`, `[Present(Reference in universe U2)]` |
| `p_ref2` | `P` | `[Present(Reference in universe U2)]` |

## Test Procedure

1. For each ordered pair among `(p100, c100)`, `(p200, c100)`, `(p100, c5_100)`, `(p100, g100)`, `(c100, g100)`, `(p_empty, c_empty)`, call `plan_equality` and `planned_equality_with_plan` under a generous meter, in both operand orders. Record the Boolean, the pair count and the refusal, if any.
2. On `(p100, c100)` call `planned_equality` with a test meter that records the admitted charge log. Record the log, `WorkUnits`, `ResultUnits` and the `ValueOccurrences` high-water mark.
3. Call `planned_equality` on `(p_ref, c_ref)` and, as a control, on `(p_ref, p_ref2)`.
4. Call `planned_equality` on `(p100, p100')` where `p100'` is a second value built with the same declaration and slots, `(p100, p200)`, and `(c100, c100')` likewise.
5. Wrap the pair `(p100, c100)` as the payload of two present `Option<Composite(P)>` values, as the only slot of two one-slot records of a declaration `W = [4; 32]`, and as the only element of two one-element `Sequence`, `Set` and `Bag` values of element type `Composite(P)` (built by `from_admitted`). Call `plan_equality` and `planned_equality` on each pair of wrappers, in both orders. As controls, wrap `(p100, p100')` the same way.
6. Call `planned_equality` on `(p100, Integer 100)`, `(p100, none Option)`, `(p100, empty Sequence)` and `(p100, Reference)` in both orders. Build a composite `bad` with declaration `P` and slots `[Present(100), Present(100)]` and call it on `(p100, bad)`. Call it on two `Population` values.
7. Call `member_equal(candidate = c100, member = p100)` under a recording meter.
8. Call `form` for an `OrderedSet` of element type `Composite(P)` over `[c100, p100]`, `[p100, c100]` and `[c100, p100, c100]`, under a recording meter.
9. Hold each of `p100`, `c100`, `g100` at the one position of a union member of a declaration `U = [9; 32]` through `UnionValue::from_admitted`, and call `plan_equality` and `planned_equality` on the pairs of step 1, on `(p_ref, c_ref)` held the same way, and on a two-position member holding `(p100, 1)` against `(c100, 2)`. Wrap one held pair as the payload of two present `Option` values. Call `planned_equality` on a union against `Integer 1`, on two unions of one member with one and two payload values, and on two unions of different declarations.

## Expected Results

1. Every pair of step 1 gives `Completed(false)` with a pair count of 1 and no refusal, in both operand orders. No pair gives a refusal of any kind.
2. The log is `equality.plan-form`, `equality.plan`, one `equality.pair`, `equality.result-retain`, in that order. `plan-form` charges `ValueOccurrences` as the larger operand occurrence count and `WorkUnits` as their sum, as FR-375 states. `WorkUnits` is the plan-form sum plus the plan, pair and retain work units that FR-375 assigns, with one pair. `ResultUnits` is 1.
3. `(p_ref, c_ref)` gives `Completed(false)` and no `ForeignReference`. The control `(p_ref, p_ref2)` refuses `ForeignReference { required: U1, supplied: U2 }`.
4. `(p100, p100')` gives `true`; `(p100, p200)` gives `false`; `(c100, c100')` gives `true`. Pair counts are those of the slot walk: 2 for each one-slot pair (the composite pair and its slot pair).
5. Each wrapped pair of different declarations gives `false` with a pair count of 2 (the wrapper pair and the inner pair), in both orders, with no refusal. Each control wrapper gives `true` with a pair count of 3 for the `Option`, the one-slot record and the one-element collections (the wrapper pair, the composite pair and its slot pair).
6. Each pair of step 6 refuses `CheckedInvariant { cause: ValueKindMismatch }`. `(p100, bad)` refuses `ValueKindMismatch` (same declaration, different slot counts). Two populations refuse `PopulationPair`. No call returns a completed Boolean.
7. `member_equal` gives `Completed(false)`. The log is `collection.member-walk` then `collection.member-test`, with `ValueOccurrences` and `WorkUnits` of the test charge equal to the pair count of 1.
8. The `OrderedSet` completes with members `[c100, p100]`, `[p100, c100]` and `[c100, p100]` respectively, with no refusal. The first two charge one membership comparison; the third charges two in total (the second occurrence against the first, then the third against the first, where it merges).
9. Every held pair of different declarations gives `false` with a pair count of 2 in both orders; the reference pair gives `false` with no `ForeignReference`; the two-position pair gives `false` with 3 pairs; the `Option` wrapper gives `false` with 3 pairs; the equal-declaration control gives `true` with the pair count of the bare composite plus 1. The union against `Integer 1`, the unions of different payload lengths and the unions of different declarations refuse `ValueKindMismatch`.
