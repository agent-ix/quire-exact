---
id: SR-15229
title: IR-705 PR74 integer-ordering metadata code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; PR74 frozen deb74ae; src/numeric.rs and tests/ir705_bounded_integer.rs
review_set: subset
---

## Summary

Ticket: IR-705; product PR quire-exact#74. Independent reviewer, distinct from coder ir705_native_parse_coder. Exact frozen clean head deb74ae509eeb015abe210f8046f827ab7de4799; published base f28d3cd3168143a8d0ef230dba5895b5e2ffbf07. Diff is only src/numeric.rs and tests/ir705_bounded_integer.rs. All415 frozen tracked-source hashes independently match. No source/artifact under review, protected qualification case/target or other worktree was modified. Repo AGENTS.md, CLAUDE.md, .agent/rules/writing_rust.md, AD-003, FR-362/371 and TC-919 were inspected. No applicable AssuranceProfile exists. No spec/plan changes: spec-review/sub-analyses are inapplicable. Rust lane is folded into code-review SR-15229. Optional whole-repository semantic fanout was not requested or run.

## Verdict

**PASS** — no introduced code/Rust or test-alignment defect. Narrow source simplification only; final aggregate remains pending.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

FR-362 integer ordering requires max(B(left),B(right)) at both operands and arithmetic, with B(0)=1. Existing magnitude_bits returns u64. The old helper converted each count to Integer, compared them, then exact_size converted its fitting result back to ChargeAmount::Units(u64); the new path takes the u64 max once and passes it to size, producing the identical charge representation. No sum, overflow, narrowing, saturation or new bound appears. Allthree named points, work/results and point order remain identical; actual left.cmp(right), rational/decimal routes, public API, cancellation, limits, denial selection and Meter traversal are unchanged. The removed helper was single-use; bits() remains for general arithmetic. No vendoring, copied implementation, test-only production path, stub, unsafe, async/lock, persistence/wire conversion or resource-bound change was found. No CI gate/dependency/config changed.

The new public integration test evaluates all11×11 ordered operand pairs and allfour relational operators against BigInt numeric and magnitude oracles. Constructors actually preserve Small64 versus Small128, including equal-valued cross-storage pairs, i64/i128 extrema, zero, and positive/negative Large beyond i128. It uses the genuine public order_numbers/Meter, tests exact-bit completion and bits-minus-one refusal, observes both admitted size requests under test-support, checks allten counters, denies each of the three points with exact payload/prefix, and reuses that same meter after the one-shot denial. Existing test-support-only point observation remains paired with real default-feature counter checks. New traces FR-362-AC-4/5/6/8 are correct contributions to their existing criteria, not claims this one test supplies every integer/rational branch. No FR-362-AC-10 tag was added and its broad boundary scope is not claimed complete. This patch removes general metadata construction but supplies no proof-memory or backend-fit evidence.

Supplied focused receipts inspected directly (not rerun by reviewer): make fmt/fmt-check exit0; make lint exit0 across default,test-support,thumbv7em-none-eabi; numeric tests3+3 passed; ir705_bounded_integer/ir653_scalar/ir673_scalar integration groups34+34 passed. Actual16GiB/swap0/jobs1/testthreads1/RAYON1/QSPECfd7, scope peak713998336bytes with allmemoryevents0. Heavy-release records hierarchy gone/allten exact guards empty. Cargo.toml/Cargo.lock unchanged. No reviewer Cargo, formatting, metadata, compiler, Kani/backend or aggregate run. Final reviewed-head aggregate is pending and required before merge; it is not inferred from focused checks.

Plan completion: not assessed

## Review data

```yaml
clean: true
scope:
- id: FR-362-AC-4
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Integer and rational comparisons over fixed and reproducibly sampled pairs
    equal independently checked numeric comparisons for equality and order. Decimal
    comparison is specified by [FR-363](./FR-363-metered-decimal-ordering.md).
- id: FR-362-AC-5
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For a successful representative of each integer arithmetic operation, each
    rational arithmetic operation, integer ordering and rational ordering, the admitted
    point sequence equals its family row in Behavior. A successful already-decided
    Boolean retention admits only `boolean.result-retain`.
- id: FR-362-AC-6
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For those successful operations on fresh meters, independently calculate
    the maximum size amount across their points from the formulas in Behavior, using
    the unreduced numerator and denominator for rational normalization. The observed
    final `integer_bits` and `value_occurrences` equal those maxima; work consumption
    equals the number of admitted points and result consumption is one.
- id: FR-362-AC-8
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Inject a one-shot denial at each named point in representative integer
    arithmetic, rational arithmetic, integer ordering, rational ordering and already-decided
    Boolean retention calls. Each returns `Incomplete` naming the denied point, with
    no completed value, no result unit for the denied charge and no later point admitted
    by that call.
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: 'For integer add, subtract, multiply and negate, rational add, subtract,
    multiply, divide and negate, and integer/rational ordering, independently calculate
    the required bit high-water amount from Behavior. On fresh meters with other limits
    sufficient, that exact bit limit completes with the expected value, while a limit
    one below stops at the first point whose amount exceeds it, with exact prior consumption,
    request and admitted prefix and no result unit. Include cancellation `1000 - 999`,
    `255 * 255`, rational `(2/3) * (3/2)`, `(3/4) / (5/7)`, `(5/7) - (4/7)`, negate
    `-5/8`, and `7/3 < 5/8`. '
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: context_only
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic
    mutant retain the original input/domain bounds, oracle, unwind3, Cadical, 600-second
    native ceiling, 16GiB memory ceiling, completion cover and counterexample checks.
    Healthy verification and both substantive falsifications are required. Existing
    all-four-operation, accounting/denial and refinement obligations remain required.
    A compiler error, MemoryExhausted, timeout, missing mutation execution or missing
    teardown is inconclusive and does not satisfy this criterion.
- id: src/numeric.rs::order
  path: src/numeric.rs
  role: examined
  excerpt: let bits = left.magnitude_bits().max(right.magnitude_bits());
- id: integer_ordering_preserves_values_sizes_and_denied_prefixes
  path: tests/ir705_bounded_integer.rs
  role: examined
  excerpt: '#[trace("FR-362-AC-4", "FR-362-AC-5", "FR-362-AC-6", "FR-362-AC-8")]'
findings: []
bindings:
- test_id: tests/ir705_bounded_integer.rs::integer_ordering_preserves_values_sizes_and_denied_prefixes
  ac_id: FR-362-AC-4
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::integer_ordering_preserves_values_sizes_and_denied_prefixes
  ac_id: FR-362-AC-5
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::integer_ordering_preserves_values_sizes_and_denied_prefixes
  ac_id: FR-362-AC-6
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::integer_ordering_preserves_values_sizes_and_denied_prefixes
  ac_id: FR-362-AC-8
  trace: correct
```
