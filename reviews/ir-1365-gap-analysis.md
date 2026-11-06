---
id: SR-2026110101324
title: IR-1365 PR 133 gap-analysis review
type: SpecReview
analysis: gap-analysis
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
```

## Verdict

CONDITIONAL: changed criterion tags exist, but the new branches lack behavioral evidence.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-942 tests only the upper outside value; the lower outside branch and reversed interval constructor are not exercised | src/quantity.rs:352 |

## Coverage

Plan completion: not assessed. Changed FR-395-AC-1 through AC-3 have computed nonignored trace bindings. Optional semantic fanout was not run. The final aggregate gate belongs to the lead.
