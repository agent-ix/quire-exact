---
id: SR-5821
title: gap-analysis of IR-667 PR30 frozen implementation
type: SpecReview
analysis: gap-analysis
scope: agent-ix/quire-exact@33bcd5cfd4a2d0f9cccdad09a77697cf9b00d99a; src/accounting.rs,
  src/division.rs, src/ieee.rs, src/lib.rs, tests/ir653_admission.rs, tests/ir653_decimal_order.rs,
  tests/ir653_scalar.rs, tests/ir667_normalize.rs, tests/ir673_core.rs, tests/ir673_scalar.rs
review_set: subset
---

## Summary

Independent trace/reverse-gap review of the frozen IR-667 implementation diff. No changed-code defects found.

## Verdict

**PASS** for the exact PR diff only. This is not a whole-repository assurance verdict.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

Base: `6359cce0f7d58c738f41ea0d0d37ed64461d0b54`. Reviewed frozen head: `33bcd5cfd4a2d0f9cccdad09a77697cf9b00d99a`. Ticket: IR-667. Scope is the PR diff only; unchanged surrounding numeric logic was read as context. No source edits, builds, tests, locks or mutation runs were performed by this reviewer. No applicable AssuranceProfile exists in this repository's spec tree.

The ten changed files are: `src/accounting.rs`, `src/division.rs`, `src/ieee.rs`, `src/lib.rs`, `tests/ir653_admission.rs`, `tests/ir653_decimal_order.rs`, `tests/ir653_scalar.rs`, `tests/ir667_normalize.rs`, `tests/ir673_core.rs`, `tests/ir673_scalar.rs`.

## Examined Scope

Scope units, including clean units, are listed below. Excerpts are verbatim criterion prefixes when a criterion exceeds 600 characters. Source-file entries identify the diff units examined.

```yaml
- id: FR-358
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: context_only
  excerpt: The `quire-exact` meter SHALL deny an injected, named charge exactly once
    at its selected 1-based occurrence. Where `test-support` is enabled, the meter
    SHALL bound the admitted-charge log to 4096 entries without changing exact accounting.
    The public `InjectedDenial::occurrence` SHALL be `NonZeroU64`, making zero unrepresentable.
- id: FR-358-AC-4
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after 4096 admissions the admitted-charge accessor
    returns all 4096 records with their points in order and `charge_log_truncated()`
    is false. On the 4097th admission the slice still contains exactly that prefix
    and the flag becomes true.
- id: FR-358-AC-5
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, after the 4097th admitted charge, `admission_count()`
    and consumed work units reflect all 4097 admissions, including the one omitted
    from the diagnostic log.
- id: FR-358-AC-6
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Without `test-support`, production `Meter` contains no admitted-charge
    log and its `Counters` accounting state remains fixed-size and heap-free.
- id: FR-358-AC-14
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support` and sufficient limits, admit an IntegerBits request
    of 9 followed by 3 and then a charge with no size request. The records expose
    respectively 9, 3 and no IntegerBits request while the consumed IntegerBits maximum
    remains 9. A separate charge with IntegerBits requests 9, 3 and zero exposes present
    9; a charge with any positive number of IntegerBits zero requests exposes present
    zero. Both have the same fixed payload size as a single-request record, with no
    heap-owned request collection and no retained request list, order or multiplicity.
    A successful `charge_plan` with pai
- id: FR-358-AC-15
  path: spec/functional/FR-358-meter-denial-and-bounded-diagnostic-log.md
  role: examined
  excerpt: Under `test-support`, admit 4097 one-work-unit charges with independently
    chosen semantic-size requests, including repeated same-kind requests and distinct
    amounts at admissions 4096 and 4097. The single record log contains exactly the
    first 4096 fixed-size records with their points and per-admission semantic-size
    maxima in admission order; truncation is false at admission 4096 and true at 4097.
    Work consumption and admission count reach 4097, including the omitted record;
    existing size maxima still include the 4097th charge.
- id: FR-362
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: context_only
  excerpt: When evaluating kernel-owned integer or rational arithmetic, integer or
    rational ordering, or a Boolean connective over already-decided operands, the
    `quire-exact` kernel SHALL return the exact outcome and charge the point order
    and amounts defined below.
- id: FR-362-AC-10
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: 'For integer add, subtract, multiply and negate, rational add, subtract,
    multiply, divide and negate, and integer/rational ordering, independently calculate
    the required bit high-water amount from Behavior. On fresh meters with other limits
    sufficient, that exact bit limit completes with the expected value, while a limit
    one below stops at the first point whose amount exceeds it, with exact prior consumption,
    request and admitted prefix and no result unit. Include cancellation `1000 - 999`,
    `255 * 255`, rational `(2/3) * (3/2)`, `(3/4) / (5/7)`, `(5/7) - (4/7)`, negate
    `-5/8`, and `7/3 < 5/8`. '
