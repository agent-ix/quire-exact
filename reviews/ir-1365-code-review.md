---
id: SR-2026110101323
title: IR-1365 PR 133 code-review review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; PR #133; spec/functional/FR-395-bounded-quantity-result-refusal.md; spec/test-cases/TC-942-bounded-quantity-result-refusal.md; src/lib.rs; src/outcome.rs; src/quantity.rs"
review_set: subset
---

## Summary

Ticket: IR-1365. Independent diff-scoped review of PR #133. No applicable AssuranceProfile found in the repository spec tree.

## Analysis

Reviewed the new requirement, test case, public result interval, refusal mapping and traced unit tests. The existing BoundedQuantityType and metered rational comparison path were read as context. No CI workflow, dependency, vendored artifact, or compatibility change is in the diff.

## Examined Scope

```yaml
scope:
- id: FR-395-AC-1
  path: spec/functional/FR-395-bounded-quantity-result-refusal.md
  role: examined
  excerpt: For a target `Quantity<U>[-1/1, 1/1]`, exact results at `-1/1` and `1/1`
    complete in `U`; a result of `2/1` refuses with no value.
- id: FR-395-AC-2
  path: spec/functional/FR-395-bounded-quantity-result-refusal.md
  role: examined
  excerpt: The refusal from `2/1` carries `U`, `-1/1` and `1/1` as typed values, returns
    `rational_out_of_domain`/`outside-domain`, and contains no converted magnitude.
- id: FR-395-AC-3
  path: spec/functional/FR-395-bounded-quantity-result-refusal.md
  role: examined
  excerpt: The old `RationalOutOfDomain` variant still carries its `RationalDomain`
    and returns `rational_out_of_domain`/`outside-domain`.
- id: TC-942
  path: spec/test-cases/TC-942-bounded-quantity-result-refusal.md
  role: examined
  excerpt: Verify FR-395-AC-1 through FR-395-AC-3 through the public kernel API.
- id: FR-387-AC-2
  path: spec/functional/FR-387-bounded-quantity-value-type.md
  role: context_only
  excerpt: '`new` returns `Err` with cause `MalformedBoundedQuantityType` for a `min`
    quantity in a unit other than `U` and for a `max` quantity in a unit other than
    `U`, and for no other input; it returns `Ok` for `min` equal to `max`, for `min`
    3, `max` 5, and for inverted bounds `min` 5, `max` 3 (and `min` 4, `max` 2), which
    are valid empty types whose accessors return the arguments, including fractional
    magnitudes such as `1/3` and `1/2`.'
```

## Verdict

FAIL: unmetered exact comparisons on public caller input.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The new result interval performs unmetered rational cross multiplication before any bounded admission; caller supplied large endpoints can consume unbounded work and memory | src/quantity.rs:54 |
| FND-002 | medium | The result admission repeats unmetered rational comparisons over caller supplied magnitudes; its documented caller charge cannot bound this work | src/quantity.rs:92 |

## Coverage

Plan completion: not assessed. Changed FR-395-AC-1 through AC-3 have computed nonignored trace bindings. Optional semantic fanout was not run. The final aggregate gate belongs to the lead.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | QuantityResultInterval::new now stores the supplied endpoints without comparing them. |
| FND-002 | fixed | admit_quantity_result now calls the metered order_rationals helper for both endpoint comparisons. |
