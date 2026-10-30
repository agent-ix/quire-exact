---
id: TC-940
title: "Fallible structural identity copy with measured reservations"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-391
    type: verifies
---
# TC-940: Fallible structural identity copy with measured reservations

## Description

Verify [FR-391](../functional/FR-391-fallible-effective-identity-copy.md) through the public `EffectiveId` and `NodeKey` copy APIs. Use caller-authored identities; this test case stores no other repository's fixture.

## Test Procedure

1. Build a root-only identity and a 10,000-link member identity with independently authored root, middle, and leaf `OriginalDeclarationKey` values. Copy both through `try_clone`, compare every owner-chain component, equality, one hasher's hash, and ordering against identities changed at each of those positions. Drop the copy and source separately while monitoring stack depth.
2. Prepare a shorter `EffectiveId` with nonempty package and node strings. In one run, let the public policy callback permit each request and use a crate-internal reservation-failure hook to inject native refusal at each actual chain or string reservation, recording `Allocation { requested }` and unchanged source. In a separate run, return a chosen non-allocation error from the public policy callback at each site and assert that same error is returned unchanged. Repeat with empty package and node strings and check that they make no request and do not consume either queued denial. Exercise `EffectiveId::try_clone` without a callback on the same inputs.
3. Invoke the *same private checked-count calculation used by production* with synthetic chain lengths at `usize::MAX / size_of::<OriginalDeclarationKey>()` and one above it. Check the exact product at the representable boundary and `Capacity { requested: usize::MAX }` on product overflow. Exercise a representable product whose `Vec` layout is impossible (including the platform `isize::MAX` bound); verify `Capacity` before allocator invocation. Use the policy-aware `NodeKey` count/layout calculation to assert `Capacity` on synthetic impossibility, then use the retained no-policy `NodeKey::try_clone` calculation to assert [FR-389-AC-12](../functional/FR-389-carry-admitted-structural-identities.md)'s `InvalidNodeKey::Allocation { requested: usize::MAX }` on count/byte overflow. Inspect actual reservation sites to confirm they call the corresponding calculations.
4. Prepare a mixed text/nested `NodeKey`; use its public policy-aware clone to inject a distinct non-allocation caller refusal before each actual string or flat-path reservation. Assert the callback sees the exact site's additional bytes, the refusal is returned unchanged, no copy is returned, and the source is intact. Exercise the no-policy `NodeKey::try_clone` separately to preserve [FR-389](../functional/FR-389-carry-admitted-structural-identities.md). Inspect the two policy-aware paths for any preflight proxy or hidden infallible copy.
5. Compile a consumer matching policy refusal, native allocation refusal and capacity failure separately. Map native failed reservation bytes and caller-reported allocator denial to QSV's allocation carrier, and size/layout incapacity to its capacity carrier, each with `StorageUnit::Bytes`, without parsing an error string or relabeling one as the other.

## Expected Results

1. Both copies are independent and componentwise identical to their sources; equality, hashes and comparisons preserve [FR-389](../functional/FR-389-carry-admitted-structural-identities.md), and the deep case uses bounded call-stack depth.
2. Native reservation failure injection returns `Allocation { requested }` with the exact failed additional payload-byte amount. A public callback denial returns the caller's chosen type and value unchanged. Both return no partial copy and leave the source intact; empty strings make no reservation.
3. The production checked-count helpers return the exact representable product and refuse synthetic overflow without an attempted allocation or impossible-size fixture. The new policy-aware paths distinguish capacity from allocator denial; the retained no-policy `NodeKey::try_clone` keeps FR-389-AC-12's overflow-as-`InvalidNodeKey::Allocation { requested: usize::MAX }` contract.
4. The `NodeKey` callback observes each real reservation and carries its own typed refusal unchanged. Both identities remain unchanged on failure, and existing no-policy copies retain their contracts.
5. The caller can preserve the error distinction and exact byte amount in its typed storage result; the copy paths contain no infallible allocation or depth-proportional recursion.
