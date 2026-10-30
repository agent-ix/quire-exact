---
id: SR-16693
title: "IR-1149 fallible Integer scope-boundary review"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-exact; FR-390 and TC-939"
review_set: subset
---

## Summary

Exact owns fallible integer copying and decimal text; QSV owns compound-unit collection, JSON encoding and total preimage admission. The revised head `a9355f4` removes the absent QSV requirement link and states that QSV must separately map output-size overflow.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved at `a9355f4`: the unresolved `ix://agent-ix/quire-semantic-value/FR-109` edge was removed. | FR-390 frontmatter |
