---
id: FR-389
title: "Carry admitted structural identities without digest constructors"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: references
  - target: ix://agent-ix/quire-exact/FR-089
    type: references
  - target: ix://agent-ix/quire-exact/FR-377
    type: references
  - target: ix://agent-ix/quire-exact/FR-379
    type: references
---
# FR-389: Carry admitted structural identities without digest constructors

## Description

When the caller supplies an admitted semantic identity, the `quire-exact` kernel SHALL carry its structural components through the matching typed identity constructor, compare them by that identity's componentwise rule, and preserve the existing value admission, equality, key ordering and refusal behavior of [FR-088](./FR-088-enum-and-unit-identity-in-the-kernel.md), [FR-089](./FR-089-kernel-refuses-a-population-pair.md), [FR-377](./FR-377-composite-canonical-key-across-declarations.md) and [FR-379](./FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md). No kernel identity constructor accepts a digest or checks an identity preimage. The caller, not the kernel, joins an identity to an admitted package, declaration, model selection or population binding.

## Inputs

- For `NodeKey`, the admitted QSpec FR-322 structural node id: non-empty owning package name, non-empty path of text or nested node-id segments, and ordinal from 0 through 9007199254740991. A text path segment may itself be empty; the path array may not.
- For `EffectiveId`, the caller-admitted pair `(owner_effective_type, original declaration key)` of QSpec FR-150. For universes, members and populations, the caller-admitted owner, declared name path and ordinal or role components established by the selected model identity contract. The kernel does not infer these from a document, graph or digest.
- For enum and union members, the admitted declaration and member identities described by QSpec FR-141 and FR-441.
- For declared and compound units, respectively the admitted unit node id and the admitted canonical compound-unit term list described by QSpec FR-142.

## Outputs

- Typed `NodeKey`, `EffectiveId`, `UniverseId`, `VariantId`, `MemberId`, `PopulationId` and `UnitId` values that retain the supplied components and remain distinct across their types and, for `UnitId`, domains.
- A typed construction refusal when the supplied components are not a structural identity of the selected kind. No partly constructed identity is returned.

## Behavior

