---
id: SR-6406
title: IR-705 inline-limb spec-review/scope-boundary review
type: SpecReview
analysis: scope-boundary
scope: agent-ix/quire-exact@f356b7d4514357bbf130ec5ccc83a75d8df92646; spec/functional/FR-371-bounded-integer-representation-refinement.md,
  spec/functional/FR-372-fallible-bounded-decimal-membership.md, spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
review_set: subset
---

## Summary

Ticket: IR-705. Frozen three-Markdown diff only; no production Rust, CI, dependency or executable-test change. No applicable AssuranceProfile was found under this repository spec tree. Rust-review was loaded before source-informed feasibility assessment; repository conventions outrank portable Rust style. Existing source reads are context only, not acceptance or a source gap review. All implementation controls remain prospective; TC-920 says PLANNED/UNRUN. No Cargo, Kani, solver, benchmark or full CI was run.

## Scope

```yaml
scope:
- id: FR-371
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For the bounded Decimal membership operation in [FR-372](./FR-372-fallible-bounded-decimal-membership.md),
    the kernel SHALL traverse an inline value's fixed-width unsigned-magnitude limbs
    without initializing or changing its private BigInt cache, including when that
    cache is empty. The Large representation borrows its retained magnitude limbs
    through the maintained dependency iterator. This stream policy does not change
    the public `as_big`, Hash or Debug materialization behavior above; the helper
    retains FR-372's one fallible scratch request.
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate
    preserve the exact value for fixed boundaries and reproducibly sampled inputs.
    Fixed cases include i128 MIN/MAX, -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated
    MIN and large cancellation into the native range. MIN has magnitude bit length
    128; its negation is positive 2^127 through genuine arbitrary-precision continuation.
    Compare fitting operations with independent checked i128 arithmetic; compare non-fitting
    results with independently authored decimal literals. Reject malformed/noncanonical
    spellings as be
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: fore. No result wraps, saturates or becomes a representation-overflow refusal.
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and
    arithmetic routes remain equal, identically ordered, identically hashed with the
    same hasher and identically formatted before and after BigInt access and cloning.
    A borrowed as_big reports the exact value throughout concurrent read-only accesses;
    all accesses to one retained materialization observe the same object. Clone/drop
    do not invalidate surviving permitted borrows or introduce a cache-state value.
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
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
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: tifies ordinary accounting/refusal allocations and does not misreport the
    entire call as allocation-free.
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: 'Public signatures and all existing Integer methods remain available, except
    that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating
    and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined
    by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their
    former plain &Integer parameters. Their existing nonzero results and infallible
    return types remain preserved; every other public signature and Integer method
    retains its existing preservation guarantee. Trait checks preserve Send/Sync and
    current panic-unwind traits. Default and '
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: test-support checks and the required thumbv7em-none-eabi build retain no_std
    plus alloc behavior and local unsafe prohibition. Existing IEEE BigInt consumers
    observe the same exact values. If lazy materialization is selected, cache concurrency
    tests demonstrate one retained immutable winner, released losers and the disclosed
    first-access allocation behavior; a safe observer records temporary candidates
    without imposing a fictitious once-only bound.
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic
    mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second
    native ceiling, 16GiB memory ceiling, completion cover and counterexample checks.
    Healthy verification and both substantive falsifications are required. Existing
    all-four-operation, accounting/denial and refinement obligations remain required.
    A compiler error, MemoryExhausted, timeout, missing mutation execution or missing
    teardown is inconclusive and does not satisfy this criterion.
- id: FR-371
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: The proposed lazy-materialization option permits the first `as_big` on
    an inline value to allocate its BigInt image; the current implementation instead
    borrows an already constructed BigInt. Hash, Debug and other BigInt-required operations
    may also trigger materialization. Such initialization is implementation work,
    not an added semantic charge. It inherits the supported allocator's existing allocation-failure
    behavior; this proposal introduces no new typed allocation-refusal policy or recovery
    guarantee.
- id: FR-371
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: PROPOSED / UNIMPLEMENTED / UNRUN. All six criteria are prospective. No
    new executable Trace bindings, proof receipt or memory-improvement claim accompanies
    this Markdown proposal. Dependency choice and materialization mechanics remain
    subject to independent architecture/spec review. The original healthy exact-kernel
    qualification had an unavailable measured peak; an older distinct control's peak
    and a separate compiler-only panic cannot establish the cause or success of this
    proposal.
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "1. The kernel SHALL expose a fallible `DecimalType::try_contains` operation\n\
    \   borrowing the Decimal and original cancellation handle, with a completed\n\
    \   Boolean result or typed `DecimalMembershipFailure`. This operation SHALL\n\
    \   leave both the type and Decimal representations unchanged."
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "2. Completed membership SHALL conform to the authoritative Decimal value\n\
    \   membership rule in [QSpec FR-140](ix://agent-ix/quire-specification/FR-140)."
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "3. When that owning rule rejects the normalized scale, the operation SHALL\n\
    \   complete with nonmembership after\n   polling cancellation and without requesting\
    \ scratch storage. Sign and zero\n   comparisons SHALL use borrowed retained data\
    \ without materializing absolute\n   values. Existing bool-only membership, Decimal\
    \ construction/normalization,\n   ordering, placement and evaluation are outside\
    \ this operation's scope.\n\n### Finite work and storage\n\nLet `lo` and `hi`\
    \ denote the type's borrowed endpoint coefficients, and let\n`b = max(bits(c),\
    \ bits(lo), bits(hi))`, with zero's magnitude bit length "
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: 'one.

    Let `k` denote the nonnegative coefficient-alignment shift requested by the

    authoritative membership rule on its scale-admissible path. The consumer establishes

    finite admitted maxima `B >= b` and `S >= k` for every operand and the actual

    shift, including both type endpoints. Define:


    | Measure | Bound and unit |

    | --- | --- |

    | Scratch length | `L = ceil(b / 32)` unsigned 32-bit elements. |

    | Division passes per endpoint | `d = min(k, b)` passes, stopping earlier when
    the quotient becomes zero. |

    | Helper arithmetic | At most `2 × (d + 2) × L` fixed-width limb steps for both
    endpoint com'
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: 'parisons. |

    | Temporary storage | At most one live scratch reservation requesting `4 × L`
    bytes, plus constant-sized scalar/iterator state. |

    | Consumer envelope | Substitute `B` and `S`: `L_B = ceil(B / 32)`, at most `2
    × (min(S, B) + 2) × L_B` limb steps and a `4 × L_B`-byte scratch request. |


    A limb step is one scratch copy/initialization, one quotient/remainder update

    of a scratch limb by ten, or one comparison of scratch and borrowed coefficient

    limbs. These are proof measures for the bounded kernel helper, not semantic

    charges or supplied-admission logical events. Fixed scalar sign/scal'
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: 'e checks,

    checked layout arithmetic and cancellation polls accompany these steps;

    there is at most one fallible allocation request. No input-sized work is hidden

    inside a scalar check or iterator operation.'
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "4. The kernel SHALL enforce the above envelope using one reusable\n  \
    \ fallibly-reserved scratch buffer of `u32` magnitude limbs and borrowed\n   magnitude-limb\
    \ streams. For an inline `i128`, the operation SHALL obtain\n   its unsigned magnitude\
    \ using `unsigned_abs`, including `i128::MIN`, and\n   yield at most four `u32`\
    \ limbs in least-significant-first order using\n   constant-sized fixed-width\
    \ state; zero yields no limbs. For a Large\n   arbitrary-precision representation,\
    \ the operation SHALL obtain magnitude\n   limbs from the maintained integer dependency's\
    \ allocation-free borrowed\n   iterator. N"
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "either stream SHALL initialize or change a materialization cache,\n  \
    \ construct a BigInt image, or request allocation. The operation SHALL NOT\n \
    \  allocate an absolute value, radix string, power of ten, cloned BigInt or\n\
    \   BigInt arithmetic result. Each yielded limb SHALL participate in the existing\n\
    \   scratch-copy or coefficient-comparison limb step, without an additional\n\
    \   input-sized traversal."
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "5. For a same-sign nonzero endpoint comparison, the helper SHALL copy\
    \ the\n   endpoint's magnitude into the scratch buffer, padding only within `L`.\n\
    \   It SHALL divide that scratch magnitude by ten at most `k` times, stopping\n\
    \   when the quotient becomes zero, and retain whether any discarded remainder\n\
    \   was nonzero. Each pass SHALL update its zero/nonzero status within its limb\n\
    \   steps, without an additional unbounded scan."
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "6. If `q = floor(abs(endpoint) / 10^k)`, magnitude comparison SHALL compare\n\
    \   `abs(c)` with `q`. When those magnitudes are equal, the shifted magnitude\n\
    \   SHALL equal the endpoint exactly when every discarded remainder was zero;\n\
    \   otherwise it is smaller. Negative same-sign comparisons SHALL reverse the\n\
    \   magnitude ordering. The two inclusive membership comparisons SHALL reuse\n\
    \   the same scratch allocation.\n\nThe division invariant is Euclidean decomposition\
    \ of the endpoint magnitude.\nSince `10 >= 2`, a magnitude of at most `b` bits\
    \ reaches quotient zero after\nat most `b` divisions. Each di"
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: 'vision step uses a `u64` intermediate

    `carry × 2^32 + limb`, with `carry < 10`; its maximum is below `10 × 2^32`,

    so quotient/remainder by ten needs no input-sized arithmetic or allocation.

    One endpoint uses at most `L` copy steps, `d × L` division steps and `L`

    comparison steps. The borrowed coefficient and endpoints remain authoritative;

    the scratch representation exists only during this kernel call.


    ### Refusal and cancellation'
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "7. Before requesting scratch storage, the kernel SHALL check element-count,\n\
    \   native layout-byte and capacity conversions without wrapping or truncation.\n\
    \   A failed conversion or layout calculation SHALL return typed capacity/size\n\
    \   failure, distinct from an actual allocator denial."
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "8. The scratch buffer SHALL request capacity through fallible reservation\
    \ before\n   writing its elements. An actual allocator denial SHALL return typed\n\
    \   allocation failure retaining the actual requested additional-element count,\n\
    \   element width and requested layout bytes. Failure SHALL NOT become\n   nonmembership,\
    \ a configured WorkUnits refusal or a panic."
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "9. The helper SHALL poll the caller's original `Cancel` before the allocation\n\
    \   request, immediately after its successful return, before each limb step,\n\
    \   and before publishing a completed result. When a poll observes cancellation,\n\
    \   the helper SHALL return typed cancellation with that handle's cause before\n\
    \   another limb step or completed result. Pre-cancelled calls SHALL request no\n\
    \   scratch allocation. A cancelled call SHALL release its owned scratch.\n\n\
    The arithmetic response gap is at most one fixed-width limb step between polls.\n\
    Allocation and release are indivisible allocator calls;"
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: ' polling cannot interrupt

    them. The storage bound describes requested live scratch layout bytes, not

    allocator-internal rounding, bookkeeping or resident memory. This contract

    does not claim a wall-clock bound on allocator calls or scheduler delays.

    If allocation itself fails, that observed failure is reported directly; no

    successful-result cancellation boundary is crossed. The operation has no

    semantic meter, replacement cancellation handle or partial admission output.


    ### Consumer boundary and qualification'
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "10. A supplied-admission consumer SHALL invoke this authoritative helper\
    \ under\n    the actual initiating logical event and cumulative phase budget owned\
    \ by\n    [QSV FR-109](ix://agent-ix/quire-semantic-value/FR-109). It SHALL establish\n\
    \    the admitted `B` and `S` premises and native scratch envelope before claiming\n\
    \    that Decimal membership is qualified. Helper limb steps SHALL NOT create\n\
    \    an additional traversal event, per-limb supplied budget debit or evaluator\n\
    \    charge."
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "11. The consumer SHALL preserve the helper's typed nonmembership, allocation,\n\
    \    capacity and cancellation distinctions and the actual membership locus.\n\
    \    A stopped supplied admission SHALL publish no partially accepted descriptor.\n\
    \    A replay-triggered final membership check SHALL retain its admission owner;\n\
    \    a conversion-triggered shared operation retains its initiating conversion\n\
    \    owner without duplicate phase debit."
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: "12. Before claiming implementation acceptance, the owning implementation\
    \ SHALL\n    demonstrate the mathematical invariant, every reachable scratch request,\n\
    \    the limb-step bound and original-handle polling against the final maintained\n\
    \    dependency and production helper. A source formula alone SHALL NOT qualify\n\
    \    the released bool-only helper or establish downstream full-size acceptance."
- id: FR-372-AC-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: Completed results match the authoritative membership rule for inclusive
    lower/upper equality, adjacent outsiders, negative and zero values, scale refusal,
    all rounding spellings and equivalent retained representations; neither representation
    is changed. The `[0,100]`, scale bounds `2,2`, `(1,0)` fixture completes true
    at the exact upper bound.
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Production helper observations on admitted `B,S` fixtures stay within the
    one-request `4 × L_B`-byte scratch envelope and `2 × (min(S,B)+2) × L_B` limb-step
    bound, including a longest-pass endpoint fixture and a large shift that terminates
    when quotient becomes zero. Controls exercise actual inline and Large production
    paths, including zero, 32-bit limb crossings, i128 MIN/MAX and signed magnitudes
    beyond i128. Stream traversal requests no allocation and leaves cold inline caches
    empty; inline streams yield at most four least-significant-first u32 limbs. Both
    stream mechanisms and all scalar s
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: teps contain no input-sized hidden allocation or work. The helper retains
    its one fallible scratch request.
- id: FR-372-AC-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: Denying the actual scratch reservation reports its actual native request
    as typed allocation failure. A separate checked-capacity failure retains its own
    classification. Removing the allocation denial yields the same membership as a
    fresh call, with no partial accepted result after either refusal.
- id: FR-372-AC-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: A pre-cancelled original handle causes no scratch request. Cancellation
    observed during the longest admitted division/comparison path returns the original
    cause before another limb step or result; cancellation observed immediately after
    successful reservation prevents scratch work. Neither failure is classified as
    nonmembership or work exhaustion.
- id: FR-372-AC-5
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: An integrated supplied Decimal fixture with independently enumerated N
    owning logical events succeeds at N and denies its actual next event at N-1 before
    entering that event's helper. Its already-spent budget and original cancellation
    are retained across arguments, with no additional helper event, per-limb debit,
    evaluator meter change or partial descriptor. Allocation/capacity/cancellation
    causes retain their supplied-admission locus.
- id: FR-372-AC-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Final implementation qualification establishes the Euclidean/remainder
    and sign invariants, exact reservation boundary, checked layout conversions, allocation-free
    inline unsigned-magnitude stream and Large borrowed dependency iterator, absence
    of cache initialization or BigInt-image construction on those streams, and stated
    arithmetic polling gap. Its record distinguishes requested storage from allocator
    overhead and work latency from unbounded allocator elapsed time.
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "- [TC-920](../test-cases/TC-920-fallible-decimal-membership-boundaries.md)\n\
    \  defines the falsifying controls.\n- [QSpec FR-140](ix://agent-ix/quire-specification/FR-140)\
    \ owns Decimal value\n  semantics and the membership rule invoked by this kernel\
    \ helper.\n- [QSV FR-109](ix://agent-ix/quire-semantic-value/FR-109) owns cumulative\n\
    \  supplied logical work, construction custody and typed phase/locus preservation.\n\
    \  [QSL FR-321](ix://agent-ix/quire-spec-language/FR-321) owns supplied consumer\n\
    \  admission. IR-714 and QSL-681 align phase interfaces; QSL owns public phase\n\
    \  projection and numeric defaul"
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "ts. Their budgets are not kernel helper counters.\n- IR-718 specifies\
    \ this prerequisite; IR-719 owns subsequent production kernel\n  and QSV integration.\
    \ QSL-503's generic Decimal/full-size supplied path awaits\n  the reviewed contract,\
    \ implemented qualification and coherent released heads.\n- The helper's maintained\
    \ dependency provides the Large representation's\n  borrowed magnitude iterator\
    \ `BigInt::iter_u32_digits`. The inline stream uses\n  the fixed-width unsigned\
    \ magnitude specified above and preserves the cache\n  observations of [FR-371](./FR-371-bounded-integer-representation-refinement.md)"
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: ".\n  The new helper preserves `no_std` and\n  `#![forbid(unsafe_code)]`;\
    \ it adds no dependency on another ecosystem crate."
