---
id: SR-13013
title: "quire-exact PR 56 IR-878 cross-repo dependency review"
type: SpecReview
analysis: dependency
scope: "agent-ix/quire-exact; PR #56 head 159f170d8ab7e339fe636934e26f602667fc0153 vs origin/main b83cab9; spec/functional/FR-369, FR-372, FR-375, FR-376, FR-380; spec/test-cases/TC-923; ticket IR-878"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: reviews
---

## Summary

Resolution of every new ix:// link and owner id added by the PR, checked against both the companion PR heads and their repos' origin/main.

Ticket: IR-878

## Reviewed units

- FR-372 (examined): The integrated supplied Decimal control formerly FR-372-AC-5 is allocated to [QSV FR-122-AC-2](ix://agent-ix/quire-semantic-value/FR-122) for the supplied path and [QSL FR-391-AC-3](ix://agent-ix/quire-spec-language/FR-391) for evaluator effects. Both remain PLANNED for owner tests.
- FR-380 (examined): The supplied-caller controls formerly FR-380-AC-6 are allocated to [QSV FR-122-AC-3](ix://agent-ix/quire-semantic-value/FR-122) and [QSL FR-391-AC-2](ix://agent-ix/quire-spec-language/FR-391); the evaluator charge control remains with QSL. Owner tests are PLANNED.
- FR-369 (examined): The shared cause carrier and kernel producers are owned here. The producer controls formerly FR-369-AC-3/4, AC-7 and AC-9 are allocated without weakening their text to RT FR-278, CG FR-042 and QSV FR-122, respectively. Their tests remain PLANNED at those owners.
- FR-375 (examined): When a checked equality caller evaluates two admitted operands, the `quire-exact` kernel SHALL expose the exact equality relation and planned `equality.pair` count from one plan, while its `Meter` records only admitted charges. The runtime's actual checked evaluation and its end-to-end controls are allocated to [RT FR-277](ix://agent-ix/quire-contract-runtime/FR-277).

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The allocation links `ix://agent-ix/quire-spec-language/FR-391` (here and in FR-380 line 121) collide with an existing id. QSL origin/main already has FR-391 'name-a-population-in-a-state-clause' (spec/functional/FR-391-name-a-population-in-a-state-clause.md, PR #982, merged 2026-10-19T02:58Z). QSL #995, opened later at 04:52Z, adds a second FR-391, and GitHub reports it as CONFLICTING. On QSL main these links resolve to the wrong requirement. The moved FR-372-AC-5 evaluator clause and the moved FR-380-AC-6 default-B clause therefore point at no real owner. QSL #995 must renumber, and this PR (and QSV #47 FR-122) must cite the new id. | spec/functional/FR-372-fallible-bounded-decimal-membership.md:173 |
| FND-002 | low | Every new owner id (RT FR-277/FR-278, CG FR-042, QSV FR-122, QSL FR-391) exists only on an unmerged companion PR. None is on its repo's main. If this PR merges first, its ix:// links dangle until the companions land. Merge order must be the companions first, or all together. | spec/functional/FR-375-composable-equality-effect-seam.md:13,37 |

## Verdict

The RT FR-277/FR-278, CG FR-042 and QSV FR-122 ids are unused on their mains and defined on their PR heads. Their relationships point back to quire-exact FR-375/FR-369/FR-372/FR-380, which exist. The QSL FR-391 id collides and blocks the PR.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-003 | low | FR-376 Dependencies links `ix://agent-ix/quire-contract-runtime/FR-277-AC-4`, and QSV #47 FR-122-AC-3 links `ix://agent-ix/quire-exact/FR-380-AC-7`. These are criterion-level ix:// addresses, and no spec in quire-exact, RT, CG or QSL uses that form. Every other link in this allocation addresses the FR artifact. An AC id is not an artifact id, so link resolution will likely miss. Link the FR and name the AC in the link text. | spec/functional/FR-376-composite-equality-across-declarations.md:60 |

## Dispositions

Round 1, PR head c58ab92.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | QSL #995 head 6862c2f defines FR-394 (spec/functional/FR-394-supplied-admission-evaluator-boundary.md), and its AC-2/AC-3 are unchanged from the former FR-391. QSL origin/main 3634209 has FR-391..393 but no FR-394. #995 is now MERGEABLE. FR-380 and QSV #47 FR-109/FR-122 were repointed too. No FR-391 reference remains in quire-exact. |
| FND-002 | still-open | Merge order still matters. RT #132, CG #449, QSV #47 and QSL #995 are all OPEN (MERGEABLE), so the FR-277/278, FR-042, FR-122, FR-394 and TC-203 links do not yet exist on any main. Merge the companions first, or all together; this resolves without a code change. |
| FND-002 | accepted-no-change | Round 2 (452e01a): This is a sequencing constraint, not a spec defect. The PR #56 body now states that RT #132, CG #449, QSV #47 and QSL #995 must merge before this PR or together with it. All four are still OPEN: RT, CG and QSV are MERGEABLE, and QSL was UNKNOWN at review time. The merger must honour that order. |
| FND-003 | fixed | Round 2 (452e01a): No criterion-level ix:// address remains in quire-exact spec/, QSV #47 d6f631d, RT #132 e815aca or QSL #995 FR-394. |
