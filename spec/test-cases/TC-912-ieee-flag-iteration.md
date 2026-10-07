---
id: TC-912
title: "Deterministic IEEE flag iteration"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-364
    type: verifies
---
# TC-912: Deterministic IEEE flag iteration

## Description

Verify [FR-364](../functional/FR-364-ieee-flag-iteration.md) through the public owner API.

## Test Procedure

Enumerate the 32 flag subsets independently from the five public variants. Construct each set through public `FromIterator` in forward and reverse order; compare both iteration sequences with the independently filtered vocabulary. Include empty and full sets.

## Expected Results

Both insertion orders produce the same stated iteration order, with precisely the present flags once each.

## Status

Planned for IR-673; executable evidence has not yet been added.
