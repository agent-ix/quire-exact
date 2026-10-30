---
id: TC-941
title: "Checked typed collection prestate and poststate readback"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-392
    type: verifies
---
# TC-941: Checked typed collection prestate and poststate readback

## Description

Exercise [FR-392](../functional/FR-392-typed-collection-state-boundary.md) through the public `quire_exact` API. This is a planned test case, not implementation evidence.

## Test Procedure

1. Build bounded sequence and ordered-set integer prestates from `[2,2,1]`; inspect both as produced poststates and compare their type, bound, element count and element order.
2. Build each kind at counts below, on and above its inclusive bounds, including an absent bound and a present `[0,u64::MAX]` bound; inject one wrong element at a known index and deny each reached charge with a public meter.
3. Use public no-recheck construction to make poststates with a wrong element, violated bound and duplicate ordered-set member. Inspect them under the declared type, then inspect a wrong kind, bound and noncollection value. Include a descendant-declared composite under an ancestor exact element type.
4. Compare inspection's recorded charge points with the specified membership and bound charges and prove no element-evaluation or result-retain charge. Deny each reached charge and check that no view escapes.

## Expected Results

- Steps 1 and 2 satisfy FR-392-AC-1 through AC-3: sequence keeps all occurrences, ordered set keeps first members, bounds count by kind, and refusals identify the typed component or bound.
- Steps 3 and 4 satisfy FR-392-AC-4 through AC-6: valid views borrow the original value; malformed poststates are refused without normalization; denied charges yield `Incomplete` without a view.
- Source inspection satisfies FR-392-AC-7: both directions use the exact kernel's one collection value representation.

## Status

Planned; no executable test or CG/Kani evidence is claimed.
