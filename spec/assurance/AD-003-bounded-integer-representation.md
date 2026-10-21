---
id: AD-003
title: "Bounded integer representation and borrowed BigInt refinement"
type: ArchitectureDescription
status: proposed
owner: quire-exact-maintainer
system: quire-exact Integer representation and its mathematical observations
relationships:
  - target: "ix://agent-ix/quire-exact/FR-371"
    type: realizes
---
# AD-003: Bounded integer representation and borrowed BigInt refinement

## System Boundary

This decision covers the private storage representation of `quire-exact::Integer`, its exact refinement to mathematical integers, and the stable borrowed `as_big` interface. It preserves the public `Integer` API, metered arithmetic schedule in FR-362, allocation admission and refusal contracts, no_std plus alloc support, and the crate's local unsafe prohibition. It does not alter Decimal semantics or the separate CG consumer proof harnesses.

## Views

The following views describe the representation, borrow, and concurrency boundaries relevant to this decision.

### Representation and refinement view

The selected experiment replaces the implemented two-tag `Small(i128, cache)` / `Large(BigInt)` architecture with **three private storage variants**: `Small64(i64, cache)`, `Small128(i128, cache)` and `Large(BigInt)`. This is a representation amendment, not another spelling of the existing value-range guard. Both inline variants refine to the same full-i128 mathematical native class. The abstraction function maps either inline payload to its exact mathematical integer, ignoring cache state; it maps Large to its stored exact mathematical value. Every initialized inline cache equals that payload's value. Equal values may have different inline variants; neither variant identity nor cache state is semantic identity.

`native()` may expose a widened i128 mathematical value from either inline variant, but arithmetic must dispatch on the private storage variants **before** that helper erases the known storage width. Every i128-representable value remains representable inline; this architecture imposes no narrower mathematical domain.

### Construction and normalization view

The following table covers the existing public construction surface and private normalization routes. No new From trait implementation or public constructor is introduced.

| Construction or normalization route | Selected storage | Exactness and restrictions |
|---|---|---|
| Existing `From<i64>` | Small64, using an exact i64 payload | Numeric widening is exact; do not round-trip through BigInt or derive a caller-domain certificate. |
| Existing `From<i128>`, `From<u64>`, `From<usize>` | Small128 when the input fits i128 | Preserve existing exact conversion behavior; do not assume a platform pointer width. Any supported non-fitting conversion retains genuine BigInt continuation. Do not narrow a fitting result to Small64 merely because its value is small. |
| Canonical parsing and `from_big`, including fitting zero and i64-range values | Small128 when i128-representable; otherwise Large | Preserve canonical-spelling refusal and exact large parsing. No minimum-width normalization. |
| `zero()`, `one()`, default | Small64 with the corresponding payload | Cache starts empty. |
| Clone of either inline variant | Same inline variant and exact numeric payload, fresh empty cache | Preserve the source variant even after source cache materialization. |
| Clone of Large | Large with exact cloned BigInt | Clone does not reinterpret representation as a domain. |
| General arithmetic BigInt result normalization | Small128 if i128-representable; otherwise Large | Includes cancellation back into the native range; no minimum-width narrowing. |

Every newly constructed inline value starts with an empty cache; observing a retained value does not reset its cache. This table specifies the listed construction and normalization boundaries; it does not redesign the other public integer operations. Their existing exact mathematical behavior, refusal behavior, and admitted signatures remain required.

### Arithmetic transition view

| Storage inputs and operation | Required dispatch and result transition |
|---|---|
| Small64 / Small64 Add, Subtract, Multiply | Match both storage variants before `native()`; widen both actual i64 payloads to i128 before ordinary arithmetic; return Small128 with the exact result. No checked-i128 overflow-to-BigInt continuation exists within this storage arm. |
| Small64 Negate | Match storage before `native()`; widen the i64 payload to i128 before negation; return Small128. In particular negated i64 MIN is positive 2^63. |
| Any other all-inline binary pair, or Small128 Negate | Preserve the implemented numeric signed-i64 representability guard and widening arm. When that guard fails, preserve checked full-i128 arithmetic and genuine BigInt overflow continuation. A fitting inline result is Small128; a promoted result normalizes as above. |
| At least one Large operand, or Large Negate | Preserve the existing general exact BigInt route and normalize a fitting result to Small128, otherwise Large. |

