---
id: TC-916
title: "Public meter charge and limit vocabularies"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-368
    type: verifies
---
# TC-916: Public meter charge and limit vocabularies

## Description

Verify [FR-368](../functional/FR-368-meter-charge-and-limit-vocabulary.md) through the public `ChargePoint` and `LimitKind` enums. Scope: FR-368-AC-1 and FR-368-AC-2.

## Test Procedure

1. For AC-1, exhaustively match every public charge-point variant against an independently authored name expectation; compare the set with `ChargePoint::ALL`, check uniqueness and each `from_code(as_str())`, and reject an unknown spelling.
2. For AC-2, exhaustively match every public limit-kind variant against the named ten-field sequence in `ScalarLimits`; compare with `LimitKind::ALL` and check name uniqueness. Include work before results at the end.

## Expected Results

Every enum variant occurs once, each spelling is unique and documented, charge points round-trip, unknown points refuse, and limit kinds follow the ten-field order. Adding an unaccounted enum variant fails the exhaustive match.

## Status

Planned for IR-673; no executable binder exists yet.
