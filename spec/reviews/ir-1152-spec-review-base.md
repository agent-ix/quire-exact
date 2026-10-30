---
id: SR-16700
title: "IR-1152 policy-aware identity copy spec review base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact PR #107; FR-391 and TC-940"
review_set: subset
---

## Summary

Independent review of PR #107 at frozen head `c05049b`. FR-391 and TC-940 cover structural equality, order, deep copy, measured reservation amounts, source preservation and typed QSV projection. No applicable active AssuranceProfile was found in this repository. The new criteria have TC-940 procedures; production trace tags await implementation. No digest, fixed output pin, vendored fixture or copied source is introduced. The policy-denial oracle below must be reconciled before implementation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The public policy callback is required to return its original caller-owned typed refusal, but AC-2 and TC-940 expected result 2 require every injected denial to return exact's `Allocation { requested }`. A caller can return an unrelated refusal, so both outcomes cannot hold for the same injection. Separate the policy-callback test from a crate-internal native-allocation injection and keep their error variants distinct. | FR-391 Outputs, Behavior 2, AC-2 and AC-5; TC-940 steps 2 and 4, Expected Results 2 and 4 |

## Coverage

TC-940 exercises all five FR-391 criteria. Synthetic count and layout controls avoid impossible fixture allocation. QSV FR-108 and FR-115 own carrier projection and publication; FR-390/TC-939 separately own `Integer` copy and decimal output.
