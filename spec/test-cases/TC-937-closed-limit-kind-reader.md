---
id: TC-937
title: "The kernel reads only declared scalar limit-kind spellings"
type: TC
relationships:
  - target: ix://agent-ix/quire-exact/FR-388
    type: verifies
---
# TC-937: The kernel reads only declared scalar limit-kind spellings

## Description

Verify the public `LimitKind::from_code` reader's round trip, refusal and
independent vocabulary completeness against FR-388-AC-1 through AC-3.

## Test Procedure

1. Iterate `LimitKind::ALL` and call `from_code(kind.as_str())` for each member.
2. Call `from_code` with an empty string, an unknown name, a case-changed
   member name, a member name with leading or trailing whitespace, and a
   member name with a malformed separator.
3. Read the authoritative `ScalarLimitsV1` full-vocabulary vector from QSpec.
   Compare its ordered member spellings with `LimitKind::ALL` mapped through
   `as_str()`. Read every declared spelling and at least one undeclared
   control through `from_code`.

## Expected Results

- Step 1: each spelling returns `Some` of its originating member.
- Step 2: every control returns `None`.
- Step 3: the ordered lists are equal, each declared spelling returns its
  corresponding kind, and each undeclared control returns `None`.

## Status

Specified. The QSpec vector is pending IR-1131.
