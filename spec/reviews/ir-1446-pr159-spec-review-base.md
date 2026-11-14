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

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | low | FR-405 Status undercounts the timed-out `NearestEven` harnesses. It says the table-row combinations' 'Exact and directed-spelling harnesses, and for Divide, SquareRoot and FusedMultiplyAdd also the NearestEven one, reached no verdict within the 600 s limit'. That enumerates 24 + 6 = 30 harnesses, but 31 timed out: `float64_multiply_nearest_even` (TC-953 row: timeout, 628924416 bytes) is missing. The next sentence (proofs held only for Add/Subtract both widths and Multiply Binary32) is correct, and TC-953 Status is correct, so no proof is overclaimed; the enumeration is just wrong. Add 'and Multiply at Binary64' to the NearestEven clause. | spec/functional/FR-405-kani-tractable-native-float-carrier.md (Status, paragraph after the table) |
| FND-005 | low | TC-953's provenance of `--unwind 3` is internally inconsistent. It says 'The first 19 runs of the table (the eight comparison and Negate harnesses and the Binary32 add, subtract, multiply and divide harnesses) also passed --unwind 3'. That parenthesis names 8 + 4 x 3 = 20 harnesses, not 19, and the evidence table is not in run order (Negate Binary32 is row 1, the Binary32 comparisons are rows 20-22), so 'first 19 runs of the table' does not identify the runs. A reader cannot tell which one harness of the 20 ran without `--unwind 3`. It does not change a verdict (Kani's unwinding assertions are on, every loop in those bodies is at most 2 or 3 iterations, the arity loop of a binary operation needs unwind 3, and the divide rows are timeouts), but the brief requires exact provenance per run. List the harness ids that carried `--unwind 3` (or fix the count). | spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:119 |

## Dispositions

Disposition pass 1, at 6986f4fd.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | The Test Procedure now describes the property-harness form and its covered and uncovered outputs, and step 8 the three per-spelling harnesses. After (TC-953:31): 'A Kani harness is a property harness (FR-405 Behavior 19, second sentence): per operation it draws the operands ... as fully symbolic bit patterns of the width, runs the native operation with a meter that never denies, and asserts the outputs listed in the harness docstring of `src/native_float/kani_proofs.rs` against an expected side that shares no function with the native body: result bits under `NearestEven` and a completed `Exact`, the NaN result (sign, quietness, payload) and the `invalid` flag ... The outputs it does not cover ... are left to the host differential.' |
| FND-002 | fixed | Every peak is now bytes plus GiB; the 600 s and the peak are both stated to include the build (about 5 s); exit status 124 for all 31 rows and none killed by the scope limit; the above-ceiling rows are marked per row and the 24 GiB scope limit is called a configured limit, not demand. After (TC-953:119): 'Peak memory is the scope's cgroup `memory.peak`, read inside the scope after the run; the scope held the whole `cargo kani` invocation, so it includes the build and is an upper bound on the solver's peak. The 600 s `timeout` also wraps the whole invocation, so it includes about 5 s of compilation ... The 31 non-VERIFIED rows all ended by that `timeout` (exit status 124); none was killed by the scope's memory limit ...' |
| FND-003 | fixed | FR-405 and TC-953 Status name IR-1526 as the owner of the load and of the table, and FR-405 stays Not Done until then. After (FR-405 Status): 'Not Done: the table rows below are for IR-1526 (the owner of the refusal seam and of the Kani tractability table) to load, and Behavior 15 is met in this repository by TC-953's evidence record.' |
| FND-004 | still-open | New this pass: FR-405 Status omits Multiply Binary64 from the timed-out NearestEven harnesses (30 enumerated of 31). One-clause fix. |
| FND-005 | still-open | New this pass: TC-953 says 'first 19 runs' carried `--unwind 3` but its parenthesis lists 20 harnesses and the table is not in run order. Name the harness ids. |
