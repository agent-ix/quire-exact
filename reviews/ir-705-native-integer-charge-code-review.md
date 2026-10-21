---
id: SR-14760
title: IR-705 native integer charge code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; IR-705; src/numeric.rs; FR-362; FR-371
review_set: subset
---

## Summary

Code-review including the Rust lane examined the full numeric.rs production/test diff and the unchanged public arithmetic, Integer magnitude/construction and exact Meter admission continuation. The shared private helper widens both u64 counts to i128 before addition. For every possible pair, 0 <= sum <= 2^65-2 < i128::MAX. Add/Sub max+1 is at most2^64; Multiply's sum at most2^65-2. Negate remains unchanged. No new public seam, arbitrary-precision replacement, assumption, unchecked narrowing, clamp, saturation, panic, unsafe or gate change.

Ticket: IR-705. Frozen source head: 62771adf9d5bc8d62136ef85d994897a3906789b; exact published base: 4550051ee681d40267badaf870495c9ccff239a6. Independent reviewer; static inspection and Quire only. No Cargo, rustc, formatter, metadata, native, Kani, CBMC or aggregate invocation by reviewer. Runtime model: gpt-6.1-sol; run: 468e6c08-4278-416e-8a4a-ce36f75fe17b. Public repository; findings posted only to private Linear. No applicable AssuranceProfile under spec/. No product/spec edits or branch pushes. No spec/plan diff, so spec-review sub-analyses do not apply.

## Verdict

**CONDITIONAL**: the exact metadata implementation is sound, but the new test binding must be repaired before merge. Native max on u64 magnitudes and exact native widened addition preserve the previous mathematical Integer amounts for Small, overflowing-native and genuine Large operand representations. Actual value operations and BigInt continuation are unchanged, as are exact_size, ChargeAmount::Exact retention above u64MAX, charge order, domain checks, counters, denial and incomplete payloads. This removes an avoidable generic operation on metadata; it establishes neither elimination of all BigInt/drop machinery nor cause of either OOM nor a healthy proof verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The new boundary test uses a qualified ix_trace_rs::trace attribute that the installed matrix engine does not bind. It is absent from FR-371-AC-3 binders and appears as an unmatched tag, so the new evidence is not discoverable through the repository traceability gate. Use the existing imported trace attribute convention and verify the computed binding. | src/numeric.rs:460-462 |

## Coverage

Read AGENTS/CLAUDE, Rust style, numeric source, accounting exact admission, Integer conversion/magnitude implementation, FR-362, FR-371 and the independently prepared next-architecture-intake.md. Searched src and tests for a preexisting equivalent helper and existing fixture setup: integer_bit_sum has one production home used consistently by Add/Sub and Multiply. Shared bits, ordering/rational formulas and Negate remain unchanged. No vendored dependency implementation, copied spec, extra manifest/provenance machinery, new dependency, lint waiver or workflow change.

The added real #[test] uses independent fixed decimal expectations plus BigInt arithmetic for (0,0), (MAX-1,1), (MAX,1), (1,MAX), (MAX,MAX). It passes actual exact_size requests to a real sufficient Meter, asserts success at u64MAX, and asserts typed charge point, kind, limit, consumed, exact next_charge and unchanged relevant counters above it. The guarded success addition is evaluated only after exact independent expected amount assertions and only when the sum fits u64; the refusal branch is explicit. Thus replacing production addition with wrap/clamp/narrowing fails meaningful assertions. No impossible giant operand or public test-only seam is invented. The binding to FR-371-AC-3 is supplemental boundary evidence, not a claim this private helper alone completes its all-public-operation criterion.

Unchanged public tests inspected: explicit 8+3 and reversed operands, cancellation subtraction, multiply exact and one-less limits; all four operations over native, overflow and 201-bit Large values; exact/one-less bit and work boundaries, every named denial prefix, result/domain outcomes and no fabricated FunctionCall. Existing independent formulas and fixed result strings remain intact. FR-362-AC-19 does not discharge all of AC-10 or negate. No async, locking, task, wire, persistence or public trait boundary is changed.

Author raw focused round2 receipts/logs inspected, not rerun: fmt/fmt-check exit0, make lint exit0 across default/test-support/bare-metal; numeric unit3 each; three integration bins26 each configuration, all pass/no ignored. Initial test compile error using point rather than charge_point is preserved in original logs and fixed in final source. Actual successful scope memory.max17179869184, swap0, peak643219456, all memory events0; that scope peak is not per-test. Cargo manifests/lock are byte unchanged. Final aggregate remains pending for the lead. Both original unchanged consumer healthy measurements (d981 and4550) exhausted16GiB without a verdict. Neither this code review nor native success satisfies FR-359-AC-10/FR-371-AC-6.

## Review data

