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

The kernel SHALL provide a native IEEE float carrier and native operations over it: a fixed-width, integer-only reimplementation of `evaluate_ieee` and `compare_ieee` (`src/ieee.rs`, the owners of IEEE arithmetic, flags and charges) that uses no arbitrary-precision integer, no `Integer`, no `Rational` and no allocation, so that a generated Kani proof can draw operand bit patterns symbolically and reach a result without arbitrary-precision work (Behavior 1 to 22).

The consumer is the float row of Contract CodeGen (CG FR-053, prerequisite P1; its AC-13 asks for a host differential of the replacements against the shipped owner). No FR owns the float carrier today. This requirement owns the carrier and the native operations, in one place, because a float operand is a bit pattern and has no admission step to specify apart from the operation. It owns native square root and fused multiply-add as well as add, subtract, multiply and divide.

CG's consumption of the carrier, its composed harness and its measurements are CG's credit and are not this requirement's acceptance (as for FR-402 and FR-398). The refusal that a row which is not Kani-tractable receives is the mechanism of IR-1526; this requirement references that seam and builds none (Behavior 16). `requires_bound` is IR-69's obligation and is not an operation of this carrier. The Integer, Decimal and quantity carriers are FR-384, FR-402 and FR-398; Decimal operations are FR-403.

## Use case

A generated harness for `a + b` over two `float32` operands calls `kani::any::<u32>()` twice, builds two carried floats from the draws, evaluates the native addition under a `Meter` and compares the result bits, flags and charges with the generated oracle. Every `u32` is a valid `binary32` operand (including every NaN payload), so the draw needs no admission step.

## Behavior

### Carrier

1. The native float SHALL hold a width tag (`Binary32` or `Binary64`) and the operand bit pattern in a `u64`, SHALL be `Copy` and SHALL have no drop glue.
2. The native float SHALL hold no owner `Integer`, `Rational`, `IeeeValue` or heap type.
3. A total constructor from a width and a bit pattern SHALL exist. If a `Binary32` pattern exceeds `u32::MAX`, then the constructor SHALL return the refusal `OutsideCarrierDomain` carrying no float, SHALL NOT mask, wrap or saturate the pattern, SHALL take no `Meter` and SHALL charge nothing.
4. Structural equality of two carried floats SHALL be equality of width and bits, so `0.0` and `-0.0`, and two NaN payloads, are distinct.
5. A conversion in each direction between the carrier and the owner's `IeeeValue` SHALL exist and SHALL be lossless; it is outside the proof path and may allocate.

### Separation

6. The native operations SHALL be a separate implementation from the owner's `evaluate_ieee` and `compare_ieee`: no native body SHALL call them or any `Rational`-based owner helper. The expected side of each Kani harness of Behavior 19 SHALL share no function with the native body, with one exception allowed by design: the bit-pattern decode, whose defects are left to the host differential (CG FR-053-AC-14).
7. The native operations SHALL be ordinary host code, compiled and callable in a host build, so that a host test of them is a test of what Kani proves and CG's host differential can run them against the shipped owner.
8. The native operations SHALL be one source for the host and Kani builds, and no body SHALL be selected by `cfg(kani)`. Where CG FR-053 calls the native operations "the carrier's cfg(kani) replacement", the term means this host-callable source and not a Kani-only body.

### Operations

9. The native evaluation SHALL support `Negate`, `Add`, `Subtract`, `Multiply`, `Divide`, `SquareRoot` and `FusedMultiplyAdd`, and the native comparison SHALL support `NumericEqual`, `TotalOrder` and `BitIdentical`, each over operands of one width. If the operand widths differ, then the call SHALL be `ill_typed` before any charge.
10. The native evaluation SHALL take a rounding mode and a `Meter`, SHALL implement the five rounding modes (toward zero, toward positive, toward negative, nearest-even, nearest-away) and SHALL implement the strict `Exact` spelling, which refuses a result that would discard a nonzero part.
11. The outcome SHALL be exactly one of: completed bits and flags; an `Exact` refusal carrying only the would-be flags; or incomplete with no bits and no flags, as the owner's.
12. For each operand pair and mode on the boundary grid of Behavior 17, the completed result bits (including the sign of zero, the overflow result of each mode, subnormal results, and the NaN produced with its payload and quietness as the owner defines them), the flag set (`invalid`, `divide_by_zero`, `overflow`, `underflow`, `inexact`) and the provenance SHALL equal the owner's. `Negate` is bitwise sign negation and raises no flag (FR-394). The equality is evidenced on that grid and by the proofs of Behavior 19 against their expected side; it is not claimed for every operand pair of the owner by a host test.
13. The native operations SHALL charge the `Meter` at the same charge points, in the same order, with the same limit kinds and amounts as the owner, SHALL stop at the same denied charge with the same incomplete record, and SHALL consume nothing for an ill-typed call.
14. The proof-path bodies SHALL use only fixed-width native integer arithmetic, SHALL perform no allocation, SHALL use no iterator adapter, SHALL name no `num_bigint` item, and SHALL contain no data-dependent loop. A loop is permitted only if its bound is a stated constant and it has no data-dependent exit: a `break`, `return` or `?` whose condition depends on an operand inside a constant-bound loop is forbidden, so every execution runs the stated number of steps.

