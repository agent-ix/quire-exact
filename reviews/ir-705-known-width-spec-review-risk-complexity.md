---
id: SR-14826
title: IR-705 known-width spec-review/risk-complexity review
type: SpecReview
analysis: risk-complexity
scope: agent-ix/quire-exact; spec/assurance/AD-003-bounded-integer-representation.md,
  spec/functional/FR-371-bounded-integer-representation-refinement.md, spec/test-cases/TC-919-bounded-integer-representation-refinement.md
review_set: subset
---

## Summary

Ticket IR-705; PR agent-ix/quire-exact#71. Frozen candidate fb717764bb8a4af9259567350918792266b00128, published parent ca8cef6c8b027bed5343bb2098b9ae0f4194cf76. Exactly AD-003, FR-371 and TC-919 Markdown change. Reviewed all paragraphs, constructor/transition/decision rows, procedures and seven ACs listed in Scope; verbatim excerpts over 600 characters are prefixes of fully read units. All six original AC rows are byte-identical to parent, including AC-6's original domains, oracle, unwind3, Cadical, 600-second/16GiB ceilings, completion cover and both substantive mutants. No source/test/dependency/CI changes, product edits, Cargo/rustc/fmt/metadata/native/Kani/CBMC/goto execution, aggregate gate, commit or push by this reviewer. No applicable AssuranceProfile exists under spec/. Static Rust context was read with rust-review's pre-implementation checklist; this is not an implementation review or acceptance. Actual IR-917 candidate/log remain unavailable, so the reported cost attribution is not independently verified and post-change fit remains UNMEASURED. No source-to-AC test binding was inspected; bindings is empty. Exact runtime model SKU is unavailable and is not invented.

## Verdict

**PASS** — No defects found in the proposed specification amendment; implementation and consumer qualification remain open.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Analysis

FR-371 technical risk HIGH, volatility MEDIUM: a third storage class changes constructor/arithmetic/cache/drop transitions and potentially Integer/Value layout, while Kani tractability remains unestablished and consumer input is missing. Mitigations are named: implement after independent review, exhaustive private transition controls, independent public refinement and metering tests, synchronized cache-lifecycle observations, same-target/configuration before/after layout, no_std/feature builds and unchanged genuine proof controls. The declaration promises neither fixed RAM savings nor a compound-expression fast path. Top hazards: (1) value-correct code with wrong private class/path; (2) cache or semantic hash/order drift across same-value representations; (3) added symbolic/layout states worsen memory; (4) Small128 chained results lose direct Small64 dispatch; (5) IR-917 input/cohort absence prevents fit measurement. Each is covered by the selected checks or explicitly retained as an open measurement risk, with no invented threshold or dominance claim. Failure-domain analysis supplies the MIN/overflow/cancellation/borrow countercases.

## Method Applicability and Evidence Limits

Quire changed-document validation exited 0: 3/3 documents grammar-clean, zero grammar findings. Computed matrix exits 0 and shows FR-371 AC-1..5 tagged, AC-6 and new AC-7 untagged. Tags are not assertion or execution proof; AC-6 remains an existing open qualification and AC-7 is explicitly proposed/unimplemented. quoin advise --json exits 0: all seven ACs authored Test, no mismatch, no inconclusive recommendations; catalog methods were read. Initial template command without repo argument exited 1; corrected quoin write . --types SpecReview exited 0 and its actual installed skeleton/schema were read. Installed module inline-schema/duplicate-archetype/inverse-edge advisories are retained in tool logs; no configuration changes or fallback engine. Quoin 0.28.4; module spec-artifacts-process@b82e42e893612b4b4ee94127714b4577ee893502.

