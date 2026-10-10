---
id: FR-371
title: "Bounded integer execution refines the mathematical representation"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-362
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-361
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-358
    type: depends_on
---
# FR-371: Bounded integer execution refines the mathematical representation

## Description

Where the kernel uses a bounded native representation for mathematical integers, the kernel SHALL preserve the exact value and public behavior of `Integer` across native and arbitrary-precision execution.

This is a proposed refinement contract for IR-705. It does not select or authorize an implementation, dependency, solver setting or proof substitute. Existing behavior is defined by [FR-362](./FR-362-exact-scalar-arithmetic-and-atom-charges.md), allocation admission by [FR-361](./FR-361-admission-before-large-exact-work.md), and denial by [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md).

## Behavior

### Exact execution and observations

The native candidate range is the full signed `i128` range. This storage range is not a declared semantic domain. `Integer` remains arbitrary precision; `IntegerInterval` remains an independently declared inclusive mathematical domain. The abstraction of native storage `n` is the mathematical integer n; the abstraction of arbitrary-precision storage is its full mathematical integer. Any materialization cache contains exactly that abstraction and contributes no separate semantic state.

When a checked native add, subtract, multiply or negate fits, the kernel SHALL retain its exact mathematical result.

When the exact result does not fit the native range, the kernel SHALL obtain the full result through genuine arbitrary-precision execution.

The kernel SHALL preserve exact values supplied by `from_big`, primitive constructors and canonical parsing.

The kernel SHALL preserve zero, sign, magnitude bits, conversions, every existing public integer operation, numeric equality and order, canonical Display and the existing mathematical Debug value independently of storage and materialization state.

When two Integers have the same mathematical value, the kernel SHALL produce the same hash input behavior as their existing BigInt representation.

Hash values are not a cross-version or cross-target persistence format. The requirement above preserves the current BigInt hashing behavior for the same hasher and target; it does not introduce a new serialized hash identity.

When evaluating public metered Add/Subtract/Multiply/Negate, the kernel SHALL preserve the charge order, exact request amounts, high-water sizes, cumulative work/results, domain refusal and incomplete prefix defined by FR-362 and FR-358.

The kernel SHALL exclude representation changes and materialization from semantic evaluator charges.

Machine overflow is neither a domain refusal nor an accounting denial. The result must not wrap, saturate or narrow. Native magnitude calculations must handle `i128::MIN` without signed absolute-value overflow. Existing arbitrary-size charge and denial amounts remain exact.

### Borrowed BigInt and resource behavior

The kernel SHALL retain the public signature `Integer::as_big(&self) -> &BigInt` and return the exact mathematical value for the lifetime of that borrow.

When a representation owns a materialized BigInt, the kernel SHALL retain that immutable object until all borrows permitted by the `Integer` lifetime have ended.

The kernel SHALL preserve `Integer`'s existing Send/Sync, clone, default, drop and panic-unwind trait behavior.

The proposed lazy-materialization option permits the first `as_big` on an inline value to allocate its BigInt image; the current implementation instead borrows an already constructed BigInt. Hash, Debug and other BigInt-required operations may also trigger materialization. Such initialization is implementation work, not an added semantic charge. It inherits the supported allocator's existing allocation-failure behavior; this proposal introduces no new typed allocation-refusal policy or recovery guarantee.

If concurrent callers construct materialization candidates, then the kernel SHALL retain exactly one immutable winner and release every losing candidate.

All callers for one Integer return the retained winner. The initializer converts only the immutable native value; it does not re-enter cache-dependent Integer methods. An inline clone starts with an empty private cache even if its source cache was materialized. Candidate allocations can occur once per simultaneously initializing caller; this is not a one-allocation or caller-count bound. No background task, retry loop, stored historical certificate or global cache is introduced. A proposed implementation must report this resource behavior explicitly rather than claim fixed-capacity or once-only allocation.

An inline value cannot return a borrowed BigInt temporary. A core unsynchronized cell that removes Sync, a leaked Box, a new guard/Cow return type, or an eagerly constructed BigInt presented as an allocation-avoiding path does not establish this proposal's refinement and bounded-path obligations.

