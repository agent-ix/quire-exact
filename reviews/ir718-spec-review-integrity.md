---
id: SR-5303
title: integrity review of IR-718 Decimal membership draft
type: SpecReview
analysis: integrity
scope: agent-ix/quire-exact@c9045f1b1a8e4605895d30ae2943a6ba868e6b65; spec/functional/FR-372-fallible-bounded-decimal-membership.md,
  spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
review_set: subset
---

## Summary

Ticket: IR-718. The transformation is sound: write endpoint magnitude as 10^k*q+r with 0<=r<10^k. Comparing coefficient magnitude against q decides strict ordering; equality depends on r=0, and the negative same-sign order reverses. k=0 gives q=endpoint and an empty all-zero remainder set. Zero magnitude bit length one makes L>=1; sign/zero comparisons bypass division. One endpoint requires L copy, d*L division and L comparison steps; two endpoints fit 2*(d+2)*L. Early-zero status is accumulated inside division rather than by a hidden scan.

## Verdict

**PASS** — No defect found in this method’s frozen specification scope; implementation qualification remains PLANNED/UNRUN.

## Assurance Context

No applicable AssuranceProfile found in the reviewed repository. No active exception applied. Private authoritative domain context was read locally; this artifact contains no private excerpts. Source and runtime implementation evidence is outside this spec-only review.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined Scope

