---
id: SR-12491
title: "Gap analysis of IR-873 temporal visit evidence"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; FR-362-AC-21/22; FR-368-AC-1/2; TC-910; TC-916; src/accounting.rs; tests/ir673_core.rs; tests/ir873_temporal_visit.rs"
review_set: subset
---
## Summary

The computed matrix binds all three changed criteria to executable tests and coverage improves by two rows. The status paragraphs in FR-362 and TC-910 still describe the now-tested temporal visit criteria as planned.

## Verdict

**CONDITIONAL** — Update FR-362 and TC-910 status prose in this PR to match the traced tests. Plan completion: not assessed.

## Scope and evidence

| Unit | Role | Evidence |
| --- | --- | --- |
| FR-362-AC-21 and AC-22 | examined | `quire matrix` reports tagged by the two new tests; focused tests pass in default and `test-support` configurations |
| FR-368-AC-1 and AC-2 | examined | AC-1 bound to the extended exhaustive test; AC-2 remains bound with unchanged `LimitKind` |
| TC-910 and TC-916 | examined | Procedures compared with charged counter and enum tests; TC-910 Status remains stale |
| `src/accounting.rs` | examined | No new limit kind or temporal traversal rule; one additive charge point |
| `tests/ir673_core.rs` and `tests/ir873_temporal_visit.rs` | examined | Public behavior, exact boundaries and denied point observed |

Base coverage: 165/194 backed, 29 unbacked, 0 contradicted. PR head: 167/194 backed, 27 unbacked, 0 contradicted. The pre-existing `TC-910` unmatched-row warning remains. The finding concerns status text rather than a missing executable test.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-362 and TC-910 still say AC-21/22 are planned until traced tests exist, although this PR supplies both tests; after merge that status text would be false. | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:80; spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md:46 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At `6829f97bdddba16cb4a910e5315494572e4d5595`, both status paragraphs identify `tests/ir873_temporal_visit.rs` as the executable `#[trace]` binding for AC-21/22; the two test functions carry `FR-362-AC-21` and `FR-362-AC-22` respectively. |
