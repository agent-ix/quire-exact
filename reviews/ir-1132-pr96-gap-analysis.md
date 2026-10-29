---
id: SR-2026102760
title: "Gap analysis of closed LimitKind reader"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@063c007cad695c4b3ab2ec3aed49ab93203b66b6; src/accounting.rs; FR-388; TC-937; IR-1132"
review_set: subset
---

## Summary

Ticket: IR-1132. Planless static gap analysis of PR #96. The computed matrix tags FR-388-AC-1 and AC-2 to the added tests; AC-4 is an inspection fulfilled by the reader's sole success path. AC-3 remains untagged pending the authoritative QSpec vector from IR-1131. Plan completion: not assessed.

## Verdict

**CONDITIONAL** — the QSpec vector and its independent equality/refusal evidence are still required for FR-388-AC-3. The shared build lane is occupied, so no test run evidence is bound to AC-1 or AC-2 yet.

## Examined scope

- `FR-388-AC-1` (examined): For every member of `LimitKind::ALL`, the public reader returns `Some(kind)` for `kind.as_str()`; each member maps back to itself, including the first and last members.
- `FR-388-AC-2` (examined): The public reader returns `None` for an empty string, an unknown name, a case-changed member name, a member name with leading or trailing whitespace, and a member name with a malformed separator; no input in this control set maps to another valid kind.
- `FR-388-AC-3` (examined): Against the authoritative QSpec `ScalarLimitsV1` full-vocabulary vector, `LimitKind::ALL` has exactly the same members in the same field order; each vector spelling is returned by `as_str()` and accepted by `from_code`, and at least one undeclared vector control is refused.
- `FR-388-AC-4` (examined): Inspection of every successful `from_code` return path shows that it returns `Some(kind)` only after the input exactly equals `kind.as_str()` for a member of `LimitKind::ALL`; there is no catch-all success path, normalization, alias or unchecked conversion.
- `TC-937` (examined): The kernel reads only declared scalar limit-kind spellings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | AC-3 has no test binding or run evidence: the QSpec full-vocabulary vector is pending IR-1131 | spec/functional/FR-388-read-closed-limit-kind-spellings.md:34 |

## Coverage

Plan completion: not assessed. `quoin matrix --repo . --json` reports AC-1/2 tagged with no run evidence, AC-3 untagged, and AC-4 `method-without-symbol` as an inspection criterion. No new reverse specification gap or stub appeared in the one-file diff. TC-937 steps 1, 2 and 4 have corresponding test or inspection evidence; step 3 awaits the authoritative QSpec vector. The residual is recorded as a blocker, not treated as satisfied by the existing local ten-name test in `tests/ir673_core.rs`.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | deferred | IR-1131 owns the authoritative QSpec full-vocabulary vector; AC-3 remains open and untagged, and IR-1132 must retain its blocker relation until that vector is consumed and tested. This disposition does not close AC-3 or the ticket. |
