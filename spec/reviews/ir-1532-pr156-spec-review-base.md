---
id: SR-17940486709564847047766
title: "IR-1532 PR #156 spec review (base)"
type: SpecReview
analysis: base
date: 2026-11-13
scope: "agent-ix/quire-exact; PR #156 @ ad0c0c85; IR-1532; spec/functional/FR-406-kernel-version-constant.md, spec/test-cases/TC-954-kernel-version-constant.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-406
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-954
    type: reviews
---

## Summary

Ticket: IR-1532. Spec-only PR #156 (branch spec/ir-1532-kernel-version, head ad0c0c85) adds FR-406 (public `KERNEL_VERSION: u32`, hand-bumped on semantic change, plus a golden outcome grid test) and TC-954. Scope is `git diff origin/main...HEAD` (two files). Checked against quire-exact `src/lib.rs` exports, `Cargo.toml` (`test-support` feature gates `Meter::admitted_charges`), `Makefile` (`build-no-std`), and the consumer quire-contract-codegen origin/main: `Cargo.toml` (quire-exact `branch = "main"`), FR-056 (kernel-version stale-row rule, merged via CG PR #811), FR-015-AC-42 (V2 operators), FR-053, and the Kani harness generators under `src/kani/` that call `quire_exact::Value`, `Integer`, `Meter`, `build_typed_prestate` and collection types. Ids FR-406 and TC-954 are free on origin/main and in the one other open PR (#141). `QSPEC_DIR=/home/peter/dev/quire-specification quire validate --scope . <both files>` exits 0 (one unrelated module-level `semantic.unused-import` warning). Linear IR-1532 read as data. No cargo, no edits.

## Verdict

FAIL (not mergeable): 3 high, 2 medium, 2 low.

What holds: AC-1 is testable (const item in an integration test, default and `test-support`, plus `make build-no-std`; the crate is unconditionally `#![no_std]`). The choice of a hand-bumped integer over a content hash is sound and introduces no build script, hash or SHA. The golden grid itself (outcomes plus ordered charges compared with recorded strings) is a genuine functional behaviour test: it fails when an operation's result changes, which is a real regression tripwire. The consumer can read the constant: CG depends on quire-exact by `branch = "main"` git dependency, `publish = false` does not affect a git dependency, and a `pub const` at the crate root is reachable as `quire_exact::KERNEL_VERSION`.

