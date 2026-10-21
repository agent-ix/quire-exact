---
id: SR-14781
title: IR-705 native-width spec-review/base review
type: SpecReview
analysis: base
scope: agent-ix/quire-exact; spec/assurance/AD-003-bounded-integer-representation.md,
  spec/functional/FR-371-bounded-integer-representation-refinement.md, spec/test-cases/TC-919-bounded-integer-representation-refinement.md
review_set: subset
---

## Summary

Frozen candidate 001d64feb149973151778b9339ac1f9bf0a88220, published base 765a95914bfc9acc5c01dd482630535113c69658. Ticket IR-705; PR agent-ix/quire-exact#69. Exactly three Markdown files changed; no Cargo, Rust, executable test, dependency or gate change. No Cargo/native/Kani/metadata/fmt job or aggregate gate was run. No applicable AssuranceProfile was found. Every listed examined unit was read, including the full text when a verbatim excerpt is truncated to 600 characters. Context-only CG criteria were read from the available repository specification; this is not inspection of the missing IR-917 candidate or log. No test body or test-to-AC binding was reviewed, so bindings is deliberately empty.

## Verdict

**FAIL** — Two recorded-revision defects require removal before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | New normative architecture status names exact kernel and CG source revisions; remove recorded revisions from the ADR and retain measured-run identities in the review or tracker evidence. | spec/assurance/AD-003-bounded-integer-representation.md:72 |
| FND-002 | high | New normative requirement status records an exact kernel revision; remove this recorded revision from the FR and retain the implementation/qualification distinction without a source pin. | spec/functional/FR-371-bounded-integer-representation-refinement.md:94 |

## Analysis

All six FR-371 AC rows are byte-identical to base. Genuine consumer controls, original ceilings and negative Decimal/quantity obligations remain mandatory. The computed matrix still marks FR-371-AC-6 untagged; this is the openly pending original proof qualification, not a new claim of coverage. Two newly introduced normative recorded-revision uses violate the installed base checklist. Remove the revisions, preserving their measured evidence in the tracker/review and keeping truthful partial implementation status.

## Scope

