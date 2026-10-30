---
id: SR-16676
title: "IR-1148 PR 103 Code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; src/lib.rs, src/node.rs"
review_set: subset
---

## Summary

Ticket: IR-1148. PR #103 frozen head 39d4831a16a7dd834b5c94b1218798bdf58c47fb. Review is limited to the PR diff. No applicable AssuranceProfile was found. Final aggregate gate belongs to the lead.

## Examined scope

- `FR-389-AC-1` (examined): Two `NodeKey`s built independently from equal admitted `{package, path, ordinal}` components compare equal, hash equally under one hasher and have `cmp == Equal`; changing only the package, a path segment or the ordinal makes them unequal. Keys with different packages order by package UTF-8 bytes; with one package, text segments sort before nested node ids, nested ids use the same order, proper path prefixes sort first, and the ordinal decides only after equal paths. Replacing an imported node's owning package with the importer's alias changes the key.
- `FR-389-AC-2` (examined): A non-empty package and path with an ordinal of 0 or 9007199254740991 construct a `NodeKey` even if the caller has not asked the kernel to resolve that key against a package. An empty package, empty path or ordinal of 9007199254740992 is refused without a key. Reading the same id from an admitted package yields the same key and does not require a second digest-decoding API. A caller that names a foreign package or nonexistent node remains responsible for the join refusal outside the kernel.
- `FR-389-AC-3` (examined): Two `EffectiveId`s made from equal `{owner_effective_type, original}` values compare and hash equally, including a `null` owner for an effective type and a recursive owning `EffectiveId` for a member. Changing either component changes the id; changing only a derivation fact's ordinal, rule, inputs or parent while keeping the identity fixed does not. Two `UniverseId`s made from equal least-root declaration `NodeId`s compare, hash and order equally; changing any `NodeId` component distinguishes them. Changing only a non-identity member of the FR-150 object-universe value while the least-root `NodeId` remains equal does not change the `UniverseId`. An effective id cannot substitute for a universe id. Both constructors accept structural components rather than a 32-byte digest, and the kernel recomputes no preimage.
- `FR-389-AC-4` (examined): Enum, union and model-variant `VariantId`s each carry only the admitted `{owner: NodeId, case: identifier}` pair. Equal pairs compare, hash and order equally; changing the owner or case distinguishes them, including the same union member name in two declarations and `Gold Plus` versus `Café` in one declaration. An empty union member name is refused or unrepresentable by the public constructor. An admitted foreign variant with the same rank is refused by the enum shape, and equal member names from different unions never compare equal in union equality. The enum-value node and union admission relation are evidence for the pair, not additional identity components.
- `FR-389-AC-5` (examined): `MemberId`s with equal `{owner, member}` components compare, hash and order equally; changing the owner arm, owner value or exact member identifier distinguishes them, with `node_id` arm before `effective_id` arm. `PopulationId`s with equal population declaration `NodeId`s compare, hash and order equally; changing any `NodeId` component distinguishes them, but changing only binding, anchor, declared maximum or Direct, Pre and Post role does not. Every `Value::Population` remains refused by `admits`, `plan_pairs` and `compare_keys` as FR-089 specifies in each role.
- `FR-389-AC-6` (examined): A declared `UnitId` constructed from a unit node and a compound `UnitId` constructed from caller-admitted canonical terms are unequal across their domains, with `declared` ordered before `terms`. Equal admitted components within one domain compare equal; changing a root-unit key or exponent changes the compound identity, and the empty term list identifies the dimensionless unit. Term arrays order by unit node then exponent numerically, including `2` before `10`. The kernel does not infer a compound unit from a checked-semantic node or normalize the supplied terms.
- `FR-389-AC-7` (examined): Every identity constructor and public read path of this requirement uses admitted structural components; the public API has no `from_digest`, `NodeKey::decode_admitted`, fixed digest byte accessor or digest-domain constant for these identities. Compiling a client that uses the new constructors and borrows or clones their results succeeds; attempts to call the removed APIs fail to compile. No old digest shape is accepted through another public constructor.
- `FR-389-AC-8` (examined): For values built from the admitted structural identities, FR-088-AC-11/12, FR-089-AC-6, FR-377-AC-1 through AC-8 and FR-379-AC-1 through AC-9 keep their stated result, refusal, pair count and collection order. The declaration-key ordering control uses `NodeKey::cmp` rather than digest bytes, and the union member control uses the declaration/member pair rather than a retyped node digest.
- `FR-389-AC-9` (examined): For a `NodeKey` with mixed text and nested-node path segments, a public borrowed view yields the exact package and ordinal and each direct segment once in component order. A text item borrows its original UTF-8 bytes; a nested-node item borrows a view exposing that node's own package, ordinal and direct path, including another nested node. The accessor and iterator do not allocate, clone a string or node, reconstruct an owned path, or mutate the identity.
- `FR-389-AC-10` (examined): A consumer can traverse at least 10,000 nested node levels using the borrowed views and an explicit fallibly reserved stack, producing the same component sequence as the structural identity without calling the owned `NodeKey::path()` accessor. View creation and advancement use bounded call-stack depth independent of nesting; if the consumer's stack or output reservation fails, it can return its own typed allocation refusal before emitting an incomplete result.
- `FR-389-AC-11` (examined): Given an already owned `String` package and `Vec<PathSegment>` path, `NodeKey::try_new` accepts the same package, path and ordinal shape as `NodeKey::new`, including the boundary ordinals, and produces an equal, equally hashed and ordered key on success. Empty package, empty path and above-bound ordinal retain AC-2's distinct shape refusals. Failure of any reservation during path flattening yields a distinct typed allocation refusal and no key; no infallible conversion or reservation is hidden inside the fallible method. A caller starting from `&str` can use its own fallible package copy before calling it.
- `FR-389-AC-12` (examined): `NodeKey::try_clone` and `PathSegment::try_clone` reproduce every component of text and nested-node inputs, including 10,000-level paths, with equal hash and order where applicable and bounded call-stack depth. Failure of any storage reservation actually performed, including nested-node carrier storage if the representation requires it, yields a distinct typed allocation refusal and no copy; the source remains unchanged. Neither method delegates an allocation to infallible `Clone`, `Box::new`, `Vec::push` without prior reservation, or an equivalent operation.

- `TC-938` (examined): public structural identity controls, including borrowed traversal and allocation refusal.

## Verdict

**PASS — The changed Rust API is deliberate. `NodeKey::try_new`, `NodeKey::try_clone`, and `PathSegment::try_clone` reserve fallibly, retain distinct shape/allocation refusals, and use a flat carrier for bounded call-stack depth. `NodeKeyView`/`NodePathIter` borrow without allocation. No production code duplication or new digest path was found.**

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Code-review includes the Rust lane. Inspected every changed reservation, direct and nested segment carrier, public re-export, deep traversal, failure-injection tests, QSV consumer boundary and repo idioms. Focused author evidence exists; reviewer did not run aggregate gates.
