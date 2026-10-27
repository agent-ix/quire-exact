---
id: SR-16647
title: "IR-1046 Code and Rust review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; PR #88 at fe8bc4cda9488838086a15c3a736fc1f9b8e5ef6; spec/functional/FR-386-map-value-type-with-text-keys.md, spec/test-cases/TC-935-map-value-type-with-text-keys.md, src/collection.rs, src/value.rs, src/value/value_type.rs, tests/ir1046_map_type.rs"
review_set: subset
---

## Summary

IR-1046 PR #88 exact frozen source fe8bc4cda9488838086a15c3a736fc1f9b8e5ef6. Code and Rust review examined FR-386-AC-1 through AC-6 and TC-935. One scoped finding.

## Examined scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-386 | examined | Debug prints a map type as `Map(MapType { values: .., bound: .. })` in compact and alternate mode. |
| FR-386-AC-1 | examined | `ValueType::map(MapType::new(V, bound))` returns a `ValueType::Map` whose `MapType::values()` equals `V` and whose `MapType::bound()` equals `bound`, for `bound` both `None` and `Some`. |
| FR-386-AC-2 | examined | Two map types are equal exactly when both value type and bound are equal. Maps differing only in value type, only in bound, and only in bound presence (`None` against `Some([0, u64::MAX])`) are unequal, and equal maps hash equal. A map is unequal to every `Collection` type of every `CollectionKind` over the same value type and bound, and to `Option` of the same value type. |
| FR-386-AC-3 | examined | A map whose value type is a map, an `Option` or a `Collection` compares by its whole nested chain: two such types differing only at the innermost link are unequal, and equal chains hash equal. |
| FR-386-AC-4 | examined | On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Map` links around `Boolean`, and one of 100,000 links cycling `Map`, `Option` and `Sequence`, clone, compare equal to their clones, hash equal to their clones, format for debug with balanced brackets, and drop. Two chains differing only at the innermost link are unequal. |
| FR-386-AC-5 | examined | `Debug` of `Map<Text, Boolean>` and of a bounded `Map<Text, Option<Boolean>>[1, 3]` prints exactly `Map(MapType { values: Boolean, bound: None })` and `Map(MapType { values: Option(Boolean), bound: Some(CardinalityBound { minimum: 1, maximum: 3 }) })`. |
| FR-386-AC-6 | examined | `ValueType::admits` on a `Map` type returns `false` for a collection value, a text value and an option value. |
| TC-935 | examined | Verify FR-386 through the public ValueType::map, MapType, ValueType::admits and ValueType traits. |

## Verdict

CONDITIONAL — one medium finding requires same-PR repair.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Tests require unequal types to have different DefaultHasher results, although Rust Hash and FR-386 require only equal types to hash equally. A correct collision would fail the suite; remove assert_ne! hash controls at lines 68, 74, 87, 91, 120 and 170 while retaining inequality and equal-hash controls. | tests/ir1046_map_type.rs:68-74 |

## Coverage

Plan completion: not assessed. Quoin matrix found all six new criteria statically tagged; no run evidence was present. Existing unrelated repository gaps are outside this PR review.
No applicable AssuranceProfile was found under spec/. FR-097 and FR-262 are local dependencies; QSpec FR-154-AC-27 and FR-415 establish the external Text-key and clock-collection meaning. No map Value implementation is claimed.
Quire validation reported 87/87 grammar-clean documents. cargo fmt --check passed. Focused Cargo test was queued behind the shared build lock and cancelled without executing; the lead owns the final code gate.
Spec-object-review has no SpecReview analysis enum and is not designed for this kernel-type diff; its object completeness questions were covered in base and integrity. Criterion-strength requires Jev and was skipped.

## Disposition-round delta audit

Reviewed fix head `73ad15b4c604ab16f41a8023ad8026f9b16bdf55` against frozen `fe8bc4cda9488838086a15c3a736fc1f9b8e5ef6`. New FR-386-AC-7 examined: `Debug` in alternate mode (`{:#?}`) of `Map<Text, Boolean>` and of the bounded `Map<Text, Option<Boolean>>[1, 3]` prints exactly what `#[derive(Debug)]` pretty-printing prints: `Map(\n    MapType {\n        values: Boolean,\n        bound: None,\n    },\n)` and the same nesting, four spaces per level with a trailing comma on every field, for the bounded type. TC-935 step 7 and the trace-tagged alternate-Debug test agree with that criterion. The real Text construction and removal of unequal-hash assertions introduce no new scoped issue. Quire reports all seven FR-386 criteria tagged and 87/87 grammar-clean documents; no Cargo command ran in this disposition pass.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The fix deletes all six unequal-hash assertions while retaining equal-hash and unequal-type controls. Verified at `73ad15b4c604ab16f41a8023ad8026f9b16bdf55`. |

After excerpt (`FND-001`, exact fixed source):

```text
    for other in &differing {
        assert!(reference != *other);
    }
```