```yaml
scope:
- id: AD-003:L16
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: This decision covers the private storage representation of `quire-exact::Integer`, its exact
    refinement to mathematical integers, and the stable borrowed `as_big` interface. It preserves the
    public `Integer` API, metered arithmetic schedule in FR-362, allocation admission and refusal contracts,
    no_std plus alloc support, and the crate's local unsafe prohibition. It does not alter Decimal semantics
    or the separate CG consumer proof harnesses.
- id: AD-003:L20
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: The following views describe the representation, borrow, and concurrency boundaries relevant
    to this decision.
- id: AD-003:L24
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: 'The selected representation has two tags: `Small(i128, cache)` and `Large(BigInt)`. The abstraction
    function maps every `Small(n, cache)` to the mathematical integer `n`, independent of cache state;
    it maps `Large(b)` to the exact value represented by `b`. A populated Small cache must equal the Small
    payload''s mathematical value. Arithmetic uses range-proven widened native operations or checked full-width
    native operations, retaining genuine BigInt continuation when the exact result does not fit i128.
    In particular, negating `i128::MIN` produces positive 2^127 through BigInt continuation.'
- id: AD-003:L28
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: Within the existing Small tag, the selected refinement first checks exact signed-i64 representability
    of both operands (the one operand for Negate). Successful conversions are widened to i128 **before**
    Add/Subtract/Multiply/Negate; their exact results fit i128, so this arm uses ordinary native arithmetic
    and returns Small without an i128-overflow-to-BigInt continuation at that operation. In particular,
    this lowers the actual `Integer::add` small path, rather than merely restating its existing checked-i128
    implementation. No third representation tag or public constructor is introduced.
- id: AD-003:L32
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Add | -2^64 | 2^64 - 2 |'
- id: AD-003:L33
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Subtract | -2^64 + 1 | 2^64 - 1 |'
- id: AD-003:L34
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Multiply | -2^126 + 2^63 | 2^126 |'
- id: AD-003:L35
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Negate | -2^63 + 1 | 2^63 |'
- id: AD-003:L37
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: Every bound lies within signed i128. The guard is numeric representability, not an IntegerInterval,
    a caller certificate, or a semantic domain restriction. Conversion failure falls through to the existing
    checked-i128 operation and genuine BigInt continuation; Large operands retain the general exact route
    and normalization. Values immediately outside i64, full-i128 fitting results, real i128 overflow,
    and Large cancellation remain admitted. No truncating cast, saturation, wrapping, assumption, Kani-only
    route, or altered proof input supplies the range proof.
- id: AD-003:L39
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: This production dispatch preserves the two tags, full-i128 Small payload, lazy cache, public
    observations, metering and refusal contracts. It does not promise that CBMC discards every conversion-failure,
    cache, or general branch. All-four-operation boundary and continuation tests plus the unchanged consumer
    qualification are required before claiming success.
- id: AD-003:L43
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: The stable public `as_big(&self) -> &BigInt` contract requires storage that outlives each returned
    borrow. For Small, the first BigInt-required observation initializes a private safe race-initialization
    cache from the immutable i128 payload; subsequent observations borrow the retained immutable winner.
    For Large, observations borrow its stored BigInt. Concurrent first access may construct multiple boxed
    candidates. The cache retains one winner and drops losing candidates; all successful callers borrow
    the winner. The initializer must not call cache-dependent methods on the same Integer.
- id: AD-003:L45
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: Cloning Small copies the i128 and starts with a fresh empty cache. Cloning Large preserves
    the exact BigInt. Equality, ordering, hashing, Debug and Display are functions of mathematical value
    only; cache initialization cannot change their results. Cache materialization and discarded candidates
    are implementation resource costs and add no semantic meter charge.
- id: AD-003:L51
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Integer value representation | Tagged Small(i128, cache) and Large(BigInt), with a cache-independent
    abstraction function. | BigInt-only storage preserves simple borrowing but constructs heap-backed
    values on the bounded path. A naked i128 cannot represent overflow results. Eager dual storage defeats
    the bounded allocation objective. |'
- id: AD-003:L52
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Native arithmetic dispatch | Exact i64 representability followed by i128 widening for all
    four elementary operations; existing checked-i128/general continuation otherwise. | Keeping checked-i128
    overflow selection even for the representable subrange leaves the reported symbolic promotion path.
    Narrowing Small storage or deleting general continuation changes the contract; an extra tag increases
    representation invariants without evidence it is needed. |'
- id: AD-003:L53
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Stable borrowed BigInt for Small | Safe no_std-compatible race initialization with alloc
    support and stable retained borrowing. | A core unsynchronized cell would remove Sync; a std-only
    primitive violates no_std; leaked boxes, local unsafe, or a changed guard/Cow API violate the constraints
    or public borrow contract. Any selected implementation must preserve the race winner/loser lifecycle
    and stable borrow contract. |'
- id: AD-003:L54
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Concurrent initialization | Allow candidate construction by racing callers, retain one immutable
    winner, and drop every losing candidate. | A once-only candidate allocation promise is not supportable
    with this race API. The design favors safe Sync and stable borrows while explicitly accepting temporary
    concurrent allocation overhead. |'
- id: AD-003:L55
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Numeric observations and metering | Preserve mathematical value, public observation, existing
    trait behavior, and FR-362/FR-358 charge/refusal order exactly. | Making representation state observable
    or adding charges for cache work would change the contract. |'
- id: AD-003:L56
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Qualification claim | Keep the native-width refinement and consumer qualification open and
    require the unchanged CG TC-025 arithmetic mutant, TC-027 healthy case, and TC-027 bound mutant at
    their existing 16GiB/600-second/unwind3/Cadical settings. | Unit tests or a changed/smaller consumer
    harness cannot establish the intended end-to-end Kani result. No memory improvement is asserted before
    those controls pass. |'
- id: AD-003:L60
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: The selected architecture requires a safe race-initialization cache that supplies stable lazy
    references without std and supports concurrent initialization using alloc. Package and version selection,
    as well as concrete feature resolution, belong to implementation dependency management. Local rustc
    1.98.1 target metadata reports pointer atomics for `thumbv7em-none-eabi`; that metadata is not a build
    result or proof of all supported targets. Implementation qualification must confirm supported target
    compatibility and that dependency feature resolution does not introduce std or weaken local `for
