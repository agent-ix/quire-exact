---
id: SR-17940486709564847047952
title: "IR-1446 PR #159 spec-review (base)"
type: SpecReview
analysis: base
date: 2026-11-13
scope: "agent-ix/quire-exact PR #159 at 62fefed0, git diff origin/main...HEAD -- spec/; IR-1446; spec/functional/FR-405-kani-tractable-native-float-carrier.md (Status), spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md (Evidence record, Status)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-405
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-953
    type: reviews
---

## Summary

Ticket: IR-1446. The spec diff fills FR-405 Status and TC-953's Evidence record and Status. It records four operations VERIFIED at both widths (Negate, NumericEqual, TotalOrder, BitIdentical), at 1.5-1.8 s and at most 0.66 GB. It records six arithmetic operations as 'table row: timeout' at both widths (600 s, 22.7-23.3 GB). Only the base review ran. The diff edits Status and evidence sections and adds no requirement statement, AC or relationship, so the EARS, integrity, object and dependency sub-analyses have nothing in scope.

Question (a). In its letter, AC-11 accepts 'a measured row ... timeout', and Behavior 21 sets the 10-minute limit. So the 600 s cap is the spec's own limit, not a shortcut against the usual 30-minute cap. The `--unwind 57` is not the cost driver either: CBMC unwinds constant-bound loops only as far as they go, and the probes in SR-17940486709564847047950 also ran with an unwind bound. The rows are honest about what ran. But they do not satisfy AC-11 in substance, for two reasons. First, the harness that ran would end FAILED, not VERIFIED, given time (code-review FND-001), and AC-11 says a FAILED verdict fails the combination. Second, the timeout comes from the six-spelling symbolic formulation (code-review FND-002). A retry is warranted before merge, with the host side made NaN-check-safe and one harness per spelling. A bit-level reference expected side is not needed: the host float side verifies quickly once the spelling is fixed. The findings below are about the spec text of the record.

## Verdict

Not mergeable until the arithmetic rows are re-measured (code-review FND-001 and FND-002). After that, FR-405 Status and the TC-953 rows must be rewritten from the new runs. The findings below are wording and provenance items for that rewrite.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | TC-953's Test Procedure still says each Kani harness 'draws ... a symbolic rounding spelling and a symbolic meter limit ... and asserts equality of result bits (NaN payload included), flags, outcome kind and the charge sequence with the independent reference'. The harnesses run a generous meter and assert a property subset: bits under `NearestEven`/`Exact`, NaN results, `invalid`, result units. The new Evidence paragraph says so, as Behavior 19 permits. TC-953 now describes two different harnesses. Make the procedure paragraph match the property-harness form and refer to the outputs list. | spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:31 |
| FND-002 | low | The provenance of the timeout peaks overstates what was measured. All 12 cluster at 22.7-23.3 GB (21.1-21.7 GiB), just under the scope's `MemoryMax=24G` with no swap. So the figure is bounded by the cap, not the harness's demand, and it includes the build and page cache. The 600 s `timeout` also wrapped the whole `cargo kani`, build included, so the solver had less than 600 s. The text mixes units ('22.7 to 23.3 GB ... just under the 24 GiB scope limit'). For the re-measured rows, give bytes and GiB consistently. Say when a peak reached the cap region (the outcome is then closer to memory exhaustion). State whether the 600 s includes the build. | spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:100, spec/functional/FR-405-kani-tractable-native-float-carrier.md:110 |
| FND-003 | low | Behavior 15 and TC-953 step 8 require a non-VERIFIED row to be recorded 'in TC-953's evidence record and in the Kani tractability table that IR-1526's seam loads'. TC-953 Status says the rows 'are not yet loaded into IR-1526's tractability table, which this repository does not own'. No ticket or owner is named for the load. AC-11 needs only the TC-953 record, so this does not block merge. But FR-405 cannot be Done under Behavior 15 until the load happens, and the rows will change (code-review FND-002). Name the load's owner or ticket in the Status, and load the re-measured rows. | spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:114, spec/functional/FR-405-kani-tractable-native-float-carrier.md:61 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-405 Status | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:93-110 | Implemented (IR-1446) ... Add \| table row: timeout \| table row: timeout ... No proof is claimed for a table-row combination: its harness reached no verdict within 600 s and its peak memory (22.7 to 23.3 GB) is above the 16 GiB ceiling. |
| TC-953 Evidence record | examined | spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:75-106 | Each row is an executed run of `cargo kani --harness native_float_kani_proofs::<id> --exact --unwind 57 --jobs 1 --solver cadical` ... with a 600 s `timeout` ... |
| TC-953 Status | examined | spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:112-114 | Run under IR-1446 ... The table rows are not yet loaded into IR-1526's tractability table, which this repository does not own ... |
| FR-405-AC-11 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:87 | ... either a Kani harness of Behavior 19 returns an executed terminal VERIFIED verdict, or a measured row exists whose outcome is one of timeout, memory exhaustion or unreliable ... A FAILED verdict (a counterexample) fails this criterion for that combination ... |
| FR-405 Behavior 21 | context_only | spec/functional/FR-405-kani-tractable-native-float-carrier.md:70 | Each harness SHALL run under a 10-minute wall-clock limit and return a terminal verdict within it, and its recorded peak memory SHALL be at most the 16 GiB per-request ceiling of CG FR-028 ... |