```yaml
scope:
- id: FR-372-unit-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '## Description'
- id: FR-372-unit-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: 'When a consumer checks an already-constructed Decimal against a well-formed

    Decimal type under finite admitted input bounds, the exact kernel SHALL return

    fallible value membership through its authoritative bounded helper, preserving

    the caller''s cancellation handle and distinguishing resource failure from

    nonmembership.'
- id: FR-372-unit-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '## Inputs'
- id: FR-372-unit-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "- A borrowed `DecimalType` and borrowed `Decimal`, including its existing\n  normalized coefficient `c` and scale `s`.\n- The caller's original borrowed `Cancel` handle.\n- The consumer's admitted magnitude-bit and scale-shift bounds, established\n  before the call as described below. These are qualification premises, not\n  newly selected numeric defaults or extra helper work allowances."
- id: FR-372-unit-5
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '## Outputs'
- id: FR-372-unit-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "- Completed membership, `true` or `false`.\n- Or a typed failure distinguishing checked capacity/size failure, actual\n  temporary-allocation denial, and cancellation carrying `CancelCause`."
- id: FR-372-unit-7
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '## Behavior'
- id: FR-372-unit-8
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '### Membership and interface'
- id: FR-372-B1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "1. The kernel SHALL expose a fallible `DecimalType::try_contains` operation\n   borrowing the Decimal and original cancellation handle, with a completed\n   Boolean result or typed `DecimalMembershipFailure`. This operation SHALL\n   leave both the type and Decimal representations unchanged."
- id: FR-372-B2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "2. For a type with inclusive coefficient bounds `lo` and `hi` and scale bounds\n   `smin` and `smax`, completed membership SHALL be exactly\n   `s* = max(s, smin) <= smax` and `lo <= c × 10^(s* - s) <= hi`.\n   Membership SHALL remain independent of the type's rounding spelling and\n   the Decimal's pre-normalized representation provenance."
- id: FR-372-B3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "3. If `s* > smax`, the operation SHALL complete with nonmembership after\n   polling cancellation and without requesting scratch storage. Sign and zero\n   comparisons SHALL use borrowed retained data without materializing absolute\n   values. Existing bool-only membership, Decimal construction/normalization,\n   ordering, placement and evaluation are outside this operation's scope."
- id: FR-372-unit-13
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '### Finite work and storage'
- id: FR-372-unit-14
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: 'Let `b = max(bits(c), bits(lo), bits(hi))`, with zero''s magnitude bit length

    one, and let `k = s* - s` on the scale-admissible path. The consumer establishes

    finite admitted maxima `B >= b` and `S >= k` for every operand and the actual

    shift, including both type endpoints. Define:'
- id: FR-372-unit-15-part-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '| Measure | Bound and unit |

    | --- | --- |

    | Scratch length | `L = ceil(b / 32)` unsigned 32-bit elements. |

    | Division passes per endpoint | `d = min(k, b)` passes, stopping earlier when the quotient becomes zero. |

    | Helper arithmetic | At most `2 × (d + 2) × L` fixed-width limb steps for both endpoint comparisons. |

    | Temporary storage | At most one live scratch reservation requesting `4 × L` bytes, plus constant-sized scalar/iterator state. |'
- id: FR-372-unit-15-part-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '| Consumer envelope | Substitute `B` and `S`: `L_B = ceil(B / 32)`, at most `2 × (min(S, B) + 2) × L_B` limb steps and a `4 × L_B`-byte scratch request. |'
- id: FR-372-unit-16
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: 'A limb step is one scratch copy/initialization, one quotient/remainder update

    of a scratch limb by ten, or one comparison of scratch and borrowed coefficient

    limbs. These are proof measures for the bounded kernel helper, not semantic

    charges or supplied-admission logical events. Fixed scalar sign/scale checks,

    checked layout arithmetic and cancellation polls accompany these steps;

    there is at most one fallible allocation request. No input-sized work is hidden

    inside a scalar check or iterator operation.'
- id: FR-372-B4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "4. The kernel SHALL enforce the above envelope using one reusable\n   fallibly-reserved scratch buffer of `u32` magnitude limbs and borrowed\n   magnitude-limb iterators. The operation SHALL obtain magnitude limbs from\n   the maintained integer dependency's allocation-free borrowed iterator,\n   rather than allocating an absolute value, radix string, power of ten,\n   cloned BigInt or BigInt arithmetic result."
- id: FR-372-B5
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "5. For a same-sign nonzero endpoint comparison, the helper SHALL copy the\n   endpoint's magnitude into the scratch buffer, padding only within `L`.\n   It SHALL divide that scratch magnitude by ten at most `k` times, stopping\n   when the quotient becomes zero, and retain whether any discarded remainder\n   was nonzero. Each pass SHALL update its zero/nonzero status within its limb\n   steps, without an additional unbounded scan."
- id: FR-372-B6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "6. If `q = floor(abs(endpoint) / 10^k)`, magnitude comparison SHALL compare\n   `abs(c)` with `q`. When those magnitudes are equal, the shifted magnitude\n   SHALL equal the endpoint exactly when every discarded remainder was zero;\n   otherwise it is smaller. Negative same-sign comparisons SHALL reverse the\n   magnitude ordering. The two inclusive membership comparisons SHALL reuse\n   the same scratch allocation."
- id: FR-372-unit-21
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: 'The division invariant is Euclidean decomposition of the endpoint magnitude.

    Since `10 >= 2`, a magnitude of at most `b` bits reaches quotient zero after

    at most `b` divisions. Each division step uses a `u64` intermediate

    `carry × 2^32 + limb`, with `carry < 10`; its maximum is below `10 × 2^32`,

    so quotient/remainder by ten needs no input-sized arithmetic or allocation.

    One endpoint uses at most `L` copy steps, `d × L` division steps and `L`

    comparison steps. The borrowed coefficient and endpoints remain authoritative;

    the scratch representation exists only during this kernel call.'
- id: FR-372-unit-22
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '### Refusal and cancellation'
- id: FR-372-B7
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "7. Before requesting scratch storage, the kernel SHALL check element-count,\n   native layout-byte and capacity conversions without wrapping or truncation.\n   A failed conversion or layout calculation SHALL return typed capacity/size\n   failure, distinct from an actual allocator denial."
- id: FR-372-B8
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "8. The scratch buffer SHALL request capacity through fallible reservation before\n   writing its elements. An actual allocator denial SHALL return typed\n   allocation failure retaining the actual requested additional-element count,\n   element width and requested layout bytes. Failure SHALL NOT become\n   nonmembership, a configured WorkUnits refusal or a panic."
- id: FR-372-B9
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "9. The helper SHALL poll the caller's original `Cancel` before the allocation\n   request, immediately after its successful return, before each limb step,\n   and before publishing a completed result. When a poll observes cancellation,\n   the helper SHALL return typed cancellation with that handle's cause before\n   another limb step or completed result. Pre-cancelled calls SHALL request no\n   scratch allocation. A cancelled call SHALL release its owned scratch."
- id: FR-372-unit-27-part-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: 'The arithmetic response gap is at most one fixed-width limb step between polls.

    Allocation and release are indivisible allocator calls; polling cannot interrupt

    them. The storage bound describes requested live scratch layout bytes, not

    allocator-internal rounding, bookkeeping or resident memory. This contract

    does not claim a wall-clock bound on allocator calls or scheduler delays.

    If allocation itself fails, that observed failure is reported directly; no

    successful-result cancellation boundary is crossed. The operation has no'
- id: FR-372-unit-27-part-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: semantic meter, replacement cancellation handle or partial admission output.
- id: FR-372-unit-28
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '### Consumer boundary and qualification'
- id: FR-372-B10
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "10. A supplied-admission consumer SHALL invoke this authoritative helper under\n    the actual initiating logical event and cumulative phase budget owned by\n    [QSV FR-109](ix://agent-ix/quire-semantic-value/FR-109). It SHALL establish\n    the admitted `B` and `S` premises and native scratch envelope before claiming\n    that Decimal membership is qualified. Helper limb steps SHALL NOT create\n    an additional traversal event, per-limb supplied budget debit or evaluator\n    charge."
- id: FR-372-B11
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "11. The consumer SHALL preserve the helper's typed nonmembership, allocation,\n    capacity and cancellation distinctions and the actual membership locus.\n    A stopped supplied admission SHALL publish no partially accepted descriptor.\n    A replay-triggered final membership check SHALL retain its admission owner;\n    a conversion-triggered shared operation retains its initiating conversion\n    owner without duplicate phase debit."
- id: FR-372-B12
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "12. Before claiming implementation acceptance, the owning implementation SHALL\n    demonstrate the mathematical invariant, every reachable scratch request,\n    the limb-step bound and original-handle polling against the final maintained\n    dependency and production helper. A source formula alone SHALL NOT qualify\n    the released bool-only helper or establish downstream full-size acceptance."
- id: FR-372-unit-33
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '## Acceptance Criteria'
- id: FR-372-AC-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Completed results match the membership formula for inclusive lower/upper equality, adjacent outsiders, negative and zero values, scale refusal, all rounding spellings and equivalent retained representations; neither representation is changed. The `[0,100]`, scale bounds `2,2`, `(1,0)` fixture completes true at the exact upper bound.
- id: FR-372-AC-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Production helper observations on admitted `B,S` fixtures stay within the one-request `4 × L_B`-byte scratch envelope and `2 × (min(S,B)+2) × L_B` limb-step bound, including a longest-pass endpoint fixture and a large shift that terminates when quotient becomes zero. Borrowed dependency iterators and all scalar steps contain no input-sized hidden allocation or work.
- id: FR-372-AC-3
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Denying the actual scratch reservation reports its actual native request as typed allocation failure. A separate checked-capacity failure retains its own classification. Removing the allocation denial yields the same membership as a fresh call, with no partial accepted result after either refusal.
- id: FR-372-AC-4
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: A pre-cancelled original handle causes no scratch request. Cancellation observed during the longest admitted division/comparison path returns the original cause before another limb step or result; cancellation observed immediately after successful reservation prevents scratch work. Neither failure is classified as nonmembership or work exhaustion.
- id: FR-372-AC-5
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: An integrated supplied Decimal fixture with independently enumerated N owning logical events succeeds at N and denies its actual next event at N-1 before entering that event's helper. Its already-spent budget and original cancellation are retained across arguments, with no additional helper event, per-limb debit, evaluator meter change or partial descriptor. Allocation/capacity/cancellation causes retain their supplied-admission locus.
- id: FR-372-AC-6
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: Final implementation qualification establishes the Euclidean/remainder and sign invariants, exact reservation boundary, checked layout conversions, allocation-free borrowed dependency iterator and stated arithmetic polling gap. Its record distinguishes requested storage from allocator overhead and work latency from unbounded allocator elapsed time.
- id: FR-372-unit-34
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '## Dependencies'
- id: FR-372-unit-35-part-1
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "- [TC-920](../test-cases/TC-920-fallible-decimal-membership-boundaries.md)\n  defines the falsifying controls.\n- [QSpec FR-140](ix://agent-ix/quire-specification/FR-140) owns Decimal value\n  semantics. This requirement paraphrases only the existing membership formula;\n  it creates no Decimal form or rounding behavior.\n- [QSV FR-109](ix://agent-ix/quire-semantic-value/FR-109) owns cumulative\n  supplied logical work, construction custody and typed phase/locus preservation.\n  [QSL FR-321](ix://agent-ix/quire-spec-language/FR-321) owns supplied consumer"
- id: FR-372-unit-35-part-2
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "  admission. IR-714 and QSL-681 align phase interfaces; QSL owns public phase\n  projection and numeric defaults. Their budgets are not kernel helper counters.\n- IR-718 specifies this prerequisite; IR-719 owns subsequent production kernel\n  and QSV integration. QSL-503's generic Decimal/full-size supplied path awaits\n  the reviewed contract, implemented qualification and coherent released heads."
- id: FR-372-dependencies-source
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: "- Source inspected: exact `762b5fcce139980db259dde908d2b66214c852e3`,\n  `DecimalType::contains`, `compare_shifted`, `Integer::decimal_digits`,\n  `Cancel::poll`, and locked num-bigint 0.4.8 `BigInt::iter_u32_digits`.\n  The released membership helper is bool-only and its digit/equal-digit\n  branches allocate. The new helper preserves `no_std` and\n  `#![forbid(unsafe_code)]`; it adds no dependency on another ecosystem crate."
- id: FR-372-unit-37
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: '## Status'
- id: FR-372-unit-38
  path: spec/functional/FR-372-fallible-bounded-decimal-membership.md
  role: examined
  excerpt: 'SPEC DRAFT. Implementation, mathematical/mechanism qualification and all runtime

    controls are PLANNED/UNRUN. No numeric default, reduced Decimal safe profile,

    allocator wall-clock guarantee or full-size downstream success is claimed.'
