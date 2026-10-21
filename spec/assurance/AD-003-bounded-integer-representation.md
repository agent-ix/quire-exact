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

The selected representation has two tags: `Small(i128, cache)` and `Large(BigInt)`. The abstraction function maps every `Small(n, cache)` to the mathematical integer `n`, independent of cache state; it maps `Large(b)` to the exact value represented by `b`. A populated Small cache must equal the Small payload's mathematical value. Arithmetic uses range-proven widened native operations or checked full-width native operations, retaining genuine BigInt continuation when the exact result does not fit i128. In particular, negating `i128::MIN` produces positive 2^127 through BigInt continuation.

### Native width dispatch

Within the existing Small tag, the selected refinement first checks exact signed-i64 representability of both operands (the one operand for Negate). Successful conversions are widened to i128 **before** Add/Subtract/Multiply/Negate; their exact results fit i128, so this arm uses ordinary native arithmetic and returns Small without an i128-overflow-to-BigInt continuation at that operation. In particular, this lowers the actual `Integer::add` small path, rather than merely restating its existing checked-i128 implementation. No third representation tag or public constructor is introduced.

| Operation on arbitrary signed-i64 inputs, widened before arithmetic | Exact minimum | Exact maximum |
|---|---|---|
| Add | -2^64 | 2^64 - 2 |
| Subtract | -2^64 + 1 | 2^64 - 1 |
| Multiply | -2^126 + 2^63 | 2^126 |
| Negate | -2^63 + 1 | 2^63 |

Every bound lies within signed i128. The guard is numeric representability, not an IntegerInterval, a caller certificate, or a semantic domain restriction. Conversion failure falls through to the existing checked-i128 operation and genuine BigInt continuation; Large operands retain the general exact route and normalization. Values immediately outside i64, full-i128 fitting results, real i128 overflow, and Large cancellation remain admitted. No truncating cast, saturation, wrapping, assumption, Kani-only route, or altered proof input supplies the range proof.

This production dispatch preserves the two tags, full-i128 Small payload, lazy cache, public observations, metering and refusal contracts. It does not promise that CBMC discards every conversion-failure, cache, or general branch. All-four-operation boundary and continuation tests plus the unchanged consumer qualification are required before claiming success.

### Borrow and concurrency view

The stable public `as_big(&self) -> &BigInt` contract requires storage that outlives each returned borrow. For Small, the first BigInt-required observation initializes a private safe race-initialization cache from the immutable i128 payload; subsequent observations borrow the retained immutable winner. For Large, observations borrow its stored BigInt. Concurrent first access may construct multiple boxed candidates. The cache retains one winner and drops losing candidates; all successful callers borrow the winner. The initializer must not call cache-dependent methods on the same Integer.

Cloning Small copies the i128 and starts with a fresh empty cache. Cloning Large preserves the exact BigInt. Equality, ordering, hashing, Debug and Display are functions of mathematical value only; cache initialization cannot change their results. Cache materialization and discarded candidates are implementation resource costs and add no semantic meter charge.

## Decisions

