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

Verify [FR-358](../functional/FR-358-meter-denial-and-bounded-diagnostic-log.md) at the public meter seam. Scope: FR-358-AC-1 through FR-358-AC-6.

## Test Procedure

1. Install a denial for the second `FunctionCall` charge. Admit one `FunctionCall` and one unrelated charge; snapshot every counter, the admission count and the test-support log. Attempt the second `FunctionCall`, then attempt it again under generous limits.
2. On a fresh meter, install a denial for the first `EqualityPlan`; snapshot its state, call `charge_plan` with a valid pair count, and call it again.
3. Inspect the public `InjectedDenial` field type and construct an occurrence using `NonZeroU64::new(1)`; verify that `NonZeroU64::new(0)` yields `None`.
4. Under `test-support`, admit 4096 charges with distinguishable points near the boundary, inspect the log and truncation flag, then admit one more and inspect the log, flag, admission count and work units.
5. Without `test-support`, inspect `Counters` for fixed-size, heap-free state and absence of a charge log.

## Expected Results

1. The selected charge returns `Incomplete` at `FunctionCall` with `WorkUnits` as limit kind and preserves the snapshot; the retry succeeds and updates accounting and the log once.
2. The first plan charge returns `Incomplete` at `EqualityPlan` with the snapshot unchanged; the second succeeds and records the ordinary plan charge.
3. Zero cannot be represented as an `InjectedDenial::occurrence`; the nonzero value is accepted.
4. The log holds the first 4096 admissions in order and remains length 4096. The flag changes from false to true only after admission 4097, while the count and work units reach 4097.
5. Production `Counters` owns no heap allocation or admitted-charge log.

## Status

Planned.
