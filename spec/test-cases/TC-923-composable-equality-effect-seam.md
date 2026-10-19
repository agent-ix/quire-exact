---
id: TC-923
title: "Check the checked equality result and its admitted pair effects"
type: TC
relationships:
  - target: "ix://agent-ix/quire-exact/FR-375"
    type: "verifies"
---
# TC-923: Check the checked equality result and its admitted pair effects

## Description

The RT end-to-end procedure formerly here is [RT TC-203](ix://agent-ix/quire-contract-runtime/TC-203), which verifies RT FR-277. The Kani claim is now RT FR-277-AC-7 (owner titan:kani:plan, PLANNED). This file retains only the pointer and the FR-375-AC-7 tool-boundary inspection.

## Test Procedure

1. For a proposed Kani dependency contract, run a focused passing control and a deliberately failing contract control through the actual RT-to-`quire-exact` compiled dependency edge. Record whether the installed tool detects the violation.

## Expected Results

No dependency contract is credited until the failing control is detected. Without that evidence, the real callee remains in the outer CBMC graph.
