---
id: SR-6414
title: "scope-boundary review of IR-705 bounded integer representation"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-exact@5af4ded259b8005ec6c1052051c82b35c3a2295e; spec/functional/FR-371-bounded-integer-representation-refinement.md, spec/test-cases/TC-919-bounded-integer-representation-refinement.md, spec/assurance/AD-003-bounded-integer-representation.md"
review_set: all
---
## Summary

Reviewed the kernel versus consumer qualification boundary and ownership language. The spec keeps implementation in quire-exact and the unchanged CG controls consumer-owned.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-371-AC-6 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and does not satisfy this criterion. |
| FR-371-AC-5 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | context_only | Public signatures and all existing Integer methods remain available, except that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync and c |
| FR-371 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | context_only | Where the kernel uses a bounded native representation for mathematical integers, the kernel SHALL preserve the exact value and public behavior of `Integer` across native and arbitrary-precision execution. |
| AD-003 (spec/assurance/AD-003-bounded-integer-representation.md) | examined | This decision covers the private storage representation of `quire-exact::Integer`, its exact refinement to mathematical integers, and the stable borrowed `as_big` interface. It preserves the public `Integer` API, metered arithmetic schedule in FR-362, allocation admission and refusal contracts, no_std plus alloc support, and the crate's local unsafe prohibition. It does not alter Decimal semantics or the separate CG consumer proof harnesses. |
| TC-919-step-6 (spec/test-cases/TC-919-bounded-integer-representation-refinement.md) | examined | For AC-6, the consumer owner selects the original genuine healthy TC027, TC027 bound mutant and TC025 arithmetic mutant against the actual reviewed/adopted kernel identity. Preflight the exact selection without execution, then use the separately allocated runner/resource envelope. Preserve original domains, oracle, unwind3, Cadical, 600s, 16GiB, completion cover and falsification witnesses. Record each actual result and cleanup separately. Run the existing all-four-operation and refinement/accounting/denial obligations; do not accept a healthy-only or sp |

## Verdict

Reviewed the kernel versus consumer qualification boundary and ownership language. The spec keeps implementation in quire-exact and the unchanged CG controls consumer-owned.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
