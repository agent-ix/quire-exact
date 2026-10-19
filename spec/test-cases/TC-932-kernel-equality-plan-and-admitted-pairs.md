---
id: TC-932
title: "Report one kernel equality plan and only admitted pair charges"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-375
    type: verifies
---
# TC-932: Report one kernel equality plan and only admitted pair charges

## Description

Direct kernel controls for FR-375-AC-9 and AC-10. RT TC-203 owns checked-evaluator controls.

## Test Procedure

1. Build equal and first-head-unequal depth-four `CollectionKind::Sequence` operands with 16 visited occurrences each. Call `planned_equality_with_plan` on each with a fresh sufficient `Meter`, and compare its relation and pair count with `plan_equality` and the admitted charge log.
2. Deny the 17th pair charge. Record the returned stop and admitted prefix. Separately deny result retention after the pair schedule.

## Expected Results

Equal and unequal cases return `true` and `false`; each reports 17 planned pairs from one plan and admits 17 pair charges on success. The 17th-pair denial retains only 16 admitted pairs, no result retention and no completed relation. Result-retain denial preserves 17 admitted pairs and no completed relation.

## Status

IMPLEMENTED: direct kernel tests in `src/equality.rs` trace FR-375-AC-9 and FR-375-AC-10, including the admitted pair prefixes on refusal.
