---
id: TC-411
title: "A quantity UnitId is a declared unit's node key or a compound unit's digest, and the two never compare equal"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: verifies
---
# TC-411: A quantity UnitId is a declared unit's node key or a compound unit's digest, and the two never compare equal

## Description

Verify FR-088-AC-12. A kernel `UnitId` is a domain-labelled digest record over a declared unit's node key and a compound unit's digest. Equality is lexical on the domain, then the bytes.

Scope: FR-088-AC-12.

## Test Procedure

1. Build a declared `UnitId` and a compound `UnitId` over the same 32 bytes.
2. Compare them, and compare each with a `UnitId` of its own domain over other bytes.
3. Read each id's domain label.

## Expected Results

- Step 2: ids of different domains are unequal over the same bytes, and ids of one domain are unequal over different bytes.
- Step 3: the declared id carries `quire.checked-semantic-node/v1` and the compound id carries `quire.value.compound-unit/v1`.

## Status

Implemented. The test is `identity::tests::tc_411_unit_id_is_a_two_domain_record_compared_on_label_then_bytes`, tagged `#[trace("FR-088-AC-12", "TC-411")]`.
