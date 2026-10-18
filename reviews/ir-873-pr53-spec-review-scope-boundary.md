---
id: SR-12444
title: "Scope boundary analysis of quire-exact PR 53"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-exact; spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md; spec/functional/FR-368-meter-charge-and-limit-vocabulary.md; spec/test-cases/TC-910-exact-scalar-arithmetic-and-atom-charges.md; spec/test-cases/TC-916-meter-charge-and-limit-vocabulary.md"
review_set: subset
---

## Summary

Ticket IR-873, PR #53 at 31a07bc7f7ca2584d8529a53f71d2aebd9db7241. Quire-exact owns only the public Meter point and charge result. QSL retains traversal and the decision to count a visit.

## Examined scope

- **FR-362** (examined): For a caller-owned temporal operator visit, the public `Meter` SHALL admit or refuse a direct `temporal.visit` charge with the same atomic work-limit and named-point behavior as other charges. The caller owns the temporal formula traversal and decides when a visit occurs.
- **FR-362-AC-21** (examined): On a fresh public meter, a direct default charge at `temporal.visit` succeeds with `work_units = 1`, `result_units = 0` and no semantic-size request when the work limit is exactly 1. With work limit 0, the same call returns `Incomplete` with `limit_kind = WorkUnits`, `limit = consumed = 0`, `next_charge = 1` and `charge_point = TemporalVisit`; no charge is admitted and all counters remain zero.
- **FR-362-AC-22** (examined): A one-shot injected denial of the first direct `temporal.visit` charge returns `Incomplete` naming `TemporalVisit` and `WorkUnits`, with `limit = consumed = 0` and `next_charge = 1`; it admits no charge, changes no counter and consumes the one-shot denial, so a second identical charge on the same otherwise sufficient meter succeeds.
- **FR-368** (examined): The `temporal.visit` charge point follows `model.navigate` at the end of `ChargePoint::ALL`; the first 62 entries retain their existing order. It names a caller-owned temporal operator visit charged through the existing `WorkUnits` limit, not a new limit kind. Its spelling is distinct from the graph-walk points `graph.expand`, `graph.edge` and `graph.result-retain`.
- **FR-368-AC-1** (examined): Every public `ChargePoint` variant occurs exactly once in `ChargePoint::ALL`, has a unique documented spelling from `as_str()` and round-trips through `from_code()`; an unknown spelling returns `None`. Exhaustive variant matching makes an added but unaccounted variant fail the check.
- **FR-368-AC-2** (examined): Every public `LimitKind` variant occurs exactly once in `LimitKind::ALL`, has a unique documented `as_str()` spelling, and the sequence is exactly the ten `ScalarLimits` fields: integer bits, decimal digits, scale expansion, text input bytes, text scalars, normalized scalars, unit edges, value occurrences, work units and result units. Exhaustive variant matching makes an added but unaccounted variant fail the check.
- **TC-910** (examined): Verify [FR-362](../functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md), AC-1 through AC-22, through public kernel scalar and direct-meter entry points without evaluating a caller expression or copying a caller-owned schedule.
- **TC-916** (examined): Verify [FR-368](../functional/FR-368-meter-charge-and-limit-vocabulary.md) through the public `ChargePoint` and `LimitKind` enums. Scope: FR-368-AC-1 and FR-368-AC-2.

## Verdict

**PASS**. Quire-exact owns only the public Meter point and charge result. QSL retains traversal and the decision to count a visit.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Evidence

Four changed documents validate grammar-clean; full spec validates 79/79. At exact base 2134917 strict coverage is 165/192 backed, 27 unbacked, 0 contradicted; at head 31a07bc it is 165/194, 29 unbacked, 0 contradicted. Meter::charge and Charge::new were read for oracle coherence; no production code changed.
