---
id: SR-17940486709564847047951
title: "IR-1446 PR #159 gap-analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-exact PR #159 at 62fefed0, git diff origin/main...HEAD; IR-1446; src/native_float.rs, src/native_float/kani_proofs.rs, tests/ir1446_native_float.rs, scripts/ir1446_native_float_mutants.py, spec/functional/FR-405-kani-tractable-native-float-carrier.md, spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-405
    type: reviews
---

## Summary

Ticket: IR-1446. This is a planless gap analysis; plan completion was not assessed. `quire matrix --scope .` at 62fefed0 gives these FR-405 rows:

- AC-1 to AC-6, AC-8, AC-10, AC-11 and AC-13: `tagged`. The AC-10 mutant runner is tagged but `#[ignore]`.
- AC-7, AC-9, AC-14 and AC-15: `method-without-symbol`. These are Inspection criteria, each bound to a tagged source-scan test.
- AC-12: `untagged`.

All new production code is owned by FR-405: the carrier, the two entry points and the harness module. `IeeeFlags::with` is widened only for it. No stub, `todo!` or `unimplemented!` exists, and no test asserts a constant against itself. Each differential compares against the shipped owner or against independent literals.

The coverage gaps of the tagged criteria (the FMA grid reduction, the missing mutants, the weak evidence checks) and the Kani evidence defects (the NaN check, the formulation) are recorded once, in the code-review artifact (SR-17940486709564847047950, FND-001 to FND-005). They are not repeated here.

## Verdict

One traceability gap that belongs to this PR: AC-12 has no tagged test.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-405-AC-12 (vacuity cover reached and a harness mutant failed per VERIFIED harness) is `untagged` in the matrix. Its evidence is prose in TC-953 ('All eight reported VERIFICATION:- FAILED'). The eight harness mutants have no script or recorded edit, so they cannot be re-run. Tag the evidence check (for example, extend the AC-11 evidence test to assert the cover and harness-mutant record per VERIFIED row), and add the eight harness mutants to the mutant script or to TC-953 as exact edits. | spec/functional/FR-405-kani-tractable-native-float-carrier.md:88, spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:104 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-405-AC-1..AC-11, AC-13..AC-15 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:77-91 | matrix rows tagged or method-without-symbol (Inspection), bound to tests/ir1446_native_float.rs |
| FR-405-AC-12 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:88 | Each VERIFIED harness has a vacuity cover that is reached and a harness mutant that it fails, and its expected side determines the full outcome of Behavior 19 or lists exactly the outputs it covers. |

## Dispositions

Disposition pass 1, at 6986f4fd (branch rebased on main since the reviewed 62fefed0; `src/` unchanged since c7ea531, the commit the Kani record measured). `quire matrix --scope .` at 6986f4fd: FR-405-AC-12 is `tagged` (tests/ir1446_native_float.rs:977, and :1027 ignored); AC-7, AC-9 and AC-14 are `method-without-symbol` (Inspection by code review, no test); every other row as before.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | AC-12 is tagged on `every_verified_harness_has_reached_covers_and_a_failing_harness_mutant` (record check: one vacuity row per VERIFIED evidence row, "k of k" covers, the mutant is a named `m("...")` edit of the host mutant script, verdict FAILED) and on the ignored `every_harness_mutant_fails_its_harness_under_kani`, which runs `scripts/ir1446_native_float_kani_mutants.py` (exact-text edit in a scratch copy, `cargo kani --harness <id> --exact --jobs 1 --solver cadical`, requires `VERIFICATION:- FAILED`). The 13 harness mutants are re-runnable. After: `#[trace("TC-953", "FR-405-AC-12")] fn every_verified_harness_has_reached_covers_and_a_failing_harness_mutant() { ... assert_eq!(recorded, expected, "one vacuity row per VERIFIED harness"); ... assert!(script.contains(&format!("m(\"{}\"", cells[2])), "{cells:?}"); assert_eq!(cells[3], "FAILED", "{cells:?}"); }` |
