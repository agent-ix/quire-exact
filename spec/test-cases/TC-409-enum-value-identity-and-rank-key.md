---
id: TC-409
title: "An enum value's VariantId and rank order sets and bags"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: verifies
---
# TC-409: An enum value's VariantId and rank order sets and bags

## Description

Verify FR-088-AC-11. A kernel enum value carries its `VariantId` and its rank in the canonical member list; the rank is the canonical key, so a set visits an ordered enum's values in declaration order and an unordered enum's values in case identifier byte order. Identity and equality use the `VariantId` only.

Scope: FR-088-AC-11.

## Test Procedure

1. Build the ranked shape of the ordered enum `b`, `a`, `c`, read each variant's rank, and form the kernel set `{c, a, b}`. Record the visiting order.
2. Repeat with the unordered enum, whose canonical list is `a`, `b`, `c`.
3. Build a value that pairs `a`'s `VariantId` with a rank other than its own and admit it against the shape.
4. Compare the ranks and the visiting order of two shapes whose variants differ in identity but not in rank or order.
5. Compare two members of one shape with `compare_keys`, the variant with the larger digest holding the lower rank.

## Expected Results

- Step 1: the ranks are `b` 0, `a` 1, `c` 2, and the set visits `b`, `a`, `c`.
- Step 2: the ranks are `a` 0, `b` 1, `c` 2, and the set visits `a`, `b`, `c`.
- Step 3: admission refuses the value.
- Step 4: the ranks and the visiting order are equal.
- Step 5: the lower rank orders first, whatever the digests.

## Status

Implemented. The tests are in `src/value.rs` and `src/key.rs`, tagged `#[trace("TC-409", "FR-088-AC-11")]`.
