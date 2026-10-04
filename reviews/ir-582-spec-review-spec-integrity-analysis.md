---
id: SR-003
title: "Spec review of quire-exact PR #1: the extracted FR and TC subset standing alone"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@0992d0d198b0ef0dbd79f86849626bc0503b9b03; spec/functional/FR-088, FR-089, FR-096, FR-097, FR-262; spec/test-cases/TC-297, TC-409, TC-411, TC-428, TC-441, TC-735; compared against quire-spec-language's FR-262-AC-2 and TC-735"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-096
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-428
    type: reviews
---
# Spec review of quire-exact PR #1

## Summary

Ticket: IR-582. `quire validate` passes all eleven files (module warnings
only). Each FR states only the kernel's behaviour, and each one says which
part stays with the caller in agent-ix/quire-spec-language. The TC
relationships resolve through `ix://agent-ix/quire-exact/...`. No ADR, no
Linear id, no quire-research reference and no local path appears. FR-089,
FR-097 and FR-262 stand alone. FR-262-AC-2 rightly narrows QSL's
evaluation-and-state-key AC to the kernel's iterative traits.

## Verdict

Changes requested on FND-001, a medium in-PR text fix. FND-002 and FND-003
are low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-096-AC-8 does not stand alone. It says each cause returns "the code and the cause the key table names for it", but this repo has no key table. Its named targets, and "the target domain or width the caller's record renders", describe the caller's record test. Fix: list the 15 kernel code and cause pairs (or point at the test's table), and name the targets the kernel test builds. Or drop the target clause. | spec/functional/FR-096-kernel-refusal-code-and-cause.md:20,26 |
| FND-002 | low | TC-428's file slug `a-refusal-record-carries-code-category-locus-and-fields` and TC-735's `evaluate-key-and-drop` are QSL's titles. Neither TC covers a record, a category, a locus, evaluation or keying any more. TC-441 and TC-735 each have the `Scope:` line twice. | spec/test-cases/TC-428-a-refusal-record-carries-code-category-locus-and-fields.md; spec/test-cases/TC-735-deep-values-and-value-types-evaluate-key-and-drop.md:13-15; spec/test-cases/TC-441-an-unbounded-collection-never-refuses-for-cardinality.md:13-15 |
| FND-003 | low | All eleven ids are reused from QSL with different, narrower statements: FR-088 here is not FR-088 in QSL. `ix://` qualification resolves this, but a bare mention of "FR-096-AC-8" in a cross-repo ticket or comment is ambiguous. The FR text calls this deliberate. Record it as a choice, or mint fresh ids. | spec/functional/*.md (Description and Dependencies) |

## Dispositions

| FND | outcome | sha/reason |
| --- | --- | --- |
| FND-001 | fixed | 02bd366 |
| FND-002 | fixed | 02bd366 |
| FND-003 | accepted-no-change | Coordinator ruling: the ids are kept so trace tags resolve, and cross-repo references qualify the id with its repository, as for quire-walk. |