- id: AD-003:L62
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: The cache adds a cell/pointer to Small. A change from an estimated 32-byte to 48-byte Integer
    is only a planning estimate, not a measured layout commitment. First BigInt materialization can allocate
    the retained value; concurrent calls can allocate additional candidates, with losers freed after initialization.
    TC-919 must record actual size and allocation behavior without treating either estimate or a fixed
    allocation count as an oracle.
- id: AD-003:L66
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '- The larger inline representation or BigInt cache branch may leave CBMC memory use unchanged
    or make it worse. The original unchanged CG healthy and mutation controls determine qualification;
    no 16GiB improvement is claimed until they pass. Owner: IR-705 proof lane; revisit after implementation
    review and consumer evidence.'
- id: AD-003:L67
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '- The currently identified target''s pointer-atomic metadata does not establish all downstream
    targets or dependency feature behavior. Owner: implementation and platform review; build the declared
    supported target set before adoption.'
- id: AD-003:L68
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '- Racing materialization can temporarily allocate multiple BigInts. Owner: implementation;
    preserve losing-candidate deallocation observations and disclose actual costs in TC-919 evidence.'
- id: AD-003:L72
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: The two-tag Small/Large representation, safe lazy borrowed BigInt cache and native arithmetic
    already exist in published kernel `765a95914bfc9acc5c01dd482630535113c69658`; merged parser and charge-metadata
    refinements do not implement the width dispatch above. This document remains proposed for that additional
    refinement and its outstanding qualification, not because the entire representation is unimplemented.
    Kernel unit/native evidence does not establish consumer proof acceptance.
- id: AD-003:L74
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: 'The unchanged original healthy artifact still has no proof verdict: the measured 16GiB run
    on that kernel exhausted memory, and the separate authorized 40GiB diagnostic also exhausted memory.
    Neither changes the original AC-6 ceiling or discharges a control. IR-917 additionally reports MemoryExhausted
    for its V2 clause implication proof at CG `a2f61a2e`, with four metered runtime helpers and symbolic
    Integer-add promotion identified by its owner as costs. Its supplied source/log are unavailable locally,
    so that attribution and peak are reported evidence, not independently verified measurements'
- id: AD-003:L76
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: After adoption, the IR-705 proof owner records separately whether the genuine IR-917 V2 fixture
    fits 16GiB, including exact source/cohort/selector, healthy assertion and completion-cover result,
    helper-mutant substantive falsification and playback, peak-memory provenance, timeout and teardown.
    Preserve its CG AC-41 and AC-119 obligations; do not substitute an older synthetic fixture or conflate
    its controls with original TC-025/TC-027. IR-917 post-change fit is currently unmeasured. If its exact
    candidate is unavailable, record that input blocker instead of a result. This additional consumer
    e
- id: FR-371:L17
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Where the kernel uses a bounded native representation for mathematical integers, the kernel
    SHALL preserve the exact value and public behavior of `Integer` across native and arbitrary-precision
    execution.
- id: FR-371:L19
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: This requirement selects the tagged representation in [AD-003](../assurance/AD-003-bounded-integer-representation.md)
    for IR-705. Existing behavior is defined by [FR-362](./FR-362-exact-scalar-arithmetic-and-atom-charges.md),
    allocation admission by [FR-361](./FR-361-admission-before-large-exact-work.md), and denial by [FR-358](./FR-358-meter-denial-and-bounded-diagnostic-log.md).
