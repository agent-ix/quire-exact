---
id: SR-14800
title: IR-705 native-width implementation code-review
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-exact; IR-705; src/integer.rs, tests/ir705_bounded_integer.rs and related specification"
review_set: subset
---

## Summary

Review the range-proven native-width implementation in Integer add/subtract/multiply/negate and its real public tests. All changed statements, preserved fallback paths, related existing refinement and Meter controls were read. The source changes only integer.rs and its existing integration test; Cargo manifests, lock, workflow, lints and public facade are unchanged.

Ticket: IR-705. Frozen source head: 888d5a5c8ce590b75954d11d1b972e0ad0d3d23c; published base: 7ff882c27849dc52a92e88cc8f54bc8b0f3c5bac. Independent reviewer, model gpt-6.1-sol, invocation 9491716e-a8e9-421b-9a5a-ff8f4e7cea9f. Read AGENTS/CLAUDE and applicable code-review/Rust/gap skills. No applicable AssuranceProfile under spec/. This review performs static source analysis and Quire computations only; no Cargo/rustc/formatter/metadata/native/backend or aggregate execution. No product edits or push. Findings go only to private Linear, not the public GitHub repository. No spec/plan change in PR70, so spec-review and sub-analyses do not apply.

## Verdict

**PASS** for scoped code-review including Rust lane; no new defects. Each exact i64 conversion retains an i64 local and explicitly widens before ordinary i128 arithmetic, then returns Small. The full signed-i64 ranges fit i128: Add [-2^64, 2^64-2], Subtract [-2^64+1, 2^64-1], Multiply [-2^126+2^63, 2^126], Negate [-2^63+1, 2^63]. No overflow, wrap, narrowing, assumption or refusal can arise in that arm. Failed conversions retain the prior checked full-i128 and genuine BigInt continuation; non-Small routes are unchanged. No claim of solver tractability, BigInt elimination, OOM causality or IR-917 fit follows from this verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Searched existing Integer operations and test helpers before assessing additions. Production has one implementation per operation and reuses native/small/from_big_normalized; no copied dependency, second authoritative rule, new helper abstraction, vendoring, unrequested tracking machinery, feature bypass, unsafe or lint waiver. Cache normalization and borrowed BigInt remain unchanged. Arithmetic is ordinary widening only after exact checks, which is safe in debug and release. No new panic, async, lock, lifecycle, wire/persistence or resource-loop surface.

Read all existing ir705 integration assertions, relevant integer unit checks, FR-371, AD-003, TC-919 and FR-362 charge definitions. New 7x7 edge Cartesian tests cover both extrema, adjacent extrema, signs, zero, cancellation and all four operations against independent checked-i128 references. Fixed literals independently check MIN squared and negated MIN. The continuation matrix includes immediately-outside-i64 on both sides, both operand orders, full-i128 overflow, and genuine Large cancellation; independent BigInt expected arithmetic does not call the implementation under test. Existing fixed decimal overflow expectations remain. The external safe allocator observes only four standalone operations on 25 extreme pairs, with setup and semantic assertions outside the window; count and bytes are both zero. These behavioral controls preserve refinement and allocation behavior; the equivalent old implementation can pass them, so they are not falsely presented as a red control of the dispatch mechanism. Source inspection verifies that mechanism; only real unchanged Kani qualification can determine tractability.

Four additional public Meter cases independently require arithmetic bit amounts 65/65/128/64 and exact results. The shared existing matrix still asserts all ten counters, sufficient/exact/one-less bit and work boundaries, each injected denial, admitted prefixes, domain refusal and no fabricated charge. Negate MIN at one-less bits properly stops at operands before any work. All preexisting cases/assertions are retained. Supported imported #[trace] appears only on actual #[test] functions. The computed matrix resolves all three new functions to the intended existing ACs, with no qualified-attribute discoverability defect.

Author raw receipts/logs and root verification were inspected, not rerun: seven focused commands first-attempt exit0, fmt/fmt-check, lint default/test-support/no_std target, integer unit9 each, three integration bins29 each configuration; no ignored focused execution. Actual scope max17179869184/swap0, peak711254016, events all0; this is scope memory, not per-test or proof memory. Cargo manifest/lock unchanged; heavy released and seven named process guards empty. Full aggregate remains the lead's once-only final gate after review custody. Prior original healthy16GiB and authorized40GiB attempts remain OOM without a verdict. FR-359-AC-10 and FR-371-AC-6/current consumer receipts remain open; whole repository gap verdict is separately FAIL in SR-14801. IR-917 owner source/cohort/selector is unavailable and cannot be substituted. Spec status describes the pre-implementation proposal and open qualification; it does not assert this reviewed candidate has no source. Durable acceptance status must continue to distinguish implementation delivery from unproved qualification, rather than manufacture a new contract or bookkeeping PR.

## Review data

