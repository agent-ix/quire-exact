---
id: SR-8702
title: "QSL-500 quire-exact PR 17 spec review at a1f6450: union hunks of FR-376, FR-377, FR-378"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact@a1f6450d23225c37f7bb83e657ec0b3224252ced; spec/functional/FR-376 item 5 and AC-9; FR-377 item 8 and AC-8; FR-378 item 7 and AC-8; spec/test-cases/TC-924 step 9, TC-925 step 8, TC-926 step 7"
review_set: subset
---

## Summary

Ticket: QSL-500 (QSL-737 AC-5 follow-through). Base spec review of the six
spec hunks. Each new AC is testable and agrees with the code. Each new TC step
matches its AC. The new FR-378 item 7 agrees with the code: `from_admitted`
stores the payload without checking it, and `union` and `evaluate_union` use
the exact `admits`. The removed "adds no union behaviour before then" sentences
were correct to remove, because the kernel now carries a union value.

## Verdict

FINDINGS: one medium, two low.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-376 item 5 and FR-377 item 8 do not state the outcome for two unions of different declarations: the code refuses `ValueKindMismatch` and gives key `None`. FR-377 also omits key `None` for one member with different payload lengths. FR-376 item 4 covers only unlike variants, and FR-369's `ValueKindMismatch` row lists neither union case. Item 1 makes different-declaration composites ordinary, so a reader would expect the same for unions. The refusal is correct, because a union has no descendant declaration and QSL FR-319-AC-3 makes the comparison ill-typed, but the spec must say so. | spec/functional/FR-376-composite-equality-across-declarations.md:35-36; spec/functional/FR-377-composite-canonical-key-across-declarations.md:37-39 |
| FND-002 | low | FR-377 item 8 says a mismatched position "orders the enclosing unions by the ascending bytes of the declaration keys, before any slot". It does not say that the member identifier and the earlier payload positions decide first. Read literally, it would put an `Alpha` union holding C after a `Zulu` union holding P. | spec/functional/FR-377-composite-canonical-key-across-declarations.md:39 |
| FND-003 | low | FR-378 item 7 makes `UnionValue::from_admitted` a caller-admitted path for a conforming descendant. Item 8 still requires the "caller-admitted, checks nothing" rustdoc only for the five pre-union items. The `UnionValue::from_admitted` rustdoc does not mention a descendant, and the `union` rustdoc does not call itself the exact path. | spec/functional/FR-378-caller-admitted-values-in-collections-options-and-composites.md:44-45; src/value.rs |
