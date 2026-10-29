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

Verify [FR-377](../functional/FR-377-composite-canonical-key-across-declarations.md) through `compare_keys` and the collection forms `form` and `form_grouped` over `Set` and `Bag`, with members built by `from_admitted_slots`. Reuse the value shapes of [TC-924](./TC-924-composite-equality-across-declarations.md) (`p100`, `p200`, `c100`, `c5_100`, `g100`), with distinct caller-admitted declaration keys `P < C < G` under `NodeKey` ordering. The fixture identities do not prescribe a digest or key length. Add:

| Name | Declaration | Slots |
| --- | --- | --- |
| `k0`, `k1`, `k2` | Distinct admitted keys `K0 < K1 < K2` under `NodeKey` ordering | `[Present(Integer 9)]`, `[Present(Integer 5)]`, `[Present(Integer 0)]`, respectively |
| `f1`, `f2` | `P` | `[Present(Float)]` each |

## Test Procedure

1. Call `compare_keys` on `(p100, c100)`, `(p200, c100)`, `(p100, c5_100)`, `(p100, g100)`, `(c100, g100)` and each reversed.
2. Call `compare_keys` on `(k0, k1)`, `(k1, k2)`, `(k0, k2)` and each reversed.
3. Call `compare_keys` on `(p100, p100')`, `(p100, p200)`, `(p100, p100)` and `(c100, c100)`.
4. Form a `Set` and a `Bag` of element type `Composite(P)` from the occurrences `[c100, p100]` and again from `[p100, c100]` through `form`, and through `form_grouped`. Form a `Bag` from `[c100, p100, c100, p100]` through `form`. Form an `OrderedSet` from `[c100, p100]` through `form`. Record the outcome and the visiting order.
5. For the occurrences `[c100, p100, p200, g100]`, check for every ordered pair `(x, y)` that `compare_keys(x, y) == Some(Equal)` exactly when `planned_equality(x, y)` gives `true`. Separately call both on the malformed pair `(c100, c5_100)` (one declaration, different slot counts), in both orders.
6. Call `compare_keys` on `(p100, Integer 100)`, `(p100, bad)` where `bad` has declaration `P` and two slots, and `(f1, f2)`. Form a `Set` from `[f1, f2]` through `form_grouped` and through `form`, and from `[p100, Integer 100]` and from `[p100, bad]` through each.
7. Wrap `(p100, c100)` as the payload of two present `Option` values, as the only slot of two records of a fourth admitted declaration `W`, and as the only element of two one-element `Sequence` values. Call `compare_keys` on each wrapper pair, and each reversed.
8. Hold `p100`, `c100` and `g100` at the one position of a union member of `U`, call `compare_keys` on each of the nine ordered pairs, on a `P` union with a large slot against a `C` union with a small slot, on an `Alpha` union holding `C` against a `Zulu` union holding `P`, on unions of different declarations, and on one member with payload lengths 1 and 2, and form a `Set` and an `OrderedSet` over `[c, p, c]` and `[p, c, p]` unions through `form`.

## Expected Results

1. `(p100, c100)`, `(p200, c100)`, `(p100, c5_100)`, `(p100, g100)` and `(c100, g100)` give `Some(Less)`; each reversed pair gives `Some(Greater)`. The slots (`p200` against `c100`, different slot counts) never decide.
2. All forward pairs give `Some(Less)` and all reversed pairs give `Some(Greater)`, despite the descending integer slots. No distinct-key pair gives `Some(Equal)` or `None`.
3. `(p100, p100')` gives `Some(Equal)`, `(p100, p200)` gives `Some(Less)`, and the two self-comparisons give `Some(Equal)`. No pair of different declarations gives `Some(Equal)`.
4. Every `Set` and `Bag` form completes with no refusal. Both supply orders yield `[p100, c100]` for the `Set`, through `form` and through `form_grouped`; the `Bag` from four occurrences visits `[p100, p100, c100, c100]`. The `OrderedSet` visits `[c100, p100]`, supply order: it is not sorted by the key.
5. The two relations agree for all sixteen ordered pairs of the four values. For `(c100, c5_100)` in both orders `compare_keys` gives `None` and `planned_equality` refuses `CheckedInvariant { cause: ValueKindMismatch }`; there is no agreeing relation for a malformed pair.
6. The first two comparisons give `None`; `(f1, f2)` gives `None`. Each `Set` formation of step 6 refuses `CheckedInvariant { cause: CanonicalKeyUnavailable }` through `form_grouped` and `CheckedInvariant { cause: ValueKindMismatch }` through `form`, which refuses at membership comparison before ordering. No call returns a false `Some(Equal)`.
7. Each wrapper pair is `Some(Less)` in the order `(P-wrapper, C-wrapper)` and `Some(Greater)` reversed.
8. The unions order `P`, `C`, `G` by declaration key, whatever the slots, and keys are equal exactly when the unions are equal; `Alpha` holding `C` orders before `Zulu` holding `P`; the different-declaration and different-length pairs give `None`. The `Set` holds the `P` union then the `C` union from either supply; the `OrderedSet` keeps the first-occurrence order of the two.
