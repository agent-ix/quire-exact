---
id: SR-16692
title: "IR-1149 fallible Integer failure-domain review"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-exact; FR-390 and TC-939"
review_set: subset
---

## Summary

The changed contract covers ordinary and injected allocation failures, unchanged source on refusal, exact byte limits, and checked arithmetic. The revised head `a9355f4` gives the maximum-bound overflow case a distinct typed result.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved at `a9355f4`: `OutputSizeOverflow` cannot be confused with `ByteLimit`, and FR-390 explicitly preserves `required > limit` for the latter. | FR-390 Outputs and AC-3, TC-939 step 4 |
