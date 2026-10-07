---
id: SR-2437
title: "Integrity review of quire-exact PR #9 Status prose (IR-673 fix round 1)"
type: SpecReview
analysis: integrity
scope: "agent-ix/quire-exact@126add38f47e13ec6ba376cf93121c1558c6a2d6; diff 5967648b49cee8b2e60c87fbd5f8372b9b23e11f..126add38f47e13ec6ba376cf93121c1558c6a2d6 -- spec: Status sections of FR-097, FR-357, FR-358, FR-361, FR-362, FR-363, FR-364, FR-365, FR-366, FR-367, FR-368, TC-441, TC-905, TC-909, TC-911, TC-912, TC-913, TC-914, TC-915, TC-916"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-097
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-357
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-358
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-361
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-362
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-363
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-364
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-365
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-366
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-367
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-368
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-441
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-905
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-909
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-911
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-912
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-913
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-914
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-915
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-916
    type: reviews
---
# Integrity review of quire-exact PR #9 Status prose (IR-673 fix round 1)

## Summary

Ticket: IR-673. PR: quire-exact#9 (draft), fix head `126add38f47e13ec6ba376cf93121c1558c6a2d6`, prior `5967648b49cee8b2e60c87fbd5f8372b9b23e11f`. Reviewer model `claude-opus-5-5`, run `6a8d6ca2-e645-451b-9fb3-42bd241049a2`; same reviewer session as SR-2435/SR-2436. Scope is only the Status-prose edits that commit 126add3 makes to 20 FR/TC documents. `git diff 5967648..126add3 -- spec` changes exactly one Status line per file and no AC, Behavior, Description or frontmatter text.

Method: each new sentence was checked against `quoin matrix --json` on 126add3 (quoin 0.28.1, quire 0.36.1 / engine 0.50.1). All 20 touched documents pass `quire validate --scope .` (exit 0). Applicability: only integrity (consistency of status claims with the computed matrix, and preservation of ID-retirement records) applies. EARS, criterion-strength, dependency, object and failure-domain analyses do not apply, because no requirement statement, AC, relationship or domain object changed.

## Verdict

**CONDITIONAL: one low finding.** Every added sentence matches the matrix. Each "PR #9 adds executable Trace bindings for ..." names exactly the criteria the matrix reports `tagged` with a PR #9 binder. FR-362:58 states that AC-1..AC-9 keep earlier bindings, AC-11..AC-18 are bound by PR #9, and AC-10 "remains planned and untagged for IR-667 because the public meter does not expose the normalize charge bit amount", which matches the matrix (FR-362-AC-10 `untagged`, no binder) and the IR-673 residual. Pointing readers to the matrix instead of restating tag state removes the drift trap. The TC-441 and TC-905 test-name lists that were removed are recoverable from the matrix. Four Status sections outside this edit set still contradict the matrix (TC-906, TC-910, FR-359, TC-907); they are recorded once, as SR-2436 FND-001 `still-open`, not duplicated here.

## Coverage

