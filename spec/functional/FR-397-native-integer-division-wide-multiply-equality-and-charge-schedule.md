---
id: FR-397
title: "Native integer division, i64-wide multiply, equality and charge schedule entries charge identically to the Integer path"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-384
    type: references
  - target: ix://agent-ix/quire-exact/FR-362
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-562
    type: references
---
# FR-397: Native integer division, i64-wide multiply, equality and charge schedule entries charge identically to the Integer path

## Description

When a generated-code consumer evaluates an operation on native operands (FR-384) that its checked bounds prove fits a machine word, the exact kernel SHALL expose a native entry for division, modulus, an i64-wide multiply and Integer equality, and a charge-only entry, so that the consumer can skip the arbitrary-precision call and still produce the value, the refusal and the Meter charge sequence of the existing `Integer` path. QSL FR-562 (AC-2, AC-3, AC-4) is the consuming requirement. This requirement adds no new charge point, amount or refusal type.

## Inputs

- `NativeInteger` operands, an optional native result interval (absent is the mathematical domain), a `DivisionProfile` and `DivisionMember` for `div`/`rem`, and a `&mut Meter`.

## Outputs

- `evaluate_native_integer_division`, `evaluate_native_integer_modulo` and `evaluate_native_integer_equality`, each returning `Outcome` of `NativeInteger` or `bool`.
- `evaluate_native_integer_multiply_i64`, returning `Result<Outcome<NativeInteger>, NativeIntegerAdmissionError>`.
- `charge_native_integer_schedule` over `NativeIntegerSchedule`, returning `Outcome<()>`.

## Behavior

1. **Division and modulus.** `evaluate_native_integer_division` and `evaluate_native_integer_modulo` return, for the same values, the value, `Undefined(DivisionByZero)` and `DivisionOutOfDomain` / `ModuloOutOfDomain` refusals of `divide` and `modulo`, under the truncating, floor and Euclidean laws, and charge the four `integer-division.*` or `integer-modulus.*` points in the same order and amounts, a zero divisor stopping after the operands charge. The one quotient outside the i128 carrier, `i128::MIN` divided by `-1`, is not in any native domain: with a result interval it is refused as out of that interval, without one it is refused as the native capacity breach, after the same charges as `divide` makes up to and including `*.domain`. The kernel `divide` completes with 2^127 there, so a consumer must fall back to the `Integer` path for it: no checked bound can prove this case fits.
2. **i64-wide multiply.** `evaluate_native_integer_multiply_i64` accepts operands that are signed 64-bit whose exact product is signed 64-bit, so every pair within `[-3037000499, 3037000499]` is accepted and `3037000500` squared is not. An operand outside signed 64-bit, or a product outside it, is a typed `NativeIntegerAdmissionError` returned before any charge; no product wraps or saturates. Accepted products, the optional result-interval refusal and the charges equal the `Multiply` of `evaluate_integer_arithmetic`. The signed32 `Multiply` plan of FR-384 is unchanged.
3. **Equality and Boolean.** `evaluate_native_integer_equality` returns whether the two payloads are equal and charges the `equality.plan-form`, `equality.plan`, `equality.pair` and `equality.result-retain` points `planned_equality` charges for two `Integer` values, for operands of any width. The Boolean connectives over native `bool` operands are `evaluate_boolean` and `retain_boolean`, which charge only `boolean.result-retain` and are unchanged.
4. **Charge-only entry.** `charge_native_integer_schedule` charges, and computes nothing, the success-path schedule of the named native arithmetic, i64 multiply, division, modulus, ordering or equality operation, and returns `Undefined(DivisionByZero)` after the operands charge for a zero divisor. The evaluators and this entry call the same charge functions, and the `Integer` division and equality paths call those functions too, so the amounts and order cannot differ. It is success-path only: a consumer uses it after its bounds prove that the result and any result domain are satisfied.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-397-AC-1 | For each of the three laws and both members, and for `modulo`, over dividend and divisor pairs of every sign combination, zero, `i128::MIN`, `i128::MAX`, `-1` and values of different widths, the native entry returns the value, `Undefined(DivisionByZero)` or refusal `divide` / `modulo` returns for the same values with and without a result interval, and the captured per-charge events and final counters equal those of the `Integer` path, including at a one-below limit. The one exception is `i128::MIN` divided by `-1` for the quotient with no result interval, where the native entry returns `CheckedInvariant(NativeIntegerCapacityBreach)` after the charges up to `*.domain` (the same events as `divide`'s prefix) while `divide` completes. | Test (TC-944) |
| FR-397-AC-2 | `evaluate_native_integer_multiply_i64` at the pairs `(3037000499, 3037000499)`, `(-3037000499, 3037000499)`, `(i64::MIN, 1)`, `(i64::MIN, -1)` and `(3037000500, 3037000500)`, zero and one, returns the value `evaluate_integer_arithmetic` returns with equal charge events for each accepted pair, and returns `CapacityExceeded` or `OperandOutsideInterval` with no charge for each rejected pair, including a result interval that excludes the product. | Test (TC-944) |
| FR-397-AC-3 | `evaluate_native_integer_equality` over equal, unequal, signed and different-width operands returns `planned_equality`'s value over `Value::Integer` operands with equal charge events; `evaluate_boolean` over the full truth tables of `And`, `Or`, `Implies` and `Not` returns the connective and charges only `boolean.result-retain`. | Test (TC-944) |
| FR-397-AC-4 | For each `NativeIntegerSchedule`, the events and counters after `charge_native_integer_schedule` equal those of the corresponding evaluator on a successful operation, and for a zero divisor equal the evaluator's, at sufficient limits and at a limit one below each charge; an `Incomplete` names the same limit at the same charge point. | Test (TC-944) |

## Status

Implemented.

## Dependencies

- [FR-384](./FR-384-admitted-native-integer-refinement.md) owns the native carrier, admission and the signed32 `Multiply` plan.
- QSL FR-562 owns the consuming fast-path rule.
