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

This requirement selects the tagged representation in [AD-003](../assurance/AD-003-bounded-integer-representation.md) for IR-705. Existing behavior is defined by [FR-362](./FR-362-exact-scalar-arithmetic-and-atom-charges.md), allocation admission by [FR-361](./FR-361-admission-before-large-exact-work.md), and denial by [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md).

## Behavior

### Exact execution and observations

The native candidate range is the full signed `i128` range. This storage range is not a declared semantic domain. `Integer` remains arbitrary precision; `IntegerInterval` remains an independently declared inclusive mathematical domain. The representation SHALL distinguish `Small(i128, cache)` from `Large(BigInt)`. The abstraction function maps `Small(n, cache)` to mathematical integer n regardless of whether `cache` is empty or initialized, and maps `Large(value)` to the exact value of `value`. An initialized cache SHALL equal the abstraction of its `Small` tag. The tag and cache state SHALL contribute no separate semantic state.

When a native add, subtract, multiply or negate fits, the kernel SHALL retain its exact mathematical result.

Within Small, operands exactly representable as signed i64 SHALL be widened to i128 before Add/Subtract/Multiply/Negate in the range-proven native arm described by [AD-003](../assurance/AD-003-bounded-integer-representation.md). This arm SHALL return the exact Small result without an i128-overflow-to-BigInt continuation at that operation. Failure of the representability check SHALL preserve the existing checked full-i128 and arbitrary-precision continuation. This internal dispatch SHALL impose no semantic domain restriction or public API change.

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

For `Small`, `as_big` SHALL lazily initialize and return a borrow of a retained immutable BigInt image; for `Large`, it SHALL borrow the stored BigInt. The `Small` borrow SHALL remain valid for the borrow lifetime tied to `&self`. The cache SHALL use a safe no_std-compatible race-initialization mechanism with alloc support and no local unsafe code. Its initializer SHALL construct a candidate only from the immutable i128 payload. Concurrent initializers MAY construct multiple candidates; exactly one SHALL be retained, every losing candidate SHALL be dropped, and every successful caller SHALL observe the retained winner. Initialization SHALL not re-enter cache-dependent Integer methods.

For the bounded Decimal membership operation in [FR-372](./FR-372-fallible-bounded-decimal-membership.md), the kernel SHALL traverse an inline value's fixed-width unsigned-magnitude limbs without initializing or changing its private BigInt cache, including when that cache is empty. The Large representation borrows its retained magnitude limbs through the maintained dependency iterator. This stream policy does not change the public `as_big`, Hash or Debug materialization behavior above; the helper retains FR-372's one fallible scratch request.

If concurrent callers construct materialization candidates, then the kernel SHALL retain exactly one immutable winner and release every losing candidate.

All callers for one Integer return the retained winner. The initializer converts only the immutable native value; it does not re-enter cache-dependent Integer methods. An inline clone starts with an empty private cache even if its source cache was materialized. Candidate allocations can occur once per simultaneously initializing caller; this is not a one-allocation or caller-count bound. No background task, retry loop, stored historical certificate or global cache is introduced. A proposed implementation must report this resource behavior explicitly rather than claim fixed-capacity or once-only allocation.

An inline value cannot return a borrowed BigInt temporary. A core unsynchronized cell that removes Sync, a leaked Box, a new guard/Cow return type, or an eagerly constructed BigInt presented as an allocation-avoiding path does not establish this proposal's refinement and bounded-path obligations.

The kernel SHALL preserve its default and test-support configurations, no_std plus alloc build for `thumbv7em-none-eabi`, and local `forbid(unsafe_code)` policy.

### Conditional mechanism and unproved tractability

The selected cache behavior is safe race initialization of a retained immutable BigInt image, compatible with `no_std` plus `alloc`. The crate SHALL retain its local `forbid(unsafe_code)` policy; this requirement authorizes no copied dependency implementation or local unsafe code. Package and version selection belong to implementation dependency management, not this requirement. The required `thumbv7em-none-eabi` target reports pointer atomics in local rustc 1.98.1 target metadata. Implementation qualification SHALL build the supported target set and verify that dependency feature resolution introduces neither std nor a target exclusion.

