---
id: SR-17940486709564847047938
title: "IR-1445 PR #160 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact PR #160 at 75efd007, git diff origin/main...HEAD; IR-1445; spec/functional/FR-401-loop-free-proof-path-text-normalization.md, spec/functional/FR-404-kani-tractable-native-text-carrier.md, spec/test-cases/TC-949-loop-free-proof-path-text-normalization.md, spec/test-cases/TC-952-kani-tractable-native-text-carrier.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-401
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-404
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-952
    type: reviews
---

## Summary

Ticket: IR-1445. Only Status and evidence-record sections changed; no AC text changed. The sub-analyses for EARS, integrity, object and dependency were not run, because no requirement statement, structure or relationship edge changed.

Key question (a) is whether OOM -> unsupported is an honest way to satisfy the criteria.

It is honest. FR-404 Behavior 10 and AC-7 key the record to "the FR-401 normalization instantiated at the chosen C". AC-7 names the outcome explicitly: "a TIMEOUT, an out-of-memory ... with the profiles marked supported fails". Otherwise the profiles "are excluded citing the record, and no owner `TextType` bound is narrowed". AC-9 needs "an executed run with a recorded outcome". The four rows are executed, and the journal corroborates them: the scope wall clocks match and each scope failed with 'oom-kill' at a 24G peak. The profiles are excluded, and no bound is narrowed.

A smaller instantiation (N = 16, D = 8, R = 16) is not required, and it could not change the verdict. Behavior 10 names the chosen C. FR-404's Out-of-scope paragraph forbids "choosing a smaller domain than the owner's so that a proof fits". FR-401's Relation paragraph says that a table that does not fit "leaves the normalizing profiles `unsupported` ... with this requirement still met on the host". A smaller run would be diagnostic only: it would locate the scaling knee. The record says plainly that no such run was made.

What the spec does allow, and the PR did not try, is a cheaper implementation re-run at C = 64. FR-401 Behaviors 1 and 6 constrain the tables and the loop forms, not the nesting of the class lookup (code-review SR, FND-001). That is a design choice for the lead, not a criterion gap.

On whether the journal row is an acceptable recorded peak: yes, as an OUT OF MEMORY outcome. The prose states correctly that the cap is a lower bound and not a measured peak. The table cell alone does not say so (FND-003).

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | medium | FR-401-AC-1 and AC-5 say "equals `NormalizationForm::apply` ... for every ordered pair" at D = 32. Some listed pairs cannot fit: U+FDFA U+FDFA under `nfkd` is 36 scalars. The new Status paragraph redefines those pairs as "the tests assert that refusal instead of equality". Status is not normative, so the AC as written is unsatisfiable and the tests pass only under the Status reading. Put the carve-out in the AC text ("or `ExceedsCapacity` where the intermediate exceeds D or the output exceeds R, Behavior 4"). | spec/functional/FR-401-loop-free-proof-path-text-normalization.md:49, spec/functional/FR-401-loop-free-proof-path-text-normalization.md:65 |
| FND-002 | medium | FR-404 Status now reads "Implemented and executed", but FR-399-AC-6 and AC-7 are still "untagged (owner pending planner ruling)". The PR #158 gap-analysis disposition (SR-17940486709564847047934 FND-002) deferred this on one condition: IR-1445 stays open as the holding ticket, and the ruling replaces "owner pending planner ruling" with a ticket id before IR-1445 closes. This PR is the last IR-1445 change. Name the owning ticket, or state that IR-1445 stays open for it. | spec/functional/FR-404-kani-tractable-native-text-carrier.md:76, spec/test-cases/TC-952-kani-tractable-native-text-carrier.md:76 |
| FND-003 | low | The four FR-401 rows of the TC-952 evidence table put the configured cap, 25769803776 bytes, in the "Peak memory" column, with the same format as a measured peak. Only the prose says "the cap is not a measured peak; it is a lower bound". A reader of the row, or a binder that parses only the cell, sees a measurement. Mark the cell, for example `>= 25769803776 bytes (cap, OOM)`, and adjust the binder's parse. Also record where symbolic execution stopped (DECODE slot 58 or 59 of 64, in `class_of` under the copy loop): it is the evidence for "had not reached the SAT solver". | spec/test-cases/TC-952-kani-tractable-native-text-carrier.md:50, spec/test-cases/TC-952-kani-tractable-native-text-carrier.md:63-66 |
| FND-004 | low | FR-401 Behavior 1 says "constant tables" and Behavior 2 says "generated const data", and AC-4(b) reads operand types from "a table const". The tables are `pub(super) static`. Behaviour is the same (immutable, and static is the better choice), but the wording does not match. Say "constant (immutable `static`) tables" in Behavior 1 and 2 and "table const or static" in AC-4(b). | spec/functional/FR-401-loop-free-proof-path-text-normalization.md:38-39 |
| FND-005 | low | TC-949 Expected Results still claims that each listed mutant "fails" a step, while the new Status says the mutants are not run and that other controls "stand in" for them. Expected Results then asserts outcomes that nothing executes. Mark those mutant lines as rationale, or move them under Status as not executed. | spec/test-cases/TC-949-loop-free-proof-path-text-normalization.md:30, spec/test-cases/TC-949-loop-free-proof-path-text-normalization.md:36 |