- id: FR-371:L25
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The native candidate range is the full signed `i128` range. This storage range is not a declared
    semantic domain. `Integer` remains arbitrary precision; `IntegerInterval` remains an independently
    declared inclusive mathematical domain. The representation SHALL distinguish `Small(i128, cache)`
    from `Large(BigInt)`. The abstraction function maps `Small(n, cache)` to mathematical integer n regardless
    of whether `cache` is empty or initialized, and maps `Large(value)` to the exact value of `value`.
    An initialized cache SHALL equal the abstraction of its `Small` tag. The tag and cache state SHALL
    c
- id: FR-371:L27
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When a native add, subtract, multiply or negate fits, the kernel SHALL retain its exact mathematical
    result.
- id: FR-371:L29
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Within Small, operands exactly representable as signed i64 SHALL be widened to i128 before
    Add/Subtract/Multiply/Negate in the range-proven native arm described by [AD-003](../assurance/AD-003-bounded-integer-representation.md).
    This arm SHALL return the exact Small result without an i128-overflow-to-BigInt continuation at that
    operation. Failure of the representability check SHALL preserve the existing checked full-i128 and
    arbitrary-precision continuation. This internal dispatch SHALL impose no semantic domain restriction
    or public API change.
- id: FR-371:L31
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When the exact result does not fit the native range, the kernel SHALL obtain the full result
    through genuine arbitrary-precision execution.
- id: FR-371:L33
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL preserve exact values supplied by `from_big`, primitive constructors and canonical
    parsing.
- id: FR-371:L35
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL preserve zero, sign, magnitude bits, conversions, every existing public integer
    operation, numeric equality and order, canonical Display and the existing mathematical Debug value
    independently of storage and materialization state.
- id: FR-371:L37
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When two Integers have the same mathematical value, the kernel SHALL produce the same hash
    input behavior as their existing BigInt representation.
- id: FR-371:L39
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Hash values are not a cross-version or cross-target persistence format. The requirement above
    preserves the current BigInt hashing behavior for the same hasher and target; it does not introduce
    a new serialized hash identity.
- id: FR-371:L41
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When evaluating public metered Add/Subtract/Multiply/Negate, the kernel SHALL preserve the
    charge order, exact request amounts, high-water sizes, cumulative work/results, domain refusal and
    incomplete prefix defined by FR-362 and FR-358.
- id: FR-371:L43
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL exclude representation changes and materialization from semantic evaluator
    charges.
- id: FR-371:L45
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Machine overflow is neither a domain refusal nor an accounting denial. The result must not
    wrap, saturate or narrow. Native magnitude calculations must handle `i128::MIN` without signed absolute-value
    overflow. Existing arbitrary-size charge and denial amounts remain exact.
- id: FR-371:L49
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL retain the public signature `Integer::as_big(&self) -> &BigInt` and return
    the exact mathematical value for the lifetime of that borrow.
- id: FR-371:L51
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When a representation owns a materialized BigInt, the kernel SHALL retain that immutable object
    until all borrows permitted by the `Integer` lifetime have ended.
- id: FR-371:L53
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL preserve `Integer`'s existing Send/Sync, clone, default, drop and panic-unwind
    trait behavior.
- id: FR-371:L55
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For `Small`, `as_big` SHALL lazily initialize and return a borrow of a retained immutable BigInt
    image; for `Large`, it SHALL borrow the stored BigInt. The `Small` borrow SHALL remain valid for the
    borrow lifetime tied to `&self`. The cache SHALL use a safe no_std-compatible race-initialization
    mechanism with alloc support and no local unsafe code. Its initializer SHALL construct a candidate
    only from the immutable i128 payload. Concurrent initializers MAY construct multiple candidates; exactly
    one SHALL be retained, every losing candidate SHALL be dropped, and every successful caller SHALL
    ob
