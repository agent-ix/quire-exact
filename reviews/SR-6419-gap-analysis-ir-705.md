---
id: SR-6419
title: gap-analysis of IR-705 bounded integer implementation
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-exact@6949844a942da9446bd76a6ae5dbf7c9a3eef3b9; Cargo.toml, Cargo.lock, src/accounting.rs, src/decimal_membership.rs, src/integer.rs, tests/ir705_bounded_integer.rs"
review_set: subset
relationships:
  - {type: references, target: "ix://agent-ix/quire-exact/FR-371"}
  - {type: references, target: "ix://agent-ix/quire-exact/TC-919"}
---

## Summary

Ticket: IR-705, PR #36 at the reviewed head. Plan completion: not assessed. The computed Quoin matrix contains 138 criteria: 107 tagged, 18 untagged, and 13 with a method but no symbol. For the changed FR-371 acceptance set, AC-1 through AC-5 have trace bindings; AC-6 has no local test binding because its required genuine healthy and mutant consumer runs belong to the downstream CG lane. The PR accurately says that consumer qualification is pending. The source paths reviewed are not stubs and the test oracles inspected are substantive, but AC-1, AC-3 and AC-4 have the concrete evidence gaps recorded in the companion code-review artifact. This is a scoped PR audit; the other untagged matrix entries include existing criteria outside this change.

## Verdict

FAIL for gap-analysis because FR-371-AC-6 remains untested/unqualified in this repository. This does not mean the PR falsely claims qualification; the ticket stays open until the external consumer evidence lands. No optional semantic sub-review was requested or run.

## Reviewed scope

- FR-371-AC-1 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:79`): Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve exact values across fixed boundaries and reproducible samples, with independent fitting and non-fitting oracles and malformed spelling refusal.
- FR-371-AC-2 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:80`): Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal, identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt access and cloning.
- FR-371-AC-3 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:81`): Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native, native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit and work boundaries, denial injected at every named point, and outside-domain results.
- FR-371-AC-4 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:82`): For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt construction/materialization.
- FR-371-AC-5 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:83`): Public signatures and all existing Integer methods remain available, except that the divisor parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain &Integer parameters.
- FR-371-AC-6 (examined, `spec/functional/FR-371-bounded-integer-representation-refinement.md:84`): Genuine consumer healthy TC027 and its bound mutant plus TC025 arithmetic mutant must complete with original bounds, oracle, limits and cleanup; compiler/resource failure is inconclusive.
- FR-372-AC-5 (context_only, `spec/functional/FR-372-fallible-bounded-decimal-membership.md`): Existing repository baseline criterion outside the IR-705 changed acceptance set; matrix reports it untagged.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-371-AC-6 has no local binding or consumer qualification receipt at this head. | `spec/functional/FR-371-bounded-integer-representation-refinement.md:84`; `tests/ir705_bounded_integer.rs` |

### FND-001 — consumer qualification remains an open evidence gap

The computed matrix reports FR-371-AC-6 with zero binders. This criterion requires a genuine downstream consumer healthy TC027 run and two substantive mutant falsifications under unchanged resource and oracle constraints. No such result is present in this repository/head; the PR body explicitly leaves this work pending. Keep the criterion and IR-705 open until the consumer owner records all three terminal outcomes against the exact published/adopted head. Do not replace the consumer harness with a local duplicate or count a compile/list receipt as proof.

## Coverage and method limits

The matrix reports 138 criteria overall (107 tagged, 18 untagged, 13 method-without-symbol). Besides FR-371-AC-6, the computed untagged set includes FR-372-AC-5 and pre-existing FR-369/FR-374/FR-375 criteria outside this PR's changed acceptance. No whole-repository closure is claimed. Plan completion: not assessed. Code/test alignment was manually checked for the scoped trace tags; optional semantic analysis was not run. No builds or tests were run by this reviewer. Kani execution is pending and receives no proof credit.
