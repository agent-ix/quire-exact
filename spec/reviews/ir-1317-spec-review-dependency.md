---
id: SR-2026110101307
title: IR-1317 combined dependency review
type: SpecReview
analysis: dependency
scope: 'agent-ix/quire-exact PR #129 and agent-ix/quire-eval PR #19; IR-1317; membership,
  checked collection removal and quantity-power descriptors'
review_set: all
---

## Summary

One combined independent review of quire-exact PR #129 and quire-eval PR #19 for IR-1317. Exact owns canonical float-leaf collection membership; quire-eval owns collection removal and the three checked descriptors. Ordinary equality remains separate. QuantityOperation::Power already belongs to quire-semantic-value; the new checked descriptor deliberately supplies no second arithmetic implementation. Public package admission and source/producer qualification are not inferred from descriptor controls.

## Verdict

**PASS**. The dependency graph is acyclic: QSpec collection semantics -> Exact membership -> eval removeOne; canonical formation also precedes eval dropFirst. QSV quantity power and QSpec package admission precede the checked power descriptor consumer. Exact membership and checked descriptors are enablement; removal functions are the consumer feature. No RT/IR/QSL backedge or producer dependency was introduced.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

No applicable AssuranceProfile exists in either spec tree. Existing ArchitectureDescription documents provide context but are not enforced review profiles. This reviewer ran no Cargo, Kani, CBMC or other heavy jobs. Supplied final own-lane logs were inspected: quire-eval all-feature workspace 74 pass / 0 fail / 0 ignored; Exact default 402 pass / 1 inherited fail / 2 ignored; Exact test-support 414 pass / 1 inherited fail / 2 ignored. The only failure is value::value_type::tests::value_type_debug_preserves_structure_and_fields, reproduced on clean main with identical actual/expected Debug strings. Formatter, native/bare-metal Clippy, no_std builds, warning-denied docs and deny checks pass in both repositories. All fourteen scoped criterion bindings are present and no new ignored test or gate suppression was introduced. The thirteen untagged Exact criteria are unchanged baseline obligations, not newly discharged here. No plan was supplied.

Plan completion: not assessed

The optional standalone semantic gap fan-out and Jev-dependent criterion-strength analysis were not run; neither was requested or needed for this bounded mixed diff. Required code/spec intent checking was performed directly in this one reviewer invocation. No child agent was spawned.

Examined scope:

```yaml
scope:
- id: FR-377-AC-6
  path: quire-exact/spec/functional/FR-377-composite-canonical-key-across-declarations.md
  role: examined
  excerpt: A composite against an `Integer`, two composites of one declaration key
    and different slot counts, and a pair of `Float` members each return `None` from
    `compare_keys`. As the two members of a `Set`, each refuses `CanonicalKeyUnavailable`
    through `form_grouped`. Through `form`, the composite/integer and mismatched-slot
    pairs refuse `ValueKindMismatch` before ordering; the distinct float pair passes
    bit-identity membership then refuses `CanonicalKeyUnavailable` at ordering.
- id: FR-393-AC-1
  path: quire-exact/spec/functional/FR-393-collection-float-membership.md
  role: examined
  excerpt: At both widths identical NaN patterns compare true; changed payload, sign,
    signaling bit, signed zero and infinity sign compare false.
- id: FR-393-AC-2
  path: quire-exact/spec/functional/FR-393-collection-float-membership.md
  role: examined
  excerpt: Float leaves nested in options and collections use the same membership
    relation, visiting all occurrence pairs after an unequal leaf.
- id: FR-393-AC-3
  path: quire-exact/spec/functional/FR-393-collection-float-membership.md
  role: examined
  excerpt: Nested comparisons retain the exact walk/test charge vectors and no result-retain
    charge; denial at either point returns Incomplete with the admitted prefix unchanged.
- id: FR-393-AC-4
  path: quire-exact/spec/functional/FR-393-collection-float-membership.md
  role: examined
  excerpt: Wrong width and wrong value kind preserve typed membership refusals; ordinary
    equality still refuses direct and nested float operands.
- id: FR-003-AC-1
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Borrowed reads distinguish both operations and preserve each ordered child
    handle and checked type/location metadata.
- id: FR-003-AC-2
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Drop removes just the leading sequence/ordered-set occurrence, preserving
    duplicates/order and the empty case; set/bag operands refuse before charging.
- id: FR-003-AC-3
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Remove deletes exactly the first match for every collection kind, reducing
    bag multiplicity by one, and preserves all occurrences on misses and empty inputs.
- id: FR-003-AC-4
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Both operations preserve kind/element type and compute the declared bounded
    or unbounded result type, including zero and maximum bounds.
- id: FR-003-AC-5
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: A wrong-type removal probe refuses before charges; canonical type-environment
    admission remains authoritative.
- id: FR-003-AC-6
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: Drop charges exactly survivor formation; remove charges the contains scan
    then formation, with a bag's distinct-member visiting rule and no Boolean result
    retain.
- id: FR-003-AC-7
  path: quire-eval/spec/functional/FR-003-shrink-checked-collections.md
  role: examined
  excerpt: A denied scan or formation charge preserves the admitted charge prefix
    and exposes no partial result.
- id: FR-004-AC-1
  path: quire-eval/spec/functional/FR-004-quantity-power-node.md
  role: examined
  excerpt: A quantity-power read retains the distinct operation and mapped base/exponent
    handles in that order, including when the source handles have descending indices.
- id: FR-004-AC-2
  path: quire-eval/spec/functional/FR-004-quantity-power-node.md
  role: examined
  excerpt: The read borrows the original checked type and location without reconstructing
    either metadata value.
```
