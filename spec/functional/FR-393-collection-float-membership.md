---
id: FR-393
title: Collection membership uses bit identity at float leaves
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-145
    type: implements
---
# FR-393: Collection membership uses bit identity at float leaves

## Description

Canonical collection membership SHALL compare float leaves by their complete
width-specific bit identity. This relation serves collection formation,
contains and removeOne; it SHALL NOT expand ordinary equality eligibility.

## Behavior

The membership occurrence-pair walk SHALL retain the existing traversal of
options, composites, unions and collections, pair counts, declaration and
reference checks, and typed malformed-value refusals. At two float leaves of
the same width it SHALL compare every bit, including NaN sign/payload and
signed zero. Different widths SHALL retain the typed value-kind refusal.

The caller SHALL still charge `collection.member-walk` before planning and
`collection.member-test` after a successful plan, with unchanged exact
occurrence/work amounts. A false leaf SHALL NOT short-circuit the remaining
pair traversal. Denied charges SHALL preserve the admitted prefix and return
the original typed incomplete cause without a Boolean result retain.
Ordinary equality SHALL retain its existing planner and IEEE eligibility.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-393-AC-1 | At both widths identical NaN patterns compare true; changed payload, sign, signaling bit, signed zero and infinity sign compare false. | Test |
| FR-393-AC-2 | Float leaves nested in options and collections use the same membership relation, visiting all occurrence pairs after an unequal leaf. | Test |
| FR-393-AC-3 | Nested comparisons retain the exact walk/test charge vectors and no result-retain charge; denial at either point returns Incomplete with the admitted prefix unchanged. | Test |
| FR-393-AC-4 | Wrong width and wrong value kind preserve typed membership refusals; ordinary equality still refuses direct and nested float operands. | Test |

## Dependencies

- QSpec FR-145 owns collection membership semantics.
- The existing occurrence-pair planner owns traversal and typed invariant causes.
- The canonical meter owns charge admission and incomplete causes.
