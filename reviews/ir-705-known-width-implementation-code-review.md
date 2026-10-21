---
id: SR-14840
title: IR-705 three-variant implementation code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; IR-705; src/integer.rs, tests/ir705_bounded_integer.rs
  and owning spec
review_set: subset
---

## Summary

Independent review of IR-705, quire-exact PR72. Frozen source 9699cbece8521a5a6e272670e79adbb79c9ba16f; published base 697d6beb0dcb1937fc2bef2eb42015adf8f4840f. Exactly src/integer.rs and tests/ir705_bounded_integer.rs change. Code-review includes its rust-review lane. Repo AGENTS/CLAUDE and the reviewed AD-003/FR-371/TC-919 architecture were read. No applicable AssuranceProfile exists under spec/. This reviewer ran static source analysis and Quire computations only: no product edits, Cargo/rustc/fmt/metadata/native/Kani/CBMC/goto execution or aggregate. Supplied focused execution evidence is inspected evidence, not a test run by this reviewer. No spec/plan change; spec-review and its sub-analyses are inapplicable. Model gpt-6.1-sol; invocation c6189da3-471a-4ca7-8438-1d1a5dafcdf7.

## Verdict

**PASS** — No implementation defect established in the reviewed scope; final aggregate and genuine consumer proof acceptance remain outstanding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Analysis

All production IntegerRepr sites were inspected: only the two inline constructors, BigInt normalization, native observation, default/clone, magnitude limbs, four arithmetic arms, four existing primitive conversions, canonical parse and as_big need representation matching. No public conversion is added. From<i64>/zero/one/default choose Small64; i128/u64/fitting usize/parse/from_big choose Small128; usize retains real non-fitting continuation without a target-width assumption. Clone preserves its variant and resets its cell. Both as_big initializers convert only immutable payloads through unchanged safe once_cell::race::OnceBox; no temporary reference, reentry, local unsafe, changed signature, no_std feature or trait weakening is introduced. The maintained race primitive retains one winner and frees losers; existing synchronized primitive lifecycle test and new actual Integer concurrent-read controls complement each other.

The Small64 pair matches precede native() in Add/Subtract/Multiply, and Small64 Negate matches before native(). Each actual i64 operand widens to i128 before ordinary arithmetic and returns Small128. Add is bounded [-2^64,2^64-2], Subtract [-2^64+1,2^64-1], Multiply [-2^126+2^63,2^126], Negate [-2^63+1,2^63]; all fit signed i128. No overflow-to-BigInt continuation occurs inside those arms. Every other inline pairing preserves the prior numeric i64 guard, checked full-i128 and genuine BigInt continuation; Large handling remains general. Fitting normalization and chaining remain Small128, with no automatic narrowing promise. Both magnitude streams derive unsigned magnitude after widening Small64, including i64 MIN; i128 MIN retains unsigned_abs. Native comparison, sign, zero, bits and to_u64 remain class/cache independent; Hash/Debug retain exact BigInt observations.

Private AC7 controls check the complete existing construction surface, class transitions, all inline pairs, immediately outside-i64 values, genuine overflow, Large cancellation, warm-cache clone class/reset, concurrent stable borrows and independent MIN limbs. Independent public tests use checked i128 and fixed decimal/BigInt references, real allocation observation and existing Meter outcomes/counters/denial prefixes. New mixed-class Meter cases preserve independently supplied 64/65/128 bit amounts. No production trace, qualified trace attribute, copied implementation, vendored input, cfg(kani), assumption, weakened domain/ceiling, deleted gate, skipped assertion or new unsafe is introduced. Existing IEEE and Decimal callers retain actual public Integer use. Four AC7 binders are actual imported #[trace] attributes on #[test] functions; private class assertions do not substitute for public refinement or proof verdicts.

No implementation defect was established. This is a representation experiment, not a claim of branch pruning, memory causality, chained-expression tractability or consumer acceptance. Original ca8 16GiB and earlier 40GiB OOM evidence is inconclusive and preserved. Genuine IR917 source/log remains unavailable; its fit is UNMEASURED / INPUT-BLOCKED, not inferred from the original TC027 fixture.

## Evidence and remaining gates

Inspected actual external logs/exit files: make fmt and fmt-check PASS; make lint default, test-support and required thumbv7em-none-eabi configurations PASS. Integer/IEEE/Decimal unit filters total 48 successful executions and eight integration suites across default/test-support total 120 successful executions; no failures or ignored tests in those focused runs. Actual scope MemoryMax=17179869184, MemorySwapMax=0, final memory.peak=783466496, no OOM events; scope hierarchy gone and all nine exact process-name guards empty. Aggregate make ci has NOT run and remains the lead's final reviewed-head gate. Temporary external layout observer produced an explicit missing-example-crate-doc warning on both sides and was removed before formatting/lint/freeze; no product warning is waived.

Baseline setup's complete production hashes equal ca8, independently checked against git objects. Observer bytes match before/after (SHA256 917841e9a282ad0b5422018f0057705d231c39d901d4b38d82ba74c72e363791), same default host/dev/default-features/rustc1.98.1. Actual raw outputs: Integer size48 alignment16, Value size144 alignment16 on both sides. No fixed layout threshold or Kani RAM improvement is inferred from these equal measurements.

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
- id: FR-371-AC-7
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Private mechanism checks distinguish Small64 and Small128 construction, normalization and clone
    routes according to AD-003 without exposing a public tag API. Source inspection identifies storage
    matching before native-value extraction for Small64 Add/Subtract/Multiply/Negate, widening before
    ordinary arithmetic, Small128 result construction and no BigInt continuation inside those arms. Check
    all other inline pair classes, failed numeric-width guards, genuine i128 overflow and Large cancellation;
    a small Small128 result remains Small128 on chaining. Confirm both inline MIN magnitude streams and
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
findings: []
bindings:
- test_id: tests/ir705_bounded_integer.rs::mixed_constructor_pairs_preserve_arithmetic_and_public_observations
  ac_id: FR-371-AC-1
  trace: correct
- test_id: src/integer.rs::tests::both_inline_classes_retain_borrows_and_reset_only_clone_caches
  ac_id: FR-371-AC-2
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::mixed_constructor_pairs_preserve_arithmetic_and_public_observations
  ac_id: FR-371-AC-2
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::mixed_inline_operations_and_materialized_source_clones_allocate_nothing
  ac_id: FR-371-AC-4
  trace: correct
- test_id: src/integer.rs::tests::both_inline_classes_retain_borrows_and_reset_only_clone_caches
  ac_id: FR-371-AC-5
  trace: correct
- test_id: src/integer.rs::tests::both_inline_classes_retain_borrows_and_reset_only_clone_caches
  ac_id: FR-371-AC-7
  trace: correct
- test_id: src/integer.rs::tests::constructor_routes_preserve_the_selected_storage_class
  ac_id: FR-371-AC-7
  trace: correct
- test_id: src/integer.rs::tests::magnitude_limb_stream_does_not_materialize_small_cache
  ac_id: FR-371-AC-7
  trace: correct
- test_id: src/integer.rs::tests::storage_pair_transitions_keep_fitting_results_wide
  ac_id: FR-371-AC-7
  trace: correct
```
