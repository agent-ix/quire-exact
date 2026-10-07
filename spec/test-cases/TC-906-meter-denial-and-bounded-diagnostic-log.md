---
id: TC-906
title: "One-shot meter denial and bounded diagnostic charge log"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-358
    type: verifies
---
# TC-906: One-shot meter denial and bounded diagnostic charge log

## Description

Verify [FR-358](../functional/FR-358-meter-denial-and-bounded-diagnostic-log.md) at the public meter seam. Scope: FR-358-AC-1 through FR-358-AC-7.

## Test Procedure

1. Set `integer_bits` limit to 8 and other limits generously, then install a denial for the second `FunctionCall` charge. Snapshot every counter, the admission count and the test-support log; attempt a `FunctionCall` charge with an `integer_bits` size of 9, and compare state to the snapshot. Admit a valid `FunctionCall` and one unrelated charge; snapshot again. Attempt the next valid `FunctionCall`, then retry it under the same limits.
2. On a fresh meter, install a denial for the first `EqualityPlan`; snapshot its state, call `charge_plan` with a valid pair count, and call it again.
3. Inspect the public `InjectedDenial` field type and construct an occurrence using `NonZeroU64::new(1)`; verify that `NonZeroU64::new(0)` yields `None`.
4. Under `test-support`, admit 4096 charges with distinguishable points near the boundary, inspect the log and truncation flag, then admit one more and inspect the log, flag, admission count and work units.
5. Without `test-support`, inspect production `Meter` for absence of an admitted-charge log and `Counters` for fixed-size, heap-free accounting state.

6. Repeat the post-cap admissions with one retained result unit each; after truncation test ordinary work exhaustion, ordinary result exhaustion and an injected named denial followed by an admissible retry. Compare every counter and the diagnostic state around each stop.
7. Check the public charge-point and limit-kind vocabularies exhaustively against their documented spellings and field order, including unknown charge-point rejection.

## Expected Results

1. The over-limit charge refuses without changing the first snapshot or advancing the named occurrence. The selected valid charge returns `Incomplete` at `FunctionCall` with `WorkUnits` as limit kind and preserves the second snapshot; its retry succeeds and updates accounting and the log once.
2. The first plan charge returns `Incomplete` at `EqualityPlan` with the snapshot unchanged; the second succeeds and records the ordinary plan charge.
3. Zero cannot be represented as an `InjectedDenial::occurrence`; the nonzero value is accepted.
4. The log holds the first 4096 admissions in order and remains length 4096. The flag changes from false to true only after admission 4097, while the count and work units reach 4097.
5. Production `Meter` contains no admitted-charge log; its `Counters` accounting state remains fixed-size and heap-free.

6. Results and work remain exact after the cap; limits and one-shot denial still enforce admission atomically, and the retry changes accounting without growing the log.
7. Every variant is represented once, spellings are unique and documented, charge points round-trip, unknown points refuse, and limit kinds follow field order.

## Status

Planned.
