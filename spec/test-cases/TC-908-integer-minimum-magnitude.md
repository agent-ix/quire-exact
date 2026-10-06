---
id: TC-908
title: "Absolute value of the minimum signed 64-bit integer"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-360
    type: verifies
---
# TC-908: Absolute value of the minimum signed 64-bit integer

## Description

Verify [FR-360](../functional/FR-360-integer-minimum-magnitude.md), AC-1, at `Integer::abs`.

## Test Procedure

Construct `Integer::from(i64::MIN)`, take its absolute value, and compare it with an independently constructed positive `2^63` value, its sign and the input.

## Expected Results

The magnitude is exactly positive `9,223,372,036,854,775,808`; the input remains negative and unequal to the magnitude.

## Status

Planned for the IR-653 CODE stage.
