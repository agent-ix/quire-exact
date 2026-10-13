---
id: SR-5140
title: code-review of IR-705 bounded integer implementation
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@6949844a942da9446bd76a6ae5dbf7c9a3eef3b9; Cargo.toml, Cargo.lock, src/accounting.rs, src/decimal_membership.rs, src/integer.rs, tests/ir705_bounded_integer.rs"
review_set: subset
relationships:
  - {type: references, target: "ix://agent-ix/quire-exact/FR-371"}
  - {type: references, target: "ix://agent-ix/quire-exact/TC-919"}
  - {type: references, target: "ix://agent-ix/quire-exact/AD-003"}
---

## Summary

Ticket: IR-705; PR #36, reviewed against `origin/main` at `d31520920fc3e9c4c7e4c50df5f3b6b791bdb7d6`. Combined code and Rust review. The implementation paths inspected preserve the mathematical `Integer` value, use safe `OnceBox` materialization, maintain the reviewed meter ordering/aggregation behavior, and keep inline magnitude iteration allocation-free. No production behavior defect was found. The new tests do not yet establish all required AC-1, AC-3, and AC-4 cases listed below. The genuine consumer Kani qualification for AC-6 remains explicitly pending; this review awards no proof credit. No builds or tests were run by the reviewer; reported CI receipts were not independently rerun. No applicable AssuranceProfile was found.

## Verdict

CONDITIONAL. Three medium test-evidence gaps remain. The source/Rust review found no implementation defect. Keep AC-6 open until the separate consumer owner records healthy TC027 plus both required substantive mutant outcomes on the published reviewed implementation under the unchanged bounds.

## Examined scope

- FR-371-AC-1 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:79`): Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve the exact value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX, -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range. MIN has magnitude bit length 128; its negation is positive 2^127 through genuine arbitrary-precision continuation. Compare fitting operations with independent checked i128 arithmetic; compare non-fitting results with independently authored decimal literals.
- FR-371-AC-2 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:80`): Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal, identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt access and cloning. A borrowed as_big reports the exact value throughout concurrent read-only accesses; all accesses to one retained materialization observe the same object. Clone/drop do not invalidate surviving permitted borrows or introduce a cache-state value.
- FR-371-AC-3 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:81`): Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native, native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete fields and admitted prefixes with the independent normative formulas. No representation transition manufactures a FunctionCall or other charge.
- FR-371-AC-4 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:82`): For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator observer around standalone Integer calls confirms no allocation on that native path, including clone after cache access.
- FR-371-AC-5 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:83`): Public signatures and all existing Integer methods remain available, except that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined by FR-374, replacing only their former plain &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync and current panic-unwind traits. Default and test-support checks and the required thumbv7em-none-eabi build retain no_std plus alloc behavior and local unsafe prohibition. Existing IEEE BigInt consumers observe the same exact values. If lazy materialization is selected, cache concurrency tests demonstrate one retained immutable winner, released losers and the disclosed first-access allocation behavior; a safe observer records temporary candidates without imposing a fictitious once-only bound.
- FR-371-AC-6 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:84`): Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and does not satisfy this criterion.
- TC-919 steps 1, 3, 4, 5, 6 (examined, `spec/test-cases/TC-919-bounded-integer-representation-refinement.md:17-22`): Inspected required independent arithmetic oracle, full metered schedule and denial cases, separate standalone/metered allocation observation, cache winner/loser observation, and actual consumer proof boundary.
- AD-003 cache contract (examined, `spec/assurance/AD-003-bounded-integer-representation.md:17-35`): Inspected safe lazy cache, immutable retained winner, loser release, and disclosed first-access allocation behavior.
- FR-362 charge schedule (context_only, `spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md`): Existing normative charge ordering/amounts remain the reference for AC-3.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Boundary and construction-route tests do not establish the full FR-371-AC-1 matrix. | `src/integer.rs:688-720`; `src/integer.rs:727-748` |
| FND-002 | medium | Metered schedule coverage omits overflow and arbitrary-size cases for Subtract, Multiply, and Negate, plus required boundary/denial variants. | `tests/ir705_bounded_integer.rs:78-161` |
| FND-003 | medium | No test separately observes allocation behavior of public metered calls as FR-371-AC-4 requires. | `tests/ir705_bounded_integer.rs:27-58`; `tests/ir705_bounded_integer.rs:78-161` |

