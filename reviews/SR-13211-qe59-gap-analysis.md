---
id: SR-13211
title: "quire-exact PR 59 gap analysis: FR-383 implementation and coverage delta"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@396a7551c5719005dfeb72844464945fe52400e3 vs origin/main 248ea79; src/accounting.rs, tests/ir895_meter_cancel.rs, spec/functional/FR-383-meter-cancellation-handle-access.md, spec/test-cases/TC-933-meter-cancellation-handle-access.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-383
    type: reviews
---

## Summary

Ticket: IR-895. Plan completion: not assessed.

Coverage was measured with quire 0.36.2 (engine 0.50.2), which is the latest release.

Strict coverage (`quire coverage --scope . --strict`) goes from 169/188 backed at origin/main to 171/188 at the PR head. The unbacked rows go from 19 to 17, and contradicted stays at 0 on both. FR-383 goes from 0/4 to 2/4. The newly backed rows are FR-383-AC-1 (tests/ir895_meter_cancel.rs:26) and FR-383-AC-2 (tests/ir895_meter_cancel.rs:46). Strict mode exits 1 on both base and head, so that failure was already present.

Strict matrix (`quire matrix --scope . --strict`) goes from 6 untagged to 4. The 4 untagged rows that remain are FR-359-AC-9, FR-359-AC-10, FR-371-AC-6 and FR-383-AC-4. On this count base is 182/188 and head is 184/188, which matches the author's numbers. FR-383-AC-3 is `method-without-symbol` (Inspection).

The 17 rows strict coverage still leaves unbacked are FR-089-AC-2, FR-090-AC-7, FR-090-AC-11, FR-090-AC-12, FR-090-AC-13, FR-359-AC-9, FR-359-AC-10, FR-359-AC-11, FR-369-AC-6, FR-369-AC-8, FR-371-AC-6, FR-372-AC-6, FR-375-AC-7, FR-378-AC-7, FR-380-AC-7, FR-383-AC-3 and FR-383-AC-4. Of these, only FR-383-AC-3 and AC-4 belong to this ticket. Both are verified by non-symbol methods: Inspection for AC-3, and the `make build-no-std` gate for AC-4, which passes at the head.

The production code added is `Meter::cancel`, and FR-383 owns it. The diff adds no code without an owning requirement. Neither new test is a stub or a tautology: mutants M1, M2, M4 and M5 are each killed (see SR-13210). The changed FR-383 and TC-933 Status sections state the computed matrix result exactly: AC-1 and AC-2 tagged, AC-3 method-without-symbol, AC-4 untagged. The diff includes no author-written review file.

## Examined criteria

- FR-383-AC-1 (examined): `Meter::new(limits).cancel()` is `None`. After `with_cancel(handle.clone())`, `cancel()` is `Some(&Cancel)` equal by shared-state identity to `handle`, and unequal to an independently created handle. Replacing the handle reports only the replacement. The test backs it.
- FR-383-AC-2 (examined): Cancel the handle obtained from `meter.cancel()` with `Requested`, then charge the meter under sufficient limits: the charge is denied at its named point, that handle records `tripped() = Some(Requested)`, and all accounting state stays unchanged. On a separate fresh meter, cancel an independent handle while its installed handle remains live: an otherwise identical charge is admitted. The test backs it.
- FR-383-AC-3 (examined): The accessor's implementation borrows the existing optional handle without cloning it or constructing a second `Meter` or `Counters`; calling it changes no meter accounting or injection state. Inspection confirms it: `self.cancel.as_ref()`.
- FR-383-AC-4 (examined): The public `Meter::cancel(&self) -> Option<&Cancel>` signature compiles in the crate's `no_std` + `alloc` build. `make build-no-std` passes.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The PR description's Evidence says "Strict matrix coverage changed from 182/188 to 184/188 backed criteria". Those are `quire matrix --strict` tagged counts. On `quire coverage --strict` the "backed" metric goes from 169/188 to 171/188. The delta of +2 (AC-1 and AC-2) is right, but the label conflates two metrics, so a reader comparing against `quire coverage` sees a 13-row mismatch. This is outside the diff and the committed Status text is accurate. | PR #59 body, Evidence bullet 4 |

## Verdict

PASS. Every FR-383 criterion is satisfied by its stated method. The coverage delta is +2 backed and 0 contradicted. The status wording in the diff is honest. The one low finding is about wording in the PR description only, does not block, and needs no code change. Mergeable as-is.