1. **Node key.** The public `NodeKey::new(package, path, ordinal)` constructor accepts an owned non-empty UTF-8 package string, a non-empty sequence of `PathSegment::Text(String)` or `PathSegment::Node(Box<NodeKey>)`, and a `u64` ordinal no greater than 9007199254740991. It returns a typed refusal for an empty package or path or an ordinal above that bound. A negative ordinal cannot be passed through this signature; the QSpec V2 wire schema rejects it before construction. The key order is QSpec FR-322's componentwise order: package by exact UTF-8 bytes, then path element by element (text before nested node id, nested ids recursively by this same order, a proper prefix first), then ordinal. Equal components give equal keys, hashes and ordering, whether the caller supplies them independently or reads them from one admitted package. An imported node retains its own package name, not the importer's alias. The kernel does not check whether the owner or path joins a package. A node id read from an admitted package uses this same constructor, with no separate digest decoder.
2. **Model identities.** `EffectiveId` has a typed constructor accepting QSpec FR-150's exact pair `(owner_effective_type, original declaration key)` and preserves that pair for equality, hashing and componentwise order. A derivation fact's ordinal, rule, inputs or parent is not an `EffectiveId` component: changing one fact without changing the pair leaves the id equal, while changing either pair member changes it. `UniverseId` separately accepts its caller-admitted structural owner, declared name path and ordinal in that owner, retaining those components. Two universes with the same local name and ordinal but different owners remain distinct. `EffectiveId` and `UniverseId` are different types. The model stage supplies these identities; the kernel neither composes a model declaration nor computes a universe component.
3. **Member and population identities.** `MemberId` and `PopulationId` likewise accept caller-admitted structural owner, name path and ordinal or role, retaining exact component equality and order without a digest. In particular, two population bindings with the same local name in different owners remain distinct, and Direct, Pre and Post admission roles of one binding remain distinct. A `Value::Population` remains opaque to the kernel and is still refused by [FR-089](./FR-089-kernel-refuses-a-population-pair.md).
4. **Enum and union member identities.** `VariantId` accepts an admitted enum member's structural `value`/`enum_value` node id for an enum value. For a union value it accepts the pair (union declaration `NodeKey`, exact member name) of QSpec FR-441; a union member key is not itself a graph node id. The two forms remain distinct. A member of another declaration remains distinct even when its member text or rank matches. The union constructor no longer retypes a `NodeKey`'s digest bytes as a `VariantId`; `UnionMember` retains the admitted declaration and member name. A model variant case name may contain any non-empty UTF-8 string admitted by the selected model document and is never forced into the source-identifier character class. An empty member name is refused with a typed result, or is unrepresentable through a validated non-empty member-name type.
5. **Unit identities.** `UnitId::declared` carries the admitted declared unit node identity. The compound arm carries QSpec FR-142's evaluator-owned canonical term list of root-unit node ids and nonzero mathematical integer exponents, ordered by the admitted root-unit keys. The empty list identifies the dimensionless unit. The caller supplies an admitted canonical term list; the kernel does not normalize or validate unit algebra. The two arms remain unequal even if their displayed components coincide, and each arm compares its own components without hashing them. The kernel does not turn a compound value identity into a checked-semantic graph node.
6. **API replacement.** The digest constructors (`from_digest`), `NodeKey::decode_admitted`, fixed digest byte accessors and digest-domain constants are not part of the structural identity API. Their removal is an API change, with no legacy alias or compatibility reader. Callers pass or borrow typed identities instead of assuming that identity values are `Copy`; clone preserves exact identity. The existing authored UTF-8 `ObjectId` stays an authored string identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-389-AC-1 | Two `NodeKey`s built independently from equal admitted `{package, path, ordinal}` components compare equal, hash equally under one hasher and have `cmp == Equal`; changing only the package, a path segment or the ordinal makes them unequal. Keys with different packages order by package UTF-8 bytes; with one package, text segments sort before nested node ids, nested ids use the same order, proper path prefixes sort first, and the ordinal decides only after equal paths. Replacing an imported node's owning package with the importer's alias changes the key. | Test |
| FR-389-AC-2 | A non-empty package and path with an ordinal of 0 or 9007199254740991 construct a `NodeKey` even if the caller has not asked the kernel to resolve that key against a package. An empty package, empty path or ordinal of 9007199254740992 is refused without a key. Reading the same id from an admitted package yields the same key and does not require a second digest-decoding API. A caller that names a foreign package or nonexistent node remains responsible for the join refusal outside the kernel. | Test |
| FR-389-AC-3 | Two `EffectiveId`s made from equal `(owner_effective_type, original declaration key)` pairs compare and hash equally. Changing either pair component changes the id; changing only a derivation fact's ordinal, rule, inputs or parent while keeping the pair fixed does not. Separately, two `UniverseId`s with equal admitted `(owner, name path, ordinal)` components compare and hash equally, and changing any one component changes the id. An effective id cannot substitute for a universe id. Both constructors accept structural components rather than a 32-byte digest, and the kernel recomputes no preimage. | Test |
| FR-389-AC-4 | An enum `VariantId` made from the admitted enum-value node and a union `VariantId` made from an admitted declaration/member pair remain distinct. Two union members with the same member name and different declarations differ; two with the same declaration and distinct case names, including `Gold Plus` and `Café`, differ byte for byte. An empty union member name is refused or unrepresentable by the public constructor. An admitted foreign variant with the same rank is refused by the enum shape, and equal member names from different unions never compare equal in union equality. | Test |
| FR-389-AC-5 | `MemberId` and `PopulationId` identities with equal admitted components compare and hash equally; changing the owner, name path or ordinal/role distinguishes them. Direct, Pre and Post of one population binding are pairwise distinct. Every `Value::Population` remains refused by `admits`, `plan_pairs` and `compare_keys` as FR-089 specifies, regardless of its structural identity. | Test |
| FR-389-AC-6 | A declared `UnitId` constructed from a unit node and a compound `UnitId` constructed from caller-admitted canonical terms are unequal across their domains. Equal admitted components within one domain compare equal; changing a root-unit key or exponent changes the compound identity, and the empty term list identifies the dimensionless unit. The kernel does not infer a compound unit from a checked-semantic node or normalize the supplied terms. | Test |
| FR-389-AC-7 | Every identity constructor and public read path of this requirement uses admitted structural components; the public API has no `from_digest`, `NodeKey::decode_admitted`, fixed digest byte accessor or digest-domain constant for these identities. Compiling a client that uses the new constructors and borrows or clones their results succeeds; attempts to call the removed APIs fail to compile. No old digest shape is accepted through another public constructor. | Test |
| FR-389-AC-8 | For values built from the admitted structural identities, FR-088-AC-11/12, FR-089-AC-6, FR-377-AC-1 through AC-8 and FR-379-AC-1 through AC-9 keep their stated result, refusal, pair count and collection order. The declaration-key ordering control uses `NodeKey::cmp` rather than digest bytes, and the union member control uses the declaration/member pair rather than a retyped node digest. | Test |

## Status

Proposed for IR-1105/IR-1133. The identity behaviors above replace the Planned representation rows of exact PR #98 and the digest-shaped API on current exact main; implementation follows this spec. QSpec #517 and #518 are open at authoring time, and QSpec's model effective-declaration and shared-kernel structural identity rows are still being completed under STD-604. This PR remains draft until those authoritative QSpec changes land and are remeasured; this status does not waive any acceptance criterion.

## Dependencies

- QSpec FR-322 defines the structural `node_id` shape and componentwise order; FR-141 defines enum-value node identity; FR-441 defines the union member pair; FR-142 defines the evaluator-owned compound-unit term identity. These are draft changes in QSpec #517/#518 at authoring time.
- QSpec FR-150 and the shared-kernel structural identity row under STD-604 own the admitted EffectiveId, UniverseId, MemberId and PopulationId components. The kernel holds them and does not mint them.
- The behavioral constraints remain [FR-088](./FR-088-enum-and-unit-identity-in-the-kernel.md), [FR-089](./FR-089-kernel-refuses-a-population-pair.md), [FR-377](./FR-377-composite-canonical-key-across-declarations.md) and [FR-379](./FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md).
