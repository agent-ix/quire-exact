---
id: FR-405
title: "A Kani-tractable native IEEE float carrier and operations refining the owner's float path"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-402
    type: references
  - target: ix://agent-ix/quire-exact/FR-403
    type: references
  - target: ix://agent-ix/quire-exact/FR-394
    type: references
  - target: ix://agent-ix/quire-exact/FR-384
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-053
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
---
# FR-405: A Kani-tractable native IEEE float carrier and operations refining the owner's float path

## Description

The kernel SHALL provide a native IEEE float carrier and native operations over it: a fixed-width, integer-only reimplementation of `evaluate_ieee` and `compare_ieee` (`src/ieee.rs`, the owners of IEEE arithmetic, flags and charges) that uses no arbitrary-precision integer, no `Integer`, no `Rational` and no allocation, so that a generated Kani proof can draw operand bit patterns symbolically and reach a result without arbitrary-precision work (Behavior 1 to 14).

The consumer is the float row of Contract CodeGen (CG FR-053, whose AC-13 asks for a host differential of the replacements against the shipped owner). No FR owns the float carrier today. This requirement is both halves of it: the carrier and the operations, in one place, because a float operand is a bit pattern and has no admission step to specify apart from the operation.

**Separation of implementations.** The native operations SHALL be a separate implementation from the owner's `evaluate_ieee` and `compare_ieee`, and from any "expected" side of a proof. The owner is the oracle of the host differential. A proof whose expected value is computed by the same body it checks proves nothing; a native operation that calls the owner's `Rational` route is not a native operation. The native operations SHALL be ordinary host code, compiled and callable in a host build, and SHALL NOT be a `cfg(kani)`-only replacement, so that a host test of them is a test of what Kani proves and so that CG's host differential can run them against the shipped owner.

Out of scope, referenced and not owned: native square root and fused multiply-add belong to Contract CodeGen's IR-1502; the `requires_bound` obligation belongs to IR-69. The native surface of this requirement has no entry for them. Integer, Decimal and quantity carriers are FR-384, FR-402 and FR-398; Decimal operations are FR-403.

## Use case

A generated harness for `a + b` over two `float32` operands calls `kani::any::<u32>()` twice, builds two carried floats from the draws, evaluates the native addition under a symbolic or fixed-limit `Meter` and compares the result bits, flags and charges with the generated oracle. Every `u32` is a valid `binary32` operand (including every NaN payload), so the draw needs no admission step.

## Behavior

### Carrier

1. The native float SHALL hold a width tag (`Binary32` or `Binary64`) and the operand bit pattern in a `u64`, SHALL be `Copy`, SHALL have no drop glue and SHALL hold no owner `Integer`, `Rational`, `IeeeValue` or heap type. A total constructor from a width and a bit pattern SHALL exist; a bit pattern outside the width (a `binary32` pattern above `u32::MAX`) SHALL be refused with a typed refusal and never masked, wrapped or saturated. Two carried floats are structurally equal exactly when width and bits are equal (so `0.0` and `-0.0`, and two NaN payloads, are distinct).
2. A conversion in each direction between the carrier and the owner's `IeeeValue` SHALL exist and SHALL be lossless; it is outside the proof path and may allocate.

### Operations

3. The native evaluation SHALL support `Negate`, `Add`, `Subtract`, `Multiply` and `Divide`, and the native comparison SHALL support `NumericEqual`, `TotalOrder` and `BitIdentical`, each over operands of one width. Operands of different widths SHALL be `ill_typed` before any charge, as the owner does. `SquareRoot` and `FusedMultiplyAdd` have no native entry (out of scope above).
4. The native evaluation SHALL take a rounding mode and a `Meter` and SHALL implement all five rounding modes (toward zero, toward positive, toward negative, nearest-even, nearest-away) and the strict `Exact` spelling, which refuses a result that would discard a nonzero part and returns only the would-be flags. The outcome SHALL be exactly one of: completed bits and flags, an `Exact` refusal carrying only the would-be flags, or incomplete with no bits and no flags, as the owner's.
5. For every operand pair and mode, the completed result bits, including the sign of zero, the overflow result of each mode, the subnormal results and the NaN produced (payload and quietness as the owner defines them), the flag set (`invalid`, `divide_by_zero`, `overflow`, `underflow`, `inexact`) and the provenance SHALL equal the owner's. `Negate` is bitwise sign negation and raises no flag (FR-394).
6. The native operations SHALL charge the `Meter` at the same charge points, in the same order, with the same limit kinds and amounts as the owner, SHALL stop at the same denied charge with the same incomplete record, and SHALL consume nothing for an ill-typed call.
7. The proof-path bodies SHALL use only fixed-width native integer arithmetic (shifts, masks, a widening multiply and a restoring or equivalent division in native width), SHALL perform no allocation, SHALL contain no data-dependent loop and no iterator adapter, SHALL NOT be selected by `cfg(kani)` and SHALL name no `num_bigint` item. Any fixed-count step sequence SHALL be stated by its count so that an inspection can check it.