### Gating of operation and width

15. Every combination of operation (the seven of Behavior 9) and comparison with width (`Binary32`, `Binary64`) SHALL be gated independently: its end state is either an executed Kani VERIFIED verdict of its Behavior 19 harness, or a measured row for it, recorded in TC-953's evidence record and in the Kani tractability table that IR-1526's seam loads, with a terminal outcome of timeout, memory exhaustion or unreliable. A FAILED verdict (a counterexample) is neither state: it is a defect of the native operation, and it fails the combination. A combination in the second state SHALL NOT fail the criteria of the other combinations, and no proof SHALL be claimed for it.
16. The refusal that a combination in the second state of Behavior 15 receives, its cause and whether it charges the `Meter`, are those of IR-1526's refusal seam, which consumes the table row. This requirement SHALL reference that seam and SHALL NOT build a second refusal mechanism.
17. The kernel SHALL carry a host differential that runs the native operations against the shipped `evaluate_ieee` and `compare_ieee` on a boundary grid fixed before the implementation, covering at least: both zeros, the smallest and largest subnormal, the smallest and largest normal, one and its neighbours, both infinities, a quiet NaN, a signalling NaN and a NaN with a nonzero payload, each in both signs, crossed in every operation and every one of the six rounding spellings; the exact ties and one-ulp-off-tie operands of each rounding mode; and the overflow, underflow and exact-cancellation edges of each operation. It SHALL compare bits, flags, provenance, the outcome kind, and every charge point with its limit kinds and amounts, at a sufficient, an exact and a one-less limit. It covers these inputs and not every operand pair.
18. The differential SHALL be able to fail. The tests SHALL include buildable mutants of the native bodies, each paired with the input that fails it, with one mutant per charge point of the owner's `ieee.rs` (including `ieee.exact-intermediate` and `ieee.round`, each also made on a classification-decided input) and per other site of TC-953's table, so that completeness is checked against the owner's charge points and not against the table alone; a mutant that no input distinguishes is not admitted.

### Proof evidence

19. For each combination that is VERIFIED under Behavior 15, a Kani harness SHALL prove the native operation over all operand bit patterns of the width, all six rounding spellings and all meter limits against an expected side whose full outcome (result bits including NaN payload, flags, outcome kind and charge sequence) is determined by a separately written reference. A harness whose expected side is a set of properties instead SHALL list in its docstring and in TC-953 exactly which outputs it covers, and the uncovered outputs are left to Behavior 17.
20. Each harness SHALL have a vacuity check: a cover that the result path is reachable, and a mutant of the native body that the harness fails.
21. Each harness SHALL run under a 10-minute wall-clock limit and return a terminal verdict within it, and its recorded peak memory SHALL be at most the 16 GiB per-request ceiling of CG FR-028. The record SHALL give the peak, its provenance, the time and the tool version; a configured ceiling or a prediction is not a record. A harness that does not return, returns UNDECIDED or exhausts memory is inconclusive and is the measured row of Behavior 15, never a pass.
22. The kernel SHALL claim no proof for a combination whose harness has no executed VERIFIED verdict, and its status SHALL record, per operation and width, the verdict or the table row.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-405-AC-1 | The carrier is `Copy`, has no drop glue and holds no owner `Integer`, `Rational`, `IeeeValue` or heap type, and the conversions to and from `IeeeValue` round-trip every boundary pattern. | Test (TC-953) |
| FR-405-AC-2 | The constructor refuses a `Binary32` pattern above `u32::MAX` (for example `1 << 32`) with `OutsideCarrierDomain`, never masks it to `0`, takes no `Meter` (a coercion to a `Meter`-free signature compiles) and charges nothing; `0.0` and `-0.0`, and two NaN payloads, are unequal under structural equality. | Test (TC-953) |
| FR-405-AC-3 | The native evaluation supports exactly Negate, Add, Subtract, Multiply, Divide, SquareRoot and FusedMultiplyAdd and the native comparison exactly NumericEqual, TotalOrder and BitIdentical; a cross-width call is `ill_typed` and a `Meter` presented to it is unchanged. | Test (TC-953) |
| FR-405-AC-4 | For each of the five rounding modes and `Exact`, the native result bits equal the owner's on the tie and one-ulp-off-tie operands of each mode, including the overflow result of each mode (infinity or the largest finite), the exact-zero sum sign, and subnormal results; the `Exact` refusal carries only the would-be flags and no bits. | Test (TC-953) |
| FR-405-AC-5 | The flag set equals the owner's on the boundary grid, including `invalid` from a NaN-producing operation, `divide_by_zero` from a finite nonzero over zero, `overflow`, `underflow` and `inexact`; and the NaN result equals the owner's in sign, quietness and payload. | Test (TC-953) |
| FR-405-AC-6 | The charge points, their order, limit kinds and amounts, the denied charge and the incomplete record equal the owner's at a sufficient, an exact and a one-less limit for every boundary case; an ill-typed call charges nothing. | Test (TC-953) |
| FR-405-AC-7 | The proof-path source contains no `num_bigint` item, no owner `Integer`, `Rational` or `Vec`, no `cfg(kani)` token, no iterator adapter and no data-dependent loop (every loop has a stated constant bound and no operand-dependent `break`, `return` or `?`), and no native body calls `evaluate_ieee`, `compare_ieee` or a `Rational`-based owner helper. | Inspection (code review; no automated step) |
| FR-405-AC-8 | A counting allocator around the native operations over the boundary grid observes zero allocations (the conversions of Behavior 5 are outside that window), and a host test calls each native operation and comparison. | Test (TC-953) |
| FR-405-AC-9 | The expected side of every Kani harness shares no function with the native body and not even the bit-pattern decode, and the harness docstring states the outputs it determines (Behavior 19). | Inspection (code review; no automated step) |
| FR-405-AC-10 | The host differential passes on the full boundary grid, each of the six rounding spellings and each operation and comparison, and each mutant of TC-953 fails it at the step TC-953 names; a mutant with no distinguishing input is rejected. | Test (TC-953) |
| FR-405-AC-11 | For each combination of operation and width, either a Kani harness of Behavior 19 returns an executed terminal VERIFIED verdict, or a measured row exists whose outcome is one of timeout, memory exhaustion or unreliable, recorded in TC-953's evidence record with its time, peak memory, tool version and harness id. A FAILED verdict (a counterexample) fails this criterion for that combination. Each combination is gated on its own harness: a non-VERIFIED outcome for one combination does not fail the criterion for any other. A harness that was not run is neither state. | Test (TC-953) |
| FR-405-AC-12 | Each VERIFIED harness has a vacuity cover that is reached and a harness mutant that it fails, and its expected side determines the full outcome of Behavior 19 or lists exactly the outputs it covers. | Test (TC-953) |
| FR-405-AC-13 | The peak memory of every executed harness, VERIFIED or not, is recorded with its provenance and is at most the 16 GiB ceiling of CG FR-028; a harness whose peak exceeds it is a non-VERIFIED row under AC-11. | Test (TC-953) |
| FR-405-AC-14 | The refusal of a combination in the table-row state is that of IR-1526's seam: the kernel defines no refusal variant, no refusal path and no `Meter` behaviour for it, and a source search finds none. | Inspection (code review; no automated step) |
| FR-405-AC-15 | No proof is claimed for a combination whose harness has no executed VERIFIED verdict, and the Status records, per operation and width, the verdict or the table row. FR-405 is Done only when every combination, SquareRoot and FusedMultiplyAdd included, has one or the other. | Inspection (TC-953) |

