---
id: FR-088
title: "The kernel carries enum rank and two-domain unit identity"
type: FR
relationships: []
---
# FR-088: The kernel carries enum rank and two-domain unit identity

## Description

The `quire-exact` kernel SHALL carry an enum value as its `VariantId` and its rank in the canonical member list, and a quantity's unit as a two-domain `UnitId`. Naming the declaration and minting the digests are the caller's. This requirement is the kernel's own behaviour; the caller's checks keep the id `FR-088` in the repository that owns them.

## Use case

A caller builds a set of enum values and a quantity over a declared and a compound unit. The set visits its members in canonical order, a value with a wrong rank is refused, and two units never compare equal across domains.

## Behavior

1. **Enum rank.** A kernel enum shape holds its variants as a canonically ranked list. A value pairs a `VariantId` with that variant's rank; identity and equality use the `VariantId` only, and the rank orders sets and bags.
2. **Two-domain unit id.** A kernel `UnitId` is a domain-labelled digest record over a declared unit's node key (`quire.checked-semantic-node/v1`) or a compound unit's digest (`quire.value.compound-unit/v1`). Equality is lexical on the domain, then the bytes.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-088-AC-11 | For an enum shape whose canonical member list is `b`, `a`, `c`, the ranks of the variants are `b` 0, `a` 1, `c` 2, and a set of all three values visits `b`, `a`, `c`. For an unordered enum whose canonical list is `a`, `b`, `c`, the ranks are `a` 0, `b` 1, `c` 2, and the set visits `a`, `b`, `c`. `compare_keys` orders two members of one shape by rank, whatever the order of their `VariantId` digests. A value that pairs a variant's `VariantId` with a rank that differs from the shape's rank for it is refused at admission. Two shapes that differ in the identity of a variant but not in rank and order give the same ranks and the same visiting order. | Test (TC-409) |
| FR-088-AC-12 | A declared unit `UnitId` and a compound unit `UnitId` are unequal when their domains or bytes differ. A declared unit's `UnitId` carries its node key under the `quire.checked-semantic-node/v1` label, and a compound unit's carries its digest under the `quire.value.compound-unit/v1` label. Two `UnitId`s of different domains never compare equal, whatever their bytes. | Test (TC-411) |

## Status

Implemented.

## Planned identity representation review

The following replacement work is **Planned**. It is not an executable claim and does not change
FR-088-AC-11 or FR-088-AC-12, their tests, or the current digest-shaped kernel API.

| Candidate | Required behavior before replacement | Upstream decision needed |
| --- | --- | --- |
| `VariantId` and `UnitId` representation | Preserve refusal of a variant from another declaration even when its rank matches; preserve inequality between declared and compound units even when their payload bytes match. | QSpec and the QSL identity producer must define the replacement identity values, their domains, and how admitted declarations and compound units mint them. |
| Related kernel identities (`NodeKey`, `EffectiveId`, `UniverseId`, `MemberId`, and `PopulationId`) | Preserve distinct checked nodes within one package, distinct effective declarations, foreign reference universes, and member/population ownership; preserve the existing comparison and refusal behavior for those distinctions. | QSpec and QSL must define one admitted representation and producer for each identity before the kernel changes its carried types. |

The replacement contract must identify the same-declaration and foreign-declaration cases that
remain equal or unequal, the admission boundary, and the ordering used by canonical keys. A
content-identity requirement does not by itself require a frozen 32-byte digest representation.
No constructor, decoder, comparison, or refusal check is removed while those upstream contracts
remain unsettled.

## Dependencies

- None. The caller's behaviour under the same id lives in `agent-ix/quire-spec-language`.
