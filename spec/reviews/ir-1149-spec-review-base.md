---
id: SR-16690
title: "IR-1149 fallible Integer spec review base"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact; FR-390 and TC-939"
review_set: subset
---

## Summary

Independent PR #106 review at frozen head `1fcb9c3`, with finding disposition at `a9355f4118705b2a3bf56ae46feb2abd79b6d2a9`, covering FR-390 and TC-939 against the current `Integer` representation and QSV `compound_unit_id` consumer. Both findings are resolved. No applicable active AssuranceProfile was found in the exact repository.

## Verdict

**PASS.** The revised contract uses `OutputSizeOverflow` for unrepresentable output length at every bound and tests the finite and `usize::MAX` cases. It also removes the nonexistent QSV FR-109 relationship.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Resolved at `a9355f4`: distinct `OutputSizeOverflow` applies at every bound when the output count cannot fit `usize`; representable `ByteLimit` always has `required > limit`, and TC-939 tests finite and maximum bounds. | FR-390 Outputs, Behavior 2, AC-3/5, TC-939 steps 4-5 |
| FND-002 | medium | Resolved at `a9355f4`: the nonexistent QSV FR-109 relationship was removed. | FR-390 frontmatter |

## Coverage

TC-939 supplies executable success, exact-bound, one-below-bound, injected reservation, and synthetic arithmetic-overflow controls for AC-1 through AC-5. The new ACs are correctly untagged in the code matrix until implementation. The review found no digest or fixed-output pin gate and no narrowing of the existing FR-389 criteria.
