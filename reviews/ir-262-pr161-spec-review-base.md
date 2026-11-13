---
id: SR-17940486709564847047960
title: "IR-262 PR #161 spec review (base)"
type: SpecReview
analysis: base
date: 2026-11-13
scope: "agent-ix/quire-exact; PR #161 @ 2aa67f6; IR-262; spec/functional/FR-407-kani-tractable-native-enum-carrier.md, spec/test-cases/TC-955-kani-tractable-native-enum-carrier.md"
review_set: base
relationships:
  - target: ix://agent-ix/quire-exact/FR-407
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-955
    type: reviews
---

## Summary

Ticket: IR-262. Spec-only PR #161 (branch spec/ir-262-native-enum-carrier, one commit 2aa67f6 on origin/main 0234129) adds FR-407 (a `Copy`, rank-only `NativeEnum` carrier and `compare_native_enum`) and TC-955. Scope is `git diff origin/main...HEAD` (two files). Review set: base checklist (no AssuranceProfile applies in quire-exact; reviewer subagent run, no lens selection offered). Checked against the owner, quire-semantic-value origin/main 3c4616d `src/enumeration.rs` (`EnumValue`, `compare_enum`, `compare`), the kernel's `src/value.rs` (`EnumShape`, `EnumMember`), `src/node.rs` (`NodeKey` holds `Arc<NodeData>`), `src/comparison.rs` (`is_ordering`, `holds`), and CG origin/main `spec/kani/functional/FR-049-enum-scalar-harnesses.md` and `Cargo.toml`. `quire validate --scope . <both files>` exits 0 (one unrelated module-level `semantic.unused-import` warning). Linear IR-262 read as data. No cargo, no edits.

## Verdict

FAIL as written (not mergeable yet): 0 high, 4 medium, 4 low. Mergeable once FND-001 to FND-004 are fixed; the lows are one-line edits and should ride in the same round.

What holds: the owner is where the PR says it is. `compare_enum` and `EnumValue` live in quire-semantic-value `src/enumeration.rs`, and the order of refusals and charges in Behavior 6 and 7 matches it exactly: distinct declaration first (`DistinctEnumDeclarations`), then ordering over an unordered declaration (`UnorderedEnumOrdering`), both with no charge; then `EnumIdentityRead` size `ValueOccurrences` 1, `EnumIdentityRead` size `ValueOccurrences` 2, the comparison, then `EnumResultRetain` `results(1)`, each `?`-stopping. Rank equality is equivalent to the owner's member-key equality within one declaration, and rank order equals the owner's position order for an ordered enum. `NodeKey` really is an `Arc`, so the tractability motivation stands. AC-1 to AC-6 and AC-8 are testable and each has a TC-955 step; AC-9 is marked PLANNED and TC-955 records no measurement. No vendoring, no SHA, pin, digest or tracking record.