| Decision | Choice | Alternatives and trade-off |
|---|---|---|
| Integer value representation | Tagged Small(i128, cache) and Large(BigInt), with a cache-independent abstraction function. | BigInt-only storage preserves simple borrowing but constructs heap-backed values on the bounded path. A naked i128 cannot represent overflow results. Eager dual storage defeats the bounded allocation objective. |
| Native arithmetic dispatch | Exact i64 representability followed by i128 widening for all four elementary operations; existing checked-i128/general continuation otherwise. | Keeping checked-i128 overflow selection even for the representable subrange leaves the reported symbolic promotion path. Narrowing Small storage or deleting general continuation changes the contract; an extra tag increases representation invariants without evidence it is needed. |
| Stable borrowed BigInt for Small | Safe no_std-compatible race initialization with alloc support and stable retained borrowing. | A core unsynchronized cell would remove Sync; a std-only primitive violates no_std; leaked boxes, local unsafe, or a changed guard/Cow API violate the constraints or public borrow contract. Any selected implementation must preserve the race winner/loser lifecycle and stable borrow contract. |
| Concurrent initialization | Allow candidate construction by racing callers, retain one immutable winner, and drop every losing candidate. | A once-only candidate allocation promise is not supportable with this race API. The design favors safe Sync and stable borrows while explicitly accepting temporary concurrent allocation overhead. |
| Numeric observations and metering | Preserve mathematical value, public observation, existing trait behavior, and FR-362/FR-358 charge/refusal order exactly. | Making representation state observable or adding charges for cache work would change the contract. |
| Qualification claim | Keep the native-width refinement and consumer qualification open and require the unchanged CG TC-025 arithmetic mutant, TC-027 healthy case, and TC-027 bound mutant at their existing 16GiB/600-second/unwind3/Cadical settings. | Unit tests or a changed/smaller consumer harness cannot establish the intended end-to-end Kani result. No memory improvement is asserted before those controls pass. |

### Dependency and resource constraints

The selected architecture requires a safe race-initialization cache that supplies stable lazy references without std and supports concurrent initialization using alloc. Package and version selection, as well as concrete feature resolution, belong to implementation dependency management. Local rustc 1.98.1 target metadata reports pointer atomics for `thumbv7em-none-eabi`; that metadata is not a build result or proof of all supported targets. Implementation qualification must confirm supported target compatibility and that dependency feature resolution does not introduce std or weaken local `forbid(unsafe_code)`.

The cache adds a cell/pointer to Small. A change from an estimated 32-byte to 48-byte Integer is only a planning estimate, not a measured layout commitment. First BigInt materialization can allocate the retained value; concurrent calls can allocate additional candidates, with losers freed after initialization. TC-919 must record actual size and allocation behavior without treating either estimate or a fixed allocation count as an oracle.

## Risks

- The larger inline representation or BigInt cache branch may leave CBMC memory use unchanged or make it worse. The original unchanged CG healthy and mutation controls determine qualification; no 16GiB improvement is claimed until they pass. Owner: IR-705 proof lane; revisit after implementation review and consumer evidence.
- The currently identified target's pointer-atomic metadata does not establish all downstream targets or dependency feature behavior. Owner: implementation and platform review; build the declared supported target set before adoption.
- Racing materialization can temporarily allocate multiple BigInts. Owner: implementation; preserve losing-candidate deallocation observations and disclose actual costs in TC-919 evidence.

### Implementation and qualification state

The two-tag Small/Large representation, safe lazy borrowed BigInt cache and native arithmetic already exist in published kernel `765a95914bfc9acc5c01dd482630535113c69658`; merged parser and charge-metadata refinements do not implement the width dispatch above. This document remains proposed for that additional refinement and its outstanding qualification, not because the entire representation is unimplemented. Kernel unit/native evidence does not establish consumer proof acceptance.

The unchanged original healthy artifact still has no proof verdict: the measured 16GiB run on that kernel exhausted memory, and the separate authorized 40GiB diagnostic also exhausted memory. Neither changes the original AC-6 ceiling or discharges a control. IR-917 additionally reports MemoryExhausted for its V2 clause implication proof at CG `a2f61a2e`, with four metered runtime helpers and symbolic Integer-add promotion identified by its owner as costs. Its supplied source/log are unavailable locally, so that attribution and peak are reported evidence, not independently verified measurements here.

After adoption, the IR-705 proof owner records separately whether the genuine IR-917 V2 fixture fits 16GiB, including exact source/cohort/selector, healthy assertion and completion-cover result, helper-mutant substantive falsification and playback, peak-memory provenance, timeout and teardown. Preserve its CG AC-41 and AC-119 obligations; do not substitute an older synthetic fixture or conflate its controls with original TC-025/TC-027. IR-917 post-change fit is currently unmeasured. If its exact candidate is unavailable, record that input blocker instead of a result. This additional consumer evidence does not replace AC-6 or weaken Decimal/quantity negative obligations.
