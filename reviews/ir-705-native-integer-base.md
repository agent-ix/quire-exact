---
id: SR-15243
title: IR-705 native integer base
type: SpecReview
analysis: base
scope: agent-ix/quire-exact; PR 78; src/native_integer.rs; src/native_integer/reference.rs;
  src/native_integer/kani_proofs.rs; src/integer.rs; src/lib.rs; src/outcome.rs; src/numeric.rs;
  src/accounting.rs; tests/ir705_native_integer.rs; tests/checked_invariant.rs; Cargo.toml;
  Makefile; CLAUDE.md; AGENTS.md
review_set: subset
---

## Summary

Reviewed exact code head 31ed8641522394746bbe352b64568724f9d01399 against merged specification a4e1fade. No Cargo, formatter, backend, provisioning or aggregate command was run by this reviewer because the lead reserved the sole heavy slot for the author proof. Reported focused checks are author/lead evidence, not independent executions here. Final root aggregate remains required.

## Verdict

PASS — Implemented capability and evaluator shape conforms to the reviewed contract; proof and consumer acceptance remain pending and are recorded in SR-15242.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

src/native_integer.rs; src/native_integer/reference.rs; src/native_integer/kani_proofs.rs; src/integer.rs; src/lib.rs; src/outcome.rs; src/numeric.rs; src/accounting.rs; tests/ir705_native_integer.rs; tests/checked_invariant.rs; Cargo.toml; Makefile; CLAUDE.md; AGENTS.md

The abstraction maps an i128 two's-complement payload to exact sign and unsigned magnitude, including MIN = -2^127. Signed addition/subtraction compares magnitudes; equal-sign sum is at most 2^128 and explicitly retains the carry. Negation changes sign with zero normalization. Multiplication expands (a0+2^64*a1)(b0+2^64*b1), retains both low-word carries, and forms the exact high word; each magnitude is at most 2^127 so the product is at most 2^254 and the high-word accumulation cannot overflow u128. The native-fit predicate includes -2^127 and excludes +2^127. Signed ordering reverses magnitude order only for negative pairs. This is an independently expressed mathematical abstraction, not production checked_i128 operations reused as the expected result. Host BigInt binding tests are finite, explicitly not the universal machine proof. All five full-width harnesses must finish, with covers; arithmetic harnesses call actual production calculate(), and ordering calls the actual public evaluator/real Meter. No assumption narrows the six symbolic i128 values or their sorted interval population. Numerical proofs do not establish arbitrary-budget Meter refinement.

Opaque fields and closed operation/operand enums prevent public certificate forging; input membership is checked before charging. Capacity and semantic result restriction remain separate. Native success contains only Copy i128/range/plan values. General Integer/as_big is unchanged except private crate visibility for endpoint ingress. Refusal target allocation occurs only on real domain refusal after arithmetic charge. No cfg replacement evaluator, unsafe, new dependency, lock or CI-lane change was found. Native schedule compares to normative formulas and existing general evaluator, with actual ten-counter, incomplete payload and support-log observations. Scope tags below are inspected bindings, not proof receipts.

## Examined scope