The inline case adds a cache cell/pointer; an estimated 32-to-48-byte `Integer` size is a planning estimate only, not a measured size guarantee. Materializing an inline value can allocate the retained BigInt and, under concurrent initialization, additional candidate BigInts whose losing boxes are dropped. Clone of a `Small` SHALL copy the i128 and begin with an empty cache; clone of `Large` SHALL preserve the exact BigInt value. Clone, equality, ordering, hashing, Debug and Display SHALL not expose or depend on cache state. The precise allocation count under races is intentionally not fixed.

For two native-representable operands whose exact result fits i128, the proof-critical operations are primitive construction, Add/Subtract/Multiply/Negate, bits/sign/zero, to_u64, native comparison and inclusive interval membership. Their candidate path must not request BigInt construction or materialization. Existing Meter/Charge collection allocation and boxed domain refusal are outside this BigInt-only allocation claim and remain subject to their existing contracts.

The representation may increase Integer/Value size. Avoiding BigInt work on this path is a mechanism objective, not a demonstrated cause of the retained MemoryExhausted outcome or a prediction that CBMC will discard every large/cache branch. No lower memory number, altered field-sensitivity threshold, narrowed input, relaxed assertion, assumed arithmetic or Kani-only implementation is specified.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-371-AC-1 | Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve the exact value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX, -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range. MIN has magnitude bit length 128; its negation is positive 2^127 through genuine arbitrary-precision continuation. Compare fitting operations with independent checked i128 arithmetic; compare non-fitting results with independently authored decimal literals. Reject malformed/noncanonical spellings as before. No result wraps, saturates or becomes a representation-overflow refusal. | Test |
| FR-371-AC-2 | Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal, identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt access and cloning. A borrowed as_big reports the exact value throughout concurrent read-only accesses; all accesses to one retained materialization observe the same object. Clone/drop do not invalidate surviving permitted borrows or introduce a cache-state value. | Test |
| FR-371-AC-3 | Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native, native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete fields and admitted prefixes with the independent normative formulas. No representation transition manufactures a FunctionCall or other charge. | Test |
| FR-371-AC-4 | For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator observer around standalone Integer calls confirms no allocation on that native path, including clone after cache access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128 MIN/MAX, compare independently prepared expected limbs, observe no stream allocation and confirm that initially empty caches remain empty. Observation of public metered calls separately identifies ordinary accounting/refusal allocations and does not misreport the entire call as allocation-free. | Test |
| FR-371-AC-5 | Public signatures and all existing Integer methods remain available, except that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync and current panic-unwind traits. Default and test-support checks and the required thumbv7em-none-eabi build retain no_std plus alloc behavior and local unsafe prohibition. Existing IEEE BigInt consumers observe the same exact values. If lazy materialization is selected, cache concurrency tests demonstrate one retained immutable winner, released losers and the disclosed first-access allocation behavior; a safe observer records temporary candidates without imposing a fictitious once-only bound. | Test |
| FR-371-AC-6 | Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and does not satisfy this criterion. | Test |

## Dependencies

[TC-919](../test-cases/TC-919-bounded-integer-representation-refinement.md) owns the planned kernel refinement checks and identifies the separate consumer-owned qualification boundary. The consumer obligations are [CG TC-025](ix://agent-ix/quire-contract-codegen/TC-025) and [CG TC-027](ix://agent-ix/quire-contract-codegen/TC-027); the kernel does not copy their harnesses or replace their assertions.

## Status

PARTIALLY IMPLEMENTED / QUALIFICATION OPEN. The published kernel already implements Small/Large, lazy borrowed BigInt and native arithmetic, including later parser and charge-metadata refinements. The additional range-proven width dispatch described in [AD-003](../assurance/AD-003-bounded-integer-representation.md) is proposed and awaits implementation and independent review. This Markdown update adds no executable Trace binding or proof receipt. The unchanged original healthy artifact exhausted memory at 16GiB; a separate 40GiB diagnostic also exhausted memory, with no proof verdict. All AC-6 controls and the remaining acceptance evidence stay required. Separately record the genuine IR-917 post-change 16GiB healthy/helper-mutant qualification; its fit is currently unmeasured and cannot replace AC-6. No 16GiB Kani improvement is claimed unless the unchanged consumer controls pass.
