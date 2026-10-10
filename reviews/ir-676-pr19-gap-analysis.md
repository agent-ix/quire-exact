---
id: SR-5061
title: "Gap analysis of IR-676 PR 19"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@0fb208955aeeca6d6f7599f2e36e4425ace7b7eb; FR-358-AC-12/13; TC-906; src/accounting.rs"
review_set: subset
---

## Summary

The computed Quire matrix binds both new acceptance criteria to the new tests. Direct code-to-test inspection found one semantic evidence gap in the AC-13 cancellation-first control. No added production behavior lacks the changed FR-358 criteria.

## Verdict

**CONDITIONAL** — one medium evidence gap. Existing repository-wide coverage debt is separate from this PR.

## Examined Units

- `FR-358-AC-12` (examined): Exact injected record under two work limits for ordinary and plan charges.
- `FR-358-AC-13` (examined): Injection before ordinary shortages; cancellation before injection.
- `TC-906-Step-9` and `TC-906-Step-10` (examined): The declared procedures for both new criteria.
- `src/accounting.rs` (examined): New tagged tests and the production branches they exercise.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | medium | AC-13 has a tagged test but no discriminating observation that cancellation did not consume the selected injection. The cancelled refusal's fields equal an injected refusal's fields, and the test does not retry with a live handle. The binding is real for the ordinary-limit cases but incomplete for cancellation precedence. | src/accounting.rs:1284-1306; FR-358-AC-13 | correct-requirement-no-evidence |

## Coverage

`quire matrix --scope . --format json` reports FR-358-AC-12 and FR-358-AC-13 both tagged to the intended new tests. Across the repository the current matrix has 80 tagged, 5 untagged, and 9 method-without-symbol criteria; the latter 14 are pre-existing and not attributed to this diff. No plan was supplied. Plan completion: not assessed.

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 5daaf4b192dbdcede80e9ba601731334d0facaf8: a live-handle retry now discriminates cancellation-first from spending the selected injection; the focused AC-13 test passes. |
