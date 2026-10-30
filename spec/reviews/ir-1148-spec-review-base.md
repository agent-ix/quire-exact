---
id: SR-1148
title: "Base review of borrowed node traversal and fallible identity copy"
type: SpecReview
analysis: base
scope: "quire-exact PR #102; FR-389-AC-9 through AC-12; TC-938"
review_set: base
---

## Summary

Independent review of the IR-1148 specification change against the existing structural identity contract, including the public `PathSegment::Node` type change and the controls for arbitrarily deep traversal and fallible construction/copy. No applicable active AssuranceProfile was found in this repository's spec tree. The three initial findings were corrected in the same PR; the revised criteria retain the shape, equality, ordering and refusal controls of AC-1 through AC-8.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The original AC-12 and TC-938 required failure injection at nested-node carrier storage even though the proposed direct `PathSegment::Node(NodeKey)` representation has no such allocation. | FR-389-AC-12; TC-938 step 8 |
| FND-002 | high | The original status text froze an implementation commit SHA in the specification. | FR-389 Status; TC-938 Status |
| FND-003 | high | The original `try_new` contract allowed `impl Into<String>` input, whose borrowed-string conversion could allocate infallibly inside a purportedly fallible method. | FR-389 Behavior 8; FR-389-AC-11; TC-938 step 8 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The criteria and test now require failure injection at every allocation actually performed, conditional on whether a nested-node carrier allocates. The direct carrier is explicitly allocation free. |
| FND-002 | fixed | Both status sections now describe implementation state without a frozen revision. |
| FND-003 | fixed | `try_new` now takes an already owned `String` and `Vec<PathSegment>`; the caller owns a fallible copy from `&str`. AC-11 and TC-938 cover the boundary and each reservation inside the method. |

## Verdict

Pass for the revised specification. TC-938 step 6 explicitly compiles the direct nested-node form, rejects the former boxed form, and reruns AC-1 through AC-8 after caller migration. No code build or test was run for this specification review.
