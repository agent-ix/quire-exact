---
id: AD-002
title: "Composable proof seam for checked equality effects"
type: ArchitectureDescription
status: proposed
owner: quire-exact-maintainer
system: quire-exact equality and the QSL checked-equality caller
relationships:
  - target: "ix://agent-ix/quire-exact/FR-375"
    type: realizes
---
# AD-002: Composable proof seam for checked equality effects

## System Boundary

The property crosses two crate owners. `quire-exact` owns `Value`, `EqualityPlan`, the private occurrence-pair walk, the exact Boolean relation, `Meter`, and all equality charge effects. QSL evaluation owns the checked comparison and its `CheckedEquality::evaluate` caller path. The assurance subject is the complete checked path: it must establish that the pair count predicted by `plan_equality` equals the number of `EqualityPair` effects admitted while the real checked evaluator returns the exact relation.

The qualification input is the finite depth-four `List` pair used by QSL's E21 equality test. Each operand contains 16 value occurrences; each evaluation forms 17 occurrence-path pairs. The two cases are equal operands and an adverse operand whose first head differs but whose shape and identity match. A separate identity-mismatch control fails checked construction; a separate work-limit control refuses during evaluation. This finite fixture is a harness contract, not a new general bound on public equality APIs.

## Views

The caller view, exact charge sequence, and proof-composition boundary below define one checked evaluation and its assurance limits.

### Runtime and effect view

The current kernel path in `src/equality.rs` is:

1. `planned_equality_stop` reads both operand occurrence counts and charges `equality.plan-form` with `max(left_occ, right_occ)` as `ValueOccurrences` and `left_occ + right_occ` as `WorkUnits`.
2. It calls private `plan_pairs`. That iterative walk uses `Integer` for its pair counter and returns the exact Boolean relation with the count. Its typed refusals include foreign references and checked-invariant failures.
3. After successful planning, it admits `equality.plan` for the complete pair count.
4. It calls `Meter::charge(equality.pair)` once per planned pair, stopping on the first refusal.
5. Only after every pair charge succeeds does it admit `equality.result-retain` and return the Boolean.

The `equality.plan` charge sets `ValueOccurrences` to the pair-count high-water mark, requires `pair_count + 2` remaining `WorkUnits` and one `ResultUnit`, then consumes one work unit. Each pair charge consumes one work unit. Result retention consumes one work unit and one result unit. Ordinary `Meter::charge` checks semantic-size counters in `LimitKind` field order, then cumulative work, then cumulative results, and admits nothing on refusal. `charge_plan` checks an injected named denial first, then pair-count `ValueOccurrences`, work reservation, and result capacity before mutating the meter.

The public `plan_equality` returns only `EqualityPlan::pair_events`; public `planned_equality` returns only `Outcome<bool>`. They share the same private walker, but callers cannot observe both the relation and count from the same invocation. The QSL E21 Kani test currently calls the actual `CheckedEquality::evaluate`, then observes the test-support meter's admitted charge log and compares its 17 pair entries with a separately obtained plan count. This is the same property and real caller route; it is not established by a plan-only proof.

### Decision: one kernel result-and-effect seam

The `quire-exact` owner SHALL expose a production API that returns the exact equality relation and the pair-event count from the same formed `PlannedPairs`, while the actual `Meter::charge` operations remain responsible for admitting effects. The API SHALL preserve the current `planned_equality` behavior for existing callers; the exact signature is implementation-owned. The QSL `CheckedEquality::evaluate` path SHALL invoke this API and preserve its existing `Outcome<bool>` result and operator handling.

This decision selects a result-bearing kernel seam because it keeps the relation and pair count tied to the single source-owned walk. A public caller-constructed plan, copied QSL walker, handwritten charge oracle, early-exit relation, or plan-only substitute would separate the property from the effects being asserted. The meter's admitted-charge log remains test-only evidence; production semantics continue to be the meter's ordered counters and typed refusal.

### Proof composition boundary

No cross-crate Kani contract is authorized by this decision. The issue evidence records no demonstrated automatic dependency-contract checking at the QSL-to-`quire-exact` boundary and no installed `--paranoid` option. `proof_for_contract`/`stub_verified` may be proposed only after an executable focused control crosses that exact crate boundary and proves both a passing case and a deliberately failing dependency-contract case. A same-crate proof or a toy control is not sufficient.

Until that control exists, the proof boundary is unsupported: the outer CBMC graph must retain the real `CheckedEquality::evaluate` caller, its conversion/admission path for the bounded operands, the call into `planned_equality` (or its result-bearing successor), private `plan_pairs` traversal, `Value::occ`, BigInt-backed `Integer` arithmetic for pair counts and meter amounts, and the `Meter::charge` / `charge_plan` effect logic. A kernel-local proof cannot by itself establish that QSL consumed the verified contract. If this full graph cannot meet the existing ceiling, record the tool boundary as a blocker; do not increase the ceiling, stub an unproved callee, weaken the relation/effect assertion, or claim proof credit from native tests.

The sound alternative is the same whole-call-graph proof with bounded, fixed-identity operands and the real caller, or a future separately reviewed proof architecture that preserves an end-to-end checked path and has its own demonstrated composition control. No such alternative is currently qualified. The focused cross-crate control is a prerequisite to choosing any contract-based decomposition, not evidence about the actual equality leaf.

## Decisions

| Decision | Choice | Alternatives and trade-off |
|---|---|---|
| Equality result and pair count ownership | One source-owned kernel walk returns both; actual meter calls admit the schedule. | Keeping separate APIs forces the caller to run a second walk to correlate relation/count; duplicating the walker or reconstructing charge counts in QSL risks semantic drift. |
| Refusal and charge order | Preserve plan-form, plan, each pair, result-retain; stop at the first typed refusal. | Prechecking a caller-computed plan or charging only after a separate verdict would change the observable order and is rejected. |
| Cross-crate Kani abstraction | No contract/stub until exact-boundary passing and failing tool controls exist. | Trusting a same-crate proof, toy contract, or unverified dependency summary cannot establish what the QSL caller consumes. Current trade-off is that the real callee stays in the outer graph and may remain intractable. |
| Input scope | Keep the IR-241 depth-four, 16-occurrence, 17-pair fixture and current proof ceiling. | Broader arbitrary-value guarantees or a larger unwind/resource ceiling are outside this proof slice. |

## Risks

- The exact QSL-to-kernel dependency-contract behavior is unqualified. Owner: QSL/Kani proof lane. Revisit only with the focused two-case cross-crate control described above.
- Existing Kani attempts reached fixture-key SHA initialization before establishing tractability of the remaining BigInt path. Removing or bypassing that path alone is not proof of the requested property. Owner: IR-241 proof lane; preserve the original ceiling and real caller.
- The API must report a completed pair count only when the same plan formed by the evaluated operands produced it; partial meter refusal must not be presented as a completed equality result or as all pair effects admitted. Owner: quire-exact implementation and independent review.
- A mismatched declaration identity is rejected by checked construction before `CheckedEquality::evaluate`; tests must distinguish that outer typed rejection from a kernel refusal reached only by invalid unchecked values.