- id: FR-371:L57
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For the bounded Decimal membership operation in [FR-372](./FR-372-fallible-bounded-decimal-membership.md),
    the kernel SHALL traverse an inline value's fixed-width unsigned-magnitude limbs without initializing
    or changing its private BigInt cache, including when that cache is empty. The Large representation
    borrows its retained magnitude limbs through the maintained dependency iterator. This stream policy
    does not change the public `as_big`, Hash or Debug materialization behavior above; the helper retains
    FR-372's one fallible scratch request.
- id: FR-371:L59
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: If concurrent callers construct materialization candidates, then the kernel SHALL retain exactly
    one immutable winner and release every losing candidate.
- id: FR-371:L61
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: All callers for one Integer return the retained winner. The initializer converts only the immutable
    native value; it does not re-enter cache-dependent Integer methods. An inline clone starts with an
    empty private cache even if its source cache was materialized. Candidate allocations can occur once
    per simultaneously initializing caller; this is not a one-allocation or caller-count bound. No background
    task, retry loop, stored historical certificate or global cache is introduced. A proposed implementation
    must report this resource behavior explicitly rather than claim fixed-capacity or once-onl
- id: FR-371:L63
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: An inline value cannot return a borrowed BigInt temporary. A core unsynchronized cell that
    removes Sync, a leaked Box, a new guard/Cow return type, or an eagerly constructed BigInt presented
    as an allocation-avoiding path does not establish this proposal's refinement and bounded-path obligations.
- id: FR-371:L65
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL preserve its default and test-support configurations, no_std plus alloc build
    for `thumbv7em-none-eabi`, and local `forbid(unsafe_code)` policy.
- id: FR-371:L69
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The selected cache behavior is safe race initialization of a retained immutable BigInt image,
    compatible with `no_std` plus `alloc`. The crate SHALL retain its local `forbid(unsafe_code)` policy;
    this requirement authorizes no copied dependency implementation or local unsafe code. Package and
    version selection belong to implementation dependency management, not this requirement. The required
    `thumbv7em-none-eabi` target reports pointer atomics in local rustc 1.98.1 target metadata. Implementation
    qualification SHALL build the supported target set and verify that dependency feature resolution i
- id: FR-371:L71
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The inline case adds a cache cell/pointer; an estimated 32-to-48-byte `Integer` size is a planning
    estimate only, not a measured size guarantee. Materializing an inline value can allocate the retained
    BigInt and, under concurrent initialization, additional candidate BigInts whose losing boxes are dropped.
    Clone of a `Small` SHALL copy the i128 and begin with an empty cache; clone of `Large` SHALL preserve
    the exact BigInt value. Clone, equality, ordering, hashing, Debug and Display SHALL not expose or
    depend on cache state. The precise allocation count under races is intentionally not fixed.
- id: FR-371:L73
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For two native-representable operands whose exact result fits i128, the proof-critical operations
    are primitive construction, Add/Subtract/Multiply/Negate, bits/sign/zero, to_u64, native comparison
    and inclusive interval membership. Their candidate path must not request BigInt construction or materialization.
    Existing Meter/Charge collection allocation and boxed domain refusal are outside this BigInt-only
    allocation claim and remain subject to their existing contracts.
