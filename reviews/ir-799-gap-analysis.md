---
id: SR-11155
title: "Gap analysis of IR-799 Decimal type equality"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact; spec/functional/FR-382-bounded-retained-decimal-type-equality.md; spec/test-cases/TC-930-bounded-retained-decimal-type-equality.md; src/numeric_type_equality.rs; ticket IR-799"
review_set: subset
---

## Summary

The computed Test Matrix tags all four FR-382 criteria to live tests. AC-2 has a partial oracle: one two-oversize fixture does not verify the full first-refusal priority.

## Verdict

**CONDITIONAL — One medium test coverage gap remains.**

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
| FND-001 | medium | Two-oversize fixtures verify only expected-lower before expected-upper; no fixture distinguishes expected-upper before actual-lower or actual-lower before actual-upper, so a reordered check could pass. | src/numeric_type_equality.rs:799 |

## Coverage

Plan completion: not assessed.

`quire matrix --scope . --format json` reports FR-382-AC-1 through AC-4 tagged to `src/numeric_type_equality.rs` tests at lines 691, 759, 838, and 872. Focused `cargo test --lib decimal_` passed 14 tests. The AC-2 test at lines 799-807 uses only the expected-lower/expected-upper oversized pair; the other ordering edges remain untested.
