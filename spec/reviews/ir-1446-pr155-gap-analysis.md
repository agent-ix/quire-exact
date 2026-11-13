---
id: SR-17940486709564847047758
title: "IR-1446 PR #155 gap-analysis"
type: SpecReview
analysis: gap-analysis
date: 2026-11-13
scope: "agent-ix/quire-exact PR #155 at 236b0ff0; IR-1446; spec/functional/FR-405-kani-tractable-native-float-carrier.md, spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-405
    type: reviews
---

## Summary

Ticket: IR-1446. Planless gap analysis of the change set. Plan completion: not assessed. The diff touches only `spec/` (FR-405, TC-953); no production code or test changed. Matrix (`quire matrix --scope .`): FR-405-AC-1..AC-8 `untagged`, AC-9..AC-13 `method-without-symbol`. This is consistent with FR-405 Status ('Not implemented; no Rust accompanies this requirement') and TC-953 Status ('Not run'). No code was added without an owning requirement. No stub or coverage inflation is possible, since no test is tagged. Upstream need versus spec: CG FR-053 P1 asks IR-1446 for binary32 and binary64 and for sqrt/fma. FR-405 covers binary32 and conditional binary64, and declines sqrt/fma. The resulting ownership gap is recorded in the base SR (FND-001, FND-003) and not repeated here.

## Verdict

No coverage gap is attributable to this PR. Every criterion is untagged because the PR delivers a spec, not an implementation, and the Status says so honestly.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-405-AC-1 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:68 | The carrier is `Copy`, has no drop glue and holds no owner or heap type; the constructor refuses a `Binary32` pattern above `u32::MAX` (for example `1 << 32`) and never masks it to `0`; `0.0` and `-0.0`, and two NaN payloads, are unequal under structural equality; the conversions to and from `Iee... |
| FR-405-AC-2 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:69 | The native evaluation supports exactly Negate, Add, Subtract, Multiply and Divide and the native comparison exactly NumericEqual, TotalOrder and BitIdentical; a cross-width call is `ill_typed` and a `Meter` presented to it is unchanged; there is no native entry for square root, fused multiply-add... |
| FR-405-AC-3 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:70 | For each of the five rounding modes and `Exact`, the native result bits equal the owner's on the tie and one-ulp-off-tie operands of each mode, including the overflow result of each mode (infinity or the largest finite), the exact-zero sum sign, and subnormal results; the `Exact` refusal carries ... |
| FR-405-AC-4 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:71 | The flag set equals the owner's on the whole boundary grid, including `invalid` from a NaN-producing operation, `divide_by_zero` from a finite nonzero over zero, `overflow`, `underflow` and `inexact`; and the NaN result equals the owner's in sign, quietness and payload. |
| FR-405-AC-5 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:72 | The charge points, their order, limit kinds and amounts, the denied charge and the incomplete record equal the owner's at a sufficient, an exact and a one-less limit for every boundary case; an ill-typed call charges nothing. |
| FR-405-AC-6 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:73 | The proof-path bodies contain no `num_bigint` item, no owner `Integer`, `Rational` or `Vec`, no `cfg(kani)` token, no data-dependent loop and no iterator adapter, and any fixed-count step sequence is stated by its count; a counting allocator around the native operations over the boundary grid obs... |
| FR-405-AC-7 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:74 | The native operations are callable in a host build (a host test calls each), are one source for the host and `cfg(kani)` builds, and are a separate implementation from the owner: no native body calls `evaluate_ieee`, `compare_ieee` or any `Rational`-based owner helper, and the expected side of ev... |
| FR-405-AC-8 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:75 | The host differential of Behavior 9 passes on the full boundary grid, each of the six rounding spellings and each operation, and the mutants of Behavior 10 each fail the differential at the step TC-953 names; a mutant with no distinguishing input is rejected. |
| FR-405-AC-9 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:76 | The width set is explicit and evidenced: the 10-minute probe of `Binary64` addition and multiplication under Kani is executed and its terminal verdict, wall time, peak memory and tool version are recorded. `Binary64` is in scope only if both returned VERIFIED within the limit; otherwise `Binary64... |
| FR-405-AC-10 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:77 | For each in-scope width, a Kani harness per operation and per comparison proves the native operations over all operand bit patterns, all six rounding spellings and all meter limits against an independent expected side, each with an executed terminal VERIFIED verdict, a vacuity cover and a failing... |
| FR-405-AC-11 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:78 | The peak memory of every harness of AC-10 is recorded with its provenance and is within the K-DR memory budget; a configured ceiling or a prediction is not accepted as the record. |
| FR-405-AC-12 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:79 | CG's float row (CG FR-053) draws its operands through the carrier and its host differential (CG FR-053-AC-13) runs the native operations against the shipped owner with the boundary coverage and the mutants of AC-8; the composed harness's peak memory, time, tool version and terminal outcome are re... |
| FR-405-AC-13 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:80 | No proof is claimed for a width, operation or comparison whose AC-10 harness has no executed VERIFIED verdict, and the status records, per operation and width, the verdict or the absence of one. |
