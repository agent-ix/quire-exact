---
id: SR-16658
title: "spec-review-base review of IR-1051"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact; spec/functional/FR-387-bounded-quantity-value-type.md; spec/test-cases/TC-936-bounded-quantity-value-type.md"
review_set: subset
---

## Summary

Reviewed quire-exact PR #91 at 8d9de350 against QSpec b30710e3, QSL cd4a7ea3, FR-088, FR-262, FR-386, and current kernel admission. QSpec conflicts with the package-bound type use case.

## Verdict

**FAIL** — QSpec FR-154 and the proposed QSL-facing type contract conflict.

## Scope

FR-387; FR-387-AC-1 through FR-387-AC-8; TC-936, all examined. Context: quire-exact FR-088, FR-262, FR-386; QSpec FR-154 at b30710e311a501bf73db3a890d6c1a7bb252aca3; QSL FR-230-AC-26 at cd4a7ea3b5c8aa797838a2925c7504a549c622c0. No applicable AssuranceProfile exists under this repository's spec tree.

## Analysis

QSpec origin/main b30710e3 FR-154 lines 225-261 makes unit bounds named invariants, leaves member types Quantity<u>, and records out-of-range successors rather than pruning them. QSL origin/main cd4a7ea3 FR-230-AC-26 says the opposite for alias Level. AC-7/AC-8 are internally falsifiable, and current ValueType::Quantity(U) already admits same-unit 101; the author's Map-only-admission analogy does not apply. Four design points: separate variant, exact Rational, same-unit-only constructor, and MalformedBoundedQuantityType are coherent as an internal kernel type, but the stated package use case cannot be approved until the QSpec/QSL owner conflict is resolved.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Package unit bounds cannot become a type admission rule under QSpec FR-154 | spec/functional/FR-387-bounded-quantity-value-type.md:20 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | QSpec main 783b3d998bfcfbaf6c4867897ade754b9dcad886 now makes a package unit bound a bounded quantity type and FR-154-AC-31 specifies static admission of exact constants. FR-387's revised Description names that owner contract, folds effective bounds, and leaves checker and runtime refusals with the caller. |
| FND-002 | still-open | The revised FR-387 still rejects min > max in its only constructor, including an effective min 5/max 3 folded from a valid alias-parent chain that QSpec FR-154-AC-29 admits as an empty domain. |
| FND-002 | fixed | At quire-exact head 4fd7bd6a90fc5d149bc381dbce74b4283e13c900, FR-387 Behavior 1 and AC-2/AC-9 make inverted effective bounds a valid empty type; AC-10 and TC-936 step 10 require refusal of every tested value, preservation of exact bound identity, and distinct identity for [5,3] versus [4,2]. Same-appliesTo contradictory constraints remain QSpec FR-154-AC-29 intake refusals. |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | high | Constructor rejection makes a QSpec-admitted empty effective bound domain unrepresentable | spec/functional/FR-387-bounded-quantity-value-type.md:53 |

QSpec main 783b3d998bfcfbaf6c4867897ade754b9dcad886 FR-154-AC-29 admits min 5 on an alias with max 3 on its parent and treats the conjunction as an unsatisfiable invariant; FR-154-AC-31 says every literal of that position refuses. FR-387 now requires the QSL caller to fold the alias chain into effective min and max before constructing this type, but AC-2 and AC-9 require `BoundedQuantityType::new(U, 5, 3)` to refuse. The checker therefore has no bounded type for a valid, empty domain. Specify how this effective domain is represented without turning an admitted cross-alias conjunction into a malformed declaration. Preserve direct-construction refusal if the owner contract calls for it, but a bypass or unbounded type would change QSpec semantics.
