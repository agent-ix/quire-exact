---
id: SR-2026102759
title: "Code review of closed LimitKind reader"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@063c007cad695c4b3ab2ec3aed49ab93203b66b6; src/accounting.rs; IR-1132"
review_set: subset
---

## Summary

Ticket: IR-1132. Static Rust review of PR #96. The new reader searches the existing closed `LimitKind::ALL` and accepts only exact `as_str()` equality. The two new tests bind FR-388-AC-1 and AC-2. No active AssuranceProfile applies.

## Verdict

**PASS (static code review)** — no code defect found. Build, tests, Clippy, deny and the final aggregate gate were not run because the shared build lane is occupied.

## Examined scope

- `FR-388-AC-1` (examined): For every member of `LimitKind::ALL`, the public reader returns `Some(kind)` for `kind.as_str()`; each member maps back to itself, including the first and last members.
- `FR-388-AC-2` (examined): The public reader returns `None` for an empty string, an unknown name, a case-changed member name, a member name with leading or trailing whitespace, and a member name with a malformed separator; no input in this control set maps to another valid kind.
- `FR-388-AC-4` (examined): Inspection of every successful `from_code` return path shows that it returns `Some(kind)` only after the input exactly equals `kind.as_str()` for a member of `LimitKind::ALL`; there is no catch-all success path, normalization, alias or unchecked conversion.
- `FR-388-AC-3` (context_only): Against the authoritative QSpec `ScalarLimitsV1` full-vocabulary vector, `LimitKind::ALL` has exactly the same members in the same field order; each vector spelling is returned by `as_str()` and accepted by `from_code`, and at least one undeclared vector control is refused.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Evidence limits

`git diff --check origin/main...HEAD` passed. Source inspection proves the sole success path returns the matching member of `ALL`. The QSpec-owned vector is pending IR-1131; full-vocabulary equality is outside this code review's clean verdict.
