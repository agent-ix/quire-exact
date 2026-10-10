---
id: SR-4741
title: "Gap analysis — IR-707 checked-invariant kernel slice"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@1fcd985118878cf71bfbe0ce8cfbcfe2a54c2e3e; spec/functional/FR-369-typed-checked-invariant-causes.md, spec/test-cases/TC-917-typed-checked-invariant-causes.md, src/collection.rs, src/division.rs, src/equality.rs, src/lib.rs, src/outcome.rs, src/value.rs, tests/checked_invariant.rs"
review_set: subset
relationships:
  - { target: "ix://agent-ix/quire-exact/FR-369", type: references }
---

## Summary

The planless repository audit found all IR-707 kernel criteria tagged and backed by real production paths. Four repository criteria remain untagged: one existing FR-362 item and three FR-369 criteria whose producers live in downstream RT/CG repositories.

## Verdict

**FAIL as a whole-repository matrix claim; PASS for the IR-707 kernel slice.** The untagged criteria are not defects in this seven-file kernel PR, but prevent a repository-wide gap-analysis PASS.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-362-AC-10 remains untagged from the pre-existing baseline; unrelated to IR-707 | FR-362-AC-10 |
| FND-002 | high | RT checked-package/function producers lack a tagged test in this repository | FR-369-AC-3 |
| FND-003 | high | RT collection/composite/equality/quantity producers lack a tagged test in this repository | FR-369-AC-4 |
| FND-004 | high | CG generated-oracle producers lack a tagged test in this repository | FR-369-AC-7 |

## Coverage

- Reconciliation: `quoin matrix --repo . --json`, quoin 0.28.3 and quire 0.36.2/engine 0.50.2; `quire matrix --scope . --strict --format json` independently reported four untagged criteria and exited 1.
- Plan completion: not assessed
- Criteria: 77 tagged, 4 untagged, 9 inspection method-without-symbol. FR-369-AC-1/2/5 are tagged by this kernel slice; FR-096-AC-8 and FR-089-AC-6 retain tags.
- Evidence: no run evidence was bound in the computed matrix; static binders were inspected directly. FR-369-AC-3/4/7 concern RT/CG code not changed or present here. FR-362-AC-10 predates this PR.
- Reverse gap: all changed production behavior is owned by FR-369-AC-1/2/5, FR-089-AC-6, or FR-096-AC-8. No added stub, ignored test, or unsupported trace tag found.
- Semantic review: performed for the changed kernel criteria and their tagged tests as requested in the reviewer brief; test-oracle conclusions are in SR-4740.

## Follow-up Ownership

The structured Linear issue records identify IR-667 (Backlog) for FR-362-AC-10's normalize-charge observability, IR-708 (Backlog) for RT producers FR-369-AC-3 and FR-369-AC-4, and IR-709 (Backlog) for CG producer FR-369-AC-7. These mappings agree with FR-369's repository ownership table and the changed kernel source inventory. They are outstanding work, not evidence that a downstream implementation has passed its own tests. The four untagged statuses are measured from the current Quoin matrix; ticket prose was not used as a gate verdict.

## Dispositions

Round 1 reviewed the unchanged PR #13 head `1fcd985118878cf71bfbe0ce8cfbcfe2a54c2e3e`. These repository-wide findings remain open in their separately owned tickets; deferral does not claim the criteria are tagged or implemented.

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | deferred | IR-667 owns the pre-existing FR-362-AC-10 normalize-charge observation seam and runnable binder; this is outside IR-707's cause-carrier change. |
| FND-002 | deferred | IR-708 owns RT checked-package/function producer migration and tests after the shared kernel carrier lands. |
| FND-003 | deferred | IR-708 owns RT collection/composite/equality/quantity producer migration and tests after the shared kernel carrier lands. |
| FND-004 | deferred | IR-709 owns CG generated-oracle producer migration and tests after the shared kernel carrier lands. |
