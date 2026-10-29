---
id: FR-388
title: "Read closed scalar limit-kind spellings"
type: FR
relationships: []
---
# FR-388: Read closed scalar limit-kind spellings

## Description

The `quire-exact` kernel SHALL expose a public `LimitKind::from_code(&str) ->
Option<LimitKind>` reader for the exact member spellings of QSpec's
`ScalarLimitsV1` in `quire.value.accounting/v1`. The reader gives a caller a
typed limit kind without inventing another spelling vocabulary.

## Behavior

1. For each `LimitKind`, `from_code(kind.as_str())` SHALL return `Some(kind)`.
2. For any string that is not an exact `ScalarLimitsV1` member spelling,
   `from_code` SHALL return `None`, including case changes, added whitespace,
   malformed separators, empty strings and unknown names. The reader SHALL
   NOT accept aliases.
3. `LimitKind::ALL` SHALL retain `ScalarLimitsV1` field order.
4. `as_str()` SHALL retain the corresponding normative spelling.
5. The accepted reader vocabulary SHALL equal those fields exactly, with no
   missing or extra name.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-388-AC-1 | For every member of `LimitKind::ALL`, the public reader returns `Some(kind)` for `kind.as_str()`; each member maps back to itself, including the first and last members. | Test |
| FR-388-AC-2 | The public reader returns `None` for an empty string, an unknown name, a case-changed member name, a member name with leading or trailing whitespace, and a member name with a malformed separator; no input in this control set maps to another valid kind. | Test |
| FR-388-AC-3 | Against the authoritative QSpec `ScalarLimitsV1` full-vocabulary vector, `LimitKind::ALL` has exactly the same members in the same field order; each vector spelling is returned by `as_str()` and accepted by `from_code`, and at least one undeclared vector control is refused. | Test |
| FR-388-AC-4 | Inspection of every successful `from_code` return path shows that it returns `Some(kind)` only after the input exactly equals `kind.as_str()` for a member of `LimitKind::ALL`; there is no catch-all success path, normalization, alias or unchecked conversion. | Inspection |

## Status

Specified. Implementation and independent full-vocabulary evidence are pending.

## Dependencies

- QSpec `quire.value.accounting/v1` owns the `ScalarLimitsV1` field vocabulary.
- IR-1131 supplies its authoritative full-vocabulary vector for
  FR-388-AC-3; this repository reads that vector from QSpec rather than
  copying it.
