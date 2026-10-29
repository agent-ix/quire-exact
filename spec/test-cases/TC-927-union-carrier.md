---
id: TC-927
title: "Check the kernel union carrier"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-379
    type: verifies
---
# TC-927: Check the kernel union carrier

## Description

Verify [FR-379](../functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md) through the public kernel: `UnionMember::from_admitted`, `union`, `evaluate_union`, `UnionValue::from_admitted`, `plan_equality`, `planned_equality`, `compare_keys`, `form`, `form_grouped`, `form_collection` and `OptionValue::present`. Member identities are caller-admitted; the kernel mints and validates none.

## Test Procedure

1. Build a nullary member and a two-integer member, read their occurrence counts and fields, and call `ValueType::admits` for their declaration, another declaration and `Integer`.
2. Call `union` with arities 0 against 1 and 2 against 1, and with a `Boolean` at an `Integer` position.
3. Call `evaluate_union` with a first argument that is undefined, with a `Boolean` at an `Integer` position, and with completed arguments under a meter that records the charge log, including a meter whose retain limit is one fewer than the result.
4. Call `planned_equality` and `plan_equality` on equal unions, on unions that differ in a payload position and in the member.
5. Call `plan_equality` and `compare_keys` on a union against a union of another declaration, an enum member and a composite, and on one member with payload lengths 1 and 2.
6. Call `compare_keys` on `Alpha` against `Zulu` unions whose admitted member identities have the opposite order, on one member with differing payload positions, and on equal identifiers with different `VariantId`s.
7. Call `form_collection` for `Set`, `Bag`, `OrderedSet` and `Sequence` over four unions in a supply order and its reverse; call `form_grouped` for a `Set` over a union holding an IEEE value, and `form_collection` for a `Sequence` over it.
8. Nest a union ten thousand levels, and a mixed union, composite and collection nest, on a 512 KiB stack; clone, compare, key, format and drop them. Read the occurrence counts of a union that shares a child.
9. Build `OptionValue::present(Composite(K), union)` for equal unions and unions of different members, and call equality and `compare_keys`.

## Expected Results

1. Occurrence counts 1 and 3; the fields as supplied; only the declaration's `Composite` admits it.
2. `Value` `WrongArity { declared: 0, supplied: 1 }`, `Value` `WrongArity { declared: 2, supplied: 1 }` and `Position(0)` `TypeMismatch`.
3. `Undefined` with no later call and no admission; `DeferredResultNotAdmitted`; one `composite.result-retain` sized by the occurrence count; `Incomplete` naming `composite.result-retain`, the limit kind and the limit.
4. Equal unions are equal with one plus the payload pairs; the others are unequal.
5. `ValueKindMismatch` for each equality call and `None` for each key.
6. Alpha before Zulu regardless of member-identity order; the payload positions decide for one member; `None` for the identifier and variant disagreement.
7. The stated element orders from either supply; `CanonicalKeyUnavailable` for the `Set` and a kept pair for the `Sequence`.
8. Every call completes; storage grows linearly; the shared child counts twice.
9. The `Option` is admitted, equal for equal unions, unequal for different members, and keys by the union key.
