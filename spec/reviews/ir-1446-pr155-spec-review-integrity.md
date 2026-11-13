---
id: SR-17940486709564847047756
title: "IR-1446 PR #155 FR-405 and TC-953 integrity analysis"
type: SpecReview
analysis: integrity
date: 2026-11-13
scope: "agent-ix/quire-exact PR #155 at 236b0ff0; IR-1446; spec/functional/FR-405-kani-tractable-native-float-carrier.md, spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-405
    type: reviews
  - target: ix://agent-ix/quire-exact/TC-953
    type: reviews
---

## Summary

Ticket: IR-1446. Structural pass: ids, relationships, verification labels, cross-references and computed matrix. `quire validate --scope .` passes on both files (one EARS warning, see the EARS SR). Every relationship target resolves (FR-402, FR-403, FR-394, FR-384 in this repo, and CG FR-053 and FR-028). TC-953 `verifies` FR-405. `quire matrix` lists all 13 FR-405 criteria: AC-1..AC-8 `untagged`, AC-9..AC-13 `method-without-symbol`. That is expected for a spec-only PR whose Status says 'Not implemented'.

## Verdict

Structurally sound. FR-405 and TC-953 are free ids. The gap left for FR-404/TC-952 (sibling PR #154, IR-1445) is a deliberate reservation, not a collision, and is acceptable. TC-953 steps map one-to-one to AC-1..AC-13. Four low findings: unnamed refusals, an undefined loop term, verification labels that do not match how the Kani-run criteria execute, and a category error in AC-2.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | Neither typed refusal is named: the out-of-width bit pattern of Behavior 1, and the out-of-scope Binary64 operand of Behavior 8. For Binary64, the FR also does not say where the refusal happens (the constructor of Behavior 1, which is total over `Binary64`, or the operations) or whether it charges the Meter. Two implementers could disagree, and CG's decoder needs a stable cause. Name the refusal variants and say that a Binary64 refusal, if any, comes before any charge. | spec/functional/FR-405-kani-tractable-native-float-carrier.md:53 |
| FND-002 | low | 'Data-dependent loop' is not defined. A `for _ in 0..24` with a data-dependent `break` (early-exit normalization) is a constant-bound loop with a data-dependent trip count. Under the lead's ruling, fixed-count constant-bound indexed loops are allowed. Say that a loop's bound is a stated constant and that it has no data-dependent exit, so AC-6's inspection has a definite rule. | spec/functional/FR-405-kani-tractable-native-float-carrier.md:49 |
| FND-003 | low | AC-9..AC-12 are labelled 'Inspection' but each needs a Kani run (the probe, the harnesses, the peaks). The matrix reports them `method-without-symbol`, so no binder will ever carry their executed verdict. AC-6 is the reverse: it is labelled 'Test' but is mostly source inspection. Label the Kani-run criteria with the method that executes them, as FR-402 does for its Kani rows, so the matrix can bind each to its run. | spec/functional/FR-405-kani-tractable-native-float-carrier.md:76 |
| FND-004 | low | AC-2 says there is 'no native entry for ... `requires_bound`'. `requires_bound` is a Contract IR lowering obligation (IR-69, CG FR-053-AC-3), not an IEEE operation, so the clause checks for something that could not exist. Move the IR-69 boundary to Description or Dependencies only. | spec/functional/FR-405-kani-tractable-native-float-carrier.md:69 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-405 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:24 | The kernel SHALL provide a native IEEE float carrier and native operations over it: a fixed-width, integer-only reimplementation of `evaluate_ieee` and `compare_ieee` (`src/ieee.rs`, the owners of IEEE arithmetic, flags and charges) that uses no arbitrary-precision integer, no `Integer`, no `Rati... |
| FR-405-B1 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:40 | 1. The native float SHALL hold a width tag (`Binary32` or `Binary64`) and the operand bit pattern in a `u64`, SHALL be `Copy`, SHALL have no drop glue and SHALL hold no owner `Integer`, `Rational`, `IeeeValue` or heap type. A total constructor from a width and a bit pattern SHALL exist; a bit pat... |
| FR-405-B7 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:49 | 7. The proof-path bodies SHALL use only fixed-width native integer arithmetic (shifts, masks, a widening multiply and a restoring or equivalent division in native width), SHALL perform no allocation, SHALL contain no data-dependent loop and no iterator adapter, SHALL NOT be selected by `cfg(kani)... |
| FR-405-B8 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:53 | 8. The required width is `Binary32`. `Binary64` addition and multiplication SHALL be in scope for this requirement only if a probe of them under Kani, with a 10-minute wall-clock limit per harness, reaches a terminal verdict of VERIFIED within that limit, recorded with its peak memory and tool ve... |
| FR-405-AC-2 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:69 | The native evaluation supports exactly Negate, Add, Subtract, Multiply and Divide and the native comparison exactly NumericEqual, TotalOrder and BitIdentical; a cross-width call is `ill_typed` and a `Meter` presented to it is unchanged; there is no native entry for square root, fused multiply-add... |
| FR-405-AC-6 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:73 | The proof-path bodies contain no `num_bigint` item, no owner `Integer`, `Rational` or `Vec`, no `cfg(kani)` token, no data-dependent loop and no iterator adapter, and any fixed-count step sequence is stated by its count; a counting allocator around the native operations over the boundary grid obs... |
| FR-405-AC-9 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:76 | The width set is explicit and evidenced: the 10-minute probe of `Binary64` addition and multiplication under Kani is executed and its terminal verdict, wall time, peak memory and tool version are recorded. `Binary64` is in scope only if both returned VERIFIED within the limit; otherwise `Binary64... |
| FR-405-AC-10 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:77 | For each in-scope width, a Kani harness per operation and per comparison proves the native operations over all operand bit patterns, all six rounding spellings and all meter limits against an independent expected side, each with an executed terminal VERIFIED verdict, a vacuity cover and a failing... |
| FR-405-AC-11 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:78 | The peak memory of every harness of AC-10 is recorded with its provenance and is within the K-DR memory budget; a configured ceiling or a prediction is not accepted as the record. |
| FR-405-AC-12 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:79 | CG's float row (CG FR-053) draws its operands through the carrier and its host differential (CG FR-053-AC-13) runs the native operations against the shipped owner with the boundary coverage and the mutants of AC-8; the composed harness's peak memory, time, tool version and terminal outcome are re... |
| FR-405-AC-13 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:80 | No proof is claimed for a width, operation or comparison whose AC-10 harness has no executed VERIFIED verdict, and the status records, per operation and width, the verdict or the absence of one. |
| TC-953 | examined | spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md:14 | Verify FR-405-AC-1 through FR-405-AC-13 by running the native float operations beside the shipped `evaluate_ieee` and `compare_ieee`, by inspecting the proof-path source, and by recording the Kani probe, verdicts and peaks. Each mutant below is a buildable variant of the named site in a scratch c... |
