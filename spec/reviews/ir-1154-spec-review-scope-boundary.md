---
id: SR-16697
title: "IR-1154 structural comparison scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-exact; FR-389-AC-13 and TC-938 step 10"
review_set: subset
---

## Summary

The exact kernel owns `NodeKey::cmp` and `partial_cmp` purity and structural order. The changed contract leaves caller-owned identity admission and QSV read-path use of comparison with their respective owners. It introduces no digest, fallback, copied dependency artifact or new public allocation API.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No scope leak found in the changed exact comparison contract. | FR-389 Behavior 10 and AC-13; TC-938 step 10 |
