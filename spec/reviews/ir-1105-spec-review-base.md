---
id: SR-6416
title: "Base spec review of admitted kernel identity changes"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact; PR #99; FR-088, FR-089, FR-377, FR-379, TC-297, TC-409, TC-411, TC-925, TC-927"
review_set: base
---

## Summary

Independent review of PR #99 at 5dabcddd for IR-1105/IR-1133. Examined all changed FR criteria and all five linked test cases. The removal of fixed digest seals preserves enum rank and foreign-variant refusal, unit-domain inequality, population-pair refusal, declaration-key priority, and union member ordering, except that FR-377-AC-2 still assumes an unprovided segmented key representation.

No active AssuranceProfile was found under this repository's spec tree. Quire validation and `git diff --check` passed. `quoin advise --mismatch-only` reports existing repository-wide advisories; the changed FR-089-AC-2 Inspection criterion continues to describe a structural dependency boundary.

## Examined units

| ID | Role | Excerpt |
| --- | --- | --- |
| FR-088 | examined | The `quire-exact` kernel SHALL carry an enum value as its admitted `VariantId` and its rank in the canonical member list, and a quantity's unit as a two-domain `UnitId`. Naming declarations and supplying admitted identities are the caller's responsibility. The kernel SHALL neither compute nor validate a digest for these identities. This requirement is the kernel's own behaviour; the caller's checks keep the id `FR-088` in the repository that owns them. |
| FR-088-AC-11 | examined | For an enum shape whose canonical member list is `b`, `a`, `c`, the ranks of the variants are `b` 0, `a` 1, `c` 2, and a set of all three values visits `b`, `a`, `c`. For an unordered enum whose canonical list is `a`, `b`, `c`, the ranks are `a` 0, `b` 1, `c` 2, and the set visits `a`, `b`, `c`. `compare_keys` orders two members of one shape by rank, even when their admitted `VariantId` identities have the opposite order. A value that pairs a variant's `VariantId` with a rank that differs from the shape's rank for it is refused at admission; a foreign variant of the same rank is refused too. T |
| FR-088-AC-12 | examined | A declared unit `UnitId` and a compound unit `UnitId` are unequal when their domains or admitted identities differ. A declared unit's `UnitId` carries its admitted node identity in the declared domain, and a compound unit's carries its admitted compound-value identity in the compound domain. Two `UnitId`s of different domains never compare equal even when their identity payloads match; two identities of one domain compare equal exactly when their payloads match. |
| FR-089 | examined | The `quire-exact` kernel `Value` SHALL carry a `Population` variant whose payload is an opaque `PopulationId`, never a population binding. The kernel is a leaf with no access to a caller's `PopulationId` to binding correspondence, so it SHALL refuse every population pair. The declared-maximum comparison is the caller's and keeps the id `FR-089` (AC-5) in the repository that owns it. |
| FR-089-AC-2 | examined | The kernel `Value::Population` variant's payload type is `PopulationId`; no kernel source file imports a population binding or any other caller's model type to define, construct or match this variant, and the crate's `[dependencies]` name no crate that owns one. |
| FR-089-AC-6 | examined | Given any `ValueType::Population(maximum)` and any `Value::Population(population_id)`, kernel `ValueType::admits` returns false; given two `Value::Population` operands, kernel `equality::plan_pairs` returns `Err(Refusal::CheckedInvariant { cause: PopulationPair })` and kernel `key::compare_keys` returns `None`. |
| FR-377 | examined | When `compare_keys` orders two composite values whose admitted declaration keys differ, the `quire-exact` kernel SHALL order them by the keys' own total order, compared before any slot. The kernel SHALL NOT derive a digest or compare a fixed digest seal to establish that order. |
| FR-377-AC-1 | examined | For admitted declaration keys `P` and `C` with `P < C` under the `NodeKey` order, `compare_keys(a, b)` for composites of `P` and `C` is `Some(Less)` and `compare_keys(b, a)` is `Some(Greater)`, whatever their slots: slots equal, slots that would order the other way, and different slot counts. |
| FR-377-AC-2 | examined | Among admitted declaration keys with a shared leading portion, the first differing part determines their order; among keys whose leading portions differ, that leading difference decides even when a later part has the opposite order. The result is the same as the `NodeKey` order in both cases, without a fixed key length or digest expectation. |
| FR-377-AC-3 | examined | Two composites of one declaration key keep the slot order of the rule before this requirement; `compare_keys(x, x)` is `Some(Equal)`; two composites of different declarations are never `Some(Equal)`, even with identical slots. |
| FR-377-AC-4 | examined | A `Set` and a `Bag` of element type `Composite(P)` formed through `form` from one composite of each of two declarations, in each of the two supply orders, complete and visit the member of the smaller declaration key first. `form_grouped` over the same two members gives the same order. A `Bag` formed through `form` from two equal members of each declaration visits the two groups in that order. |
| FR-377-AC-5 | examined | Equal keys occur exactly when values are equal: for a set of composites of two declarations with equal and unequal slots, every pair has `compare_keys == Some(Equal)` exactly when `planned_equality` gives `true`. |
| FR-377-AC-6 | examined | A composite against an `Integer`, two composites of one declaration key and different slot counts, and a pair of `Float` members each return `None` from `compare_keys`. As the two members of a `Set`, each refuses `CanonicalKeyUnavailable` through `form_grouped` and `ValueKindMismatch` through `form`, which refuses at membership comparison before any ordering. |
| FR-377-AC-7 | examined | The key order of a pair of different declarations holds as the payload of two `Option`s, in a record slot and as an element of two `Sequence` values, ordering the enclosing values by the same declaration-key rule. |
| FR-377-AC-8 | examined | `compare_keys` on unions of one member that hold a `P`, `C` or `G` composite at the payload position returns `Some(Less)` for `P` against `C` and `C` against `G`, whatever the slots, and `Some(Equal)` agrees with `planned_equality` on all nine ordered pairs of the three; an `Alpha` union holding `C` orders before a `Zulu` union holding `P`; `compare_keys` returns `None` for unions of different declarations and for one member with different payload lengths; `form` of a `Set` over `C`, `P`, `C` unions (and over `P`, `C`, `P`) keeps the `P` union then the `C` union; `form` of an `OrderedSet` keep |
| FR-379 | examined | The `quire-exact` kernel SHALL carry one active member of a union declaration, with its payload values in declared position order, as `Value::Union`, with construction, comparison, key order, clone, `Debug` and drop that do not grow the native stack with the depth. |
| FR-379-AC-1 | examined | A nullary member has an occurrence count of 1 and a two-position member over two integers has 3. The value reports its declaration, `VariantId`, identifier and payload in position order. `ValueType::Composite` of its declaration admits it, `Composite` of another declaration and `Integer` do not. |
| FR-379-AC-2 | examined | `union` refuses an arity of 0 against 1 supplied and 2 against 1 supplied with `Value` `WrongArity`, and a `Boolean` at an `Integer` position with `Position(0)` `TypeMismatch`. |
| FR-379-AC-3 | examined | `evaluate_union` returns `Undefined` for the first argument that is undefined, runs no later argument and admits no charge. A completed `Boolean` at an `Integer` position refuses `DeferredResultNotAdmitted`. A completed evaluation runs the arguments in order and admits one `composite.result-retain` with the occurrence count of the result, and a retain limit of one fewer returns `Incomplete` naming the charge point, the limit kind and the limit. |
| FR-379-AC-4 | examined | Unions of one declaration and member with equal payloads are equal, with differing payload positions or members are not, and the pair count is one plus the payload pairs. |
| FR-379-AC-5 | examined | A union against a union of another declaration, an enum member and a composite refuses `ValueKindMismatch` in `plan_equality` and gives `None` from `compare_keys`. One member with payload lengths 1 and 2 refuses `ValueKindMismatch` and gives `None`. |
| FR-379-AC-6 | examined | Members `Alpha` and `Zulu` order by identifier even when their admitted member identities have the opposite order and regardless of their payloads. One member orders by the payload positions in order. Declaration order and member-identity order do not reach the order. Equal identifiers with different `VariantId`s give `None`. |
| FR-379-AC-7 | examined | `Set`, `Bag`, `OrderedSet` and `Sequence` over four unions, from a supply order and its reverse, hold the stated elements. A union with an IEEE payload refuses `CanonicalKeyUnavailable` through `form_grouped` for a `Set` and is kept by a `Sequence`. |
| FR-379-AC-8 | examined | Clone, equality, key comparison, `Debug` and drop of a union nested ten thousand levels complete on a 512 KiB stack, a mixed union, composite and collection nest does the same, storage grows linearly with depth, and a shared child counts each occurrence. |
| FR-379-AC-9 | examined | A present `Option` of declared type `Composite(K)` over a union of declaration `K` is admitted, two such `Option` values with equal unions are equal and with different members are not, and they compare by the key of the union. |
| TC-297 | examined | Verify FR-089-AC-6: the kernel refuses every population pair. It catches a kernel `admits` that admits every `Value::Population` under any `ValueType::Population` (widening the type with no declared maximum checked), a `plan_pairs` that compares two population identities structurally as equality operands, and a `compare_keys` that orders population identities by their admitted identities. |
| TC-409 | examined | Verify FR-088-AC-11. A kernel enum value carries its `VariantId` and its rank in the canonical member list; the rank is the canonical key, so a set visits an ordered enum's values in declaration order and an unordered enum's values in case identifier byte order. Identity and equality use the `VariantId` only. |
| TC-411 | examined | Verify FR-088-AC-12. A kernel `UnitId` carries the admitted identity of a declared unit or compound unit with a distinct domain. Equality requires both the domain and the identity payload to match. |
| TC-925 | examined | Verify [FR-377](../functional/FR-377-composite-canonical-key-across-declarations.md) through `compare_keys` and the collection forms `form` and `form_grouped` over `Set` and `Bag`, with members built by `from_admitted_slots`. Reuse the value shapes of [TC-924](./TC-924-composite-equality-across-declarations.md) (`p100`, `p200`, `c100`, `c5_100`, `g100`), with distinct caller-admitted declaration keys `P < C < G` under `NodeKey` ordering. The fixture identities do not prescribe a digest or key length. Add: |
| TC-927 | examined | Verify [FR-379](../functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md) through the public kernel: `UnionMember::from_admitted`, `union`, `evaluate_union`, `UnionValue::from_admitted`, `plan_equality`, `planned_equality`, `compare_keys`, `form`, `form_grouped`, `form_collection` and `OptionValue::present`. Member identities are caller-admitted; the kernel mints and validates none. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-2 and TC-925 require leading and final key parts although the admitted `NodeKey` contract only promises a total order; a replacement identity may have no such parts, so this case cannot be constructed without imposing an unapproved representation. Keep a falsifying order test based on distinct admitted keys and `NodeKey` ordering, with slots arranged to oppose it. | spec/functional/FR-377-composite-canonical-key-across-declarations.md:47; spec/test-cases/TC-925-composite-canonical-key-across-declarations.md:18-22 |

## Verdict

The initial review required one spec correction. FND-001 was fixed at PR head c13f6d4985d43a165645ae6f0d8d369b35ac1a6f. No code or build checks were run for this spec-only PR.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-377-AC-2 now uses three distinct admitted keys ordered by `NodeKey`; TC-925 checks all forward and reverse comparisons with descending integer slots and refuses `Equal` or `None` for distinct keys. No segmented or fixed-byte representation is needed. |
