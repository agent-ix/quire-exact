---
id: SR-006
title: "Spec review of quire-exact PR #2: FR-357, TC-905 and the FR-096-AC-8 edit"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@45c000f1e28f5b2443e636721b98ba2b8a5937f9; spec/functional/FR-357-single-member-integer-division.md, spec/functional/FR-096-kernel-refusal-code-and-cause.md, spec/test-cases/TC-905-single-member-integer-division.md; compared against agent-ix/quire-specification PR #188 (FR-147, FR-271, FR-272, TC-192) head f0002b92"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-357
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-096
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-905
    type: reviews
---
# Spec review of quire-exact PR #2

## Summary

Ticket: IR-601. PR: quire-exact#2.

FR-357 is atomic. It has a description, a use case, behaviour and five testable ACs, each
verified by TC-905. It defers the language rule to QSpec FR-147 without copying it. TC-905
names the defects it catches and lists every AC in scope. The FR-096-AC-8 edit drops
`both-outside-domain`, which cannot occur under member-only checking, and keeps the
twelve-cause count in FR-096 Behavior 1 correct.

## Verdict

Request changes: one high, one medium and one low finding. The spellings do not match
QSpec #188, AC-5 leaves out the member-only MIN/-1 case, and the operator notation is mixed.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-096-AC-8 names the refusal `DivisionMemberOutOfDomain` with code `division_member_out_of_domain`, and FR-357 Behavior 1 names `DivisionMemberOutOfDomain`. QSpec #188 names the code `division_out_of_domain`. Fix: align to #188 (`DivisionOutOfDomain`, `division_out_of_domain`) together with SR-004 FND-001. | spec/functional/FR-096-kernel-refusal-code-and-cause.md:25; spec/functional/FR-357-single-member-integer-division.md:19 |
| FND-002 | medium | FR-357-AC-5 says MIN/-1 "refuses in one [domain] that does not" hold 2^63, but gives no cause. It also leaves out the case that shows the member-only rule: the remainder of MIN/-1 completes with 0 in a signed-64 domain (QSpec #188 DIV-05). TC-905 step 5 inherits the gap. Fix: in a signed-64 domain the quotient refuses with `quotient-outside-domain` and the remainder returns 0, and TC-905 step 5 should say the same. | spec/functional/FR-357-single-member-integer-division.md:31; spec/test-cases/TC-905-single-member-integer-division.md:23 |
| FND-003 | low | FR-357's description names the operators `div` and `rem`, as QSpec does. AC-2, the use case and TC-905 step 2 write `10 / y` and `x % -1`. Fix: write `10 div y` and `x rem -1`, to match the description and QSpec FR-147-AC-10. | spec/functional/FR-357-single-member-integer-division.md:15; spec/functional/FR-357-single-member-integer-division.md:28; spec/test-cases/TC-905-single-member-integer-division.md:20 |