### FND-001 — FR-371-AC-1 fixed-boundary/construction coverage is incomplete

The new AC-1 tagged test covers a few arithmetic overflows, i64 MIN squared, and magnitude of i128 MIN; the integration case covers signed-domain bounds. The cache/trait test exercises primitive, parse, from_big and arithmetic equality at 42, but no AC-1 test covers the specified large cancellation into native range, malformed/noncanonical rejection, reproducibly sampled cases, or fitting-result checks through an independent checked-i128 oracle across the construction routes. Those are explicit acceptance cases and include changed conversion/parse paths. Add the missing fixed cases and independent checks, or bind existing tests that actually establish them.

### FND-002 — FR-371-AC-3 metered operation matrix is incomplete

The new metered test checks native results for all four operations but only Add overflow; it exercises one integer-bit denial and one outside-domain result. It does not establish overflow/arbitrary-size schedules for Subtract, Multiply, and Negate, independently calculated exact/one-less bit and work boundaries for each named point, or injected denials at every named point and their full refusal prefixes. Add the missing metered cases with the independent FR-362 oracle; do not infer them from standalone arithmetic coverage.

### FND-003 — FR-371-AC-4 omits the separately observed metered facade

The allocation observer covers standalone Integer operations, and the limb-stream test covers expected limbs, zero allocations, and an empty cache. No allocation test observes a public metered call and distinguishes ordinary accounting/refusal allocations from BigInt materialization. Add a bounded metered observation for that distinction; it need not claim that the full call allocates nothing.

## Verdict details

The tests `cache_is_invisible_to_traits_clones_and_concurrent_borrows` and `once_box_race_retains_one_candidate_and_releases_losers` provide the agreed compositional evidence for public `as_big` value/stable shared borrow and safe losing-candidate release. `magnitude_limb_stream_does_not_materialize_small_cache` checks zero allocation and independently prepared limbs for zero, 32-bit crossing, and i128 MIN/MAX. Those previously disputed points are resolved in this head. Kani evidence remains external and pending, as the PR body states.

## Dispositions

Focused check receipt supplied by the lead for `eb80b7cf9b77f2b662c1833a5e9a1dbbfb1a9899`: default `cargo test --test ir705_bounded_integer --locked` passed 6/6; `cargo test --test ir705_bounded_integer --locked --features test-support` passed 5/5. Log: `<receipt log>`. The reviewer did not run Cargo.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | eb80b7cf9b77f2b662c1833a5e9a1dbbfb1a9899 |
| FND-002 | fixed | eb80b7cf9b77f2b662c1833a5e9a1dbbfb1a9899 |
| FND-003 | fixed | eb80b7cf9b77f2b662c1833a5e9a1dbbfb1a9899 |

### FND-001

The added AC-1 test covers primitive, parse, and `from_big` routes at signed boundaries; rejects malformed/noncanonical spellings; checks MIN/MAX arithmetic and large cancellation to 81; and uses deterministic sampled inputs with checked `i128` arithmetic for fitting Add/Subtract/Multiply/Negate. Focused default and test-support runs passed.

```rust
for noncanonical in ["", "-", "+1", "01", "-0", " 1", "1 "] {
    assert!(Integer::from_str(noncanonical).is_err(), "{noncanonical:?}");
}
```

### FND-002

The added AC-3 matrix covers all four arithmetic operations in native, native-overflow, and arbitrary-size cases. It asserts exact and one-less bit limits, exact and one-less work limits, all named injected denial points, complete counters, and admitted prefixes under `test-support`. Focused default and test-support runs passed.

```rust
for (index, denied_point) in points.into_iter().enumerate() {
    let mut meter = Meter::new(limits()).with_injected_denial(InjectedDenial {
        point: denied_point,
        occurrence: NonZeroU64::new(1).unwrap(),
    });
```

### FND-003

The added AC-4 test separately measures successful and refused public metered calls and compares their allocation totals with manually admitted accounting-only charges. It asserts the expected ordinary size-vector allocation rather than claiming the complete metered call is allocation-free. The default focused run passed; the test is intentionally cfg-disabled under `test-support`. The test-support matrix run passed independently.

```rust
assert_eq!(
    public_allocations.bytes_total,
    baseline_allocations.bytes_total
);
```

All three findings are fixed at the reviewed candidate head. The original finding rows and text above remain unchanged. AC-6 consumer Kani qualification is a separate external dependency tracked by SR-6419; this disposition grants no Kani or proof credit.
