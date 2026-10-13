---
id: SR-7885
title: "EARS review of quire-exact PR #38 (FR-376..378)"
type: SpecReview
analysis: ears-conformance
scope: "agent-ix/quire-exact@e9c9ed4a05c3c58505200785a77929f737402029; spec/functional/FR-369, FR-375, FR-376, FR-377, FR-378; spec/test-cases/TC-917, TC-923, TC-924, TC-925, TC-926"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-376
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-377
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-378
    type: reviews
---

## Summary

Ticket: QSL-737. PR: quire-exact#38. EARS check of the three new requirement statements and the SHALL clauses in their Behavior sections. FR-376 is a clean event-driven statement. FR-377 embeds a consequence clause, and FR-378 states its trigger as a standing condition with a response that cannot be measured.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The FR-378 statement uses an event keyword ("When a caller holds knowledge the kernel does not") for a standing condition, and its response ("SHALL let the caller supply that admission to collection construction") names no observable outcome. An EARS form such as "Where a caller supplies an element admission, the kernel SHALL use it in place of ValueType::admits for each element and SHALL store each admitted element unchanged" would make it testable on its own. | FR-378:20 |
| FND-002 | low | The FR-377 statement folds a second obligation into a purpose clause: "so that set, bag and ordered-set formation ... completes instead of refusing CanonicalKeyUnavailable". That is a separate behavior, and it is wrong for OrderedSet (see the integrity review). Keep the key-order SHALL, and move the formation outcome to Behavior 5 for Set and Bag. | FR-377:18 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-376 | examined | When the kernel equality plan pairs two composite values whose declaration keys differ, the `quire-exact` kernel SHALL decide that pair as not equal, with no checked-invariant refusal. |
| FR-377 | examined | When `compare_keys` orders two composite values whose declaration keys differ, the `quire-exact` kernel SHALL order them by the ascending byte order of their 32-byte declaration keys, compared before any slot, so that set, bag and ordered-set formation over such values completes instead of refusing `CanonicalKeyUnavailable`. |
| FR-378 | examined | When a caller holds knowledge the kernel does not (which declaration conforms to which, so that a descendant-declared composite belongs at an ancestor-typed element or payload slot), the `quire-exact` kernel SHALL let the caller supply that admission to collection construction. The kernel accepts an already-admitted `Option` payload through `OptionValue::from_admitted`. The kernel keeps its exact, environment-less constructors and `ValueType::admits` unchanged. QSL FR-359 (items 5 and 8 and cross-repo requirement 3) states the language-level need. |
| FR-378 | examined | **Doc comments.** The documentation of `OptionValue::from_admitted` SHALL say that it is the path for a caller-admitted payload, including a conforming descendant, and that it checks nothing. The documentation of `ValueType::admits` SHALL say that it is the exact predicate and is not the conformance check for a slot that can hold a descendant. |

## Verdict

FINDINGS (2 low). FR-376 conforms. FR-378 Behavior 1 and 8 SHALLs are singular and testable.
