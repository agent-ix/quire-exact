---
id: TC-909
title: "Denied kernel size charges precede large allocations"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-361
    type: verifies
---
# TC-909: Denied kernel size charges precede large allocations

## Description

Verify [FR-361](../functional/FR-361-admission-before-large-exact-work.md), AC-1 through AC-9, at the kernel's arithmetic, division and decimal entry points.

## Test Procedure

1. Install a test-only allocator observer with a safe API and keep the crate-wide unsafe-code lint in force. Construct every operand before opening a per-thread peak-request window around only the denied call.
2. With operands of at least 65,536 bits, deny integer multiplication and an arithmetic subtraction whose result is 1 at `integer-arithmetic.arithmetic`; deny a rational operation and a mathematically cancelling product at `rational-arithmetic.arithmetic`.
3. Inject the first denial at `integer-division.arithmetic` and then at `integer-modulus.arithmetic` for large nonzero operands.
4. With small decimal coefficients and a `2^20`-place shift, separately deny `decimal.scale-expansion`, `decimal.arithmetic` and `decimal.result-retain` under the limit and point fixtures named in AC-4 through AC-6.
5. In every case inspect the `Incomplete` record, admitted points and largest single allocation request made inside the window.

6. For AC-7, complete floor quotient division of a positive at-least-65,536-bit dividend by 3 at its exact operand-bit limit and compare with an independently calculated quotient and bit count.
7. For AC-8, lower that bit limit by one, measure only the stopped call and inspect its exact incomplete record, empty point log and allocation bound.
8. For AC-9, repeat result-retain denial with `Multiply` of `(1, 0)` by `(1, 0)` into the Exact scale-range target; inspect all five incomplete fields, the three-point prefix, zero result consumption and allocation bound.

## Expected Results

Each case stops at its stated charge point and limit kind with the exact requested amount. The largest request is below one eighth of the operand byte length for integer and rational arithmetic, division and modulus fixtures and below 4,096 bytes for all decimal shift fixtures. No later charge in the denied call is admitted. Exact-limit floor division completes; one-under denies operands with zero consumption. Multiply retention reports prior decimal digits 2 and requests `2^20 + 1` without allocating the retained power of ten.

## Status

AC-1 through AC-6 have pre-IR-673 tags. AC-7 through AC-9 are planned and untagged.
