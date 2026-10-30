---
id: FR-391
title: "Copy structural identities through a measured reservation policy"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-389
    type: references
---
# FR-391: Copy structural identities through a measured reservation policy

## Description

When a caller needs a separate owned copy of an admitted structural `EffectiveId` or `NodeKey`, the exact kernel SHALL expose a fallible copy that lets the caller's storage policy observe each real reservation. The caller remains responsible for admitting the identity and for its enclosing registry's publication.

## Inputs

- A borrowed `EffectiveId` containing a flat root-first chain of `OriginalDeclarationKey` values, each with owned package and node UTF-8 strings; or a borrowed `NodeKey` containing a flat path and owned text.
- For policy-aware copying, a caller-supplied reservation callback receiving the minimum additional payload bytes immediately before each real reservation and returning a caller-owned refusal type on denial.

## Outputs

- A separately owned, structurally equal identity on success.
- A typed copy error distinguishing the callback's original refusal, `Allocation { requested: usize }` for a failed representable allocator reservation, and `Capacity { requested: usize }` for an unrepresentable reservation count, byte product, or layout. No partial identity is returned. `requested` is the minimum additional payload-byte amount, or `usize::MAX` when that amount cannot be represented.

## Behavior

1. **Structural declaration copy.** `EffectiveId::try_clone(&self)` SHALL copy the flat chain and each key's package and node strings through fallible reservations.
   The resulting identity SHALL preserve root-first chain order, original UTF-8 bytes, equality, hash, and componentwise ordering against every other `EffectiveId`.
   The copy SHALL use bounded call-stack depth independent of owner depth.
   The existing `Clone` trait and construction signatures remain available.
2. **Measured policy entry points.** Both `EffectiveId` and `NodeKey` SHALL expose public, policy-aware fallible clone entry points that borrow the source and pass the actual minimum additional byte request to a caller callback immediately before each representable heap reservation they perform. The copy operation SHALL return the callback's refusal as its original typed value, distinct from a native allocator refusal and capacity failure. The policy entry point SHALL return no copy when the callback refuses. The existing `NodeKey::try_clone` convenience operation and its [FR-389](./FR-389-carry-admitted-structural-identities.md) refusal contract remain available; `EffectiveId::try_clone` likewise requires no callback. A consumer can use the policy entry point without an infallible preflight copy or a proxy request for the enclosing identity.
3. **Reservation accounting.** Before each chain, path or string reservation, the fallible copy SHALL calculate the minimum additional payload bytes with checked arithmetic. A new string of length `n` requests `n` bytes and an empty string requests none. A new flat chain of `k` keys requests `k × size_of::<OriginalDeclarationKey>()` bytes; a `NodeKey` path reservation uses its actual additional flat-token count multiplied by `size_of::<FlatSegment>()`, including nested headers and child tokens. A failed reservation amount SHALL describe that site rather than allocator spare capacity or the enclosing identity.
   An allocator denial of a representable reservation SHALL return `Allocation { requested }`.
   An unrepresentable count, byte product, or layout SHALL return `Capacity { requested }` without invoking the policy callback or allocator. The typed results let a QSV caller map allocation, capacity and callback refusals separately to its `EnvironmentFailure`, with `StorageUnit::Bytes` for exact's native byte amounts.
4. **Failure atomicity.** On a refusal at any actual chain, path or string reservation, each policy-aware copy SHALL return no identity and leave the source unchanged. Empty strings SHALL make no reservation or consume a queued policy denial.
   The copy SHALL perform no reservation through `Clone`, `to_owned`, `collect`, an unreserved push, or equivalent infallible allocation.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-391-AC-1 | A root-only `EffectiveId` and a member identity with at least 10,000 owner links each copy successfully through the public API. The result is separately owned, equals and hashes like the source, retains its exact root-first package/node bytes and compares identically against identities differing at the root, a middle owner, or the leaf. Copy and destruction use bounded call-stack depth independent of owner depth. | Test |
| FR-391-AC-2 | A crate test denies each actual reservation in a prepared identity whose chain, package, and node strings are nonempty. Each refusal returns `Allocation { requested }` with no copy and an unchanged source; the amount is exactly the additional payload bytes of that site (`chain_len × size_of::<OriginalDeclarationKey>()`, `package.len()`, or `node.len()`). Empty package or node strings request no reservation and do not consume the injected denial. | Test |
| FR-391-AC-3 | The same checked-count calculation used by the production chain reservation is exercised in a crate test with synthetic counts at the exact representable multiplication boundary and one above it, without allocating a near-`usize::MAX` vector. The boundary returns the exact product; overflow returns `Capacity { requested: usize::MAX }` before any allocator call. A representable count with an unrepresentable layout returns `Capacity`, not `Allocation`. Source values remain unchanged. | Test |
| FR-391-AC-4 | A consumer can match the public copy error without inspecting text: a representable denied request maps to `EnvironmentFailure::Allocation(StorageRequest::new(requested, StorageUnit::Bytes))`, and overflow or layout incapacity maps to `EnvironmentFailure::Capacity` with the same byte unit and request amount. Inspection of the copy path finds no infallible allocation or depth-proportional recursion. | Test |
| FR-391-AC-5 | A public policy-aware `EffectiveId` clone and a public policy-aware `NodeKey` clone each invoke a caller callback immediately before every actual representable chain, path, package, or text reservation with that site's exact additional byte count. Injection at each site returns the caller's original typed denial, no copy and an unchanged source. Native allocator refusal and pre-reservation count/layout incapacity remain separate variants; an impossible layout never invokes the callback. Existing no-policy `NodeKey::try_clone` and `EffectiveId::try_clone` remain callable with their stated behavior. | Test |

## Dependencies

- [FR-389](./FR-389-carry-admitted-structural-identities.md) defines the structural components, order and existing `NodeKey` copy semantics. This requirement adds the policy-aware boundary and fallible `EffectiveId` copy.
- QSV FR-108 and FR-115 own type-environment admission and sealing, including the typed storage carrier and atomic registry publication. The exact kernel does not publish an environment.
