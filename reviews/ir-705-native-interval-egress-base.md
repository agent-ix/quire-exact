---
id: SR-15259
title: IR-705 native interval egress base
type: SpecReview
analysis: base
scope: agent-ix/quire-exact; IR-705 PR81; exact1fb4df677fe9d7ae8c8fc038b9b7608146ad81d0
  versus b692cc83d84eac33477a7d3d2770bc11f0e0350a; full repository matrix/source inventory;
  integer/native egress, actual public ingress, refusal and cache tests; FR384/371/359/383
  AD003 TC919
review_set: subset
---

## Summary

Unchanged FR384/FR371/AD003/TC919 contract permits this private typed egress conversion; no specification amendment is needed.

## Verdict

PASS — Component review is not yet final acceptance: actual endpoint theorem/control receipts remain pending.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Coverage

Plan completion: not assessed

Reviewed frozen public 1fb4df677fe9d7ae8c8fc038b9b7608146ad81d0 against merged b692cc83d84eac33477a7d3d2770bc11f0e0350a. The only production delta adds crate-private IntegerInterval::from_native_interval consuming the opaque validated NativeIntegerInterval, and delegates the existing private egress to it. Exact endpoints use unchanged Integer::from(i128), preserving Small128 storage with initially empty safe caches. No public scalar-pair unchecked constructor, trait, dependency, cfg replacement, Meter/arithmetic policy, domain clipping or proof assumption is introduced. Public NativeIntegerInterval::new validates order; the sole non-test direct interval literal is the already-proved sorted operation.capacity image. The new helper cannot manufacture a forged operation capability.

The domain-refusal call remains inside the actual excluded Some semantic-domain branch, after operands/arithmetic charges and before result-retain. Boxed IntegerOutOfDomain target, full-i128 endpoint values, cancellation/denial priority, cumulative counters and pending result-retain injection are unchanged. General Integer/as_big and interval spanning remain available unchanged. Eliminating redundant general Ord is a structural mechanism; it proves neither absence of every BigInt drop continuation nor 16GiB genuine-consumer fit.

Independently compared all 471 production-candidate archived Git files to exact d227558fdd9a20be2bce7827b38cf386bb2d2cb8, read the focused raw log and verified its SHA256. Actual focused default/support unit test each passes one selected test; native integration each passes all ten with zero filtered cases. Formatting, both focused Clippy -Dwarnings configurations and thumbv7em-none-eabi build pass. Actual cap16GiB/swap0, peak485785600B, zero memory events and scope teardown are recorded. Accepted focused-final logs have positive test counts. No verifier or aggregate was executed by this reviewer.

The new harness draws two unrestricted i128 values, orders them with min/max, calls the actual public checked interval constructor, actual private egress and actual public TryFrom ingress. Every valid interval is represented by a draw equal to its endpoints; no assumption narrows the population. Exact lower/upper assertions precede the roundtrip assertion. Six explicit covers require full MIN..MAX, both singleton extrema, asymmetric negative/positive intervals and interior zero straddling. This is endpoint/roundtrip refinement, not a new universal BigInt/cache or Meter theorem. Ordinary tests independently bind endpoints to BigInt and stable as_big object identity, and compare actual native/general/normative cumulative refusal sequences across all four operations.

Pure numerical/SMT, admission and Meter helper bodies remain unchanged except this explicitly reviewed egress dependency. PR78 numerical receipts retain their actual historical execution heads and scopes; PR80 direct/adapter/injection/schedule/ordering healthy receipts retain actual496a and canonical/substantive-controls actual2bac subjects. The interval-refusal conversion is the changed edge requiring its own new theorem, differential checks and mutant. No old execution is relabeled as current-head execution, no failed timeout becomes successful, and no blind unrelated proof rerun is demanded. Whole IR-705 and consumer16GiB qualification remain open.

