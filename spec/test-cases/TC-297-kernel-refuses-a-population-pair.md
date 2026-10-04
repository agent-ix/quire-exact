---
id: TC-297
title: "Kernel admits, plan_pairs and compare_keys refuse a population pair"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-089
    type: verifies
---
# TC-297: Kernel admits, plan_pairs and compare_keys refuse a population pair

## Description

Verify FR-089-AC-6: the kernel refuses every population pair. It catches a kernel `admits` that admits every `Value::Population` under any `ValueType::Population` (widening the type with no declared maximum checked), a `plan_pairs` that compares two population identities structurally as equality operands, and a `compare_keys` that orders population identities by digest.

Scope: FR-089-AC-6.

## Test Procedure

1. Check `ValueType::Population(Some(5)).admits(&Value::Population(id))` for a `PopulationId` built with `PopulationId::from_digest`.
2. Call `plan_pairs` on two `Value::Population` operands with distinct `PopulationId`s.
3. Call `compare_keys` on two `Value::Population` values with distinct `PopulationId`s.

## Expected Results

Step 1 returns `false`. Step 2 returns `Err(Refusal::CheckedInvariant)`. Step 3 returns `None`.

## Status

Implemented. The tests are `value::tests::admits_refuses_a_population_pair` (step 1), `equality::tests::plan_pairs_refuses_a_population_pair` (step 2) and `key::tests::compare_keys_yields_no_key_for_a_population_pair` (step 3).