- id: FR-371:L75
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The representation may increase Integer/Value size. Avoiding BigInt work on this path is a
    mechanism objective, not a demonstrated cause of the retained MemoryExhausted outcome or a prediction
    that CBMC will discard every large/cache branch. No lower memory number, altered field-sensitivity
    threshold, narrowed input, relaxed assertion, assumed arithmetic or Kani-only implementation is specified.
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve
    the exact value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX,
    -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range.
    MIN has magnitude bit length 128; its negation is positive 2^127 through genuine arbitrary-precision
    continuation. Compare fitting operations with independent checked i128 arithmetic; compare non-fitting
    results with independently authored decimal literals. Reject malformed/noncanonical spellings as be
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain
    equal, identically ordered, identically hashed with the same hasher and identically formatted before
    and after BigInt access and cloning. A borrowed as_big reports the exact value throughout concurrent
    read-only accesses; all accesses to one retained materialization observe the same object. Clone/drop
    do not invalidate surviving permitted borrows or introduce a cache-state value.
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362 schedule and counters
    on native, native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated
    exact/one-less bit and work boundaries, denial injected at every named point, and outside-domain results.
    Compare all completed/refused/incomplete fields and admitted prefixes with the independent normative
    formulas. No representation transition manufactures a FunctionCall or other charge.
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate,
    magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external
    allocator observer around standalone Integer calls confirms no allocation on that native path, including
    clone after cache access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128
    MIN/MAX, compare independently prepared expected limbs, observe no stream allocation and confirm that
    initially empty caches remain empty. Observation of public metered calls separately iden
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'Public signatures and all existing Integer methods remain available, except that the divisor
    parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve
    the admitted NonZeroInteger signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md),
    replacing only their former plain &Integer parameters. Their existing nonzero results and infallible
    return types remain preserved; every other public signature and Integer method retains its existing
    preservation guarantee. Trait checks preserve Send/Sync and current panic-unwind traits. Default and '
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original
    input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion
    cover and counterexample checks. Healthy verification and both substantive falsifications are required.
    Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler
    error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and
    does not satisfy this criterion.
