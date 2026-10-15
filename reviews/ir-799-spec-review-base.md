---
id: SR-11156
title: "Base spec review of IR-799 Decimal type equality"
type: SpecReview
analysis: base
scope: "agent-ix/quire-exact; spec/functional/FR-382-bounded-retained-decimal-type-equality.md; spec/test-cases/TC-930-bounded-retained-decimal-type-equality.md; src/numeric_type_equality.rs; ticket IR-799"
review_set: subset
---

## Summary

FR-382 and TC-930 state the bounded borrowed identity contract, typed size and cancellation stops, and measurable work limit. The specified order and boundary behavior are internally consistent.

## Verdict

**PASS — No specification defect found.**

## Scope examined

- `FR-382` (examined): When a supplied-admission caller compares two retained Decimal types as type links, `quire-exact` SHALL offer fallible equality under that caller's authenticated finite magnitude ceiling `B`.
- `FR-382-AC-1` (examined): Equal distinct Decimal types complete `true`; a difference in either signed endpoint, either scale or rounding mode completes `false` under authenticated `B`. Endpoint equality is mathematical integer equality. Borrowed inputs remain unchanged.
- `FR-382-AC-2` (examined): At `B >= 1`, every endpoint position completes at exact `B` and returns typed `UnsupportedSize` with the correct position, measured `B+1` bits and `B` when one bit over. Two oversized endpoints report the first in expected-lower, expected-upper, actual-lower, actual-upper order, even if an earlier retained field differs. At `B = 0`, zero is refused at expected lower with measured one bit.
- `FR-382-AC-3` (examined): Observed production comparisons at inline/Large and 32-bit limb boundaries perform at most `2 ceil(B/32)` limb-pair steps of fixed-width work and no input-sized clone or allocation; auxiliary storage is constant. Source inspection finds no hidden unbounded fallback or reachable storage/capacity failure.
- `FR-382-AC-4` (examined): A pre-cancelled original handle stops before bound validation, including when expected lower exceeds `B`; cancellation observed after a late limb step stops before another step or Boolean publication with the original cause. A fresh handle completes the same operands.
- `TC-930` (examined): Verify FR-382 through the production borrowed `DecimalType::try_eq_bounded` operation; procedures 1-4 exercise retained equality, size order, work/storage, and cancellation.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |
