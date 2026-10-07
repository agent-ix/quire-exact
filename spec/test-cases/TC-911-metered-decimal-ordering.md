---
id: TC-911
title: "Decimal ordering values, retained charges and analytic refusal"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-363
    type: verifies
---
# TC-911: Decimal ordering values, retained charges and analytic refusal

## Description

Verify [FR-363](../functional/FR-363-metered-decimal-ordering.md), AC-1 through AC-7, through `order_numbers` with decimal operands.

## Test Procedure

1. Construct decimal pairs whose exact values are equal but whose retained representations differ, plus positive, negative and zero pairs at different scales. Compare every ordering operator with an independent exact rational oracle.
2. On fresh meters with `test-support`, inspect charge order and all consumed counters for the retained `(0, 5)` versus `(0, 0)` and `(100, 2)` versus `(2, 0)` fixtures. Derive expected sizes from the FR's `B`, `D`, `SB` and `SD` definitions without calling kernel amount helpers.
3. With `decimal_digits = 64` and other limits at `u64::MAX`, test retained `(1, 0) < (1, u32::MAX)`; inspect the incomplete record, admitted points and result consumption.
4. On three fresh, generously limited meters, install an occurrence-1 one-shot denial at `ordering.operands`, `ordering.arithmetic`, and `ordering.result-retain` in turn. Compare retained `(0, 5) >= (0, 0)` each time; inspect the incomplete record, charge-log prefix, work and result counters, and absence of a completed Boolean.

5. For AC-6, check direct normalized comparison and equality APIs for `(110, 2)` and `(11, 1)`, keeping snapshots of their retained representations.
6. For AC-7, on separate fresh meters compare that pair and the narrow pair with itself; inspect all counters and the ordered points.

## Expected Results

Values agree with the independent mathematical oracle, charge points and counters equal the FR's amounts, and the large-scale case refuses at `ordering.arithmetic` for `DecimalDigits` with exact `next_charge = 4,294,967,296` after only the operand charge. Each injected denial names its selected point as a `WorkUnits` incomplete, logs only earlier admitted points, and leaves result units at zero. The normalized pair is equal and neither is less; retained representations remain distinct, with bits/digits/shift 8/3/1 for the mixed pair and 4/2/0 for the narrow pair.

## Status

PR #9 adds executable Trace bindings for FR-363-AC-6 and AC-7. Run `quire matrix` for the current criterion-to-test mapping.
