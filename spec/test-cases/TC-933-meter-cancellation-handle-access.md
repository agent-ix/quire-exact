---
id: TC-933
title: "Meter exposes the handle it polls"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-383
    type: verifies
---
# TC-933: Meter exposes the handle it polls

## Description

Planned public-API tests for [FR-383](../functional/FR-383-meter-cancellation-handle-access.md).
Scope: FR-383-AC-1 through AC-3.

## Test Procedure

1. Construct a fresh meter without cancellation, then install one handle and
   replace it with an independent handle. Compare the accessor result with each
   handle using `Cancel`'s shared-state identity equality.
2. Through the accessor, cancel a meter's installed handle. Attempt a charge
   under sufficient limits and inspect the denial, `tripped()` cause, and
   unchanged counters and admission count. Repeat with only an independent
   handle cancelled and assert the charge is admitted.
3. Inspect the accessor implementation for a borrow of the meter field and no
   counter copy or state mutation. Compile the public call under `no_std` +
   `alloc`.

## Expected Results

The absent, installed and replacement handles satisfy FR-383-AC-1. Only the
installed handle controls the meter charge as FR-383-AC-2 states. Inspection
and compilation satisfy FR-383-AC-3.

## Status

PLANNED. Three criteria; zero implemented Trace bindings. The test matrix is
computed by `quire matrix`, not maintained in this file.