- id: FR-362-AC-20
  path: spec/functional/FR-362-exact-scalar-arithmetic-and-atom-charges.md
  role: examined
  excerpt: On fresh sufficient public meters under `test-support`, evaluate the six
    rational calls listed below with no result domain. Each completes to the listed
    canonical value and admits operands, arithmetic, normalize and result-retain in
    order, consuming four work units and one result unit. The actual normalize record
    has only IntegerBits present, with the listed amount independently derived from
    the unreduced parts and `B(0) = 1`. The recorded arithmetic amount equals its
    separate listed bound. The normalize observation is not replaced by that bound,
    the final bit maximum or the reduced value's pa
- id: src/accounting.rs
  path: src/accounting.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: src/division.rs
  path: src/division.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: src/ieee.rs
  path: src/ieee.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: src/lib.rs
  path: src/lib.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir653_admission.rs
  path: tests/ir653_admission.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir653_decimal_order.rs
  path: tests/ir653_decimal_order.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir653_scalar.rs
  path: tests/ir653_scalar.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir667_normalize.rs
  path: tests/ir667_normalize.rs
  role: examined
  excerpt: 'Six public API tests: actual request projections, refusal atomicity, prefix
    truncation, rational normalize, integer arithmetic and ordering boundaries.'
- id: tests/ir673_core.rs
  path: tests/ir673_core.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
- id: tests/ir673_scalar.rs
  path: tests/ir673_scalar.rs
  role: examined
  excerpt: 'Frozen changed lines: typed admission records and preserved point projections.'
```

## Execution Evidence

Independently read receipt, subject events and stdout/stderr at `/tmp/ix-handoff/ir667-normalize-focused/full-candidate/attempt-2c7de383903d46e0975b9e6c3aa9ea42`: head matches the frozen candidate; make-ci and Quire validation both exit 0; scope teardown CONFIRMED. Logged make-ci commands cover fmt, default/support all-target Clippy, thumbv7em-none-eabi library Clippy/build, both test configurations, deny and documentation with warnings denied. The logs report 190 default and 199 test-support test executions, zero failed/ignored. Six new ir667_normalize tests pass in the support lane. Quire reports 164/164 documents grammar-clean, zero grammar findings. This is reviewed author-run evidence, not reviewer execution. Stable-rustfmt unsupported-option warnings and ambient duplicate archetype/inverse warnings are disclosed; no gate was weakened by the diff. No CI, Cargo, lockfile or specification file changed.

Normalize-undercharge proposal was read as source-derived reasoning only. No executed counterfactual or mutant kill is claimed.

## Coverage

Plan completion: not assessed

Reconciliation: fresh `quire matrix --scope . --format json` and `quire coverage --scope . --json` at the frozen reviewer head; no Quoin execution evidence store was queried. Static tags are not presented as runtime proof.

Whole-repository matrix observation: 124 criteria; 102 tagged, 11 method-without-symbol, 11 untagged, zero tagged-by-ignored-test. The scoped four acceptance criteria FR358 AC14/15 and FR362 AC10/20 are all tagged to the six actual new test functions. The matrix binder locations are tests/ir667_normalize.rs:53, :136, :217, :359, :487 and :588. AC10 has three family tests; AC20 binds the rational-normalize test separately. No new tags are stale or ignored. Scope does not claim 124/124 coverage.

Ambient gaps outside the PR: FR369 AC3/4/7/9, FR371 AC1..6, FR372 AC5. Their spec files are unchanged and git grep at the actual base finds no source/test tag for those IDs. This PR-scoped PASS does not resolve or grant whole-repository assurance PASS over these existing gaps. Coverage also reports external QSpec/local TC tags with no minted target and six absent-archetype declarations plus one property-shape diagnostic. These were inspected as context, not converted into changed-code findings. New ACs and binders use real declared targets; no hand-written matrix, blanket file tag, or inflated tag-count claim is used.

Reverse inventory for the changed behavior: (1) fixed typed per-admission payload and eight size fields, (2) admitted_charges typed-prefix accessor and root export, (3) charge observation/coalescing/atomic commit, (4) charge_plan actual pair observation, (5) shared cap/truncation custody, (6) existing caller point projections, (7) independent boundary/normalize public tests. Owning requirements are FR358 and FR362; no unowned behavior or newly imposed request/count/execution limit was found. Zero changed source stubs, zero new hollow test bodies, zero tests mocking the unit under review. The fixed record and per-admission maxima are visible independently of counters, while tests retain the existing arithmetic/accounting observations.

Optional separate repository-wide semantic fan-out was not run. Diff-scoped source/test oracle and requirement correspondence was explicitly requested and covered in the code/Rust review; this gap result remains bounded to the PR.