```yaml
scope:
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate
    preserve the exact value for fixed boundaries and reproducibly sampled inputs.
    Fixed cases include i128 MIN/MAX, -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated
    MIN and large cancellation into the native range. MIN has magnitude bit length
    128; its negation is positive 2^127 through genuine arbitrary-precision continuation.
    Compare fitting operations with independent checked i128 arithmetic; compare non-fitting
    results with independently authored decimal literals. Reject malformed/noncanonical
    spellings as be
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and
    arithmetic routes remain equal, identically ordered, identically hashed with the
    same hasher and identically formatted before and after BigInt access and cloning.
    A borrowed as_big reports the exact value throughout concurrent read-only accesses;
    all accesses to one retained materialization observe the same object. Clone/drop
    do not invalidate surviving permitted borrows or introduce a cache-state value.
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362
    schedule and counters on native, native-overflow and arbitrary-size cases. Test
    sufficient limits, independently calculated exact/one-less bit and work boundaries,
    denial injected at every named point, and outside-domain results. Compare all
    completed/refused/incomplete fields and admitted prefixes with the independent
    normative formulas. No representation transition manufactures a FunctionCall or
    other charge.
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For primitive inputs and result fitting i128, inspection identifies native
    Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt
    construction/materialization. A safe external allocator observer around standalone
    Integer calls confirms no allocation on that native path, including clone after
    cache access. Inline magnitude-stream controls cover zero, 32-bit limb crossings
    and i128 MIN/MAX, compare independently prepared expected limbs, observe no stream
    allocation and confirm that initially empty caches remain empty. Observation of
    public metered calls separately iden
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'Public signatures and all existing Integer methods remain available, except
    that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating
    and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined
    by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their
    former plain &Integer parameters. Their existing nonzero results and infallible
    return types remain preserved; every other public signature and Integer method
    retains its existing preservation guarantee. Trait checks preserve Send/Sync and
    current panic-unwind traits. Default and '
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic
    mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second
    native ceiling, 16GiB memory ceiling, completion cover and counterexample checks.
    Healthy verification and both substantive falsifications are required. Existing
    all-four-operation, accounting/denial and refinement obligations remain required.
    A compiler error, MemoryExhausted, timeout, missing mutation execution or missing
    teardown is inconclusive and does not satisfy this criterion.
- id: FR-371-AC-7
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Private mechanism checks distinguish Small64 and Small128 construction,
    normalization and clone routes according to AD-003 without exposing a public tag
    API. Source inspection identifies storage matching before native-value extraction
    for Small64 Add/Subtract/Multiply/Negate, widening before ordinary arithmetic,
    Small128 result construction and no BigInt continuation inside those arms. Check
    all other inline pair classes, failed numeric-width guards, genuine i128 overflow
    and Large cancellation; a small Small128 result remains Small128 on chaining.
    Confirm both inline MIN magnitude streams and
- id: FR-384-AC-1
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Admission accepts valid finite Add/Subtract/Multiply/Negate domains and
    rejects reversed ranges, nonrepresentable endpoints and overflowing intermediate
    result intervals with distinct typed reasons. Boundaries cover native MIN/MAX,
    zero, -1, singleton domains and multiplication sign combinations. A narrowing
    final domain does not conceal an unrepresentable intermediate. Public clients
    cannot forge or widen certificates.
- id: FR-384-AC-2
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: A machine-checked refinement against an independently expressed mathematical
    abstraction of the actual BigInt semantics covers every admitted native payload/operation
    and its abstraction to mathematical Integer/BigInt. Independent BigInt differential
    tests exercise extrema, cancellation, products and negation plus reproducibly
    sampled accepted domains; deliberate arithmetic and admission-bound mutants falsify
    the relevant property. Evidence states exactly which obligation each method establishes.
- id: FR-384-AC-3
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Fresh and cumulative real Meter comparisons against the normative formulas
    and general exact calls agree on completed values, domain refusal, cancellation,
    every named injected denial, all counters, admitted prefix and complete incomplete
    fields at sufficient, exact and one-less limits. Every operation and Boolean/comparison
    consumer used by the genuine profiles is exercised; arithmetic equivalence alone
    does not satisfy accounting refinement.
- id: FR-384-AC-4
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Source inspection and ordinary generated-client execution demonstrate native
    carriers persist through a multi-operation expression without successful-path
    BigInt/cache/general Integer construction. Input/output conversions are identified
    separately. Default and test-support checks, required no_std target and existing
    borrowed as_big regression checks pass without local unsafe or verification-only
    implementation.
- id: FR-384-AC-5
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Genuine CG TC-027 healthy, TC-027 bound mutant, TC-025 arithmetic mutant,
    IR-917 clause implication and IR-948 V2 contract profiles execute the reviewed
    production admitted API over their original inputs, domains and assertions. Healthy
    cases verify with their final completion covers at 16 GiB; each required substantive
    mutant falsifies with original-domain native playback. The IR-948 impossible-precondition
    control must complete as CoverUnsatisfied at the same ceiling, not resource exhaustion
    or a vacuous Verified. Unwind3, Cadical and existing 600-second ceilings remain
    unchanged where speci
```
