---
id: SR-1133
title: "Review of structural exact identity constructors"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact; PR #100; FR-389, FR-377, FR-379, TC-924, TC-938"
review_set: base
---

## Summary

Independent spec review of exact PR #100 at 6b6698f for IR-1105. QSpec #517 and #518 remain drafts. Their FR-150, FR-201, FR-322, FR-141, FR-142 and FR-441 text was used to check the proposed constructor shapes. No active AssuranceProfile applies in this repository. Quire validation, diff check and targeted Quoin advice passed; the existing repo-wide advice is unrelated to FR-389.

## Examined units

| ID | Role | Excerpt |
| --- | --- | --- |
| FR-389 | examined | The kernel SHALL carry structural identity components through typed constructors, preserving admission, equality, ordering and refusal without digest constructors. |
| FR-389-AC-1 | examined | Equal NodeKey components compare and hash equally; package, path and ordinal changes distinguish keys, and component order decides comparison. |
| FR-389-AC-2 | examined | Boundary ordinals construct; empty package, empty path and above-bound ordinal refuse; admitted read uses the same constructor. |
| FR-389-AC-3 | examined | EffectiveId and UniverseId use equal admitted structural components, and changing owner, name path or ordinal distinguishes them. |
| FR-389-AC-4 | examined | Enum node and union declaration/member VariantIds are distinct; foreign variant and empty member controls refuse. |
| FR-389-AC-5 | examined | MemberId and PopulationId compare structural components; Direct, Pre and Post differ; population value paths refuse. |
| FR-389-AC-6 | examined | Declared and compound UnitIds remain separate; canonical terms and empty dimensionless list govern compound identity. |
| FR-389-AC-7 | examined | All identity APIs use structural components; digest constructors and accessors are absent; new consumer compiles. |
| FR-389-AC-8 | examined | Existing FR-088, FR-089, FR-377 and FR-379 controls keep their stated outcomes using structural identities. |
| FR-377 | examined | Different declaration keys order by NodeKey before slots. |
| FR-379-AC-6 | context_only | Equal identifiers with different VariantIds give None. |
| TC-924 | examined | Composite equality fixtures use distinct structural declaration keys. |
| TC-938 | examined | Public constructor and removal controls for structural identities. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | EffectiveId constructor shape and AC-3 contradict QSpec FR-150: effective declaration identity is `{owner_effective_type, original}`, with ordinal in facts, not identity; an owner/name-path/ordinal constructor can equate distinct originals or distinguish one identity after fact changes. Specify the exact two-component identity and test changes to each component. | spec/functional/FR-389-carry-admitted-structural-identities.md:37,49; QSpec #517 FR-150:68-73 |
| FND-002 | medium | AC-8 incorporates FR-379-AC-6 unchanged, but the new union VariantId is exactly `(declaration, member name)` per QSpec FR-441. Under one declaration, equal identifiers cannot have different VariantIds; the mandated None oracle is unreachable through the admitted constructor. Retain all reachable FR-379 outcomes and replace this impossible control with a falsifying pair at the appropriate admission boundary. | spec/functional/FR-389-carry-admitted-structural-identities.md:39,54; spec/functional/FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md:44,58; QSpec #517 FR-441 |

## Verdict

Changes required for FND-001 and FND-002 before this spec can serve as the implementation contract. NodeKey shape and ordinal bounds match QSpec #517/#518 drafts, and ObjectId stays authored. No code or build checks were run.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-389 Behavior 2 and AC-3 now use the exact QSpec FR-150 pair `(owner_effective_type, original declaration key)`, explicitly exclude derivation-fact ordinal, and TC-938 changes each pair member while varying facts under a fixed pair. |
| FND-002 | fixed | FR-379 Behavior 6 and AC-6 now use the reachable same-name cross-declaration admission refusal and `compare_keys == None`; same-declaration members order by UTF-8 bytes. TC-938 step 3 exercises both cross-declaration controls. |
| FND-001 | fixed | Reverified at a0ba40fb: FR-389 now names the recursive owner (`null` or EffectiveId), original `(domain package identity, IR node identity)`, and UniverseId's separate `{model_selection, root_types}` value. TC-938 varies the admitted components and excludes derivation facts. This matches QSpec #517 ef5bba08. |
| FND-001 | fixed | Reverified at 5af5a6b: FR-389 uses QSpec #517 FR-150 recursive EffectiveId with original `{package, node}`, and QSpec FR-201's distinct MemberId, VariantId, UnitId, UniverseId and PopulationId shapes. AC-3/4/5/6 and TC-938 preserve equality, order, typed refusal and role controls against QSpec #518 43fcf4f2 structural-identity-vectors.json SI-01..SI-06 (including SI-03-terms and SI-03-exponent); ObjectId remains authored. |
