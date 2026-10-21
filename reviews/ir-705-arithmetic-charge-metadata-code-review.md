---
id: SR-15231
title: IR-705 PR75 integer-arithmetic charge metadata code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; PR75 frozen2e2edf1; src/numeric.rs and unchanged Meter/public
  arithmetic integration seams
review_set: subset
---

## Summary

Ticket: IR-705; product PR quire-exact#75. Independent reviewer distinct from coder ir705_native_parse_coder. Exact clean frozen head2e2edf1fad5f8d6d8bf52a9fb1c01d5cfdf2adde; published base982633b93828f39d012a1ff5a8ca2ea5a1440616. Full diff changes ONLY src/numeric.rs (137insertions/40deletions), including private production charge construction and inline unit tests. All418 frozen tracked-source hashes independently match. No product/source/review input, protected qualification case/target or other worktree was modified. AGENTS.md/CLAUDE.md/.agent/rules/writing_rust.md and applicable FR362/371/AD003/TC919 context were inspected. No applicable AssuranceProfile exists. No spec/plan changes: spec-review/sub-analyses are inapplicable. Rust lane is folded into SR15231 code-review; optional broad semantic fanout was not requested or run. No reviewer Cargo/compiler/metadata/fmt/Kani/goto/backend or aggregate execution.

## Verdict

**PASS** — no introduced code/Rust/test-alignment defect; source-metadata simplification only. Aggregate pending.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

FR362's integer-arithmetic formulas remain max(B(a),B(b))+1 for Add/Subtract, B(a)+B(b) for Multiply, and B(a) for Negate. Each magnitude count is u64. The production builder maps these to two counts, widens BOTH with i128::from before adding, then uses checked u64::try_from on the exact sum. Maximum possible metadata sum is2^65-2, within i128; no truncation, wrapping, saturation, unsafe cast or narrower mathematical input is introduced. Fitting values go directly to Charge::size/Units; only genuine amounts above u64::MAX construct Integer and call exact_size, preserving the exact refusal payload. Previously every amount constructed Integer then exact_size converted fitting values back to Units. Negate's count+0 is mathematically the same amount. This alters metadata construction, not actual Integer arithmetic or storage.

The returned Charge retains IntegerArithmeticArithmetic, one IntegerBits request, default work1/results0. The evaluator performs exactly ONE actual meter.charge at the same location: after the unchanged operand charge and before left.add/sub/mul/neg and optional domain refusal. Result retention, other families, Meter cancellation/injection/size-field priority/admission atomicity, full counter behavior, representation, public API, dependency/lock/CI/proof inputs are unchanged. Exact_size and size canonicalize fitting requests to the same ChargeAmount::Units form; oversized requests stay Exact. Construction of the charge before Meter denial is unchanged in ordering, and denial priority is still determined by the actual Meter, not an injected shortcut. No vendoring, duplicate computation, fake public giant operand, test-only production branch, stub, async/lock or wire/persistence conversion was found.

The strengthened existing metadata test calls the same private production charge builder and real Meter, using independently authored decimal strings plus BigInt arithmetic. Cases0,small5,u64MAX,u64MAX+1 in both count orders and2*u64MAX detect lost precision/saturation. A real admitted operand prefix precedes the request. Fits check successful high-water/admission counts, actual requested size under test-support and exact one-less refusal payloads where the prefix fits. Oversized cases assert full Incomplete fields with the exact unbounded requested amount and unchanged counters/prefix. New cancellation/injection control installs the real cancellation handle or actual one-shot denial after that prefix and confirms WorkUnits denial takes priority over the oversized IntegerBits request, with no later admission/result. Allten counters and actual test-support points are checked. It supports FR371AC3 without pretending impossible bit counts are public Integer objects or that this alone exercises every public operation. Existing unchanged public allfour-operation, native/overflow/Large/domain/boundary/point-denial tests retain operation-level coverage. No whole FR362AC10 completion, new allocation measurement, old-behavior red or proof-fit improvement is claimed.

Focused coder evidence inspected directly: initial lint correctly failed clone_on_copy in the new test; its red log/source remain archived. Candidate fix dereferences Copy ScalarLimits and formats tests; retry1 reran fmt/fmt-check/lint(allthree default/test-support/thumbv7em-none-eabi lanes), numeric4+4 tests and integration34+34 across ir705_bounded_integer/ir653_scalar/ir673_scalar, all exit0. Actual MemoryMax16GiB/SwapMax0/jobs1/testthreads1/RAYON1/QSPECfd7 and own target. Successful scope exact memory.peak654741504bytes with every memory event0. Heavy-release records both scopes inactive, hierarchy gone and ten exact compiler/backend guards empty. Manifests/lock unchanged. These are supplied focused execution receipts, not reviewer execution or proof memory measurements. Final reviewed/custody-head once-only aggregate remains pending and required before merge.

Plan completion: not assessed

## Review data

```yaml
clean: true
scope:
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362
    schedule and counters on native, native-overflow and arbitrary-size cases. Test
    sufficient limits, independently calculated exact
- id: FR-362-AC-1
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For binary operations, use pairs from a fixed set including `i128::MIN`,
    -1, 0, 1 and `i128::MAX`; for negate, use each member as a unary operand. Compare
    integer add, subtract, multiply and negate on
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For a successful representative of each integer arithmetic operation, each
    rational arithmetic operation, integer ordering and rational ordering, the admitted
    point sequence equals its family row in B
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate
    the maximum size amount across their points from the formulas in Behavior, using
    the unreduced numerator and denominator for ra
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Inject a one-shot denial at each named point in representative integer
    arithmetic, rational arithmetic, integer ordering, rational ordering and already-decided
    Boolean retention calls. Each returns `I
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: For integer add, subtract, multiply and negate, rational add, subtract,
    multiply, divide and negate, and integer/rational ordering, independently calculate
    the required bit high-water amount from Beha
- id: integer_arithmetic_charge
  path: src/numeric.rs
  role: examined
  excerpt: let amount = i128::from(left) + i128::from(right);
- id: integer_arithmetic-admission
  path: src/numeric.rs
  role: examined
  excerpt: meter.charge(integer_arithmetic_charge(operation))?;
- id: ChargeAmount::exact
  path: src/accounting.rs
  role: examined
  excerpt: 'Some(units) => Self::Units(units),

    None => Self::Exact(Box::new(amount)),'
findings: []
bindings:
- test_id: src/numeric.rs::tests::integer_bit_counts_preserve_exact_amounts_above_the_meter_limit
  ac_id: FR-371-AC-3
  trace: correct
- test_id: src/numeric.rs::tests::arithmetic_metadata_keeps_oversized_requests_behind_cancellation_and_injection
  ac_id: FR-371-AC-3
  trace: correct
```
