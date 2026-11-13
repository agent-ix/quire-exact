---
id: SR-17940486709564847047757
title: "IR-1446 PR #155 FR-405 EARS analysis"
type: SpecReview
analysis: ears-conformance
date: 2026-11-13
scope: "agent-ix/quire-exact PR #155 at 236b0ff0; IR-1446; spec/functional/FR-405-kani-tractable-native-float-carrier.md, spec/test-cases/TC-953-native-float-carrier-matches-owner-and-verifies.md"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-405
    type: reviews
---

## Summary

Ticket: IR-1446. EARS pass over FR-405's Description, the separation paragraph and Behavior 1..14. Patterns used: ubiquitous (most items), event-driven (B13), unwanted-behaviour (B8 'If ... then'), state-driven (B14 'Until').

## Verdict

Generally conformant. B8 uses a correct unwanted-behaviour form, and B14 uses a state-driven form. Three low findings: two compound statements (one flagged by `quire validate`) and one event-driven item whose actor is outside the system.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The 'Separation of implementations' paragraph holds three SHALL clauses (separate from the owner and from the expected side; ordinary host code; not cfg(kani)-only). `quire validate` warns `ears:non-singular` at line 28. Split it into numbered Behavior items, one SHALL each, so AC-7 maps to each. | spec/functional/FR-405-kani-tractable-native-float-carrier.md:28 |
| FND-002 | low | Behavior 1 holds five obligations: representation, Copy and no drop glue, no owner or heap type, a total constructor with typed refusal, and structural equality. AC-1 bundles them in the same way, so one failing clause fails the whole AC without saying which. Split them, or number the clauses so TC-953 step 1 can report each. | spec/functional/FR-405-kani-tractable-native-float-carrier.md:40 |
| FND-003 | low | Behavior 13's event-driven clause ('When CG's float row ... draws its operands through the carrier, ... SHALL be recorded') has no system of interest as its subject. The actor is CG, not the kernel, so it is not a requirement on this system. This is part of base FND-002. | spec/functional/FR-405-kani-tractable-native-float-carrier.md:61 |

## Scope

| Unit | Role | Location | Excerpt |
| --- | --- | --- | --- |
| FR-405 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:24 | The kernel SHALL provide a native IEEE float carrier and native operations over it: a fixed-width, integer-only reimplementation of `evaluate_ieee` and `compare_ieee` (`src/ieee.rs`, the owners of IEEE arithmetic, flags and charges) that uses no arbitrary-precision integer, no `Integer`, no `Rati... |
| FR-405-Separation | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:28 | **Separation of implementations.** The native operations SHALL be a separate implementation from the owner's `evaluate_ieee` and `compare_ieee`, and from any "expected" side of a proof. The owner is the oracle of the host differential. A proof whose expected value is computed by the same body it ... |
| FR-405-B1 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:40 | 1. The native float SHALL hold a width tag (`Binary32` or `Binary64`) and the operand bit pattern in a `u64`, SHALL be `Copy`, SHALL have no drop glue and SHALL hold no owner `Integer`, `Rational`, `IeeeValue` or heap type. A total constructor from a width and a bit pattern SHALL exist; a bit pat... |
| FR-405-B2 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:41 | 2. A conversion in each direction between the carrier and the owner's `IeeeValue` SHALL exist and SHALL be lossless; it is outside the proof path and may allocate. |
| FR-405-B3 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:45 | 3. The native evaluation SHALL support `Negate`, `Add`, `Subtract`, `Multiply` and `Divide`, and the native comparison SHALL support `NumericEqual`, `TotalOrder` and `BitIdentical`, each over operands of one width. Operands of different widths SHALL be `ill_typed` before any charge, as the owner ... |
| FR-405-B4 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:46 | 4. The native evaluation SHALL take a rounding mode and a `Meter` and SHALL implement all five rounding modes (toward zero, toward positive, toward negative, nearest-even, nearest-away) and the strict `Exact` spelling, which refuses a result that would discard a nonzero part and returns only the ... |
| FR-405-B5 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:47 | 5. For every operand pair and mode, the completed result bits, including the sign of zero, the overflow result of each mode, the subnormal results and the NaN produced (payload and quietness as the owner defines them), the flag set (`invalid`, `divide_by_zero`, `overflow`, `underflow`, `inexact`)... |
| FR-405-B6 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:48 | 6. The native operations SHALL charge the `Meter` at the same charge points, in the same order, with the same limit kinds and amounts as the owner, SHALL stop at the same denied charge with the same incomplete record, and SHALL consume nothing for an ill-typed call. |
| FR-405-B7 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:49 | 7. The proof-path bodies SHALL use only fixed-width native integer arithmetic (shifts, masks, a widening multiply and a restoring or equivalent division in native width), SHALL perform no allocation, SHALL contain no data-dependent loop and no iterator adapter, SHALL NOT be selected by `cfg(kani)... |
| FR-405-B8 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:53 | 8. The required width is `Binary32`. `Binary64` addition and multiplication SHALL be in scope for this requirement only if a probe of them under Kani, with a 10-minute wall-clock limit per harness, reaches a terminal verdict of VERIFIED within that limit, recorded with its peak memory and tool ve... |
| FR-405-B9 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:57 | 9. The kernel SHALL carry a host differential that runs the native operations against the shipped `evaluate_ieee` and `compare_ieee` on a boundary grid fixed before the implementation, covering at least: both zeros, the smallest and largest subnormal, the smallest and largest normal, one and its ... |
| FR-405-B10 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:58 | 10. The differential SHALL be able to fail. The tests SHALL include buildable mutants of the native bodies, each paired with the input that fails it, over at least: the rounding increment of each mode, the tie rule, the sticky bit, the subnormal boundary, the overflow result of each mode, the sig... |
| FR-405-B11 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:59 | 11. A Kani harness SHALL prove the native `Binary32` operations and comparisons over all operand bit patterns, rounding modes and meter limits against an expected side that does not share a body with the native operation (a separately written reference, or the property statements of IEEE 754 for ... |
| FR-405-B12 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:60 | 12. The peak memory of every harness of Behavior 11 SHALL be within the K-DR memory budget. No fit is claimed from a prediction or a configured ceiling; the claim is the recorded peak. |
| FR-405-B13 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:61 | 13. When CG's float row (CG FR-053) draws its operands through the carrier, the peak memory, time, tool version and terminal outcome of its composed harness SHALL be recorded; the harness consumes the carrier through the public constructors of Behavior 1 and the operations of Behavior 3 and 4, an... |
| FR-405-B14 | examined | spec/functional/FR-405-kani-tractable-native-float-carrier.md:62 | 14. Until Behavior 11 holds for a width, the kernel SHALL claim no proof for that width. |

## Dispositions

Round 1, against PR #155 head c4424e48 (fix commit c4424e4 over 236b0ff0).

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | - |
| FND-002 | fixed | - |
| FND-003 | fixed | - |
