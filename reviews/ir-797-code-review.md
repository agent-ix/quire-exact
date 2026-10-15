---
id: SR-11065
title: "IR-797 PR 47 validated UTF-8 scalar count code review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; PR #47 head 7b19e9c3450563f439867d1bf848fb1924381cfc vs base 634c7d4; src/text.rs; ticket IR-797"
review_set: subset
---

## Summary

Reviewed the single-file Rust diff in PR #47 against IR-797 acceptance items 1–3 and the existing QSpec-FR-141 text contract. This is a diff-scoped code and Rust review; the candidate does not claim a completed CG Kani proof.

## Scope examined

- IR-797 acceptance 1 (examined): Focused tests compare the new count against Rust `chars().count()` for valid ASCII, 2/3/4-byte scalars, combining sequences and normalized forms, including bounds and meter charges. Invalid UTF-8 remains refused before any count.
- IR-797 acceptance 2 (examined): Independent Rust/code/gap review and required quire-exact gates pass on a source-changing PR. A bounded CG Kani comparison confirms the same supported proof class and unchanged cap, with an actual terminal verdict or an honest measured failure.
- IR-797 acceptance 3 (examined): No cap inflation, weakened assertion, skipped admission, copied schema/artifact, unsafe unchecked UTF-8 path, or fabricated proof result.
- src/text.rs:52 (examined): `scalar_count` accepts `&str`, counts non-continuation bytes, and is used only where the previous code used `chars().count()`.
- src/text.rs:525 (examined): The equivalence and admission tests execute real text admission, normalized profiles, meter charges, a too-short bound, a scalar-limit refusal, and invalid UTF-8 rejection.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

**PASS for the PR diff.** `&str` guarantees validated UTF-8; a scalar has one leading byte, so the helper preserves the two replaced counts. The original validation and meter admission calls remain in the same order. The two focused tests passed at the exact head, including the `test-support` charge-record assertions. Prior full quire-exact CI was reported passed to this reviewer; this pass did not rerun full gates. No CG Kani terminal proof or performance result was reviewed, so acceptance item 2 remains externally pending.
