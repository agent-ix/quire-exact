---
id: SR-11981
title: "IR-705 quire-exact PR 52 gap analysis: FR-359-AC-8/9/11/12 against TC-931 and tagged tests"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@9eab47cc66fa5fcc6a88ed3e38b4dbed01e8d60d; spec/functional/FR-359-cumulative-meter-boundary.md; spec/test-cases/TC-931-single-pass-meter-accounting.md; src/accounting.rs; src/accounting/kani_proofs.rs"
review_set: subset
---

## Summary

Ticket: IR-705. AC-to-test check for the FR-359 criteria this PR touches,
using `quire matrix` at head 9eab47cc and reading each tagged test.
Plan completion: not assessed.

- FR-359-AC-8 (Test): tagged by nine tests (src/accounting.rs:1133, 1152,
  1176, 1207, 1498, 1687, 1717, 1739, 1767). Covered: 7-then-9 and
  9-then-7 with `next_charge = 7`/`9` and `limit = 5`; field order in both
  attachment orders; request counts 0, 1, 2, 3, 64, 1000; reordered and
  repeated maxima including zero; refusal atomicity (counters, admissions,
  log, truncation); a pending injected occurrence left unspent; injected
  denial precedence (line 1498). `consumed()` for every kind is covered by
  the AC-7 tests.
- FR-359-AC-9 (Test): matrix shows `untagged`. The harness
  `meter_integer_arithmetic_schedule_matches_reference` has only the comment
  "FR-359-AC-9 is PLANNED on IR-844; this harness exhausts memory at 40 GiB."
  Nothing claims credit for AC-9. This is the honest state the owner ruled.
- FR-359-AC-11 (Inspection): `method-without-symbol`, as expected for
  Inspection. My own inspection agrees: the only semantic-size traversal in
  `Meter::charge` is `charge.sizes.iter()` (trip count = request count);
  `observe_size` is called eight times in straight-line code; `charge_plan`,
  `check_injected`, `incomplete` and `consumed` have no loops.
- FR-359-AC-12 (Test): tagged by `tc_931_admitted_log_matches_model_ordered_prefix`
  (line 1848) and `tc_931_admitted_log_matches_model_same_kind` (line 1901).
  Both compare the whole `admitted_charges()` slice with an independent
  fold, cover the absent third admission (result limit 0 against 1), an
  unchanged log on refusal, and same-kind zero and distinct amounts.
- FR-359-AC-10: external, deliberately untagged; not examined further.

## Verdict

PASS WITH FINDINGS (one low). AC-8, AC-11 and AC-12 are backed as claimed and
AC-9 is honestly unbacked. One AC-8 clause relies on an untagged test.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | FR-359-AC-8 says cancellation precedence remains unchanged, and TC-931 step 1 asks for a cancellation-order check on a separate meter. No `Trace: FR-359-AC-8` test covers cancellation. The existing `a_cancelled_handle_denies_the_next_charge_and_records_the_trip` passes, but it is not bound to AC-8, so the matrix does not show this clause covered. | src/accounting.rs:1286, spec/functional/FR-359-cumulative-meter-boundary.md:36 |
