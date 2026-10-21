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
  - target: "ix://agent-ix/quire-exact/FR-384"
    type: realizes
---
# AD-003: Bounded integer representation and borrowed BigInt refinement

## System Boundary

This decision covers the private storage representation of `quire-exact::Integer`, its exact refinement to mathematical integers, and the stable borrowed `as_big` interface. It preserves the public `Integer` API, metered arithmetic schedule in FR-362, allocation admission and refusal contracts, no_std plus alloc support, and the crate's local unsafe prohibition. It does not alter Decimal semantics or the separate CG consumer proof harnesses.

The additive admitted native API decision below supersedes the earlier prohibition on a public narrowed API only for the explicitly selected new facility. The general Integer representation, arbitrary-precision continuation and borrowed as_big obligations above remain in force. Existing retrospective experiment descriptions do not establish current proof qualification.

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

### Admitted native execution view (IR-705 option 2)

Peter authorized the additive kernel-owned bounded-native API. Its controlling requirement is [FR-384](../functional/FR-384-admitted-native-integer-refinement.md). The separate carrier has native payload storage and no BigInt or borrowed-image cache. Kernel admission checks conservative whole-operation domains and every reachable intermediate before exposing an execution capability. A result-domain restriction never stands in for a storage-capacity proof. Generated clients use this same production API in native and proof builds.

The admitted API uses operator-specific capacity. Its Copy carrier and interval endpoints remain i128. Add/Subtract/Negate retain full signed-i128 operands when the complete image fits i128; ordering retains every signed-i128 payload. Only Multiply requires both complete operand intervals within signed32 `[-2147483648, 2147483647]`. It admits all such interval pairs, computes their exact four-corner signed64 image and retains results in i128; it refuses any outside endpoint before evaluator charges, even with a zero opposite operand or payload. The semantic result-domain endpoints remain i128 and cannot justify accepting a larger operand domain. This is a capacity decision for the additive API, not a narrower general Integer representation or a change to any original consumer input domain.

The native mathematical abstraction and each admitted arithmetic transition must refine arbitrary-precision Integer/BigInt over every accepted operand. Composition preserves the abstraction through an entire expression, including comparisons, Boolean decisions and error propagation. Public production Meter accounting remains real and ordered. This is an explicit additive capability with an admission contract, rather than an optimization that relies on CBMC discovering unreachable branches of general Integer.

The general Integer API remains the fallback for consumers whose domains cannot be admitted. The bounded route has no successful implicit promotion to BigInt: the certificate prevents capacity overflow for admitted inputs. A malformed or insufficient certificate is refused before successful native execution; a caller cannot mask it by adding an assumption, shrinking input domains or classifying an inconclusive proof as verified.

## Decisions

| Decision | Choice | Alternatives and trade-off |
|---|---|---|
| Additive admitted native API | Select a separately admitted kernel-owned native carrier and operation capability under FR-384, leaving general Integer unchanged. | Further private storage/charge tuning preserves fewer APIs but has not produced a healthy 16 GiB verdict. Copying an evaluator into CG loses the single kernel ownership and is rejected. A narrowed capability requires explicit admission and full-capacity refinement evidence. |
| Admitted Multiply capacity | Complete signed32 operand intervals, exact widened signed64 four-corner image and i128 retained result; all other native capacities remain as above. | Unrestricted i128 multiplication requires a wider refinement obligation. This bounded capability excludes larger operand intervals even for products that fit; callers retain the general exact route without shrinking their source domains. Proof qualification must cover every accepted signed32 pair, not only consumer fixture ranges. |
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

The published baseline implements the two-tag Small/Large representation, safe lazy borrowed BigInt cache, native arithmetic, canonical fitting parse, charge metadata and numeric i64-guarded widening. Actual static source/CFG inspection confirms that the widened arithmetic arm exists; it does not establish fallback feasibility or memory dominance. The three-variant architecture above is **PROPOSED / UNIMPLEMENTED** and awaits independent specification review before implementation. It amends the selected representation; it is not a claim that the existing guards are missing. Kernel/native evidence does not establish consumer proof acceptance. The operator-specific admitted Multiply contract is prospective until reviewed implementation and fresh full-capacity refinement evidence land. Earlier unrestricted-i128 multiplication attempts do not discharge it; their unsuccessful records remain retained without a new proof claim.

The unchanged original healthy artifact still has no proof verdict: the measured 16GiB run on the existing implementation exhausted memory, and the separate authorized 40GiB diagnostic also exhausted memory. Neither changes the original AC-6 ceiling or discharges a control. IR-917 additionally reports MemoryExhausted for its V2 clause implication proof, with four metered runtime helpers and symbolic Integer-add promotion identified by its owner as costs. Its supplied source/log are unavailable locally, so that attribution and peak are reported evidence, not independently verified measurements here.

After adoption, the IR-705 proof owner records separately whether the genuine IR-917 V2 fixture fits 16GiB, including exact source/cohort/selector, healthy assertion and completion-cover result, helper-mutant substantive falsification and playback, peak-memory provenance, timeout and teardown. Preserve its CG AC-41 and AC-119 obligations; do not substitute an older synthetic fixture or conflate its controls with original TC-025/TC-027. IR-917 post-change fit is currently unmeasured. If its exact candidate is unavailable, record that input blocker instead of a result. This additional consumer evidence does not replace AC-6 or weaken Decimal/quantity negative obligations.

### Option-2 qualification boundary

The selected facility changes generated production call sites and hence may change emitted source bytes. Qualification preserves original semantic inputs, finite domains, selected clause, independent oracle, assertions, completion cover and substantive mutation controls. It does not require byte-identical generated source from the superseded general-only route. Each changed artifact and actual kernel identity is recorded. General BigInt differential/refinement checks and native-capacity admission controls precede genuine CG TC-027, IR-917 and IR-948 profiles; none replaces another. Healthy acceptance remains 16 GiB. Up to 40 GiB or larger authorized diagnostics report their own actual peak and outcome and do not earn 16 GiB acceptance. Owner: IR-705 code lead; revisit after the first actual profile verdict.
