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

## Delta review at 01be6621

The confined IR-1105 delta from 58b98ea removes `rust-toolchain.toml` and the redundant `clippy.toml` `msrv = "1.98"` line. `Cargo.toml` still declares `rust-version = "1.98"`; `.github/workflows/ci.yml` still installs stable Rust, rustfmt, Clippy and `thumbv7em-none-eabi`, so the diff does not narrow the declared CI jobs or the no_std target. `git diff --check 58b98ea..HEAD` passed. No build or lint was run in this review.

The original static code verdict applies to the reader at this head. The config delta has the documentation finding below. Peter's no-docs scope instruction keeps the documentation correction separate; this artifact reports the mismatch rather than silently treating the prose as current.

**Delta verdict: CONDITIONAL** — the compiler configuration itself has no static defect; the documentation claim is stale and remains an IR-1105 residual.

## New findings (round 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | medium | README and CLAUDE.md still say the repository pins Rust 1.98.1 and the no_std target in the deleted rust-toolchain.toml | README.md:120; CLAUDE.md:24,27 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-002 | deferred | IR-1105 owns the stale compiler-pin prose. Peter's no-docs scope instruction excludes these docs from this PR; do not count the mismatch as corrected. The config delta otherwise preserves Cargo's numeric floor and the CI target installation. |
