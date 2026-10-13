---
id: SR-6413
title: "risk-complexity review of IR-705 bounded integer representation"
type: SpecReview
analysis: risk-complexity
scope: "agent-ix/quire-exact@5af4ded259b8005ec6c1052051c82b35c3a2295e; spec/functional/FR-371-bounded-integer-representation-refinement.md, spec/test-cases/TC-919-bounded-integer-representation-refinement.md, spec/assurance/AD-003-bounded-integer-representation.md"
review_set: all
---
## Summary

Reviewed concurrency, novel cache behavior, resource overhead and the proof ceiling. AD-003 assigns mitigations and keeps all consumer proof claims explicitly unverified.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-371 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | context_only | Where the kernel uses a bounded native representation for mathematical integers, the kernel SHALL preserve the exact value and public behavior of `Integer` across native and arbitrary-precision execution. |
| FR-371-AC-4 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator observer around standalone Integer calls confirms no allocation on that native path, including clone after cache access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128 MIN/MAX, compare independently prepared expected limbs, observe no stream allocation and confirm that initially empty caches remain empty. Observation |
| FR-371-AC-5 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Public signatures and all existing Integer methods remain available, except that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync and c |
| FR-371-AC-6 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and does not satisfy this criterion. |
| AD-003-risk (spec/assurance/AD-003-bounded-integer-representation.md) | examined | - The larger inline representation or BigInt cache branch may leave CBMC memory use unchanged or make it worse. The original unchanged CG healthy and mutation controls determine qualification; no 16GiB improvement is claimed until they pass. Owner: IR-705 proof lane; revisit after implementation review and consumer evidence. - The currently identified target's pointer-atomic metadata does not establish all downstream targets or dependency feature behavior. Owner: implementation and platform review; build the declared supported target set before adoption. |
| AD-003-dep (spec/assurance/AD-003-bounded-integer-representation.md) | context_only | The selected dependency is once_cell 1.21.4 with default features disabled and `alloc` enabled. Its safe `race::OnceBox` API supplies lazy stable references without std and supports concurrent initialization. Local rustc 1.98.1 target metadata reports pointer atomics for `thumbv7em-none-eabi`; that metadata is not a build result or proof of all supported targets. Implementation qualification must confirm supported target compatibility and that feature resolution does not introduce std or weaken local `forbid(unsafe_code)`. |

## Verdict

Reviewed concurrency, novel cache behavior, resource overhead and the proof ceiling. AD-003 assigns mitigations and keeps all consumer proof claims explicitly unverified.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