```yaml
clean: false
scope:
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For a successful representative of each integer arithmetic operation, each rational arithmetic operation,
    integer ordering and rational ordering, the admitted point sequence equals its family row in Behavior. A successful
    already-decided Boolean retention admits only `boolean.result-retain`.
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate the maximum size amount across
    their points from the formulas in Behavior, using the unreduced numerator and denominator for rational normalization.
    The observed final `integer_bits` and `value_occurrences` equal those maxima; work consumption equals the number
    of admitted points and result consumption is one.
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Inject a one-shot denial at each named point in representative integer arithmetic, rational arithmetic,
    integer ordering, rational ordering and already-decided Boolean retention calls. Each returns `Incomplete` naming
    the denied point, with no completed value, no result unit for the denied charge and no later point admitted
    by that call.
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For integer add, subtract, multiply and negate, rational add, subtract, multiply, divide and negate,
    and integer/rational ordering, independently calculate the required bit high-water amount from Behavior. On
    fresh meters with other limits sufficient, that exact bit limit completes with the expected value, while a limit
    one below stops at the first point whose amount exceeds it, with exact prior consumption, request and admitted
    prefix and no result unit. Include cancellation `1000 - 999`, `255 * 255`, rational `(2/3) * (3/2)`, `(3/4)
    / (5/7)`, `(5/7) - (4/7)`, negate `-5/8`, and `7/3 < 5/8`. '
- id: FR-362-AC-19
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'Through public `evaluate_integer_arithmetic` and a fresh public `Meter` for each call, add `8 + 3` and
    `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`, without a result bound and with all non-bit limits sufficient.
    The independently calculated `integer-arithmetic.arithmetic` bit amounts are respectively 5, 5, 5 and 6: add/subtract
    use `max(B(a), B(c)) + 1`, including the larger right operand and a result that cancels to one bit; multiply
    uses `B(a) + B(c)`. At each exact `integer_bits` limit, the call completes to respectively 11, 11, 1 and 24,
    admits operands, arithmetic and result-retain in '
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native,
    native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit
    and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete
    fields and admitted prefixes with the independent normative formulas. No representation transition manufactures
    a FunctionCall or other charge.
findings:
- fnd: FND-001
  severity: medium
  confidence: high
  method: code-review
  check: coverage
  artifact_id: FR-371-AC-3
  related_ids: []
  path: src/numeric.rs
  lines: 460-462
  excerpt: Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native,
    native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit
    and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete
    fields and admitted prefixes with the independent normative formulas. No representation transition manufactures
    a FunctionCall or other charge.
  finding: The new boundary test uses a qualified ix_trace_rs::trace attribute that the installed matrix engine
    does not bind. It is absent from FR-371-AC-3 binders and appears as an unmatched tag, so the new evidence is
    not discoverable through the repository traceability gate. Use the existing imported trace attribute convention
    and verify the computed binding.
bindings:
- test_id: src/numeric.rs::tests::integer_bit_counts_preserve_exact_amounts_above_the_meter_limit
  ac_id: FR-371-AC-3
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::metered_native_and_overflow_operations_keep_the_existing_charge_schedule
  ac_id: FR-371-AC-3
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::metered_operation_matrix_preserves_fr362_amounts_work_and_denial_prefixes
  ac_id: FR-371-AC-3
  trace: correct
- test_id: tests/ir653_scalar.rs::public_integer_arithmetic_admits_exact_bits_and_denies_one_under
  ac_id: FR-362-AC-19
  trace: correct
```

## Disposition round 1

Rechecked exact clean head acb80345abace4ca2fb70f1f63c058ea39656327 against initial62771ad. Only test-module import and attribute spelling change; production/helper and all test assertions are byte unchanged. Independent actual recomputed matrix binds src/numeric.rs:464 tests::integer_bit_counts_preserve_exact_amounts_above_the_meter_limit plus the two existing public tests to FR-371-AC-3. All188 criterion statements/statuses remain identical to separately archived4550; current binder total253. Original review/findings remain unedited above. Actual root currentPR68 decision and all three actual continuing owner comments read back. PreviousPR67 deferrals are not reused as current disposition authority. Focused trace-fix fmt/check and exact unit1 each configuration PASS, actual16GiB/swap0 peak257134592/events0; manifests/lock unchanged,7guard release empty. Reviewer ran Quire only, no heavy invocation.

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Imported trace macro and computed exact symbol now resolves under FR-371-AC-3. |

```yaml
dispositions:
- fnd: FND-001
  outcome: fixed
  after_excerpt: "    #[test]\n    #[trace(\"FR-371-AC-3\")]\n    fn integer_bit_counts_preserve_exact_amounts_above_the_meter_limit()\
    \ {"
```

## Current Disposition Verdict

**PASS** for focused code/Rust review after verified trace fix; no further findings. Exact-head final aggregate remains required.