Jev criterion-strength was NOT run: its installed skill explicitly prohibits running without the client. The installed schema does contain criterion-strength, but that does not supply a client. Manual falsifier scrutiny is not calibrated Jev evidence. Gap-analysis and executable Rust gates are not applicable to this Markdown-only proposed mechanism; no production change is claimed. Object/app/security reviews are not applicable: no new public domain object, payload, UI, auth or security boundary. Architecture-evaluation of an implemented architecture is not claimed for this unimplemented proposal; architectural consistency and risks are examined in the ordinary spec lenses.

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
  excerpt: 'The selected experiment replaces the implemented two-tag `Small(i128, cache)` / `Large(BigInt)`
    architecture with **three private storage variants**: `Small64(i64, cache)`, `Small128(i128, cache)`
    and `Large(BigInt)`. This is a representation amendment, not another spelling of the existing value-range
    guard. Both inline variants refine to the same full-i128 mathematical native class. The abstraction
    function maps either inline payload to its exact mathematical integer, ignoring cache state; it maps
    Large to its stored exact mathematical value. Every initialized inline cache equals that payload'
- id: AD-003:L26
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '`native()` may expose a widened i128 mathematical value from either inline variant, but arithmetic
    must dispatch on the private storage variants **before** that helper erases the known storage width.
    Every i128-representable value remains representable inline; this architecture imposes no narrower
    mathematical domain.'
- id: AD-003:L30
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: The following table covers the existing public construction surface and private normalization
    routes. No new From trait implementation or public constructor is introduced.
- id: AD-003:L34
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Existing `From<i64>` | Small64, using an exact i64 payload | Numeric widening is exact;
    do not round-trip through BigInt or derive a caller-domain certificate. |'
- id: AD-003:L35
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Existing `From<i128>`, `From<u64>`, `From<usize>` | Small128 when the input fits i128 |
    Preserve existing exact conversion behavior; do not assume a platform pointer width. Any supported
    non-fitting conversion retains genuine BigInt continuation. Do not narrow a fitting result to Small64
    merely because its value is small. |'
- id: AD-003:L36
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Canonical parsing and `from_big`, including fitting zero and i64-range values | Small128
    when i128-representable; otherwise Large | Preserve canonical-spelling refusal and exact large parsing.
    No minimum-width normalization. |'
- id: AD-003:L37
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| `zero()`, `one()`, default | Small64 with the corresponding payload | Cache starts empty.
    |'
- id: AD-003:L38
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Clone of either inline variant | Same inline variant and exact numeric payload, fresh empty
    cache | Preserve the source variant even after source cache materialization. |'
- id: AD-003:L39
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Clone of Large | Large with exact cloned BigInt | Clone does not reinterpret representation
    as a domain. |'
- id: AD-003:L40
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| General arithmetic BigInt result normalization | Small128 if i128-representable; otherwise
    Large | Includes cancellation back into the native range; no minimum-width narrowing. |'
- id: AD-003:L42
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: Every newly constructed inline value starts with an empty cache; observing a retained value
    does not reset its cache. This table specifies the listed construction and normalization boundaries;
    it does not redesign the other public integer operations. Their existing exact mathematical behavior,
    refusal behavior, and admitted signatures remain required.
- id: AD-003:L48
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Small64 / Small64 Add, Subtract, Multiply | Match both storage variants before `native()`;
    widen both actual i64 payloads to i128 before ordinary arithmetic; return Small128 with the exact
    result. No checked-i128 overflow-to-BigInt continuation exists within this storage arm. |'
- id: AD-003:L49
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Small64 Negate | Match storage before `native()`; widen the i64 payload to i128 before negation;
    return Small128. In particular negated i64 MIN is positive 2^63. |'
- id: AD-003:L50
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Any other all-inline binary pair, or Small128 Negate | Preserve the implemented numeric
    signed-i64 representability guard and widening arm. When that guard fails, preserve checked full-i128
    arithmetic and genuine BigInt overflow continuation. A fitting inline result is Small128; a promoted
    result normalizes as above. |'
- id: AD-003:L51
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| At least one Large operand, or Large Negate | Preserve the existing general exact BigInt
    route and normalize a fitting result to Small128, otherwise Large. |'
