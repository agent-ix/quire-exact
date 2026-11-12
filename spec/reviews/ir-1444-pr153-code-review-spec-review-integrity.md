---
id: SR-17940486709564847047717
title: "IR-1444/IR-1508 PR #153 spec-review/integrity"
type: SpecReview
analysis: integrity
date: 2026-11-12
scope: "agent-ix/quire-exact PR #153 at 1ead0352 (base e20118a); IR-1508, IR-1444; FR-402, FR-403, TC-950, TC-951 cross-consistency after the harness removal"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/TC-951
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-402
    type: reviews
---

## Summary

Ticket: IR-1508 (also IR-1444). Integrity check across the four changed spec files. `quire validate` passes on them.

Clean:

- The TC-951 Description range is now AC-1 to AC-15. TC-950's still covers AC-1 to AC-12, which is consistent: AC-13 is PLANNED and TC-950 step 5 says so.
- The Status sections of FR-402, FR-403, TC-950 and TC-951 agree with each other and with the matrix.
- The FR-403 Dependencies bullet that pointed at the CLAUDE.md Kani instruction is replaced by the Verus-track bullet.

## Verdict

Low only. One stale sentence remains (FND-001).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The TC-951 Description still ends "The host steps have run (see Status); the Kani proofs and the CG measurement have not." No Kani proof remains, and TC-951 Status now says so. Change it to "the CG measurement (step 11) has not run, and AC-15 waits on a Verus track". | spec/test-cases/TC-951-native-decimal-operations-match-owner-path.md:14 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| TC-951 Description | examined | Verify FR-403-AC-1 through FR-403-AC-15 ... the Kani proofs and the CG measurement have not. |
| TC-950 Description | examined | Verify FR-402-AC-1 through FR-402-AC-12 through the public kernel API, the host differential and the recorded measurement. |
| FR-402 Status | examined | Implemented (IR-1444) ... The full-width proof is PLANNED on a separate Verus track (AC-13) and is not claimed. |
| FR-403 Status | examined | Implemented (IR-1508) ... the full-width proof is PLANNED on a separate Verus track (AC-15) and is not claimed. |
| FR-403 Dependencies | examined | A full-width proof of the operations waits on a separate Verus proof track (Behavior 41). |
