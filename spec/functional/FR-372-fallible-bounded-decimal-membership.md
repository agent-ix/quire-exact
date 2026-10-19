---
id: FR-372
title: "Fallible bounded Decimal supplied membership"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-semantic-value/FR-109
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-321
    type: references
---
# FR-372: Fallible bounded Decimal supplied membership

## Description

When a consumer checks an already-constructed Decimal against a well-formed
Decimal type under finite admitted input bounds, the exact kernel SHALL return
fallible value membership through its authoritative bounded helper, preserving
the caller's cancellation handle and distinguishing resource failure from
nonmembership.

## Inputs

- A borrowed `DecimalType` and borrowed `Decimal`, including its existing
  normalized coefficient `c` and scale `s`.
- The caller's original borrowed `Cancel` handle.
- The consumer's admitted magnitude-bit and scale-shift bounds, established
  before the call as described below. These are qualification premises, not
  newly selected numeric defaults or extra helper work allowances.

## Outputs

- Completed membership, `true` or `false`.
- Or a typed failure distinguishing checked capacity/size failure, actual
  temporary-allocation denial, and cancellation carrying `CancelCause`.

## Behavior

### Membership and interface

1. The kernel SHALL expose a fallible `DecimalType::try_contains` operation
   borrowing the Decimal and original cancellation handle, with a completed
   Boolean result or typed `DecimalMembershipFailure`. This operation SHALL
   leave both the type and Decimal representations unchanged.
