---
id: SR-17940486709564847047935
title: "IR-1445 PR #158 spec-review"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact PR #158 at cf32ace5, git diff origin/main...HEAD; IR-1445; spec/functional/FR-404-kani-tractable-native-text-carrier.md (Status; read against B5, B10, AC-6..AC-9), spec/test-cases/TC-952-kani-tractable-native-text-carrier.md (Evidence record, Status)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-404
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-952
    type: reviews
---

## Summary

Ticket: IR-1445. The spec delta has three parts. FR-404 Status changes from Planned to
"Implemented (IR-1445): C = 64". The TC-952 evidence record fills the two probe rows (VERIFIED
103.0 s / 3,796,357,120 B at C = 64; VERIFIED 49.2 s / 2,419,408,896 B at C = 32) and records
the FR-401 row as `NOT RUN` ("none: the FR-401 proof-path normalization is not implemented in
the kernel"). TC-952 Status claims "Executed for AC-1 through AC-6, AC-8 and AC-9" and gives AC-7
as not run.

Decision on question 3, `NOT RUN` against FR-404's own rules:

- B10 as a behavior is met. A missing record is "not VERIFIED", so the normalizing profiles are
  unsupported (`proof_path_supports` false), and no `TextType` is narrowed.
- AC-7 is not met. Its first sentence is the obligation: "The measurement of Behavior 10 is
  recorded, with the FR-401 normalization instantiated at the chosen C and run under Kani."
  "Otherwise they are excluded citing the record" presupposes that record. A `NOT RUN` row with
  harness id "none" is a missing record, not a measurement.
- AC-9 is not met. "Done only with AC-6, AC-7 and AC-8 each citing an executed run with a
  recorded outcome … an unexecuted harness does not satisfy it." Its one escape (neither
  candidate VERIFIED) is for AC-6, not AC-7. B5's "the recorded rows are the measured end state"
  also covers only the capacity probe.
- No measured AC-7 row can be produced in this PR. The FR-401 normalization does not exist (FR-401
  Status: "Planned (IR-1501). No Rust or test exists"), and a Kani run of the shipped
  `NormalizationForm::apply` would measure a different subject, which AC-7 does not accept.
- The owning ticket is closed. IR-1501 and IR-1499 are Done in Linear with only the spec PR #145
  attached. So AC-7, as recorded, is parked under a ticket nobody builds, which is exactly what
  FR-404 forbids.

So `NOT RUN` is acceptable as the recorded current state, with the profiles unsupported, but not as
a Done state. A measured probe is not required in this PR, because none is possible. What the PR
must change is listed under FND-001 and FND-002, and gap-analysis FND-001 covers the test tags.

## Findings

| ID | Severity | Summary | Refs |
|---|---|---|---|
| FND-001 | high | The PR claims a Done state that FR-404-AC-9 forbids. TC-952 Status says "Executed for … AC-8 and AC-9". FR-404 Status says "Implemented (IR-1445)" with no open criterion. But AC-7 has no executed record (the row is `NOT RUN`, harness "none"), and AC-9 makes Done conditional on AC-7 citing an executed run. Change TC-952 Status to "Executed for AC-1 to AC-6 and AC-8. AC-7 and AC-9 open: the FR-401 normalization is not in the kernel, so no Behavior 10 record exists; the normalizing profiles are unsupported on the proof path meanwhile (B10)". Change FR-404 Status to "Implemented except AC-7/AC-9 (open, owner <ticket>)". IR-1445 must not be closed on this merge. | spec/test-cases/TC-952-kani-tractable-native-text-carrier.md:69-71, spec/functional/FR-404-kani-tractable-native-text-carrier.md:74-76 |
| FND-002 | high | AC-7's measurement has no live owner. The `NOT RUN` row defers it to the FR-401 normalization, whose ticket IR-1501 is Done with spec only (PR #145), so no ticket builds FR-401 or runs the AC-7 measurement. Before merge, a live ticket must own the FR-401 implementation and the AC-7 Kani run at C = 64: reopen IR-1501, or file a new IR ticket, triaged by the code lead. It must block IR-1445's Done, and FR-404 Status and the TC-952 FR-401 row must name it, replacing "none: … not implemented in the kernel" with "NOT RUN, owner <ticket>". | spec/test-cases/TC-952-kani-tractable-native-text-carrier.md:63, spec/functional/FR-401-loop-free-proof-path-text-normalization.md:59-61 |
| FND-003 | low | The TC-952 evidence preamble reads "fits the 16 GiB ceiling" while the runs were capped at 24G. That is correct, since the peaks are 3.8 GB and 2.4 GB, but the record does not say the scope's `memory.peak` included the compile step in the same scope (TC-952 gives total wall "including the build"), so the peak is an upper bound and not the solver's peak. State which. The reviewer's rerun, with a cached build, read 3,850,371,072 B. | spec/test-cases/TC-952-kani-tractable-native-text-carrier.md:50 |

## Verdict

Not mergeable. The probe rows are real executed measurements and reproduce, and C = 64 correctly
follows B5 and AC-8. But the Status text claims AC-9, and so Done, while AC-7 has no record and no
live owner. FND-001 and FND-002 must be fixed in this PR, together with gap-analysis FND-001 (the
AC-7/AC-9 tags). The fix is text and tracker work only: no new Kani run is required, because AC-7
cannot be measured until FR-401 exists. FND-003 is low.
