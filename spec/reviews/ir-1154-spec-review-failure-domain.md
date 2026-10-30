---
id: SR-16696
title: "IR-1154 structural comparison failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-exact; FR-389-AC-13 and TC-938 step 10"
review_set: subset
---

## Summary

The test procedure can catch the current comparison-frame heap allocation, depth-proportional recursion, wrong nested ordinal or proper-prefix precedence, mutation, and a false zero-allocation reading caused by concurrent tests. The flat preorder representation with span metadata permits a constant-stack event walk; the spec does not mandate a particular implementation or a depth cap.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No untested failure mode found in the changed comparison obligation. | FR-389-AC-13; TC-938 step 10 |
