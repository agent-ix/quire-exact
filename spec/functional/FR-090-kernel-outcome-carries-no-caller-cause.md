---
id: FR-090
title: "The kernel outcome has a closed checked-failure cause boundary"
type: FR
relationships: []
---
# FR-090: The kernel outcome has a closed checked-failure cause boundary

## Description

The `quire-exact` kernel `Refusal` and `Undefined` enums SHALL name only causes the kernel itself raises, except for the closed `CheckedInvariantCause` transport shared with `quire-contract-runtime`'s extracted exact checking residue and CG's generated checked oracles under [FR-369](./FR-369-typed-checked-invariant-causes.md). A cause whose meaning depends on a QSL checker, model or evaluator remains that caller's and stays out of the kernel's closed sets. QSL's existing direct construction of the unit `CheckedInvariant` SHALL migrate under QSL-owned requirements before it pins the new kernel API. The caller's handling of those causes keeps the id `FR-090` in `agent-ix/quire-spec-language`.

## Use case

A caller refuses an evaluation for a wrong snapshot anchor, an absent lookup key, or a false precondition. It reports each through its own family result, never through a kernel variant.

## Behavior

1. **Closed kernel sets.** `Refusal` has no variant that carries a wrong-snapshot cause, and `Undefined` has no precondition-false variant and no absent-key variant. The shared checked-invariant carrier accepts only FR-369's exact/kernel, RT residue and generated-oracle causes, not a general caller code or message.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-090-AC-7 | `Refusal` has no variant whose payload is a wrong-snapshot cause. | Inspection (TC-388) |
| FR-090-AC-11 | `Undefined` has no `PreconditionFalse` variant. | Inspection (TC-407) |
| FR-090-AC-12 | `Undefined` has no `AbsentKey` variant. | Inspection (TC-408) |
| FR-090-AC-13 | `Refusal::CheckedInvariant` carries only FR-369's closed typed cause; it has no arbitrary caller message, QSL model/checker type, catalog code or generic caller-cause escape hatch. | Inspection |

## Status

The original exclusions are implemented. The typed carrier in AC-13 is planned for IR-707; QSL's direct constructors need their own migration before its lock pins IR-707.

## Dependencies

- [FR-369](./FR-369-typed-checked-invariant-causes.md) defines the narrow shared carrier. The caller's behaviour under the older ids lives in `agent-ix/quire-spec-language`.