- id: FR-372
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: context_only
  excerpt: 'SPEC DRAFT. Implementation, mathematical/mechanism qualification and all
    runtime

    controls are PLANNED/UNRUN. No numeric default, reduced Decimal safe profile,

    allocator wall-clock guarantee or full-size downstream success is claimed.'
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: context_only
  excerpt: "1. Check interval `[0,100]` with scales `2,2` and supplied `(1,0)`;\n\
    \   independently calculate the lifted coefficient 100 and exact upper equality.\n\
    \   Repeat at exact lower equality with `[100,200]`, then at adjacent outside\n\
    \   values, signed endpoints, zero and normalized scale beyond `smax`. Use\n \
    \  all six rounding spellings and equivalent representations `(1,0)` and\n   `(100,2)`,\
    \ observing that retained representations remain unchanged."
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: context_only
  excerpt: "2. Include a nonzero-discarded-remainder boundary such as `[0,99]`, scales\n\
    \   `2,2`, `(1,0)` and a negative counterpart. For a lower-bound remainder\n \
    \  control, check `[199,299]`, scales `2,2`, `(1,0)`: the divided lower\n   endpoint's\
    \ quotient is 1 with a nonzero remainder, but the lifted coefficient\n   100 is\
    \ below 199 and membership is false. Include its signed counterpart.\n   Calculate\
    \ the expected result independently with exact rational arithmetic\n   on bounded\
    \ fixtures. A mutant ignoring the remainder flag or negative sign\n   reversal\
    \ must fail these comparisons."
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "3. Independently choose admitted `B,S` and compute the FR-372 scratch\
    \ and\n   limb-step bounds. Include an endpoint requiring every available division\n\
    \   pass, coefficient equality requiring the full comparison, native 32-bit\n\
    \   limb crossings, and a shift exceeding the endpoint's decimal length.\n   Exercise\
    \ actual inline and Large coefficient/endpoint paths through the\n   production\
    \ helper. Include inline zero and i128 MIN/MAX and positive and\n   negative Large\
    \ magnitudes beyond i128, with limb crossings on both paths.\n   Prepare independent\
    \ expected limbs and membership oracles before observa"
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "tion;\n   a maintained BigInt iterator may serve as a stream oracle prepared\
    \ outside\n   the observed interval, not as a replacement for the production inline\
    \ path.\n   Observe each actual stream's least-significant-first limbs with no\
    \ allocation,\n   at most four inline limbs, and initially empty inline caches\
    \ remaining empty.\n   Observe production scratch requests and limb steps through\
    \ a seam that\n   does not replace helper arithmetic. Assert the formula bound,\
    \ one reusable\n   reservation, no scale-sized scratch and early quotient-zero\
    \ termination."
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "4. Deny the actual fallible scratch reservation on a small otherwise-valid\n\
    \   fixture, including the shifted upper-equality fixture from step 1. Compare\n\
    \   its typed allocation request with the observed request, including additional\n\
    \   elements, element width and layout bytes. Repeat denial through actual inline\n\
    \   and Large production helper paths with independently prepared valid fixtures;\n\
    \   confirm that denial does not initialize an initially empty inline cache or\n\
    \   construct a BigInt image. Stream traversal requests no allocation; this does\n\
    \   not assert that the helper's one fallible sc"
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "ratch request is allocation-free.\n   Separately exercise checked native\n\
    \   layout/capacity overflow through the production checked-layout calculation\n\
    \   with boundary-sized measure inputs, without constructing an impossibly\n \
    \  large Decimal; this is not an allocator-denial fixture. Remove the\n   denial\
    \ and assert the same completed membership as a fresh call. Observe\n   scratch\
    \ release and absence of a completed/partially accepted result on\n   refusal.\
    \ Keep unsafe allocator instrumentation outside this forbidding crate;\n   use\
    \ a maintained safe test seam rather than a copied allocator implemen"
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: tation.
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: context_only
  excerpt: "5. Pre-cancel the original caller handle with each supported cause and\
    \ observe\n   no scratch request. At deterministic observed polling boundaries,\
    \ cancel\n   immediately after successful reservation and during the longest admitted\n\
    \   division/comparison path. Assert the original cause, no following limb step,\n\
    \   no completed result, and scratch release. A fresh-handle mutant must fail.\n\
    \   Do not substitute wall-clock timing assertions for the polling oracle."
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: context_only
  excerpt: "6. Through the actual supplied-admission caller, independently enumerate\
    \ the\n   FR-109 event order and N for a fixture with Decimal membership and another\n\
    \   argument sharing an initially spent budget. Run N-1 and N. Observe denial\n\
    \   before the real denied event/helper, successful cumulative spend and the\n\
    \   owning phase/locus. Repeat native scratch denial, capacity refusal and\n \
    \  cancellation through that caller; assert no descriptor is partially admitted,\n\
    \   no helper limb debit or fifth logical event appears, and an external\n   evaluator\
    \ meter remains unchanged. Exercise replay's final mem"
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: context_only
  excerpt: "bership\n   through the same admission owner and retain conversion ownership\
    \ where\n   that shared operation is initiated by conversion."
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "7. Analyze the final production helper, inline fixed-width unsigned-magnitude\n\
    \   stream and maintained Large borrowed iterator against\n   the Euclidean decomposition,\
    \ remainder accumulation, carry width, sign\n   reversal, checked scratch layout\
    \ and per-step polling claims. Account for\n   every allocation and every input-sized\
    \ loop. Record requested layout bytes\n   separately from unknown allocator rounding/overhead,\
    \ and arithmetic work\n   response separately from indivisible allocator-call\
    \ elapsed time. Refuse\n   qualification if any premise is absent; a small source-predicted\
    \ event count\n   o"
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: r a bool-only helper is insufficient evidence.
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: 'Membership agrees with the independent mathematical oracle, preserving
    all

    accepted forms and inclusive boundaries. Observed helper work and scratch

    requests satisfy the symbolic admitted envelope on both actual representation

    paths. Inline streams yield at most four least-significant-first u32 limbs,

    preserve the full magnitude of i128 MIN, and leave initially empty caches empty.

    Neither stream allocates or constructs a BigInt image; the helper retains its

    one fallible scratch request. Actual allocation denial,

    checked capacity refusal and original-handle cancellation have distinct typed

    caus'
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: 'es; stopped calls release scratch and expose no accepted result. Integrated

    N/N-1 controls preserve existing phase ownership, cumulative spend and event

    order without evaluator debit. The qualification record establishes the

    production arithmetic/storage/polling mechanism and states allocator limitations.'
