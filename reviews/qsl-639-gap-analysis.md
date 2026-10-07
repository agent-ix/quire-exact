---
id: SR-2439
title: "Gap analysis of quire-exact PR #10: QSL-639 linear memory for nested types"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@9ea9b679e10e63ec7fabc9d042afd3fbaa9bff02; PR #10 diff vs origin/main: src/value.rs, src/value/value_type.rs; criteria FR-262-AC-2 (this repo), FR-261-AC-3 (cited, owned by agent-ix/quire-spec-language)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-262
    type: reviews
---
# Gap analysis of quire-exact PR #10: QSL-639 linear memory for nested types

## Summary

Ticket: QSL-639. PR: quire-exact#10, head `9ea9b679e10e63ec7fabc9d042afd3fbaa9bff02`. Reviewer model `claude-opus-5-5`, run `9367b1b3-0e93-4a8a-8a58-801a174e21eb`. Planless gap analysis: Plan completion: not assessed.

Method: ran `quire matrix --strict` (quire 0.36.1 / engine 0.50.1) on this head and read the binders, grepped every `#[trace(...)]` id the diff adds against `spec/`, and read FR-262 here and FR-261 in a local `agent-ix/quire-spec-language` checkout. The diff changes production code only in `src/value.rs` (the two variant payload types and their constructors) and `src/value/value_type.rs` (Clone and Drop). Both are owned in substance by FR-262, which covers deep value types that clone, compare, hash, format and drop without native recursion. The linear-memory property itself has no owning criterion in this repo. The matrix's one untagged criterion and its `method-without-symbol` rows are pre-existing and are not touched by this PR. No stub, `todo!`, `#[ignore]` or tautological assertion was found. The oracle strength of the two new tests is judged in SR-2438.

## Verdict

**PASS with one medium traceability finding.** The behaviour is implemented and tested, and FR-262-AC-2 stays backed by the unchanged TC-735 tests, which still pass under Arc. The two new tests cite `FR-261-AC-3`, which does not exist in this repo's spec. `quire matrix --strict` drops that tag silently, so the new memory guarantee is untraced here (FND-001). Fixing it is a spec and tag edit in this PR, with no code change.

## Coverage

| Unit | Role | Path:line | Excerpt |
| --- | --- | --- | --- |
| FR-262-AC-2 | examined | spec/functional/FR-262-deep-values-and-types-in-the-kernel.md | On a thread with a 512 KiB stack, a `ValueType` of 100,000 nested `Option`s around `Boolean` clones, compares equal to its clone, hashes equal to its clone, formats for debug and drops. A `Value` of the same depth compares equal to its clone, and it and its clone drop. |
| FR-261-AC-3 | examined | agent-ix/quire-spec-language spec/functional/FR-261-read-other-untrusted-json-at-any-depth.md | On a thread with a 512 KiB stack, the observation document reader admits a snapshot whose population's field holds a recursive value 100,000 levels deep, with `observation.input_bytes` and `observation.values` raised to fit and the field's declared type admitting that value. ... |
| FR-262 | context_only | spec/functional/FR-262-deep-values-and-types-in-the-kernel.md | The caller's evaluation of a deep value keeps the id `FR-262` in the repository that owns it. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Both new tests carry `#[trace("FR-261-AC-3")]`. That id has no spec file in quire-exact, so `quire matrix --strict` binds the tag to nothing and drops it without a warning. FR-261-AC-3 also belongs to quire-spec-language and is about the observation document reader (snapshot admission, `observation.values` refusal, the `ObservationLimits` builder, TC-733), none of which these kernel tests exercise. The kernel guarantee this PR adds has no owning criterion in this repo: payload types are shared, a value N deep holds O(N) type nodes, and a 100,000-deep value with types as deep is admitted on 512 KiB. The repo's precedent is FR-262. It carries the caller's FR id into this spec with its own AC and TC-735, and its tests are tagged `TC-735, FR-262-AC-2`. Fix: add an AC to FR-262 here, or a quire-exact FR-261 file mirroring FR-262's pattern, that states the linear-node property and the deep-typed admission. Give it a TC and retag both tests to that AC and TC. | src/value.rs:2161, 2173 |
