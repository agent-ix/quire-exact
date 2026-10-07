---
id: TC-915
title: "Distinct text profiles refuse before any charge"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-367
    type: verifies
---
# TC-915: Distinct text profiles refuse before any charge

## Description

Verify [FR-367](../functional/FR-367-text-profile-mismatch-before-charging.md) through the public owner API.

## Test Procedure

Admit "a" independently as Nfc and BinaryUtf8. Snapshot a fresh comparison meter with all limits zero, then call public compare_text with Equal. Inspect the exact IllTyped cause, all ten consumed counters, admissions and test-support log state.

## Expected Results

The result is DistinctTextProfiles before any charge or incomplete outcome; the meter equals its fresh snapshot.

## Status

Planned for IR-673; executable evidence has not yet been added.