Full computed repository matrix, not a handwritten table:194 criteria,171tagged,18method-without-symbol inspection rows,4untagged,1tagged-by-ignored-test. Matrix input includes all34 requirement documents returned by the engine. Whole source surface inventory covers34 Rust source files and20 integration test files: accounting/cancellation/limits/outcomes; identity/key/node/location/reference; integer/native/numeric/rational/decimal/IEEE/quantity; comparisons/equality; bounded collection/value/composite and text behavior. Existing owning requirements and unchanged prior review scope were reconciled; no new ownerless behavior, empty implementation, todo!/unimplemented!, test-only evaluator or hollow test was found. The helper belongs to FR384 explicit egress/domain-refusal and FR371 stable general representation; it adds no independent feature needing a new requirement.

Coverage diagnostics include8 optional module archetypes with no matching documents and historical unmatched/untracked tags. These do not mean an unreadable requirement corpus or zero criteria:194 criteria actually compute. No changed test has an unmatched or untracked trace; actual new test attributes bind only actual #[test] items and their asserted contract portions. Historical raw diagnostic outputs are retained in coverage-review.json; hand-written row/status diagnostics are not treated as test execution or edited. All prior consumer/trace debts are retained explicitly rather than hidden by the narrow production diff. No plan supplied: Plan completion: not assessed. Optional whole-repository semantic fanout was not run; focused intent/test/code alignment was reviewed directly under Rust/code methods. No spec/plan text changes; EARS, object, dependency and other specification-authoring subanalyses are inapplicable.

## Examined scope and bindings

```yaml
scope:
- id: FR-371-AC-1
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Primitive construction, canonical parsing, from_big and Add/Subtract/Multiply/Negate
    preserve the exact value for fixed boundaries and reproducibly sampled inputs.
    Fixed cases include i128 MIN/MAX, -1, 0, 1, i64 MIN squared, MAX+1, MIN-1, negated
    MIN and large cancellation into the native range. MIN has magnitude bit length
    128; its negation is positive 2^127 through genuine arbitrary-precision continuation.
    Compare fitting operations with independent checked i128 arithmetic; compare non-fitting
    results with independently authored decimal literals. Reject malformed/noncanonical
    spellings as be
- id: FR-371-AC-2
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Equal mathematical values constructed by primitive, parse, from_big and
    arithmetic routes remain equal, identically ordered, identically hashed with the
    same hasher and identically formatted before and after BigInt access and cloning.
    A borrowed as_big reports the exact value throughout concurrent read-only accesses;
    all accesses to one retained materialization observe the same object. Clone/drop
    do not invalidate surviving permitted borrows or introduce a cache-state value.
- id: FR-371-AC-3
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: Every public metered integer arithmetic operation preserves the FR-362
    schedule and counters on native, native-overflow and arbitrary-size cases. Test
    sufficient limits, independently calculated exact/one-less bit and work boundaries,
    denial injected at every named point, and outside-domain results. Compare all
    completed/refused/incomplete fields and admitted prefixes with the independent
    normative formulas. No representation transition manufactures a FunctionCall or
    other charge.
- id: FR-371-AC-4
  path: spec/functional/FR-371-bounded-integer-representation-refinement.md
  role: examined
  excerpt: For primitive inputs and result fitting i128, inspection identifies native
    Add/Subtract/Multiply/Negate, magnitude, to_u64 and interval paths without BigInt
    construction/materialization. A safe external allocator observer around standalone
    Integer calls confirms no allocation on that native path, including clone after
    cache access. Inline magnitude-stream controls cover zero, 32-bit limb crossings
    and i128 MIN/MAX, compare independently prepared expected limbs, observe no stream
    allocation and confirm that initially empty caches remain empty. Observation of
    public metered calls separately iden
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
bindings:
- test_id: native_interval_egress_preserves_exact_endpoints_and_stable_borrows
  ac_id: FR-384-AC-1
  trace: correct
- test_id: native_interval_egress_preserves_exact_endpoints_and_stable_borrows
  ac_id: FR-384-AC-4
  trace: correct
- test_id: native_interval_egress_preserves_exact_endpoints_and_stable_borrows
  ac_id: FR-371-AC-2
  trace: correct
- test_id: native_interval_domain_refusal_preserves_target_and_pending_retain_denial
  ac_id: FR-384-AC-3
  trace: correct
```

