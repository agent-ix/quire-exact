---
id: SR-11066
title: "IR-797 PR 47 validated UTF-8 scalar count gap analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; PR #47 head 7b19e9c3450563f439867d1bf848fb1924381cfc vs base 634c7d4; src/text.rs; ticket IR-797"
review_set: subset
---

## Summary

Checked the changed text counter and tests against IR-797 acceptance items 1–3, QSpec-FR-141 ownership recorded in the text module, and the computed repository Test Matrix. The analysis is limited to the PR diff.

## Scope examined

- IR-797 acceptance 1 (examined): Focused tests compare the new count against Rust `chars().count()` for valid ASCII, 2/3/4-byte scalars, combining sequences and normalized forms, including bounds and meter charges. Invalid UTF-8 remains refused before any count.
- IR-797 acceptance 2 (examined): Independent Rust/code/gap review and required quire-exact gates pass on a source-changing PR. A bounded CG Kani comparison confirms the same supported proof class and unchanged cap, with an actual terminal verdict or an honest measured failure.
- IR-797 acceptance 3 (examined): No cap inflation, weakened assertion, skipped admission, copied schema/artifact, unsafe unchecked UTF-8 path, or fabricated proof result.
- src/text.rs:52 (examined): The one new helper replaces two scalar-count expressions without changing the text payload validation boundary or charge sequence.
- src/text.rs:525 (examined): The two new tests exercise scalar count equivalence and public admission behavior, including both positive and adverse outcomes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS for the PR diff.** The implementation has a stated text-contract owner, its new tests execute production code, and no stub, copied artifact, skipped admission, or inflated test claim appears in the changed file. The Kani comparison and terminal verdict are outside this review and remain unproved.

## Coverage

Plan completion: not assessed

`quoin matrix --json` computed 149 tagged, 15 `method-without-symbol`, and 19 untagged criteria at this head; those non-tagged rows predate the diff and none is a new IR-797 local criterion. This change adds no repository spec criterion or trace tag. The IR-797 acceptance checks are ticket data rather than quire criteria, so the matrix cannot claim their discharge. Both focused Rust tests passed; their oracle checks are examined in SR-11065. Optional repository-wide semantic review was not run, as the delegated scope is this PR diff.
