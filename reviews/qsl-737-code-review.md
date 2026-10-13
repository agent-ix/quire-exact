---
id: SR-8686
title: "QSL-737 quire-exact PR 39 code review: composite equality and keys across declarations"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@17abd1ac1560ed855502bb0f6883b415428593bf; src/equality.rs; src/key.rs; src/collection.rs; src/value.rs; tests/composite_declarations.rs; spec/functional/FR-376, FR-377, FR-378 status lines"
review_set: subset
---

## Summary

Ticket: QSL-737. Code review with the rust-review lane folded in, over the
diff of quire-exact PR 39 (one commit on main 714b8e8). Reviewer-only: no
build or test run, per the brief. The PR states `make ci` is green; that is
not re-measured here.

Checked:

- `plan_pairs` (src/equality.rs:234-241): a new first guard returns `false`
  for two composites whose declaration keys differ, counted as one pair, with
  no slot pushed. The same-declaration arm now guards only on slot count, so
  same declaration with different slot counts still falls to the
  `ValueKindMismatch` catch-all. Composite against non-composite, two
  populations (`PopulationPair`) and collection kind mismatch
  (`CollectionKindMismatch`) arms are unchanged. The declaration check runs
  before any slot is pushed, so no `ForeignReference` can come from skipped
  slots. Traversal does not short-circuit, so FR-375-AC-2 holds.
- `compare_keys` leaf (src/key.rs:148-162): different declarations order by
  `NodeKey::as_bytes()` lexicographic comparison, before slots. With the
  same-declaration slot walk this is a lexicographic order on
  (declaration bytes, slots), so it is total, antisymmetric and transitive on
  well-formed composites. Equal keys occur only for one declaration with equal
  slot keys, which matches the equality plan.
- `sort_by_key` uses `slice::sort_by` (stable, O(n log n) comparisons) and is
  only reached for `Set` and `Bag`; `OrderedSet` and `Sequence` keep supply
  order. `coalesce` charges `collection.member-walk`/`member-test` per
  candidate-member comparison for `Set`, `Bag` and `OrderedSet`; a
  cross-declaration comparison is one pair.
- Same-declaration composites: equality and key behaviour are unchanged.
- Doc comments: the FR-378 Behavior 8 text is present on `form`,
  `form_grouped`, `OptionValue::from_admitted`, `from_admitted_slots` and
  `ValueType::admits`; `construct_collection` and `form_collection` docs
  correctly say they use the exact `admits`. The stale "which a checked
  program never produces" wording is gone from `compare_keys` and
  `plan_pairs`. FR-369's `ValueKindMismatch` row and FR-369-AC-2 already
  carry the FR-376 wording on main.
- Rust: no `unsafe`, no new panic or index in src, no integer conversion,
  iteration stays on an explicit work stack. Tests carry `/// Trace:` and
  `#[trace("TC-92x", ...)]` per repo convention; charge-log assertions are
  behind `#[cfg(feature = "test-support")]`, and each such test also asserts
  ungated `admission_count`/`consumed` values.
- `Implemented` status edits: accurate. No constructor was added, as the
  FR-378 status says.
- Public-repo rules: SPDX header on the new test file, no private text, no
  local paths, no build artifacts in the diff. Consistent with
  CONTENT_RIGHTS.md.

## Verdict

PASS WITH FINDINGS (two low). The behaviour change is small, correct and
matches FR-376 and FR-377. Equality and key stay consistent on every pair the
equality plan completes. The findings are cosmetic.

Observation, not a defect: because a cross-declaration pair visits no slots, a
`Set` of `Composite(P)` holding a `P` and a `C` member that each carry a
`Float` slot now forms. Two same-declaration members with `Float` slots still
refuse. This follows FR-376 Behavior 2 as written.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The same-declaration composite arm of the key `leaf` still guards on `left.declaration() == right.declaration()`, which the new preceding arm already implies. `plan_pairs` dropped the same redundant guard, so the two matches now read differently for one rule. | src/key.rs:155-157 |
| FND-002 | low | The edited `compare_keys` rustdoc left one line unwrapped (96 chars, against about 78 for its neighbours): "of equal rank and different variants, a `Float`, or a population. `pub`: an evaluator groups". | src/key.rs:60 |
