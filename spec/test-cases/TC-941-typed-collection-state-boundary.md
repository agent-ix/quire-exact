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

1. Build bounded sequence and ordered-set integer prestates from `[2,2,1]` with an authenticated integer admission authority; inspect both as produced poststates and compare their type, bound, element count and stored element order. Check that the builder's ordered-set order is the source stream's first-occurrence order.
2. Build each kind at reachable counts 1 through 5 under `[2,4]`, counting ordered-set distinct members rather than source occurrences. Check `[0,0]` at zero and one, and compare an absent bound with present `[0,u64::MAX]` for type identity while admitting ordinary counts. Inject one wrong exact-type element at a known index and assert `Component::Element(index)` with `ConstructionCause::TypeMismatch` before collection charges; inject a matching-outer element rejected by the authority and assert its typed admission cause. Deny each reached charge with a public meter.
3. Use public no-recheck construction to make poststates with a wrong element, violated bound and duplicate ordered-set member. Inspect them under the declared type, then inspect a wrong kind, bound and noncollection value. Forge a `Composite(P)` element with the correct outer declaration key but a missing or wrong nested required slot; show `ValueType::admits` accepts its outer type and the schema-aware authority rejects it. Withhold the authority and check the typed missing-admission-authority refusal and absence of a view. Store a conforming descendant-declared `Composite(C)` under exact element type `Composite(P)` and show exact inspection refuses at that element despite a conformance-admitting authority, while FR-378's separately trusted construction can retain it.
4. Compare inspection's recorded charge points with the specified membership and bound charges and prove no element-evaluation or result-retain charge. Deny each reached charge and check that no view escapes. Inspect two distinct-member ordered sets in different stored orders: both are valid views, and each retains its own order; only the source-bearing builder can prove original first-occurrence order.

## Expected Results

- Steps 1 and 2 satisfy FR-392-AC-1 through AC-3: sequence keeps all occurrences, ordered set keeps first members, bounds count by kind, and refusals identify the typed component or bound.
- Steps 3 and 4 satisfy FR-392-AC-4 through AC-6: valid views borrow the original value; forged nested members are rejected by the caller's authenticated schema authority; missing authority and malformed poststates refuse without normalization; denied charges yield `Incomplete` without a view.
- Source inspection satisfies FR-392-AC-7: both directions use the exact kernel's one collection value representation.

## Status

Planned; no executable test or CG/Kani evidence is claimed.