- id: FR-371:L90
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: '[TC-919](../test-cases/TC-919-bounded-integer-representation-refinement.md) owns the planned
    kernel refinement checks and identifies the separate consumer-owned qualification boundary. The consumer
    obligations are [CG TC-025](ix://agent-ix/quire-contract-codegen/TC-025) and [CG TC-027](ix://agent-ix/quire-contract-codegen/TC-027);
    the kernel does not copy their harnesses or replace their assertions.'
- id: FR-371:L94
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'PARTIALLY IMPLEMENTED / QUALIFICATION OPEN. Published kernel `765a95914bfc9acc5c01dd482630535113c69658`
    already implements Small/Large, lazy borrowed BigInt and native arithmetic, including later parser
    and charge-metadata refinements. The additional range-proven width dispatch described in [AD-003](../assurance/AD-003-bounded-integer-representation.md)
    is proposed and awaits implementation and independent review. This Markdown update adds no executable
    Trace binding or proof receipt. The unchanged original healthy artifact exhausted memory at 16GiB;
    a separate 40GiB diagnostic also exhausted '
- id: TC-919:L13
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: This planned case checks [FR-371](../functional/FR-371-bounded-integer-representation-refinement.md)
    at real kernel entry points. It separates exact semantic refinement, implementation resource observations
    and genuine consumer proof qualification. This document does not execute or replace consumer-owned
    CG TC025/TC027.
- id: TC-919:L17
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 1. For AC-1, construct each fixed boundary through primitive, canonical decimal and from_big
    routes where available. Calculate fitting Add/Subtract/Multiply/Negate results through independent
    checked i128 arithmetic and non-fitting results from independent decimal literals. Include i64 MIN/MAX
    pairs for all four operations, mixed signs, zero and cancellation, plus inputs immediately outside
    i64 on both sides that still fit i128. Inspect that the representability-guarded arm widens before
    arithmetic and contains no checked-i128 overflow/promotion choice, while failed guards retain full-i128
    and
- id: TC-919:L18
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 2. For AC-2, compare mathematical equality, order, hash input behavior for the same hasher/target,
    Display and Debug across construction routes, cache access and cloning. Obtain as_big borrows and
    retain them during synchronized concurrent read-only accesses. Check exact values and stable retained-object
    identity. Join owned threads before dropping their owner; test clone/drop with permitted surviving
    borrows without introducing a lifetime violation as the oracle.
- id: TC-919:L19
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 3. For AC-3, independently derive all operand/arithmetic bit requests and work/result totals
    from FR-362. Exercise native, overflow and arbitrary-size Add/Subtract/Multiply/Negate, including
    cancellation, at exact and one-less limits on fresh meters and on a shared cumulative meter. Inject
    denial at every family point. Check the actual outcome, all incomplete fields, admitted prefix and
    counters; test an optional domain excluding the exact result. No separate copy of the implementation's
    amount helper supplies the oracle.
- id: TC-919:L20
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 4. For AC-4, inspect the actual native branches and use a safe external allocator observer
    around only standalone Integer constructors, fitting operations, bits, to_u64, native interval checks
    and clone. Repeat clone after source cache materialization. Keep operands/observer setup outside the
    window as appropriate and record each included call. Separately observe the metered facade to distinguish
    accounting/refusal allocations; never infer BigInt absence merely from an aggregate peak.
- id: TC-919:L21
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: '5. For AC-5, inspect the implementation dependency manifest and lockfile to record the selected
    safe race-initialization dependency and alloc/no_std feature configuration, without treating a package
    or version as a requirement-level invariant. Compile the existing public method and trait checks under
    default/test-support and the required no_std target in the implementation lane. Exercise real IEEE
    consumers. Use synchronized owned concurrent readers and safe allocation/deallocation observations
    to check the exact retained value, one retained winner, released temporary candidates, and that all '
- id: TC-919:L22
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 6. For AC-6, the consumer owner selects the original genuine healthy TC027, TC027 bound mutant
    and TC025 arithmetic mutant against the actual reviewed/adopted kernel identity. Preflight the exact
    selection without execution, then use the separately allocated runner/resource envelope. Preserve
    original domains, oracle, unwind3, Cadical, 600s, 16GiB, completion cover and falsification witnesses.
    Record each actual result and cleanup separately. Run the existing all-four-operation and refinement/accounting/denial
    obligations; do not accept a healthy-only or split-subset verdict as whole qualifica
- id: TC-919:L26
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Exact values and every public observation match the mathematical contract. Native arithmetic
    overflow reaches genuine arbitrary-precision execution. Meter prefixes, exact requests, counters and
    domain refusal remain unchanged. The bounded standalone path makes no allocation, while lazy BigInt
    access, if selected, exhibits its explicitly disclosed allocation behavior and stable retained borrow.
    Local unsafe/std leakage, lost traits, temporary borrow, cache-state equality/hash, wrapping, extra
    charge or unobserved discarded candidate falsifies the corresponding criterion.
- id: TC-919:L28
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Healthy proof verification and both substantive mutant falsifications are separately observed
    under the unchanged ceilings. Missing mutation execution, MemoryExhausted, compiler panic, timeout
    or unconfirmed cleanup is inconclusive. Any memory/phase observations are attributed to the actual
    measured run and harness; a configured ceiling or prediction is not measured peak memory.
- id: TC-919:L32
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: WIDTH-DISPATCH CHECKS PLANNED / CONSUMER QUALIFICATION OPEN. The baseline Small/Large mechanism
    is implemented; this update claims no execution or result for the new width-dispatch checks, no new
    Trace binding and no successful consumer proof. The unchanged original CG TC-025/TC-027 healthy and
    mutation controls remain a separate consumer-owned qualification boundary; no 16GiB improvement is
    established until all three complete as specified. The kernel and consumer checks have different owners
    and must produce their own evidence before AC-6 or IR-705 is declared complete.
- id: TC-919:L34
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The separate IR-917 V2 clause implication consumer must be measured after reviewed kernel adoption,
    with its exact fixture, identity/cohort and healthy/helper-mutant controls retained. Record actual
    16GiB outcome and peak provenance on IR-705, including inconclusive failures or missing candidate
    input. Its post-change fit is unmeasured; it neither replaces the original AC-6 controls above nor
    changes their ceilings or the existing Decimal/quantity negative obligations.
- id: FR-015-AC-41
  path: agent-ix/quire-contract-codegen:spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'A V2 clause whose body holds integer arithmetic (add, subtract, multiply, negate) as an operand
    of a comparison lowers, and its harness keeps the independent native assertion of FR-015-AC-37 for
    each arithmetic subterm (the oracle''s outcome against the operation evaluated natively in `i128`
    over the same operands, `Refused` assumed away nowhere): a postcondition `amount < 1000 implies amount
    + 1 <= 1000` over `amount: Int[0, 1000]` yields one contract harness. With the installed backend the
    unmutated clause verifies with its cover satisfied, and the same clause with `+` mutated to return
    `left'
- id: FR-015-AC-119
  path: agent-ix/quire-contract-codegen:spec/kani/functional/FR-015-bounded-kani-obligations.md
  role: context_only
  excerpt: 'PLANNED (IR-917; depends on FR-015-AC-40 and FR-015-AC-41). QSL source authors an operation
    with one parameter `amount: Int[0, 1000]` and its V2 postcondition `amount < 1000 implies amount +
    1 <= 1000`; both occurrences of `amount` read that operation parameter, not a state field or subject-produced
    post-state value. The admitted `CheckedPackageV2` yields a clause-identified FR-014-AC-38 oracle and
    an FR-015 postcondition harness for that same clause, not a scalar implication harness. Under FR-015-AC-39
    the harness draws the parameter once within its declared finite domain and passes that same'
```

## Method limits

Jev criterion-strength is unavailable: installed schema includes criterion-strength, but the installed skill explicitly has no Jev client and says not to run. Manual falsifier scrutiny is not a calibrated Jev run. Object review is inapplicable: no public domain-object/schema or endpoint is introduced; the private representation invariant is covered by integrity and scope-boundary. App/security lenses are inapplicable. No exact model deployment ID is exposed; marker uses unavailable.

## Dispositions

### Round 1

Reviewed fix head 1fd2947b2c6cb9cc453ef6093052289ee76eaaec. The exact three replacements in AD-003 and one FR-371 replacement remove the new recorded source revisions. The algorithm, native-width range table, TC-919, all six acceptance rows, genuine IR-917 cover requirement and open qualification state are unchanged. CG AC-41 and AC-119 independently require the healthy cover; no external obligation was invented. No new finding.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AD-003 implementation status and reported IR-917 status no longer record exact source revisions. Truthful implemented/proposed distinction, actual inconclusive failures and separate future controls remain. |
| FND-002 | fixed | FR-371 Status no longer records an exact kernel revision; the partial implementation and open qualification description remains. |

## Current Verdict

**PASS** — Both original HIGH findings are fixed at the reviewed fix head. The original FAIL, finding rows and examined scope remain above as the review record. This accepts only the Markdown specification refinement; implementation and consumer qualification remain open.

## Disposition data

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: The two-tag Small/Large representation, safe lazy borrowed BigInt cache and native arithmetic
    already exist in the published kernel; merged parser and charge-metadata refinements do not implement
    the width dispatch above. This document remains proposed for that additional refinement and its outstanding
    qualification, not because the entire representation is unimplemented. Kernel unit/native evidence
    does not establish consumer proof acceptance.
- fnd: FND-002
  outcome: fixed
  after_excerpt: PARTIALLY IMPLEMENTED / QUALIFICATION OPEN. The published kernel already implements Small/Large,
    lazy borrowed BigInt and native arithmetic, including later parser and charge-metadata refinements.
    The additional range-proven width dispatch described in [AD-003](../assurance/AD-003-bounded-integer-representation.md)
    is proposed and awaits implementation and independent review. This Markdown update adds no executable
    Trace binding or proof receipt. The unchanged original healthy artifact exhausted memory at 16GiB;
    a separate 40GiB diagnostic also exhausted memory, with no proof verdict. All AC-6
```
