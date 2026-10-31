---
id: SR-16674
title: "QSL-1319 code-review — cached structural NodeKey hash"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; QSL-1319; quire-exact#116 head 1dabe140 vs eea6b122; src/node.rs; spec/functional/FR-389-carry-admitted-structural-identities.md; TC-938"
review_set: subset
---

## Summary

Independent scoped code-review review of quire-exact PR #116. Ticket: QSL-1319. Frozen head: 1dabe140f12ca069fdac2daddb8b0d862f4450eb. Plan completion: not assessed.

## Verdict

**PASS** — cached hash is computed from immutable structural components, copied unchanged by fallible clone, and writes one u64 without traversing descendants. No code defect found in the scoped diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Examined FR-389-AC-1/11/12/14 and TC-938 against the changed NodeKey constructor, Hash implementation, fallible clone and deep hash test. The existing Eq/Ord/Drop paths and test controls were read for context. No CI, dependency, manifest, or public API file changed. `quire matrix` and `quire coverage --strict` were run at the frozen head; code-review relied on author-reported focused fmt, Clippy, and test results and did not repeat Cargo gates. No plan was supplied. Plan completion: not assessed.
