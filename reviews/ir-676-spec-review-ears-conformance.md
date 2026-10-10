---
id: SR-5044
title: EARS review of IR-676 injected denial specification
type: SpecReview
analysis: ears-conformance
scope: agent-ix/quire-exact@cc99c297ee2727aeb26ba4ce67ebc6ec208cf0a0; spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md;
  spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md
review_set: subset
---

## Summary

Ticket: IR-676. The new behavior statements have a named subject, one shall each, and concrete observable responses; scoped Quire grammar validation reported zero findings.

## Verdict

**PASS** — No findings in this method.

## Examined scope

- FR-358-AC-12 (examined, `spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md`): After admitting two work units, inject occurrence 1 at point P and attempt a three-work-unit `charge` at P under otherwise sufficient limits whose configured work limits are 10 and 100 on separate meters. Both refusals report exactly `limit_kind = WorkUnits`, `charge_point = P`, `limit = consumed = 2`, and `next_charge = 3`, independently of either configured limit. Repeat with an injected `equality.plan` denial, `pairs = 2`, the same pre-consumed work and limits, and sufficient value-occurrence and result limits: both `charge_plan` refusals report `WorkUnits`, `charge_point = equality.plan`, `limit = consumed = 2`, and `next_charge = 4` (`pairs + 2`), rather than the one work unit a successful plan would commit.
- FR-358-AC-13 (examined, `spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md`): With an occurrence-1 injected denial selected, make the same `charge` independently fail an ordinary semantic-size counter and make a `charge_plan` with `pairs = 2` independently fail its work reservation under a configured work limit of 3 after two admitted work units, with all other limits sufficient. In each case, the injected `WorkUnits` record is returned before the ordinary refusal: `limit = consumed` equals the pre-charge work, `next_charge` is the charge's work amount or the plan's reservation respectively, and every consumed counter and admission count remain unchanged. The criterion does not override cancellation, which is checked before injection.
- TC-906 (examined, `spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md`): For AC-12, on each of two fresh meters with work limits 10 and 100 and all other limits sufficient, admit two work units at an unrelated point, install an occurrence-1 denial at P, and request a three-work-unit `charge` at P. Compare every `Incomplete` field and the pre-refusal state across the two limits. Repeat on two fresh meters after admitting two work units, this time denying `charge_plan` at `equality.plan` with `pairs = 2`; compare its recorded reservation with the one work unit a successful plan would commit.
- TC-906 (examined, `spec/test-cases/TC-906-meter-denial-and-bounded-diagnostic-log.md`): For AC-13, first make a selected `charge` with a semantic size greater than its configured limit and independently confirm that the ordinary charge would refuse at that size counter, with other limits sufficient. Then inject occurrence 1 at the same point and repeat the charge. Separately, after admitting two work units under a configured work limit of 3 and otherwise sufficient limits, confirm that `charge_plan` with `pairs = 2` would refuse its four-unit work reservation, then inject occurrence 1 at `equality.plan` and repeat. Compare each returned record and every consumed counter and admission count to their pre-refusal values; keep cancellation inactive in both cases.
- FR-358 (examined, `spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md`): An injected denial shall report `Incomplete { limit_kind: WorkUnits, limit: w, consumed: w, next_charge: n, charge_point: P }`, where `w` is the admitted work consumed immediately before the denied charge and `P` is the selected point. The record shall be independent of the configured `ScalarLimits.work_units`: `n` is the denied charge's own work amount for `charge`, or the `pairs + 2` availability reservation for `charge_plan` at `equality.plan`. A successful `charge_plan` commits one work unit, but its injected denial reports the reservation it checked, not that eventual commit.
- FR-358 (examined, `spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md`): At the selected occurrence, an injected denial shall take precedence over an ordinary `ScalarLimits` counter refusal of that same charge, including a plan reservation that exceeds the remaining work. An active cancellation is checked before injection and is outside this ordinary-limit precedence rule.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
