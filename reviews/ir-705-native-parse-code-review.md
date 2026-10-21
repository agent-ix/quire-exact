---
id: SR-14740
title: IR-705 native canonical parsing code-review
type: SpecReview
analysis: code-review
scope: agent-ix/quire-exact; IR-705; src/integer.rs; tests/ir705_bounded_integer.rs;
  FR-371; AD-003
review_set: subset
---

## Summary

Focused code-review with the rust-review lane examined the entire two-file diff and Integer construction, normalization, retained borrowing, equality/order/hash/format and arithmetic continuation. The three production lines use the existing Small constructor only after unchanged canonical validation. Native parse overflow falls through to the unchanged genuine BigInt parser. No copied dependency algorithm, new ABI, representation, unsafe, test-only production path, narrowing, panic, accounting charge, dependency or gate weakening is introduced.

Ticket: IR-705. Frozen head: 6360878195198be4b492deb504668e2e4df590b4; exact base: d98191a808b8689a04b8d30be8f23b0e7676e8f3. Independent reviewer, static inspection and Quire only; no Cargo, rustc, rustfmt, Kani, solver or aggregate job executed by this reviewer. Runtime model: gpt-6.1-sol; run: f49c0172-66a7-42f1-90fc-5f69eb26cd0c. Public repository; private Linear comments only. No applicable AssuranceProfile was found under spec/. No product/spec edits or branch pushes. Spec-review/sub-analyses are inapplicable because no specification or plan changed.

## Verdict

**PASS** for the focused change. Canonical zero, signs, ASCII-only digits, i128 MIN/MAX and just-outside boundaries retain mathematical results and exact NonCanonicalInteger refusal. Same existing Small state is produced as primitive/from_big normalization. Borrow/cache lifecycle, default, Send/Sync, panic traits and no_std mechanism are unchanged. Canonical input receives one bounded native parse attempt before arbitrary-precision continuation; this neither changes semantic domains nor imposes a new public bound.

The new tests assert independent fixed native/BigInt boundary values, overflow continuation arithmetic, malformed spelling refusals, stable borrowed object, cross-route comparisons/hash/Debug/Display and clone/drop observations. Existing reproducible sampled arithmetic and all-four-operation meter controls remain byte unchanged. The allocation observation is deliberately tagged to AC-1 and documented as a diagnostic beyond value semantics; it does not invent an AC-4 allocation-free parsing requirement. No tautological expected value or mocked subject was found in the additions.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Loaded reviewer, code-review, rust-review, rust-style and repo conventions. Searched existing Integer construction/parsing and integration fixture helpers before assessing duplication. The existing canonical validator remains the single spelling authority; no handwritten second parser is introduced. The test additions extend existing sampled tests with boundary, refusal, cross-route and isolated allocation observations rather than restate a production algorithm. No .github workflow, Makefile, manifest, lock, coverage threshold or lint waiver changed. No async, IPC, persistence, task spawn, lock or cancellation behavior changed.

Author raw logs and receipts were inspected, not rerun: make fmt/fmt-check/lint exit0 (default/test-support Clippy and bare-metal target); bounded-integer integration 9 default and8 test-support pass; integer unit subset9 each configuration pass. Actual cgroup receipts state 17179869184-byte memory.max, swap0, no OOM events; cumulative peak771661824 bytes is not per-test peak. The complete old base integer.rs byte-matches git show at the base and compiles before the new allocation test fails with named assertion on canonical 0 (count_total2 versus0, exit101). Current source byte-matches the restored healthy file and that exact test passes exit0. This is a real sensitivity control, not a compiler failure. Cargo manifests/lock are unchanged. The lead must perform the once-only final aggregate gate after artifact custody. No consumer proof, causal OOM explanation, tractability success or completed IR-705 claim follows. FR-371's historical PROPOSED status is read as stale context, never evidence that existing implementation is absent.

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
- id: AD-003
  path: spec/assurance/AD-003-bounded-integer-representation.md
  role: context_only
  excerpt: 'The selected representation has two tags: `Small(i128, cache)` and `Large(BigInt)`. The abstraction
    function maps every `Small(n, cache)` to the mathematical integer `n`, independent of cache state;
    it maps `Large(b)` to the exact value represented by `b`. A populated Small cache must equal the Small
    payload''s mathematical value. Arithmetic uses checked native operations when the exact result fits
    i128 and genuine BigInt continuation when it does not. In particular, negating `i128::MIN` produces
    positive 2^127 through BigInt continuation.'
findings: []
bindings:
- test_id: tests/ir705_bounded_integer.rs::canonical_parsing_preserves_native_boundaries_and_arbitrary_precision
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::fitting_canonical_parsing_requests_no_allocations
  ac_id: FR-371-AC-1
  trace: correct
- test_id: tests/ir705_bounded_integer.rs::parsed_values_keep_observations_and_borrows_across_materialization_and_clone
  ac_id: FR-371-AC-2
  trace: correct
```