2. Completed membership SHALL conform to the authoritative Decimal value
   membership rule in [QSpec FR-140](ix://agent-ix/quire-specification/FR-140).
3. When that owning rule rejects the normalized scale, the operation SHALL
   complete with nonmembership after
   polling cancellation and without requesting scratch storage. Sign and zero
   comparisons SHALL use borrowed retained data without materializing absolute
   values. Existing bool-only membership, Decimal construction/normalization,
   ordering, placement and evaluation are outside this operation's scope.

### Finite work and storage

Let `lo` and `hi` denote the type's borrowed endpoint coefficients, and let
`b = max(bits(c), bits(lo), bits(hi))`, with zero's magnitude bit length one.
Let `k` denote the nonnegative coefficient-alignment shift requested by the
authoritative membership rule on its scale-admissible path. The consumer establishes
finite admitted maxima `B >= b` and `S >= k` for every operand and the actual
shift, including both type endpoints. Define:

| Measure | Bound and unit |
| --- | --- |
| Scratch length | `L = ceil(b / 32)` unsigned 32-bit elements. |
| Division passes per endpoint | `d = min(k, b)` passes, stopping earlier when the quotient becomes zero. |
| Helper arithmetic | At most `2 × (d + 2) × L` fixed-width limb steps for both endpoint comparisons. |
| Temporary storage | At most one live scratch reservation requesting `4 × L` bytes, plus constant-sized scalar/iterator state. |
| Consumer envelope | Substitute `B` and `S`: `L_B = ceil(B / 32)`, at most `2 × (min(S, B) + 2) × L_B` limb steps and a `4 × L_B`-byte scratch request. |

A limb step is one scratch copy/initialization, one quotient/remainder update
of a scratch limb by ten, or one comparison of scratch and borrowed coefficient
limbs. These are proof measures for the bounded kernel helper, not semantic
charges or supplied-admission logical events. Fixed scalar sign/scale checks,
checked layout arithmetic and cancellation polls accompany these steps;
there is at most one fallible allocation request. No input-sized work is hidden
inside a scalar check or iterator operation.

4. The kernel SHALL enforce the above envelope using one reusable
   fallibly-reserved scratch buffer of `u32` magnitude limbs and borrowed
   magnitude-limb streams. For an inline `i128`, the operation SHALL obtain
   its unsigned magnitude using `unsigned_abs`, including `i128::MIN`, and
   yield at most four `u32` limbs in least-significant-first order using
   constant-sized fixed-width state; zero yields no limbs. For a Large
   arbitrary-precision representation, the operation SHALL obtain magnitude
   limbs from the maintained integer dependency's allocation-free borrowed
   iterator. Neither stream SHALL initialize or change a materialization cache,
   construct a BigInt image, or request allocation. The operation SHALL NOT
   allocate an absolute value, radix string, power of ten, cloned BigInt or
   BigInt arithmetic result. Each yielded limb SHALL participate in the existing
   scratch-copy or coefficient-comparison limb step, without an additional
   input-sized traversal.
5. For a same-sign nonzero endpoint comparison, the helper SHALL copy the
   endpoint's magnitude into the scratch buffer, padding only within `L`.
   It SHALL divide that scratch magnitude by ten at most `k` times, stopping
   when the quotient becomes zero, and retain whether any discarded remainder
   was nonzero. Each pass SHALL update its zero/nonzero status within its limb
   steps, without an additional unbounded scan.
6. If `q = floor(abs(endpoint) / 10^k)`, magnitude comparison SHALL compare
   `abs(c)` with `q`. When those magnitudes are equal, the shifted magnitude
   SHALL equal the endpoint exactly when every discarded remainder was zero;
   otherwise it is smaller. Negative same-sign comparisons SHALL reverse the
   magnitude ordering. The two inclusive membership comparisons SHALL reuse
   the same scratch allocation.

The division invariant is Euclidean decomposition of the endpoint magnitude.
Since `10 >= 2`, a magnitude of at most `b` bits reaches quotient zero after
at most `b` divisions. Each division step uses a `u64` intermediate
`carry × 2^32 + limb`, with `carry < 10`; its maximum is below `10 × 2^32`,
so quotient/remainder by ten needs no input-sized arithmetic or allocation.
One endpoint uses at most `L` copy steps, `d × L` division steps and `L`
comparison steps. The borrowed coefficient and endpoints remain authoritative;
the scratch representation exists only during this kernel call.

### Refusal and cancellation

7. Before requesting scratch storage, the kernel SHALL check element-count,
   native layout-byte and capacity conversions without wrapping or truncation.
   A failed conversion or layout calculation SHALL return typed capacity/size
   failure, distinct from an actual allocator denial.
8. The scratch buffer SHALL request capacity through fallible reservation before
   writing its elements. An actual allocator denial SHALL return typed
   allocation failure retaining the actual requested additional-element count,
   element width and requested layout bytes. Failure SHALL NOT become
   nonmembership, a configured WorkUnits refusal or a panic.
9. The helper SHALL poll the caller's original `Cancel` before the allocation
   request, immediately after its successful return, before each limb step,
   and before publishing a completed result. When a poll observes cancellation,
   the helper SHALL return typed cancellation with that handle's cause before
   another limb step or completed result. Pre-cancelled calls SHALL request no
   scratch allocation. A cancelled call SHALL release its owned scratch.

The arithmetic response gap is at most one fixed-width limb step between polls.
Allocation and release are indivisible allocator calls; polling cannot interrupt
them. The storage bound describes requested live scratch layout bytes, not
allocator-internal rounding, bookkeeping or resident memory. This contract
does not claim a wall-clock bound on allocator calls or scheduler delays.
If allocation itself fails, that observed failure is reported directly; no
successful-result cancellation boundary is crossed. The operation has no
semantic meter, replacement cancellation handle or partial admission output.

### Consumer boundary and qualification

10. A supplied-admission consumer SHALL invoke this authoritative helper under
    the actual initiating logical event and cumulative phase budget owned by
    [QSV FR-109](ix://agent-ix/quire-semantic-value/FR-109). It SHALL establish
    the admitted `B` and `S` premises and native scratch envelope before claiming
    that Decimal membership is qualified. Helper limb steps SHALL NOT create
    an additional traversal event, per-limb supplied budget debit or evaluator
    charge.
11. The consumer SHALL preserve the helper's typed nonmembership, allocation,
    capacity and cancellation distinctions and the actual membership locus.
    A stopped supplied admission SHALL publish no partially accepted descriptor.
    A replay-triggered final membership check SHALL retain its admission owner;
    a conversion-triggered shared operation retains its initiating conversion
    owner without duplicate phase debit.
12. Before claiming implementation acceptance, the owning implementation SHALL
    demonstrate the mathematical invariant, every reachable scratch request,
    the limb-step bound and original-handle polling against the final maintained
    dependency and production helper. A source formula alone SHALL NOT qualify
    the released bool-only helper or establish downstream full-size acceptance.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-372-AC-1 | Completed results match the authoritative membership rule for inclusive lower/upper equality, adjacent outsiders, negative and zero values, scale refusal, all rounding spellings and equivalent retained representations; neither representation is changed. The `[0,100]`, scale bounds `2,2`, `(1,0)` fixture completes true at the exact upper bound. | Test |
| FR-372-AC-2 | Production helper observations on admitted `B,S` fixtures stay within the one-request `4 × L_B`-byte scratch envelope and `2 × (min(S,B)+2) × L_B` limb-step bound, including a longest-pass endpoint fixture and a large shift that terminates when quotient becomes zero. Controls exercise actual inline and Large production paths, including zero, 32-bit limb crossings, i128 MIN/MAX and signed magnitudes beyond i128. Stream traversal requests no allocation and leaves cold inline caches empty; inline streams yield at most four least-significant-first u32 limbs. Both stream mechanisms and all scalar steps contain no input-sized hidden allocation or work. The helper retains its one fallible scratch request. | Test |
| FR-372-AC-3 | Denying the actual scratch reservation reports its actual native request as typed allocation failure. A separate checked-capacity failure retains its own classification. Removing the allocation denial yields the same membership as a fresh call, with no partial accepted result after either refusal. | Test |
| FR-372-AC-4 | A pre-cancelled original handle causes no scratch request. Cancellation observed during the longest admitted division/comparison path returns the original cause before another limb step or result; cancellation observed immediately after successful reservation prevents scratch work. Neither failure is classified as nonmembership or work exhaustion. | Test |
| FR-372-AC-6 | Final implementation qualification establishes the Euclidean/remainder and sign invariants, exact reservation boundary, checked layout conversions, allocation-free inline unsigned-magnitude stream and Large borrowed dependency iterator, absence of cache initialization or BigInt-image construction on those streams, and stated arithmetic polling gap. Its record distinguishes requested storage from allocator overhead and work latency from unbounded allocator elapsed time. | Analysis |

The integrated supplied Decimal control formerly FR-372-AC-5 is allocated to [QSV FR-122-AC-2](ix://agent-ix/quire-semantic-value/FR-122) for the supplied path and [QSL FR-394-AC-3](ix://agent-ix/quire-spec-language/FR-394) for evaluator effects. Both remain PLANNED for owner tests.

## Dependencies

- [TC-920](../test-cases/TC-920-fallible-decimal-membership-boundaries.md)
  defines the falsifying controls.
- [QSpec FR-140](ix://agent-ix/quire-specification/FR-140) owns Decimal value
  semantics and the membership rule invoked by this kernel helper.
- [QSV FR-109](ix://agent-ix/quire-semantic-value/FR-109) owns cumulative
  supplied logical work, construction custody and typed phase/locus preservation.
  [QSL FR-321](ix://agent-ix/quire-spec-language/FR-321) owns supplied consumer
  admission. IR-714 and QSL-681 align phase interfaces; QSL owns public phase
  projection and numeric defaults. Their budgets are not kernel helper counters.
- IR-718 specifies this prerequisite; IR-719 owns subsequent production kernel
  and QSV integration. QSL-503's generic Decimal/full-size supplied path awaits
  the reviewed contract, implemented qualification and coherent released heads.
- The helper's maintained dependency provides the Large representation's
  borrowed magnitude iterator `BigInt::iter_u32_digits`. The inline stream uses
  the fixed-width unsigned magnitude specified above and preserves the cache
  observations of [FR-371](./FR-371-bounded-integer-representation-refinement.md).
  The new helper preserves `no_std` and
  `#![forbid(unsafe_code)]`; it adds no dependency on another ecosystem crate.

## Status

SPEC DRAFT. Implementation, mathematical/mechanism qualification and all runtime
controls are PLANNED/UNRUN. No numeric default, reduced Decimal safe profile,
allocator wall-clock guarantee or full-size downstream success is claimed.