| Unit | Role | Path:line | Excerpt |
| --- | --- | --- | --- |
| FR-097 Status | examined | spec/functional/FR-097-unbounded-collection-in-the-kernel.md:32 | PR #9 adds executable Trace bindings for AC-8. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-357 Status | examined | spec/functional/FR-357-single-member-integer-division.md:41 | PR #9 adds executable Trace bindings for AC-7. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-358 Status | examined | spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md:36 | PR #9 adds executable Trace bindings for AC-8 through AC-11. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-361 Status | examined | spec/functional/FR-361-admission-before-large-exact-work.md:41 | PR #9 adds executable Trace bindings for AC-7 through AC-9. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-362 Status | examined | spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md:58 | PR #9 adds executable Trace bindings for AC-11 through AC-18. AC-1 through AC-9 retain earlier bindings. AC-10 remains planned and untagged for IR-667 because the public meter does not expose the normalize charge bit amount. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-363 Status | examined | spec/functional/FR-363-metered-decimal-ordering.md:45 | PR #9 adds executable Trace bindings for AC-6 and AC-7. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-364 Status | examined | spec/functional/FR-364-ieee-flag-iteration.md:25 | PR #9 adds executable Trace bindings for AC-1. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-365 Status | examined | spec/functional/FR-365-decimal-rounding-ties-and-default.md:28 | PR #9 adds executable Trace bindings for AC-1 and AC-2. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-366 Status | examined | spec/functional/FR-366-ieee-exceptional-value-semantics.md:28 | PR #9 adds executable Trace bindings for AC-1 through AC-4. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-367 Status | examined | spec/functional/FR-367-text-profile-mismatch-before-charging.md:25 | PR #9 adds executable Trace bindings for AC-1. Run `quire matrix` for the current criterion-to-test mapping. |
| FR-368 Status | examined | spec/functional/FR-368-meter-charge-and-limit-vocabulary.md:26 | PR #9 adds executable Trace bindings for AC-1 and AC-2. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-441 Status | examined | spec/test-cases/TC-441-an-unbounded-collection-never-refuses-for-cardinality.md:32 | PR #9 adds executable Trace bindings for FR-097-AC-8. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-905 Status | examined | spec/test-cases/TC-905-single-member-integer-division.md:34 | PR #9 adds executable Trace bindings for FR-357-AC-7. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-909 Status | examined | spec/test-cases/TC-909-admission-before-large-exact-work.md:33 | PR #9 adds executable Trace bindings for FR-361-AC-7 through AC-9. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-911 Status | examined | spec/test-cases/TC-911-metered-decimal-ordering.md:31 | PR #9 adds executable Trace bindings for FR-363-AC-6 and AC-7. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-912 Status | examined | spec/test-cases/TC-912-ieee-flag-iteration.md:25 | PR #9 adds executable Trace bindings for FR-364-AC-1. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-913 Status | examined | spec/test-cases/TC-913-decimal-rounding-ties-and-default.md:26 | PR #9 adds executable Trace bindings for FR-365-AC-1 and AC-2. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-914 Status | examined | spec/test-cases/TC-914-ieee-exceptional-value-semantics.md:28 | PR #9 adds executable Trace bindings for FR-366-AC-1 through AC-4. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-915 Status | examined | spec/test-cases/TC-915-text-profile-mismatch-before-charging.md:25 | PR #9 adds executable Trace bindings for FR-367-AC-1. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-916 Status | examined | spec/test-cases/TC-916-meter-charge-and-limit-vocabulary.md:26 | PR #9 adds executable Trace bindings for FR-368-AC-1 and AC-2. Run `quire matrix` for the current criterion-to-test mapping. |
| TC-906 Status | context_only | spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md:41 | Existing AC-1..AC-6 evidence predates IR-673; AC-8..AC-11 require new executable tests. |
| TC-910 Status | context_only | spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md:37 | AC-1 through AC-9 have pre-IR-673 tags. AC-10 through AC-18 require direct new tests. |
| FR-359 Status | context_only | spec/functional/FR-359-cumulative-meter-boundary.md:31 | Planned. |
| TC-907 Status | context_only | spec/test-cases/TC-907-cumulative-meter-boundary.md:31 | Planned for the IR-653 CODE stage. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The rewrite of FR-358's Status (:36) deleted "AC-7 was moved to FR-368 without reusing its ID". That sentence was the recorded fix for spec-review SR-2420 FND-005 (merged in spec PR #8). The AC table now jumps from FR-358-AC-6 to FR-358-AC-8 with nothing explaining the gap, so a later author can reasonably reuse AC-7 and silently alias a retired criterion. The dropped "AC-3 and AC-6 require inspection" clause loses nothing, because their Verification column still says Inspection. Fix: keep the new matrix pointer and restore one sentence, e.g. "AC-7 was retired to FR-368; its ID is not reused." | spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md:36 |
