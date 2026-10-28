---
id: SR-15247
title: IR-705 bounded Multiply dependency
type: SpecReview
analysis: dependency
scope: agent-ix/quire-exact; PR79; spec/functional/FR-384-admitted-native-integer-refinement.md;
  spec/assurance/AD-003-bounded-integer-representation.md; spec/test-cases/TC-919-bounded-integer-representation-refinement.md
review_set: subset
---

## Summary

Independent review of exact a6ca2ec5eba997361535d68469e273c5eeb8752a against main a4e1fade. Prerequisite graph: FR-358/FR-362/FR-371 -> FR-384 -> consumer-owned CG FR-015 qualification. AD-003 realizes FR-371 and FR-384; TC-919 verifies both. The added verifies edge is not a runtime prerequisite and introduces no dependency cycle. Kernel-native capability is enablement; authentic consumer profile acceptance remains downstream. Specification merge precedes code and qualification; the failed unrestricted-i128 trials cannot discharge the amended capacity.

## Verdict

PASS — no defect found in the reviewed specification change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Examined scope

```yaml
scope:
- id: FR-384-AC-1
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Admission accepts every valid Add/Subtract/Negate domain whose complete
    interval image fits signed i128, retains full-i128 ordering, and accepts every
    valid Multiply interval pair whose four operand endpoints lie in signed32. It
    rejects any out-of-signed32 Multiply endpoint as a typed operation-capacity refusal
    before evaluator charges, including `[2147483648, 2147483648] * [0, 0]` and `[-2147483649,
    0] * [0, 0]`; payload/product zero or a semantic result restriction cannot conceal
    the unsupported interval. In-capacity Multiply image endpoints equal the exact
    four-corner signed64 minimum/maxim
- id: FR-384-AC-2
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: A machine-checked refinement against an independently expressed mathematical
    abstraction of the actual BigInt semantics covers every admitted native payload/operation
    under the operator-specific capacity policy and its abstraction to mathematical
    Integer/BigInt, including all signed32 multiplication pairs with widened signed64
    results and full admitted signed-i128 Add/Subtract/Negate/ordering capacities.
    Admission proof covers all valid in-capacity interval pairs and rejects out-of-signed32
    Multiply endpoints independently of actual payload and semantic result bounds.
    Independent BigInt differ
- id: FR-384-AC-3
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Fresh and cumulative real Meter comparisons against the normative formulas
    and general exact calls agree on completed values, domain refusal, cancellation,
    every named injected denial, all counters, admitted prefix and complete incomplete
    fields at sufficient, exact and one-less limits. Every operation and Boolean/comparison
    consumer used by the genuine profiles is exercised; arithmetic equivalence alone
    does not satisfy accounting refinement.
- id: FR-384-AC-4
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Source inspection and ordinary generated-client execution demonstrate native
    carriers persist through a multi-operation expression without successful-path
    BigInt/cache/general Integer construction. Input/output conversions are identified
    separately. Default and test-support checks, required no_std target and existing
    borrowed as_big regression checks pass without local unsafe or verification-only
    implementation.
- id: FR-384-AC-5
  path: spec/functional/FR-384-admitted-native-integer-refinement.md
  role: examined
  excerpt: Genuine CG TC-027 healthy, TC-027 bound mutant, TC-025 arithmetic mutant,
    IR-917 clause implication and IR-948 V2 contract profiles execute the reviewed
    production admitted API over their original inputs, domains and assertions. Healthy
    cases verify with their final completion covers at 16 GiB; each required substantive
    mutant falsifies with original-domain native playback. The IR-948 impossible-precondition
    control must complete as CoverUnsatisfied at the same ceiling, not resource exhaustion
    or a vacuous Verified. Unwind3, Cadical and existing 600-second ceilings remain
    unchanged where speci
- id: AD-003
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: examined
  excerpt: "---\nid: AD-003\ntitle: \"Bounded integer representation and borrowed\
    \ BigInt refinement\"\ntype: ArchitectureDescription\nstatus: proposed\nowner:\
    \ quire-exact-maintainer\nsystem: quire-exact Integer representation and its mathematical\
    \ observations\nrelationships:\n  - target: \"ix://agent-ix/quire-exact/FR-371\"\
    \n    type: realizes\n  - target: \"ix://agent-ix/quire-exact/FR-384\"\n    type:\
    \ realizes\n---\n# AD-003: Bounded integer representation and borrowed BigInt\
    \ refinement\n\n## System Boundary\n\nThis decision covers the private storage\
    \ representation of `quire-exact::Integer`, its exact refinement to mathemati"
- id: TC-919
  path: spec/test-cases/TC-919-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For [FR-384](../functional/FR-384-admitted-native-integer-refinement.md),
    separately test certificate admission, complete native-capacity operation refinement
    and chained real-Meter behavior before consumer qualification. Use independently
    calculated BigInt interval endpoints and results, exact/one-less limits, cancellation
    and injected denials. Check full signed-i128 Add/Subtract/Negate capacities wherever
    their complete result image fits, and full-i128 ordering. For Multiply, quantify
    over every signed32 operand pair and all valid in-capacity interval pairs, deriving
    the exact four-corner si
```

## Coverage

Source-only review; no Cargo, resolver, compiler, Kani, aggregate, namespace or host security command executed. No applicable AssuranceProfile found. Criterion-strength/Jev not selected because no new criterion or weakness claim requires it. Prerequisite graph: FR-358/FR-362/FR-371 -> FR-384 -> consumer-owned CG FR-015 qualification. AD-003 realizes FR-371 and FR-384; TC-919 verifies both. The added verifies edge is not a runtime prerequisite and introduces no dependency cycle. Kernel-native capability is enablement; authentic consumer profile acceptance remains downstream. Specification merge precedes code and qualification; the failed unrestricted-i128 trials cannot discharge the amended capacity.
