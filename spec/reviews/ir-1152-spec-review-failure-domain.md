---
id: SR-16702
title: "IR-1152 identity copy failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-exact PR #107; FR-391 and TC-940"
review_set: subset
---

## Summary

The callback aborts the copy on denial; native allocator denial and arithmetic or layout incapacity have separate typed outcomes. The source is borrowed and remains unchanged. Empty strings do not consume policy callbacks. Deep identities use flat storage and a bounded call stack. No further failure-domain finding beyond the callback-refusal ambiguity recorded in SR-16700.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | (placeholder) No separate failure-domain finding; callback-refusal ambiguity is in SR-16700 FND-001. | FR-391 Behavior 2-4; TC-940 steps 2-5 |
