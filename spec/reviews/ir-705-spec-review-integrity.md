---
id: SR-6410
title: "integrity review of IR-705 bounded integer representation"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@5af4ded259b8005ec6c1052051c82b35c3a2295e; spec/functional/FR-371-bounded-integer-representation-refinement.md, spec/test-cases/TC-919-bounded-integer-representation-refinement.md, spec/assurance/AD-003-bounded-integer-representation.md"
review_set: all
---
## Summary

Reviewed criterion atomicity, observable outcomes, consistency between FR-371, TC-919 and AD-003, and the planned status. No contradictions or untestable obligations were found.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-371 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Where the kernel uses a bounded native representation for mathematical integers, the kernel SHALL preserve the exact value and public behavior of `Integer` across native and arbitrary-precision execution. |
| FR-371-AC-1 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve the exact value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX, -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range. MIN has magnitude bit length 128; its negation is positive 2^127 through genuine arbitrary-precision continuation. Compare fitting operations with independent checked i128 arithmetic; compare non-fitting results with independently authored decimal literals. Rejec |
| FR-371-AC-2 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal, identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt access and cloning. A borrowed as_big reports the exact value throughout concurrent read-only accesses; all accesses to one retained materialization observe the same object. Clone/drop do not invalidate surviving permitted borrows or introduce a cache-state value. |
| FR-371-AC-3 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native, native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete fields and admitted prefixes with the independent normative formulas. No representation transition manufactures a FunctionCall or other charge. |
| FR-371-AC-4 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator observer around standalone Integer calls confirms no allocation on that native path, including clone after cache access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128 MIN/MAX, compare independently prepared expected limbs, observe no stream allocation and confirm that initially empty caches remain empty. Observation |
| FR-371-AC-5 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Public signatures and all existing Integer methods remain available, except that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync and c |
| FR-371-AC-6 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and does not satisfy this criterion. |
| TC-919 (spec/test-cases/TC-919-bounded-integer-representation-refinement.md) | examined | This planned case checks [FR-371](../functional/FR-371-bounded-integer-representation-refinement.md) at real kernel entry points. It separates exact semantic refinement, implementation resource observations and genuine consumer proof qualification. This document does not execute or replace consumer-owned CG TC025/TC027. |
| TC-919-expected (spec/test-cases/TC-919-bounded-integer-representation-refinement.md) | examined | Exact values and every public observation match the mathematical contract. Native arithmetic overflow reaches genuine arbitrary-precision execution. Meter prefixes, exact requests, counters and domain refusal remain unchanged. The bounded standalone path makes no allocation, while lazy BigInt access, if selected, exhibits its explicitly disclosed allocation behavior and stable retained borrow. Local unsafe/std leakage, lost traits, temporary borrow, cache-state equality/hash, wrapping, extra charge or unobserved discarded candidate falsifies the correspo |
| TC-919-status (spec/test-cases/TC-919-bounded-integer-representation-refinement.md) | examined | PLANNED / UNRUN. No implementation, runtime allocation observation, test execution, Trace binding or proof result is claimed by this proposal. The unchanged original CG TC-025/TC-027 healthy and mutation controls remain a separate consumer-owned qualification boundary; no 16GiB improvement is established until all three complete as specified. The kernel and consumer checks have different owners and must produce their own evidence before AC-6 or IR-705 is declared complete. |
| AD-003 (spec/assurance/AD-003-bounded-integer-representation.md) | examined | This decision covers the private storage representation of `quire-exact::Integer`, its exact refinement to mathematical integers, and the stable borrowed `as_big` interface. It preserves the public `Integer` API, metered arithmetic schedule in FR-362, allocation admission and refusal contracts, no_std plus alloc support, and the crate's local unsafe prohibition. It does not alter Decimal semantics or the separate CG consumer proof harnesses. |

## Verdict

Reviewed criterion atomicity, observable outcomes, consistency between FR-371, TC-919 and AD-003, and the planned status. No contradictions or untestable obligations were found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
