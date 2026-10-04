---
id: SR-001
title: "Code review of quire-exact PR #1: import quire-exact from quire-spec-language"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact@0992d0d198b0ef0dbd79f86849626bc0503b9b03; PR #1 diff origin/main...HEAD (Cargo.toml, Cargo.lock, Makefile, clippy.toml, deny.toml, rust-toolchain.toml, CLAUDE.md, .github/workflows/ci.yml, src/**, removed scripts/check_unsafe_comments.sh, scripts/unsafe_comment_baseline.txt, tests/integration.rs) plus the scaffold commit 08994ad (.gitignore, LICENSE, CLA.md, CONTENT_RIGHTS.md, .github/workflows/cla.yml); rust-review lane folded in"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-088
    type: reviews
---
# Code review of quire-exact PR #1

## Summary

Ticket: IR-582. Rust-review lane folded in.

Checked:

- `diff -r ~/dev/quire-spec-language/quire-exact/src src` against QSL main
  3dc4f52: every differing line on both sides is a comment line (`//`, `///`,
  `//!`) or blank. No code line, attribute or doc-test differs, and
  `src/value/value_type.rs` is byte-identical. All inline unit tests came
  along (QSL's crate has no `tests/` dir).
- `#![no_std]`, `#![forbid(unsafe_code)]` and `[lints.rust] unsafe_code =
  "forbid"`; no `unsafe` in src. Dependencies and features match QSL's
  `quire-exact/Cargo.toml`.
- `make ci` runs `build-no-std` (`cargo build --locked --target
  thumbv7em-none-eabi`), a thumbv7em clippy pass, and both feature lanes for
  clippy and test. The log at ~/dev/worktrees/logs/ir582-quire-exact-ci.log
  ends `exit=0`.
- Org files: LICENSE (AGPL-3.0), CLA.md, CONTENT_RIGHTS.md, cla.yml and
  `.agent/rules` are identical to agent-ix/quire-walk. `.gitignore` has
  unanchored `target/`, `*-target/`, `target-*/`, `.worktrees/`.
- History: `git log --all --name-only` lists 55 paths, all source, spec or
  config. No `target/`, `.rlib`, `.rmeta`, `.so` or other artifact ever
  entered the history. No `quire-research`, local path or `blob/` link in
  any commit message or any blob of any commit.
- Removing `scripts/check_unsafe_comments.sh` is right: `unsafe_code =
  "forbid"` makes the audit guard nothing.

## Verdict

Changes requested, no high. FND-001 makes the PR body's "no QSL, ADR or
ticket references" claim false and is a quick in-PR fix. FND-005 is the
copy this extraction leaves in QSL, closed by the QSL half of IR-582.
FND-002 to FND-004 are low; FND-003 and FND-004 need the owner's clearance
because they touch the workflow.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | QSL-internal references are left in the src doc comments, though the PR body says there are none. Six name QSL module paths (`quire_spec_language::value::text`'s own `Text::length` and so on), one cites QSL's `AD-005`, one names QSL's `quire-semantic-value` `ObjectClosure`, and three name QSL's `node-identity-preimage.schema.json`. Fix: reword them the way the rest of the scrub did ("a caller"). | src/text.rs:116,130,159,239; src/comparison.rs:52; src/numeric.rs:53; src/rational.rs:2; src/reference.rs:9; src/node.rs:6,47; src/location.rs:16 |
| FND-002 | low | Bare requirement ids in comments resolve to nothing in this repo and share its id space. QSL ids: FR-044, FR-107, FR-151, TC-196, TC-243, NFR-071, FR-272, FR-275-AC-5, FR-276, FR-089-AC-5. QSpec ids missing the `QSpec` prefix: FR-035, FR-140, FR-141, FR-142, FR-144, FR-147, FR-148, FR-149, TC-192. `quire coverage` reads four of them as trace tags on production functions (CR-061). Fix: prefix the QSpec ids, drop or reword the QSL ones. | src/accounting.rs:267-286,453,527,534-836; src/cancel.rs:20,63; src/comparison.rs:113-162; src/numeric.rs:39,247,313; src/value.rs:25,62,80,92,245,1271; src/collection.rs:378; src/identity.rs:255; src/ieee.rs:1585,1647 |
| FND-003 | low | `.github/workflows/ci.yml` does not run what `make ci` runs. It installs `thumbv7em-none-eabi` but never builds for it, skips the thumbv7em clippy pass and the `test-support` clippy and test lanes, and runs `cargo deny check licenses` where the Makefile runs a full `cargo deny check`. The no_std build is covered only by local `make ci`. Fix: have the workflow run `make ci`. A workflow edit needs the owner's clearance first. | .github/workflows/ci.yml:13-34,52-53; Makefile:36-51,66-67,88 |
| FND-004 | low | The PR edits `.github/workflows/ci.yml`: it deletes the "Unsafe audit" step and adds the thumbv7em target. The edit is needed, because the script is deleted, but workflow edits need the owner's clearance first. Confirm that clearance before merge, or keep the edit out of this PR. | .github/workflows/ci.yml:17,28-30 |
| FND-005 | medium | The extraction leaves two copies of the crate. QSL main still carries `quire-exact/` (the same code), and QSL main is still changing it (last change 2026-10-04 01:57 -0700). Until QSL depends on agent-ix/quire-exact by git and deletes `quire-exact/`, the two drift. Any QSL change to the crate after this import must be ported here before the switch. Fix: the QSL half of IR-582. The copy is gone only when QSL's `quire-exact/` path 404s. | src/; ~/dev/quire-spec-language/quire-exact/ |