For the direct Small64 arms, the exact extrema after widening are:

| Operation on arbitrary signed-i64 inputs | Exact minimum | Exact maximum |
|---|---|---|
| Add | -2^64 | 2^64 - 2 |
| Subtract | -2^64 + 1 | 2^64 - 1 |
| Multiply | -2^126 + 2^63 | 2^126 |
| Negate | -2^63 + 1 | 2^63 |

All fit signed i128, including i64 MIN squared at 2^126. Genuine i128 overflow remains supported in the other inline arms: negating i128 MIN yields positive 2^127 through BigInt. No truncating cast, saturation, wrapping, assumption, copied kernel, Kani-only implementation, public narrowed API or altered proof input supplies this mechanism.

**Chaining limit:** Small64-pair arithmetic always returns Small128, even when the result happens to fit i64. The next operation can therefore take the existing value-guarded general inline route. This experiment adds no automatic minimum-width result normalization, caller-domain certificate or compound-expression fast-path guarantee. It does not promise that the four-helper IR-917 clause avoids general continuation, or that CBMC prunes all cache/general branches. Numeric casts preserve value relations already; retained storage width is a distinct structural experiment, not evidence that the backend previously lost those relations.

### Borrow and concurrency view

The stable public `as_big(&self) -> &BigInt` contract requires storage that outlives each returned borrow. For either Small64 or Small128, the first BigInt-required observation initializes its own private safe race-initialization cache from its immutable numeric payload; subsequent observations borrow the retained immutable winner. For Large, observations borrow its stored BigInt. Concurrent first access may construct multiple boxed candidates. The cache retains one winner and drops losing candidates; all successful callers borrow the winner. The initializer must not call cache-dependent methods on the same Integer.

Cloning either inline variant preserves that variant and its numeric payload and starts with a fresh empty cache. Cloning Large preserves the exact BigInt. Equality, ordering, hashing, Debug and Display are functions of mathematical value only; cache initialization cannot change their results. Cache materialization and discarded candidates are implementation resource costs and add no semantic meter charge.

## Decisions

| Decision | Choice | Alternatives and trade-off |
|---|---|---|
| Integer value representation | Three private variants Small64, Small128 and Large, with one cache-independent mathematical abstraction. | The implemented two-tag representation already supports full-i128 native execution; retaining it avoids another tag and transition invariants. The selected experiment retains construction width directly, at the cost of additional states and possible layout growth. BigInt-only or eager dual storage defeats the bounded allocation objective. |
| Native arithmetic dispatch | Small64 storage arms widen to i128 before all four operations and return Small128; other inline arms retain existing numeric guards, checked-i128/general continuation. | Repeating the implemented numeric width guard adds no new mechanism. Retaining a width tag avoids re-deriving width in the direct storage arms, but may not improve symbolic execution and does not optimize subsequent Small128 results. Deleting general continuation would break arbitrary precision. |
| Stable borrowed BigInt for both inline variants | Safe no_std-compatible race initialization with alloc support and stable retained borrowing. | A core unsynchronized cell would remove Sync; a std-only primitive violates no_std; leaked boxes, local unsafe, or a changed guard/Cow API violate the constraints or public borrow contract. Any selected implementation must preserve the race winner/loser lifecycle and stable borrow contract. |
| Concurrent initialization | Allow candidate construction by racing callers, retain one immutable winner, and drop every losing candidate. | A once-only candidate allocation promise is not supportable with this race API. The design favors safe Sync and stable borrows while explicitly accepting temporary concurrent allocation overhead. |
| Numeric observations and metering | Preserve mathematical value, public observation, existing trait behavior, and FR-362/FR-358 charge/refusal order exactly. | Making representation state observable or adding charges for cache work would change the contract. |
| Qualification claim | Keep the native-width refinement and consumer qualification open and require the unchanged CG TC-025 arithmetic mutant, TC-027 healthy case, and TC-027 bound mutant at their existing 16GiB/600-second/unwind3/Cadical settings. | Unit tests or a changed/smaller consumer harness cannot establish the intended end-to-end Kani result. No memory improvement is asserted before those controls pass. |