- id: TC-920
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: context_only
  excerpt: 'PLANNED/UNRUN. IR-719 owns executable implementation and qualification.
    The

    integrated phase controls await the owning QSV/QSL interfaces; no kernel-only

    test or source inspection claims QSL-503 full-size acceptance.'
- id: src/integer.rs
  path: src/integer.rs
  role: context_only
  excerpt: '#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]

    pub struct Integer(BigInt);'
- id: src/decimal_membership.rs
  path: src/decimal_membership.rs
  role: context_only
  excerpt: "    fn copy_endpoint(&mut self, endpoint: &Integer) -> Result<(), DecimalMembershipFailure>\
    \ {\n        let mut digits = endpoint.as_big().iter_u32_digits();\n        if\
    \ self.scratch.is_empty() {\n            let (limbs, _layout) = scratch_layout(self.magnitude_bits)?;\n\
    \            poll(self.cancel)?;\n            self.scratch\n                .try_reserve_exact(limbs)\n\
    \                .map_err(|error| match error.kind() {\n                    TryReserveErrorKind::CapacityOverflow\
    \ => DecimalMembershipFailure::Capacity {\n                        magnitude_bits:\
    \ self.magnitude_bits,\n                    },"
- id: src/decimal_membership.rs
  path: src/decimal_membership.rs
  role: context_only
  excerpt: "\n                    TryReserveErrorKind::AllocError { layout, .. } =>\
    \ {\n                        DecimalMembershipFailure::Allocation {\n        \
    \                    additional_elements: limbs,\n                           \
    \ element_width: size_of::<u32>(),\n                            layout_bytes:\
    \ layout.size(),\n                        }\n                    }\n         \
    \       })?;\n            self.observation.reserved();\n            poll(self.cancel)?;\n\
    \            for _ in 0..limbs {\n                poll(self.cancel)?;\n      \
    \          // The fallible reservation has admitted every push in this loop"
