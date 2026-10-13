---
id: SR-8701
title: "QSL-500 quire-exact PR 17 gap analysis at a1f6450: union ACs to tests"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@a1f6450d23225c37f7bb83e657ec0b3224252ced; spec/functional/FR-376, FR-377, FR-378; spec/test-cases/TC-924, TC-925, TC-926; tests/union.rs; tests/union_declarations.rs; src/value.rs; src/equality.rs; src/key.rs"
review_set: subset
---

## Summary

Ticket: QSL-500 (QSL-737 AC-5 follow-through). Planless gap analysis of the
union ACs against their tests, with an oracle-strength check. Plan completion:
not assessed. The computed matrix (`quire matrix`, quire 0.36.2, engine
0.50.2) at this head lists FR-376-AC-9 (five tests), FR-377-AC-8 (two tests)
and FR-378-AC-8 (one test) as tagged. The other untagged and
method-without-symbol rows (FR-369, FR-371 to FR-375, and others) predate this
PR and are outside it.

Oracle strength of the new tests in tests/union_declarations.rs:

- FR-376-AC-9: P/C, C/G and P/G pairs, in both orders, give `false` with pair
  count 2. A two-position payload with a mismatch in the first position gives
  3 pairs, which shows the traversal does not stop early. An `Option` wrapper
  gives 3 pairs. Reference slots of different universes give `false`, and a
  same-declaration control with the same universes refuses
  `ForeignReference`, so the absence of the refusal is not vacuous. The
  equal-declaration control checks a pair count of 1 plus the bare count.
  `Integer` against a union, and one member with payload lengths 1 and 2,
  refuse `ValueKindMismatch`. Treating different-declaration unions like
  composites would break `foreign_union_enum_and_composite_kinds_remain_distinct`.
- FR-377-AC-8: P before C and C before G, with a large-slot P against a
  small-slot C. `Set` forms through `form` from both supplies and keeps P then
  C. `OrderedSet` keeps supply order. See FND-002 for the "every pair" clause.
- FR-378-AC-8: `UnionValue::from_admitted` keeps the C payload (declaration
  and slot count). `union` refuses `Position(0)` `TypeMismatch`, and
  `evaluate_union` refuses `DeferredResultNotAdmitted`. Both accept P.

In tests/union.rs, the kernel tests are strong: identifier-ASCII key order
does not depend on digest or declaration order, a `Set`, `Bag`, `OrderedSet`
and `Sequence` each form from forward and reversed supply, the first stop wins
with no later call, one retain charge with occurrences 3, the retain limit is
named, and arity is refused before any call. Clone, compare, key, format and
drop run 10,000 levels deep on a 512 KiB stack, storage grows linearly, and
shared children count each occurrence. Those 14 tests trace only to QSL
criteria, which FND-001 records.

## Verdict

FINDINGS: one medium, one low. The three new ACs are covered by tests that can
fail. The union carrier itself has no quire-exact requirement.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | No quire-exact FR specifies the kernel union carrier: `Value::Union`, `union`, `evaluate_union`, identifier-byte key order, the member, length and declaration refusal rules, and the iterative depth paths. The 14 tests in tests/union.rs tag only QSL FR-319, FR-321, FR-322 and FR-323 ids. This repo's computed matrix lists none of them, and the QSL ids share a number space with this repo's own FRs (FR-088 and FR-262 exist in both with different meanings). Several bindings are partial: FR-321-AC-4 also requires admission, conversion and a named limit; FR-319-AC-3 is "refused by type checking", not a kernel refusal; FR-319-AC-2 is about `VariantId` minting. Main has a precedent for borrowed ids (FR-142 to FR-147). | tests/union.rs:103-604 |
| FND-002 | low | FR-377-AC-8 requires that "`Some(Equal)` agrees with `planned_equality` on every pair", and TC-925 step 8 says "each ordered pair". The test checks three pairs, (P,C), (P,P200) and (P,P), not every ordered pair of the P, C and G unions. | tests/union_declarations.rs:186-188 |

## New findings (disposition pass 1)

Reviewed at agent-ix/quire-exact@da4941069d740eb18a2463b66b5f52d546c69989. `quire matrix` lists FR-379-AC-1 to AC-9 as tagged. The 14 union.rs tests now carry FR-379 tags.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | medium | FR-379-AC-6 says "Equal identifiers with different `VariantId`s give `None`", and TC-927 step 6 calls it, but no test asserts it. Deleting the `left.variant() != right.variant()` check in the equal-identifier branch of `leaf` would leave every test green, and would give `Some(Equal)` for a pair that equality calls `false`. | tests/union.rs:188-212; src/key.rs:169-172 |
| FND-004 | low | FR-379-AC-5 requires that one member with payload lengths 1 and 2 "refuses `ValueKindMismatch`". The AC-5-tagged tests assert only the key `None` for that pair. The refusal is asserted only in `unlike_union_pairings_stay_checked_invariant_faults`, which is tagged FR-376-AC-9. | tests/union_declarations.rs:213-234 |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | da49410 |
| FND-002 | fixed | da49410 |