The kernel SHALL preserve its default and test-support configurations, no_std plus alloc build for `thumbv7em-none-eabi`, and local `forbid(unsafe_code)` policy.

### Conditional mechanism and unproved tractability

One design candidate is an inline i128 with a private immutable BigInt cache plus a large BigInt case. A safe dependency candidate is `once_cell::race::OnceBox<BigInt>` with default features disabled and alloc enabled. Its public API supplies stable borrowed initialization without std and permits concurrent candidate construction. Neither dependency adoption nor this mechanism is settled by this proposed contract. No dependency implementation may be copied into this repository.

The current required no_std target reports pointer atomics in compiler target metadata. That lookup is not a target build or proof of every supported target. Before adopting an atomic cache, implementation review must establish the supported target set and build/dependency compatibility without silently excluding targets or adding std/local unsafe.

For two native-representable operands whose exact result fits i128, the proof-critical operations are primitive construction, Add/Subtract/Multiply/Negate, bits/sign/zero, to_u64, native comparison and inclusive interval membership. Their candidate path must not request BigInt construction or materialization. Existing Meter/Charge collection allocation and boxed domain refusal are outside this BigInt-only allocation claim and remain subject to their existing contracts.

The representation may increase Integer/Value size. Avoiding BigInt work on this path is a mechanism objective, not a demonstrated cause of the retained MemoryExhausted outcome or a prediction that CBMC will discard every large/cache branch. No lower memory number, altered field-sensitivity threshold, narrowed input, relaxed assertion, assumed arithmetic or Kani-only implementation is specified.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-371-AC-1 | Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve the exact value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX, -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range. MIN has magnitude bit length 128; its negation is positive 2^127 through genuine arbitrary-precision continuation. Compare fitting operations with independent checked i128 arithmetic; compare non-fitting results with independently authored decimal literals. Reject malformed/noncanonical spellings as before. No result wraps, saturates or becomes a representation-overflow refusal. | Test |
| FR-371-AC-2 | Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal, identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt access and cloning. A borrowed as_big reports the exact value throughout concurrent read-only accesses; all accesses to one retained materialization observe the same object. Clone/drop do not invalidate surviving permitted borrows or introduce a cache-state value. | Test |
| FR-371-AC-3 | Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native, native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete fields and admitted prefixes with the independent normative formulas. No representation transition manufactures a FunctionCall or other charge. | Test |
| FR-371-AC-4 | For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator observer around standalone Integer calls confirms no allocation on that native path, including clone after cache access. Observation of public metered calls separately identifies ordinary accounting/refusal allocations and does not misreport the entire call as allocation-free. | Test |
| FR-371-AC-5 | Public signatures and all existing Integer methods remain available. Trait checks preserve Send/Sync and current panic-unwind traits. Default and test-support checks and the required thumbv7em-none-eabi build retain no_std plus alloc behavior and local unsafe prohibition. Existing IEEE BigInt consumers observe the same exact values. If lazy materialization is selected, cache concurrency tests demonstrate one retained immutable winner, released losers and the disclosed first-access allocation behavior; a safe observer records temporary candidates without imposing a fictitious once-only bound. | Test |
| FR-371-AC-6 | Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and does not satisfy this criterion. | Test |

## Dependencies

[TC-919](../test-cases/TC-919-bounded-integer-representation-refinement.md) owns the planned kernel refinement checks and identifies the separate consumer-owned qualification boundary. The consumer obligations are [CG TC-025](ix://agent-ix/quire-contract-codegen/TC-025) and [CG TC-027](ix://agent-ix/quire-contract-codegen/TC-027); the kernel does not copy their harnesses or replace their assertions.

## Status

PROPOSED / UNIMPLEMENTED / UNRUN. All six criteria are prospective. No new executable Trace bindings, proof receipt or memory-improvement claim accompanies this Markdown proposal. Dependency choice and materialization mechanics remain subject to independent architecture/spec review. The original healthy exact-kernel qualification had an unavailable measured peak; an older distinct control's peak and a separate compiler-only panic cannot establish the cause or success of this proposal.