### Width set

8. The required width is `Binary32`. `Binary64` addition and multiplication SHALL be in scope for this requirement only if a probe of them under Kani, with a 10-minute wall-clock limit per harness, reaches a terminal verdict of VERIFIED within that limit, recorded with its peak memory and tool version. If the probe does not, then `Binary64` is out of scope for this requirement, the native surface SHALL refuse a `Binary64` operand with a typed refusal rather than answer from a different route, and a follow-up requirement SHALL be filed for it; the domain is not narrowed to make a proof fit.

### Evidence

9. The kernel SHALL carry a host differential that runs the native operations against the shipped `evaluate_ieee` and `compare_ieee` on a boundary grid fixed before the implementation, covering at least: both zeros, the smallest and largest subnormal, the smallest and largest normal, one and its neighbours, both infinities, a quiet NaN, a signalling NaN and a NaN with a nonzero payload, each in both signs, crossed in every operation and every one of the six rounding spellings; the exact ties and one-ulp-off-tie operands of each rounding mode; and the overflow, underflow and exact-cancellation edges of each operation. The differential SHALL compare bits, flags, provenance, the outcome kind, and every charge point with its limit kinds and amounts, at a sufficient, an exact and a one-less limit.
10. The differential SHALL be able to fail. The tests SHALL include buildable mutants of the native bodies, each paired with the input that fails it, over at least: the rounding increment of each mode, the tie rule, the sticky bit, the subnormal boundary, the overflow result of each mode, the sign of an exact zero sum, the NaN payload, each flag, the `Exact` refusal and each charge point; a mutant that no input distinguishes is not admitted.
11. A Kani harness SHALL prove the native `Binary32` operations and comparisons over all operand bit patterns, rounding modes and meter limits against an expected side that does not share a body with the native operation (a separately written reference, or the property statements of IEEE 754 for the operation, never the native body or the owner's route). Each harness SHALL return a terminal VERIFIED verdict, recorded with its peak memory, time and tool version. A harness that does not return, returns UNDECIDED or exhausts memory is inconclusive and never a pass; the proof SHALL NOT be claimed for a harness that has no executed verdict. Each harness SHALL have a vacuity check (a cover that the result path is reachable, and a mutant of the native body that the harness fails).
12. The peak memory of every harness of Behavior 11 SHALL be within the K-DR memory budget. No fit is claimed from a prediction or a configured ceiling; the claim is the recorded peak.
13. When CG's float row (CG FR-053) draws its operands through the carrier, the peak memory, time, tool version and terminal outcome of its composed harness SHALL be recorded; the harness consumes the carrier through the public constructors of Behavior 1 and the operations of Behavior 3 and 4, and the row's host differential (CG FR-053-AC-13) SHALL run the native operations against the shipped owner of Behavior 9, with the boundary coverage and the mutants of Behavior 9 and 10.
14. Until Behavior 11 holds for a width, the kernel SHALL claim no proof for that width.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-405-AC-1 | The carrier is `Copy`, has no drop glue and holds no owner or heap type; the constructor refuses a `Binary32` pattern above `u32::MAX` (for example `1 << 32`) and never masks it to `0`; `0.0` and `-0.0`, and two NaN payloads, are unequal under structural equality; the conversions to and from `IeeeValue` round-trip every boundary pattern. | Test (TC-953) |
| FR-405-AC-2 | The native evaluation supports exactly Negate, Add, Subtract, Multiply and Divide and the native comparison exactly NumericEqual, TotalOrder and BitIdentical; a cross-width call is `ill_typed` and a `Meter` presented to it is unchanged; there is no native entry for square root, fused multiply-add or `requires_bound`. | Test (TC-953) |
| FR-405-AC-3 | For each of the five rounding modes and `Exact`, the native result bits equal the owner's on the tie and one-ulp-off-tie operands of each mode, including the overflow result of each mode (infinity or the largest finite), the exact-zero sum sign, and subnormal results; the `Exact` refusal carries only the would-be flags and no bits. | Test (TC-953) |
| FR-405-AC-4 | The flag set equals the owner's on the whole boundary grid, including `invalid` from a NaN-producing operation, `divide_by_zero` from a finite nonzero over zero, `overflow`, `underflow` and `inexact`; and the NaN result equals the owner's in sign, quietness and payload. | Test (TC-953) |
| FR-405-AC-5 | The charge points, their order, limit kinds and amounts, the denied charge and the incomplete record equal the owner's at a sufficient, an exact and a one-less limit for every boundary case; an ill-typed call charges nothing. | Test (TC-953) |
| FR-405-AC-6 | The proof-path bodies contain no `num_bigint` item, no owner `Integer`, `Rational` or `Vec`, no `cfg(kani)` token, no data-dependent loop and no iterator adapter, and any fixed-count step sequence is stated by its count; a counting allocator around the native operations over the boundary grid observes zero allocations (the conversions of Behavior 2 are outside that window). | Test (TC-953) |
| FR-405-AC-7 | The native operations are callable in a host build (a host test calls each), are one source for the host and `cfg(kani)` builds, and are a separate implementation from the owner: no native body calls `evaluate_ieee`, `compare_ieee` or any `Rational`-based owner helper, and the expected side of every Kani harness shares no function with the native body. | Test (TC-953) |
| FR-405-AC-8 | The host differential of Behavior 9 passes on the full boundary grid, each of the six rounding spellings and each operation, and the mutants of Behavior 10 each fail the differential at the step TC-953 names; a mutant with no distinguishing input is rejected. | Test (TC-953) |
| FR-405-AC-9 | The width set is explicit and evidenced: the 10-minute probe of `Binary64` addition and multiplication under Kani is executed and its terminal verdict, wall time, peak memory and tool version are recorded. `Binary64` is in scope only if both returned VERIFIED within the limit; otherwise `Binary64` operands are refused with a typed refusal and a follow-up requirement is filed. A probe that was not run, or whose verdict is a timeout, is not evidence of scope. | Inspection (TC-953) |
| FR-405-AC-10 | For each in-scope width, a Kani harness per operation and per comparison proves the native operations over all operand bit patterns, all six rounding spellings and all meter limits against an independent expected side, each with an executed terminal VERIFIED verdict, a vacuity cover and a failing mutant; the verdicts and the tool version are recorded. A harness without an executed verdict is recorded as inconclusive and no proof is claimed for it. | Inspection (TC-953) |
| FR-405-AC-11 | The peak memory of every harness of AC-10 is recorded with its provenance and is within the K-DR memory budget; a configured ceiling or a prediction is not accepted as the record. | Inspection (TC-953) |
| FR-405-AC-12 | CG's float row (CG FR-053) draws its operands through the carrier and its host differential (CG FR-053-AC-13) runs the native operations against the shipped owner with the boundary coverage and the mutants of AC-8; the composed harness's peak memory, time, tool version and terminal outcome are recorded. Until the row consumes the carrier the criterion is unexecuted and no composed fit is claimed. | Inspection (TC-953) |
| FR-405-AC-13 | No proof is claimed for a width, operation or comparison whose AC-10 harness has no executed VERIFIED verdict, and the status records, per operation and width, the verdict or the absence of one. | Inspection (TC-953) |

## Status

Specified (IR-1446). Not implemented; no Rust accompanies this requirement, no Kani harness has run and no probe result or measurement exists.

## Dependencies

- `src/ieee.rs` (`evaluate_ieee`, `compare_ieee`) owns IEEE arithmetic, flags and charges and is the oracle of the host differential. FR-394 owns bitwise negation. FR-384 is the native-width and host-availability precedent; FR-402 and FR-403 are the sibling Decimal carrier and operations and the precedent for the evidence structure.
- CG FR-053 (the float row, AC-13) is the consumer; CG FR-028 owns the per-request memory ceiling. CG's consumption is CG's credit, not this requirement's acceptance.
- Native square root and fused multiply-add are Contract CodeGen's IR-1502, and `requires_bound` is IR-69; neither is owned or required here.
- IR-1445 (text) and IR-1443 (quantity) are sibling carriers and not prerequisites.
