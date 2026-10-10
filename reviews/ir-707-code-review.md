---
id: SR-4740
title: "Code and Rust review — IR-707 kernel checked-invariant causes"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@1fcd985118878cf71bfbe0ce8cfbcfe2a54c2e3e; src/collection.rs, src/division.rs, src/equality.rs, src/lib.rs, src/outcome.rs, src/value.rs, tests/checked_invariant.rs; FR-369-AC-1/2/5, FR-089-AC-6, FR-096-AC-8, FR-090"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-exact/FR-369", type: references }
---

## Summary

Reviewed the seven-file PR diff at the stated SHA against FR-369, TC-917, and the related kernel requirements. The public 23-cause carrier, every changed kernel producer, catalog mapping, equality precedence, and traced tests agree with the specified kernel slice.

## Verdict

**PASS** — no defect found in the frozen kernel diff. CI workflow files were unchanged. The three RT/CG criteria are outside this repository's PR and are recorded in the companion gap analysis.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Review Evidence

The enum has exactly twenty-three closed variants; three retain `IllTypedCause`. `Refusal::CheckedInvariant` requires a cause and both catalog accessors return `None`. All seven kernel constructor sites map to the specified causes. `plan_pairs` checks two populations before the kind-mismatch arm; the tests also check both operand directions for a mixed population pair. The public integration test verifies traits, all cause mappings, payload retention, refusal inequality, and the old constructor's compile failure. The changed Rust has no `unsafe`, `serde`, fallback/default cause, QSL type or dependency, unbounded new loop, or arithmetic conversion. No workflow changed in the PR diff.

Reviewed the new test oracles for source mutations: replacing any mapped kernel cause changes a direct assertion, dropping a public variant breaks the test's construction, and adding a variant breaks the exhaustive match. The source-extraction compiler check is supplemental to the real public enum match. `git diff --check` passed. The author owns the aggregate pre-PR/final gate; this independent review did not duplicate its cargo build or Kani run.