What fails: the Description understates the owner's declaration identity (it also compares the member list), the differential has no buildable home in quire-exact and the spec does not name one, AC-7 claims every mutant fails the differential although most are caught elsewhere, and the FR-049-AC-3 minted-identity limitation is left as a PR question instead of being stated with what CG must do.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Owner description is incomplete: QSV main `EnumValue` also holds `members: Arc<[String]>`, and `compare_enum` refuses `DistinctEnumDeclarations` when the declaration key, the ordered flag or the member list differs. Behavior 2 equates declarations by tag, count and ordered flag and tells the caller to assign one tag per declaration node, so two owner values with one key and different same-length member lists compare under the carrier but are refused by the owner. Name the field, require one tag per distinct (declaration key, ordered, member list), and add that case to the differential | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:24, spec/functional/FR-407-kani-tractable-native-enum-carrier.md:37, spec/test-cases/TC-955-kani-tractable-native-enum-carrier.md:22 |
| FND-002 | medium | The differential (Behavior 10, AC-6, AC-7, TC-955 step 5) needs the owner, but QSV depends on quire-exact (git, branch main), so quire-exact cannot dev-depend on QSV without an upward dependency from the kernel on its consumer (and, without a self-patch, a second quire_exact whose Meter/IllTyped types do not unify). "Run where both are visible" names no place. Fix: verify AC-6 in quire-exact against a literal expected charge sequence on a recording Meter; place the AC-7 differential in quire-semantic-value tests/ beside compare_enum and say so (QSV ticket), run after the carrier is on quire-exact main | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:56, spec/functional/FR-407-kani-tractable-native-enum-carrier.md:67, spec/functional/FR-407-kani-tractable-native-enum-carrier.md:68, spec/test-cases/TC-955-kani-tractable-native-enum-carrier.md:22 |
| FND-003 | medium | AC-7 says each mutant of TC-955 fails the differential at the step TC-955 names, but most mutants (from_index bounds, from_member rank, Box carrier, scan mutants) cannot be seen by a differential and are assigned to steps 1, 2, 3 and 6. Scope AC-7 to the Behavior 6 to 8 mutants and make the table name the step-5 differential failure for each | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:68, spec/test-cases/TC-955-kani-tractable-native-enum-carrier.md:29 |
| FND-004 | medium | FR-049-AC-3 and AC-6 require a minted member identity (same position, different key) to fail the eq/ne proof; with a rank-only carrier compare_native_enum cannot see it, so those CG criteria cannot hold under Kani. FR-407 leaves this as a PR question. State the limitation and what CG must do: amend FR-049-AC-3/AC-6 (CG ticket) so the minted-identity mutant is caught at construction (operands built with from_member against the owner-admitted EnumShape, a minted VariantId refused with MemberNotInShape, FR-407-AC-3), and the proof claims eq/ne over rank only | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:74 |
| FND-005 | low | CountExceedsCarrier is unreachable and untestable: EnumShape::rank already panics (expect) past u32::MAX positions, no AC covers it, and a test would need over 4 billion variants. Delete the refusal (the kernel already bounds shapes to u32) | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:42 |
| FND-006 | low | The ban on calling compare_enum or EnumValue (Behavior 8, AC-8) and the mutant "The comparison delegates to compare_enum" are vacuous in quire-exact: the kernel cannot name a QSV item (QSV depends on it), so the mutant is unbuildable and the scan cannot fail. Remove those items; keep the NodeKey, cfg(kani), loop and allocation bans | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:49, spec/functional/FR-407-kani-tractable-native-enum-carrier.md:69, spec/test-cases/TC-955-kani-tractable-native-enum-carrier.md:42 |
| FND-007 | low | The relationship section says CG uses compare_native_enum "as the native expression of the oracle", which contradicts the Use case (the carrier is the oracle side and a < b over the indices is the native expression). Say which side the carrier replaces | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:74, spec/functional/FR-407-kani-tractable-native-enum-carrier.md:30 |
| FND-008 | low | The refusals IndexOutOfRange and MemberNotInShape have no named error type and no return signature for from_index/from_member, so two implementers could pick different shapes (unit enum or IllTyped cause). Name the type and the signature | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:41, spec/functional/FR-407-kani-tractable-native-enum-carrier.md:42 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-407 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:24 | The owner's EnumValue (quire-semantic-value) holds a NodeKey (an Arc), a member NodeKey and a Box<str>. |
| FR-407-B2 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:37 | Two operands are of one declaration exactly when their tags, member counts and ordered flags are all equal. |
| FR-407-B6 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:47 | if the operands are not of one declaration, return Err(IllTyped) with DistinctEnumDeclarations; else if the operator is an ordering operator and the declaration is not ordered, return UnorderedEnumOrdering; in both cases consume nothing from the Meter. |
| FR-407-B7 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:48 | EnumIdentityRead with size ValueOccurrences 1, then EnumIdentityRead with size ValueOccurrences 2, then, after the comparison and before returning, EnumResultRetain with results 1. |
| FR-407-AC-1 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:62 | NativeEnum is Copy, has no drop glue, holds no NodeKey, VariantId, Arc, String, Box or Vec, and has no public field; zero allocations under a counting allocator. |
| FR-407-AC-2 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:63 | from_index refuses index == member_count, member_count + 1, u32::MAX against a smaller count and any index with a count of zero, each with IndexOutOfRange. |
| FR-407-AC-3 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:64 | from_member agrees with from_index for every member, refuses a wrong-rank member and a foreign variant with MemberNotInShape. |
| FR-407-AC-4 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:65 | For the six operators over every ordered pair of an ordered enum of three members, the Boolean equals the native operator over the indices. |
| FR-407-AC-5 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:66 | Different tags, or same tag but different count or ordered flag, are refused with DistinctEnumDeclarations, which takes precedence over UnorderedEnumOrdering. |
| FR-407-AC-6 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:67 | The charge points, order, limit kinds and amounts of Behavior 7, the denied charge and the incomplete record equal the owner's. |
| FR-407-AC-7 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:68 | The host differential of Behavior 10 passes beside the owner's compare_enum, and each mutant of TC-955 fails it at the step TC-955 names. |
| FR-407-AC-8 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:69 | The comparison source contains no cfg(kani), loop, iterator adapter, allocation, NodeKey or EnumValue, and no call of compare_enum. |
| FR-407-AC-9 | examined | spec/functional/FR-407-kani-tractable-native-enum-carrier.md:70 | PLANNED: the Kani harness of Behavior 9 returns an executed terminal VERIFIED verdict. |
| TC-955 | examined | spec/test-cases/TC-955-kani-tractable-native-enum-carrier.md:14 | Verify FR-407-AC-1 through FR-407-AC-9 through the public kernel API, a host differential beside the owner's compare_enum and the recorded Kani measurement. |
| QSV compare_enum | context_only | quire-semantic-value src/enumeration.rs:292 | if left.declaration != right.declaration or left.ordered != right.ordered or left.members != right.members: DistinctEnumDeclarations |
| CG FR-049-AC-3 | context_only | quire-contract-codegen spec/kani/functional/FR-049-enum-scalar-harnesses.md | A minted member identity (same position, different key) makes the eq or ne proof fail. |
