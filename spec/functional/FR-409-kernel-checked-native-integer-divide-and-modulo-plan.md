---
id: FR-409
title: "A kernel-checked native integer Divide and Modulo plan with measured Kani verdicts"
type: FR
org: agent-ix
relationships:
  - target: ix://agent-ix/quire-exact/FR-384
    type: references
  - target: ix://agent-ix/quire-exact/FR-397
    type: references
  - target: ix://agent-ix/quire-exact/FR-405
    type: references
  - target: ix://agent-ix/quire-exact/AD-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/AD-006
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-057
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-028
    type: references
---
# FR-409: A kernel-checked native integer Divide and Modulo plan with measured Kani verdicts

## Description

The kernel SHALL extend the checked native integer plan of [FR-384](./FR-384-admitted-native-integer-refinement.md) with a `Divide` operation per `DivisionProfile` and a `Modulo` operation, so that a consumer can admit native division operands through `NativeIntegerPlan::new` and `NativeIntegerPlan::admit` and evaluate the admitted operation with the value, refusals and Meter charges of the exact `divide` and `modulo` (Behavior 1 to 12). Each Kani harness over the plan SHALL settle as an executed VERIFIED verdict or as a measured not-tractable row (Behavior 13 to 20).

Today `NativeIntegerOperation` and `NativeIntegerOperationKind` (`src/native_integer.rs`) name only `Add`, `Subtract`, `Multiply` and `Negate`, and FR-384's capability table says the closed interval-operation input "cannot encode ... an unsupported division operation". The plan-free evaluators `evaluate_native_integer_division` and `evaluate_native_integer_modulo` of [FR-397](./FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md) exist, but they check no operand interval and derive no capacity. CG AD-006 (ix://agent-ix/quire-contract-codegen/AD-006) requires a kernel-checked plan before native-route evaluation. CG FR-057 (ix://agent-ix/quire-contract-codegen/FR-057) therefore refuses native `div` and `mod` (its AC-6) until such a plan exists. IR-1530 measured general-route `rt::Integer` arithmetic as not tractable from 2 bits, so a VERIFIED division verdict needs this native plan.

The consumer is CG FR-057 (IR-854, AC-7 and AC-8 on the native route). CG's harness, its measurements and its credit are CG's and are not this requirement's acceptance (as for FR-402, FR-405 and FR-407). The refusal that a measured not-tractable row receives is the mechanism of IR-1526: this requirement references that seam and builds none. Amending AD-006 to allow plan-free evaluation is out of scope, and so is a constant-divisor tractability flag (CG FR-056), unless the measured rows need one (see Open items).

## Use case

A CG native-route harness for `x div y` under the floor law over `x: Int[-100, 100]` and `y: Int[-3, 3]` builds `NativeIntegerPlan::new(NativeIntegerOperation::Divide { profile: Floor, member: Quotient, dividend: [-100, 100], divisor: [-3, 3] }, Some(result_domain))` from constant endpoints, draws `x` and `y` as symbolic `i64` values, admits them, and evaluates the admitted operation under a `Meter`. It gets `Undefined(DivisionByZero)` at `y == 0`, the floor quotient when it lies in the result domain, and `DivisionOutOfDomain { member: Quotient }` otherwise, exactly as `divide` gives over `Integer`.

## Inputs

- A `NativeIntegerOperation::Divide` carrying a `DivisionProfile` (truncating, floor or Euclidean), a `DivisionMember` (quotient or remainder), a dividend interval and a divisor interval; or a `NativeIntegerOperation::Modulo` carrying a dividend interval and a divisor interval.
- An optional semantic result interval (`None` is the mathematical domain), as for the existing plan.
- Binary native operands for admission, and a `&mut Meter` for evaluation.

## Outputs

- A `NativeIntegerPlan` with its kernel-derived capacity interval, or a typed `NativeIntegerAdmissionError`.
- An opaque `AdmittedNativeIntegerOperation`, or a typed admission error.
- An `Outcome<NativeInteger>` from evaluation, with the real Meter's charge events.
- For each Kani harness, a terminal record: VERIFIED with its covers and a failing harness mutant, or a measured row (timeout, memory exhaustion or unreliable) with its time and peak.

## Behavior

### Plan shape

