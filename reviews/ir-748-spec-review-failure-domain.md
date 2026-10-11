---
id: SR-6410
title: "Failure-domain review of the equality seam"
type: SpecReview
analysis: failure-domain
scope: "agent-ix/quire-exact@702a13f5089b6ec0971e2ca5f8ac138e15887495; spec/functional/FR-375-composable-equality-effect-seam.md, spec/test-cases/TC-923-composable-equality-effect-seam.md, spec/assurance/AD-002-composable-equality-proof-seam.md"
review_set: subset
---

## Summary

Identity, bounded recursive input, typed kernel refusals, meter refusal, and the unsupported cross-crate contract boundary are explicit. No additional unstated failure domain was found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder). | - |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-375 | examined | When a checked equality caller evaluates two admitted operands, the `quire-exact` kernel SHALL expose the exact equality result and the count of `equality.pair` charges admitted by that same evaluation, without a second caller-side equality implementation or a caller-authored charge oracle. |
| FR-375 | examined | For the bounded IR-241 qualification input, operands are the two separately constructed depth-four `List` values in `qsl-eval/tests/it/equality_matrix.rs::e21_operands`: each has `occ() == 16` and the pair plan has 17 events. The input is fixed and finite; the seam does not promise a bound for arbitrary recursive or caller-supplied values. Admission of the input and its composite declaration identity precedes evaluation. |
| FR-375-AC-4 | examined | \| FR-375-AC-4 \| With equal-shaped operands whose composite declaration identities differ, checked construction refuses with `IllTypedCause::TypeMismatch` before `CheckedEquality::evaluate`; the evaluator is not called and no equality pair charge is admitted. \| Test \| |
| FR-375-AC-5 | examined | \| FR-375-AC-5 \| With `work_units = 50` and the bounded equal pair, the actual checked evaluation admits plan-form work of 32, then refuses `equality.plan` because its atomic reservation requires 19 remaining work units but only 18 remain. It returns the existing typed `Incomplete` at `WorkUnits` with limit 50, consumed 32 and `next_charge = 19`; no plan, pair, or result-retain charge is admitted and no equality result is returned. \| Test \| |
| FR-375-AC-7 | examined | \| FR-375-AC-7 \| No cross-crate Kani contract, stub, or assumed postcondition is treated as checked unless a focused tool control demonstrates both a passing and a deliberately failing dependency-contract case through the exact QSL-to-quire-exact dependency boundary. Until that evidence exists, the proof status is unsupported at that boundary and the real callee remains in the outer CBMC graph. \| Inspection \| |
| AD-002 | examined | - A mismatched declaration identity is rejected by checked construction before `CheckedEquality::evaluate`; tests must distinguish that outer typed rejection from a kernel refusal reached only by invalid unchecked values. |
| AD-002 | examined | The property crosses two crate owners. `quire-exact` owns `Value`, `EqualityPlan`, the private occurrence-pair walk, the exact Boolean relation, `Meter`, and all equality charge effects. QSL evaluation owns the checked comparison and its `CheckedEquality::evaluate` caller path. The assurance subject is the complete checked path: it must establish that the pair count predicted by `plan_equality` equals the number of `EqualityPair` effects admitted while the real checked evaluator returns the exact relation. |
| AD-002 | examined | No cross-crate Kani contract is authorized by this decision. The issue evidence records no demonstrated automatic dependency-contract checking at the QSL-to-`quire-exact` boundary and no installed `--paranoid` option. `proof_for_contract`/`stub_verified` may be proposed only after an executable focused control crosses that exact crate boundary and proves both a passing case and a deliberately failing dependency-contract case. A same-crate proof or a toy control is not sufficient. |
| AD-002 | examined | Until that control exists, the proof boundary is unsupported: the outer CBMC graph must retain the real `CheckedEquality::evaluate` caller, its conversion/admission path for the bounded operands, the call into `planned_equality` (or its result-bearing successor), private `plan_pairs` traversal, `Value::occ`, BigInt-backed `Integer` arithmetic for pair counts and meter amounts, and the `Meter::charge` / `charge_plan` effect logic. A kernel-local proof cannot by itself establish that QSL consumed the verified contract. If this full graph cannot meet the existing ceiling, record the tool boundary |
| AD-002 | examined | The sound alternative is the same whole-call-graph proof with bounded, fixed-identity operands and the real caller, or a future separately reviewed proof architecture that preserves an end-to-end checked path and has its own demonstrated composition control. No such alternative is currently qualified. The focused cross-crate control is a prerequisite to choosing any contract-based decomposition, not evidence about the actual equality leaf. |

## Verdict

PASS: no findings for this analysis.
