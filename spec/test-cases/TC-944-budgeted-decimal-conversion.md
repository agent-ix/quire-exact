---
id: TC-944
title: "Admit decimal conversion work and storage before growth"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-390
    type: verifies
---
# TC-944: Admit decimal conversion work and storage before growth

## Description

Exercise the real `Integer::try_to_decimal_string_with_budget` conversion on
independently supplied zero, signed inline and multi-limb decimal literals.
Keep the source prepared before the conversion and preserve each caller's
typed denial cause.

## Test Procedure

1. Compare literal bytes with both the budgeted and existing API. Admit the
   exact measured cumulative storage and work, then refuse one-short output
   bytes independently of budget denial.
2. Record actual full-capacity scratch and output requests, then refuse each
   request by setting the cumulative limit to the preceding admitted prefix.
   Assert no later request, unchanged source, and a caller storage denial.
3. Refuse the scratch vector's first reservation through the real radix-limb
   append helper and assert its length and capacity remain zero. Refuse zero
   work through the public API and assert no storage request occurs.
4. Cancel after the first admitted conversion scratch at the next work
   callback. Assert the deadline cause, no completed text, and unchanged
   source. Repeat uncancelled through the same public budgeted path and check
   literal bytes and greater completed work. Refuse one-short completed work
   separately from storage and byte-bound failures.

## Expected Results

Literal bytes and legacy decimal refusal semantics are preserved. Every denied
growth is refused before requesting it; cancellation and work refusal retain
their caller cause. Tests use semantic scratch-admission events, not fixed poll
numbers, timeouts or allocation thresholds.

## Status

Planned. Runtime implementation and focused verification are pending.
