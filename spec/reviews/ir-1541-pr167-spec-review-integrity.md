---
id: SR-17940486709564847047992
title: "IR-1541 PR #167 spec-review integrity"
type: SpecReview
analysis: integrity
date: 2026-11-14
scope: "agent-ix/quire-exact PR #167 at 12323e22, git diff origin/main...HEAD; IR-1541; spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md, spec/test-cases/TC-957-native-integer-divide-modulo-plan.md, spec/functional/FR-384-admitted-native-integer-refinement.md; cross-checked against quire-contract-codegen main f0a1cce4 and PR #835 head f585f1a4 (FR-057, FR-014-AC-81, FR-056, AD-006)"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-exact/FR-409
    type: reviews
  - target: ix://agent-ix/quire-exact/FR-384
    type: reviews
---

## Summary

Ticket: IR-1541. Cross-document consistency of FR-409 with FR-384, FR-397, FR-405 (precedent), and with what CG consumes (FR-057 in PR #835, FR-014, FR-056, AD-006, IR-1526).

What CG consumes is satisfiable: FR-057 needs `NativeIntegerPlan::new(Divide { profile, member, .. })` / `Modulo` built from i64 constant endpoints, the quotient and remainder members under each law, and the native evaluators' outcomes; FR-409 supplies all of these. CG never reaches MIN/-1 (i64 operands; inferred `[-2^63, 2^63]` is refused `BoundNotI64` at lowering). Plan capacities match CG FR-014's inferred intervals, so CG's capacity chaining (`completed_interval(plan.capacity(), ..)`) is unaffected. FR-409's `w` is the signed width of CG FR-056 (`[-2^(n-1), 2^(n-1)-1]` is n bits). CG main (`git grep` at f0a1cce4) only constructs `NativeIntegerOperation` variants (src/oracle/scalar/native.rs, src/oracle/clause/native.rs, generated strings in src/kani/generate/clause/v2.rs) and has no `match` on `NativeIntegerOperation` or `NativeIntegerOperationKind`, so adding variants does not break CG. `quire validate` passes.

## Verdict

Mergeable on integrity after the medium fixes; no high integrity finding. See the base SR for the blocking findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The FR-384 edit adds Divide and Modulo to FR-384's operation set without scoping FR-384's own obligations, which now read over them: "SHALL establish a compositional refinement from every admitted carrier and operation" (FR-384-AC-2 machine-checked refinement) and the FR-362 operands/arithmetic/result-retain charge sequence. FR-409 allows measured not-tractable rows instead of proofs and uses the four `integer-division.*`/`integer-modulus.*` points. Say in the same sentence that FR-384's refinement and charge obligations stay over Add/Subtract/Multiply/Negate and FR-409 owns Divide/Modulo evidence and charges. | spec/functional/FR-384-admitted-native-integer-refinement.md:45, :61, :69, :78 |
| FND-002 | medium | Open item "Remainder for CG rem" is stale against the consumer: it says CG FR-057 keeps `rem` Underived and "CG needs no remainder from this plan now", but FR-057 in PR #835 makes `rem` derivable inside IR-854 (FR-014-AC-81) and its native route calls `evaluate_native_integer_division(.., Remainder, ..)` through the plan, so FR-057-AC-7's native `rem` verdict needs the remainder member and its rows. Restate as a current consumer need. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:131 |
| FND-003 | medium | Behavior 20 / AC-8 make a QE acceptance depend on writing rows into IR-1526's CG table, keyed by operation kind, profile, member and divisor shape so that "the seam refuses that capability". The FR-056 seam keys on catalog operation ids, width and route and has no profile, member or constant-divisor feature, and FR-057 forbids shipping a row that refuses a shape measured Verified. A profile- or shape-specific kernel row is therefore either not loadable or over-refuses. FR-405 (precedent) keeps no criterion dependent on the table. Make TC-957's evidence record the QE acceptance and hand the table load to IR-1526/CG. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:101, :125; spec/test-cases/TC-957-native-integer-divide-modulo-plan.md:26 |
| FND-004 | low | Open item "Exhaustive matches" leaves a pre-merge check to the coder that is already answerable: CG main has no exhaustive match on either enum (constructions only). Record the result and close the item. | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:135 |

## Scope

| Unit | Role | Excerpt |
| --- | --- | --- |
| FR-409 Description | examined | The consumer is CG FR-057 (IR-854, AC-7 and AC-8 on the native route). CG's harness, its measurements and its credit are CG's |
| FR-409 Open items | examined | Remainder for CG rem; MIN / -1 with a result interval; Capability name format; Constant-divisor flag; Exhaustive matches |
| FR-409 Dependencies | examined | FR-384 owns the native carrier, the plan, admission and the existing four operations. |
| FR-409-AC-8 | examined | Each measured row appears in TC-957's evidence record and in the tractability table that IR-1526's seam loads. |
| FR-384 capability table row | examined | FR-409 adds checked Divide and Modulo operations to this set. |
| FR-384 refinement paragraph | context_only | The implementation SHALL establish a compositional refinement from every admitted carrier and operation to the general mathematical Integer/BigInt semantics. |
| FR-405 Dependencies | context_only | The table has no versioned home of its own; TC-953's evidence record is the versioned copy of each row's essentials, and no criterion depends on the table's path. |
| CG FR-057 Routes | context_only | When the plan exists, the native oracle SHALL call the native evaluators through it |
| CG FR-056 width | context_only | two's complement signed n (for [-2^(n-1), 2^(n-1)-1]) |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | low | FR-409 Description says CG FR-056's table "carries native rows with an optional route field"; FR-056 on CG main (198df58f, PR #835 merged) says each row "carries its required `route` field". Change "optional" to "required". | spec/functional/FR-409-kernel-checked-native-integer-divide-and-modulo-plan.md:30 |

## Dispositions

Round 1, fix head 6b9b9104 (reviewed head 12323e22), checked against CG main 198df58f where PR #835 (FR-057, FR-056) is merged. FR-057-AC-9 and FR-056 now state that native-route rows from FR-409 / TC-957 are copied by CG and quire-exact ships no second table, matching FR-409 Behavior 20 and AC-8.

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | FR-384 row now says its refinement obligation and FR-362 charge sequence stay over the four operations and FR-409 owns Divide/Modulo evidence and charges. |
| FND-002 | fixed | Description and Open items state rem is a live consumer need via FR-057 / FR-014-AC-81; no member combination is optional. |
| FND-003 | fixed | Behavior 20, AC-8 and TC-957 step 7 make TC-957's evidence record the kernel's only record; CG copies rows into FR-056's table, consistent with merged FR-056 and FR-057-AC-9. |
| FND-004 | fixed | Exhaustive-matches open item resolved with the CG finding (constructions only, no CG change). |
