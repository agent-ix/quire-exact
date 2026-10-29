---
id: SR-1132
title: "Base review of IR-1132 closed LimitKind reader"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact@ca2cf4ca13e7a7caf77586628ead69b78b37b44d; spec/functional/FR-388-read-closed-limit-kind-spellings.md; spec/test-cases/TC-937-closed-limit-kind-reader.md"
review_set: base
---

## Summary

FR-388 specifies the public typed reader, exact spellings, field order and refusal behavior. TC-937 verifies round trips and QSpec vector equality, but its finite negative controls do not prove the reader rejects every other spelling. IR-1131 is a correctly recorded blocker for the authoritative QSpec vector; no active AssuranceProfile applies in this repository.

## Scope

Reviewed the two added spec documents in PR #95 at the stated head, existing FR-368/TC-916 and `src/accounting.rs` for vocabulary ownership and API fit, QSpec `value-accounting.md` at origin/main for `ScalarLimitsV1`, and the structured IR-1131 blocker. No hash, digest, frozen revision, local vocabulary table or copied upstream vector was added by the diff.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-937's vector equality and finite refusal samples can pass if `from_code` accepts an extra untested alias; add a branch inspection or equivalent exhaustive subset oracle for accepted spellings, alongside the QSpec vector's completeness oracle | FR-388 behavior 2 and 5; FR-388-AC-2/3; TC-937 steps 2/3 |

## Dependency disposition

IR-1131 remains a blocker for full-vocabulary evidence, not a spec defect. The spec correctly reads the pending QSpec vector from its authoritative repository and does not copy its members.

## Disposition

| FND | Outcome | Evidence |
| --- | --- | --- |
| FND-001 | fixed | Follow-up commit `d7b06096f0cdf309045239ff882bc2c71fbc6408` adds FR-388-AC-4 and TC-937 step 4, requiring inspection of every successful decoder return path for exact equality to a member in `LimitKind::ALL`, plus rejection of catch-all, normalization, aliases and unchecked conversion. |
