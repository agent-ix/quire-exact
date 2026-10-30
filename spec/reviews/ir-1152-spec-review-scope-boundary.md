---
id: SR-16703
title: "IR-1152 identity copy scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-exact PR #107; FR-391 and TC-940"
review_set: subset
---

## Summary

Exact owns the borrowed structural copy, its real reservation callback, typed native storage failure and count/layout checks. QSV FR-108 and FR-115 own environment admission, storage-unit projection, registry publication and rollback. FR-390/TC-939 own `Integer` copy and decimal output; FR-391 does not claim those operations. The boundary is appropriately allocated.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | (placeholder) No scope-boundary finding. | FR-391 Description, Behavior 2-4, Dependencies; FR-390 Dependencies |
