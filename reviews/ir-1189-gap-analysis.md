---
id: SR-2026102766
title: "IR-1189 exact gap-analysis"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@5f4518391278c8264729b6ce5f41beec33da68b7; Cargo.toml; Cargo.lock; src/node.rs; IR-1189"
review_set: subset
---

## Summary

Independent review of PR #120 at frozen candidate 5f4518391278c8264729b6ce5f41beec33da68b7, base 82e8f9b4d9583ba4eee505550ff80a211333696f. No applicable AssuranceProfile.

## Verdict

PASS for the scoped PR delta. Repository-wide assurance is FAIL because the computed matrix includes 13 pre-existing untagged criteria; this is not a new PR defect.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Scoped delta traceability PASS: the new test correctly binds FR-389-AC-13 and exercises production comparison. No new reverse gap or stub found. Repository-wide matrix is not clean: 255 criteria, 221 tagged, 21 method-without-symbol, 13 untagged. Existing untagged criteria outside this delta: FR-359-AC-9, FR-359-AC-10, FR-371-AC-6, FR-384-AC-5, FR-385-AC-1, FR-385-AC-2, FR-385-AC-3, FR-385-AC-4, FR-385-AC-5, FR-385-AC-6, FR-385-AC-7, FR-388-AC-3, FR-391-AC-4. These baseline gaps are outside the change and are not asserted fixed. Plan completion: not assessed. Optional semantic audit skipped (not requested).

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
