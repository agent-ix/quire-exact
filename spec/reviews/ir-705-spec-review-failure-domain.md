---
id: SR-6409
title: "failure-domain review of IR-705 bounded integer representation"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-exact@5af4ded259b8005ec6c1052051c82b35c3a2295e; spec/functional/FR-371-bounded-integer-representation-refinement.md, spec/test-cases/TC-919-bounded-integer-representation-refinement.md, spec/assurance/AD-003-bounded-integer-representation.md"
review_set: all
---
## Summary

Reviewed concurrent cache initialization, stable borrows, losing-candidate cleanup, clone/drop and allocator failure behavior. The stated race, ownership and panic/resource limits cover the identified failure modes.

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-371 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | context_only | Where the kernel uses a bounded native representation for mathematical integers, the kernel SHALL preserve the exact value and public behavior of `Integer` across native and arbitrary-precision execution. |
| FR-371-AC-2 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal, identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt access and cloning. A borrowed as_big reports the exact value throughout concurrent read-only accesses; all accesses to one retained materialization observe the same object. Clone/drop do not invalidate surviving permitted borrows or introduce a cache-state value. |
| FR-371-AC-4 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator observer around standalone Integer calls confirms no allocation on that native path, including clone after cache access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128 MIN/MAX, compare independently prepared expected limbs, observe no stream allocation and confirm that initially empty caches remain empty. Observation |
| FR-371-AC-5 (spec/functional/FR-371-bounded-integer-representation-refinement.md) | examined | Public signatures and all existing Integer methods remain available, except that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync and c |
| TC-919-step-2 (spec/test-cases/TC-919-bounded-integer-representation-refinement.md) | examined | For AC-2, compare mathematical equality, order, hash input behavior for the same hasher/target, Display and Debug across construction routes, cache access and cloning. Obtain as_big borrows and retain them during synchronized concurrent read-only accesses. Check exact values and stable retained-object identity. Join owned threads before dropping their owner; test clone/drop with permitted surviving borrows without introducing a lifetime violation as the oracle. |
| TC-919-step-5 (spec/test-cases/TC-919-bounded-integer-representation-refinement.md) | examined | For AC-5, verify the lock resolves once_cell 1.21.4 with default features disabled and alloc enabled; compile the existing public method and trait checks under default/test-support and the required no_std target in the eventual implementation lane. Exercise real IEEE consumers. For `once_cell::race::OnceBox<BigInt>`, use synchronized owned concurrent readers and safe allocation/deallocation observations to check the exact retained value, one retained winner, released temporary candidates, and that all returned borrows refer to the winner. Record first-ac |
| AD-003-cache (spec/assurance/AD-003-bounded-integer-representation.md) | examined | The stable public `as_big(&self) -> &BigInt` contract requires storage that outlives each returned borrow. For Small, the first BigInt-required observation initializes a private safe `once_cell::race::OnceBox<BigInt>` from the immutable i128 payload; subsequent observations borrow the retained immutable winner. For Large, observations borrow its stored BigInt. Concurrent first access may construct multiple boxed candidates. The OnceBox retains one winner and drops losing candidates; all successful callers borrow the winner. The initializer must not call  |
| AD-003-risk (spec/assurance/AD-003-bounded-integer-representation.md) | examined | - The larger inline representation or BigInt cache branch may leave CBMC memory use unchanged or make it worse. The original unchanged CG healthy and mutation controls determine qualification; no 16GiB improvement is claimed until they pass. Owner: IR-705 proof lane; revisit after implementation review and consumer evidence. - The currently identified target's pointer-atomic metadata does not establish all downstream targets or dependency feature behavior. Owner: implementation and platform review; build the declared supported target set before adoption. |

## Verdict

Reviewed concurrent cache initialization, stable borrows, losing-candidate cleanup, clone/drop and allocator failure behavior. The stated race, ownership and panic/resource limits cover the identified failure modes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
