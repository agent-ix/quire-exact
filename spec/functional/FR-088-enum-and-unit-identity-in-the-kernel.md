---
id: FR-088
title: "The kernel carries enum rank and two-domain unit identity"
type: FR
relationships: []
---
# FR-088: The kernel carries enum rank and two-domain unit identity

## Description

The `quire-exact` kernel SHALL carry an enum value as its admitted `VariantId` and its rank in the canonical member list, and a quantity's unit as a two-domain `UnitId`. Naming declarations and supplying admitted identities are the caller's responsibility. The kernel SHALL neither compute nor validate a digest for these identities. This requirement is the kernel's own behaviour; the caller's checks keep the id `FR-088` in the repository that owns them.

## Use case

A caller builds a set of enum values and a quantity over a declared and a compound unit. The set visits its members in canonical order, a value with a wrong rank is refused, and two units never compare equal across domains.

## Behavior

1. **Enum rank.** A kernel enum shape holds its variants as a canonically ranked list. A value pairs a `VariantId` with that variant's rank; identity and equality use the `VariantId` only, and the rank orders sets and bags.
2. **Two-domain unit id.** A kernel `UnitId` distinguishes a declared unit's admitted node identity from a compound unit's admitted value identity. Equality compares the domain first and the admitted identity within that domain. Matching identity payloads across the two domains remain unequal. The kernel does not derive either payload.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-088-AC-11 | For an enum shape whose canonical member list is `b`, `a`, `c`, the ranks of the variants are `b` 0, `a` 1, `c` 2, and a set of all three values visits `b`, `a`, `c`. For an unordered enum whose canonical list is `a`, `b`, `c`, the ranks are `a` 0, `b` 1, `c` 2, and the set visits `a`, `b`, `c`. `compare_keys` orders two members of one shape by rank, even when their admitted `VariantId` identities have the opposite order. A value that pairs a variant's `VariantId` with a rank that differs from the shape's rank for it is refused at admission; a foreign variant of the same rank is refused too. Two shapes that differ in the identity of a variant but not in rank and order give the same ranks and the same visiting order. | Test (TC-409) |
| FR-088-AC-12 | A declared unit `UnitId` and a compound unit `UnitId` are unequal when their domains or admitted identities differ. A declared unit's `UnitId` carries its admitted node identity in the declared domain, and a compound unit's carries its admitted compound-value identity in the compound domain. Two `UnitId`s of different domains never compare equal even when their identity payloads match; two identities of one domain compare equal exactly when their payloads match. | Test (TC-411) |

## Status

Identity, rank, domain separation and refusal are required. The prior digest constructors and fixed digest representations are removed by IR-1105/IR-1133; the caller supplies admitted identities without a kernel digest seal.

## Dependencies

- The caller's identity admission and producer behavior under the same id lives in `agent-ix/quire-spec-language`; the source identity vocabulary is owned by `agent-ix/quire-specification`.
