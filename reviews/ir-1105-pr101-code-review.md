---
id: SR-2026102763
title: "IR-1105 exact PR 101 code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact PR #101 at 79950c8cdd0ea67c271d92ab843711f914997548; src/equality.rs, src/identity.rs, src/key.rs, src/lib.rs, src/location.rs, src/node.rs, src/outcome.rs, src/quantity.rs, src/reference.rs, src/value.rs, src/value/value_type.rs, tests/composite_declarations.rs, tests/ir1051_bounded_quantity_type.rs, tests/structural_identity_public.rs, tests/union.rs, tests/union_declarations.rs, spec/functional/FR-389-carry-admitted-structural-identities.md"
review_set: subset
---

## Summary

The scoped diff carries admitted structural NodeKey and value identities through the public API. QSpec #517 de755df and #518 43fcf4 were checked for identity members, ordering, and SI-01 through SI-06 vector cases. The 10,000-depth tests use a 512 KiB stack; their execution remains pending under the shared build lock.

## Verdict

PASS — no findings in the changed code or FR-389 criterion bindings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

- FR-389-AC-1 (examined): tagged; 2 traced test binder(s).
- FR-389-AC-2 (examined): tagged; 1 traced test binder(s).
- FR-389-AC-3 (examined): tagged; 2 traced test binder(s).
- FR-389-AC-4 (examined): tagged; 1 traced test binder(s).
- FR-389-AC-5 (examined): tagged; 1 traced test binder(s).
- FR-389-AC-6 (examined): tagged; 1 traced test binder(s).
- FR-389-AC-7 (examined): tagged; 3 traced test binder(s).
- FR-389-AC-8 (examined): tagged; 5 traced test binder(s).

## Method and limits

Rust review included API visibility, componentwise Eq/Ord/Hash, flat depth safety, caller admission boundaries, no digest compatibility path, and the changed value/equality/key call sites. No CI workflow or gate was narrowed. No tests or full gates were run by this reviewer; focused tests were queued by the coder.

## Post-review delta at 97c5343

The same PR changed only `UniverseId` storage to `Box<NodeKey>`, the `UnitId` debug formatter, a matching `Refusal` doc comment, and three test ownership fixes. `Box<NodeKey>` delegates equality, ordering, and hashing to the node; the public `new` and `root_type` signatures and QSpec FR-201 component remain the same. The custom `UnitId` formatter keeps its nested representation compact inside the FR-387 alternate bounded-quantity layout. The FR-387-AC-6 bounded quantity debug test and the other ten bounded quantity tests passed in the coder's focused log. The coder's Clippy log exits 0, and `git diff --check c295996..97c5343` is clean. No production identity component, AC, CI gate, or digest compatibility path changed. The lead owns the final full gate. No real finding was recorded in the original review, so no finding disposition row is needed.
