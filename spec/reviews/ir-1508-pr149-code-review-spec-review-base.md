---
id: SR-17940486709564847047686
title: "IR-1508 PR #149 FR-403 and TC-951 Status spec review"
type: SpecReview
analysis: base
date: 2026-11-11
scope: "agent-ix/quire-exact; PR #149 @ 0cc3bb82; IR-1508; Status sections of spec/functional/FR-403-native-decimal-operations-refine-owner-path.md and spec/test-cases/TC-951-native-decimal-operations-match-owner-path.md, TC-951 Description"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-403
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-951
    type: reviews
---

## Summary

Ticket: IR-1508. This is a spec-review (base) of PR #149's spec edits: the FR-403 Status line, the TC-951 Description's last sentence and the TC-951 Status line. It also rules on the coder's declared spec gap in FR-403 Behavior 3 (b)/(c).

I ran `quire validate --scope . <file>` on both changed files and on their origin/main versions. All four runs exit 0 with identical output (only the pre-existing module advisories), so the edits add no warnings.

Ruling on the coder's gap, which is option (ii), an acceptable implementer choice. In Behavior 3, Divide's `N` and `D` fall under both (b) ("a shifted operand") and (c) ("or `N` or `D`"). The code names a Divide overflow `Numerator` or `Denominator`, and uses `ShiftedOperand` only for Add and Subtract. The overlap decides nothing. `kn = max(0, T + s_b - s_a)` and `kd = max(0, s_a - s_b - T)` cannot both be above zero, so at most one of `N` and `D` is shifted and can overflow. The first failing quantity is therefore the same however (b) and (c) are read, and only its label differs. `Numerator`/`Denominator` is strictly more informative than `ShiftedOperand`. The independent `model()` in the differential pins the label, and so does the reference that Kani will use. No spec edit is needed in this PR. A one-line clarification belongs with the next FR-403 edit (FND-002).

## Verdict

**CHANGES REQUIRED (one medium, one low).** The FR-403 Status is accurate. It credits AC-1 to AC-11 on host evidence. It says that the seven Behavior 37 proofs and the AC-9 approximation proof are written and not executed, that AC-12 is not credited, and that the all-`u32` half of AC-9 rests on host checks until the proof runs. It says that no measurement exists and that AC-13 waits for IR-259. It claims no fit and no budget. I verified independently that the AC-9 all-`u32` property holds (see code-review SR), so the host-only reading is not a hidden failure. The TC-951 Status contradicts itself about step 10.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-951 Status opens with "Run on the host: steps 1 to 8 and 10 for the input and inspection mutants". Step 10 is "Run each mutant", and each mutant is "a buildable variant of the named site in a scratch copy". The same Status later says "the mutants have not been built and run as a set", and the PR body confirms that the coder built none. Asserting each mutant's distinguishing input is step 1 to 8 evidence, not a step 10 run. This repeats PR #148's TC-950 FND-001. Drop "and 10" from the run list. Say that the boundary set holds each mutant's distinguishing input, that the mutants were not run, and that the reviewer ran eight controls, all killed (listed in the code-review SR). | spec/test-cases/TC-951-native-decimal-operations-match-owner-path.md:103 |
| FND-002 | low | In FR-403 Behavior 3, (b) "a shifted operand" and (c) "... or `N` or `D`" both cover Divide's numerator and denominator, and neither Behavior 2 nor AC-1 says how a Divide overflow is named. Because `kn` and `kd` are never both above zero, the overlap changes no decision, only the name (Numerator or Denominator against ShiftedOperand), so this is a wording gap and not a behavioural one. Ruling (ii): the coder's naming is an acceptable implementer choice, and no edit is required in this PR. At the next FR-403 edit, restrict (b) to "a shifted operand of `Add` or `Subtract`" and keep `N`/`D` in (c), or name the Divide quantities explicitly. | spec/functional/FR-403-native-decimal-operations-refine-owner-path.md:51 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-403 Status | examined | Implemented (IR-1508): the admission, evaluation, gcd, ordering and read-out, with the host differential, the independent reference binding and the source inspection (AC-1 to AC-11). The seven proofs of Behavior 37 and the approximation proof of AC-9 are written and not executed ...; AC-12 is not credited, and the all-u32 half of AC-9 stands on the host checks until the proof runs. No measurement exists and none is claimed; AC-13 waits for CG's decimal row (IR-259). |
| TC-951 Description | examined | ... The host steps have run (see Status); the Kani proofs and the CG measurement have not. |
| TC-951 Status | examined | Run on the host: steps 1 to 8 and 10 for the input and inspection mutants ... the mutants have not been built and run as a set. The seven proofs of step 9 and the all-u32 proof of step 6 are written and unexecuted. Step 11 waits for IR-259. |
| FR-403 Behavior 3 | examined | ... (b) a shifted operand lies outside signed 128 bits; (c) the aligned sum, difference, product or negation, or N or D, lies outside signed 128 bits ... |
| FR-403-AC-9 | context_only | ... the lower and upper approximations of log2(10) ... give the same floor for every k from 0 to 4294967295, which is checked by a proof over all symbolic u32 values. |
| FR-403-AC-12 | context_only | For each of the six operations and the ordering, a Kani proof with no unwind bound verifies the refinement of Behavior 37, with its covers. |

## Dispositions

Round 1, head cf296b07. `quire validate --scope .` on FR-403 and TC-951 at cf296b07 exits 0 with no new warnings. The consequent doc-comment drift in operations.rs is recorded as code-review FND-003.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | TC-951 Status now reads "Run on the host: steps 1 to 8 ... The boundary set holds each mutant's distinguishing input ... but the TC-951 mutants were not built and run as a set (step 10). The PR #149 reviewer ran eight controls on the unmutated branch, all killed by an assertion failure: M1 ... M8 ...". It also gives the differential's real size, 19 types and 38 operands. |
| FND-002 | fixed | FR-403 Behavior 3 (b) now reads "a shifted operand of `Add` or `Subtract` lies outside signed 128 bits", and `N` and `D` stay in (c). The code was already consistent with this (Divide names Numerator or Denominator). |
