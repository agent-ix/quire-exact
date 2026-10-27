---
id: TC-936
title: "Bounded quantity value type"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-387
    type: verifies
---
# TC-936: Bounded quantity value type

## Description

Verify [FR-387](../functional/FR-387-bounded-quantity-value-type.md) through the public `BoundedQuantityType::new`, `ValueType::bounded_quantity`, `ValueType::admits` and the traits of `ValueType`. Scope: FR-387-AC-1 through FR-387-AC-9.

## Test Procedure

1. Build bounded quantity types over unit `U` for each of the four bound presence combinations; read `unit()`, `min()` and `max()`; wrap with `ValueType::bounded_quantity`.
2. Call `new` with min 5 / max 3, with a min in another unit, with a max in another unit, with min equal to max, with 3 / 5, and with `1/3` / `1/2`.
3. Compare and hash the types of FR-387-AC-3 against each other, against `ValueType::Quantity(U)` and against `Option` of the same type.
4. Nest a bounded type in `Option`, `Collection` and `Map` links; compare and hash chains differing only in the innermost bound.
5. On a 512 KiB stack thread build the 100,000-link `Option` chain, clone, compare, hash, format and drop it, and a second chain differing in the innermost bound.
6. Format the `[0, 100]` and `[None, None]` types in compact and alternate (`{:#?}`) mode.
7. Call `admits` on `[0, 100]`, `[1, 100]`, `[0, None]` and `[None, 100]` with the magnitudes of FR-387-AC-7.
8. Call `admits` on a `[0, 100]` type with a magnitude 50 quantity in another unit, a `Text` value, a `Boolean` value and an `Option` value; call `admits` on `ValueType::Quantity(U)` with magnitude 101.
9. Build `[0, 100]` and `[0, 50]` types over `U` and a `[0, 100]` type from effective bounds folded from two different caller chains; compare them and read the accessors; observe the error cause of every refused construction in step 2.

## Expected Results

1. Accessors return the arguments; the result is a `ValueType::BoundedQuantity`.
2. `Err` with `MalformedBoundedQuantityType` for the first three; `Ok` for the others.
3. Equal exactly for equal unit, minimum and maximum; equal types hash equal; presence differences and the cross-variant pairs are unequal.
4. Differing innermost bounds make the chains unequal; equal chains hash equal.
5. Every operation completes with no stack overflow; balanced brackets; the innermost difference is unequal.
6. The exact `#[derive(Debug)]` strings in both modes.
7. `true` (admitted) for 0, 50, 100 on `[0, 100]`; `false` (not admitted) for -1, 101 and `1/2` on `[1, 100]`; `[0, None]` and `[None, 100]` as FR-387-AC-7.
8. `false` for the four values on the bounded type; `true` on `Quantity(U)` for 101.
9. The only cause seen is `MalformedBoundedQuantityType`; no other cause exists on the constructor; equal effective bounds give equal types regardless of origin; `[0, 50]` is unequal to `[0, 100]` though it refines it.
