---
id: FR-384
title: "Admitted native integer evaluation refines arbitrary precision"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-371
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-362
    type: depends_on
  - target: ix://agent-ix/quire-exact/FR-358
    type: depends_on
---
# FR-384: Admitted native integer evaluation refines arbitrary precision

## Description

Where a caller selects the additive admitted native integer API, the kernel SHALL evaluate only certified finite integer operations and preserve the mathematical result, real Meter accounting and refusal semantics of its arbitrary-precision API.

This requirement does not replace [FR-371](./FR-371-bounded-integer-representation-refinement.md). General `Integer`, its Small64/Small128/BigInt representation, and its borrowed `as_big` contract retain their existing behavior. The explicit admitted API is a production kernel facility, available under ordinary builds, rather than a verification-only arithmetic implementation.

## Behavior

### Admission and mathematical interpretation

The kernel SHALL own construction of the admitted execution capability and native value carrier. The native carrier SHALL be Copy and contain an i128 payload, with no general Integer, BigInt or lazy cache. Its mathematical interpretation is the exact signed i128 payload. A caller cannot construct an unchecked capability, widen its certified domain, or relabel another operation's certificate through public fields.

Before execution, the kernel SHALL check the requested finite operand domains and each operation's entire mathematical result interval against its native capacity. Admission SHALL include every reachable intermediate, rather than only the expression's final result domain. Add, Subtract, Multiply and Negate use exact conservative interval transfer; Multiply includes all four endpoint products. An input domain is distinct from an optional result-domain restriction. Admission SHALL NOT narrow the requested input population, invent a result-domain restriction or add a harness assumption.

The kernel SHALL derive one checked operation plan from the requested operand intervals, operation and optional semantic result domain. Add/Subtract compute endpoint sums/differences, Multiply computes the minimum/maximum of all four endpoint products, and Negate reverses/negates endpoints with checked i128 arithmetic. When a calculation cannot be represented, admission SHALL refuse rather than narrow. For each evaluation, the plan SHALL check operand membership and return an opaque Copy admitted operation that retains the checked plan association. Only that admitted operation is accepted by the native evaluator; combining values with a different plan cannot bypass membership or operation identity. Admission failure occurs before any evaluator charge.

A capacity failure SHALL return a typed admission refusal identifying the offending operation or domain. It SHALL NOT become an arithmetic domain refusal, a Meter denial, a successful truncated result, or a proof result. When bounded admission is unavailable, an existing general exact route remains available subject to the caller's existing capability policy; callers cannot report that route verified merely because admission failed.

### Public capability shape

The public contract SHALL expose these distinct roles (concrete enum variant names may follow repository conventions):

| Role | Contract |
| --- | --- |
| `NativeInteger` | Opaque Copy i128 payload; construction from i128 preserves every payload without granting an operation capability; `value()` returns its exact i128 observation and `to_integer()` explicitly constructs the existing general Integer from that value. No `as_big` method or hidden retained cache. |
| Native interval | Validated inclusive i128 minimum/maximum, with a fallible conversion from general mathematical intervals. Reversed endpoints and nonrepresentable mathematical endpoints are distinct typed admission failures. |
| Closed interval-operation input | Exactly Add(two intervals), Subtract(two intervals), Multiply(two intervals), or Negate(one interval). It cannot encode a binary operation with a missing operand or an unsupported division operation. |
| `NativeIntegerPlan::new` | Accepts the closed interval-operation and optional semantic result interval; computes the capacity interval itself. No caller-supplied unchecked result/capacity certificate is accepted. |
| Plan operand admission | Accepts a closed unary/binary native operand shape, checks arity and membership in the plan's actual operand intervals before any Meter charge, and returns an opaque Copy admitted operation or a typed admission failure. |
| Opaque admitted operation | Retains/copies its exact validated plan, operation and payloads. Evaluation cannot replace the plan after admission and returns existing `Outcome<NativeInteger>` through the actual public Meter. |
| Native comparison | Kernel-owned ordering/comparison returns existing `Outcome<bool>` under its unchanged normative Meter schedule; the runtime's existing lazy Boolean helper remains its own independent call. |

The native interval constructors, checked plan constructor and per-call operand admission SHALL use only bounded native arithmetic and storage on their successful paths. A general Integer/BigInt interval conversion is an explicit ingress operation outside that execution path; moving the general representation or cache into plan construction does not meet this contract. Emitted clients may reconstruct plans from native constant endpoints through the same checked production constructor, rather than an unchecked public certificate constructor.

The optional semantic result interval must itself be representable by the native interval role. An unrepresentable semantic interval is a capability refusal, not a narrowed substitute interval. Even when the mathematical result fits the capacity interval, exclusion from that separate semantic interval produces the existing semantic domain refusal during evaluation.

### Execution and accounting

For an admitted operation and values within its certified input domains, the kernel SHALL produce the same mathematical completed value as the general exact operation. Native overflow SHALL never wrap or saturate. A runtime breach of a certified carrier/domain invariant SHALL be a typed failure, without a fabricated completed value or BigInt approximation.

