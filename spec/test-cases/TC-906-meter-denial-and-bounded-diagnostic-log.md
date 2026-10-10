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

Verify [FR-358](../functional/FR-358-meter-denial-and-bounded-diagnostic-log.md) at the public meter seam. Scope: FR-358-AC-1 through AC-6 and AC-8 through AC-13. AC-7 was moved to [FR-368](../functional/FR-368-meter-charge-and-limit-vocabulary.md).

## Test Procedure

1. Set `integer_bits` limit to 8 and other limits generously, then install a denial for the second `FunctionCall` charge. Snapshot every counter, the admission count and the test-support log; attempt a `FunctionCall` charge with an `integer_bits` size of 9, and compare state to the snapshot. Admit a valid `FunctionCall` and one unrelated charge; snapshot again. Attempt the next valid `FunctionCall`, then retry it under the same limits.
2. On a fresh meter, install a denial for the first `EqualityPlan`; snapshot its state, call `charge_plan` with a valid pair count, and call it again.
3. Inspect the public `InjectedDenial` field type and construct an occurrence using `NonZeroU64::new(1)`; verify that `NonZeroU64::new(0)` yields `None`.
4. Under `test-support`, admit 4096 charges with distinguishable points near the boundary, inspect the log and truncation flag, then admit one more and inspect the log, flag, admission count and work units.
5. Without `test-support`, inspect production `Meter` for absence of an admitted-charge log and `Counters` for fixed-size, heap-free accounting state.

6. For AC-8, repeat all 4097 admissions with one result unit each; read both cumulative counters and the bounded log.
7. For AC-9 and AC-10, construct separate meters with work and result limits exactly 4097, admit 4097 matching charges, then attempt one more matching unit and compare every counter and diagnostic field to the snapshot.
8. For AC-11, after 4097 admissions inject an occurrence-1 named denial, compare state before/after its refusal, then retry with ordinary limits sufficient.
9. For AC-12, on each of two fresh meters with work limits 10 and 100 and all other limits sufficient, admit two work units at an unrelated point, install an occurrence-1 denial at P, and request a three-work-unit `charge` at P. Compare every `Incomplete` field and the pre-refusal state across the two limits. Repeat on two fresh meters after admitting two work units, this time denying `charge_plan` at `equality.plan` with `pairs = 2`; compare its recorded reservation with the one work unit a successful plan would commit.
10. For AC-13, first make a selected `charge` with a semantic size greater than its configured limit and independently confirm that the ordinary charge would refuse at that size counter, with other limits sufficient. Then inject occurrence 1 at the same point and repeat the charge. Separately, after admitting two work units under a configured work limit of 3 and otherwise sufficient limits, confirm that `charge_plan` with `pairs = 2` would refuse its four-unit work reservation, then inject occurrence 1 at `equality.plan` and repeat. Compare each returned record and every consumed counter and admission count to their pre-refusal values; keep cancellation inactive in both cases.

## Expected Results

1. The over-limit charge refuses without changing the first snapshot or advancing the named occurrence. The selected valid charge returns `Incomplete` at `FunctionCall` with `WorkUnits` as limit kind and preserves the second snapshot; its retry succeeds and updates accounting and the log once.
2. The first plan charge returns `Incomplete` at `EqualityPlan` with the snapshot unchanged; the second succeeds and records the ordinary plan charge.
3. Zero cannot be represented as an `InjectedDenial::occurrence`; the nonzero value is accepted.
4. The log holds the first 4096 admissions in order and remains length 4096. The flag changes from false to true only after admission 4097, while the count and work units reach 4097.
5. Production `Meter` contains no admitted-charge log; its `Counters` accounting state remains fixed-size and heap-free.

6. Both cumulative counters and admissions reach 4097 while the ordered diagnostic prefix stays at 4096.
7. Each ordinary shortage reports its own limit kind and leaves all admitted state unchanged.
8. The injected refusal is atomic and spent; retry updates exact accounting once without extending the prefix.
9. The ordinary-point refusals under work limits 10 and 100 are identical: `WorkUnits`, point P, `limit = consumed = 2`, and `next_charge = 3`. Both plan refusals are `WorkUnits` at `equality.plan` with `limit = consumed = 2` and `next_charge = 4` (`pairs + 2`), rather than the plan's successful one-unit work commit. Neither configured limit appears in an injected record, and none of the refusals changes admitted state.
10. The selected semantic-size charge returns its injected `WorkUnits` record rather than the otherwise first short size counter. The selected plan returns its injected `WorkUnits` record with `limit = consumed = 2` and `next_charge = 4`, rather than the ordinary work-refusal record whose `limit` would be the configured 3. No counter or admission count changes. This result does not assert precedence over cancellation.

## Status

PR #9 adds executable Trace bindings for FR-358-AC-8 through AC-11. Run `quire matrix` for the current criterion-to-test mapping.
FR-358-AC-12 and AC-13 are specified here; their executable Trace bindings are follow-up work.