## Verdict

The AC-7 and AC-9 verdict is honest and meets the spec as written; no smaller instantiation is required. Mergeable once FND-001 is fixed (the AC text must carry the refusal carve-out the tests implement) and FND-002 is fixed (an owning ticket for FR-399-AC-6 and AC-7, or an explicit statement that IR-1445 is held open). FND-003 to FND-005 are low.

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-006 | medium | FR-399 and TC-947 are stale, and the AC-6/AC-7 deviations are recorded in the wrong spec. FR-399 Status still says "Planned (IR-1499). No Rust, test or proof exists", and TC-947 Status says "Planned (IR-1499)", yet three TC-947-tagged tests now bind FR-399-AC-6 and AC-7. The two deviations (`admit_text` plus a built oracle in place of the missing host `from_admitted`, and the AC-7 allocation check over supported profiles only) are written only in TC-952 Status, under FR-404. Update FR-399 and TC-947 Status (owner IR-1445, the bound tests, both deviations), or amend AC-6 and AC-7. | spec/functional/FR-399-non-metered-text-constructor-from-admitted-parts.md:58-60, spec/test-cases/TC-947-non-metered-text-constructor-from-admitted-parts.md |

## Dispositions

| FND | outcome | reason |
|---|---|---|
| FND-001 | fixed | The FR-401-AC-1 text now carries the carve-out: "Where the intermediate form of a pair exceeds D = 32 scalars ... or its output exceeds R = 64 bytes, the function returns `ExceedsCapacity` (Behavior 4) and the test asserts that refusal in place of equality." AC-5 adds "Pairs whose buffers do not fit (the refusal of AC-1) are not compared." The Status paragraph was removed. |
| FND-002 | fixed | FR-404 and TC-952 Status now say FR-399-AC-6 and AC-7 are "owned by IR-1445 (planner ruling)" and bound in TC-947. The stale FR-399/TC-947 side is new FND-006. |
| FND-003 | fixed | Each FR-401 row's peak cell reads "cap 25769803776 bytes (OOM kill; not a measured peak)". The record states where each shape stopped: shape 1 in symbolic execution at decode slot 59 (58) of 64; shape 2 after symex and post-processing, in the SAT stage. The journal and logs corroborate both. |
| FND-004 | fixed | Behaviors 1 and 2 now say "immutable `static` arrays". AC-4(b) still says "table const", which a reader takes to mean the table, so that is accepted as is. |
| FND-005 | fixed | TC-949 Expected Results now opens with "The mutant lines below are the rationale for the controls, not executed results". |
| FND-006 | fixed | FR-399 Status now splits AC-1 to AC-5 (Planned, IR-1499) from AC-6 and AC-7 (owned by IR-1445, bound by the three named tests). It records both deviations: `admit_text` as the host counterpart, and supported profiles only for the allocation check. TC-947 Status says the same. |
