---
id: SR-17940486709564847047991
title: "IR-1541 PR #167 FR-409 and TC-957 spec review"
type: SpecReview
analysis: base
date: 2026-11-14
scope: "agent-ix/quire-exact PR #167 at 12323e22, git diff origin/main...HEAD; IR-1541; spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md, spec/test-cases/TC-957-native-integer-divide-modulo-plan.md, spec/functional/FR-384-admitted-native-integer-refinement.md (one row)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-409
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-957
    type: reviews
---

## Summary

Ticket: IR-1541. Spec-only PR: new FR-409 (Behavior 1-23, AC-1..10), new TC-957, one sentence in FR-384's capability table.

Checked against quire-exact main `src/native_integer.rs` and `src/division.rs`: `NativeIntegerOperation` and `NativeIntegerOperationKind` have only Add, Subtract, Multiply and Negate and neither is `#[non_exhaustive]` (only `NativeIntegerSchedule` is). `DivisionProfile` has three laws (Truncating, Floor, Euclidean); `DivisionMember` is Quotient or Remainder; `modulo` and `evaluate_native_integer_modulo` are always Euclidean. The plan-free `native_division` charges operands, returns `Undefined(DivisionByZero)` before the arithmetic charge, then arithmetic, domain, and refuses `DivisionOutOfDomain`/`ModuloOutOfDomain` after the domain charge and before result-retain, which matches Behavior 7-9.

Capacity rules recomputed by hand: `|q| <= |a|` holds under all three laws for every nonzero divisor (the only escape is `i128::MIN` by `-1`); `|r| <= |b| - 1` for truncating/floor and `0 <= m <= |b| - 1` for Euclidean. Behavior 4/5 are sound and match CG FR-014's inferred intervals (`div [-A, A]`, `mod [0, max(B-1,0)]`, FR-014-AC-81 `rem`). Behavior 4 refusing MIN/-1 even under a result interval matches FR-384's rule that each operation's entire mathematical result interval is checked against capacity. AC-2 and AC-3 vectors verified (`-7 div 2` = -3/-4/-4, `-7 div -2` = 3/3/4, `-7 mod +-2` = 1, `i64::MIN div -1` = 2^63, `i64::MIN mod -1` = 0). The window table of Behavior 15 is correct and holds exactly `|b|` integers, so the quotient check is not tautological. All 13 TC-957 mutant rows' distinguishing inputs do distinguish. `quire validate --scope . 'spec/**/*.md'` passes (quire 0.36.2).

## Verdict

