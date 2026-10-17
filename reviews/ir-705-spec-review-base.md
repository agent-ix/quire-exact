---
id: SR-11985
title: "IR-705 quire-exact PR 52 spec review: FR-359 and TC-931 AC-9 deferral and delivery status edits"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact@9eab47cc66fa5fcc6a88ed3e38b4dbed01e8d60d; spec/functional/FR-359-cumulative-meter-boundary.md; spec/test-cases/TC-931-single-pass-meter-accounting.md"
review_set: subset
---

## Summary

Ticket: IR-705. Spec review of the PR's edits to FR-359 (AC-9 row, AC-12
evidence line, status paragraph) and TC-931 (Description, step 3, Status).
Both files pass `quire validate`. The question was whether the AC-9 deferral
to IR-844 is stated honestly and consistently, not whether to re-run Kani.

Consistent and honest:

- FR-359-AC-9 starts with "PLANNED, deferred to IR-844 (u64 Meter
  redesign)". The FR-359 status paragraph says AC-9 is not delivered, the
  harness carries no `Trace` tag, and no Kani result or mutation control is
  claimed.
- The measured figures (40 GiB, symex 67.4 s, 1,758,078 steps, 74,098 VCCs /
  23,675 after simplification, OOM kill after 123 s in CBMC propositional
  reduction; 16 GiB and per-shape runs also exhausted memory) are the same in
  FR-359, TC-931 Status, the harness comment and the PR body.
- AC-8, AC-11 and AC-12 delivery claims match the matrix: AC-8 and AC-12
  are tagged, AC-11 is Inspection. The AC-12 test names match the tests.
- The AC-9 acceptance thresholds (16 GiB, 600 s, MemoryExhausted fails) are
  unchanged, so the deferral does not weaken the criterion.

## Verdict

PASS WITH FINDINGS (one medium, two low). The deferral is honest. The
medium finding is that TC-931 marks only step 3 as deferred, while step 4 and
the Expected Results still read as current procedure for AC-9.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-931 marks only step 3 "PLANNED, deferred to IR-844", and Status says "AC-9 and this step 3 are planned". Step 4 (the AC-9 ALL-traversal and refusal-order mutation controls) and the Expected Results sentences ("The standalone baseline Kani proof verifies under its specified ceiling ...", "The ALL-traversal control reports ...") are not marked deferred. A reader executing TC-931 under IR-705 would treat them as due now. | spec/test-cases/TC-931-single-pass-meter-accounting.md:20, spec/test-cases/TC-931-single-pass-meter-accounting.md:25, spec/test-cases/TC-931-single-pass-meter-accounting.md:29 |
| FND-002 | low | FR-359-AC-9 (after its PLANNED prefix) and the Behavior paragraph still say, in the present tense, "The `cfg(kani)` harness tagged `Trace: FR-359-AC-9` uses production `Meter::charge`" and "`HARNESS` is the symbol of the `cfg(kani)` harness tagged `Trace: FR-359-AC-9`". The only harness in the tree is deliberately untagged, so no such harness exists yet. Wording such as "the IR-844 harness, once tagged" would avoid the contradiction. | spec/functional/FR-359-cumulative-meter-boundary.md:21, spec/functional/FR-359-cumulative-meter-boundary.md:37 |
| FND-003 | low | The TC-931 Status paragraph says AC-9 is deferred twice ("AC-9 is PLANNED / UNRUN, deferred to IR-844." and "AC-9 and this step 3 are planned for IR-844: ..."). The second sentence also has a misplaced closing parenthesis: "... each per-shape run also exhausted memory.) and carries no Trace tag." | spec/test-cases/TC-931-single-pass-meter-accounting.md:29 |
