---
id: SR-16701
title: "IR-1152 policy-aware identity copy integrity review"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact PR #107; FR-391 and TC-940"
review_set: subset
---

## Summary

The new policy-aware `NodeKey` operation and the existing no-policy `NodeKey::try_clone` have overlapping failure language. FR-389 explicitly requires `Allocation { requested: usize::MAX }` on count overflow; FR-391 explicitly retains that convenience refusal contract while also requiring `Capacity` on count overflow. The operation to which each result applies must be stated unambiguously.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | `FR-391` retains `NodeKey::try_clone` and its FR-389 refusal contract, yet its general fallible-copy reservation rule mandates `Capacity { requested }` for unrepresentable count or byte product. FR-389-AC-12 mandates `Allocation { requested: usize::MAX }` for that same no-policy operation. Scope the new `Capacity` result to the policy-aware entry point, or explicitly reconcile the old contract and tests without claiming both outcomes. | FR-391 Behavior 2-3, AC-3 and AC-5; FR-389-AC-12; TC-940 step 4 |

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | At `a040148`, Behavior 3 limits the new `Capacity` outcome to policy-aware operations and `EffectiveId::try_clone`; it explicitly preserves the no-policy `NodeKey::try_clone` FR-389-AC-12 `InvalidNodeKey::Allocation { requested: usize::MAX }` overflow result. AC-3 and TC-940 step 3 test both paths separately. |

## Verdict

PASS at `a0401486071a65cef16d5a3914a919f472229652` for the integrity finding.