### Dependency and resource constraints

The selected architecture requires a safe race-initialization cache that supplies stable lazy references without std and supports concurrent initialization using alloc. Package and version selection, as well as concrete feature resolution, belong to implementation dependency management. Local rustc 1.98.1 target metadata reports pointer atomics for `thumbv7em-none-eabi`; that metadata is not a build result or proof of all supported targets. Implementation qualification must confirm supported target compatibility and that dependency feature resolution does not introduce std or weaken local `forbid(unsafe_code)`.

Each inline variant carries its own cache cell/pointer. Adding a third variant may change discriminant placement, alignment, Integer size and enclosing Value size. Record actual before/after layouts for the same target and configuration; no byte-size estimate is a correctness or performance threshold. First BigInt materialization can allocate the retained value; concurrent calls can allocate additional candidates, with losers freed after initialization. TC-919 must record actual size and allocation behavior without treating layout growth or a fixed allocation count as a correctness oracle.

## Risks

- The third storage variant, its transitions, larger inline representation or BigInt cache branch may leave CBMC memory use unchanged or make it worse. The original unchanged CG healthy and mutation controls determine qualification; no 16GiB improvement is claimed until they pass. Owner: IR-705 proof lane; revisit after implementation review and consumer evidence.
- The currently identified target's pointer-atomic metadata does not establish all downstream targets or dependency feature behavior. Owner: implementation and platform review; build the declared supported target set before adoption.
- Racing materialization can temporarily allocate multiple BigInts. Owner: implementation; preserve losing-candidate deallocation observations and disclose actual costs in TC-919 evidence.

### Implementation and qualification state

The published baseline implements the two-tag Small/Large representation, safe lazy borrowed BigInt cache, native arithmetic, canonical fitting parse, charge metadata and numeric i64-guarded widening. Actual static source/CFG inspection confirms that the widened arithmetic arm exists; it does not establish fallback feasibility or memory dominance. The three-variant architecture above is **PROPOSED / UNIMPLEMENTED** and awaits independent specification review before implementation. It amends the selected representation; it is not a claim that the existing guards are missing. Kernel/native evidence does not establish consumer proof acceptance.

The unchanged original healthy artifact still has no proof verdict: the measured 16GiB run on the existing implementation exhausted memory, and the separate authorized 40GiB diagnostic also exhausted memory. Neither changes the original AC-6 ceiling or discharges a control. IR-917 additionally reports MemoryExhausted for its V2 clause implication proof, with four metered runtime helpers and symbolic Integer-add promotion identified by its owner as costs. Its supplied source/log are unavailable locally, so that attribution and peak are reported evidence, not independently verified measurements here.

After adoption, the IR-705 proof owner records separately whether the genuine IR-917 V2 fixture fits 16GiB, including exact source/cohort/selector, healthy assertion and completion-cover result, helper-mutant substantive falsification and playback, peak-memory provenance, timeout and teardown. Preserve its CG AC-41 and AC-119 obligations; do not substitute an older synthetic fixture or conflate its controls with original TC-025/TC-027. IR-917 post-change fit is currently unmeasured. If its exact candidate is unavailable, record that input blocker instead of a result. This additional consumer evidence does not replace AC-6 or weaken Decimal/quantity negative obligations.
