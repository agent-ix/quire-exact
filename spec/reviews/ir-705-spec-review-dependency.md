---
id: SR-6411
title: "dependency review of IR-705 bounded integer representation"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-exact@5af4ded259b8005ec6c1052051c82b35c3a2295e; spec/functional/FR-371-bounded-integer-representation-refinement.md, spec/test-cases/TC-919-bounded-integer-representation-refinement.md, spec/assurance/AD-003-bounded-integer-representation.md"
review_set: all
---
## Summary

Reviewed the representation dependency, feature boundary, no_std constraints, atomic target qualification and dependency edges. The chosen API and required compatibility checks are explicit; no missing prerequisite edge was found in this diff.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-371 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | context_only | Where the kernel uses a bounded native representation for mathematical integers, the kernel SHALL preserve the exact value and public behavior of `Integer` across native and arbitrary-precision execution. |
| FR-371-AC-5 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Public signatures and all existing Integer methods remain available, except that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync and c |
| FR-371-AC-6 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | context_only | Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and does not satisfy this criterion. |
| AD-003-dep (spec/assurance/AD-003-bounded-integer-representation.md) | examined | The selected dependency is once_cell 1.21.4 with default features disabled and `alloc` enabled. Its safe `race::OnceBox` API supplies lazy stable references without std and supports concurrent initialization. Local rustc 1.98.1 target metadata reports pointer atomics for `thumbv7em-none-eabi`; that metadata is not a build result or proof of all supported targets. Implementation qualification must confirm supported target compatibility and that feature resolution does not introduce std or weaken local `forbid(unsafe_code)`. |
| AD-003-risk (spec/assurance/AD-003-bounded-integer-representation.md) | context_only | - The larger inline representation or BigInt cache branch may leave CBMC memory use unchanged or make it worse. The original unchanged CG healthy and mutation controls determine qualification; no 16GiB improvement is claimed until they pass. Owner: IR-705 proof lane; revisit after implementation review and consumer evidence. - The currently identified target's pointer-atomic metadata does not establish all downstream targets or dependency feature behavior. Owner: implementation and platform review; build the declared supported target set before adoption. |

## Verdict

Reviewed the representation dependency, feature boundary, no_std constraints, atomic target qualification and dependency edges. The chosen API and required compatibility checks are explicit; no missing prerequisite edge was found in this diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
