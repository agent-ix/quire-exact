---
id: TC-913
title: "Six-mode decimal ties and strict default"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-365
    type: verifies
---
# TC-913: Six-mode decimal ties and strict default

## Description

Verify [FR-365](../functional/FR-365-decimal-rounding-ties-and-default.md) through the public owner API.

## Test Procedure

1. Construct the scale-zero target with each public rounding mode and round positive and negative halves on separate sufficient meters. Inspect the retained coefficient/scale or typed refusal and result counter.
2. Read the default mode and the target mode constructed with it, then round 0.1 under that target on a fresh meter.

## Expected Results

Results follow the six-mode tie table. Exact and the default refuse discarded nonzero digits without retaining a result.

## Status

PR #9 adds executable Trace bindings for FR-365-AC-1 and AC-2. Run `quire matrix` for the current criterion-to-test mapping.
