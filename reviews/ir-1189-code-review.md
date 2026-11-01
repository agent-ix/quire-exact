---
id: SR-2026102765
title: "IR-1189 exact code-review"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@5f4518391278c8264729b6ce5f41beec33da68b7; Cargo.toml; Cargo.lock; src/node.rs; IR-1189"
review_set: subset
---

## Summary

Independent review of PR #120 at frozen candidate 5f4518391278c8264729b6ce5f41beec33da68b7, base 82e8f9b4d9583ba4eee505550ff80a211333696f. No applicable AssuranceProfile.

## Verdict

PASS for the scoped PR delta. No actionable code defect found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

The ArrayVec starts with one live frame; push follows is_full, last_mut replaces depth indexing, and pop preserves the phase machine. Both cursors retain the existing 1024-frame bound and restart through compare_indexed on overflow. No new unsafe, heap allocation, structural-order change, public API change, duplicated implementation, or gate weakening found. The no_std dependency disables default features. The new real-path test asserts both orders and zero allocations across depth 0/8/64/512/1024/2048; elapsed values are observations only. Existing structural-oracle and mixed-path fallback tests were inspected.

## Examined scope

scope:
- id: FR-389-AC-13
  path: spec/functional/FR-389-carry-admitted-structural-identities.md
  role: examined
  excerpt: For independently constructed equal and differing `NodeKey`s, `cmp` and
    `partial_cmp` each perform zero heap allocations on the calling thread and return
    the same QSpec FR-322 order in both directions. The controls include package,
    text, text-before-node, nested package/path/ordinal, a proper prefix at the top
    level and inside a nested node, and outer ordinal after an otherwise equal path.
    A 10,000-level pair compares on a small call stack without allocating or overflowing
    it, including equality and a difference near the deepest level. Every `cmp ==
    Equal` pair remains equal and hashes equally

## Evidence limits

No Cargo, full suite or Kani run, per user speed rule. Supplied evidence: Rust 1.98.1 fmt and all-target Clippy -D warnings passed; new focused test passed debug/release and structural-oracle test passed. Optimized 100k early mismatches measured 94.674ms baseline versus 28.586ms candidate; debug did not improve. These supplied timing observations are not assertions or independently rerun. Static diff check passed.