## Status

Implemented (IR-1446): `src/native_float.rs` (carrier, `evaluate_native_float`, `compare_native_float`), harnesses in `src/native_float/kani_proofs.rs`, tests in `tests/ir1446_native_float.rs`, the host mutants in `scripts/ir1446_native_float_mutants.py`. The verdict or table row per operation and width (TC-953's evidence record has the time, peak memory, tool version and harness id of each):

| Operation | Binary32 | Binary64 |
| --- | --- | --- |
| Negate | VERIFIED | VERIFIED |
| NumericEqual | VERIFIED | VERIFIED |
| TotalOrder | VERIFIED | VERIFIED |
| BitIdentical | VERIFIED | VERIFIED |
| Add | table row: timeout | table row: timeout |
| Subtract | table row: timeout | table row: timeout |
| Multiply | table row: timeout | table row: timeout |
| Divide | table row: timeout | table row: timeout |
| SquareRoot | table row: timeout | table row: timeout |
| FusedMultiplyAdd | table row: timeout | table row: timeout |

No proof is claimed for a table-row combination: its harness reached no verdict within 600 s and its peak memory (22.7 to 23.3 GB) is above the 16 GiB ceiling. The native operations for those combinations are host-callable and tested against the owner by the differential (AC-4 to AC-6, AC-10); that is a test, not a proof. The rows are for IR-1526's seam to load.

## Dependencies

- `src/ieee.rs` (`evaluate_ieee`, `compare_ieee`) owns IEEE arithmetic, flags and charges and is the oracle of the host differential. FR-394 owns bitwise negation. FR-384 is the native-width and host-availability precedent; FR-402 and FR-403 are the sibling Decimal carrier and operations and the precedent for the evidence structure.
- CG FR-053 (the float row, P1, AC-13, AC-14) is the consumer; CG FR-028 owns the 16 GiB per-request ceiling. CG's consumption is CG's credit, not this requirement's acceptance.
- IR-1526 owns the refusal seam that consumes a non-VERIFIED table row (Behavior 16) and loads the Kani tractability table as data. The table has no versioned home of its own; TC-953's evidence record is the versioned copy of each row's essentials, and no criterion depends on the table's path.
- IR-69 owns `requires_bound`, which is not an operation here. IR-1502 is CG's oracle derivation for square root and fused multiply-add and is an oracle, not a native operation; this requirement owns the native square root and fused multiply-add.
- IR-1445 (text) and IR-1443 (quantity) are sibling carriers and not prerequisites.
