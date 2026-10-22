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

## Round 1 conformance checkpoint

PASS at b5cf31fa136e132f62d9176a432315780c524c42. Only two new traced public tests and initial review custody were added; every production/proof/manifest/lock byte remains identical to31ed8641. Complete new tests inspected, neither changes certification or evaluator semantics. SR-15241 findings are fixed; SR-15242 pending proof and later generated-client/profile evidence remain separate holds. No new base finding.

## Round 2 implementation/spec conformance

PASS for reviewed component contract at 30bfe47fda029e90a1f19d1bc40f7083498288f3 against kernel main 03c55fcca0d023e63ee8c5baf4cb8ded563b6856. Full FR-384/AD-003/TC-919 conformance checked: Multiply accepts every complete signed32 interval pair, retains exact signed64 four-corner image in i128, and refuses any wider endpoint before Meter even for zero product or narrow semantic bounds. Add/Sub/Neg/order retain full128 capacity. Optional semantic bounds remain128 and independent. Typed admission versus semantic refusals and original as_big/general Integer remain separate. The new independent SMT method is permitted machine-checked composition and preserves every logical containment edge through actual Rust bridges. Old full128 multiplication and C2 timeouts remain failures. No normative spec edit occurs in this code PR; EARS/integrity requirement-authoring sub-analyses are therefore inapplicable.

Independently inspected the actual SMT theorem, Int-to-Real embedding theorem, protocol and composition.json, all 20 query/source hashes and raw results, all 16 actual SAT model assignments with independent exact-integer recomputation, and source-bound Rust receipts. The healthy Real theorem negates below-all-four or above-all-four under exactly six interval/membership premises; it has no width, integrality or sign restriction. Its UNSAT plus unbounded Int-to-Real product/order/min/max preservation entails the integer corner theorem. Nonvacuity, all nine signs and signed32 extrema have actual SAT witnesses. Three substantive mutant countermodels expose omitted b*d upper corner, Add-for-Multiply and omitted lower membership. Contradictory membership/sign controls return UNSAT and are not treated as theorem evidence.

All four final-head Rust bridges pass their real properties and covers: payload roundtrip 0/235 failed, one cover; checked-wide-product 0/112 failed, three covers; exact four-wide-corners 0/121 failed, two covers; actual full-i128 contains/order 0/59 failed, two covers. Maximum bridge cgroup peak is 1,032,101,888 B under 16 GiB/swap0, memory events0, complete teardown. The exact native i128 payload maps to a mathematical signed integer; checked i128 multiplication Some establishes exact non-overflowing product, with signed64 roundtrip. Every actual admitted i128 payload roundtrips signed32. C1 binds each actual interval calculation to the full supported plan, and the wide corner bridge binds capacity to four exact products with independent semantic bounds. The contains bridge returns mathematical enclosure to the actual production filter. These edges close containment by reviewed machine-checked composition without a stub or assumed arithmetic identity; no C2 Kani PASS is claimed.

Thirteen successful Kani execution archives were checked for actual raw successful summaries/covers and complete resource/teardown records; production/reference/manifests/lock and each relevant harness/helper body are byte-equal to this candidate. Earlier heads remain stated accurately: A/D and Add/Sub at 0ef9099; B and Neg/Order at 3cd4f525; C1/classifier at 0243ec95. The four new bridges and SMT protocol executed at 30bfe47f. This is source-equivalence-supported composition, never a claim that the older runs executed at the final head. Full-i128 Add/Subtract/Negate/ordering retain their original universal domains and actual covers. Independent signed-limb Math A/B remain separate from checked-wide embedding bridges. General Integer/as_big, Meter, existing proof inputs and consumer domain boundaries remain unchanged.

Actual production arithmetic mutant bounded_product -> checked_add falsifies the independent payload theorem with original-domain witness (1073741824,0), and native generated playback has one failing mutant test versus one passing healthy test. Actual capacity mutant permits exactly left=2147483648 through a safe widened calculation and falsifies the universal interval policy; corrected playback has one failing mutant test versus one passing healthy test. The initial wrong selector ran zero tests and remains explicitly unqualified. All unrestricted-i128 multiplication, combined interval and C2 timeouts remain unsuccessful, with original sources/raw results/targets preserved. Their resource outcomes supply no theorem.

Focused ordinary evidence at source-identical production: default/support nine public tests each, reference and existing Integer/scalar/invariant controls, fmt/lint/no_std all pass. Original SR-15241 FND-001 derived exact/one-less cumulative boundaries and FND-002 ordering cancellation/injection tests survive with their assertions intact. Reviewer ran no Cargo/Kani/SMT/provisioning or aggregate job. The lead still owns the single final aggregate after fixes/custody. No dependency, manifest, lock, CI gate, unsafe or cfg production-body replacement was introduced.

Original base PASS is not program acceptance. Follow SR-15241 FND-003 for obsolete active diagnostic entry-point cleanup; SR-15242 retains owned downstream and inherited assurance debt.

```yaml
scope:
- id: FR-384-AC-1
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Admission accepts every valid Add/Subtract/Negate domain whose complete
    interval image fits signed i128, retains full-i128 ordering, and accepts every
    valid Multiply interval pair whose four operand endpoints lie in signed32. It
    rejects any out-of-signed32 Multiply endpoint as a typed operation-capacity refusal
    before evaluator charges, including `[2147483648, 2147483648] * [0, 0]` and `[-2147483649,
    0] * [0, 0]`; payload/product zero or a semantic result restriction cannot conceal
    the unsupported interval. In-capacity Multiply image endpoints equal the exact
    four-corner signed64 minimum/maxim
- id: FR-384-AC-2
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: A machine-checked refinement against an independently expressed mathematical
    abstraction of the actual BigInt semantics covers every admitted native payload/operation
    under the operator-specific capacity policy and its abstraction to mathematical
    Integer/BigInt, including all signed32 multiplication pairs with widened signed64
    results and full admitted signed-i128 Add/Subtract/Negate/ordering capacities.
    Admission proof covers all valid in-capacity interval pairs and rejects out-of-signed32
    Multiply endpoints independently of actual payload and semantic result bounds.
    Independent BigInt differ
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

## Round 3 final narrow conformance

PASS at d82ca775e0954c6ceb66d042539a9cc832722abd. Complete 30bfe47f -> d82ca775 diff inspected: exact C2 helper/nine-wrapper retirement and two truthful qualification comments; all retained function/helper bodies and every other tracked file unchanged. Kernel policy, actual production semantics, full operator-specific theorem population and complete SMT/Rust composition retain their reviewed identity. All 86 unsuccessful archive file hashes independently checked. Ordinary fmt-check passed at actual16GiB/swap0 with peak33,529,856B/events0/teardown; no verifier/aggregate rerun performed by reviewer. All code findings fixed; no new conformance issue. Lead final aggregate and downstream owned qualification remain distinct.