For a valid admitted operation, native evaluation SHALL return the ordinary Outcome with the native carrier as its completed payload. A semantic result-domain refusal SHALL retain the existing typed IntegerOutOfDomain target payload and occur after the arithmetic charge and before result-retain, even though that refusal may allocate an ordinary target representation.

The admitted API SHALL use the public production `Meter`, its actual counters, cancellation checks and charge mechanism. It SHALL preserve the operands, arithmetic and result-retain sequence and exact amounts of [FR-362](./FR-362-exact-scalar-arithmetic-and-atom-charges.md), including cumulative work/results, semantic-size maxima, cancellation and injected-denial precedence. A declared result domain excluding the exact result SHALL return the same domain refusal after the same admitted prefix. The carrier's representation capacity is not that declared domain. Admission work is a separate ingress action and SHALL NOT manufacture semantic evaluator charges.

Chained admitted operations SHALL retain the native carrier without conversion through `Integer`, BigInt, a lazy cache or a general representation discriminator on the successful evaluation path. Explicit ingress/egress conversion at the caller boundary is permitted and SHALL preserve value. The carrier SHALL NOT expose an `as_big` borrow requiring hidden retained allocation. The existing `Integer::as_big` signature and borrow lifetime remain unchanged.

Boolean and comparison consumers SHALL use kernel-owned operations over the admitted native values, with the same comparison/Boolean charge and refusal contracts as their general counterparts. A generator cannot copy arithmetic, Meter logic or comparison semantics into a private evaluator. A separately authored native expectation used only as a test oracle is not a production evaluator.

### Refinement and evidence

The implementation SHALL establish a compositional refinement from every admitted carrier and operation to the general mathematical Integer/BigInt semantics. The proof obligation quantifies over the complete accepted native capacity and certified operand domains; fixed examples, the small IR-917 fixture range, or a narrower test-only constructor cannot supply that obligation. Meter refinement SHALL cover all counters, each refusal point and complete observable incomplete records, including cumulative execution on the same actual Meter.

The implementation SHALL retain default/test-support and no_std plus alloc compatibility and the local unsafe prohibition. It SHALL introduce no cfg(kani) replacement arithmetic, assumed arithmetic identity, copied kernel, skipped healthy proof, or success classification for unknown/resource-exhausted evidence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-384-AC-1 | Admission accepts valid finite Add/Subtract/Multiply/Negate domains and rejects reversed ranges, nonrepresentable endpoints and overflowing intermediate result intervals with distinct typed reasons. Boundaries cover native MIN/MAX, zero, -1, singleton domains and multiplication sign combinations. A narrowing final domain does not conceal an unrepresentable intermediate. Public clients cannot forge or widen certificates. | Test |
| FR-384-AC-2 | A machine-checked refinement against an independently expressed mathematical abstraction of the actual BigInt semantics covers every admitted native payload/operation and its abstraction to mathematical Integer/BigInt. Independent BigInt differential tests exercise extrema, cancellation, products and negation plus reproducibly sampled accepted domains; deliberate arithmetic and admission-bound mutants falsify the relevant property. Evidence states exactly which obligation each method establishes. | Test |
| FR-384-AC-3 | Fresh and cumulative real Meter comparisons against the normative formulas and general exact calls agree on completed values, domain refusal, cancellation, every named injected denial, all counters, admitted prefix and complete incomplete fields at sufficient, exact and one-less limits. Every operation and Boolean/comparison consumer used by the genuine profiles is exercised; arithmetic equivalence alone does not satisfy accounting refinement. | Test |
| FR-384-AC-4 | Source inspection and ordinary generated-client execution demonstrate native carriers persist through a multi-operation expression without successful-path BigInt/cache/general Integer construction. Input/output conversions are identified separately. Default and test-support checks, required no_std target and existing borrowed as_big regression checks pass without local unsafe or verification-only implementation. | Test |
| FR-384-AC-5 | Genuine CG TC-027 healthy, TC-027 bound mutant, TC-025 arithmetic mutant, IR-917 clause implication and IR-948 V2 contract profiles execute the reviewed production admitted API over their original inputs, domains and assertions. Healthy cases verify with their final completion covers at 16 GiB; each required substantive mutant falsifies with original-domain native playback. The IR-948 impossible-precondition control must complete as CoverUnsatisfied at the same ceiling, not resource exhaustion or a vacuous Verified. Unwind3, Cadical and existing 600-second ceilings remain unchanged where specified. Compiler failure, unknown, timeout, MemoryExhausted, missing mutant or missing teardown is inconclusive. Larger diagnostic budgets establish no 16 GiB fit. | Test |

## Dependencies

[AD-003](../assurance/AD-003-bounded-integer-representation.md) records the additive admitted API decision and its proof boundary. Consumer-owned [CG FR-015](ix://agent-ix/quire-contract-codegen/FR-015) owns authentic CheckedPackageV2 generation and proof profiles; kernel refinement cannot replace those consumer runs. This requirement is prospective until implementation, independent review and actual qualification evidence land.
