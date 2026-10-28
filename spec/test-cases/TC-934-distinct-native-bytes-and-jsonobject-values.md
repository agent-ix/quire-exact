---
id: TC-934
title: "Distinct native Bytes and JsonObject values"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-385
    type: verifies
---
# TC-934: Distinct native Bytes and JsonObject values

## Description

Verify [FR-385](../functional/FR-385-distinct-native-bytes-and-jsonobject-values.md)
through the kernel's public type-admission, constructor, equality-plan,
canonical-key and accounting interfaces. Scope: FR-385-AC-1 through
FR-385-AC-7.

## Test Procedure

1. Construct each Bytes payload of FR-385-AC-2 and each JsonObject payload of
   FR-385-AC-3 exactly as printed there (literal characters, including U+10000
   and U+FFFF in the member names, and no added escapes), admit each constructed value against both native types, and
   offer a Text, Integer, Reference, UUID and Timestamp value to both.
2. Plan equality over the pairs of FR-385-AC-4, over each native against a
   value of another kind, and call `compare_keys` over the pairs of
   FR-385-AC-6 and over a mixed pair.
3. Read the variable size of Bytes `deadbeef` and the JsonObject `{"a":1}`,
   and re-run the existing UUID, Timestamp, scalar and composite vectors.

## Expected Results

- Step 1: the stated payloads construct, each other payload, including the
  escaped-string and UTF-16 member-order refusals, returns its
  stated cause with no value, and each value is admitted only by its own type.
- Step 2: the stated Boolean results, one terminal pair each; the mixed plan
  returns `ValueKindMismatch`, the mixed key comparison `None`, `02`
  sorts before `0a`, `0a` before `0a00`, and `0200` before `10`.
- Step 3: four and seven variable bytes, and every existing vector keeps its
  result.