Not mergeable. The plan design, capacity rules and host evidence are sound and consistent with CG FR-014/FR-057 and AD-006. The settlement and mutant machinery has two contradictions that no implementation can satisfy as written (FND-001, FND-002), plus a self-contradiction in the remainder expected side (FND-003) and a churn-policy requirement (FND-005). All are fixable in this PR with text changes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | AC-7/Behavior 19 require every VERIFIED evaluation harness to FAIL "its harness mutant (Behavior 22)", but TC-957's mutant table names only five w=8 evaluation harnesses, leaving 72 of the 77 with no assigned mutant, and for the constant-divisor `-1` quotient harnesses (3 profiles x 2 widths) and the `Modulo` `-1` harnesses no Behavior 22 mutant is distinguishable (every law gives `q = -a`, every remainder is 0, the divisor is never 0). Those harnesses can be neither VERIFIED nor a measured row, so "no combination left without a record" is unsatisfiable. Define a mutant per harness (or per shape) that its shape can kill, or exempt shapes where none exists with a stated record. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:97, :124, :106-111; spec/test-cases/TC-957-native-integer-divide-modulo-plan.md:32-46 |
| FND-002 | high | Behavior 22 and AC-9 require each mutant, including "zero-divisor check moved after the arithmetic charge", to also yield a Kani FAILED verdict. Moving the guard changes only charge events, which Behavior 16 says harnesses do not determine (real Meter denying no charge), so no harness can fail it; TC-957's mutant table says "none (charges are host-only)". AC and TC contradict; carve the exception into Behavior 22 and AC-9. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:108-111, :126; spec/test-cases/TC-957-native-integer-divide-modulo-plan.md:43 |
| FND-003 | medium | Behavior 15 says the expected side "SHALL share no function with the production division body" yet obtains the remainder harness's companion quotient `q` "from the production quotient of the same profile". At w=128 that quotient plan is refused by Behavior 4 for the full dividend interval, so the source of `q` is also unbuildable through the plan. State the permitted source (e.g. plan-free evaluator, or an independent expectation such as CG FR-057's shifted `rem_euclid`, which Behavior 15 currently forbids) and reconcile with AC-6. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:85, :123 |
| FND-004 | medium | Behavior 14's shapes fix only the dividend and divisor intervals and say nothing about the result interval, yet Behavior 15/16 require the out-of-domain refusal and its cover, and TC-957's "Result-interval check skipped" mutant is assigned to the truncating quotient w=8 harness. Whether each evaluation harness draws a result interval (and how) is undefined. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:77-81, :86, :90; spec/test-cases/TC-957-native-integer-divide-modulo-plan.md:46 |
| FND-005 | medium | Behavior 18, AC-7 and TC-957 step 6 / evidence table require recording the Kani, CBMC and solver versions and the commit per harness. Recorded tool versions and revisions are the churn artifact removed under QSL-406 and EA-49; drop the requirement (and the Tools/Commit columns), leaving outcome, time, peak with provenance and harness id. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:95, :124; spec/test-cases/TC-957-native-integer-divide-modulo-plan.md:25, :50, :54 |
| FND-006 | low | Behavior 19 says each harness settles as "exactly one of" VERIFIED or a measured row, then adds a third state: larger symbolic widths "recorded as covered by that smallest failing width" without being run. Name that state as an allowed terminal record in Behavior 19 and AC-7. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:96-100, :124 |
| FND-007 | low | AC-9 requires the MIN / -1 mutant's plan-harness playback to be "replayed through divide ... over Integer" giving a different outcome; the plan harness draws intervals and performs no division, and the difference is plan refusal vs construction. State the plan-level comparison for that mutant. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:111, :126 |
| FND-008 | low | AC-5's verification is "Test (TC-957)" and says the harnesses "run", but TC-957 step 5 only lists them ("There is no automated step") and running is AC-7's step 6. Make AC-5 Inspection of presence and shape, or name its automated check. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:122; spec/test-cases/TC-957-native-integer-divide-modulo-plan.md:24 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-409 | examined | The kernel SHALL extend the checked native integer plan of FR-384 with a Divide operation per DivisionProfile and a Modulo operation ... Each Kani harness over the plan SHALL settle as an executed VERIFIED verdict or as a measured not-tractable row |
| FR-409-AC-1 | examined | NativeIntegerOperation has Divide { profile, member, dividend, divisor } and Modulo { dividend, divisor } ... unary operands return OperandShapeMismatch naming Divide or Modulo, each before any charge. |
| FR-409-AC-2 | examined | A quotient plan whose dividend interval contains i128::MIN and whose divisor interval contains -1 returns CapacityExceeded { operation: Divide } under each profile ... Each capacity equals Behavior 5's interval |
| FR-409-AC-3 | examined | On the Behavior 21 grid ... returns the same outcome kind, value and refusal payload as divide or modulo over Integer and as the plan-free evaluator, with equal charge events |
| FR-409-AC-4 | examined | charge_native_integer_schedule over an admitted Divide or Modulo operation produces the events and counters of NativeIntegerSchedule::Division or Modulus |
| FR-409-AC-5 | examined | The plan harness of Behavior 13 runs ... and the evaluation harnesses of Behavior 14 run for each of the seven member combinations in each shape. |
| FR-409-AC-6 | examined | Code review of each harness finds an expected side that shares no function with the production division body |
| FR-409-AC-7 | examined | Every harness of AC-5 has one terminal record: VERIFIED with every cover reached and its harness mutant FAILED, or a measured row |
| FR-409-AC-8 | examined | Each measured row appears in TC-957's evidence record and in the tractability table that IR-1526's seam loads. |
| FR-409-AC-9 | examined | Each mutant of Behavior 22 fails the host differential ... Each also yields a Kani FAILED verdict with concrete playback |
| FR-409-AC-10 | examined | No proof is claimed for a harness without an executed VERIFIED verdict. |
| TC-957 | examined | Verify FR-409-AC-1 through FR-409-AC-10 ... mutant table of 13 rows, empty evidence record |
| FR-384 capability table row | examined | FR-409 adds checked Divide and Modulo operations to this set. |
| CG FR-057 | context_only | On the native route ... When the plan exists, the native oracle SHALL call the native evaluators through it, with the same expected side and outcome mapping. |
| CG FR-014-AC-21 / AC-81 | context_only | div or mod derive [-A, A] or [0, max(B - 1, 0)] ... rem infers [0, max(B - 1, 0)] (Euclidean) or [-max(B - 1, 0), max(B - 1, 0)] |

## Dispositions

Round 1, fix head 6b9b9104 (reviewed head 12323e22). Verified against `git diff 12323e22 6b9b9104`, not the author's summary; `quire validate --scope . 'spec/**/*.md'` passes. Each profile-swap distinguishing input in Behavior 22's family table and TC-957's new rows re-derived by hand (e.g. divisor -7, dividend -1: Euclidean (1, 6) vs truncating (0, -1); dividend 1 would not distinguish and is correctly not used).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Behavior 22 adds a per-family assigned-mutant table covering every plan and evaluation harness; constant -1 shapes take the result-interval-skipped mutant, made reachable by Behavior 14's drawn result interval; TC-957 adds the matching rows. |
| FND-002 | fixed | Behavior 22, AC-9 and TC-957 step 8 make the moved zero guard host-only. |
| FND-003 | fixed | Behavior 15 takes the companion quotient from the plan-free evaluate_native_integer_division as a second subject output (uniqueness of (q, r) in W keeps it non-tautological) and handles MIN / -1 with r == 0. |
| FND-004 | fixed | Behavior 14 requires every evaluation harness to draw the result interval as None or a sorted unrestricted pair. |
| FND-005 | fixed | Tool versions and commit removed from Behavior 18, AC-7, TC-957 step 6 and the evidence table. |
| FND-006 | fixed | Behavior 19, AC-7 and TC-957 name the third state "not run, covered by the smallest failing width". |
| FND-007 | fixed | Plan-harness kills replay the intervals through the healthy NativeIntegerPlan::new (Behavior 22, AC-9, TC-957 step 8). |
| FND-008 | fixed | AC-5 is now presence/shape Inspection (TC-957). |