1. `NativeIntegerOperation` SHALL gain a `Divide` variant holding `profile: DivisionProfile`, `member: DivisionMember`, `dividend: NativeIntegerInterval` and `divisor: NativeIntegerInterval`, and a `Modulo` variant holding `dividend` and `divisor` intervals and no profile. `NativeIntegerOperationKind` SHALL gain `Divide` and `Modulo`. The `Modulo` operation SHALL always mean the Euclidean remainder, independent of any `div` or `rem` law, as `modulo` does.
2. The plan SHALL take the profile and member only from the `Divide` variant the caller constructs. No other input SHALL select or default a profile.
3. `NativeIntegerPlan::admit` SHALL accept `NativeIntegerOperands::Binary` for `Divide` and `Modulo`, SHALL check the dividend against the dividend interval and the divisor against the divisor interval before any charge, and SHALL return `OperandShapeMismatch { operation: Divide }` or `{ operation: Modulo }` for unary operands. A divisor of zero that lies in the divisor interval SHALL be admitted: the zero divisor is an evaluation outcome and not an admission failure.

### Capacity

With `M` the largest magnitude of the two dividend endpoints and `B` the largest magnitude of the two divisor endpoints (each computed without overflow, so `|i128::MIN|` is `2^127`):

4. If the member is the quotient, the dividend interval contains `i128::MIN` and the divisor interval contains `-1`, then `NativeIntegerPlan::new` SHALL return `CapacityExceeded { operation: Divide }` before any charge. This holds with or without a semantic result interval: a result interval that would exclude `2^127` SHALL NOT conceal the capacity refusal (FR-384 and AD-006). It is the only quotient that leaves the i128 carrier, under all three laws.
5. Otherwise, for the quotient member, the plan's capacity SHALL be `[max(-M, i128::MIN), min(M, i128::MAX)]`. For the remainder member it SHALL be `[-(B' - 1), B' - 1]`, and for `Modulo` it SHALL be `[0, B' - 1]`, where `B'` is `max(B, 1)` and `B' - 1` is computed without overflow (`i128::MAX` when `B` is `2^127`). The remainder and `Modulo` plans SHALL never be refused for capacity.
6. Plan construction and admission SHALL use only bounded native arithmetic, SHALL perform no division of a symbolic operand, and SHALL construct no `Integer`, as FR-384 requires of the existing plan.

### Evaluation

7. When an admitted `Divide` operation is evaluated, the kernel SHALL return the outcome and charge events that `evaluate_native_integer_division(profile, member, dividend, divisor, result_domain, meter)` returns for the same values, and `divide` returns over `Integer` for the same values and domain. When an admitted `Modulo` operation is evaluated, it SHALL return those of `evaluate_native_integer_modulo` and `modulo`. The charges are the four `integer-division.*` or `integer-modulus.*` points in FR-397's order, not the `integer-arithmetic.*` points.
8. If the admitted divisor is zero, then evaluation SHALL return `Undefined(DivisionByZero)` after the operands charge and SHALL make no later charge, as `divide` and `modulo` do.
9. If the member lies outside the semantic result interval, then evaluation SHALL return `Refused(DivisionOutOfDomain { member, .. })` for `Divide` or `Refused(ModuloOutOfDomain { .. })` for `Modulo`, after the domain charge and before result-retain, carrying the result interval as `divide` and `modulo` carry it.
10. A completed member SHALL lie in the plan's capacity. If it does not, evaluation SHALL return `CheckedInvariant(NativeIntegerCapacityBreach)` and no completed value. Behavior 4 and 5 make that outcome unreachable for an admitted operation, and the Kani harness of Behavior 14 asserts it unreachable.
11. `charge_native_integer_schedule(NativeIntegerSchedule::Arithmetic(admitted))` over an admitted `Divide` or `Modulo` operation SHALL charge the schedule of `NativeIntegerSchedule::Division` or `NativeIntegerSchedule::Modulus` over the same operands.
12. The `Add`, `Subtract`, `Multiply` and `Negate` plans, their capacities, refusals and charges SHALL be unchanged, and the plan-free evaluators of FR-397 SHALL stay public and unchanged.

### Kani harnesses

13. A plan harness per operation kind (`Divide` with each member, and `Modulo`) SHALL draw both intervals' endpoints as unrestricted `i128` values and the result interval as `None` or two unrestricted endpoints. It SHALL assert that `NativeIntegerPlan::new` refuses exactly in the case of Behavior 4, that the capacity equals Behavior 5's interval as an independently written expression computes it, and that admission succeeds exactly when both operands are members. It performs no division.
14. An evaluation harness SHALL exist for each of the seven member combinations (the quotient and the remainder under each of the three profiles, and `Modulo`) in each shape below. It SHALL build the plan through `NativeIntegerPlan::new`, draw both operands symbolically within the shape's intervals with no assumption on the divisor, the dividend or the outcome, admit them and evaluate under a real `Meter` that denies no charge. The shapes are:
    - symbolic divisor: dividend and divisor intervals both `[-2^(w-1), 2^(w-1) - 1]` for `w` in 8, 16, 32, 64 and 128, so zero and both signs are drawn;
    - constant divisor: the divisor interval a single literal, one harness each for `7`, `-7` and `-1`, with the dividend interval at `w` = 64 and `w` = 128.

    For the quotient member at `w` = 128 the dividend interval SHALL be `[i128::MIN + 1, i128::MAX]`, because Behavior 4 refuses the full interval with a divisor interval containing `-1`. The plan harness of Behavior 13 covers that refusal.
15. The expected side of each evaluation harness SHALL be relational and SHALL share no function with the production division body:
    - `b == 0` requires `Undefined(DivisionByZero)`.
    - A completed quotient `q` requires `a - q * b` to lie in the profile's remainder window `W`: truncating `[0, |b| - 1]` when `a >= 0` and `[-(|b| - 1), 0]` otherwise; floor `[0, b - 1]` when `b > 0` and `[b + 1, 0]` otherwise; Euclidean `[0, |b| - 1]`. `W` holds exactly `|b|` consecutive integers, so this determines `q`.
    - A completed remainder `r`, or a completed `Modulo` value `m`, requires `a == q * b + r` (or `+ m`) and `r` in the profile's `W` (`m` in the Euclidean `W`). `q` is the companion quotient that the harness obtains from the production quotient of the same profile. At `i128::MIN` and `-1`, where no native quotient exists, the expected side uses the exact quotient `2^127` in its abstraction.
    - A drawn result interval that excludes the member requires the refusal of Behavior 9.
    - Every product SHALL be written quotient times divisor, in that operand order, in the subject and in the expectation.
    - Arithmetic SHALL be exact: `i128` for `w` up to 64, where every product fits, and the independent signed-limb abstraction of `src/native_integer/reference.rs` (or an equally independent one) for `w` = 128.
    - The expected side SHALL contain no Rust `/` or `%`, and none of `checked_div`, `checked_rem`, `wrapping_div`, `wrapping_rem`, `div_euclid` or `rem_euclid`.
16. Each evaluation harness SHALL assert that `CheckedInvariant(NativeIntegerCapacityBreach)` and every other outcome not named in Behavior 15 are unreachable. It SHALL end with one cover that a completed result is reached, and with one cover per refusal kind that its shape can reach. Its docstring SHALL list the outputs it determines. Charge amounts and limit denials are determined by the host differential of Behavior 21, not by the harness.

### Measurement and settlement

17. Before the full harness set runs, a probe SHALL run the truncating quotient harness at `w` = 8 and at `w` = 64 (symbolic divisor) under a 10-minute wall-clock limit, and SHALL record each one's time and peak. Every Kani run SHALL be one job at a time: no memory-heavy run overlaps another.
18. Each harness SHALL run under a 600-second wall-clock limit. Its record SHALL give the outcome, the time, the peak memory with its provenance, the Kani, CBMC and solver versions, the commit and the harness id. A configured ceiling or a prediction is not a record. A peak above CG FR-028's 16 GiB ceiling makes the harness non-VERIFIED.
19. Each harness SHALL settle as exactly one of:
    - an executed VERIFIED verdict whose covers are all reached and whose harness mutant (Behavior 22) the harness fails;
    - a measured row with outcome timeout, memory exhaustion or unreliable.

    A FAILED verdict SHALL be a defect of the plan or evaluator and SHALL settle neither way. If a symbolic-divisor harness at width `w` is a measured row, then the larger widths of that member combination SHALL be recorded as covered by that smallest failing width, and SHALL NOT be left without a record. A harness that was not run SHALL be neither state, and no combination SHALL be left without a record.
20. Each measured row SHALL be recorded in TC-957's evidence record and in the Kani tractability table that IR-1526's seam loads. The row SHALL name the capability as the operation kind, profile, member, divisor shape and smallest failing width, so that the seam refuses that capability at or above that width. The kernel SHALL define no refusal variant, refusal path or Meter behaviour for a measured row.

### Host evidence and mutants

21. The kernel SHALL carry a host differential of the plan path against `divide` and `modulo` over `Integer` and against the plan-free evaluators. It SHALL use a grid of dividends and divisors, fixed before implementation, holding `0`, `±1`, `±2`, `±7`, `i64::MIN`, `i64::MIN + 1`, `i64::MAX`, `i128::MIN`, `i128::MIN + 1` and `i128::MAX`, under each profile and member and for `Modulo`, with result intervals that contain and that exclude the member. It SHALL compare the outcome kind, the value, the refusal payload and every charge event and counter at a sufficient limit and at a limit one below each charge. Kani's symbolic results are not trusted alone: every width or shape claimed VERIFIED SHALL also be covered by this differential.
22. The tests SHALL include buildable mutants, each paired with the input that fails it. The mutant set SHALL include:
    - **wrong profile:** the plan's profile swapped for each of the other two laws, six swaps; and `Modulo` evaluated under the truncating and the floor law;
    - **missing zero guard:** the zero-divisor check removed or moved after the arithmetic charge;
    - **MIN / -1 unguarded:** the Behavior 4 refusal removed, so the plan admits `i128::MIN` with `-1`.

    Each of these SHALL be killed by the host differential, and by a Kani harness whose shape reaches its distinguishing input, with a FAILED verdict and concrete playback. The plan harness of Behavior 13 kills the MIN / -1 mutant, and the evaluation harnesses kill the others. The decoded playback SHALL be replayed on the host through `divide` or `modulo` over `Integer`, showing the kernel value or outcome differs from the mutant's. A mutant that no input distinguishes SHALL NOT be admitted.
23. No proof SHALL be claimed for a harness without an executed VERIFIED verdict. The Status SHALL record, per member combination, shape and width, the verdict or the table row.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-409-AC-1 | `NativeIntegerOperation` has `Divide { profile, member, dividend, divisor }` and `Modulo { dividend, divisor }`, and `NativeIntegerOperationKind` has `Divide` and `Modulo`. A `Divide` plan under each of the three profiles and both members, and a `Modulo` plan, construct for in-capacity intervals and admit binary operands that are members, including a zero divisor in the divisor interval. A dividend or divisor outside its interval returns `OperandOutsideInterval` with the failing position, and unary operands return `OperandShapeMismatch` naming `Divide` or `Modulo`, each before any charge. | Test (TC-957) |
| FR-409-AC-2 | A quotient plan whose dividend interval contains `i128::MIN` and whose divisor interval contains `-1` returns `CapacityExceeded { operation: Divide }` under each profile, with no result interval and with a result interval that excludes `2^127`. The same intervals for the remainder member and for `Modulo` construct. A dividend `[i128::MIN, i128::MIN]` with a divisor `[1, 2]`, and a dividend `[i128::MIN + 1, 0]` with a divisor `[-1, -1]`, construct. Each capacity equals Behavior 5's interval, including `B` = `2^127` (capacity `[-i128::MAX, i128::MAX]` for the remainder member) and a divisor `[0, 0]` (capacity `[0, 0]`). | Test (TC-957) |
| FR-409-AC-3 | On the Behavior 21 grid, under each profile and member and for `Modulo`, with and without a result interval, evaluating the admitted operation returns the same outcome kind, value and refusal payload as `divide` or `modulo` over `Integer` and as the plan-free evaluator, with equal charge events and counters at a sufficient limit and at one below each charge. This includes `Undefined(DivisionByZero)` after only the operands charge, `DivisionOutOfDomain { member }` or `ModuloOutOfDomain`, `i64::MIN div -1` completing `2^63`, `i64::MIN mod -1` completing `0`, and the vectors `-7 div 2` = `-3`, `-4`, `-4` and `-7 div -2` = `3`, `3`, `4` (truncating, floor, Euclidean), with `-7 mod 2` = `-7 mod -2` = `1`. | Test (TC-957) |
| FR-409-AC-4 | `charge_native_integer_schedule` over an admitted `Divide` or `Modulo` operation produces the events and counters of `NativeIntegerSchedule::Division` or `Modulus` over the same operands, including the zero-divisor stop. The existing four arithmetic plans' capacities, refusals and charges, and the plan-free evaluators' outcomes, are unchanged on their existing tests. | Test (TC-957) |
| FR-409-AC-5 | The plan harness of Behavior 13 runs for `Divide` with each member and for `Modulo`, and the evaluation harnesses of Behavior 14 run for each of the seven member combinations in each shape. Each expected side is relational per Behavior 15, with products ordered quotient then divisor and no division operation. | Test (TC-957) |
| FR-409-AC-6 | Code review of each harness finds an expected side that shares no function with the production division body, contains none of the operations Behavior 15 forbids, keeps quotient-then-divisor operand order, and has a docstring listing the outputs it determines. Code review of plan construction and admission finds no division of a symbolic operand and no `Integer` construction. | Inspection (code review; no automated step) |
| FR-409-AC-7 | The probe of Behavior 17 ran before the full set, and its two records exist with time and peak. Every harness of AC-5 has one terminal record: VERIFIED with every cover reached and its harness mutant FAILED, or a measured row of timeout, memory exhaustion or unreliable. Each record has its time, peak and provenance, tool versions, commit and harness id. A peak above 16 GiB is not VERIFIED. A FAILED verdict on a healthy harness fails this criterion. Larger symbolic widths above a smallest failing width are recorded as covered by it. | Test (installed Kani; TC-957) |
| FR-409-AC-8 | Each measured row appears in TC-957's evidence record and in the tractability table that IR-1526's seam loads. The row names the operation kind, profile, member, divisor shape and smallest failing width. A source search finds no kernel refusal variant, refusal path or Meter behaviour defined for a measured row. | Inspection (TC-957) |
| FR-409-AC-9 | Each mutant of Behavior 22 fails the host differential at its distinguishing input: the six profile swaps, `Modulo` under truncating and floor, the removed or moved zero guard, and the removed MIN / -1 refusal. Each also yields a Kani FAILED verdict with concrete playback in the harness whose shape reaches that input, and the playback replayed through `divide` or `modulo` over `Integer` gives a different value or outcome from the mutant's. The MIN / -1 mutant is killed by the plan harness, with playback `i128::MIN` and `-1` inside the drawn intervals. A mutant with no distinguishing input is rejected. | Test (installed Kani; TC-957) |
| FR-409-AC-10 | No proof is claimed for a harness without an executed VERIFIED verdict. The Status records, per member combination, shape and width, the verdict or the table row, and claims none before the runs exist. | Inspection (TC-957) |

## Open items

- **Remainder for CG `rem`.** The `Divide` variant carries `member`, so the plan exposes the remainder under each law. CG FR-057 keeps `quire.op.integer.rem` `Underived` today: CG FR-014 refuses it as `OperationNotDerivable`. CG needs no remainder from this plan now. Exposing it here means that a later FR-014 derivation of `rem` (owned inside IR-854) needs no second kernel plan change. Whether the remainder rows must reach VERIFIED for CG's MVP is the consumer's decision, and this requirement only records them.
- **MIN / -1 with a result interval.** Behavior 4 refuses the quotient plan at `i128::MIN` with `-1` even when a result interval would make the oracle's outcome `DivisionOutOfDomain`, which the native evaluator can reproduce. That choice keeps FR-384's rule that a semantic interval never conceals a capacity refusal. Admitting those plans instead would be a change to that rule, and it does not affect CG, whose operands are `i64`.
- **Capability name format.** Behavior 20 fixes the fields of a measured row. The exact key string belongs to IR-1526's table format.
- **Constant-divisor flag.** If a symbolic-divisor shape is a measured row at a width where the constant-divisor shape is VERIFIED, CG FR-056 has no flag to tell the shapes apart. That split is CG's seam gap (CG FR-057), and it is not built here.
- **Exhaustive matches.** Adding variants to the public, non-`#[non_exhaustive]` `NativeIntegerOperation` and `NativeIntegerOperationKind` breaks any downstream exhaustive `match`. CG constructs these variants. The coder confirms that CG has no exhaustive match before merge, or records the needed CG change.

