---
id: TC-411
title: "Declared and compound unit identities remain distinct"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: verifies
---
# TC-411: Declared and compound unit identities remain distinct

## Description

Verify FR-088-AC-12. A kernel `UnitId` carries the admitted identity of a declared unit or compound unit with a distinct domain. Equality requires both the domain and the identity payload to match.

Scope: FR-088-AC-12.

## Test Procedure

1. Build a declared `UnitId` and a compound `UnitId` with matching admitted identity payloads, plus another identity within each domain.
2. Compare each cross-domain pair, each same-domain pair with matching payloads and each same-domain pair with differing payloads.
3. Read each id's domain.

## Expected Results

- Step 2: ids of different domains are unequal despite matching payloads; same-domain ids compare equal exactly when their identity payloads match.
- Step 3: declared and compound domains remain distinct.

## Status

The current test is `identity::tests::tc_411_unit_id_is_a_two_domain_record_compared_on_label_then_bytes`, tagged `#[trace("FR-088-AC-12", "TC-411")]`; update its construction to the admitted identity API with IR-1105/IR-1133.
