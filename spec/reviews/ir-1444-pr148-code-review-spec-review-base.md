---
id: SR-17940486709564847047683
title: "IR-1444 PR #148 FR-402 and TC-950 Status-line spec review"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #148 @ 7b451716; IR-1444; Status sections of spec/functional/FR-402-kani-tractable-native-decimal-carrier.md and spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-950
    type: reviews
---

## Summary

Ticket: IR-1444. This is a spec-review (base) of the only spec edits in PR #148: the FR-402 Status line and the TC-950 Status line.

I ran `quire validate --scope . <file>` on each changed file, and on the origin/main copy of each for comparison. All four runs exit 0 with the same single advisory, the pre-existing `semantic.inline-data-schema` module warning, so the edits add no warnings. The coder's "not a regular file" error came from passing the spec directory; `quire validate` takes a document path or glob.

## Verdict

**CHANGES REQUIRED (one medium, one low).** The FR-402 Status line is honest. It credits AC-1 to AC-9 and AC-12. It states that the two AC-10 proofs are written and not executed, that AC-10 and AC-11 are not credited, and that no measurement exists or is claimed. It states no fit and no budget. Its AC-7 credit depends on code-review FND-001. The TC-950 Status line overclaims step 6.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-950 Status says "Steps 1 to 4 and 6 ... run on the host". Step 6 is "Run each mutant of the next section": 31 buildable mutants, each run against its named input. The PR body, the commit and the ticket record no mutant run. The reviewer ran one (the normalized 32-strip removed), and it was caught. Either record the mutant runs, with which test failed for each, or reword the Status to say that the boundary set holds each mutant's failing input and that the mutants were not run (one reviewer control excepted). | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:81 |
| FND-002 | low | TC-950 Description still ends "This case is planned; no part of it has run.", which contradicts the new Status saying steps 1 to 4 ran. Drop or update the sentence in the same edit (half-applied wording). | spec/test-cases/TC-950-kani-tractable-native-decimal-carrier.md:14 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-402 Status | examined | Implemented (IR-1444): the carrier, constructor, normalized view, read-out, ingress and host tests (AC-1 to AC-9, AC-12). The two Kani proofs of AC-10 are written and not executed, under the repository's standing instruction; AC-10 and AC-11 are not credited. No measurement exists and none is claimed. |
| TC-950 Status | examined | Steps 1 to 4 and 6 (AC-1 to AC-9, AC-12) run on the host through tests/ir1444_native_decimal.rs, the unit binding of the references in src/native_decimal/tests.rs and the module doctests. The host half of step 5 (reference bound to the owner route) runs; the two Kani proofs are written and unexecuted. Step 7 (AC-11) has not run. No proof result or measurement exists. |
| TC-950 Description | context_only | This case is planned; no part of it has run. |