- id: AD-003:L53
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: 'For the direct Small64 arms, the exact extrema after widening are:'
- id: AD-003:L57
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Add | -2^64 | 2^64 - 2 |'
- id: AD-003:L58
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Subtract | -2^64 + 1 | 2^64 - 1 |'
- id: AD-003:L59
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Multiply | -2^126 + 2^63 | 2^126 |'
- id: AD-003:L60
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Negate | -2^63 + 1 | 2^63 |'
- id: AD-003:L62
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: 'All fit signed i128, including i64 MIN squared at 2^126. Genuine i128 overflow remains supported
    in the other inline arms: negating i128 MIN yields positive 2^127 through BigInt. No truncating cast,
    saturation, wrapping, assumption, copied kernel, Kani-only implementation, public narrowed API or
    altered proof input supplies this mechanism.'
- id: AD-003:L64
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '**Chaining limit:** Small64-pair arithmetic always returns Small128, even when the result
    happens to fit i64. The next operation can therefore take the existing value-guarded general inline
    route. This experiment adds no automatic minimum-width result normalization, caller-domain certificate
    or compound-expression fast-path guarantee. It does not promise that the four-helper IR-917 clause
    avoids general continuation, or that CBMC prunes all cache/general branches. Numeric casts preserve
    value relations already; retained storage width is a distinct structural experiment, not evidence
    that the b'
- id: AD-003:L68
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: The stable public `as_big(&self) -> &BigInt` contract requires storage that outlives each returned
    borrow. For either Small64 or Small128, the first BigInt-required observation initializes its own
    private safe race-initialization cache from its immutable numeric payload; subsequent observations
    borrow the retained immutable winner. For Large, observations borrow its stored BigInt. Concurrent
    first access may construct multiple boxed candidates. The cache retains one winner and drops losing
    candidates; all successful callers borrow the winner. The initializer must not call cache-dependent
    metho
- id: AD-003:L70
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: Cloning either inline variant preserves that variant and its numeric payload and starts with
    a fresh empty cache. Cloning Large preserves the exact BigInt. Equality, ordering, hashing, Debug
    and Display are functions of mathematical value only; cache initialization cannot change their results.
    Cache materialization and discarded candidates are implementation resource costs and add no semantic
    meter charge.
- id: AD-003:L76
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Integer value representation | Three private variants Small64, Small128 and Large, with
    one cache-independent mathematical abstraction. | The implemented two-tag representation already supports
    full-i128 native execution; retaining it avoids another tag and transition invariants. The selected
    experiment retains construction width directly, at the cost of additional states and possible layout
    growth. BigInt-only or eager dual storage defeats the bounded allocation objective. |'
- id: AD-003:L77
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Native arithmetic dispatch | Small64 storage arms widen to i128 before all four operations
    and return Small128; other inline arms retain existing numeric guards, checked-i128/general continuation.
    | Repeating the implemented numeric width guard adds no new mechanism. Retaining a width tag avoids
    re-deriving width in the direct storage arms, but may not improve symbolic execution and does not
    optimize subsequent Small128 results. Deleting general continuation would break arbitrary precision.
    |'
- id: AD-003:L78
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Stable borrowed BigInt for both inline variants | Safe no_std-compatible race initialization
    with alloc support and stable retained borrowing. | A core unsynchronized cell would remove Sync;
    a std-only primitive violates no_std; leaked boxes, local unsafe, or a changed guard/Cow API violate
    the constraints or public borrow contract. Any selected implementation must preserve the race winner/loser
    lifecycle and stable borrow contract. |'
- id: AD-003:L79
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Concurrent initialization | Allow candidate construction by racing callers, retain one immutable
    winner, and drop every losing candidate. | A once-only candidate allocation promise is not supportable
    with this race API. The design favors safe Sync and stable borrows while explicitly accepting temporary
    concurrent allocation overhead. |'
- id: AD-003:L80
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Numeric observations and metering | Preserve mathematical value, public observation, existing
    trait behavior, and FR-362/FR-358 charge/refusal order exactly. | Making representation state observable
    or adding charges for cache work would change the contract. |'