What fails: (1) the recorded-version-beside-the-grid check is vacuous: re-recording the golden strings without bumping leaves recorded version equal to `KERNEL_VERSION` and passes, which is the exact broken state it claims to catch; it is version-tracking ceremony and must be removed, with Behavior 6 stating honestly that the bump on re-record is reviewer-enforced. (2) AC-2 pins the constant to a literal in the test and scans source text, both antipatterns. (3) The bump rule exempts refactors and performance changes, but the named consumer's rows are Kani cost measurements (`wall_seconds`, `peak_bytes`, verdict) of harnesses that execute quire-exact code, and cost changes on exactly those changes; the FR must either widen the rule or state that limit for CG. The grid also misses operations CG's rows key on, and the FR relationship points at CG FR-053 instead of FR-056.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | Recorded-version check is vacuous: re-recording golden strings without a bump passes (recorded == KERNEL_VERSION unchanged); AC-4's "an outcome change without a matching bump cannot pass", Behavior 5's "therefore requires editing the recorded version" and TC-954 mutant 3 are false. Version-tracking ceremony; remove the check and state in Behavior 6 that the bump on re-record is reviewer-enforced | spec/functional/FR-406-kernel-version-constant.md:28, spec/functional/FR-406-kernel-version-constant.md:29, spec/functional/FR-406-kernel-version-constant.md:38, spec/test-cases/TC-954-kernel-version-constant.md:21, spec/test-cases/TC-954-kernel-version-constant.md:30 |
| FND-002 | high | AC-2 / TC-954 step 2 assert KERNEL_VERSION equals a literal recorded in the test (a version pin edited on every bump) and scan src/ and the repo for build.rs/env!/include_str!/git (a source-text scan); the cross-build equality cannot fail for a const literal. Remove AC-2 and mutant 4, or reduce to an inspection that the constant is an integer literal | spec/functional/FR-406-kernel-version-constant.md:36, spec/test-cases/TC-954-kernel-version-constant.md:19 |
| FND-003 | high | Bump rule (Behavior 3) exempts refactors and performance changes, but the named consumer (CG FR-056 rows: verdict, wall_seconds, peak_bytes of Kani probes whose harnesses run quire_exact Value/Integer/Meter/build_typed_prestate/collection code) goes stale on exactly those changes (the FR-402..405 carrier class); rows stay current after a cost-changing refactor. Widen the rule or state this limit for CG explicitly | spec/functional/FR-406-kernel-version-constant.md:16, spec/functional/FR-406-kernel-version-constant.md:26 |
| FND-004 | medium | Golden grid omits operations CG's tractability rows and V2 path key on: integer subtract, negate and the six comparisons; non-zero divide/modulo (only division by zero); decimal multiply (the measured-bad row); the native integer/decimal carriers; Value prestate construction. Behavior 5 family list and TC step 3 should name them | spec/functional/FR-406-kernel-version-constant.md:28, spec/functional/FR-406-kernel-version-constant.md:37, spec/test-cases/TC-954-kernel-version-constant.md:20 |
| FND-005 | medium | Relationship and Dependencies cite CG FR-053 (float scalar harnesses); the kernel-version stale-row consumer is CG FR-056 (merged). Retarget the references edge to FR-056 | spec/functional/FR-406-kernel-version-constant.md:7, spec/functional/FR-406-kernel-version-constant.md:48 |
| FND-006 | low | Ordered charges come from Meter::admitted_charges, available only under the test-support feature; AC-3/TC step 3 do not say the golden test runs with test-support, and TC step 2 runs it in "two feature sets" | spec/functional/FR-406-kernel-version-constant.md:37, spec/test-cases/TC-954-kernel-version-constant.md:20 |
| FND-007 | low | AC-5 requires the crate docs to state the bump rule, which no Behavior requires (Behavior 1 names only the doc comment); Behavior 4 (never decrease) has no AC | spec/functional/FR-406-kernel-version-constant.md:27, spec/functional/FR-406-kernel-version-constant.md:39 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-406 | examined | spec/functional/FR-406-kernel-version-constant.md:16 | The consumer is Contract CodeGen's measured Kani tractability table (IR-1526, kernel-version half of its stale-row rule). A row records the KERNEL_VERSION it was measured against; a row whose recorded value differs from the constant a build exports is stale. |
| FR-406-B3 | examined | spec/functional/FR-406-kernel-version-constant.md:26 | A change that can change the result of any public kernel operation for some input ... SHALL bump KERNEL_VERSION by at least one. A change that alters none of these (a refactor, a rename, a new operation or type that no existing operation uses, a performance change, a comment) does not require a bump. |
| FR-406-B5 | examined | spec/functional/FR-406-kernel-version-constant.md:28 | The test fails if any outcome differs or if the recorded version differs from KERNEL_VERSION. Updating the expected outcomes therefore requires editing the recorded version in the same change, and bumping the constant requires re-recording. |
| FR-406-B6 | examined | spec/functional/FR-406-kernel-version-constant.md:29 | A semantic change outside the grid is not detected by Behavior 5 and is covered by Behavior 3 alone; a reviewer, not the test, catches it. |
| FR-406-AC-1 | examined | spec/functional/FR-406-kernel-version-constant.md:35 | quire_exact::KERNEL_VERSION is a public u32 const reachable from the crate root in default, test-support and no_std builds, usable in a const context, and its doc comment states the bump rule. |
| FR-406-AC-2 | examined | spec/functional/FR-406-kernel-version-constant.md:36 | The value is identical across two separate builds of the same source and across the feature sets, and no build.rs, env!, option_env! or git lookup contributes to it. |
| FR-406-AC-3 | examined | spec/functional/FR-406-kernel-version-constant.md:37 | The golden test evaluates the grid of Behavior 5 and passes on the unchanged source. |
| FR-406-AC-4 | examined | spec/functional/FR-406-kernel-version-constant.md:38 | a fixture whose recorded version differs from KERNEL_VERSION makes the version check fail, so an outcome change without a matching bump cannot pass. |
| FR-406-AC-5 | examined | spec/functional/FR-406-kernel-version-constant.md:39 | The doc comment and the crate docs state that the constant is bumped by the author of a semantic change and name the change classes of Behavior 3 that do and do not require a bump. |
| TC-954 | examined | spec/test-cases/TC-954-kernel-version-constant.md:14 | Verify FR-406-AC-1 through FR-406-AC-5 through the public API, a golden outcome test and a source inspection. |
| CG FR-056 | context_only | quire-contract-codegen spec/kani/functional/FR-056-kani-tractability-refusal.md:86 | the Kani, CBMC and solver versions, and the generator and kernel version the probe ran against ... A row is current when its generator and kernel version equal CG's. |
