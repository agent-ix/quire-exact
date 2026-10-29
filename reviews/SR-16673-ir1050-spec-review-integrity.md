---
id: SR-16673
title: "IR-1050 PR93 spec-review/integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact; IR-1050; FR-383-AC-4, TC-933, TC-931, FR-359; PR93"
review_set: subset
---

## Summary

Sole reviewer on quire-exact PR93. Initial head e33dfca908e494308077b57ee5cde5d0800c2f67; final head cad41aa525f900f1aace044d948a1ea0f1206166. Initial frozen head had two stale current-matrix status paragraphs. Same-PR fix cad41aa corrected both to 11 and removed AC4 from the untagged list; all remaining residuals are unchanged.

## Verdict

**PASS after same-PR disposition**.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | At initial head e33dfca, TC-931 and FR-359 still called FR-383-AC-4 untagged and counted 12 strict gaps although the new Trace binder made the computed matrix 11; current-head status was false. | spec/test-cases/TC-931-single-pass-meter-accounting.md:47; spec/functional/FR-359-cumulative-meter-boundary.md:73 |

## Coverage

Initial frozen head had two stale current-matrix status paragraphs. Same-PR fix cad41aa corrected both to 11 and removed AC4 from the untagged list; all remaining residuals are unchanged.

## Review data

```yaml
clean: false
scope:
- id: FR-383-AC-4
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: examined
  excerpt: The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles
    in the crate's `no_std` + `alloc` build.
- id: FR-383-AC-3
  path: spec/functional/FR-383-meter-cancellation-handle-access.md
  role: context_only
  excerpt: The accessor's implementation borrows the existing optional handle without
    cloning it or constructing a second `Meter` or `Counters`; calling it changes
    no meter accounting or injection state.
- id: TC-933
  path: spec/test-cases/TC-933-meter-cancellation-handle-access.md
  role: examined
  excerpt: AC-4 has a Trace-bound public signature probe and is verified in the `no_std`
    + `alloc` build by `make build-no-std`.
- id: FR-359
  path: spec/functional/FR-359-cumulative-meter-boundary.md
  role: context_only
  excerpt: '`quire matrix --strict` exits 1 because the current matrix has 12 untagged
    criteria: FR-359-AC-9, FR-359-AC-10, FR-371-AC-6, FR-383-AC-4, FR-384-AC-5 and
    FR-385-AC-1 through AC-7.'
- id: TC-931
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  role: examined
  excerpt: '`quire matrix --strict` exits 1 because the current matrix has 12 untagged
    criteria: FR-359-AC-9, FR-359-AC-10, FR-371-AC-6, FR-383-AC-4, FR-384-AC-5 and
    FR-385-AC-1 through AC-7.'
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: spec-review/integrity
  check: soundness
  artifact_id: TC-931
  related_ids:
  - FR-359
  - FR-383-AC-4
  path: spec/test-cases/TC-931-single-pass-meter-accounting.md
  lines: '47'
  excerpt: '`quire matrix --strict` exits 1 because the current matrix has 12 untagged
    criteria: FR-359-AC-9, FR-359-AC-10, FR-371-AC-6, FR-383-AC-4, FR-384-AC-5 and
    FR-385-AC-1 through AC-7.'
  finding: At initial head e33dfca, TC-931 and FR-359 still called FR-383-AC-4 untagged
    and counted 12 strict gaps although the new Trace binder made the computed matrix
    11; current-head status was false.
```

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Both current-matrix status paragraphs now say 11 untagged and omit FR-383-AC-4 at cad41aa525f900f1aace044d948a1ea0f1206166. |
