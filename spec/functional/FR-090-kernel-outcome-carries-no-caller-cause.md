---
id: FR-090
title: "The kernel outcome carries no cause owned by a caller"
type: FR
relationships: []
---
# FR-090: The kernel outcome carries no cause owned by a caller

## Description

The `quire-exact` kernel `Refusal` and `Undefined` enums SHALL name only causes the kernel itself raises. A cause whose meaning depends on a caller's checker, model or evaluator is the caller's, and stays out of the kernel's closed sets. The caller's handling of those causes keeps the id `FR-090` in `agent-ix/quire-spec-language`.

## Use case

A caller refuses an evaluation for a wrong snapshot anchor, an absent lookup key, or a false precondition. It reports each through its own family result, never through a kernel variant.

## Behavior

1. **Closed kernel sets.** `Refusal` has no variant that carries a wrong-snapshot cause, and `Undefined` has no precondition-false variant and no absent-key variant.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-090-AC-7 | `Refusal` has no variant whose payload is a wrong-snapshot cause. | Inspection (TC-388) |
| FR-090-AC-11 | `Undefined` has no `PreconditionFalse` variant. | Inspection (TC-407) |
| FR-090-AC-12 | `Undefined` has no `AbsentKey` variant. | Inspection (TC-408) |

## Status

Implemented.

## Dependencies

- None. The caller's behaviour under the same ids lives in `agent-ix/quire-spec-language`.