- id: src/decimal_membership.rs
  path: src/decimal_membership.rs
  role: context_only
  excerpt: ".\n                self.scratch.push(digits.next().unwrap_or(0));\n  \
    \              self.observation.step(LimbStep::Copy);\n            }\n       \
    \ } else {\n            for limb in &mut self.scratch {\n                poll(self.cancel)?;\n\
    \                *limb = digits.next().unwrap_or(0);\n                self.observation.step(LimbStep::Copy);\n\
    \            }\n        }\n        Ok(())\n    }"
```

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Assessment

| Responsibility | Owner | Class | Assumed / guaranteed contract |
| --- | --- | --- | --- |
| inline magnitude, Large borrowing, fallible scratch, counted steps and original-handle polls | quire-exact | core | guaranteed only by future FR-372/TC-920 qualification |
| retained Large iterator mechanism | maintained num-bigint dependency | infrastructure | assumption until TC-920 step7 qualifies final dependency |
| mathematical Decimal membership rule | QSpec FR-140 | core external | assumed named authoritative contract; oracle controls verify adherence |
| cumulative budget, initiating event, failure phase/locus and descriptor custody | QSV FR-109 / QSL FR-321 | core external | explicitly pending owning integration controls |

Context: consumer -> kernel helper -> inline fixed-width stream or retained dependency iterator; helper -> fallible scratch allocator, borrowing original Cancel. The stream introduces no semantic meter, additional logical event or evaluator charge. as_big/Hash/Debug remain their public materialization contract. No copy of dependency code or consumer algorithm is added; kernel-only evidence cannot establish full-size downstream acceptance.

## Verdict

PASS: the kernel, dependency and consumer responsibilities stay separated. This is a specification-diff review, not implementation or proof acceptance.
