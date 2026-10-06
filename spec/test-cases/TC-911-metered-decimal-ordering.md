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

Verify [FR-363](../functional/FR-363-metered-decimal-ordering.md), AC-1 through AC-5, through `order_numbers` with decimal operands.

## Test Procedure

1. Construct decimal pairs whose exact values are equal but whose retained representations differ, plus positive, negative and zero pairs at different scales. Compare every ordering operator with an independent exact rational oracle.
2. On fresh meters with `test-support`, inspect charge order and all consumed counters for the retained `(0, 5)` versus `(0, 0)` and `(100, 2)` versus `(2, 0)` fixtures. Derive expected sizes from the FR's `B`, `D`, `SB` and `SD` definitions without calling kernel amount helpers.
3. With `decimal_digits = 64` and other limits at `u64::MAX`, test retained `(1, 0) < (1, u32::MAX)`; inspect the incomplete record, admitted points and result consumption.
4. On three fresh, generously limited meters, install an occurrence-1 one-shot denial at `ordering.operands`, `ordering.arithmetic`, and `ordering.result-retain` in turn. Compare retained `(0, 5) >= (0, 0)` each time; inspect the incomplete record, charge-log prefix, work and result counters, and absence of a completed Boolean.

## Expected Results

Values agree with the independent mathematical oracle, charge points and counters equal the FR's amounts, and the large-scale case refuses at `ordering.arithmetic` for `DecimalDigits` with exact `next_charge = 4,294,967,296` after only the operand charge. Each injected denial names its selected point as a `WorkUnits` incomplete, logs only earlier admitted points, and leaves result units at zero.

## Status

Planned for the IR-653 CODE stage.
