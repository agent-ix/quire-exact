---
id: SR-16636
title: gap-analysis review of IR-844 Meter Kani proof disablement
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@9c267346025f643856c4d29ae3a9d08e5727569e; src/accounting.rs, src/accounting/kani_proofs.rs, tests/meter_kani_gate.rs, scripts/meter_kani_gate.py, scripts/meter_kani_scope.sh, Makefile, docs/meter-kani-gate.md, spec/functional/FR-359-cumulative-meter-boundary.md"
review_set: subset
---

## Summary

Ticket: IR-844. Planless gap analysis of PR #87 at 9c267346025f643856c4d29ae3a9d08e5727569e. Plan completion: not assessed.

`quire matrix --scope . --strict` (quire 0.36.2, engine 0.50.2) at this head reports FR-359-AC-9 as `untagged` with symbol `(none)`. FR-359-AC-1 to AC-8 and AC-12 stay `tagged`, so their bindings are unaffected. AC-10 is `untagged` and AC-11 is `method-without-symbol`, the same as before this PR.

The PR removes the only FR-359-AC-9 binding, `tests/meter_kani_gate.rs::meter_proof_and_production_all_loop_control`. That test now only panics. Keeping the tag would have been coverage inflation by a dummy test, which TC-931 forbids. Removing it is correct, and the AC row is left unchanged and unqualified, as ordered.

The diff adds no stubs that claim coverage, no code without an owning requirement, and no coverage inflation. No run evidence was produced or bound. Under the operator's order, no Kani or Cargo command ran.

## Verdict

**CHANGES REQUESTED.** The matrix truthfully shows FR-359-AC-9 as untagged. However, FR-359's own evidence paragraph still states that a `#[trace("FR-359-AC-9")]` binding exists and that it passed. That paragraph sits outside the AC row and outside the diff. The spec text and the computed matrix now disagree about AC-9's binding.

## Examined scope

- `FR-359-AC-9` (examined): The `cfg(kani)` harness `meter_integer_arithmetic_schedule_matches_reference` uses production `Meter::charge` for the exact three-charge FR-362 schedule ... For the baseline, any `UnwindBoundExhausted`, `MemoryExhausted`, timeout, compile error or missing assertion fails acceptance.
- `tests/meter_kani_gate.rs` (examined): ignored test with no trace tag, whose body is a `panic!` with the DO NOT RUN warning.
- `src/accounting/kani_proofs.rs` (examined): the untagged proof, compiled out by `#[cfg(all(kani, any()))]`.
- `FR-359-AC-1` (context_only): its binding is unchanged and still `tagged`.
- `spec/functional/FR-359-cumulative-meter-boundary.md` (context_only): the line 71 evidence paragraph for IR-810.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-359's evidence paragraph still says `#[trace("FR-359-AC-9")]` on `meter_proof_and_production_all_loop_control` binds the criterion and that the lane passed, but the matrix at head shows AC-9 untagged and the gate disabled | spec/functional/FR-359-cumulative-meter-boundary.md:71 |

## New findings (disposition pass 1)

Disposition pass 1 re-checked fixed head 9b186c75b91ff6de838320fbbf361cf7eb1a06c7. `quire matrix --scope . --strict` (quire 0.36.2, engine 0.50.2) exits 1 and reports 12 untagged criteria and 0 tagged only by ignored tests: FR-359-AC-9, FR-359-AC-10, FR-371-AC-6, FR-383-AC-4, FR-384-AC-5 and FR-385-AC-1..AC-7. FR-359-AC-9 is `untagged` with symbol `(none)`. No live `trace("FR-359-AC-9")` tag exists in src/, tests/ or scripts/. No Kani, proof or Cargo command ran.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | FR-359's "IR-844 baseline evidence" paragraph still ends in the present tense: "the executable local test binding is recorded below". It carries no historical label of its own, and the paragraph below now says that binding was removed | spec/functional/FR-359-cumulative-meter-boundary.md:67 |
| FND-003 | low | The rewritten "At this head" sentences list the untagged criteria as only AC-9, FR-359-AC-10, FR-371-AC-6 and FR-383-AC-4. The matrix at head shows 12 untagged, adding FR-384-AC-5 and FR-385-AC-1..AC-7 | spec/functional/FR-359-cumulative-meter-boundary.md:73, spec/test-cases/TC-931-single-pass-meter-accounting.md:47 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-359:71 no longer claims a live binding: "its former `#[trace("FR-359-AC-9")]` binding has been removed. The prior lane receipt is historical, not evidence that this disabled head passed." FR-359:73 reads "At this head, Quire reports AC-9 untagged", which matches the computed matrix |
