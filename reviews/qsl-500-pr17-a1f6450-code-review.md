---
id: SR-8700
title: "QSL-500 quire-exact PR 17 code review at a1f6450: kernel union carrier"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@a1f6450d23225c37f7bb83e657ec0b3224252ced; src/value.rs; src/equality.rs; src/key.rs; src/lib.rs; tests/union.rs; tests/union_declarations.rs; reviews/qsl-500-code-review.md; reviews/qsl-500-gap-analysis.md"
review_set: subset
---

## Summary

Ticket: QSL-500 (with the QSL-737 AC-5 follow-through). Code review with the
rust-review lane folded in, over the whole union implementation of quire-exact
PR 17 at its head after the merge of main. This head was never reviewed
before; the two review files carried in `reviews/` cover older heads and were
read as data only. Reviewer-only: nothing was built or run here. The coder
reports `make ci` green on this head (fmt-check, clippy for all targets, the
features and the no_std target, test, the no_std build, deny and docs); the
coder's log shows all of these passing.

Checked:

- `UnionMember`, `UnionValue`, `union`, `evaluate_union`, `check_union_arity`
  (src/value.rs). Arity is refused before any argument runs. Positions run in
  order under the first-stop rule through the same `admitted` helper that
  `evaluate_tuple` uses. A completed union charges `composite.result-retain`
  once, sized by `occ`, which is 1 plus the payload occurrences.
- Iterative paths. `Drop for UnionValue` and the `Value::Union` arm of
  `release_children` hand the payload to the `drop_nested` worklist through
  `Arc::into_inner`, so a shared node is only decremented. `defer_node` queues
  unions. `Debug` goes through `Step::Union` on the `DebugWriter` worklist. The
  member is a leaf with no nested values. `Clone` is one `Arc` per level. There
  is no recursion on any new path.
- Equality (`plan_pairs`). Two unions of one declaration and one member
  queue their payload positions in order and count one pair for the union.
  Different members give `false` with one pair. One member with different
  payload lengths refuses `ValueKindMismatch`. Different declarations, and a
  union against any other variant, fall to the catch-all and refuse
  `ValueKindMismatch`. Payload composites of different declarations give the
  FR-376 `false` through the existing composite arm.
- Keys (`leaf` in src/key.rs). Members order by the identifier's ASCII bytes,
  then by payload position. Equal identifiers with different `VariantId`s,
  equal `VariantId`s with different identifiers, or different payload lengths
  give `None`. Different declarations give `None`. On every pair whose
  admission is well formed, `Some(Equal)` occurs exactly when equality is
  `true`. Unions never meet another kind with `Some`, so the total order holds
  within one keyed type.
- `ValueType::admits`: `Composite(K)` admits a union of declaration `K`. The
  catch-all is now spelled out per variant, so a new `Value` variant cannot
  slip through it silently.
- `#![forbid(unsafe_code)]` holds. The library code adds no `unwrap`,
  `expect`, indexing or `as` cast. The no_std surface is `alloc`-only.
  `Value` and `UnionValue` derive neither `Hash` nor `PartialEq`;
  `UnionMember` derives `Eq` over all three fields, which is consistent.
- Union of different declarations (asked in the brief). This is not a gap
  against FR-376 or FR-377. The FR-376 rule exists because a record value
  whose most-specific declaration is a descendant can sit at an ancestor-typed
  slot. A union declaration has no descendant, and QSL FR-319-AC-3 makes
  comparing `A::X(1)` with `B::X(1)` a type error. Two unions of different
  declarations therefore never share one static type in a checked program,
  and refusing them as a checked-invariant fault (`ValueKindMismatch`, key
  `None`) is the right response. The spec does not say so; the spec review
  (SR-8702 FND-001) records that.
- Public-repo rules: CONTENT_RIGHTS.md forbids local workstation paths, and
  the review files carried in this PR break that rule (FND-001). The test
  fixtures copy values from a private repository (FND-002).

## Verdict

FINDINGS: two high, one low. The union implementation is correct.
Equality, keys, charges, drop, debug and clone are iterative and consistent,
and the evaluation order, arity and admission refusals match `evaluate_tuple`.
The two high findings concern public-repo content, not behaviour. Both must
be fixed before merge.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The two carried review files put local workstation paths (a home-directory tool path, a worktree path and a harness root path) into a public repo, against CONTENT_RIGHTS.md. They are also stale for this head: they review a15c465 and say "No spec diff exists, so spec-review and all spec sub-analyses were skipped", which is false at a1f6450. | reviews/qsl-500-code-review.md:37,43; reviews/qsl-500-gap-analysis.md:36,42 |
| FND-002 | high | tests/union.rs embeds four 32-byte digests (the `declaration()`, `circle()` and `empty()` keys, and the foreign union keys) copied from a vector file in a private repository. The comment says "Published QSpec node-identity union_member_vectors", but no public main carries them. The kernel mints nothing, so the bytes are opaque and any fixture bytes would serve, as `fixture()` already shows. | tests/union.rs:29-55,183-189 |
| FND-003 | low | The `drop_nested` rustdoc still lists the node structs as `OptionValue`, `CompositeValue` and `CollectionValue`, without `UnionValue`. The `Value` rustdoc was updated to "four node structs". | src/value.rs:570-571 |
