---
id: SR-16691
title: "IR-1149 fallible Integer spec integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact; FR-390 and TC-939"
review_set: subset
---

## Summary

The copy and decimal contracts are separately observable and TC-939 reaches each new criterion. The revised head `a9355f4` distinguishes an unrepresentable output count from a representable exceeded byte limit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved at `a9355f4`: `OutputSizeOverflow` has a separate typed result at `max_bytes = usize::MAX`, and TC-939 names the exact maximum-bound oracle. | FR-390-AC-3, TC-939 step 4 |