## Status

PLANNED (IR-1541). No `Divide` or `Modulo` plan variant exists, and no harness of this requirement has run. No verdict is claimed for any member combination, shape or width.

## Dependencies

- [FR-384](./FR-384-admitted-native-integer-refinement.md) owns the native carrier, the plan, admission and the existing four operations. This requirement extends its closed operation set with `Divide` and `Modulo`.
- [FR-397](./FR-397-native-integer-division-wide-multiply-equality-and-charge-schedule.md) owns the plan-free division and modulus evaluators and the charge schedule entry, which the plan path must equal.
- [FR-405](./FR-405-kani-tractable-native-float-carrier.md) is the precedent for per-combination settlement, measured rows and the evidence record.
- `src/division.rs` owns `divide`, `modulo`, `DivisionProfile` and `DivisionMember`, the exact oracle of Behavior 7 and Behavior 21. [AD-003](../assurance/AD-003-bounded-integer-representation.md) records the admitted native API decision.
- CG AD-006 owns the native-route admission rule that this plan satisfies. CG FR-057 is the consumer. CG FR-028 owns the 16 GiB ceiling.
- IR-1526 owns the refusal seam and the tractability table that loads the measured rows.
- [TC-957](../test-cases/TC-957-native-integer-divide-modulo-plan.md) defines the verification.
