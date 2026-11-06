---
id: TC-942
title: "Bounded quantity result retains its typed target on refusal"
type: TC
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-395
    type: verifies
---
# TC-942: Bounded quantity result retains its typed target on refusal

## Description

Verify FR-395-AC-1 through FR-395-AC-5 through the public kernel API.

## Test Procedure

1. Build a bounded result target over one `UnitId` with exact inclusive endpoints `-1/1` and `1/1`.
2. Evaluate the two exact endpoints and `2/1` and `-2/1` against the target.
3. Check a reversed `[5/1, 3/1]` target against `2/1`, `4/1`, and `6/1`, then deny the comparison charge and inspect the incomplete outcome.
4. Inspect the refusal's typed target, code and cause, and compare the old plain rational refusal.

## Expected Results

The endpoints complete in the target unit. `2/1` and `-2/1` refuse without values, retaining the unit and both endpoints, with code `rational_out_of_domain` and cause `outside-domain`. The reversed target retains its ordered endpoints and refuses every tested magnitude. A denied comparison charge returns an incomplete outcome. The plain rational refusal retains its existing target and code/cause.

## Status

Implemented.
