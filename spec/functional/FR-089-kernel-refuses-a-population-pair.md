---
id: FR-089
title: "The kernel refuses every population pair"
type: FR
relationships: []
---
# FR-089: The kernel refuses every population pair

## Description

The `quire-exact` kernel `Value` SHALL carry a `Population` variant whose payload is an opaque `PopulationId`, never a population binding. The kernel is a leaf with no access to a caller's `PopulationId` to binding correspondence, so it SHALL refuse every population pair. The declared-maximum comparison is the caller's and keeps the id `FR-089` (AC-5) in the repository that owns it.

## Use case

A caller admits a population argument. The kernel never admits it on its own; the caller resolves the identity to its binding and compares the declared maximum.

## Behavior

1. **Opaque identity.** `Value::Population` carries a `PopulationId` and nothing else.
2. **Refused pair.** `ValueType::admits` returns false for every population pair, `equality::plan_pairs` refuses two population operands with `Refusal::CheckedInvariant`, and `key::compare_keys` returns `None` for two population values.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-089-AC-6 | Given any `ValueType::Population(maximum)` and any `Value::Population(population_id)`, kernel `ValueType::admits` returns false; given two `Value::Population` operands, kernel `equality::plan_pairs` returns `Err(Refusal::CheckedInvariant)` and kernel `key::compare_keys` returns `None`. | Test (TC-297) |

## Status

Implemented.

## Dependencies

- None. The caller's behaviour under the same id lives in `agent-ix/quire-spec-language`.
