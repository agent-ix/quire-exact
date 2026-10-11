---
id: SR-6415
title: "Scope-boundary review of the equality seam"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-exact@702a13f5089b6ec0971e2ca5f8ac138e15887495; spec/functional/FR-375-composable-equality-effect-seam.md, spec/test-cases/TC-923-composable-equality-effect-seam.md, spec/assurance/AD-002-composable-equality-proof-seam.md"
review_set: subset
---

## Summary

Ownership is assigned to quire-exact for the kernel API and to QSL for CheckedEquality::evaluate; QSL’s actual caller path is an explicit requirement, while cross-crate Kani composition remains unsupported absent a control. No responsibility-allocation defect was found.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder). | - |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| AD-002 | examined | The property crosses two crate owners. `quire-exact` owns `Value`, `EqualityPlan`, the private occurrence-pair walk, the exact Boolean relation, `Meter`, and all equality charge effects. QSL evaluation owns the checked comparison and its `CheckedEquality::evaluate` caller path. The assurance subject is the complete checked path: it must establish that the pair count predicted by `plan_equality` equals the number of `EqualityPair` effects admitted while the real checked evaluator returns the exact relation. |
| AD-002 | examined | The `quire-exact` owner SHALL expose a production API that returns the exact equality relation and the pair-event count from the same formed `PlannedPairs`, while the actual `Meter::charge` operations remain responsible for admitting effects. The API SHALL preserve the current `planned_equality` behavior for existing callers; the exact signature is implementation-owned. The QSL `CheckedEquality::evaluate` path SHALL invoke this API and preserve its existing `Outcome<bool>` result and operator handling. |
| AD-002 | examined | The property crosses two crate owners. `quire-exact` owns `Value`, `EqualityPlan`, the private occurrence-pair walk, the exact Boolean relation, `Meter`, and all equality charge effects. QSL evaluation owns the checked comparison and its `CheckedEquality::evaluate` caller path. The assurance subject is the complete checked path: it must establish that the pair count predicted by `plan_equality` equals the number of `EqualityPair` effects admitted while the real checked evaluator returns the exact relation. |
| AD-002 | examined | No cross-crate Kani contract is authorized by this decision. The issue evidence records no demonstrated automatic dependency-contract checking at the QSL-to-`quire-exact` boundary and no installed `--paranoid` option. `proof_for_contract`/`stub_verified` may be proposed only after an executable focused control crosses that exact crate boundary and proves both a passing case and a deliberately failing dependency-contract case. A same-crate proof or a toy control is not sufficient. |
| AD-002 | examined | Until that control exists, the proof boundary is unsupported: the outer CBMC graph must retain the real `CheckedEquality::evaluate` caller, its conversion/admission path for the bounded operands, the call into `planned_equality` (or its result-bearing successor), private `plan_pairs` traversal, `Value::occ`, BigInt-backed `Integer` arithmetic for pair counts and meter amounts, and the `Meter::charge` / `charge_plan` effect logic. A kernel-local proof cannot by itself establish that QSL consumed the verified contract. If this full graph cannot meet the existing ceiling, record the tool boundary |
| AD-002 | examined | The sound alternative is the same whole-call-graph proof with bounded, fixed-identity operands and the real caller, or a future separately reviewed proof architecture that preserves an end-to-end checked path and has its own demonstrated composition control. No such alternative is currently qualified. The focused cross-crate control is a prerequisite to choosing any contract-based decomposition, not evidence about the actual equality leaf. |
| FR-375 | examined | The seam is owned by `quire-exact::equality`, where `plan_pairs` forms both the relation and the count, and `planned_equality` admits the charge schedule. The API SHALL be a result-bearing extension of `planned_equality` or an equivalent kernel API. `CheckedEquality::evaluate` SHALL use that real kernel path and expose its result through its existing `Outcome<bool>` behavior. |

## Verdict

PASS: no findings for this analysis.
