---
id: TC-925
title: "Check the canonical key across composite declarations"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-377
    type: verifies
---
# TC-925: Check the canonical key across composite declarations

## Description

Verify [FR-377](../functional/FR-377-composite-canonical-key-across-declarations.md) through `compare_keys` and the collection forms `form`, `form_grouped` and `form_collection`, with members built by `from_admitted_slots`. Reuse the fixtures of [TC-924](./TC-924-composite-equality-across-declarations.md) (`P = [1; 32]`, `C = [2; 32]`, `G = [3; 32]`, `p100`, `p200`, `c100`, `c5_100`, `g100`) and add:

| Name | Declaration | Slots |
| --- | --- | --- |
| `k0`, `k1` | `[1; 31]` followed by byte `0`, then `1` | `[Present(Integer 9)]` |
| `hi` | `[9; 32]` | `[Present(Integer 0)]` |
| `lo` | `[2; 32]` | `[Present(Integer 99)]` |
| `f1`, `f2` | `P` | `[Present(Float)]` each |

## Test Procedure

1. Call `compare_keys` on `(p100, c100)`, `(p200, c100)`, `(p100, c5_100)`, `(p100, g100)`, `(c100, g100)` and each reversed.
2. Call `compare_keys` on `(k0, k1)`, `(hi, lo)` and each reversed.
3. Call `compare_keys` on `(p100, p100')`, `(p100, p200)`, `(p100, p100)` and `(c100, c100)`.
4. Form a `Set`, an `OrderedSet` and a `Bag` of element type `Composite(P)` from the occurrences `[c100, p100]` and again from `[p100, c100]` through `form`. Form a `Bag` from `[c100, p100, c100, p100]`. Record the outcome and the visiting order.
5. For the occurrences `[c100, p100, p200, c5_100]`, check for every pair `(x, y)` that `compare_keys(x, y) == Some(Equal)` exactly when `planned_equality(x, y)` gives `true`.
6. Call `compare_keys` on `(p100, Integer 100)`, `(p100, bad)` where `bad` has declaration `P` and two slots, and `(f1, f2)`. Form a `Set` from `[f1, f2]` through `form_grouped`.
7. Wrap `(p100, c100)` as the payload of two present `Option` values, as the only slot of two records of declaration `[4; 32]`, and as the only element of two one-element `Sequence` values. Call `compare_keys` on each wrapper pair, and each reversed.

## Expected Results

1. `(p100, c100)`, `(p200, c100)`, `(p100, c5_100)`, `(p100, g100)` and `(c100, g100)` give `Some(Less)`; each reversed pair gives `Some(Greater)`. The slots (`p200` against `c100`, different slot counts) never decide.
2. `(k0, k1)` gives `Some(Less)` and `(hi, lo)` gives `Some(Greater)`: the first differing byte of the declaration key decides, whatever the slots.
3. `(p100, p100')` gives `Some(Equal)`, `(p100, p200)` gives `Some(Less)`, and the two self-comparisons give `Some(Equal)`. No pair of different declarations gives `Some(Equal)`.
4. Every form completes with no `CanonicalKeyUnavailable`. Both supply orders yield `[p100, c100]` for the `Set` and `OrderedSet`; the `Bag` visits `[p100, p100, c100, c100]`.
5. The two relations agree for all sixteen ordered pairs.
6. The first two comparisons give `None`; `(f1, f2)` gives `None`. The `Set` of two `Float` members refuses `CheckedInvariant { cause: CanonicalKeyUnavailable }`. No call returns a false `Some(Equal)`.
7. Each wrapper pair is `Some(Less)` in the order `(P-wrapper, C-wrapper)` and `Some(Greater)` reversed.
