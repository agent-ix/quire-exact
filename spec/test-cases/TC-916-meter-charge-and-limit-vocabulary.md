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

Verify [FR-368](../functional/FR-368-meter-charge-and-limit-vocabulary.md) through the public `ChargePoint` and `LimitKind` enums. Scope: FR-368-AC-1 through AC-3. AC-4 is an inspection of the kernel/caller ownership boundary.

## Test Procedure

1. For AC-1, exhaustively match every public charge-point variant against an independently authored name expectation; compare the set with `ChargePoint::ALL`, check uniqueness and each `from_code(as_str())`, and reject an unknown spelling. Check that `TemporalVisit` spells `temporal.visit` and is at index 62 after `ModelNavigate` at index 61, without changing entries 0 through 61.
2. For AC-2, exhaustively match every public limit-kind variant against the named ten-field sequence in `ScalarLimits`; compare with `LimitKind::ALL` and check name uniqueness. Include work before results at the end.
3. For AC-3, independently enumerate the pre-extension 63 charge-point identities and spellings, then require the appended `FrameWrite` and `FrameRedefinesLink` variants at indices 63 and 64 of an exactly 65-entry `ChargePoint::ALL`. Assert the exact two spellings, their `from_code()` round-trips, refusal of near-miss names, and unchanged first 63 entries. The exhaustive match from step 1 must fail to compile if a new variant lacks a name expectation.
4. For AC-4, inspect the kernel's public charge-point/meter API and the caller's operation-frame decision path. Identify which component detects writes, follows links, and charges its own frame meter. Confirm its `ModelNormalizationLimits.work_units` counter is independent of the kernel's `ScalarLimits.work_units`, with no new kernel frame traversal or shared counter.

## Expected Results

Every enum variant occurs once, each spelling is unique and documented, charge points round-trip, unknown points refuse, and limit kinds follow the ten-field order. The temporal-visit point remains after `model.navigate` and before the two ordered frame points. Adding an unaccounted enum variant fails the exhaustive match. The frame caller retains its own traversal, admission and work counter.

## Status

PR #9 adds executable Trace bindings for FR-368-AC-1 and AC-2. Run `quire matrix` for the current criterion-to-test mapping.

The frame names in AC-3 and boundary inspection in AC-4 are planned. The existing AC-1 test's 63-entry assertion must be updated together with the eventual code; this test case does not claim that new bindings already exist.
