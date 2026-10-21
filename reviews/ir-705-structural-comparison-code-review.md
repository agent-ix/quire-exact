---
id: SR-14842
title: IR-705 structural comparison code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; IR-705; src/integer.rs, tests/ir705_bounded_integer.rs
  and owning spec
review_set: subset
---

## Summary

Independent nonauthor review of IR-705 PR73. Frozen source d7fdf86e50eaf151f28b3dc81f81d547c79af670; published base e5148c6307652dbc538dc6b647660bcea099c9f8. The delta changes only src/integer.rs and tests/ir705_bounded_integer.rs. Repo AGENTS.md/CLAUDE.md, lints and AD-003/FR-371/TC-919 were inspected. No applicable AssuranceProfile exists in spec/. No product changes or Cargo/rustc/fmt/metadata/native/Kani/backend/goto execution was performed by this reviewer. Supplied focused execution evidence was inspected directly. Code-review includes its Rust lane in one artifact per both method skills. Spec/plan review is inapplicable because no spec/plan changes. Model gpt-6.1-sol; invocation 95bedea3-2186-4d88-9ff5-80ac9d4b0803.

## Verdict

**PASS** — No introduced implementation defect established; final aggregate and genuine consumer qualifications remain outstanding.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Analysis

The private tuple field and enum are confined to integer.rs. Production Large is constructed only by failed checked i128 conversion in from_big_normalized (line77) and clone of an existing Large (line101). All public from_big/parse and general arithmetic results use that normalization; primitive/default/zero/one construction uses inline constructors. Audited abs, pow, split_factor_two, shifted_left, Add/Subtract/Multiply/Negate, gcd, exact_div, both division/remainder methods, power_of_ten, power_product_bits, power_of_two and usize continuation. There is no mutable public integer payload, tag factory, deserializer or hidden production Large fabrication. as_big returns immutable &BigInt; each inline cache initializer uses only immutable payload, and Large borrows stored BigInt. The new sign comparison is therefore justified by the already-selected construction policy: negative Large is strictly below i128 MIN and positive Large strictly above i128 MAX. Zero Large is unreachable through production routes.

Ord covers every closed storage pair with explicit enum alternatives: Small64/Small64 uses i64 comparison, both cross-inline pairs widen i64 exactly, Small128/Small128 uses i128, Large/Large uses genuine BigInt ordering, and both mixed directions use the Large sign. There is no catch-all silently accepting a future variant. Eq delegates once to this numeric order; PartialOrd remains Some(cmp). Sign-directed mixed comparison never materializes inline BigInt or mutates cache. It preserves reflexivity, antisymmetry and transitivity as the mathematical order of the three value ranges, including equal Small64/Small128 values. Hash/Display/Debug/as_big, constructors, arithmetic, Meter, interval inclusivity, trait surface, storage and cache implementation are byte-unchanged. No extra public API, unsafe, cast truncation, new panic, conditional Kani implementation, duplicated comparison authority, vendored implementation, changed dependency or weakened CI lane appears.

New independent public BigInt oracle tests cover every storage pair with i64 MIN/MAX, i128 MIN/MAX, zero, both signs, same-value cross-inline representations, MIN-1, MAX+1 and ±2^200. Every pair checks cmp/partial_cmp/equality and reversed ordering; reflexivity is explicit. The second loop runs after deliberate materialization (some earlier Hash/Debug observers also warm caches; empty-cache allocation behavior is separately checked). Clone/hash/Display/Debug remain exact. New private tests inspect true Large construction/clone/overflow results and cancellation to Small128, empty inline caches before/after mixed comparisons and stable later borrows. Existing constructor, cache concurrency, traits, arithmetic Cartesian/overflow, independent Meter boundary/denial/prefix, MIN magnitude, Decimal and IEEE tests remain intact. All four new test functions carry actual imported #[trace] binders; five resolved AC bindings are added, none removed. The diagnostic is supplemental AC2 evidence and expressly does not expand AC4 to promise allocation-free operations on every Large input.

Old-control source was compared byte-for-byte to git show e5148c6:src/integer.rs: it is the complete original production file, not a comparator stub. Its actual log compiles and runs the named integration test, then fails the allocation assertion with count1 versus expected0 and exit101. The restored current source matches healthy-formatted-integer.rs byte-for-byte and the actual restored run passes with exit0. This is a substantive production red/green resource control, not a compiler failure. Manifests/lock are unchanged. All observer/reference setup is outside the allocation measurement; the measured window performs real public Eq/Ord in both directions.

Raw candidate logs and exits confirm make fmt/fmt-check/lint (default, test-support and thumbv7em-none-eabi Clippy) pass; Integer/IEEE/Decimal unit subsets total52 successful executions, eight integration bins across both configurations total124, plus one restored diagnostic. Each recorded phase uses own target/jobs1, verified QSPEC fd7a16b and actual16GiB/swap0. Receipts and final-cgroup.txt record peak913600512 bytes and zero memory events; heavy-release.json records inactive/not-found scope, hierarchy gone and all ten exact process guards empty. These are author-run focused checks, not review-run or aggregate evidence. Final make ci remains the lead's once-only final reviewed-head gate.

