---
id: TC-938
title: "Admitted structural identities replace kernel digest constructors"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-389
    type: verifies
---
# TC-938: Admitted structural identities replace kernel digest constructors

## Description

Verify [FR-389](../functional/FR-389-carry-admitted-structural-identities.md) through the public exact API with caller-authored structural identities. Use the authoritative QSpec identity contracts and selected producer outputs for the final admitted-component cases; this repository stores no copy of another repository's schema or vector.

## Test Procedure

1. Construct `NodeKey` values twice from one admitted `{package, path, ordinal}` and once for each single-component change. Use two packages whose UTF-8 names have known order, a path whose first segment is text against one whose first segment is a nested node id, a prefix path against its extension, and equal paths with different ordinals. Compare equality, hash under one hasher and `cmp` in both directions. Use an imported node with its owning package name and then the importer's alias as a control. Attempt an empty package, an empty path, ordinals 0 and 9007199254740991, and the above-bound ordinal 9007199254740992 at `NodeKey::new`. Read the original id from an admitted package and construct its key through the same API.
2. Construct pairs of `EffectiveId` and `UniverseId` values with equal components and with each owner, name-path and ordinal component varied separately. Compare equality, hash and order. Compile a client that holds one of each type and cannot substitute one for the other.
3. Construct an enum `VariantId` from an admitted `value`/`enum_value` node and union `VariantId`s from `(declaration node, member name)` pairs. Use one member name in two declarations; in one declaration use `Gold Plus` and `Café`; attempt an empty member name. Try an enum foreign member at a matching rank. Compare union values whose member names match but declarations differ.
4. Construct `MemberId`s with equal and differing admitted owner/name/ordinal components and `PopulationId`s with equal and differing owner/name/role components, including Direct, Pre and Post. Exercise the public population admission, equality planning and key comparison paths.
5. Construct declared `UnitId`s from admitted unit node keys and compound `UnitId`s from caller-admitted canonical term lists. Compare equal and differing values of each domain, cross-domain values, a changed root key, a changed nonzero exponent and the empty dimensionless list. Keep normalization and term admission outside the kernel.
6. Compile a consumer that constructs, borrows and clones each typed identity, then constructs an enum, union, quantity, reference and composite value. Check that calls to `from_digest`, `NodeKey::decode_admitted`, fixed digest byte accessors and digest-domain constants fail to compile. Run the existing FR-088, FR-089, FR-377 and FR-379 behavioral controls with structural identities.

## Expected Results

1. Equal components give equal, identically hashed and equally ordered `NodeKey`s; the listed component changes give distinct keys and QSpec FR-322 componentwise order. The importer's alias gives a different key from the imported owner's package name. Ordinals 0 and 9007199254740991 construct; empty package, empty path and 9007199254740992 refuse with typed errors. The admitted read returns the same key without a second digest decoder. Kernel construction does not assert that the package or node exists.
2. Each identity type preserves equality, hash and component order of its admitted structural components; each one-component change distinguishes the values; the two types cannot be interchanged.
3. The enum and union forms remain distinct; union names compare byte for byte and their declaration is part of identity. An empty member name is refused with a typed result or cannot be represented by the constructor's validated input type. The foreign enum member is refused even at the matching rank, and matching member names of different union declarations remain different.
4. Equal members and populations compare equally; each changed component distinguishes them, including all three role pairs. The population paths retain FR-089's refusals.
5. Equal identities within a domain compare equally and every specified changed component distinguishes them; declared and compound domains never compare equal, and the empty compound list identifies the dimensionless unit. The exact constructor does not normalize terms or derive a compound unit from a graph node.
6. The structural client compiles, all removed digest API calls do not, and the existing admission, equality, key ordering, pair count and refusal controls preserve their stated results.

## Status

Planned with FR-389 after the authoritative QSpec structural identity stack lands. QSpec #517/#518 are open at authoring time; the exact kernel code and the caller-owned admission checks are not claimed complete by this test plan.