## Final new-subject qualification checkpoint

PASS for this component at exact frozen public 1fb4df677fe9d7ae8c8fc038b9b7608146ad81d0. No production change since reviewed d227; only the new actual-conversion harness is added at1fb. New same-PR FND005 is fully fixed by actual qualified evidence, not an assumption or a substituted evaluator. Six inherited findings retain explicit IR-705 ownership; no new source/control finding remains open. Component is review-mergeable after byte-exact typed-review custody and the lead final ordinary aggregate. This is not whole IR-705 completion or genuine consumer fit.

Independently rechecked every one of471 archived Git files against healthy execution1fb, empty healthy patch and identical tracked-before/after hashes. Actual universal production egress/public TryFrom roundtrip:0of400failed,398SUCCESS,2UNREACHABLE, all6cover properties SATISFIED,0.4222611seconds, final cgroup memory.peak439541760B. Assertion paths remain in the actual production conversion. Full i128 endpoint domain is preserved with no assume or clipping. The two unreachable safety checks are disclosed rather than counted as failed or omitted claims.

Actual production endpoint-assignment swap:1of400failed/1unreachable,1.815263seconds, finalpeak488271872B. Its six covers are2SATISFIED/4UNSATISFIABLE because non-singleton paths hit the intended earlier endpoint assertion; this is mutant evidence, not a claim all healthy covers pass on the mutant. The assertion-generated witness is distinct from the two printed cover-generated witnesses. Exact lower=-170141183460469231731687303715884105727 and upper=1 causes actual lowerSome(1) versus expectedSome(lower), native panic at the intended exact lower-endpoint assertion.

Standard cargo kani playback selects exactly that generated test: mutant1FAIL innerCargo101/outerKani1, same test and concrete values restored1PASS inner/outer0. Final peaks492855296B/496922624B respectively. All three actual stages are reconstructed from the exact base Gitarchive+their source.patch and checked against every tracked-before hash; tracked-after is equal. The original proof body is byte-exact, same native test/witness body is retained across bad/good replay, actual only production mutation is the two swapped assignments, and healthy production is fully restored. All healthy/mutant/replay scopes use16GiB/swap0/600s, proofs useCadical/unwind3, zero memory events, scopesGone and empty proof-engine guards. Ordinary foreign Cargo/rustc observations are permitted and are not falsely reported absent. No reviewer verifier or aggregate rerun.

The independent compiler-only diagnostic at actual production d227 with unchanged CGc861 original semantic artifact reports linked-stage reachable584to489 and total function-listing lines94842to88643. Trial private from_native_interval is reachable; IntegerInterval::spanning and Integer/BigInt Ord roots are absent, while BigInt drop glue remains. These are the same linked-stage metrics, distinct from earlier45953 pre-instrumentation-stage numbers. The matched proof package source/feature delta names only kernelb692to d227. Independently verified the final link provenance: installed goto-cc and exact kani_lib.c support bytes, baseline symtab input hashes,12unchanged original semantic source/artifact files, and recomputed both callgraph reachability and listing-line counts from actual saved outputs. Final diagnostic/source-lock custody is complete. The diagnostic contains no backend verdict and establishes neither memory dominance nor16GiB consumer fit. The diagnostic was not relabeled as a backend verdict.

Prior PR78 numerical/SMT and PR80 Meter qualification remain at their actual archived execution heads and only verified unchanged dependency closures carry; no old receipt becomes a new-head execution. Previous genuineIR917 MemoryExhausted and all earlier unsuccessful numerical experiments retain their actual unsuccessful results. Future healthy/mutant/native-playback and IR948 impossible-precondition profiles remain IR705-owned.