There is no new consumer proof result. Actual original e514 healthy attempt was oom-kill at17179869184 bytes with no assertion/cover/verdict; actual IR705 comment6fbdeeff and retained healthy-qualification evidence preserve this. Removing avoidable mixed comparison allocation establishes neither mixed-branch feasibility in that proof nor memory dominance or RAM improvement. FR371AC6 and five current CG consumer proof/receipt obligations stay open. Genuine IR917 source/log is absent and its post-change fit remains independently INPUT-BLOCKED / UNMEASURED; original Add is never substituted.

## Review data

```yaml
clean: true
scope:
- id: FR-362-AC-1
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: For binary operations, use pairs from a fixed set including `i128::MIN`, -1, 0, 1 and `i128::MAX`; for
    negate, use each member as a unary operand. Compare integer add, subtract, multiply and negate only when the
    corresponding checked `i128` operation returns a value. Repeat over reproducibly sampled signed inputs. Every
    compared kernel result equals the independent checked result.
- id: FR-362-AC-2
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: Under sufficient limits, `Integer::from(i128::MAX)` multiplied by 2 completes to the independently authored
    decimal value `340282366920938463463374607431768211454`; negating `Integer::from(i128::MIN)` completes to `170141183460469231731687303715884105728`.
    Neither result is narrowed to `i128`.
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
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate preserve the exact
    value for fixed boundaries and reproducibly sampled inputs. Fixed cases include i128 MIN/MAX, -1, 0, 1, i64
    MIN squared, MAX+1, MIN-1, negated MIN and large cancellation into the native range. MIN has magnitude bit length
    128; its negation is positive 2^127 through genuine arbitrary-precision continuation. Compare fitting operations
    with independent checked i128 arithmetic; compare non-fitting results with independently authored decimal literals.
    Reject malformed/noncanonical spellings as be
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and arithmetic routes remain equal,
    identically ordered, identically hashed with the same hasher and identically formatted before and after BigInt
    access and cloning. A borrowed as_big reports the exact value throughout concurrent read-only accesses; all
    accesses to one retained materialization observe the same object. Clone/drop do not invalidate surviving permitted
    borrows or introduce a cache-state value.
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362 schedule and counters on native,
    native-overflow and arbitrary-size cases. Test sufficient limits, independently calculated exact/one-less bit
    and work boundaries, denial injected at every named point, and outside-domain results. Compare all completed/refused/incomplete
    fields and admitted prefixes with the independent normative formulas. No representation transition manufactures
    a FunctionCall or other charge.
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For primitive inputs and result fitting i128, inspection identifies native Add/Subtract/Multiply/Negate,
    magnitude, to_u64 and interval paths without BigInt construction/materialization. A safe external allocator
    observer around standalone Integer calls confirms no allocation on that native path, including clone after cache
    access. Inline magnitude-stream controls cover zero, 32-bit limb crossings and i128 MIN/MAX, compare independently
    prepared expected limbs, observe no stream allocation and confirm that initially empty caches remain empty.
    Observation of public metered calls separately iden
- id: FR-371-AC-5
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: 'Public signatures and all existing Integer methods remain available, except that the divisor parameters
    of Integer::exact_div, Integer::div_rem_truncating and Integer::div_mod_floor preserve the admitted NonZeroInteger
    signatures defined by [FR-374](./FR-374-nonzero-integer-divisor-admission.md), replacing only their former plain
    &Integer parameters. Their existing nonzero results and infallible return types remain preserved; every other
    public signature and Integer method retains its existing preservation guarantee. Trait checks preserve Send/Sync
    and current panic-unwind traits. Default and '
- id: FR-371-AC-6
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Genuine consumer healthy TC027, its bound mutant, and TC025 arithmetic mutant retain the original input/domain
    bounds, oracle, unwind3, Cadical, 600-second native ceiling, 16GiB memory ceiling, completion cover and counterexample
    checks. Healthy verification and both substantive falsifications are required. Existing all-four-operation,
    accounting/denial and refinement obligations remain required. A compiler error, MemoryExhausted, timeout, missing
    mutation execution or missing teardown is inconclusive and does not satisfy this criterion.
- id: FR-371-AC-7
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Private mechanism checks distinguish Small64 and Small128 construction, normalization and clone routes
    according to AD-003 without exposing a public tag API. Source inspection identifies storage matching before
    native-value extraction for Small64 Add/Subtract/Multiply/Negate, widening before ordinary arithmetic, Small128
    result construction and no BigInt continuation inside those arms. Check all other inline pair classes, failed
    numeric-width guards, genuine i128 overflow and Large cancellation; a small Small128 result remains Small128
    on chaining. Confirm both inline MIN magnitude streams and
findings: []
bindings:
- test_id: src/integer.rs::tests::mixed_comparisons_preserve_empty_inline_caches_and_later_borrows
  ac_id: FR-371-AC-2
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::structural_comparisons_match_independent_bigint_order_across_all_storage_pairs
  ac_id: FR-371-AC-2
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::mixed_large_inline_comparisons_do_not_allocate_inline_bigint_images
  ac_id: FR-371-AC-2
  trace: correct
- test_id: src/integer.rs::tests::genuine_large_construction_and_results_stay_outside_i128
  ac_id: FR-371-AC-7
  trace: correct
- test_id: src/integer.rs::tests::mixed_comparisons_preserve_empty_inline_caches_and_later_borrows
  ac_id: FR-371-AC-7
  trace: correct
```