```yaml
clean: true
scope:
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve
    the exact value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX,
    -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range.
    MIN has magnitude bit length 128; its negation is positive 2^127 through genuine arbitrary-precision
    continuation. Compare fitting operations with independent checked i128 arithmetic; compare non-fitting
    results with independently authored decimal literals. Reject malformed/noncanonical spellings as be
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain
    equal, identically ordered, identically hashed with the same hasher and identically formatted before
    and after BigInt access and cloning. A borrowed as_big reports the exact value throughout concurrent
    read-only accesses; all accesses to one retained materialization observe the same object. Clone/drop
    do not invalidate surviving permitted borrows or introduce a cache-state value.
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362 schedule and counters
    on native, native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated
    exact/one-less bit and work boundaries, denial injected at every named point, and outside-domain results.
    Compare all completed/refused/incomplete fields and admitted prefixes with the independent normative
    formulas. No representation transition manufactures a FunctionCall or other charge.
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate,
    magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external
    allocator observer around standalone Integer calls confirms no allocation on that native path, including
    clone after cache access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128
    MIN/MAX, compare independently prepared expected limbs, observe no stream allocation and confirm that
    initially empty caches remain empty. Observation of public metered calls separately iden
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'Public signatures and all existing Integer methods remain available, except that the divisor
    parameters of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve
    the admitted NonZeroInteger signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md),
    replacing only their former plain &Integer parameters. Their existing nonzero results and infallible
    return types remain preserved; every other public signature and Integer method retains its existing
    preservation guarantee. Trait checks preserve Send/Sync and current panic-unwind traits. Default and '
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original
    input/domain bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion
    cover and counterexample checks. Healthy verification and both substantive falsifications are required.
    Existing all-four-operation, accounting/denial and refinement obligations remain required. A compiler
    error, MemoryExhausted, timeout, missing mutation execution or missing teardown is inconclusive and
    does not satisfy this criterion.
- id: FR-362-AC-1
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For binary operations, use pairs from a fixed set including `i128::MIN`, -1, 0, 1 and `i128::MAX`;
    for negate, use each member as a unary operand. Compare integer add, subtract, multiply and negate
    only when the corresponding checked `i128` operation returns a value. Repeat over reproducibly sampled
    signed inputs. Every compared kernel result equals the independent checked result.
- id: FR-362-AC-2
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes to the independently
    authored decimal value `340282366920938463463374607431768211454`; negating `Integer::from(i128::MIN)`
    completes to `170141183460469231731687303715884105728`. Neither result is narrowed to `i128`.
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For a successful representative of each integer arithmetic operation, each rational arithmetic
    operation, integer ordering and rational ordering, the admitted point sequence equals its family row
    in Behavior. A successful already-decided Boolean retention admits only `boolean.result-retain`.
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate the maximum size amount
    across their points from the formulas in Behavior, using the unreduced numerator and denominator for
    rational normalization. The observed final `integer_bits` and `value_occurrences` equal those maxima;
    work consumption equals the number of admitted points and result consumption is one.
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Inject a one-shot denial at each named point in representative integer arithmetic, rational
    arithmetic, integer ordering, rational ordering and already-decided Boolean retention calls. Each
    returns `Incomplete` naming the denied point, with no completed value, no result unit for the denied
    charge and no later point admitted by that call.
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For integer add, subtract, multiply and negate, rational add, subtract, multiply, divide and
    negate, and integer/rational ordering, independently calculate the required bit high-water amount
    from Behavior. On fresh meters with other limits sufficient, that exact bit limit completes with the
    expected value, while a limit one below stops at the first point whose amount exceeds it, with exact
    prior consumption, request and admitted prefix and no result unit. Include cancellation `1000 - 999`,
    `255 * 255`, rational `(2/3) * (3/2)`, `(3/4) / (5/7)`, `(5/7) - (4/7)`, negate `-5/8`, and `7/3 <
    5/8`. '
- id: FR-362-AC-19
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'Through public `evaluate_integer_arithmetic` and a fresh public `Meter` for each call, add
    `8 + 3` and `3 + 8`, subtract `8 - 7`, and multiply `8 * 3`, without a result bound and with all non-bit
    limits sufficient. The independently calculated `integer-arithmetic.arithmetic` bit amounts are respectively
    5, 5, 5 and 6: add/subtract use `max(B(a), B(c)) + 1`, including the larger right operand and a result
    that cancels to one bit; multiply uses `B(a) + B(c)`. At each exact `integer_bits` limit, the call
    completes to respectively 11, 11, 1 and 24, admits operands, arithmetic and result-retain in '
findings: []
bindings:
- test_id: tests/ir705_bounded_integer.rs::signed_i64_extremes_refine_all_four_widened_operations
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::failed_width_guards_retain_full_i128_and_arbitrary_precision_results
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::metered_operation_matrix_preserves_fr362_amounts_work_and_denial_prefixes
  ac_id: FR-371-AC-3
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::widened_i64_extreme_operations_request_no_allocations
  ac_id: FR-371-AC-4
  trace: correct
```