- id: TC-920-unit-1
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: '## Description'
- id: TC-920-unit-2
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: 'Verify [FR-372](../functional/FR-372-fallible-bounded-decimal-membership.md)

    through the real fallible kernel operation and its actual supplied-admission

    caller. Prepare types, Decimal normalization and observer/cancellation handles

    before observing the helper. Do not replace membership logic, copy the Decimal

    algorithm into QSV or infer an expected count from the counter under test.'
- id: TC-920-unit-3
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: '## Test Procedure'
- id: TC-920-procedure-1
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "1. Check interval `[0,100]` with scales `2,2` and supplied `(1,0)`;\n   independently calculate the lifted coefficient 100 and exact upper equality.\n   Repeat at exact lower equality with `[100,200]`, then at adjacent outside\n   values, signed endpoints, zero and normalized scale beyond `smax`. Use\n   all six rounding spellings and equivalent representations `(1,0)` and\n   `(100,2)`, observing that retained representations remain unchanged."
- id: TC-920-procedure-2
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "2. Include a nonzero-discarded-remainder boundary such as `[0,99]`, scales\n   `2,2`, `(1,0)` and a negative counterpart. For a lower-bound remainder\n   control, check `[199,299]`, scales `2,2`, `(1,0)`: the divided lower\n   endpoint's quotient is 1 with a nonzero remainder, but the lifted coefficient\n   100 is below 199 and membership is false. Include its signed counterpart.\n   Calculate the expected result independently with exact rational arithmetic\n   on bounded fixtures. A mutant ignoring the remainder flag or negative sign\n   reversal must fail these comparisons."
- id: TC-920-procedure-3
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "3. Independently choose admitted `B,S` and compute the FR-372 scratch and\n   limb-step bounds. Include an endpoint requiring every available division\n   pass, coefficient equality requiring the full comparison, native 32-bit\n   limb crossings, and a shift exceeding the endpoint's decimal length.\n   Observe production scratch requests and limb steps through a seam that\n   does not replace helper arithmetic. Assert the formula bound, one reusable\n   reservation, no scale-sized scratch and early quotient-zero termination."
- id: TC-920-procedure-4-part-1
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "4. Deny the actual fallible scratch reservation on a small otherwise-valid\n   fixture, including the shifted upper-equality fixture from step 1. Compare\n   its typed allocation request with the observed request, including additional\n   elements, element width and layout bytes. Separately exercise checked native\n   layout/capacity overflow through the production checked-layout calculation\n   with boundary-sized measure inputs, without constructing an impossibly\n   large Decimal; this is not an allocator-denial fixture. Remove the"
- id: TC-920-procedure-4-part-2
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "   denial and assert the same completed membership as a fresh call. Observe\n   scratch release and absence of a completed/partially accepted result on\n   refusal. Keep unsafe allocator instrumentation outside this forbidding crate;\n   use a maintained safe test seam rather than a copied allocator implementation."
- id: TC-920-procedure-5
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "5. Pre-cancel the original caller handle with each supported cause and observe\n   no scratch request. At deterministic observed polling boundaries, cancel\n   immediately after successful reservation and during the longest admitted\n   division/comparison path. Assert the original cause, no following limb step,\n   no completed result, and scratch release. A fresh-handle mutant must fail.\n   Do not substitute wall-clock timing assertions for the polling oracle."
- id: TC-920-procedure-6-part-1
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "6. Through the actual supplied-admission caller, independently enumerate the\n   FR-109 event order and N for a fixture with Decimal membership and another\n   argument sharing an initially spent budget. Run N-1 and N. Observe denial\n   before the real denied event/helper, successful cumulative spend and the\n   owning phase/locus. Repeat native scratch denial, capacity refusal and\n   cancellation through that caller; assert no descriptor is partially admitted,\n   no helper limb debit or fifth logical event appears, and an external"
- id: TC-920-procedure-6-part-2
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "   evaluator meter remains unchanged. Exercise replay's final membership\n   through the same admission owner and retain conversion ownership where\n   that shared operation is initiated by conversion."
- id: TC-920-procedure-7
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: "7. Analyze the final production helper and maintained borrowed iterator against\n   the Euclidean decomposition, remainder accumulation, carry width, sign\n   reversal, checked scratch layout and per-step polling claims. Account for\n   every allocation and every input-sized loop. Record requested layout bytes\n   separately from unknown allocator rounding/overhead, and arithmetic work\n   response separately from indivisible allocator-call elapsed time. Refuse\n   qualification if any premise is absent; a small source-predicted event count\n   or a bool-only helper is insufficient evidence."
- id: TC-920-unit-12
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: '## Expected Results'
- id: TC-920-unit-13-part-1
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: 'Membership agrees with the independent mathematical oracle, preserving all

    accepted forms and inclusive boundaries. Observed helper work and scratch

    requests satisfy the symbolic admitted envelope. Actual allocation denial,

    checked capacity refusal and original-handle cancellation have distinct typed

    causes; stopped calls release scratch and expose no accepted result. Integrated

    N/N-1 controls preserve existing phase ownership, cumulative spend and event

    order without evaluator debit. The qualification record establishes the'
- id: TC-920-unit-13-part-2
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: production arithmetic/storage/polling mechanism and states allocator limitations.
- id: TC-920-unit-14
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: '## Status'
- id: TC-920-unit-15
  path: spec/test-cases/TC-920-fallible-decimal-membership-boundaries.md
  role: examined
  excerpt: 'PLANNED/UNRUN. IR-719 owns executable implementation and qualification. The

    integrated phase controls await the owning QSV/QSL interfaces; no kernel-only

    test or source inspection claims QSL-503 full-size acceptance.'
```
