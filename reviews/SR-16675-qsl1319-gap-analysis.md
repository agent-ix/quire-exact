---
id: SR-16675
title: "QSL-1319 gap-analysis — cached structural NodeKey hash"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; QSL-1319; quire-exact#116 head 1dabe140 vs eea6b122; src/node.rs; spec/functional/FR-389-carry-admitted-structural-identities.md; TC-938"
review_set: subset
---

## Summary

Independent scoped gap-analysis review of quire-exact PR #116. Ticket: QSL-1319. Frozen head: 1dabe140f12ca069fdac2daddb8b0d862f4450eb. Plan completion: not assessed.

## Verdict

**CONDITIONAL** — the computed matrix omits FR-389-AC-14 despite two new test tags; restore table continuity without changing criterion text.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-14 is outside the parsed acceptance-criteria table, so new trace tags do not bind | spec/functional/FR-389-carry-admitted-structural-identities.md:70; src/node.rs:1651,1710 |

## Coverage

Examined FR-389-AC-1/11/12/14 and TC-938 against the changed NodeKey constructor, Hash implementation, fallible clone and deep hash test. The existing Eq/Ord/Drop paths and test controls were read for context. No CI, dependency, manifest, or public API file changed. `quire matrix` and `quire coverage --strict` were run at the frozen head; code-review relied on author-reported focused fmt, Clippy, and test results and did not repeat Cargo gates. No plan was supplied. Plan completion: not assessed.

## Dispositions

At fix head 1b0d8a8b454c57788682d04e9ea62ea7fcbbb82b, the single blank line before FR-389-AC-14 was removed without changing the criterion text or test code. `quire matrix --scope . --format json` now mints FR-389-AC-14 and binds both test functions at `src/node.rs:1651` and `:1710`. The finding is resolved; the original conditional verdict records the frozen review head.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-14 is now a row of the acceptance-criteria table, and the computed matrix binds both trace tags. |