- id: AD-003:L81
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '| Qualification claim | Keep the native-width refinement and consumer qualification open and
    require the unchanged CG TC-025 arithmetic mutant, TC-027 healthy case, and TC-027 bound mutant at
    their existing 16GiB/600-second/unwind3/Cadical settings. | Unit tests or a changed/smaller consumer
    harness cannot establish the intended end-to-end Kani result. No memory improvement is asserted before
    those controls pass. |'
- id: AD-003:L85
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: The selected architecture requires a safe race-initialization cache that supplies stable lazy
    references without std and supports concurrent initialization using alloc. Package and version selection,
    as well as concrete feature resolution, belong to implementation dependency management. Local rustc
    1.98.1 target metadata reports pointer atomics for `thumbv7em-none-eabi`; that metadata is not a build
    result or proof of all supported targets. Implementation qualification must confirm supported target
    compatibility and that dependency feature resolution does not introduce std or weaken local `for
- id: AD-003:L87
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: Each inline variant carries its own cache cell/pointer. Adding a third variant may change discriminant
    placement, alignment, Integer size and enclosing Value size. Record actual before/after layouts for
    the same target and configuration; no byte-size estimate is a correctness or performance threshold.
    First BigInt materialization can allocate the retained value; concurrent calls can allocate additional
    candidates, with losers freed after initialization. TC-919 must record actual size and allocation
    behavior without treating layout growth or a fixed allocation count as a correctness oracle.
- id: AD-003:L91
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: '- The third storage variant, its transitions, larger inline representation or BigInt cache
    branch may leave CBMC memory use unchanged or make it worse. The original unchanged CG healthy and
    mutation controls determine qualification; no 16GiB improvement is claimed until they pass. Owner:
    IR-705 proof lane; revisit after implementation review and consumer evidence.

    - The currently identified target''s pointer-atomic metadata does not establish all downstream targets
    or dependency feature behavior. Owner: implementation and platform review; build the declared supported
    target set before adoption.'
- id: AD-003:L97
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: The published baseline implements the two-tag Small/Large representation, safe lazy borrowed
    BigInt cache, native arithmetic, canonical fitting parse, charge metadata and numeric i64-guarded
    widening. Actual static source/CFG inspection confirms that the widened arithmetic arm exists; it
    does not establish fallback feasibility or memory dominance. The three-variant architecture above
    is **PROPOSED / UNIMPLEMENTED** and awaits independent specification review before implementation.
    It amends the selected representation; it is not a claim that the existing guards are missing. Kernel/native
    evide
- id: AD-003:L99
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: 'The unchanged original healthy artifact still has no proof verdict: the measured 16GiB run
    on the existing implementation exhausted memory, and the separate authorized 40GiB diagnostic also
    exhausted memory. Neither changes the original AC-6 ceiling or discharges a control. IR-917 additionally
    reports MemoryExhausted for its V2 clause implication proof, with four metered runtime helpers and
    symbolic Integer-add promotion identified by its owner as costs. Its supplied source/log are unavailable
    locally, so that attribution and peak are reported evidence, not independently verified measurements '
- id: AD-003:L101
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
    declared inclusive mathematical domain. The representation SHALL distinguish three private storage
    variants `Small64(i64, cache)`, `Small128(i128, cache)` and `Large(BigInt)` as specified in AD-003.
    Both inline variants SHALL refine to the same full-i128 mathematical native class. The abstraction
    function maps either inline payload to its mathematical integer regardless of variant identity or
    cache st
- id: FR-371:L27
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When a native add, subtract, multiply or negate fits, the kernel SHALL retain its exact mathematical
    result.
- id: FR-371:L29
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For two Small64 operands, Add/Subtract/Multiply SHALL match their actual storage variants before
    any helper erases width, widen their i64 payloads to i128 before ordinary arithmetic, and return Small128
    with the exact result. Small64 Negate SHALL follow the same storage-first dispatch and widened computation.
    These arms SHALL contain no checked-i128 overflow-to-BigInt continuation. Other inline combinations
    SHALL retain the existing numeric signed-i64 guard with widened arithmetic and the checked full-i128/genuine
    arbitrary-precision continuation when that guard fails. Large inputs SHALL retai
- id: FR-371:L31
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'The kernel SHALL follow AD-003''s constructor/normalization table: existing `From<i64>` uses
    Small64; existing `From<i128>`, `From<u64>` and `From<usize>` use Small128 when representable, with
    `From<usize>` preserving its existing genuine non-fitting continuation without platform-width assumptions.
    This policy SHALL NOT introduce a new public conversion API. Fitting canonical parse and from_big
    SHALL yield Small128, including small numeric values. Zero/one/default SHALL yield Small64. Clone
    SHALL preserve the source inline variant and payload while resetting its cache. This policy SHALL
    NOT imp'
- id: FR-371:L33
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: A Small64 operation's Small128 result MAY take the general inline route at the next operation.
    The mechanism SHALL NOT be presented as a guaranteed compound-expression fast path or successful IR-917
    qualification.
- id: FR-371:L35
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When the exact result does not fit the native range, the kernel SHALL obtain the full result
    through genuine arbitrary-precision execution.
- id: FR-371:L37
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL preserve exact values supplied by `from_big`, primitive constructors and canonical
    parsing.
- id: FR-371:L39
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL preserve zero, sign, magnitude bits, conversions, every existing public integer
    operation, numeric equality and order, canonical Display and the existing mathematical Debug value
    independently of storage and materialization state.
- id: FR-371:L41
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When two Integers have the same mathematical value, the kernel SHALL produce the same hash
    input behavior as their existing BigInt representation.
- id: FR-371:L43
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Hash values are not a cross-version or cross-target persistence format. The requirement above
    preserves the current BigInt hashing behavior for the same hasher and target; it does not introduce
    a new serialized hash identity.
- id: FR-371:L45
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When evaluating public metered Add/Subtract/Multiply/Negate, the kernel SHALL preserve the
    charge order, exact request amounts, high-water sizes, cumulative work/results, domain refusal and
    incomplete prefix defined by FR-362 and FR-358.
- id: FR-371:L47
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL exclude representation changes and materialization from semantic evaluator
    charges.
- id: FR-371:L49
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Machine overflow is neither a domain refusal nor an accounting denial. The result must not
    wrap, saturate or narrow. Native magnitude calculations must handle `i128::MIN` without signed absolute-value
    overflow. Small64 magnitude and limb-stream calculation SHALL widen before deriving unsigned magnitude,
    including i64 MIN; neither inline stream SHALL initialize its BigInt cache. Existing arbitrary-size
    charge and denial amounts remain exact.
- id: FR-371:L53
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL retain the public signature `Integer::as_big(&self) -> &BigInt` and return
    the exact mathematical value for the lifetime of that borrow.
- id: FR-371:L55
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: When a representation owns a materialized BigInt, the kernel SHALL retain that immutable object
    until all borrows permitted by the `Integer` lifetime have ended.
- id: FR-371:L57
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL preserve `Integer`'s existing Send/Sync, clone, default, drop and panic-unwind
    trait behavior.
- id: FR-371:L59
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: "For each of `Small64` and `Small128`, `as_big` SHALL lazily initialize and return a borrow\
    \ of a retained immutable BigInt image; for `Large`, it SHALL borrow the stored BigInt. Each inline\
    \ borrow SHALL remain valid for the borrow lifetime tied to `&self`. The cache SHALL use a safe no_std-compatible\
    \ race-initialization mechanism with alloc support and no local unsafe code. Its initializer SHALL\
    \ construct a candidate only from that variant\u2019s immutable numeric payload. Concurrent initializers\
    \ MAY construct multiple candidates; exactly one SHALL be retained, every losing candidate SHALL be\
    \ droppe"
- id: FR-371:L61
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For the bounded Decimal membership operation in [FR-372](./FR-372-fallible-bounded-decimal-membership.md),
    the kernel SHALL traverse an inline value's fixed-width unsigned-magnitude limbs without initializing
    or changing its private BigInt cache, including when that cache is empty. The Large representation
    borrows its retained magnitude limbs through the maintained dependency iterator. This stream policy
    does not change the public `as_big`, Hash or Debug materialization behavior above; the helper retains
    FR-372's one fallible scratch request.
- id: FR-371:L63
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: If concurrent callers construct materialization candidates, then the kernel SHALL retain exactly
    one immutable winner and release every losing candidate.
- id: FR-371:L65
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: All callers for one Integer return the retained winner. The initializer converts only the immutable
    native value; it does not re-enter cache-dependent Integer methods. An inline clone starts with an
    empty private cache even if its source cache was materialized. Candidate allocations can occur once
    per simultaneously initializing caller; this is not a one-allocation or caller-count bound. No background
    task, retry loop, stored historical certificate or global cache is introduced. A proposed implementation
    must report this resource behavior explicitly rather than claim fixed-capacity or once-onl
- id: FR-371:L67
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: An inline value cannot return a borrowed BigInt temporary. A core unsynchronized cell that
    removes Sync, a leaked Box, a new guard/Cow return type, or an eagerly constructed BigInt presented
    as an allocation-avoiding path does not establish this proposal's refinement and bounded-path obligations.
- id: FR-371:L69
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The kernel SHALL preserve its default and test-support configurations, no_std plus alloc build
    for `thumbv7em-none-eabi`, and local `forbid(unsafe_code)` policy.
- id: FR-371:L73
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The selected cache behavior is safe race initialization of a retained immutable BigInt image,
    compatible with `no_std` plus `alloc`. The crate SHALL retain its local `forbid(unsafe_code)` policy;
    this requirement authorizes no copied dependency implementation or local unsafe code. Package and
    version selection belong to implementation dependency management, not this requirement. The required
    `thumbv7em-none-eabi` target reports pointer atomics in local rustc 1.98.1 target metadata. Implementation
    qualification SHALL build the supported target set and verify that dependency feature resolution i
- id: FR-371:L75
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Both inline variants carry a cache cell/pointer. The third variant may change Integer/Value
    layout; record actual before/after size and alignment on the same target/configuration without a fixed-size
    correctness or performance threshold. Materializing an inline value can allocate the retained BigInt
    and, under concurrent initialization, additional candidate BigInts whose losing boxes are dropped.
    Clone of either inline variant SHALL preserve its storage class, copy its numeric payload and begin
    with an empty cache; clone of `Large` SHALL preserve the exact BigInt value. Clone, equality, orderi
- id: FR-371:L77
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For two native-representable operands whose exact result fits i128, the proof-critical operations
    are primitive construction, Add/Subtract/Multiply/Negate, bits/sign/zero, to_u64, native comparison
    and inclusive interval membership. Their candidate path must not request BigInt construction or materialization.
    Existing Meter/Charge collection allocation and boxed domain refusal are outside this BigInt-only
    allocation claim and remain subject to their existing contracts.
- id: FR-371:L79
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
- id: FR-371-AC-7
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Private mechanism checks distinguish Small64 and Small128 construction, normalization and clone
    routes according to AD-003 without exposing a public tag API. Source inspection identifies storage
    matching before native-value extraction for Small64 Add/Subtract/Multiply/Negate, widening before
    ordinary arithmetic, Small128 result construction and no BigInt continuation inside those arms. Check
    all other inline pair classes, failed numeric-width guards, genuine i128 overflow and Large cancellation;
    a small Small128 result remains Small128 on chaining. Confirm both inline MIN magnitude streams and
- id: FR-371:L95
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: '[TC-919](../test-cases/TC-919-bounded-integer-representation-refinement.md) owns the planned
    kernel refinement checks and identifies the separate consumer-owned qualification boundary. The consumer
    obligations are [CG TC-025](ix://agent-ix/quire-contract-codegen/TC-025) and [CG TC-027](ix://agent-ix/quire-contract-codegen/TC-027);
    the kernel does not copy their harnesses or replace their assertions. The five unchanged actual CG
    consumer proof/receipt tests required by [FR-359-AC-10](./FR-359-cumulative-meter-boundary.md) remain
    a separate adoption boundary; the three original healthy/mutant co'
- id: FR-371:L99
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: BASELINE IMPLEMENTED / THREE-VARIANT AMENDMENT PROPOSED, UNIMPLEMENTED / QUALIFICATION OPEN.
    The published kernel already implements two-tag Small/Large, lazy borrowed BigInt, native arithmetic,
    fitting canonical parse, charge-metadata refinements and numeric signed-i64-guarded widening. The
    selected three-variant architecture in [AD-003](../assurance/AD-003-bounded-integer-representation.md)
    is a distinct private representation experiment awaiting independent review and implementation. This
    Markdown update adds no executable Trace binding or proof receipt. The unchanged original healthy
    artif
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
    i64 on both sides that still fit i128. Exercise every inline storage pair and the direct Small64 arithmetic
    boundaries; retain numeric-guard and failed-guard full-i128/genuine BigInt continuation checks for
- id: TC-919:L24
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: '7. For AC-7, use private unit tests, without a public introspection API, to check the complete
    AD-003 construction/normalization table for existing supported routes: signed i64, i128, u64, usize,
    zero/one/default, fitting and overflowing parse/from_big, clone of both inline variants before/after
    cache access, and fitting/large general-result normalization. Preserve the exact arithmetic and magnitude
    behavior of private unsigned-width helpers without adding public primitive conversions. Use same-value
    routes to establish both inline classes exist without treating class equality as mathematical '
- id: TC-919:L30
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Exact values and every public observation match the mathematical contract. Native arithmetic
    overflow reaches genuine arbitrary-precision execution. Meter prefixes, exact requests, counters and
    domain refusal remain unchanged. The bounded standalone path makes no allocation, while lazy BigInt
    access, if selected, exhibits its explicitly disclosed allocation behavior and stable retained borrow.
    Local unsafe/std leakage, lost traits, temporary borrow, cache-state equality/hash, wrapping, extra
    charge or unobserved discarded candidate falsifies the corresponding criterion.
- id: TC-919:L32
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Healthy proof verification and both substantive mutant falsifications are separately observed
    under the unchanged ceilings. Missing mutation execution, MemoryExhausted, compiler panic, timeout
    or unconfirmed cleanup is inconclusive. Any memory/phase observations are attributed to the actual
    measured run and harness; a configured ceiling or prediction is not measured peak memory.
- id: TC-919:L36
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: THREE-VARIANT CHECKS PLANNED / CONSUMER QUALIFICATION OPEN. The baseline two-tag Small/Large
    mechanism and numeric i64-guarded widening are implemented. The new Small64/Small128/Large architecture
    is proposed and unimplemented; this update claims no new execution, Trace binding, proof result or
    improvement. Retaining width in a private tag may increase symbolic states or layout size, and chained
    results do not regain Small64 automatically. The unchanged original CG TC-025/TC-027 healthy and mutation
    controls remain a separate consumer-owned qualification boundary; no 16GiB improvement is estab
- id: TC-919:L38
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: The separate IR-917 V2 clause implication consumer must be measured after reviewed kernel adoption,
    with its exact fixture, identity/cohort and healthy/helper-mutant controls retained. Record actual
    16GiB outcome and peak provenance on IR-705, including inconclusive failures or missing candidate
    input. Its post-change fit is unmeasured; it neither replaces the original AC-6 controls above nor
    changes their ceilings or the existing Decimal/quantity negative obligations.
- id: src/integer.rs:IntegerRepr
  path: src/integer.rs
  role: context_only
  excerpt: "enum IntegerRepr {\n    Small {\n        value: i128,\n        cache: once_cell::race::OnceBox<BigInt>,\n\
    \    },\n    Large(BigInt),\n}"
- id: src/integer.rs:From<i64>
  path: src/integer.rs
  role: context_only
  excerpt: "impl From<i64> for Integer {\n    fn from(value: i64) -> Self {\n        Self::small(i128::from(value))\n\
    \    }\n}"
- id: FR-362
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: When evaluating kernel-owned integer or rational arithmetic, integer or rational ordering,
    or a Boolean connective over already-decided operands, the `quire-exact` kernel SHALL return the exact
    outcome and charge the point order and amounts defined below.
bindings: []
```
